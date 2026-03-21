pub mod kernels;

use std::ffi::CString;
use std::os::raw::{c_char, c_uchar, c_void};

#[cfg(target_os = "macos")]
#[link(name = "ane_bridge")]
extern "C" {
    fn ane_bridge_init() -> bool;
    fn ane_model_create_from_mil(mil_text: *const c_char, weights: *const c_uchar, weights_size: usize) -> *mut c_void;
    fn ane_model_destroy(model: *mut c_void);
    fn ane_model_compile(model: *mut c_void) -> bool;
    fn ane_model_load(model: *mut c_void) -> bool;
    fn ane_model_unload(model: *mut c_void);
    
    fn ane_iosurface_create(size: usize) -> *mut c_void;
    fn ane_iosurface_destroy(surface: *mut c_void);
    fn ane_iosurface_get_ptr(surface: *mut c_void) -> *mut u8;
    
    fn ane_model_evaluate(model: *mut c_void, input: *mut c_void, output: *mut c_void) -> bool;
}

pub struct AneIOSurface {
    ptr: *mut c_void,
    size: usize,
}

unsafe impl Send for AneIOSurface {}
unsafe impl Sync for AneIOSurface {}

impl AneIOSurface {
    pub fn new(size: usize) -> Self {
        #[cfg(target_os = "macos")]
        {
            let ptr = unsafe { ane_iosurface_create(size) };
            Self { ptr, size }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Self { ptr: std::ptr::null_mut(), size }
        }
    }
    
    pub fn get_data_mut(&mut self) -> &mut [u8] {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let data_ptr = ane_iosurface_get_ptr(self.ptr);
                std::slice::from_raw_parts_mut(data_ptr, self.size)
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            // Dummy for linux tests
            unsafe { std::slice::from_raw_parts_mut(std::ptr::NonNull::dangling().as_ptr(), 0) }
        }
    }
}

impl Drop for AneIOSurface {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            unsafe { ane_iosurface_destroy(self.ptr) };
        }
    }
}

pub struct AneModel {
    handle: *mut c_void,
}

unsafe impl Send for AneModel {}
unsafe impl Sync for AneModel {}

impl AneModel {
    pub fn from_mil(mil: &str, weights: &[u8]) -> Option<Self> {
        #[cfg(target_os = "macos")]
        {
            let c_mil = CString::new(mil).ok()?;
            let handle = unsafe {
                ane_model_create_from_mil(c_mil.as_ptr(), weights.as_ptr(), weights.len())
            };

            if handle.is_null() {
                None
            } else {
                Some(Self { handle })
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Some(Self { handle: std::ptr::null_mut() })
        }
    }
    
    pub fn compile_and_load(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                if ane_model_compile(self.handle) {
                    ane_model_load(self.handle)
                } else {
                    false
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }
    
    pub fn evaluate(&self, input: &AneIOSurface, output: &AneIOSurface) -> bool {
        #[cfg(target_os = "macos")]
        {
            unsafe { ane_model_evaluate(self.handle, input.ptr, output.ptr) }
        }
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }
}

impl Drop for AneModel {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                ane_model_unload(self.handle);
                ane_model_destroy(self.handle);
            }
        }
    }
}

pub struct AneLimb {
    initialized: bool,
}

impl AneLimb {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        {
            let initialized = unsafe { ane_bridge_init() };
            Self { initialized }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Self { initialized: false }
        }
    }
    
    pub fn is_operational(&self) -> bool {
        self.initialized
    }
}
