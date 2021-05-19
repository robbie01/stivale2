pub mod tag;
pub use tag::*;

use cstr_core::{CStr, c_char};

#[repr(C, packed)]
pub struct Structure {
    bootloader_brand: [c_char; 64],
    bootloader_version: [c_char; 64],
    tags: u64
}

impl Structure {
    pub fn bootloader_brand(&self) -> &CStr {
        // Safe, the bootloader guarantees a 0-terminated string
        unsafe { CStr::from_ptr(&self.bootloader_brand as *const c_char) }
    }

    pub fn bootloader_version(&self) -> &CStr {
        // Safe, the bootloader guarantees a 0-terminated string
        unsafe { CStr::from_ptr(&self.bootloader_version as *const c_char) }
    }

    fn tags(&self) -> *const () {
        self.tags as *const ()
    }
}