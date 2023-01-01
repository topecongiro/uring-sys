#![allow(nonstandard_style)]

mod iouring;

use libc::msghdr;
pub use iouring::*;

pub const LIBURING_UDATA_TIMEOUT: libc::__u64 = libc::__u64::MAX;

#[derive(Debug)]
#[repr(C)]
pub struct io_uring {
    pub sq: io_uring_sq,
    pub cq: io_uring_cq,
    pub flags: libc::c_uint,
    pub ring_fd: libc::c_int,

    pub features: libc::c_uint,
    pub enter_ring_fd: libc::c_int,
    pub int_flags: libc::__u8,
    pub pad: [libc::__u8; 3],
    pub pad2: libc::c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct io_uring_sq {
    pub khead: *mut libc::c_uint,
    pub ktail: *mut libc::c_uint,
    pub kring_mask: *mut libc::c_uint,
    pub kring_entries: *mut libc::c_uint,
    pub kflags: *mut libc::c_uint,
    pub kdropped: *mut libc::c_uint,
    pub array: *mut libc::c_uint,
    pub sqes: *mut io_uring_sqe,

    pub sqe_head: libc::c_uint,
    pub sqe_tail: libc::c_uint,

    pub ring_sz: libc::size_t,
    pub ring_ptr: *mut libc::c_void,

    pub ring_mask: libc::c_uint,
    pub ring_entries: libc::c_uint,

    pub pad: [libc::c_uint; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct io_uring_cq {
    pub khead: *mut libc::c_uint,
    pub ktail: *mut libc::c_uint,
    pub kring_mask: *mut libc::c_uint,
    pub kring_entries: *mut libc::c_uint,
    pub kflags: *mut libc::c_uint,
    pub koverflow: *mut libc::c_uint,
    pub cqes: *mut io_uring_cqe,

    pub ring_sz: libc::size_t,
    pub ring_ptr: *mut libc::c_void,

    pub ring_mask: libc::c_uint,
    pub ring_entries: libc::c_uint,

    pub pad: [libc::c_uint; 2],
}

#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: i64,
    pub tv_nsec: libc::c_longlong,
}

#[link(name = "uring")]
extern "C" {
    pub fn io_uring_queue_init(
        entries: libc::c_uint,
        ring: *mut io_uring,
        flags: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_queue_init_params(
        entries: libc::c_uint,
        ring: *mut io_uring,
        params: *mut io_uring_params,
    ) -> libc::c_int;

    pub fn io_uring_queue_mmap(
        fd: libc::c_int,
        params: *mut io_uring_params,
        ring: *mut io_uring,
    ) -> libc::c_int;

    pub fn io_uring_get_probe_ring(ring: *mut io_uring) -> *mut io_uring_probe;
    pub fn io_uring_get_probe() -> *mut io_uring_probe;
    pub fn io_uring_free_probe(probe: *mut io_uring_probe);

    pub fn io_uring_dontfork(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_queue_exit(ring: *mut io_uring);
    pub fn io_uring_peek_batch_cqe(
        ring: *mut io_uring,
        cqes: *mut *mut io_uring_cqe,
        count: libc::c_uint,
    ) -> libc::c_uint;
    pub fn io_uring_wait_cqes(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
        ts: *const __kernel_timespec,
        sigmask: *const libc::sigset_t,
    ) -> libc::c_int;
    pub fn io_uring_wait_cqe_timeout(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        ts: *mut __kernel_timespec,
    ) -> libc::c_int;

    pub fn io_uring_submit(ring: *mut io_uring) -> libc::c_int;
    pub fn io_uring_submit_and_wait(ring: *mut io_uring, wait_nr: libc::c_uint) -> libc::c_int;
    pub fn io_uring_submit_and_wait_timeout(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
        ts: *mut __kernel_timespec,
        sigmask: *mut libc::sigset_t,
    ) -> libc::c_int;

    pub fn io_uring_register_buffers(
        ring: *mut io_uring,
        iovecs: *const libc::iovec,
        nr_iovecs: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_buffers_tags(
        ring: *mut io_uring,
        iovecs: *mut libc::iovec,
        tags: *const libc::__u64,
        nr: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_buffers_sparse(
        ring: *mut io_uring,
        nr: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_buffers_update_tag(
        ring: *mut io_uring,
        off: libc::c_uint,
        iovecs: *const libc::iovec,
        tags: *const libc::__u64,
        nr: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_unregister_buffers(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files(
        ring: *mut io_uring,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_files_sparse(
        ring: *mut io_uring,
        nr: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_files_tags(
        ring: *mut io_uring,
        files: *const libc::c_int,
        tags: *const libc::__u64,
        nr: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_register_files_update_tag(
        ring: *mut io_uring,
        off: libc::c_uint,
        files: *const libc::c_int,
        tags: *const libc::__u64,
        nr_files: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_unregister_files(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files_update(
        ring: *mut io_uring,
        off: libc::c_uint,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_register_eventfd(ring: *mut io_uring, fd: libc::c_int) -> libc::c_int;
    pub fn io_uring_register_eventfd_async(ring: *mut io_uring, fd: libc::c_int) -> libc::c_int;
    pub fn io_uring_unregister_eventfd(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_probe(
        ring: *mut io_uring,
        p: *mut io_uring_probe,
        nr: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_register_personality(ring: *mut io_uring) -> libc::c_int;
    pub fn io_uring_unregister_personality(ring: *mut io_uring, id: libc::c_int) -> libc::c_int;

    pub fn io_uring_register_restrictions(
        ring: *mut io_uring,
        res: *mut io_uring_restriction,
        nr_res: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_enable_rings(ring: *mut io_uring);

    pub fn io_uring_register_iowq_aff(
        ring: *mut io_uring,
        cpusz: libc::size_t,
        mask: *const libc::cpu_set_t,
    );
    pub fn io_uring_unregister_iowq_aff(ring: *mut io_uring);
    pub fn io_uring_register_iowq_max_workers(ring: *mut io_uring, values: *mut libc::c_uint);

    pub fn io_uring_register_ring_fd(ring: *mut io_uring) -> libc::c_int;
    pub fn io_uring_unregister_ring_fd(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_buf_ring(
        ring: *mut io_uring,
        reg: *mut io_uring_buf_reg,
        flag: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_unregister_buf_ring(ring: *mut io_uring, bgid: libc::c_int) -> libc::c_int;
    pub fn io_uring_register_sync_cancel(ring: *mut io_uring, reg: *mut io_uring_sync_cancel_reg) -> libc::c_int;
    pub fn io_uring_register_file_alloc_range(
        ring: *mut io_uring,
        off: libc::c_uint,
        len: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_get_events(ring: *mut io_uring) -> libc::c_int;
    pub fn io_uring_submit_and_get_events(ring: *mut io_uring) -> libc::c_int;


    pub fn io_uring_enter(
        fd: libc::c_uint,
        to_submit: libc::c_uint,
        min_complete: libc::c_uint,
        flags: libc::c_uint,
        sig: *mut libc::sigset_t,
    ) -> libc::c_int;
    pub fn io_uring_enter2(
        fd: libc::c_uint,
        to_submit: libc::c_uint,
        min_complete: libc::c_uint,
        flags: libc::c_uint,
        sig: *mut libc::sigset_t,
        sz: libc::size_t,
    ) -> libc::c_int;
    pub fn io_uring_setup(entries: libc::c_uint, p: *mut io_uring_params) -> libc::c_int;
    pub fn io_uring_register(
        fd: libc::c_uint,
        opcode: libc::c_uint,
        arg: *mut libc::c_void,
        nr_args: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_mlock_size(entries: libc::c_uint, flags: libc::c_uint);
    pub fn io_uring_mlock_size_params(entries: libc::c_uint, p: *mut io_uring_params);

    pub fn io_uring_major_version() -> libc::c_int;
    pub fn io_uring_minor_version() -> libc::c_int;
    pub fn io_uring_check_version(major: libc::c_int, minor: libc::c_int) -> bool;
}

#[link(name = "rusturing")]
extern "C" {
    #[link_name = "rust_io_uring_opcode_supported"]
    pub fn io_uring_opcode_supported(p: *const io_uring_probe, op: libc::c_int) -> libc::c_int;

    #[link_name = "rust_io_uring_cq_advance"]
    pub fn io_uring_cq_advance(ring: *mut io_uring, nr: libc::c_uint);

    #[link_name = "rust_io_uring_cqe_seen"]
    pub fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);

    #[link_name = "rust_io_uring_sqe_set_data"]
    pub fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, data: *mut libc::c_void);

    #[link_name = "rust_io_uring_cqe_get_data"]
    pub fn io_uring_cqe_get_data(cqe: *mut io_uring_cqe) -> *mut libc::c_void;

    #[link_name = "rust_io_uring_sqe_set_data64"]
    pub fn io_uring_sqe_set_data64(sqe: *mut io_uring_sqe, data: libc::__u64);

    #[link_name = "rust_io_uring_cqe_get_data64"]
    pub fn io_uring_cqe_get_data64(cqe: *mut io_uring_cqe) -> libc::__u64;

    #[link_name = "rust_io_uring_sqe_set_flags"]
    pub fn io_uring_sqe_set_flags(sqe: *mut io_uring_sqe, flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_rw"]
    pub fn io_uring_prep_rw(
        op: libc::c_int,
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *const libc::c_void,
        len: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_splice"]
    pub fn io_uring_prep_splice(
        sqe: *mut io_uring_sqe,
        fd_in: libc::c_int,
        off_in: libc::loff_t,
        fd_out: libc::c_int,
        off_out: libc::loff_t,
        nbytes: libc::c_uint,
        splice_flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_tee"]
    pub fn io_uring_prep_tee(
        sqe: *mut io_uring_sqe,
        fd_in: libc::c_int,
        fd_out: libc::c_int,
        nbytes: libc::c_uint,
        splice_flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_readv"]
    pub fn io_uring_prep_readv(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_readv2"]
    pub fn io_uring_prep_readv2(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::__u64,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_read_fixed"]
    pub fn io_uring_prep_read_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::__u64,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_writev"]
    pub fn io_uring_prep_writev(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_writev2"]
    pub fn io_uring_prep_writev2(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::__u64,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_write_fixed"]
    pub fn io_uring_prep_write_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *const libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::__u64,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_recvmsg"]
    pub fn io_uring_prep_recvmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *mut libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_sendmsg"]
    pub fn io_uring_prep_sendmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *const libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_poll_add"]
    pub fn io_uring_prep_poll_add(sqe: *mut io_uring_sqe, fd: libc::c_int, poll_mask: libc::c_uint);

    #[link_name = "rust_io_uring_prep_poll_multishot"]
    pub fn io_uring_prep_poll_multishot(
        io_uring_sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        poll_mask: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_poll_remove"]
    pub fn io_uring_prep_poll_remove(sqe: *mut io_uring_sqe, user_data: libc::__u64);

    #[link_name = "rust_io_uring_prep_poll_update"]
    pub fn io_uring_prep_poll_update(
        sqe: *mut io_uring_sqe,
        old_user_data: libc::__u64,
        new_user_data: libc::__u64,
        poll_mask: libc::c_uint,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_fsync"]
    pub fn io_uring_prep_fsync(sqe: *mut io_uring_sqe, fd: libc::c_int, fsync_flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_nop"]
    pub fn io_uring_prep_nop(sqe: *mut io_uring_sqe);

    #[link_name = "rust_io_uring_prep_timeout"]
    pub fn io_uring_prep_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        count: libc::c_uint,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_timeout_remove"]
    pub fn io_uring_prep_timeout_remove(
        sqe: *mut io_uring_sqe,
        user_data: libc::__u64,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_timeout_update"]
    pub fn io_uring_prep_timeout_update(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        user_data: libc::__u64,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_accept"]
    pub fn io_uring_prep_accept(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_accept_direct"]
    pub fn io_uring_prep_accept_direct(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
        file_index: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_multishot_accept"]
    pub fn io_uring_prep_multishot_accept(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_multishot_accept_direct"]
    pub fn io_uring_prep_multishot_accept_direct(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_cancel64"]
    pub fn io_uring_prep_cancel64(
        sqe: *mut io_uring_sqe,
        user_data: libc::__u64,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_cancel"]
    pub fn io_uring_prep_cancel(
        sqe: *mut io_uring_sqe,
        user_data: *mut libc::c_void,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_cancel_fd"]
    pub fn io_uring_prep_cancel_fd(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_link_timeout"]
    pub fn io_uring_prep_link_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_connect"]
    pub fn io_uring_prep_connect(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: libc::socklen_t,
    );

    #[link_name = "rust_io_uring_prep_files_update"]
    pub fn io_uring_prep_files_update(
        sqe: *mut io_uring_sqe,
        fds: *mut libc::c_int,
        nr_fds: libc::c_uint,
        offset: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_fallocate"]
    pub fn io_uring_prep_fallocate(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        mode: libc::c_int,
        offset: libc::off_t,
        len: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_openat"]
    pub fn io_uring_prep_openat(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mode: libc::mode_t,
    );

    #[link_name = "rust_io_uring_prep_openat_direct"]
    pub fn io_uring_prep_openat_direct(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mode: libc::mode_t,
        file_index: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_close"]
    pub fn io_uring_prep_close(sqe: *mut io_uring_sqe, fd: libc::c_int);

    #[link_name = "rust_io_uring_prep_close_direct"]
    pub fn io_uring_prep_close_direct(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        file_index: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_read"]
    pub fn io_uring_prep_read(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_write"]
    pub fn io_uring_prep_write(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *const libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_statx"]
    pub fn io_uring_prep_statx(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mask: libc::c_uint,
        statx: *mut libc::statx,
    );

    #[link_name = "rust_io_uring_prep_fadvise: libc::c_int"]
    pub fn io_uring_prep_fadvise(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        offset: libc::off_t,
        len: libc::off_t,
        advice: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_madvise"]
    pub fn io_uring_prep_madvise(
        sqe: *mut io_uring_sqe,
        addr: *mut libc::c_void,
        length: libc::off_t,
        advice: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_send"]
    pub fn io_uring_prep_send(
        sqe: *mut io_uring_sqe,
        sockfd: libc::c_int,
        buf: *const libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_send_zc"]
    pub fn io_uring_prep_send_zc(
        sqe: *mut io_uring_sqe,
        sockfd: libc::c_int,
        buf: *const libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
        zc_flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_send_zc_fixed"]
    pub fn io_uring_prep_send_zc_fixed(
        sqe: *mut io_uring_sqe,
        sockfd: libc::c_int,
        buf: *const libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
        zc_flags: libc::c_uint,
        buf_index: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_sendmsg_zc"]
    pub fn io_uring_prep_sendmsg_zc(
        sqe: *mut io_uring_sqe,
        sockfd: libc::c_int,
        msg: *const msghdr,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_send_set_addr"]
    pub fn io_uring_prep_send_set_addr(
        sqe: *mut io_uring_sqe,
        dest_addr: *const libc::sockaddr,
        addr_len: libc::__u16,
    );

    #[link_name = "rust_io_uring_prep_recv"]
    pub fn io_uring_prep_recv(
        sqe: *mut io_uring_sqe,
        sockfd: libc::c_int,
        buf: *mut libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
    );

    #[allow(improper_ctypes)]
    #[link_name = "rust_io_uring_prep_openat2"]
    pub fn io_uring_prep_openat2(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        how: *mut libc::open_how,
    );

    #[allow(improper_ctypes)]
    #[link_name = "rust_io_uring_prep_openat2_direct"]
    pub fn io_uring_prep_openat2_direct(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        how: *mut libc::open_how,
        file_index: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_epoll_ctl"]
    pub fn io_uring_prep_epoll_ctl(
        sqe: *mut io_uring_sqe,
        epfd: libc::c_int,
        fd: libc::c_int,
        op: libc::c_int,
        ev: *mut libc::epoll_event,
    );

    #[link_name = "rust_io_uring_prep_provide_buffers"]
    pub fn io_uring_prep_provide_buffers(
        sqe: *mut io_uring_sqe,
        addr: *mut libc::c_void,
        len: libc::c_int,
        nr: libc::c_int,
        bgid: libc::c_int,
        bid: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_remove_buffers"]
    pub fn io_uring_prep_remove_buffers(sqe: *mut io_uring_sqe, nr: libc::c_int, bgid: libc::c_int);

    #[link_name = "rust_io_uring_prep_shutdown"]
    pub fn io_uring_prep_shutdown(sqe: *mut io_uring_sqe, fd: libc::c_int, how: libc::c_int);

    #[link_name = "rust_io_uring_prep_unlinkat"]
    pub fn io_uring_prep_unlinkat(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_unlink"]
    pub fn io_uring_prep_unlink(
        sqe: *mut io_uring_sqe,
        path: *const libc::c_char,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_renameat"]
    pub fn io_uring_prep_renameat(
        sqe: *mut io_uring_sqe,
        olddfd: libc::c_int,
        oldpath: *const libc::c_char,
        newdfd: libc::c_int,
        newpath: *const libc::c_char,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_rename"]
    pub fn io_uring_prep_rename(
        sqe: *mut io_uring_sqe,
        oldpath: *const libc::c_char,
        newpath: *const libc::c_char,
    );

    #[link_name = "rust_io_uring_prep_sync_file_range"]
    pub fn io_uring_prep_sync_file_range(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        len: libc::c_uint,
        offset: libc::__u64,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_mkdirat"]
    pub fn io_uring_prep_mkdirat(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        mode: libc::mode_t,
    );

    #[link_name = "rust_io_uring_prep_mkdir"]
    pub fn io_uring_prep_mkdir(
        sqe: *mut io_uring_sqe,
        path: *const libc::c_char,
        mode: libc::mode_t,
    );

    #[link_name = "rust_io_uring_prep_symlinkat"]
    pub fn io_uring_prep_symlinkat(
        sqe: *mut io_uring_sqe,
        target: *const libc::c_char,
        newdirfd: libc::c_int,
        linkpath: *const libc::c_char,
    );

    #[link_name = "rust_io_uring_prep_symlink"]
    pub fn io_uring_prep_symlink(
        sqe: *mut io_uring_sqe,
        target: *const libc::c_char,
        linkpath: *const libc::c_char,
    );

    #[link_name = "rust_io_uring_prep_linkat"]
    pub fn io_uring_prep_linkat(
        sqe: *mut io_uring_sqe,
        olddfd: libc::c_int,
        oldpath: *const libc::c_char,
        newdfd: libc::c_int,
        newpath: *const libc::c_char,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_link"]
    pub fn io_uring_prep_link(
        sqe: *mut io_uring_sqe,
        oldpath: *const libc::c_char,
        newpath: *const libc::c_char,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_msg_ring"]
    pub fn io_uring_prep_msg_ring(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        len: libc::c_uint,
        data: libc::__u64,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_getxattr"]
    pub fn io_uring_prep_getxattr(
        sqe: *mut io_uring_sqe,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        path: *const libc::c_char,
        len: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_setxattr"]
    pub fn io_uring_prep_setxattr(
        sqe: *mut io_uring_sqe,
        name: *const libc::c_char,
        value: *const libc::c_char,
        path: *const libc::c_char,
        flags: libc::c_int,
        len: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_fgetxattr"]
    pub fn io_uring_prep_fgetxattr(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        len: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_fsetxattr"]
    pub fn io_uring_prep_fetxattr(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        name: *const libc::c_char,
        value: *const libc::c_char,
        flags: libc::c_int,
        len: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_socket"]
    pub fn io_uring_prep_socket(
        sqe: *mut io_uring_sqe,
        domain: libc::c_int,
        ty: libc::c_int,
        protocol: libc::c_int,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_socket_direct"]
    pub fn io_uring_prep_socket_direct(
        sqe: *mut io_uring_sqe,
        domain: libc::c_int,
        ty: libc::c_int,
        protocol: libc::c_int,
        file_index: libc::c_uint,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_socket_direct_alloc"]
    pub fn io_uring_prep_socket_direct_alloc(
        sqe: *mut io_uring_sqe,
        domain: libc::c_int,
        ty: libc::c_int,
        protocol: libc::c_int,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_sq_ready"]
    pub fn io_uring_sq_ready(ring: *const io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_sq_space_left"]
    pub fn io_uring_sq_space_left(ring: *const io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_sqring_wait"]
    pub fn io_uring_sqring_wait(ring: *mut io_uring) -> libc::c_int;

    #[link_name = "rust_io_uring_cq_ready"]
    pub fn io_uring_cq_ready(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_cq_has_overflow"]
    pub fn io_uring_cq_has_overflow(ring: *const io_uring) -> bool;

    #[link_name = "rust_io_uring_cq_eventfd_enabled"]
    pub fn io_uring_cq_eventfd_enabled(ring: *mut io_uring) -> bool;

    #[link_name = "rust_io_uring_cq_eventfd_toggle"]
    pub fn io_uring_cq_eventfd_toggle(ring: *mut io_uring, enabled: bool) -> libc::c_int;

    #[link_name = "rust_io_uring_wait_cqe_nr"]
    pub fn io_uring_wait_cqe_nr(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
    ) -> libc::c_int;

    #[link_name = "rust_io_uring_peek_cqe"]
    pub fn io_uring_peek_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;

    #[link_name = "rust_io_uring_wait_cqe"]
    pub fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;

    #[link_name = "rust_io_uring_buf_ring_mask"]
    pub fn io_uring_buf_ring_mask(ring_entries: libc::__u32) -> libc::c_int;

    #[link_name = "rust_io_uring_buf_ring_init"]
    pub fn io_uring_buf_ring_init(br: *mut io_uring_buf_ring);

    #[link_name = "rust_io_uring_buf_ring_add"]
    pub fn io_uring_buf_ring_add(
        br: *mut io_uring_buf_ring,
        addr: *mut libc::c_void,
        len: libc::c_uint,
        bid: libc::c_ushort,
        mask: libc::c_int,
        buf_offset: libc::c_int,
    );

    #[link_name = "rust_io_uring_buf_ring_advance"]
    pub fn io_uring_buf_ring_advance(br: *mut io_uring_buf_ring, count: libc::c_int);

    #[link_name = "rust_io_uring_buf_ring_cq_advance"]
    pub fn io_uring_buf_ring_cq_advance(ring: *mut io_uring, br: *mut io_uring_buf_ring, count: libc::c_int);

    #[link_name = "rust_io_uring_get_sqe"]
    pub fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
}
