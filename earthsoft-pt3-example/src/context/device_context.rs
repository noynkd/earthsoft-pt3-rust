use crate::context;
use earthsoft_sdk::pt3;

// =============================================================================
// DeviceContext
// =============================================================================

#[derive(Debug)]
pub struct DeviceContext {
    device: pt3::Device,
    #[allow(dead_code)]
    bus_context: std::sync::Arc<context::BusContext>,
    is_open: std::sync::atomic::AtomicBool,
}

impl DeviceContext {
    pub(in crate::context) fn new(device: pt3::Device, bus_context: std::sync::Arc<context::BusContext>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            device,
            bus_context: bus_context,
            is_open: std::sync::atomic::AtomicBool::new(false),
        })
    }

    pub fn create_tuner(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> context::TunerContext {
        context::TunerContext::new(
            isdb,
            tuner,
            self.clone(),
        )
    }

    pub fn open(self: &std::sync::Arc<Self>) -> Result<(), pt3::Error> {
        if self.is_open.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(())
        }

        self.device.open()
            .map(|()| {
                self.is_open.store(true, std::sync::atomic::Ordering::SeqCst);
            })
    }

    // pub fn close(self: &std::sync::Arc<Self>) -> Result<(), pt3::Error> {
    //     if !self.is_open.load(std::sync::atomic::Ordering::SeqCst) {
    //         return Ok(())
    //     }

    //     self.device.close()
    //         .map(|()| {
    //             self.is_open.store(false, std::sync::atomic::Ordering::SeqCst);
    //         })
    // }

    pub fn get_constant_info(self: &std::sync::Arc<Self>) -> Result<pt3::ConstantInfo, pt3::Error> {
        self.device.get_constant_info()
    }

    pub fn set_lnb_power(self: &std::sync::Arc<Self>, power: pt3::LnbPower) -> Result<(), pt3::Error> {
        self.device.set_lnb_power(power)
    }

    pub fn get_lnb_power(self: &std::sync::Arc<Self>) -> Result<pt3::LnbPower, pt3::Error> {
        self.device.get_lnb_power()
    }

    // pub fn set_lnb_power_when_close(self: &std::sync::Arc<Self>, power: pt3::LnbPower) -> Result<(), pt3::Error> {
    //     self.device.set_lnb_power_when_close(power)
    // }

    // pub fn get_lnb_power_when_close(self: &std::sync::Arc<Self>) -> Result<pt3::LnbPower, pt3::Error> {
    //     self.device.get_lnb_power_when_close()
    // }

    pub fn init_tuner(self: &std::sync::Arc<Self>) -> Result<(), pt3::Error> {
        self.device.init_tuner()
    }

    pub fn set_tuner_sleep(self: &std::sync::Arc<Self>, isdb:pt3::Isdb, tuner: u32, sleep: bool) -> Result<(), pt3::Error> {
        self.device.set_tuner_sleep(isdb, tuner, sleep)
    }

    pub fn get_tuner_sleep(self: &std::sync::Arc<Self>, isdb:pt3::Isdb, tuner: u32) -> Result<bool, pt3::Error> {
        self.device.get_tuner_sleep(isdb, tuner)
    }

    pub fn set_frequency(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, channel: u32, offset: i32) -> Result<(), pt3::Error> {
        self.device.set_frequency(isdb, tuner, channel, offset)
    }

    // pub fn get_frequency(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<(u32, i32), pt3::Error> {
    //     self.device.get_frequency(isdb, tuner)
    // }

    pub fn get_frequency_offset(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<(i32, i32), pt3::Error> {
        self.device.get_frequency_offset(isdb, tuner)
    }

    pub fn get_cn_agc(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<(u32, u32, u32), pt3::Error> {
        self.device.get_cn_agc(isdb, tuner)
    }

    pub fn get_rf_level(self: &std::sync::Arc<Self>, tuner: u32) -> Result<f32, pt3::Error> {
        self.device.get_rf_level(tuner)
    }

    pub fn set_satellite_id(self: &std::sync::Arc<Self>, tuner: u32, id: u32) -> Result<(), pt3::Error> {
        self.device.set_satellite_id(tuner, id)
    }

    // pub fn get_satellite_id(self: &std::sync::Arc<Self>, tuner: u32) -> Result<u32, pt3::Error> {
    //     self.device.get_satellite_id(tuner)
    // }

    // pub fn set_inner_error_rate_layer(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, layer: u32) -> Result<(), pt3::Error> {
    //     self.device.set_inner_error_rate_layer(isdb, tuner, layer)
    // }

    pub fn get_inner_error_rate(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<pt3::ErrorRate, pt3::Error> {
        self.device.get_inner_error_rate(isdb, tuner)
    }

    pub fn get_corrected_error_rate(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, layer: u32) -> Result<pt3::ErrorRate, pt3::Error> {
        self.device.get_corrected_error_rate(isdb, tuner, layer)
    }

    pub fn reset_corrected_error_count(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<(), pt3::Error> {
        self.device.reset_corrected_error_count(isdb, tuner)
    }

    pub fn get_error_count(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<u32, pt3::Error> {
        self.device.get_error_count(isdb, tuner)
    }

    pub fn get_satellite_tmcc(self: &std::sync::Arc<Self>, tuner: u32) -> Result<pt3::SatelliteTmcc, pt3::Error> {
        self.device.get_satellite_tmcc(tuner)
    }

    // pub fn get_satellite_layer(self: &std::sync::Arc<Self>, tuner: u32) -> Result<pt3::SatelliteLayer, pt3::Error> {
    //     self.device.get_satellite_layer(tuner)
    // }

    pub fn get_terrestrial_tmcc(self: &std::sync::Arc<Self>, tuner: u32) -> Result<pt3::TerrestrialTmcc, pt3::Error> {
        self.device.get_terrestrial_tmcc(tuner)
    }

    pub fn set_amp_power(self: &std::sync::Arc<Self>, power: bool) -> Result<(), pt3::Error> {
        self.device.set_amp_power(power)
    }

    // pub fn set_layer_enable(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, layer_mask: u32) -> Result<(), pt3::Error> {
    //     self.device.set_layer_enable(isdb, tuner, layer_mask)
    // }

    // pub fn get_layer_enable(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<u32, pt3::Error> {
    //     self.device.get_layer_enable(isdb, tuner)
    // }

    pub fn set_ts_pins_mode(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, mode: pt3::TsPinsMode) -> Result<(), pt3::Error> {
        self.device.set_ts_pins_mode(isdb, tuner, mode)
    }

    pub fn get_ts_pins_level(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<pt3::TsPinsLevel, pt3::Error> {
        self.device.get_ts_pins_level(isdb, tuner)
    }

    pub fn get_ts_sync_byte(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<u8, pt3::Error> {
        self.device.get_ts_sync_byte(isdb, tuner)
    }

    pub fn set_ram_pins_mode(self: &std::sync::Arc<Self>, mode: pt3::RamPinsMode) -> Result<(), pt3::Error> {
        self.device.set_ram_pins_mode(mode)
    }

    pub fn lock_buffer(self: &std::sync::Arc<Self>, buffer: &mut [u8], direction: pt3::TransferDirection) -> Result<pt3::BufferHandle, pt3::Error> {
        self.device.lock_buffer(buffer, direction)
    }

    pub fn unlock_buffer(self: &std::sync::Arc<Self>, handle: &pt3::BufferHandle) -> Result<(), pt3::Error> {
        self.device.unlock_buffer(handle)
    }

    pub fn get_buffer_info(self: &std::sync::Arc<Self>, handle: &pt3::BufferHandle) -> Result<Vec<pt3::BufferInfo>, pt3::Error> {
        self.device.get_buffer_info(handle)
    }

    pub fn set_transfer_page_descriptor_address(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, page_descriptor_address: u64) -> Result<(), pt3::Error> {
        self.device.set_transfer_page_descriptor_address(isdb, tuner, page_descriptor_address)
    }

    pub fn set_transfer_enabled(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, enable: bool) -> Result<(), pt3::Error> {
        self.device.set_transfer_enabled(isdb, tuner, enable)
    }

    pub fn get_transfer_enabled(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<bool, pt3::Error> {
        self.device.get_transfer_enabled(isdb, tuner)
    }

    pub fn set_transfer_test_mode(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32, test_mode: bool, initial: u16, not_op: bool) -> Result<(), pt3::Error> {
        self.device.set_transfer_test_mode(isdb, tuner, test_mode, initial, not_op)
    }

    pub fn get_transfer_info(self: &std::sync::Arc<Self>, isdb: pt3::Isdb, tuner: u32) -> Result<pt3::TransferInfo, pt3::Error> {
        self.device.get_transfer_info(isdb, tuner)
    }

    pub fn sync_buffer_cpu(self: &std::sync::Arc<Self>, handle: &pt3::BufferHandle) -> Result<(), pt3::Error> {
        self.device.sync_buffer_cpu(handle)
    }

    pub fn sync_buffer_io(self: &std::sync::Arc<Self>, handle: &pt3::BufferHandle) -> Result<(), pt3::Error> {
        self.device.sync_buffer_io(handle)
    }

}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        if *self.is_open.get_mut() {
            _ = self.device.close();
        }

        _ = self.device.delete();
    }
}

unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}
