use crate::pt3;
use earthsoft_pt3_sys::ffi;

// =============================================================================
// Pt3
// =============================================================================

#[derive(Debug)]
pub struct Pt3;

impl Pt3 {
    pub fn new() -> Result<std::sync::Arc<Self>, pt3::Error> {
        pt3::Error::from(unsafe {
            ffi::load_pt3_lib()
        })
        .check_result()
        .map(|_| {
            std::sync::Arc::new(Self)
        })
    }

    pub fn create_bus(self: &std::sync::Arc<Self>) -> Result<std::sync::Arc<pt3::Bus>, pt3::Error> {
        let mut raw_ptr = std::ptr::null_mut();

        pt3::Error::from(unsafe {
            ffi::create_pt3_bus(
                &mut raw_ptr,
            )
        })
        .check_result()
        .and_then(|_| {
            if raw_ptr.is_null() {
                return Err(pt3::Error::InternalError);
            }

            Ok(pt3::Bus::new(raw_ptr, self.clone()))
        })
    }
}

impl Drop for Pt3 {
    fn drop(&mut self) {
        _ = pt3::Error::from(unsafe {
            ffi::free_pt3_lib()
        })
        .check_result();
    }
}
