pub mod kernels;

use std::ffi::CString;
use std::os::raw::{c_char, c_uchar, c_void};

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
        let ptr = unsafe { ane_iosurface_create(size) };
        Self { ptr, size }
    }
    
    pub fn get_data_mut(&mut self) -> &mut [u8] {
        unsafe {
            let data_ptr = ane_iosurface_get_ptr(self.ptr);
            std::slice::from_raw_parts_mut(data_ptr, self.size)
        }
    }
}

impl Drop for AneIOSurface {
    fn drop(&mut self) {
        unsafe { ane_iosurface_destroy(self.ptr) };
    }
}

pub struct AneModel {
    handle: *mut c_void,
}

unsafe impl Send for AneModel {}
unsafe impl Sync for AneModel {}

impl AneModel {
    pub fn from_mil(mil: &str, weights: &[u8]) -> Option<Self> {
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
    
    pub fn compile_and_load(&self) -> bool {
        unsafe {
            if ane_model_compile(self.handle) {
                ane_model_load(self.handle)
            } else {
                false
            }
        }
    }
    
    pub fn evaluate(&self, input: &AneIOSurface, output: &AneIOSurface) -> bool {
        unsafe { ane_model_evaluate(self.handle, input.ptr, output.ptr) }
    }
}

impl Drop for AneModel {
    fn drop(&mut self) {
        unsafe {
            ane_model_unload(self.handle);
            ane_model_destroy(self.handle);
        }
    }
}

pub struct AneLimb {
    initialized: bool,
}

impl AneLimb {
    pub fn new() -> Self {
        let initialized = unsafe { ane_bridge_init() };
        Self { initialized }
    }
    
    pub fn is_operational(&self) -> bool {
        self.initialized
    }
}
