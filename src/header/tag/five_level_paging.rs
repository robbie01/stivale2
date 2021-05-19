use crate::{new_tag, HeaderTag, TagHeader, private::TagPrivate};

new_tag! {
    pub struct FiveLevelPaging {
        IDENTIFIER = 0x932f477032007e8f;
    }
}

impl FiveLevelPaging {
    pub const fn new() -> Self {
        FiveLevelPaging {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 }
        }
    }

    pub const fn with_next<T: HeaderTag>(next: *const T) -> Self {
        FiveLevelPaging {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } }
        }
    }
}