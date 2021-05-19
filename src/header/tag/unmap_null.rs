use crate::{new_tag, HeaderTag, TagHeader, private::TagPrivate};

new_tag! {
    pub struct UnmapNull {
        IDENTIFIER = 0x92919432b16fe7e7;
    }
}

impl UnmapNull {
    pub const fn new() -> Self {
        UnmapNull {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 }
        }
    }

    pub const fn with_next<T: HeaderTag>(next: *const T) -> Self {
        UnmapNull {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } }
        }
    }
}