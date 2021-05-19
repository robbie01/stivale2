use crate::new_tag;
use core::slice;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum FramebufferMemoryModel {
    RGB = 1
}

new_tag! {
    pub struct Framebuffer {
        addr: u64,
        pub width: u16,
        pub height: u16,
        pub pitch: u16,
        pub bpp: u16,
        memory_model: u8,
        pub red_mask_size: u8,
        pub red_mask_shift: u8,
        pub green_mask_size: u8,
        pub green_mask_shift: u8,
        pub blue_mask_size: u8,
        pub blue_mask_shift: u8;
        IDENTIFIER = 0x506461d2950408fa;
    }
}

impl Framebuffer {
    pub fn buffer(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr as *mut u8, (self.height * self.pitch) as usize) }
    }

    pub fn memory_model(&self) -> Result<FramebufferMemoryModel, u8> {
        FromPrimitive::from_u8(self.memory_model).ok_or(self.memory_model)
    }
}