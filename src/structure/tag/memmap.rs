use crate::new_tag;
use core::slice;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum MemmapType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    BootloaderReclaimable = 0x1000,
    KernelAndModules = 0x1001,
    Framebuffer = 0x1002
}

#[repr(C, packed)]
pub struct MemmapEntry {
    base: u64,
    length: u64,
    r#type: u32,
    _unused: u32
}

impl MemmapEntry {
    pub fn as_slice(&self) -> &[u8] {
        // Safe, guaranteed by bootloader
        unsafe { slice::from_raw_parts(self.base as *const u8, self.length as usize) }
    }

    pub fn as_mut_slice(&self) -> &mut [u8] {
        // Safe, guaranteed by bootloader
        unsafe { slice::from_raw_parts_mut(self.base as *mut u8, self.length as usize) }
    }

    pub fn r#type(&self) -> MemmapType {
        FromPrimitive::from_u32(self.r#type).unwrap()
    }
}

new_tag! {
    pub struct Memmap {
        entries: u64,
        memmap: ();
        IDENTIFIER = 0x2187f79e8612de07;
    }
}

impl Memmap {
    pub fn entries(&self) -> &[MemmapEntry] {
        unsafe { slice::from_raw_parts((&self.memmap as *const ()).cast(), self.entries as usize) }
    }
}