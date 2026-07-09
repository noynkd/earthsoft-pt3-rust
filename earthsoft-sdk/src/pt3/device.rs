use crate::pt3;
use earthsoft_pt3_sys::ffi;

// =============================================================================
// Device
// =============================================================================

#[derive(Debug)]
pub struct Device {
    raw_ptr: *mut ffi::RawPt3Device,
}

impl Device {
    pub(crate) fn new(raw_ptr: *mut ffi::RawPt3Device) -> Self {
        Self {
            raw_ptr: raw_ptr,
        }
    }

    pub fn delete(&self) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::delete_pt3_device(self.raw_ptr)
        })
        .check_result()
    }

    pub fn open(&self) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::open_pt3_device(self.raw_ptr)
        })
        .check_result()
    }

    pub fn close(&self) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::close_pt3_device(self.raw_ptr)
        })
        .check_result()
    }

    pub fn get_constant_info(&self) -> Result<ConstantInfo, pt3::Error> {
        let mut raw_constant_info = std::mem::MaybeUninit::<ffi::RawPt3ConstantInfo>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_constant_info(
                self.raw_ptr,
                raw_constant_info.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_constant_info.assume_init()
            }
            .into()
        })
    }

    pub fn set_lnb_power(&self, power: LnbPower) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_lnb_power(
                self.raw_ptr,
                power as u32,
            )
        })
        .check_result()
    }

    pub fn get_lnb_power(&self) -> Result<LnbPower, pt3::Error> {
        let mut raw_lnb_power = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_lnb_power(
                self.raw_ptr,
                &mut raw_lnb_power,
            )
        })
        .check_result()
        .and_then(|_| {
            raw_lnb_power
                .try_into()
                .map_err(|_| pt3::Error::InternalError)
        })
    }

    pub fn set_lnb_power_when_close(&self, power: LnbPower) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_lnb_power_when_close(
                self.raw_ptr,
                power as u32,
            )
        })
        .check_result()
    }

    pub fn get_lnb_power_when_close(&self) -> Result<LnbPower, pt3::Error> {
        let mut raw_lnb_power = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_lnb_power_when_close(
                self.raw_ptr,
                &mut raw_lnb_power,
            )
        })
        .check_result()
        .and_then(|_| {
            raw_lnb_power
                .try_into()
                .map_err(|_| pt3::Error::InternalError)
        })
    }

    pub fn init_tuner(&self) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::init_pt3_tuner(
                self.raw_ptr,
            )
        })
        .check_result()
    }

    pub fn set_tuner_sleep(&self, isdb: Isdb, tuner: u32, sleep: bool) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_tuner_sleep(
                self.raw_ptr,
                isdb as u32,
                tuner,
                sleep as u8,
            )
        })
        .check_result()
    }

    pub fn get_tuner_sleep(&self, isdb: Isdb, tuner: u32) -> Result<bool, pt3::Error> {
        let mut raw_sleep = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_tuner_sleep(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_sleep,
            )
        })
        .check_result()
        .map(|_| {
            raw_sleep != 0
        })
    }

    pub fn set_frequency(&self, isdb: Isdb, tuner: u32, channel: u32, offset: i32) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_frequency(
                self.raw_ptr,
                isdb as u32,
                tuner,
                channel,
                offset,
            )
        })
        .check_result()
    }

    // TODO: 構造体を使用して返すようにするか検討する
    pub fn get_frequency(&self, isdb: Isdb, tuner: u32) -> Result<(u32, i32), pt3::Error> {
        let mut raw_channel = 0;
        let mut raw_offset = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_frequency(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_channel,
                &mut raw_offset,
            )
        })
        .check_result()
        .map(|_| {
            (raw_channel, raw_offset)
        })
    }

    // TODO: 構造体を使用して返すようにするか検討する
    pub fn get_frequency_offset(&self, isdb: Isdb, tuner: u32) -> Result<(i32, i32), pt3::Error> {
        let mut raw_clock = 0;
        let mut raw_offset = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_frequency_offset(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_clock,
                &mut raw_offset,
            )
        })
        .check_result()
        .map(|_| {
            (raw_clock, raw_offset)
        })
    }

    // TODO: 構造体を使用して返すようにするか検討する
    pub fn get_cn_agc(&self, isdb: Isdb, tuner: u32) -> Result<(u32, u32, u32), pt3::Error> {
        let mut raw_cn100 = 0;
        let mut raw_current_agc = 0;
        let mut raw_max_agc = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_cn_agc(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_cn100,
                &mut raw_current_agc,
                &mut raw_max_agc,
            )
        })
        .check_result()
        .map(|_| {
            (raw_cn100, raw_current_agc, raw_max_agc)
        })
    }

    pub fn get_rf_level(&self, tuner: u32) -> Result<f32, pt3::Error> {
        let mut raw_rf_level = 0.0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_rf_level(
                self.raw_ptr,
                tuner,
                &mut raw_rf_level,
            )
        })
        .check_result()
        .map(|_| {
            raw_rf_level
        })
    }

    pub fn set_satellite_id(&self, tuner: u32, id: u32) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_satellite_id(
                self.raw_ptr,
                tuner,
                id,
            )
        })
        .check_result()
    }

    pub fn get_satellite_id(&self, tuner: u32) -> Result<u32, pt3::Error> {
        let mut raw_id = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_satellite_id(
                self.raw_ptr,
                tuner,
                &mut raw_id,
            )
        })
        .check_result()
        .map(|_| {
            raw_id
        })
    }

    pub fn set_inner_error_rate_layer(&self, isdb: Isdb, tuner: u32, layer: u32) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_inner_error_rate_layer(
                self.raw_ptr,
                isdb as u32,
                tuner,
                layer,
            )
        })
        .check_result()
    }

    pub fn get_inner_error_rate(&self, isdb: Isdb, tuner: u32) -> Result<ErrorRate, pt3::Error> {
        let mut raw_error_rate = std::mem::MaybeUninit::<ffi::RawPt3ErrorRate>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_inner_error_rate(
                self.raw_ptr,
                isdb as u32,
                tuner,
                raw_error_rate.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_error_rate.assume_init()
            }
            .into()
        })
    }

    pub fn get_corrected_error_rate(&self, isdb: Isdb, tuner: u32, layer: u32) -> Result<ErrorRate, pt3::Error> {
        let mut raw_error_rate = std::mem::MaybeUninit::<ffi::RawPt3ErrorRate>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_corrected_error_rate(
                self.raw_ptr,
                isdb as u32,
                tuner,
                layer,
                raw_error_rate.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_error_rate.assume_init()
            }
            .into()
        })
    }

    pub fn reset_corrected_error_count(&self, isdb: Isdb, tuner: u32) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::reset_pt3_corrected_error_count(
                self.raw_ptr,
                isdb as u32,
                tuner,
            )
        })
        .check_result()
    }

    pub fn get_error_count(&self, isdb: Isdb, tuner: u32) -> Result<u32, pt3::Error> {
        let mut raw_error_count = 0u32;

        pt3::Error::from(unsafe {
            ffi::get_pt3_error_count(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_error_count,
            )
        })
        .check_result()
        .map(|_| {
            raw_error_count
        })
    }

    pub fn get_satellite_tmcc(&self, tuner: u32) -> Result<SatelliteTmcc, pt3::Error> {
        let mut raw_tmcc = std::mem::MaybeUninit::<ffi::RawPt3SatelliteTmcc>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_satellite_tmcc(
                self.raw_ptr,
                tuner,
                raw_tmcc.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_tmcc.assume_init()
            }
            .into()
        })
    }

    pub fn get_satellite_layer(&self, tuner: u32) -> Result<SatelliteLayer, pt3::Error> {
        let mut raw_layer = std::mem::MaybeUninit::<ffi::RawPt3SatelliteLayer>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_satellite_layer(
                self.raw_ptr,
                tuner,
                raw_layer.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_layer.assume_init()
            }
            .into()
        })
    }

    pub fn get_terrestrial_tmcc(&self, tuner: u32) -> Result<TerrestrialTmcc, pt3::Error> {
        let mut raw_tmcc = std::mem::MaybeUninit::<ffi::RawPt3TerrestrialTmcc>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_terrestrial_tmcc(
                self.raw_ptr,
                tuner,
                raw_tmcc.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_tmcc.assume_init()
            }
            .into()
        })
    }

    pub fn set_amp_power(&self, power: bool) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_amp_power(
                self.raw_ptr,
                power as u8,
            )
        })
        .check_result()
    }

    pub fn set_layer_enable(&self, isdb: Isdb, tuner: u32, layer_mask: u32) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_layer_enable(
                self.raw_ptr,
                isdb as u32,
                tuner,
                layer_mask.into(),
            )
        })
        .check_result()
    }

    pub fn get_layer_enable(&self, isdb: Isdb, tuner: u32) -> Result<u32, pt3::Error> {
        let mut raw_layer_mask = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_layer_enable(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_layer_mask,
            )
        })
        .check_result()
        .map(|_| {
            raw_layer_mask
        })
    }

    pub fn set_ts_pins_mode(&self, isdb: Isdb, tuner: u32, mode: TsPinsMode) -> Result<(), pt3::Error> {
        let raw_ts_pins_mode = mode.into();

        pt3::Error::from(unsafe {
            ffi::set_pt3_ts_pins_mode(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &raw_ts_pins_mode,
            )
        })
        .check_result()
    }

    pub fn get_ts_pins_level(&self, isdb: Isdb, tuner: u32) -> Result<TsPinsLevel, pt3::Error> {
        let mut raw_ts_pins_level = std::mem::MaybeUninit::<ffi::RawPt3TsPinsLevel>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_ts_pins_level(
                self.raw_ptr,
                isdb as u32,
                tuner,
                raw_ts_pins_level.as_mut_ptr(),
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_ts_pins_level.assume_init()
            }
            .into()
        })
    }

    pub fn get_ts_sync_byte(&self, isdb: Isdb, tuner: u32) -> Result<u8, pt3::Error> {
        let mut raw_sync_byte = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_ts_sync_byte(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_sync_byte,
            )
        })
        .check_result()
        .map(|_| {
            raw_sync_byte
        })
    }

    pub fn set_ram_pins_mode(&self, mode: RamPinsMode) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_ram_pins_mode(
                self.raw_ptr,
                mode as u32,
            )
        })
        .check_result()
    }

    pub fn lock_buffer(&self, buffer: &mut [u8], direction: TransferDirection) -> Result<BufferHandle, pt3::Error> {
        let mut raw_handle: *mut std::ffi::c_void = std::ptr::null_mut();

        pt3::Error::from(unsafe {
            ffi::lock_pt3_buffer(
                self.raw_ptr,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                buffer.len() as u32,
                direction as u32,
                &mut raw_handle,
            )
        })
        .check_result()
        .map(|_| {
            BufferHandle {
                raw_ptr: raw_handle,
            } 
        })
    }

    pub fn unlock_buffer(&self, handle: &BufferHandle) -> Result<(), pt3::Error> {
        if handle.raw_ptr.is_null() {
            return Err(pt3::Error::InvalidParameter);
        }

        pt3::Error::from(unsafe {
            ffi::unlock_pt3_buffer(
                self.raw_ptr,
                handle.raw_ptr,
            )
        })
        .check_result()
    }

    pub fn get_buffer_info(&self, handle: &BufferHandle) -> Result<Vec<BufferInfo>, pt3::Error> {
        if handle.raw_ptr.is_null() {
            return Err(pt3::Error::InvalidParameter);
        }

        let mut raw_info_table: *const ffi::RawPt3BufferInfo = std::ptr::null();
        let mut raw_info_count = 0u32;

        pt3::Error::from(unsafe {
            ffi::get_pt3_buffer_info(
                self.raw_ptr,
                handle.raw_ptr,
                &mut raw_info_table,
                &mut raw_info_count,
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                std::slice::from_raw_parts(raw_info_table, raw_info_count as usize)
            }
            .iter()
            .map(|&raw| raw.into())
            .collect()
        })
    }

    pub fn set_transfer_page_descriptor_address(&self, isdb: Isdb, tuner: u32, page_descriptor_address: u64) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_transfer_page_descriptor_address(
                self.raw_ptr,
                isdb as u32,
                tuner,
                page_descriptor_address,
            )
        })
        .check_result()
    }

    pub fn set_transfer_enabled(&self, isdb: Isdb, tuner: u32, enable: bool) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_transfer_enabled(
                self.raw_ptr,
                isdb as u32,
                tuner,
                enable as u8,
            )
        })
        .check_result()
    }

    pub fn get_transfer_enabled(&self, isdb: Isdb, tuner: u32) -> Result<bool, pt3::Error> {
        let mut raw_enabled = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_transfer_enabled(
                self.raw_ptr,
                isdb as u32,
                tuner,
                &mut raw_enabled,
            )
        })
        .check_result()
        .map(|_| {
            raw_enabled != 0
        })
    }

    pub fn set_transfer_test_mode(&self, isdb: Isdb, tuner: u32, test_mode: bool, initial: u16, not_op: bool) -> Result<(), pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::set_pt3_transfer_test_mode(
                self.raw_ptr,
                isdb as u32,
                tuner,
                test_mode as u8,
                initial,
                not_op as u8,
            )
        })
        .check_result()
    }

    pub fn get_transfer_info(&self, isdb: Isdb, tuner: u32) -> Result<TransferInfo, pt3::Error> {
        let mut raw_transfer_info = std::mem::MaybeUninit::<ffi::RawPt3TransferInfo>::uninit();

        pt3::Error::from(unsafe {
            ffi::get_pt3_transfer_info(
                self.raw_ptr,
                isdb as u32,
                tuner,
                raw_transfer_info.as_mut_ptr()
            )
        })
        .check_result()
        .map(|_| {
            unsafe {
                raw_transfer_info.assume_init()
            }
            .into()
        })
    }

    pub fn sync_buffer_cpu(&self, handle: &BufferHandle) -> Result<(), pt3::Error> {
        if handle.raw_ptr.is_null() {
            return Err(pt3::Error::InvalidParameter);
        }

        pt3::Error::from(unsafe {
            ffi::sync_pt3_buffer_cpu(
                self.raw_ptr,
                handle.raw_ptr,
            )
        })
        .check_result()
    }

    pub fn sync_buffer_io(&self, handle: &BufferHandle) -> Result<(), pt3::Error> {
        if handle.raw_ptr.is_null() {
            return Err(pt3::Error::InvalidParameter);
        }

        pt3::Error::from(unsafe {
            ffi::sync_pt3_buffer_io(
                self.raw_ptr,
                handle.raw_ptr,
            )
        })
        .check_result()
    }
}

// =============================================================================
// Isdb
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum Isdb {
    #[default]
    Satellite   = 0,
    Terrestrial = 1,
}

impl Isdb {
    pub const ALL: [Isdb; 2] = [
        Isdb::Satellite,
        Isdb::Terrestrial,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for Isdb {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Isdb::Satellite),
            1 => Ok(Isdb::Terrestrial),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// LnbPower
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum LnbPower {
    #[default]
    PowerOff = 0,
    Power15V = 1,
    Power11V = 2,
}

impl LnbPower {
    pub const ALL: [LnbPower; 3] = [
        LnbPower::PowerOff,
        LnbPower::Power15V,
        LnbPower::Power11V,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for LnbPower {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LnbPower::PowerOff),
            1 => Ok(LnbPower::Power15V),
            2 => Ok(LnbPower::Power11V),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// RamPinsMode
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum RamPinsMode {
    #[default]
    Normal  = 0,
    Low     = 1,
    High    = 2,
}

impl RamPinsMode {
    pub const ALL: [RamPinsMode; 3] = [
        RamPinsMode::Normal,
        RamPinsMode::Low,
        RamPinsMode::High,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for RamPinsMode {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RamPinsMode::Normal),
            1 => Ok(RamPinsMode::Low),
            2 => Ok(RamPinsMode::High),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// SatelliteLayerIndex
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum SatelliteLayerIndex {
    #[default]
    Low     = 0,
    High    = 1,
}

impl SatelliteLayerIndex {
    pub const ALL: [SatelliteLayerIndex; 2] = [
        SatelliteLayerIndex::Low,
        SatelliteLayerIndex::High,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for SatelliteLayerIndex {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SatelliteLayerIndex::Low),
            1 => Ok(SatelliteLayerIndex::High),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// SatelliteLayerMask
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum SatelliteLayerMask {
    #[default]
    None    = 0,
    Low     = 1 << SatelliteLayerIndex::Low as u32,
    High    = 1 << SatelliteLayerIndex::High as u32,
}

impl SatelliteLayerMask {
    pub const ALL: [SatelliteLayerMask; 3] = [
        SatelliteLayerMask::None,
        SatelliteLayerMask::Low,
        SatelliteLayerMask::High,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for SatelliteLayerMask {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SatelliteLayerMask::None),
            1 => Ok(SatelliteLayerMask::Low),
            2 => Ok(SatelliteLayerMask::High),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// TerrestrialLayerIndex
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum TerrestrialLayerIndex {
    #[default]
    A   = 0,
    B   = 1,
    C   = 2,
}

impl TerrestrialLayerIndex {
    pub const ALL: [TerrestrialLayerIndex; 3] = [
        TerrestrialLayerIndex::A,
        TerrestrialLayerIndex::B,
        TerrestrialLayerIndex::C,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for TerrestrialLayerIndex {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TerrestrialLayerIndex::A),
            1 => Ok(TerrestrialLayerIndex::B),
            2 => Ok(TerrestrialLayerIndex::C),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// TerrestrialLayerMask
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum TerrestrialLayerMask {
    #[default]
    None    = 0,
    A       = 1 << TerrestrialLayerIndex::A as u32,
    B       = 1 << TerrestrialLayerIndex::B as u32,
    C       = 1 << TerrestrialLayerIndex::C as u32,
}

impl TerrestrialLayerMask {
    pub const ALL: [TerrestrialLayerMask; 4] = [
        TerrestrialLayerMask::None,
        TerrestrialLayerMask::A,
        TerrestrialLayerMask::B,
        TerrestrialLayerMask::C,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for TerrestrialLayerMask {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TerrestrialLayerMask::None),
            1 => Ok(TerrestrialLayerMask::A),
            2 => Ok(TerrestrialLayerMask::B),
            4 => Ok(TerrestrialLayerMask::C),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// TransferDirection
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum TransferDirection {
    #[default]
    Write   = 1 << 0,   // 0x01
    Read    = 1 << 1,   // 0x02
    WriteRead = Self::Write as u32 | Self::Read as u32, // 0x03
}

impl TransferDirection {
    pub const ALL: [TransferDirection; 3] = [
        TransferDirection::Write,
        TransferDirection::Read,
        TransferDirection::WriteRead,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for TransferDirection {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TransferDirection::Write),
            2 => Ok(TransferDirection::Read),
            3 => Ok(TransferDirection::WriteRead),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// TsPinMode
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum TsPinMode {
    #[default]
    Normal  = 0,
    Low     = 1,
    High    = 2,
}

impl TsPinMode {
    pub const ALL: [TsPinMode; 3] = [
        TsPinMode::Normal,
        TsPinMode::Low,
        TsPinMode::High,
    ];
    pub const COUNT: usize = Self::ALL.len();
}

impl TryFrom<u32> for TsPinMode {
    type Error = pt3::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TsPinMode::Normal),
            1 => Ok(TsPinMode::Low),
            2 => Ok(TsPinMode::High),
            _ => Err(pt3::Error::InvalidParameter),
        }
    }
}

// =============================================================================
// BufferInfo
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BufferInfo {
    pub address: u64,
    pub size: u32,
}

impl From<ffi::RawPt3BufferInfo> for BufferInfo {
    fn from(value: ffi::RawPt3BufferInfo) -> Self {
        Self {
            address: value.address,
            size:    value.size,
        }
    }
}

// =============================================================================
// BufferHandle
// =============================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct BufferHandle {
    raw_ptr: *mut std::ffi::c_void,
}

// =============================================================================
// ConstantInfo
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConstantInfo {
    pub pt_version: u8,
    pub register_map_version: u8,
    pub fpga_version: u8,
    pub is_ts_supported: bool,
    pub page_descriptor_size_bits: u32,
}

impl From<ffi::RawPt3ConstantInfo> for ConstantInfo {
    fn from(value: ffi::RawPt3ConstantInfo) -> Self {
        Self {
            pt_version:                value.pt_version,
            register_map_version:      value.register_map_version,
            fpga_version:              value.fpga_version,
            is_ts_supported:           value.is_ts_supported != 0,
            page_descriptor_size_bits: value.page_descriptor_size_bits,
        }
    }
}

// =============================================================================
// ErrorRate
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ErrorRate {
    pub numerator: u32,
    pub denominator: u32,
}

impl From<ffi::RawPt3ErrorRate> for ErrorRate {
    fn from(value: ffi::RawPt3ErrorRate) -> Self {
        Self {
            numerator:   value.numerator,
            denominator: value.denominator,
        }
    }
}

// =============================================================================
// SatelliteLayer
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SatelliteLayer {
    pub mode: [u32; SatelliteLayerIndex::COUNT],
    pub count: [u32; SatelliteLayerIndex::COUNT],
}

impl From<ffi::RawPt3SatelliteLayer> for SatelliteLayer {
    fn from(value: ffi::RawPt3SatelliteLayer) -> Self {
        Self {
            mode:  value.mode,
            count: value.count,
        }
    }
}

// =============================================================================
// SatelliteTmcc
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SatelliteTmcc {
    pub indicator: u32,
    pub mode: [u32; 4],
    pub slot: [u32; 4],
    pub id: [u32; 8],
    pub emergency: u32,
    pub up_link: u32,
    pub ext_flag: u32,
    pub ext_data: [u32; 2],
}

impl From<ffi::RawPt3SatelliteTmcc> for SatelliteTmcc {
    fn from(value: ffi::RawPt3SatelliteTmcc) -> Self {
        Self {
            indicator: value.indicator,
            mode:      value.mode,
            slot:      value.slot,
            id:        value.id,
            emergency: value.emergency,
            up_link:   value.up_link,
            ext_flag:  value.ext_flag,
            ext_data:  value.ext_data,
        }
    }
}

// =============================================================================
// TerrestrialTmcc
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerrestrialTmcc {
    pub system: u32,
    pub indicator: u32,
    pub emergency: u32,
    pub partial: u32,
    pub mode: [u32; TerrestrialLayerIndex::COUNT],
    pub rate: [u32; TerrestrialLayerIndex::COUNT],
    pub interleave: [u32; TerrestrialLayerIndex::COUNT],
    pub segment: [u32; TerrestrialLayerIndex::COUNT],
    pub phase: u32,
    pub reserved: u32,
}

impl From<ffi::RawPt3TerrestrialTmcc> for TerrestrialTmcc {
    fn from(value: ffi::RawPt3TerrestrialTmcc) -> Self {
        Self {
            system:     value.system,
            indicator:  value.indicator,
            emergency:  value.emergency,
            partial:    value.partial,
            mode:       value.mode,
            rate:       value.rate,
            interleave: value.interleave,
            segment:    value.segment,
            phase:      value.phase,
            reserved:   value.reserved,
        }
    }
}

// =============================================================================
// TransferInfo
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TransferInfo {
    pub busy: bool,
    pub status: u32,
    pub internal_fifo_a_overflow: bool,
    pub internal_fifo_a_underflow: bool,
    pub external_fifo_overflow: bool,
    pub external_fifo_max_used_bytes: u32,
    pub internal_fifo_b_overflow: bool,
    pub internal_fifo_b_underflow: bool,
}

impl From<ffi::RawPt3TransferInfo> for TransferInfo {
    fn from(value: ffi::RawPt3TransferInfo) -> Self {
        Self {
            busy:                         value.busy != 0,
            status:                       value.status,
            internal_fifo_a_overflow:     value.internal_fifo_a_overflow != 0,
            internal_fifo_a_underflow:    value.internal_fifo_a_underflow != 0,
            external_fifo_overflow:       value.external_fifo_overflow != 0,
            external_fifo_max_used_bytes: value.external_fifo_max_used_bytes,
            internal_fifo_b_overflow:     value.internal_fifo_b_overflow != 0,
            internal_fifo_b_underflow:    value.internal_fifo_b_underflow != 0,
        }
    }
}

// =============================================================================
// TsPinsLevel
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TsPinsLevel {
    pub clock: bool,
    pub data: bool,
    pub byte: bool,
    pub valid: bool,
}

impl From<ffi::RawPt3TsPinsLevel> for TsPinsLevel {
    fn from(value: ffi::RawPt3TsPinsLevel) -> Self {
        Self {
            clock: value.clock != 0,
            data:  value.data != 0,
            byte:  value.byte != 0,
            valid: value.valid != 0,
        }
    }
}

// =============================================================================
// TsPinsMode
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TsPinsMode {
    pub clock_data: TsPinMode,
    pub byte: TsPinMode,
    pub valid: TsPinMode,
}

impl From<TsPinsMode> for ffi::RawPt3TsPinsMode {
    fn from(value: TsPinsMode) -> Self {
        Self {
            clock_data: value.clock_data as u32,
            byte:       value.byte as u32,
            valid:      value.valid as u32,
        }
    }
}
