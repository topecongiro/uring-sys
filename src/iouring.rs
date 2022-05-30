//! io_uring interface.

/// IO submission data structure (Submission Queue Entry)
#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: IoRingOp,    /* type of operation for this sqe */
    pub flags: libc::__u8,   /* IOSQE_ flags */
    pub ioprio: libc::__u16, /* ioprio for the request */
    pub fd: libc::__s32,     /* file descriptor to do IO on */
    pub off_addr2: off_addr2,
    pub addr_splice_off_in: addr_splice_off_in, /* pointer to buffer or iovecs */
    pub len: libc::__u32,                       /* buffer size or number of iovecs */
    pub cmd_flags: cmd_flags,
    pub user_data: libc::__u64, /* data to be passed back at completion time */
    pub buf_index: buf_index_padding, /* index into fixed buffers, if used */
}

#[repr(C)]
pub union off_addr2 {
    pub off: libc::__u64,
    pub addr2: libc::__u64,
}

#[repr(C)]
pub union addr_splice_off_in {
    pub addr: libc::__u64,
    pub splice_off_in: libc::__u64,
}

#[repr(C)]
pub union buf_index_padding {
    pub buf_index: buf_index,
    pub __pad2: [libc::__u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf_index {
    pub index_or_group: libc::__u16,
    pub personality: libc::__u16,
    pub splice_fd_in: libc::__s32,
}

#[repr(C)]
pub union cmd_flags {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: libc::__u32,
    pub poll_events: libc::__u16,
    pub sync_range_flags: libc::__u32,
    pub msg_flags: libc::__u32,
    pub timeout_flags: libc::__u32,
    pub accept_flags: libc::__u32,
    pub cancel_flags: libc::__u32,
    pub open_flags: libc::__u32,
    pub statx_flags: libc::__u32,
    pub fadvise_advice: libc::__u32,
    pub splice_flags: libc::__u32,
    pub rename_flags: libc::__u32,
    pub unlink_flags: libc::__u32,
    pub hardlink_flags: libc::__u32,
}

#[allow(non_camel_case_types)]
type __kernel_rwf_t = libc::c_int;

// sqe.flags
pub const IOSQE_FIXED_FILE: libc::__u8 = 1 << 0; /* use fixed fileset */
pub const IOSQE_IO_DRAIN: libc::__u8 = 1 << 1; /* issue after inflight IO */
pub const IOSQE_IO_LINK: libc::__u8 = 1 << 2; /* links next sqe */
pub const IOSQE_IO_HARDLINK: libc::__u8 = 1 << 3; /* like LINK, but stronger */
pub const IOSQE_ASYNC: libc::__u8 = 1 << 4; /* always go async */
pub const IOSQE_BUFFER_SELECT: libc::__u8 = 1 << 5; /* select buf from sqe->buf_group */
pub const IOSQE_CQE_SKIP_SUCCESS: libc::__u8 = 1 << 6; /* don't post CQE if request succeeded */

// io_uring_setup flags
/// `io_context` is polled.
pub const IORING_SETUP_IOPOLL: libc::c_uint = 1 << 0;
/// SQ poll thread.
pub const IORING_SETUP_SQPOLL: libc::c_uint = 1 << 1;
/// `sq_thread_cpu` is valid.
pub const IORING_SETUP_SQ_AFF: libc::c_uint = 1 << 2;
/// App defines CQ size.
pub const IORING_SETUP_CQSIZE: libc::c_uint = 1 << 3;
/// Clamp SQ/CQ ring sizes.
pub const IORING_SETUP_CLAMP: libc::c_uint = 1 << 4;
/// Attach to existing wq.
pub const IORING_SETUP_ATTACH_WQ: libc::c_uint = 1 << 5;
/// Start with ring disabled.
pub const IORING_SETUP_R_DISABLED: libc::c_uint = 1 << 6;
/// Continue submit on error.
pub const IORING_SETUP_SUBMIT_ALL: libc::c_uint = 1 << 7;
/// Cooperative task running.
///
/// When requests complete, they often require forcing the submitter to transition to
/// the kernel to complete.
/// If this flag is set, work will be done when the task transitions anyway, rather
/// than force an inter-processor interrupt reschedule. This avoids interrupting
/// a task running in userspace, and saves an IPI.
pub const IORING_SETUP_COOP_TASKRUN: libc::c_uint = 1 << 8;
/// If COOP_TASKRUN is set, get notified if task work is available for
/// running and a kernel transition would be needed to run it. This sets
/// IORING_SQ_TASKRUN in the sq ring flags. Not valid with COOP_TASKRUN.
pub const IORING_SETUP_TASKRUN_FLAG: libc::c_uint = 1 << 9;
/// SQEs are 128b.
pub const IORING_SETUP_SQE128: libc::c_uint = 1 << 10;
/// CQEs are 32b.
pub const IORING_SETUP_CQE32: libc::c_uint = 1 << 11;

#[repr(u8)]
#[non_exhaustive]
#[allow(nonstandard_style)]
#[derive(Clone, Copy, Debug)]
pub enum IoRingOp {
    IORING_OP_NOP = 0,
    IORING_OP_READV,
    IORING_OP_WRITEV,
    IORING_OP_FSYNC,
    IORING_OP_READ_FIXED,
    IORING_OP_WRITE_FIXED,
    IORING_OP_POLL_ADD,
    IORING_OP_POLL_REMOVE,
    IORING_OP_SYNC_FILE_RANGE,
    IORING_OP_SENDMSG,
    IORING_OP_RECVMSG,
    IORING_OP_TIMEOUT,
    IORING_OP_TIMEOUT_REMOVE,
    IORING_OP_ACCEPT,
    IORING_OP_ASYNC_CANCEL,
    IORING_OP_LINK_TIMEOUT,
    IORING_OP_CONNECT,
    IORING_OP_FALLOCATE,
    IORING_OP_OPENAT,
    IORING_OP_CLOSE,
    IORING_OP_FILES_UPDATE,
    IORING_OP_STATX,
    IORING_OP_READ,
    IORING_OP_WRITE,
    IORING_OP_FADVISE,
    IORING_OP_MADVISE,
    IORING_OP_SEND,
    IORING_OP_RECV,
    IORING_OP_OPENAT2,
    IORING_OP_EPOLL_CTL,
    IORING_OP_SPLICE,
    IORING_OP_PROVIDE_BUFFERS,
    IORING_OP_REMOVE_BUFFERS,
    IORING_OP_TEE,
    IORING_OP_SHUTDOWN,
    IORING_OP_RENAMEAT,
    IORING_OP_UNLINKAT,
    IORING_OP_MKDIRAT,
    IORING_OP_SYMLINKAT,
    IORING_OP_LINKAT,
    IORING_OP_MSG_RING,
    IORING_OP_FSETXATTR,
    IORING_OP_SETXATTR,
    IORING_OP_FGETXATTR,
    IORING_OP_GETXATTR,
    IORING_OP_SOCKET,
    IORING_OP_URING_CMD,

    /* this goes last, obviously */
    IORING_OP_LAST,
}

// sqe.cmd_flags.fsync_flags
pub const IORING_FSYNC_DATASYNC: libc::__u32 = 1 << 0;

// sqe.cmd_flags.timeout_flags
pub const IORING_TIMEOUT_ABS: libc::__u32 = 1 << 0;
pub const IORING_TIMEOUT_UPDATE: libc::__u32 = 1 << 1;
pub const IORING_TIMEOUT_BOOTTIME: libc::__u32 = 1 << 2;
pub const IORING_TIMEOUT_REALTIME: libc::__u32 = 1 << 3;
pub const IORING_LINK_TIMEOUT_UPDATE: libc::__u32 = 1 << 4;
pub const IORING_TIMEOUT_ETIME_SUCCESS: libc::__u32 = 1 << 5;
pub const IORING_TIMEOUT_CLOCK_MASK: libc::__u32 =
    IORING_TIMEOUT_BOOTTIME | IORING_TIMEOUT_REALTIME;
pub const IORING_TIMEOUT_UPDATE_MASK: libc::__u32 =
    IORING_TIMEOUT_UPDATE | IORING_LINK_TIMEOUT_UPDATE;

// sqe.cmd_flags.splice_flags
pub const SPLICE_F_FD_IN_FIXED: libc::__u32 = 1 << 31;

/*
 * POLL_ADD flags. Note that since sqe->poll_events is the flag space, the
 * command flags for POLL_ADD are stored in sqe->len.
 *
 * IORING_POLL_ADD_MULTI	Multishot poll. Sets IORING_CQE_F_MORE if
 *				the poll handler will continue to report
 *				CQEs on behalf of the same SQE.
 *
 * IORING_POLL_UPDATE		Update existing poll request, matching
 *				sqe->addr as the old user_data field.
 */
pub const IORING_POLL_ADD_MULTI: libc::__u32 = 1 << 0;
pub const IORING_POLL_UPDATE_EVENTS: libc::__u32 = 1 << 1;
pub const IORING_POLL_UPDATE_USER_DATA: libc::__u32 = 1 << 2;

/// If set, instead of first attempting to send or receive and arm poll if that yields an
/// -EAGAIN result, arm poll upfront and skip the initial transfer attempt.
///
/// `send`/`sendmsg` and `recv`/`recvmsg` flags (`sqe.addr2`).
pub const IORING_RECVSEND_POLL_FIRST: libc::__u32 = 1 << 0;

/// IO completion data structure (Completion Queue Entry)
#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: libc::__u64, /* sqe->data submission passed back */
    pub res: libc::__s32,       /* result code for this event */
    pub flags: libc::__u32,
}

// cqe.flags
/// If set, the upper 16 bits are the buffer ID
pub const IORING_CQE_F_BUFFER: libc::__u32 = 1 << 0;
/// If set, parent SQE will generate more CQE entries
pub const IORING_CQE_F_MORE: libc::__u32 = 1 << 1;
/// If set, more data to read after socket recv.
pub const IORING_CQE_F_SOCK_NONEMPTY: libc::__u32 = 1 << 2;

pub const IORING_CQE_BUFFER_SHIFT: libc::c_int = 16;

// Magic offsets for the application to mmap the data it needs
pub const IORING_OFF_SQ_RING: libc::__u64 = 0;
pub const IORING_OFF_CQ_RING: libc::__u64 = 0x8000000;
pub const IORING_OFF_SQES: libc::__u64 = 0x10000000;

// Filled with the offset for mmap(2)
#[repr(C)]
pub struct io_sqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub flags: libc::__u32,
    pub dropped: libc::__u32,
    pub array: libc::__u32,
    pub resv1: libc::__u32,
    pub resv2: libc::__u64,
}

// sq_ring.kflags
/// Needs `io_uring_enter` wakeup
pub const IORING_SQ_NEED_WAKEUP: libc::c_uint = 1 << 0;
/// CQ ring is overflown
pub const IORING_SQ_CQ_OVERFLOW: libc::c_uint = 1 << 1;
/// Task should enter the kernel
pub const IORING_SQ_CQ_TASKRUN: libc::c_uint = 1 << 2;

#[repr(C)]
pub struct io_cqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub overflow: libc::__u32,
    pub cqes: libc::__u32,
    pub flags: libc::__u32,
    pub resv1: libc::__u32,
    pub resv2: libc::__u64,
}

// cq_ring.kflags
pub const IORING_CQ_EVENTFD_DISABLED: libc::c_uint = 1 << 0;

pub const IORING_ENTER_GETEVENTS: libc::c_uint = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP: libc::c_uint = 1 << 1;
pub const IORING_ENTER_SQ_WAIT: libc::c_uint = 1 << 2;
pub const IORING_ENTER_EXT_ARG: libc::c_uint = 1 << 3;

/// Passed in for io_uring_setup(2). Copied back with updated info on success
#[repr(C)]
pub struct io_uring_params {
    pub sq_entries: libc::__u32,
    pub cq_entries: libc::__u32,
    pub flags: libc::__u32,
    pub sq_thread_cpu: libc::__u32,
    pub sq_thread_idle: libc::__u32,
    pub features: libc::__u32,
    pub wq_fd: libc::__u32,
    pub resv: [libc::__u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

// io_uring_params.features flags
pub const IORING_FEAT_SINGLE_MMAP: libc::__u32 = 1 << 0;
pub const IORING_FEAT_NODROP: libc::__u32 = 1 << 1;
pub const IORING_FEAT_SUBMIT_STABLE: libc::__u32 = 1 << 2;
pub const IORING_FEAT_RW_CUR_POS: libc::__u32 = 1 << 3;
pub const IORING_FEAT_CUR_PERSONALITY: libc::__u32 = 1 << 4;
pub const IORING_FEAT_FAST_POLL: libc::__u32 = 1 << 5;
pub const IORING_FEAT_POLL_32BITS: libc::__u32 = 1 << 6;
pub const IORING_FEAT_SQPOLL_NONFIXED: libc::__u32 = 1 << 7;
pub const IORING_FEAT_EXT_ARG: libc::__u32 = 1 << 8;
pub const IORING_FEAT_NATIVE_WORKERS: libc::__u32 = 1 << 9;
pub const IORING_FEAT_RSRC_TAGS: libc::__u32 = 1 << 10;
pub const IORING_FEAT_CQE_SKIP: libc::__u32 = 1 << 11;
pub const IORING_FEAT_LINKED_FILE: libc::__u32 = 1 << 12;

#[repr(C)]
#[non_exhaustive]
#[allow(nonstandard_style)]
#[derive(Clone, Copy, Debug)]
pub enum IoUringRegisterOp {
    IORING_REGISTER_BUFFERS = 0,
    IORING_UNREGISTER_BUFFERS = 1,
    IORING_REGISTER_FILES = 2,
    IORING_UNREGISTER_FILES = 3,
    IORING_REGISTER_EVENTFD = 4,
    IORING_UNREGISTER_EVENTFD = 5,
    IORING_REGISTER_FILES_UPDATE = 6,
    IORING_REGISTER_EVENTFD_ASYNC = 7,
    IORING_REGISTER_PROBE = 8,
    IORING_REGISTER_PERSONALITY = 9,
    IORING_UNREGISTER_PERSONALITY = 10,
    IORING_REGISTER_RESTRICTIONS = 11,
    IORING_REGISTER_ENABLE_RINGS = 12,

    /* extended with tagging */
    IORING_REGISTER_FILES2 = 13,
    IORING_REGISTER_FILES_UPDATE2 = 14,
    IORING_REGISTER_BUFFERS2 = 15,
    IORING_REGISTER_BUFFERS_UPDATE = 16,

    /* set/clear io-wq thread affinities */
    IORING_REGISTER_IOWQ_AFF = 17,
    IORING_UNREGISTER_IOWQ_AFF = 18,

    /* set/get max number of io-wq workers */
    IORING_REGISTER_IOWQ_MAX_WORKERS = 19,

    IORING_REGISTER_RING_FDS = 20,
    IORING_UNREGISTER_RING_FDS = 21,

    /* register ring based provide buffer group */
    IORING_REGISTER_PBUF_RING = 22,
    IORING_UNREGISTER_PBUF_RING = 23,

    /* this goes last */
    IORING_REGISTER_LAST,
}

/*
io-wq worker categories.
Applications can index the array they pass in to IORING_REGISTER_IOWQ_MAX_WORKERS
instead of using non-specific 0 and 1 values for bound/unbound.
*/
pub const IO_WQ_BOUND: usize = 0;
pub const IO_WQ_UNBOUND: usize = 1;

#[repr(C, align(8))]
pub struct io_uring_rsrc_register {
    pub nr: libc::__u32,
    pub resv: libc::__u32,
    pub resv2: libc::__u64,
    pub data: libc::__u64,
    pub tags: libc::__u64,
}

#[repr(C, align(8))]
pub struct io_uring_rsrc_update {
    pub offset: libc::__u32,
    pub resv: libc::__u32,
    pub data: libc::__u64,
}

#[repr(C, align(8))]
pub struct io_uring_rsrc_update2 {
    pub offset: libc::__u32,
    pub resv: libc::__u32,
    pub data: libc::__u64,
    pub tags: libc::__u64,
    pub nr: libc::__u32,
    pub resv2: libc::__u32,
}

/// Skip updating fd indexes set to this value in the fd table
pub const IORING_REGISTER_FILES_SKIP: libc::c_int = -2;

#[repr(C)]
pub struct io_uring_probe_op {
    pub op: libc::__u8,
    pub resv: libc::__u8,
    pub flags: libc::__u16,
    pub resv2: libc::__u32,
}

#[repr(C)]
pub struct io_uring_probe {
    pub last_op: libc::__u8,
    pub ops_len: libc::__u8,
    pub resv: libc::__u16,
    pub resv2: [libc::__u32; 3],
    pub ops: [io_uring_probe_op; 0],
}

#[repr(C)]
pub struct io_uring_restriction {
    pub opcode: libc::__u16,
    pub op_flags: io_uring_restriction_op_flags,
    pub resv: libc::__u8,
    pub resv2: [libc::__u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct io_uring_buf {
    pub addr: libc::__u64,
    pub len: libc::__u32,
    pub bid: libc::__u16,
    pub resv: libc::__u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union io_uring_buf_ring {
    pub tail: io_uring_buf_ring_tail,
    pub bufs: [io_uring_buf; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct io_uring_buf_ring_tail {
    pub resv1: libc::__u64,
    pub resv2: libc::__u32,
    pub resv3: libc::__u16,
    pub tail: libc::__u16,
}

#[repr(C)]
pub struct io_uring_buf_reg {
    pub ring_addr: libc::__u64,
    pub ring_entries: libc::__u32,
    pub bgid: libc::__u16,
    pub pad: libc::__u16,
    pub resv: [libc::__u64; 3],
}

#[repr(C)]
pub union io_uring_restriction_op_flags {
    pub register_op: libc::__u8, /* IORING_RESTRICTION_REGISTER_OP */
    pub sqe_op: libc::__u8,      /* IORING_RESTRICTION_SQE_OP */
    pub sqe_flags: libc::__u8,   /* IORING_RESTRICTION_SQE_FLAGS_* */
}

/// io_uring_restriction->opcode values
#[repr(u16)]
#[non_exhaustive]
#[allow(nonstandard_style)]
#[derive(Clone, Copy, Debug)]
pub enum IoUringRestrictionOpcode {
    /// Allow an io_uring_register(2) opcode
    IORING_RESTRICTION_REGISTER_OP = 0,
    ///Allow an sqe opcode
    IORING_RESTRICTION_SQE_OP = 1,
    /// Allow sqe flags
    IORING_RESTRICTION_SQE_FLAGS_ALLOWED = 2,
    /// Require sqe flags (these flags must be set on each submission)
    IORING_RESTRICTION_SQE_FLAGS_REQUIRED = 3,
}

#[repr(C)]
pub struct io_uring_getevents_arg {
    pub sigmask: libc::__u64,
    pub sigmask_sz: libc::__u32,
    pub pad: libc::__u32,
    pub ts: libc::__u64,
}
