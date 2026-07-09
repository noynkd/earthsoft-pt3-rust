use crate::ffi;

#[link(name = "earthsoft_pt3", kind = "static")]
unsafe extern "C" {
    pub(crate) fn LoadPt3Lib() -> i32;
    pub(crate) fn FreePt3Lib() -> i32;
    pub(crate) fn CreatePt3Bus(bus: *mut *mut ffi::RawPt3Bus) -> i32;
}

pub unsafe fn load_pt3_lib() -> i32 {
    unsafe { LoadPt3Lib() }
}

pub unsafe fn free_pt3_lib() -> i32 {
    unsafe { FreePt3Lib() }
}

pub unsafe fn create_pt3_bus(bus: *mut *mut ffi::RawPt3Bus) -> i32 {
    unsafe { CreatePt3Bus(bus) }
}
