use windows_sys::Win32;

// =============================================================================
// MemoryBuffer
// =============================================================================

#[derive(Debug)]
pub struct MemoryBuffer {
    ptr: *mut std::ffi::c_void,
    size: usize,
}

impl MemoryBuffer {
    pub fn new(size: usize) -> Result<Self, MemoryBufferError> {
        if size == 0 {
            return Err(MemoryBufferError::InvalidParameter);
        }

        let ptr = unsafe {
            Win32::System::Memory::VirtualAlloc(
                std::ptr::null_mut(),
                size,
                Win32::System::Memory::MEM_COMMIT,
                Win32::System::Memory::PAGE_READWRITE,
            )
        };

        if ptr.is_null() {
            eprintln!("Win32::System::Memory::VirtualAlloc() に失敗しました.");
            return Err(MemoryBufferError::VirtualAlloc);
        }

        Ok(Self { ptr, size })
    }

    pub fn slice(&self) -> &[u8] {
        let ptr = self.ptr as *const u8;
        let size = self.size;

        if ptr.is_null() || size == 0 {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn slice_mut(&mut self) -> &mut [u8] {
        let ptr = self.ptr as *mut u8;
        let size = self.size;

        if ptr.is_null() || size == 0 {
            return &mut [];
        }

        unsafe { std::slice::from_raw_parts_mut(ptr, size) }
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }

        let result = unsafe {
            Win32::System::Memory::VirtualFree(self.ptr, 0, Win32::System::Memory::MEM_RELEASE)
        };

        if result == 0 {
            eprintln!("Win32::System::Memory::VirtualFree() に失敗しました.");
            return;
        }
    }
}

// =============================================================================
// MemoryBufferError
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryBufferError {
    InvalidParameter, // 0x102
    VirtualAlloc,     // 0x600
}

impl MemoryBufferError {
    pub fn status(&self) -> i32 {
        match *self {
            MemoryBufferError::InvalidParameter => 0x102,
            MemoryBufferError::VirtualAlloc => 0x600,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryBufferError::InvalidParameter => "無効なパラメーターが渡されました.",
            MemoryBufferError::VirtualAlloc => {
                "Win32::System::Memory::VirtualAlloc() に失敗しました."
            }
        }
    }
}

impl std::fmt::Display for MemoryBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (status: 0x{:X})", self.as_str(), self.status())
    }
}

impl std::error::Error for MemoryBufferError {}
