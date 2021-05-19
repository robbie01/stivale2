use crate::{new_tag, HeaderTag, TagHeader, private::TagPrivate};

new_tag! {
    pub struct Framebuffer {
        width: u16,
        height: u16,
        bpp: u16;
        IDENTIFIER = 0x92919432b16fe7e7;
    }
}

impl Framebuffer {
    pub const fn new() -> Self {
        Framebuffer {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 },
            width: 0,
            height: 0,
            bpp: 0
        }
    }

    pub const fn with_next<T: HeaderTag>(next: *const T) -> Self {
        Framebuffer {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } },
            width: 0,
            height: 0,
            bpp: 0
        }
    }

    pub const fn with_format(width: u16, height: u16, bpp: u16) -> Self {
        Framebuffer {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 },
            width: width,
            height: height,
            bpp: bpp
        }
    }

    pub const fn with_next_and_format<T: HeaderTag>(next: *const T, width: u16, height: u16, bpp: u16) -> Self {
        Framebuffer {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } },
            width: width,
            height: height,
            bpp: bpp
        }
    }
}