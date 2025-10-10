use crate::wasm_result::WasmResult;
use alloc::alloc;
use std::mem;

#[unsafe(no_mangle)]
/// Allocates a new area of memory and returns a pointer to it.
/// *Parameters*
/// - `size`: allocated size in bytes
/// - `align`: alignment of the allocated area
pub fn malloc(size: usize, align: usize) -> *mut u8 {
    unsafe { alloc::alloc(alloc::Layout::from_size_align_unchecked(size, align)) }
}
#[unsafe(no_mangle)]
/// Frees a previously allocated area
/// *Parameters*
/// - `ptr`: allocated address
/// - `size`: allocated size
/// - `align`: alignment of the allocated area
pub fn free(ptr: *mut u8, size: usize, align: usize) {
    unsafe { alloc::dealloc(ptr, alloc::Layout::from_size_align_unchecked(size, align)) }
}

#[unsafe(no_mangle)]
/// Allocates a WasmResult area and returns a pointer to it.
/// Rust doesn't handle the allocated region, user is responsible for
/// freeing after usage
pub fn get_return_area() -> Box<WasmResult> {
    Box::new(unsafe { mem::zeroed() })
}

pub struct HostBytes {
    ptr: *mut u8,
    len: usize,
}

impl HostBytes {
    pub(crate) unsafe fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            ptr: ptr as *mut u8,
            len,
        }
    }
}

impl std::ops::Deref for HostBytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl std::ops::DerefMut for HostBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl std::ops::Drop for HostBytes {
    fn drop(&mut self) {
        crate::print_wasm!("HostBytes::drop");
        free(self.ptr, self.len, 1)
    }
}
