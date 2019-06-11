#![no_std]

#[no_mangle]
pub extern fn extcodesize( address: *const u8) -> i32 {
    panic!("extcodesize not available")
}

#[no_mangle]
pub extern fn extcodecopy( dest: *mut u8, address: *const u8) {
    panic!("extcodecopy not available");
}

// #[no_mangle]
// pub extern fn dcall(
//             gas: i64,
//             address: *const u8,
//             input_ptr: *const u8,
//             input_len: u32,
//             result_ptr: *mut u8,
//             result_len: u32,
//     ) -> i32 {
//         panic!("dcall not available")
//     }

#[no_mangle]
pub extern fn cap9_syscall_low(input_ptr: *const u8, input_len: u32, result_ptr: *mut u8, result_len: u32) -> i32 {
    panic!("cap9_syscall_low not available")
}

#[no_mangle]
pub extern fn gasleft() -> i64 {
    panic!("gasleft not available");
}
