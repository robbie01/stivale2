use crate::new_tag;
use cstr_core::{CStr, c_char};

new_tag! {
    pub struct Cmdline {
        cmdline: u64;
        IDENTIFIER = 0xe5e76a1b4597a781;
    }
}

impl Cmdline {
    pub fn cmdline(&self) -> &CStr {
        // Safe, the bootloader guarantees a 0-terminated string
        unsafe { CStr::from_ptr(self.cmdline as *const c_char) }
    }
}