use crate::new_tag;

new_tag! {
    pub struct Rsdp {
        rsdp: u64;
        IDENTIFIER = 0x9e1786930a375e78;
    }
}

impl Rsdp {
    pub fn rsdp(&self) -> *const () {
        self.rsdp as *const ()
    }
}