pub mod tag;
pub use tag::*;

use crate::{HeaderTag, structure::Structure};
use core::mem::transmute;
use bitflags::bitflags;

bitflags! {
    pub struct HeaderFlags: u64 {
        const HIGHER_HALF_POINTERS = 0b10;
    }
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
    pub const fn new<T: HeaderTag>(entry_point: Option<extern "sysv64" fn(structure: *const Structure) -> !>, stack: *const (), flags: HeaderFlags, tags: *const T) -> Header {
        unsafe {
            Header {
                // This is needed because Rust doesn't currently support null function pointers. Safe, I think.
                entry_point: match entry_point {
                    Some(ep) => transmute(ep),
                    None => 0
                },
                stack: transmute(stack), // lord forgive me
                flags: flags.bits(),
                tags: transmute(tags)
            }
        }
    }
}