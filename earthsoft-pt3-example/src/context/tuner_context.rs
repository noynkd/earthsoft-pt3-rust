use crate::buffer;
use crate::context;
use std::io::Write;
use earthsoft_sdk::pt3;

// =============================================================================
// TunerContext
// =============================================================================

#[derive(Debug)]
pub struct  TunerContext {
    isdb: pt3::Isdb,
    tuner: u32,
    device_context: std::sync::Arc<context::DeviceContext>,
    ring_buffer: Option<buffer::RingBuffer>,
    active_session: Option<TunerSession>,
}

impl TunerContext {
    const BLOCK_SIZE: u32   = 4096 * 47 * 8;
    const BLOCK_COUNT: u32  = 32;
    const SYNC_BYTE: u8     = 0x47;
    const NOT_SYNC_BYTE: u8 = !Self::SYNC_BYTE;

    pub(in crate::context) fn new(isdb: pt3::Isdb, tuner: u32, device_context: std::sync::Arc<context::DeviceContext>) -> Self {
        Self {
            isdb,
            tuner,
            device_context,
            ring_buffer: None,
            active_session: None,
        }
    }

    pub fn start(&mut self) -> Result<(), pt3::Error> {
        if self.ring_buffer.is_none() {
            let mut buffer = buffer::RingBuffer::new(Self::BLOCK_SIZE, Self::BLOCK_COUNT);
            buffer.allocate(self.device_context.clone(), true)
                .inspect_err(|e| {
                    eprintln!("buffer::RingBuffer::allocate() に失敗しました: {:?}", e);
                })?;

            for block_index in 0..Self::BLOCK_COUNT {
                let bytes = buffer.slice_block_mut(block_index);

                if bytes.is_empty() {
                    break;
                }

                bytes[0] = Self::NOT_SYNC_BYTE;

                buffer.sync_cpu(block_index)?;
            }
            
            self.ring_buffer.replace(buffer);
        }


        let mut ring_buffer = self.ring_buffer.take().unwrap();
        let (return_tx, return_rx): (std::sync::mpsc::Sender<buffer::RingBuffer>, std::sync::mpsc::Receiver<buffer::RingBuffer>) = std::sync::mpsc::channel();

        let stop_signal = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let worker_stop = stop_signal.clone();
        let device = self.device_context.clone();
        let isdb = self.isdb;
        let tuner = self.tuner;

        let filename = std::format!(
            "ISDB-{}{}.ts",
            if isdb == pt3::Isdb::Satellite { "S" } else { "T" },
            tuner
        );
        let mut file = match std::fs::File::create(&filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ファイル '{}' を開けませんでした: {:?}", filename, e);
                return Err(pt3::Error::InternalError); 
            },
        };

        device.set_transfer_test_mode(isdb, tuner, false, 0, false)
            .inspect_err(|e| {
                eprintln!("context::DeviceContext::set_transfer_test_mode() に失敗しました: {:?}", e);
            })?;
        device.set_transfer_page_descriptor_address(isdb, tuner, ring_buffer.get_descriptor_address())
            .inspect_err(|e| {
                eprintln!("context::DeviceContext::set_transfer_page_descriptor_address() に失敗しました: {:?}", e);
            })?;
        device.set_transfer_enabled(isdb, tuner, true)
            .inspect_err(|e| {
                eprintln!("context::DeviceContext::set_transfer_enabled() に失敗しました: {:?}", e);
            })?;

        let handle = std::thread::spawn(move || {
            let mut block_index = 0;

            while !worker_stop.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(1));

                let mut next = block_index + 1;
                if Self::BLOCK_COUNT <= next {
                    next = 0;
                }

                if Self::check_ready(&mut ring_buffer, next) {
                    if let Err(e) = ring_buffer.sync_io(block_index) {
                        eprintln!("buffer::RingBuffer::sync_io() に失敗しました: {:?}", e);
                    }

                    let bytes = ring_buffer.slice_block_mut(block_index);

                    if let Err(e) = file.write(bytes) {
                        eprintln!("ファイル '{}' の書き込みに失敗しました: {:?}", filename, e);
                    }

                    bytes[0] = Self::NOT_SYNC_BYTE;

                    if let Err(e) = ring_buffer.sync_cpu(block_index) {
                        eprintln!("buffer::RingBuffer::sync_cpu() に失敗しました: {:?}", e);
                    }

                    block_index += 1;
                    if Self::BLOCK_COUNT <= block_index {
                        block_index = 0;
                    }
                }
            }

            match device.get_transfer_info(isdb, tuner) {
                Ok(transfer_info) => {
                    if transfer_info.internal_fifo_a_overflow {
                        print!("[internal_fifo_a_overflow]");
                    }
                    if transfer_info.internal_fifo_a_underflow {
                        print!("[internal_fifo_a_underflow]");
                    }
                    if transfer_info.external_fifo_overflow {
                        print!("[external_fifo_overflow]");
                    } else {
                        print!("[external_fifo_max_usage_bytes: {}]", transfer_info.external_fifo_max_used_bytes);
                    }
                    if transfer_info.internal_fifo_b_overflow {
                        print!("[internal_fifo_b_overflow]");
                    }
                    if transfer_info.internal_fifo_b_underflow {
                        print!("[internal_fifo_b_underflow]");
                    }
                    println!();
                },
                Err(e) => {
                    eprintln!("context::DeviceContext::get_transfer_info() に失敗しました: {:?}", e);
                },
            };

            if let Err(e) = device.set_transfer_enabled(isdb, tuner, false) {
                eprintln!("context::DeviceContext::set_transfer_enabled() に失敗しました: {:?}", e);
            }

            let _ = return_tx.send(ring_buffer);
        });

        self.active_session.replace(TunerSession {
            stop_signal,
            worker_handle: Some(handle),
            ring_buffer_receiver: return_rx,
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut session) = self.active_session.take() {
            session.stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);

            if let Some(handle) = session.worker_handle.take() {
                let _ = handle.join();
            }

            if let Ok(ring_buffer) = session.ring_buffer_receiver.recv() {
                self.ring_buffer.replace(ring_buffer);
            }
        }
    }

    fn check_ready(buffer: &mut buffer::RingBuffer, block_index: u32) -> bool {
        if let Err(e) = buffer.sync_cpu(block_index) {
            eprintln!("buffer::RingBuffer::sync_cpu() に失敗しました: {:?}", e);
            return false;
        }

        let sync_byte = buffer.slice_block(block_index)[0];

        return match sync_byte {
            Self::SYNC_BYTE     => true,
            Self::NOT_SYNC_BYTE => false,
            other_byte     => {
                eprintln!(
                    "同期バイトの値({:#04x})が同期({:#04x})でも初期値({:#04x})でもありません。",
                    other_byte,
                    Self::SYNC_BYTE,
                    Self::NOT_SYNC_BYTE,
                );
                return false;
            }
        }
    }
}

impl Drop for TunerContext {
    fn drop(&mut self) {
        if let Some(mut session) = self.active_session.take() {
            session.stop_signal.store(true, std::sync::atomic::Ordering::SeqCst);

            if let Some(handle) = session.worker_handle.take() {
                let _ = handle.join();
            }

            if let Ok(ring_buffer) = session.ring_buffer_receiver.recv() {
                self.ring_buffer.replace(ring_buffer);
            }
        }
    }
}

// =============================================================================
// TunerSession
// =============================================================================

#[derive(Debug)]
struct TunerSession {
    stop_signal: std::sync::Arc<std::sync::atomic::AtomicBool>,
    worker_handle: Option<std::thread::JoinHandle<()>>,
    ring_buffer_receiver: std::sync::mpsc::Receiver<buffer::RingBuffer>,
}
