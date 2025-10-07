extern crate alloc;
extern crate core;

mod allocation;
mod builder;
mod crypto;
mod token;
mod wasm_export;
mod wasm_result;

#[allow(unused)]
pub(crate) fn make_rng() -> rand::rngs::StdRng {
    let mut data = [0u8; 8];
    getrandom::getrandom(&mut data[..]).unwrap();
    rand::SeedableRng::seed_from_u64(u64::from_le_bytes(data))
}

unsafe extern "C" {
    #[cfg(feature = "print")]
    pub fn print(ptr: *const u8, len: usize);
    #[cfg(feature = "ffi")]
    pub fn extern_func(
        symbols_ptr: *const u8,
        left_ptr: *const u8,
        left_len: usize,
        right_ptr: *const u8,
        right_len: usize,
        user_data: u64,
        ret: &mut wasm_result::WasmResult,
    );
}

#[macro_export]
macro_rules! print_wasm {
    ($($args:tt)*) => {
        #[cfg(feature = "print")]
        {
            let msg = format!($($args)*);
            unsafe { $crate::print(msg.as_ptr(), msg.len()) };
        }
    };
}
