use crate::context;
use earthsoft_sdk::pt3;

// =============================================================================
// PhysicalBlock
// =============================================================================

#[derive(Debug)]
pub struct PhysicalBlock {
    device: std::sync::Arc<context::DeviceContext>,
    handle: pt3::BufferHandle,
    buffer_info: Vec<pt3::BufferInfo>,
}

impl PhysicalBlock {
    pub fn new(device: std::sync::Arc<context::DeviceContext>, buffer: &mut [u8], direction: pt3::TransferDirection) -> Result<Self, pt3::Error> {
        let handle = device.lock_buffer(buffer, direction)?;

        let buffer_info = device.get_buffer_info(&handle)
            .inspect_err(|_| {
                _ = device.unlock_buffer(&handle);
            })?;

        Ok(Self {
            device,
            handle,
            buffer_info,
        })
    }

    pub fn sync_cpu(&self) -> Result<(), pt3::Error> {
        self.device.sync_buffer_cpu(&self.handle)
    }

    pub fn sync_io(&self) -> Result<(), pt3::Error> {
        self.device.sync_buffer_io(&self.handle)
    }

    pub fn get_buffer_info(&self) -> &Vec<pt3::BufferInfo> {
        &self.buffer_info
    }
}

impl Drop for PhysicalBlock {
    fn drop(&mut self) {
        _ = self.device.unlock_buffer(&self.handle);
    }
}
