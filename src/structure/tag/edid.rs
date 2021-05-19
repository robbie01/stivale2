use crate::new_tag;
use core::slice;

new_tag! {
    pub struct Edid {
        size: u64,
        information: ();
        IDENTIFIER = 0x968609d7af96b845;
    }
}

impl Edid {
    pub fn information(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((&self.information as *const ()).cast(), self.size as usize) }
    }
}