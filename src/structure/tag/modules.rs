use crate::new_tag;
use core::slice;
use cstr_core::{CStr, c_char};

pub struct Module {
    begin: u64,
    end: u64,
    string: [c_char; 128]
}

impl Module {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.begin as *const u8, (self.end - self.begin) as usize) }
    }

    pub fn string(&self) -> &CStr {
        unsafe { CStr::from_ptr(&self.string as *const c_char) }
    }
}

new_tag! {
    pub struct Modules {
        count: u64,
        modules: ();
        IDENTIFIER = 0x4b6fe466aade04ce;
    }
}

impl Modules {
    pub fn modules(&self) -> &[Module] {
        unsafe { slice::from_raw_parts((&self.modules as *const ()).cast(), self.count as usize) }
    }
}