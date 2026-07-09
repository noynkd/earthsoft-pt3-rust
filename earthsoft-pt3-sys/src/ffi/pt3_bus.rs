use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RawPt3DeviceInfo {
    pub bus: u32,
    pub slot: u32,
    pub function: u32,
    pub pt_version: u32,
}

#[derive(Debug)]
pub enum RawPt3Bus {}

#[link(name = "earthsoft_pt3", kind = "static")]
unsafe extern "C" {
    fn DeletePt3Bus(bus: *mut RawPt3Bus) -> i32;
    fn GetPt3Version(bus: *mut RawPt3Bus, version: *mut u32) -> i32;
    fn ScanPt3DeviceInfo(bus: *mut RawPt3Bus, deviceInfo: *mut RawPt3DeviceInfo, deviceInfoCount: *mut u32) -> i32;
    fn CreatePt3Device(bus: *mut RawPt3Bus, deviceInfo: *const RawPt3DeviceInfo, device: *mut *mut ffi::RawPt3Device) -> i32;
}

pub unsafe fn delete_pt3_bus(bus: *mut RawPt3Bus) -> i32 {
    unsafe { DeletePt3Bus(bus) }
}

pub unsafe fn get_pt3_version(bus: *mut RawPt3Bus, version: *mut u32) -> i32 {
    unsafe { GetPt3Version(bus, version) }
}

pub unsafe fn scan_pt3_device_info(bus: *mut RawPt3Bus, device_info: *mut RawPt3DeviceInfo, device_info_count: *mut u32) -> i32 {
    unsafe { ScanPt3DeviceInfo(bus, device_info, device_info_count) }
}

pub unsafe fn create_pt3_device(bus: *mut RawPt3Bus, device_info: *const RawPt3DeviceInfo, device: *mut *mut ffi::RawPt3Device) -> i32 {
   unsafe { CreatePt3Device(bus, device_info, device) } 
}
