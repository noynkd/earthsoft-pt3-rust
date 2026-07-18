use crate::pt3::{ChannelSetting, DeviceContext, RingBuffer};
use crate::{Channel, Error, Packet, PacketStream, TunerInfo};
use async_trait::async_trait;
use earthsoft_sdk::pt3;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

// =============================================================================
// Tuner
// =============================================================================

#[derive(Debug, Clone)]
pub(crate) struct Tuner {
    device: Arc<pt3::Device>,
    isdb: pt3::Isdb,
    tuner: u32,
    device_context: Arc<Mutex<DeviceContext>>,
    tuner_info: TunerInfo,
    channels: Vec<ChannelSetting>,
    ring_buffer: Arc<Mutex<Option<RingBuffer>>>,
    active_session: Arc<Mutex<Option<TunerSession>>>,
}

impl Tuner {
    const BLOCK_SIZE: u32 = 4096 * 47 * 8;
    const BLOCK_COUNT: u32 = 32;
    const CHUNK_SIZE: usize = 188 * 256;
    const SYNC_BYTE: u8 = 0x47;
    const NOT_SYNC_BYTE: u8 = !Self::SYNC_BYTE;

    pub fn new(
        device: Arc<pt3::Device>,
        isdb: pt3::Isdb,
        tuner: u32,
        device_context: Arc<Mutex<DeviceContext>>,
        tuner_info: TunerInfo,
        channels: Vec<ChannelSetting>,
    ) -> Self {
        Self {
            device,
            isdb,
            tuner,
            device_context,
            tuner_info,
            channels,
            ring_buffer: Default::default(),
            active_session: Default::default(),
        }
    }

    fn check_ready(buffer: &mut RingBuffer, block_index: u32) -> bool {
        if buffer.sync_cpu(block_index).is_err() {
            return false;
        }
        let sync_byte = buffer.slice_block(block_index)[0];
        match sync_byte {
            0x47 => true,
            0xb8 => false, // !0x47 の値
            _ => false,
        }
    }
}

#[async_trait]
impl crate::Tuner for Tuner {
    async fn get_info(&self) -> Result<TunerInfo, Error> {
        let (cn100, _, _) = self.device.get_cn_agc(self.isdb, self.tuner).map_err(|e| {
            eprintln!(
                "earthsoft_pt3::pt3::Device::get_cn_agc() に失敗しました。: {:?}",
                e
            );
            Error::Failure
        })?;

        Ok(TunerInfo {
            signal_level: cn100 as f32 / 100.0,
            ..self.tuner_info.clone()
        })
    }

    async fn get_channels(&self) -> Result<Vec<Channel>, Error> {
        let channels = self
            .channels
            .iter()
            .map(|channel_setting| channel_setting.into())
            .collect();

        Ok(channels)
    }

    async fn get_current_channel(&self) -> Result<Channel, Error> {
        let (channel, _offset) = self
            .device
            .get_frequency(self.isdb, self.tuner)
            .map_err(|e| {
                eprintln!(
                    "earthsoft_pt3::pt3::Device::get_frequency() に失敗しました。: {:?}",
                    e
                );
                Error::Failure
            })?;
        let tsid = if self.isdb == pt3::Isdb::Satellite {
            self.device.get_satellite_id(self.tuner).map_err(|e| {
                eprintln!(
                    "earthsoft_pt3::pt3::Device::get_satellite_id() に失敗しました。: {:?}",
                    e
                );
                Error::Failure
            })?
        } else {
            0
        };

        self.channels
            .iter()
            .find(|channel_setting| {
                channel_setting.channel == channel && channel_setting.tsid == tsid
            })
            .map(|channel_setting| channel_setting.into())
            .ok_or_else(|| {
                eprintln!(
                    "チャンネル設定を取得できませんでした。: channel = {}, tsid = {}",
                    channel, tsid
                );
                Error::Failure
            })
    }

    async fn set_channel(&self, channel: &Channel) -> Result<(), Error> {
        let channel = self
            .channels
            .iter()
            .find(|channel_setting| {
                channel_setting.space == channel.space && channel_setting.index == channel.index
            })
            .ok_or_else(|| {
                eprintln!(
                    "チャンネル設定を取得できませんでした。: space = {}, index = {}",
                    channel.space, channel.index
                );
                Error::Failure
            })?;

        self.device
            .set_frequency(self.isdb, self.tuner, channel.channel, 0)
            .map_err(|e| {
                eprintln!(
                    "earthsoft_pt3::pt3::Device::set_frequency() に失敗しました。: {:?}",
                    e
                );
                Error::Failure
            })?;
        if self.isdb == pt3::Isdb::Satellite {
            self.device
                .set_satellite_id(self.tuner, channel.tsid)
                .map_err(|e| {
                    eprintln!(
                        "earthsoft_pt3::pt3::Device::get_satellite_id() に失敗しました。: {:?}",
                        e
                    );
                    Error::Failure
                })?
        }

        Ok(())
    }

    async fn start_stream(&self) -> Result<PacketStream, Error> {
        let mut session_lock = self.active_session.lock().unwrap();
        if session_lock.is_some() {
            eprintln!("ストリームはすでに開始されています。");
            return Err(Error::Failure);
        }

        let mut rb_lock = self.ring_buffer.lock().unwrap();
        if rb_lock.is_none() {
            let mut buffer = RingBuffer::new(Self::BLOCK_SIZE, Self::BLOCK_COUNT);
            buffer.allocate(self.device.clone(), true).map_err(|e| {
                eprintln!("buffer::RingBuffer::allocate() に失敗しました: {:?}", e);
                Error::Failure
            })?;
            for block_index in 0..Self::BLOCK_COUNT {
                let bytes = buffer.slice_block_mut(block_index);
                if bytes.is_empty() {
                    break;
                }
                bytes[0] = Self::NOT_SYNC_BYTE;
                buffer.sync_cpu(block_index).map_err(|_| Error::Failure)?;
            }
            rb_lock.replace(buffer);
        }

        let mut ring_buffer = rb_lock.take().unwrap();

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Packet, Error>>(4096);
        let (return_tx, return_rx) = std::sync::mpsc::channel();

        let stop_signal = Arc::new(AtomicBool::new(false));
        let worker_stop = stop_signal.clone();
        let device = self.device.clone();
        let isdb = self.isdb;
        let tuner = self.tuner;

        device
            .set_transfer_test_mode(isdb, tuner, false, 0, false)
            .map_err(|_| Error::Failure)?;
        device
            .set_transfer_page_descriptor_address(isdb, tuner, ring_buffer.get_descriptor_address())
            .map_err(|_| Error::Failure)?;
        device
            .set_transfer_enabled(isdb, tuner, true)
            .map_err(|_| Error::Failure)?;

        let handle = std::thread::spawn(move || {
            let mut block_index = 0;

            while !worker_stop.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(1));

                let mut next = block_index + 1;
                if Self::BLOCK_COUNT <= next {
                    next = 0;
                }

                if Self::check_ready(&mut ring_buffer, next) {
                    if let Err(e) = ring_buffer.sync_io(block_index) {
                        eprintln!("buffer::RingBuffer::sync_io() に失敗しました: {:?}", e);
                    }

                    // 1_540_096 (8192 Packet)
                    let bytes = ring_buffer.slice_block_mut(block_index);

                    let mut receiver_alive = true;

                    for chunk in bytes.chunks(Self::CHUNK_SIZE) {
                        let packet = chunk.to_vec();

                        if tx.blocking_send(Ok(packet)).is_err() {
                            receiver_alive = false;
                            break;
                        }
                    }

                    bytes[0] = Self::NOT_SYNC_BYTE;
                    if let Err(e) = ring_buffer.sync_cpu(block_index) {
                        eprintln!("buffer::RingBuffer::sync_cpu() に失敗しました: {:?}", e);
                    }

                    if !receiver_alive {
                        break;
                    }

                    block_index += 1;
                    if Self::BLOCK_COUNT <= block_index {
                        block_index = 0;
                    }
                }
            }

            // if let Ok(transfer_info) = device.get_transfer_info(isdb, tuner) {
            //     // 必要に応じてバッファあふれ等のログを出力...
            // }

            let _ = device.set_transfer_enabled(isdb, tuner, false);
            let _ = return_tx.send(ring_buffer); // リングバッファを本体へ返却
        });

        session_lock.replace(TunerSession {
            stop_signal,
            worker_handle: Some(handle),
            ring_buffer_receiver: return_rx,
        });

        let stream = ReceiverStream::new(rx);

        Ok(Box::pin(stream))
    }

    async fn stop_stream(&self) -> Result<(), Error> {
        let mut session_lock = self.active_session.lock().unwrap();

        if let Some(mut session) = session_lock.take() {
            session.stop_signal.store(true, Ordering::SeqCst);

            if let Some(handle) = session.worker_handle.take() {
                let _ = handle.join();
            }

            if let Ok(ring_buffer) = session.ring_buffer_receiver.recv() {
                self.ring_buffer.lock().unwrap().replace(ring_buffer);
            }
        }

        Ok(())
    }
}

impl Drop for Tuner {
    fn drop(&mut self) {
        // TODO: チューナーのスレッドが開いている場合は閉じる
        if let Some(mut session) = self.active_session.lock().unwrap().take() {
            session.stop_signal.store(true, Ordering::SeqCst);

            if let Some(handle) = session.worker_handle.take() {
                let _ = handle.join();
            }

            if let Ok(ring_buffer) = session.ring_buffer_receiver.recv() {
                self.ring_buffer.lock().unwrap().replace(ring_buffer);
            }
        }

        // チューナーを省電力モードにする
        _ = self
            .device
            .set_tuner_sleep(self.isdb, self.tuner, true)
            .map_err(|e| {
                eprintln!(
                    "earthsoft_pt3::pt3::Device::set_tuner_sleep() に失敗しました。: {:?}",
                    e
                );
                Error::Failure
            });

        // デバイスコンテキストにチューナーを閉じることを伝える
        let mut context = self.device_context.lock().unwrap();

        context.deactivate(self.isdb as usize, self.tuner as usize);
    }
}

// =============================================================================
// TunerSession
// =============================================================================

#[derive(Debug)]
struct TunerSession {
    stop_signal: Arc<AtomicBool>,
    worker_handle: Option<JoinHandle<()>>,
    ring_buffer_receiver: Receiver<RingBuffer>,
}
