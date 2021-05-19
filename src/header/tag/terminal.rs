use crate::{new_tag, Tag, TagHeader, private::TagPrivate};

new_tag! {
    pub struct Terminal {
        flags: u64;
        IDENTIFIER = 0xa85d499b1823be72;
    }
}

impl Terminal {
    pub const fn new() -> Self {
        Terminal {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 },
            flags: 0
        }
    }

    pub const fn with_next<T: Tag>(next: *const T) -> Self {
        Terminal {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } },
            flags: 0
        }
    }
}