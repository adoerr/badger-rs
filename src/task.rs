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
