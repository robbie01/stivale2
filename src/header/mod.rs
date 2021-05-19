pub mod tag;
pub use tag::*;

use crate::{Tag, structure::Structure};
use bitflags::bitflags;

bitflags! {
    pub struct HeaderFlags: u64 {
        const HIGHER_HALF_POINTERS = 0b10;
    }
}

#[repr(C)]
union EntryPoint {
    fn_ptr: extern "sysv64" fn(*const Structure) -> !,
    int_ptr: u64
}

#[repr(C, packed)]
#[allow(dead_code)]
pub struct Header {
    entry_point: u64,
    stack: u64,
    flags: u64,
    tags: u64
}

impl Header {
    pub const fn new<T: Tag>(entry_point: Option<extern "sysv64" fn(structure: *const Structure) -> !>, stack: *const (), flags: HeaderFlags, tags: *const T) -> Header {
        unsafe {
            Header {
                // This is needed because Rust doesn't currently support null function pointers. Safe, I think.
                entry_point: match entry_point {
                    Some(ep) => EntryPoint { fn_ptr: ep },
                    None => EntryPoint { int_ptr: 0 }
                }.int_ptr,
                stack: core::mem::transmute(stack), // lord forgive me
                flags: flags.bits(),
                tags: core::mem::transmute(tags)
            }
        }
    }
}