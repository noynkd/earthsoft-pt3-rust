const SATELLITE_LAYER_COUNT: usize = 2;
const TERRESTRIAL_LAYER_COUNT: usize = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3BufferInfo {
    pub address: u64,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3ConstantInfo {
    pub pt_version: u8,
    pub register_map_version: u8,
    pub fpga_version: u8,
    pub is_ts_supported: u8,
    pub page_descriptor_size_bits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3ErrorRate {
    pub numerator: u32,
    pub denominator: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3SatelliteLayer {
    pub mode: [u32; SATELLITE_LAYER_COUNT],
    pub count: [u32; SATELLITE_LAYER_COUNT],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3SatelliteTmcc {
    pub indicator: u32,
    pub mode: [u32; 4],
    pub slot: [u32; 4],
    pub id: [u32; 8],
    pub emergency: u32,
    pub up_link: u32,
    pub ext_flag: u32,
    pub ext_data: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3TerrestrialTmcc {
    pub system: u32,
    pub indicator: u32,
    pub emergency: u32,
    pub partial: u32,
    pub mode: [u32; TERRESTRIAL_LAYER_COUNT],
    pub rate: [u32; TERRESTRIAL_LAYER_COUNT],
    pub interleave: [u32; TERRESTRIAL_LAYER_COUNT],
    pub segment: [u32; TERRESTRIAL_LAYER_COUNT],
    pub phase: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3TsPinsLevel {
    pub clock: u8,
    pub data: u8,
    pub byte: u8,
    pub valid: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3TsPinsMode {
    pub clock_data: u32,
    pub byte: u32,
    pub valid: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3TransferInfo {
    pub busy: u8,
    pub status: u32,
    pub internal_fifo_a_overflow: u8,
    pub internal_fifo_a_underflow: u8,
    pub external_fifo_overflow: u8,
    pub external_fifo_max_used_bytes: u32,
    pub internal_fifo_b_overflow: u8,
    pub internal_fifo_b_underflow: u8,
}

#[derive(Debug)]
pub enum RawPt3Device {}

#[link(name = "earthsoft_pt3", kind = "static")]
unsafe extern "C" {
    fn DeletePt3Device(device: *mut RawPt3Device) -> i32;
    fn OpenPt3Device(device: *mut RawPt3Device) -> i32;
    fn ClosePt3Device(device: *mut RawPt3Device) -> i32;
    fn GetPt3ConstantInfo(device: *mut RawPt3Device, constantInfo: *mut RawPt3ConstantInfo) -> i32;
    fn SetPt3LnbPower(device: *mut RawPt3Device, power: u32) -> i32;
    fn GetPt3LnbPower(device: *mut RawPt3Device, power: *mut u32) -> i32;
    fn SetPt3LnbPowerWhenClose(device: *mut RawPt3Device, power: u32) -> i32;
    fn GetPt3LnbPowerWhenClose(device: *mut RawPt3Device, power: *mut u32) -> i32;
    fn InitPt3Tuner(device: *mut RawPt3Device) -> i32;
    fn SetPt3TunerSleep(device: *mut RawPt3Device, isdb: u32, tuner: u32, sleep: u8) -> i32;
    fn GetPt3TunerSleep(device: *mut RawPt3Device, isdb: u32, tuner: u32, sleep: *mut u8) -> i32;
    fn SetPt3Frequency(device: *mut RawPt3Device, isdb: u32, tuner: u32, channel: u32, offset: i32) -> i32;
    fn GetPt3Frequency(device: *mut RawPt3Device, isdb: u32, tuner: u32, channel: *mut u32, offset: *mut i32) -> i32;
    fn GetPt3FrequencyOffset(device: *mut RawPt3Device, isdb: u32, tuner: u32, clock: *mut i32, offset: *mut i32) -> i32;
    fn GetPt3CnAgc(device: *mut RawPt3Device, isdb: u32, tuner: u32, cn100: *mut u32, currentAgc: *mut u32, maxAgc: *mut u32) -> i32;
    fn GetPt3RfLevel(device: *mut RawPt3Device, tuner: u32, level: *mut f32) -> i32;
    fn SetPt3SatelliteId(device: *mut RawPt3Device, tuner: u32, id: u32) -> i32;
    fn GetPt3SatelliteId(device: *mut RawPt3Device, tuner: u32, id: *mut u32) -> i32;
    fn SetPt3InnerErrorRateLayer(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer: u32) -> i32;
    fn GetPt3InnerErrorRate(device: *mut RawPt3Device, isdb: u32, tuner: u32, errorRate: *mut RawPt3ErrorRate) -> i32;
    fn GetPt3CorrectedErrorRate(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer: u32, errorRate: *mut RawPt3ErrorRate) -> i32;
    fn ResetPt3CorrectedErrorCount(device: *mut RawPt3Device, isdb: u32, tuner: u32) -> i32;
    fn GetPt3ErrorCount(device: *mut RawPt3Device, isdb: u32, tuner: u32, count: *mut u32) -> i32;
    fn GetPt3SatelliteTmcc(device: *mut RawPt3Device, tuner: u32, tmcc: *mut RawPt3SatelliteTmcc) -> i32;
    fn GetPt3SatelliteLayer(device: *mut RawPt3Device, tuner: u32, layer: *mut RawPt3SatelliteLayer) -> i32;
    fn GetPt3TerrestrialTmcc(device: *mut RawPt3Device, tuner: u32, tmcc: *mut RawPt3TerrestrialTmcc) -> i32;
    fn SetPt3AmpPower(device: *mut RawPt3Device, power: u8) -> i32;
    fn SetPt3LayerEnable(device: *mut RawPt3Device, isdb: u32, tuner: u32, layerMask: u32) -> i32;
    fn GetPt3LayerEnable(device: *mut RawPt3Device, isdb: u32, tuner: u32, layerMask: *mut u32) -> i32;
    fn SetPt3TsPinsMode(device: *mut RawPt3Device, isdb: u32, tuner: u32, mode: *const RawPt3TsPinsMode) -> i32;
    fn GetPt3TsPinsLevel(device: *mut RawPt3Device, isdb: u32, tuner: u32, level: *mut RawPt3TsPinsLevel) -> i32;
    fn GetPt3TsSyncByte(device: *mut RawPt3Device, isdb: u32, tuner: u32, syncByte: *mut u8) -> i32;
    fn SetPt3RamPinsMode(device: *mut RawPt3Device, mode: u32) -> i32;
    fn UnlockPt3Buffer(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32;
    fn GetPt3BufferInfo(device: *mut RawPt3Device, handle: *mut std::ffi::c_void, infoTable: *mut *const RawPt3BufferInfo, infoCount: *mut u32) -> i32;
    fn SetPt3TransferPageDescriptorAddress(device: *mut RawPt3Device, isdb: u32, tuner: u32, pageDescriptorAddress: u64) -> i32;
    fn SetPt3TransferEnabled(device: *mut RawPt3Device, isdb: u32, tuner: u32, enabled: u8) -> i32;
    fn GetPt3TransferEnabled(device: *mut RawPt3Device, isdb: u32, tuner: u32, enabled: *mut u8) -> i32;
    fn SetPt3TransferTestMode(device: *mut RawPt3Device, isdb: u32, tuner: u32, testMode: u8, initial: u16, notOp: u8) -> i32;
    fn GetPt3TransferInfo(device: *mut RawPt3Device, isdb: u32, tuner: u32, transferInfo: *mut RawPt3TransferInfo) -> i32;
    fn LockPt3Buffer(device: *mut RawPt3Device, ptr: *mut std::ffi::c_void, size: u32, direction: u32, handle: *mut *mut std::ffi::c_void) -> i32;
    fn SyncPt3BufferCpu(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32;
    fn SyncPt3BufferIo(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32;
}

pub unsafe fn delete_pt3_device(device: *mut RawPt3Device) -> i32 {
    unsafe { DeletePt3Device(device) }
}

pub unsafe fn open_pt3_device(device: *mut RawPt3Device) -> i32 {
    unsafe { OpenPt3Device(device) }
}

pub unsafe fn close_pt3_device(device: *mut RawPt3Device) -> i32 {
    unsafe { ClosePt3Device(device) }
}

pub unsafe fn get_pt3_constant_info(device: *mut RawPt3Device, constant_info: *mut RawPt3ConstantInfo) -> i32 {
    unsafe { GetPt3ConstantInfo(device, constant_info) }
}

pub unsafe fn set_pt3_lnb_power(device: *mut RawPt3Device, power: u32) -> i32 {
    unsafe { SetPt3LnbPower(device, power) }
}

pub unsafe fn get_pt3_lnb_power(device: *mut RawPt3Device, power: *mut u32) -> i32 {
    unsafe { GetPt3LnbPower(device, power) }
}

pub unsafe fn set_pt3_lnb_power_when_close(device: *mut RawPt3Device, power: u32) -> i32 {
    unsafe { SetPt3LnbPowerWhenClose(device, power) }
}

pub unsafe fn get_pt3_lnb_power_when_close(device: *mut RawPt3Device, power: *mut u32) -> i32 {
    unsafe { GetPt3LnbPowerWhenClose(device, power) }
}

pub unsafe fn init_pt3_tuner(device: *mut RawPt3Device) -> i32 {
    unsafe { InitPt3Tuner(device) }
}

pub unsafe fn set_pt3_tuner_sleep(device: *mut RawPt3Device, isdb: u32, tuner: u32, sleep: u8) -> i32 {
    unsafe { SetPt3TunerSleep(device, isdb, tuner, sleep) }
}

pub unsafe fn get_pt3_tuner_sleep(device: *mut RawPt3Device, isdb: u32, tuner: u32, sleep: *mut u8) -> i32 {
    unsafe { GetPt3TunerSleep(device, isdb, tuner, sleep) }
}

pub unsafe fn set_pt3_frequency(device: *mut RawPt3Device, isdb: u32, tuner: u32, channel: u32, offset: i32) -> i32 {
    unsafe { SetPt3Frequency(device, isdb, tuner, channel, offset) }
}

pub unsafe fn get_pt3_frequency(device: *mut RawPt3Device, isdb: u32, tuner: u32, channel: *mut u32, offset: *mut i32) -> i32 {
    unsafe { GetPt3Frequency(device, isdb, tuner, channel, offset) }
}

pub unsafe fn get_pt3_frequency_offset(device: *mut RawPt3Device, isdb: u32, tuner: u32, clock: *mut i32, offset: *mut i32) -> i32 {
    unsafe { GetPt3FrequencyOffset(device, isdb, tuner, clock, offset) }
}

pub unsafe fn get_pt3_cn_agc(device: *mut RawPt3Device, isdb: u32, tuner: u32, cn100: *mut u32, current_agc: *mut u32, max_agc: *mut u32) -> i32 {
    unsafe { GetPt3CnAgc(device, isdb, tuner, cn100, current_agc, max_agc) }
}

pub unsafe fn get_pt3_rf_level(device: *mut RawPt3Device, tuner: u32, level: *mut f32) -> i32 {
    unsafe { GetPt3RfLevel(device, tuner, level) }
}

pub unsafe fn set_pt3_satellite_id(device: *mut RawPt3Device, tuner: u32, id: u32) -> i32 {
    unsafe { SetPt3SatelliteId(device, tuner, id) }
}

pub unsafe fn get_pt3_satellite_id(device: *mut RawPt3Device, tuner: u32, id: *mut u32) -> i32 {
    unsafe { GetPt3SatelliteId(device, tuner, id) }
}

pub unsafe fn set_pt3_inner_error_rate_layer(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer: u32) -> i32 {
    unsafe { SetPt3InnerErrorRateLayer(device, isdb, tuner, layer) }
}

pub unsafe fn get_pt3_inner_error_rate(device: *mut RawPt3Device, isdb: u32, tuner: u32, error_rate: *mut RawPt3ErrorRate) -> i32 {
    unsafe { GetPt3InnerErrorRate(device, isdb, tuner, error_rate) }
}

pub unsafe fn get_pt3_corrected_error_rate(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer: u32, error_rate: *mut RawPt3ErrorRate) -> i32 {
    unsafe { GetPt3CorrectedErrorRate(device, isdb, tuner, layer, error_rate) }
}

pub unsafe fn reset_pt3_corrected_error_count(device: *mut RawPt3Device, isdb: u32, tuner: u32) -> i32 {
    unsafe { ResetPt3CorrectedErrorCount(device, isdb, tuner) }
}

pub unsafe fn get_pt3_error_count(device: *mut RawPt3Device, isdb: u32, tuner: u32, count: *mut u32) -> i32 {
    unsafe { GetPt3ErrorCount(device, isdb, tuner, count) }
}

pub unsafe fn get_pt3_satellite_tmcc(device: *mut RawPt3Device, tuner: u32, tmcc: *mut RawPt3SatelliteTmcc) -> i32 {
    unsafe { GetPt3SatelliteTmcc(device, tuner, tmcc) }
}

pub unsafe fn get_pt3_satellite_layer(device: *mut RawPt3Device, tuner: u32, layer: *mut RawPt3SatelliteLayer) -> i32 {
    unsafe { GetPt3SatelliteLayer(device, tuner, layer) }
}

pub unsafe fn get_pt3_terrestrial_tmcc(device: *mut RawPt3Device, tuner: u32, tmcc: *mut RawPt3TerrestrialTmcc) -> i32 {
    unsafe { GetPt3TerrestrialTmcc(device, tuner, tmcc) }
}

pub unsafe fn set_pt3_amp_power(device: *mut RawPt3Device, power: u8) -> i32 {
    unsafe { SetPt3AmpPower(device, power) }
}

pub unsafe fn set_pt3_layer_enable(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer_mask: u32) -> i32 {
    unsafe { SetPt3LayerEnable(device, isdb, tuner, layer_mask) }
}

pub unsafe fn get_pt3_layer_enable(device: *mut RawPt3Device, isdb: u32, tuner: u32, layer_mask: *mut u32) -> i32 {
    unsafe { GetPt3LayerEnable(device, isdb, tuner, layer_mask) }
}

pub unsafe fn set_pt3_ts_pins_mode(device: *mut RawPt3Device, isdb: u32, tuner: u32, mode: *const RawPt3TsPinsMode) -> i32 {
    unsafe { SetPt3TsPinsMode(device, isdb, tuner, mode) }
}

pub unsafe fn get_pt3_ts_pins_level(device: *mut RawPt3Device, isdb: u32, tuner: u32, level: *mut RawPt3TsPinsLevel) -> i32 {
    unsafe { GetPt3TsPinsLevel(device, isdb, tuner, level) }
}

pub unsafe fn get_pt3_ts_sync_byte(device: *mut RawPt3Device, isdb: u32, tuner: u32, sync_byte: *mut u8) -> i32 {
    unsafe { GetPt3TsSyncByte(device, isdb, tuner, sync_byte) }
}

pub unsafe fn set_pt3_ram_pins_mode(device: *mut RawPt3Device, mode: u32) -> i32 {
    unsafe { SetPt3RamPinsMode(device, mode) }
}

pub unsafe fn unlock_pt3_buffer(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32 {
    unsafe { UnlockPt3Buffer(device, handle) }
}

pub unsafe fn get_pt3_buffer_info(device: *mut RawPt3Device, handle: *mut std::ffi::c_void, info_table: *mut *const RawPt3BufferInfo, info_count: *mut u32) -> i32 {
    unsafe { GetPt3BufferInfo(device, handle, info_table, info_count) }
}

pub unsafe fn set_pt3_transfer_page_descriptor_address(device: *mut RawPt3Device, isdb: u32, tuner: u32, page_descriptor_address: u64) -> i32 {
    unsafe { SetPt3TransferPageDescriptorAddress(device, isdb, tuner, page_descriptor_address) }
}

pub unsafe fn set_pt3_transfer_enabled(device: *mut RawPt3Device, isdb: u32, tuner: u32, enabled: u8) -> i32 {
    unsafe { SetPt3TransferEnabled(device, isdb, tuner, enabled) }
}

pub unsafe fn get_pt3_transfer_enabled(device: *mut RawPt3Device, isdb: u32, tuner: u32, enabled: *mut u8) -> i32 {
    unsafe { GetPt3TransferEnabled(device, isdb, tuner, enabled) }
}

pub unsafe fn set_pt3_transfer_test_mode(device: *mut RawPt3Device, isdb: u32, tuner: u32, test_mode: u8, initial: u16, not_op: u8) -> i32 {
    unsafe { SetPt3TransferTestMode(device, isdb, tuner, test_mode, initial, not_op) }
}

pub unsafe fn get_pt3_transfer_info(device: *mut RawPt3Device, isdb: u32, tuner: u32, transfer_info: *mut RawPt3TransferInfo) -> i32 {
    unsafe { GetPt3TransferInfo(device, isdb, tuner, transfer_info) }
}

pub unsafe fn lock_pt3_buffer(device: *mut RawPt3Device, ptr: *mut std::ffi::c_void, size: u32, direction: u32, handle: *mut *mut std::ffi::c_void) -> i32 {
    unsafe { LockPt3Buffer(device, ptr, size, direction, handle) }
}

pub unsafe fn sync_pt3_buffer_cpu(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32 {
    unsafe { SyncPt3BufferCpu(device, handle) }
}

pub unsafe fn sync_pt3_buffer_io(device: *mut RawPt3Device, handle: *mut std::ffi::c_void) -> i32 {
    unsafe { SyncPt3BufferIo(device, handle) }
}
