use core::arch::asm;

#[allow(unused)]
/// Task structure
pub struct Task {
    /// next task in the list
    pub next: Option<&'static mut Task>,
    /// saved stack pointer
    pub stack_ptr: *mut u32,
    /// task stack
    pub stack: [u8; 4096],
}

/// Return the current stack pointer
#[inline(always)]
pub fn stack_ptr() -> *mut u32 {
    let sp: *mut u32;

    unsafe {
        asm!("mov {}, sp", out(reg) sp);
    }

    sp
}
