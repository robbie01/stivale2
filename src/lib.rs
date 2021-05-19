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
        impl $crate::Tag for $name {
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
        impl $crate::Tag for $name {
            const IDENTIFIER: u64 = $identifier;
        }
    }
}

pub(crate) use new_tag;

pub trait Tag: private::TagPrivate {
    const IDENTIFIER: u64;

    fn next<T: Tag>(&self) -> Option<&T> {
        unsafe { (self.tag_header().next as *const T).as_ref() }
    }
}

mod private {
    use crate::*;

    pub trait TagPrivate {
        fn tag_header(&self) -> &TagHeader {
            unsafe { &*((self as *const Self).cast()) }
        }
    
        fn tag_header_mut(&mut self) -> &mut TagHeader {
            unsafe { &mut *((self as *mut Self).cast()) }
        }
    }
    impl TagPrivate for header::FiveLevelPaging {}
    impl TagPrivate for header::Framebuffer {}
    impl TagPrivate for header::Smp {}
    impl TagPrivate for header::Terminal {}
    impl TagPrivate for header::UnmapNull {}
    impl TagPrivate for structure::Cmdline {}
    impl TagPrivate for structure::Edid {}
    impl TagPrivate for structure::Framebuffer {}
    impl TagPrivate for structure::Memmap {}
    impl TagPrivate for structure::Terminal {}
}