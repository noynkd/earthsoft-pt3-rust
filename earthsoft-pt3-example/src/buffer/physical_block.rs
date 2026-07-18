use earthsoft_sdk::pt3;

// =============================================================================
// PhysicalBlock
// =============================================================================

#[derive(Debug)]
pub struct PhysicalBlock {
    handle: pt3::BufferHandle,
    buffer_info: Vec<pt3::BufferInfo>,
}

impl PhysicalBlock {
    pub fn new(device: std::sync::Arc<pt3::Device>, buffer: &mut [u8], direction: pt3::TransferDirection) -> Result<Self, pt3::Error> {
        let handle = device.lock_buffer(buffer, direction)?;

        let buffer_info = handle.get_buffer_info()
            .inspect_err(|e| {
                eprintln!("{}", e);
            })?;

        Ok(Self {
            handle,
            buffer_info,
        })
    }

    pub fn sync_cpu(&self) -> Result<(), pt3::Error> {
        self.handle.sync_buffer_cpu()
    }

    pub fn sync_io(&self) -> Result<(), pt3::Error> {
        self.handle.sync_buffer_io()
    }

    pub fn get_buffer_info(&self) -> &Vec<pt3::BufferInfo> {
        &self.buffer_info
    }
}
