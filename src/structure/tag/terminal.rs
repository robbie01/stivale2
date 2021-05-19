use crate::new_tag;
use bitflags::bitflags;

bitflags! {
    struct TerminalFlags: u32 {
        const COLS_AND_ROWS_PROVIDED = 0b1;
    }
}

new_tag! {
    pub struct Terminal {
        flags: u32,
        cols: u16,
        rows: u16,
        pub term_write: u64;
        IDENTIFIER = 0xc2b3f4c3233b0974;
    }
}

impl Terminal {
    pub fn cols_and_rows_provided(&self) -> bool {
        // the compiler complains about unaligned references if you store the flags directly in the struct
        TerminalFlags::from_bits_truncate(self.flags).contains(TerminalFlags::COLS_AND_ROWS_PROVIDED)
    }

    pub fn cols(&self) -> Option<u16> {
        self.cols_and_rows_provided().then(|| self.cols)
    }

    pub fn rows(&self) -> Option<u16> {
        self.cols_and_rows_provided().then(|| self.rows)
    }

    pub fn cols_and_rows(&self) -> Option<(u16, u16)> {
        self.cols_and_rows_provided().then(|| (self.cols, self.rows))
    }

    pub unsafe fn call_term_write(addr: u64, buffer: &[u8]) -> () {
        let term_write: fn(*const u8, usize) -> () = core::mem::transmute(addr as *const ());
        term_write(buffer.as_ptr(), buffer.len())
    }

    pub fn term_write(&self, buffer: &[u8]) -> () {
        unsafe { Self::call_term_write(self.term_write, buffer) }
    }
}