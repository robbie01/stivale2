#![no_std]
#![feature(const_fn_trait_bound)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_union)]
#![feature(const_fn_transmute)]

pub mod header;
pub mod structure;

pub use header::{Header, HeaderFlags};
pub use structure::Structure;

#[repr(C, packed)]
pub struct TagHeader {
    identifier: u64,
    next: u64
}

macro_rules! new_tag {
    { $v:vis struct $name:ident { IDENTIFIER = $identifier:literal ; } } => {
        #[repr(C, packed)]
        #[allow(dead_code)]
        $v struct $name {
            tag_header: $crate::TagHeader
        }
        impl $crate::private::TagPrivate for $name {
            const IDENTIFIER: u64 = $identifier;
        }
    };
    { $v:vis struct $name:ident { $( $v2:vis $field:ident : $type:ty ),* ; IDENTIFIER = $identifier:literal ; } } => {
        #[repr(C, packed)]
        #[allow(dead_code)]
        $v struct $name {
            tag_header: $crate::TagHeader,
            $( $v2 $field : $type ),*
        }
        impl $crate::private::TagPrivate for $name {
            const IDENTIFIER: u64 = $identifier;
        }
    }
}

pub(crate) use new_tag;

pub trait Tag: private::TagPrivate {}
pub trait HeaderTag: private::HeaderTagPrivate {}
pub trait StructureTag: private::StructureTagPrivate {}

impl<T: private::TagPrivate> Tag for T {}
impl<T: private::HeaderTagPrivate> HeaderTag for T {}
impl<T: private::StructureTagPrivate> StructureTag for T {}

mod private {
    use crate::*;

    pub trait TagPrivate {
        const IDENTIFIER: u64;

        fn next<T: Tag>(&self) -> Option<&T> {
            unsafe { (self.tag_header().next as *const T).as_ref() }
        }

        fn tag_header(&self) -> &TagHeader {
            unsafe { &*((self as *const Self).cast()) }
        }
    
        fn tag_header_mut(&mut self) -> &mut TagHeader {
            unsafe { &mut *((self as *mut Self).cast()) }
        }
    }
    pub trait HeaderTagPrivate {}
    pub trait StructureTagPrivate {}

    impl HeaderTagPrivate for header::FiveLevelPaging {}
    impl HeaderTagPrivate for header::Framebuffer {}
    impl HeaderTagPrivate for header::Smp {}
    impl HeaderTagPrivate for header::Terminal {}
    impl HeaderTagPrivate for header::UnmapNull {}

    impl StructureTagPrivate for structure::Cmdline {}
    impl StructureTagPrivate for structure::Edid {}
    impl StructureTagPrivate for structure::Framebuffer {}
    impl StructureTagPrivate for structure::Memmap {}
    impl StructureTagPrivate for structure::Terminal {}
}