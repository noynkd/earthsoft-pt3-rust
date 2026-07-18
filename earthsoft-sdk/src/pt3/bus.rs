use crate::pt3;
use earthsoft_pt3_sys::ffi;

// =============================================================================
// Bus
// =============================================================================

#[derive(Debug)]
pub struct Bus {
    raw_ptr: *mut ffi::RawPt3Bus,
    _pt3: std::sync::Arc<pt3::Pt3>,
}

impl Bus {
    pub(crate) fn new(raw_ptr: *mut ffi::RawPt3Bus, pt3: std::sync::Arc<pt3::Pt3>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            raw_ptr,
            _pt3: pt3,
        })
    }

    pub fn get_version(self: &std::sync::Arc<Self>) -> Result<u32, pt3::Error> {
        let mut version = 0;

        pt3::Error::from(unsafe {
            ffi::get_pt3_version(
                self.raw_ptr,
                &mut version,
            )
        })
        .check_result()
        .map(|_| {
            version
        })
    }

    pub fn scan_device_info(self: &std::sync::Arc<Self>, max_device_count: usize) -> Result<Vec<DeviceInfo>, pt3::Error> {
        let mut raw_device_info = vec![
            ffi::RawPt3DeviceInfo::default();
            max_device_count
        ];
        let mut raw_device_info_count = max_device_count as u32;

        pt3::Error::from(unsafe {
            ffi::scan_pt3_device_info(
                self.raw_ptr,
                raw_device_info.as_mut_ptr(),
                 &mut raw_device_info_count,
            )
        })
        .check_result()
        .map(|_| {
            let count = (raw_device_info_count as usize).min(max_device_count);

            raw_device_info[0..count]
                .iter()
                .map(|&raw| raw.into())
                .collect()
        })
    }

    pub fn create_device(self: &std::sync::Arc<Self>, device_info: &DeviceInfo) -> Result<std::sync::Arc<pt3::Device>, pt3::Error> {
        let mut raw_ptr: *mut ffi::RawPt3Device = std::ptr::null_mut();
        let raw_device_info = ffi::RawPt3DeviceInfo::from(*device_info);

        pt3::Error::from(unsafe {
            ffi::create_pt3_device(
                self.raw_ptr,
                &raw_device_info,
                &mut raw_ptr,
            )
        })
        .check_result()
        .and_then(|_| {
            if raw_ptr.is_null() {
                return Err(pt3::Error::InternalError);
            }

            Ok(pt3::Device::new(raw_ptr, self.clone()))
        })
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        _ = pt3::Error::from(unsafe {
            ffi::delete_pt3_bus(self.raw_ptr)
        })
        .check_result()
    }
}

unsafe impl Send for Bus {}
unsafe impl Sync for Bus {}

// =============================================================================
// DeviceInfo
// =============================================================================

#[derive(Debug, Copy, Clone, Default)]
pub struct DeviceInfo {
    pub bus: u32,
    pub slot: u32,
    pub function: u32,
    pub pt_version: u32,
}

impl From<ffi::RawPt3DeviceInfo> for DeviceInfo {
    fn from(value: ffi::RawPt3DeviceInfo) -> Self {
        Self {
            bus:        value.bus,
            slot:       value.slot,
            function:   value.function,
            pt_version: value.pt_version,
        }
    }
}

impl From<DeviceInfo> for ffi::RawPt3DeviceInfo {
    fn from(value: DeviceInfo) -> Self {
        Self {
            bus:        value.bus,
            slot:       value.slot,
            function:   value.function,
            pt_version: value.pt_version,
        }
    }
}
