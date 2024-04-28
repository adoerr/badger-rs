use core::{arch::asm, ptr::addr_of_mut};

use cortex_m::asm;
use defmt::info;

/// Stack size
pub const STACK_SIZE: usize = 1024;

/// Scheduler task
pub static mut SCHEDULER: Task = Task {
    next: None,
    stack_ptr: 0 as *mut u32,
    stack: [0; STACK_SIZE],
};

/// Pointer ot the current task
static mut CURRENT: *mut Task = 0 as *mut Task;

#[allow(unused)]
#[repr(C)]
/// Task structure
pub struct Task {
    /// next task in the list
    pub next: Option<&'static mut Task>,
    /// saved stack pointer
    pub stack_ptr: *mut u32,
    /// task stack
    pub stack: [u8; STACK_SIZE],
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

/// Initialize scheduler
pub fn init_scheduler() {
    info!("init scheduler with stack pointer: {:?}", stack_ptr());

    unsafe {
        SCHEDULER.stack_ptr = &mut SCHEDULER.stack[STACK_SIZE - 1] as *mut u8 as *mut u32;
    }
}

#[no_mangle]
fn scheduler() -> ! {
    info!("call scheduler with stack pointer: {:?}", stack_ptr());

    unsafe {
        CURRENT = addr_of_mut!(SCHEDULER);
    }
    unsafe {
        info!("current task: {:?}", CURRENT as usize);
    }

    loop {
        asm::wfi();
    }
}

pub fn task_switch() -> ! {
    info!("task_switch");
    scheduler()
}
