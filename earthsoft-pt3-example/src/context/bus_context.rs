use crate::context;
use earthsoft_sdk::pt3;

// =============================================================================
// BusContext
// =============================================================================

#[derive(Debug)]
pub struct BusContext {
    pt3: pt3::Pt3,
    bus: pt3::Bus,
}

impl BusContext {
    pub fn new() -> Result<std::sync::Arc<Self>, pt3::Error> {
        let pt3 = pt3::Pt3::new()?;
        let bus = pt3.create_bus()?;

        Ok(std::sync::Arc::new(Self {
            pt3,
            bus,
        }))
    }

    // pub fn get_version(self: &std::sync::Arc<Self>) -> Result<u32, pt3::Error> {
    //     self.bus.get_version()
    // }

    pub fn scan_device_info(self: &std::sync::Arc<Self>, max_device_count: usize) -> Result<Vec<pt3::DeviceInfo>, pt3::Error> {
        self.bus.scan_device_info(max_device_count)
    }

    pub fn create_device(self: &std::sync::Arc<Self>, device_info: &pt3::DeviceInfo) -> Result<std::sync::Arc<context::DeviceContext>, pt3::Error> {
        let device = self.bus.create_device(device_info)?;

        Ok(context::DeviceContext::new(
            device,
            self.clone(),
        ))
    }
}

impl Drop for BusContext {
    fn drop(&mut self) {
        _ = self.bus.delete();
        _ = self.pt3.delete();
    }
}
