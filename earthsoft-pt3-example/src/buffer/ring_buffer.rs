use crate::buffer;
use earthsoft_sdk::pt3;

// =============================================================================
// RingBuffer
// =============================================================================

#[derive(Debug, Default)]
pub struct RingBuffer {
    page_buffer: buffer::PageBuffer,
    ts_buffer:   buffer::TsBuffer,
}

impl RingBuffer {
    pub fn new(block_size: u32, block_count: u32) -> Self {
        Self {
            page_buffer: buffer::PageBuffer::new(block_size * block_count),
            ts_buffer: buffer::TsBuffer::new(block_size, block_count),
        }
    }

    pub fn allocate(&mut self, device: std::sync::Arc<pt3::Device>, loop_back: bool) -> Result<(), pt3::Error> {
        self.page_buffer.allocate(device.clone())?;
        self.ts_buffer.allocate(device.clone())?;

        self.page_buffer.build_page_descriptor(&self.ts_buffer, loop_back)?;
        self.page_buffer.sync_cpu()?;

        Ok(())
    }

    pub fn sync_cpu(&self, block_index: u32) -> Result<(), pt3::Error> {
        self.ts_buffer
            .sync_cpu(block_index)
            .inspect_err(|e| {
                eprintln!("buffer::TsBuffer::sync_cpu() に失敗しました: {:?}", e);
            })
    }

    pub fn sync_io(&self, block_index: u32) -> Result<(), pt3::Error> {
        self.ts_buffer
            .sync_io(block_index)
            .inspect_err(|e| {
                eprintln!("buffer::TsBuffer::sync_io() に失敗しました: {:?}", e);
            })
    }

    pub fn slice_block(&self, block_index: u32) ->  &[u8] {
        self.ts_buffer.slice_block(block_index)
    }

    pub fn slice_block_mut(&mut self, block_index: u32) ->  &mut [u8] {
        self.ts_buffer.slice_block_mut(block_index)
    }

    pub fn get_descriptor_address(&self) -> u64 {
        self.page_buffer.get_descriptor_address()
    }
}

unsafe impl Send for RingBuffer {}
unsafe impl Sync for RingBuffer {}
