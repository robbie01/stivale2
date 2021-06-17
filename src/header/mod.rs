pub mod tag;
pub use tag::*;

use crate::{HeaderTag, structure::Structure};
use core::{mem::transmute, ptr::null};
use bitflags::bitflags;

bitflags! {
    pub struct HeaderFlags: u64 {
        const HIGHER_HALF_POINTERS = 0b10;
    }
}

pub type EntryPoint = extern "sysv64" fn(structure: *const Structure) -> !;

#[repr(C, packed)]
#[allow(dead_code)]
pub struct Header {
    entry_point: *const (),
    stack: *const (),
    flags: u64,
    tags: *const ()
}

impl Header {
    pub const fn new<T: HeaderTag>(entry_point: Option<EntryPoint>, stack: *const (), flags: HeaderFlags, tags: *const T) -> Header {
        unsafe {
            Header {
                // This is needed because Rust doesn't currently support null function pointers. Safe, I think.
                entry_point: match entry_point {
                    Some(ep) => ep as *const (),
                    None => null()
                },
                stack: stack,
                flags: flags.bits(),
                tags: tags as *const ()
            }
        }
    }
}