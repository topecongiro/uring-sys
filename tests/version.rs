use uring_sys2::{io_uring_check_version, io_uring_major_version, io_uring_minor_version};

#[test]
fn version() {
    unsafe {
        assert!(io_uring_check_version(io_uring_major_version(), io_uring_minor_version()));
    }
}