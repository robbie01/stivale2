use crate::{new_tag, Tag, TagHeader, private::TagPrivate};
use bitflags::bitflags;

bitflags! {
    pub struct SmpFlags: u64 {
        const X2APIC = 0b1;
    }
}

new_tag! {
    pub struct Smp {
        flags: u64;
        IDENTIFIER = 0x1ab015085f3273df;
    }
}

impl Smp {
    pub const fn new(flags: SmpFlags) -> Self {
        Smp {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: 0 },
            flags: flags.bits()
        }
    }

    pub const fn with_next<T: Tag>(next: *const T, flags: SmpFlags) -> Self {
        Smp {
            tag_header: TagHeader { identifier: Self::IDENTIFIER, next: unsafe { next as u64 } },
            flags: flags.bits()
        }
    }
}