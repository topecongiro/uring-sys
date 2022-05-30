use std::ffi::CString;

pub fn strerror(code: i32) -> String {
    unsafe {
        CString::from_raw(libc::strerror(code))
            .into_string()
            .unwrap()
    }
}
