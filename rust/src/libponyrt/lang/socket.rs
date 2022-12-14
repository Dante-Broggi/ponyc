use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:4"]
pub mod _types_h {
    #[c2rust::src_loc = "43:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "46:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "120:1"]
    pub type __darwin_socklen_t = u32;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:4"]
pub mod sys__types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "73:1"]
    pub type __darwin_sigset_t = u32;
    #[c2rust::src_loc = "75:1"]
    pub type __darwin_uid_t = u32;
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int32_t.h:4"]
pub mod _u_int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type u_int32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:4"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:4"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sigset_t.h:4"]
pub mod _sigset_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sigset_t = __darwin_sigset_t;
    use super::sys__types_h::__darwin_sigset_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:4"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:4"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/signal.h:4"]
pub mod signal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct __siginfo {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub si_pid: pid_t,
        pub si_uid: uid_t,
        pub si_status: libc::c_int,
        pub si_addr: *mut libc::c_void,
        pub si_value: sigval,
        pub si_band: libc::c_long,
        pub __pad: [libc::c_ulong; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "269:7"]
    pub union __sigaction_u {
        pub __sa_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
        pub __sa_sigaction:
            Option<unsafe extern "C" fn(libc::c_int, *mut __siginfo, *mut libc::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "286:9"]
    pub struct sigaction {
        pub __sigaction_u: __sigaction_u,
        pub sa_mask: sigset_t,
        pub sa_flags: libc::c_int,
    }
    use super::_pid_t_h::pid_t;
    use super::_sigset_t_h::sigset_t;
    use super::_uid_t_h::uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:4"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:4"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:4"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_iovec_t.h:4"]
pub mod _iovec_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:6"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type pony_trace_fn = Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "84:1"]
    pub type pony_serialise_fn = Option<
        unsafe extern "C" fn(
            *mut pony_ctx_t,
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            libc::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> size_t>;
    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;
    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:22"]
    pub struct _pony_type_t {
        pub id: u32,
        pub size: u32,
        pub field_count: u32,
        pub field_offset: u32,
        pub instance: *mut libc::c_void,
        pub trace: pony_trace_fn,
        pub serialise_trace: pony_trace_fn,
        pub serialise: pony_serialise_fn,
        pub deserialise: pony_trace_fn,
        pub custom_serialise_space: pony_custom_serialise_space_fn,
        pub custom_deserialise: pony_custom_deserialise_fn,
        pub dispatch: pony_dispatch_fn,
        pub final_0: pony_final_fn,
        pub event_notify: u32,
        pub traits: *mut *mut uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
        #[c2rust::src_loc = "262:1"]
        pub fn pony_alloc(ctx: *mut pony_ctx_t, size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "508:1"]
        pub fn pony_error();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/asio.h:6"]
pub mod asio_h {
    #[c2rust::src_loc = "24:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "32:3"]
    pub const ASIO_DESTROYED: C2RustUnnamed = 4294967295;
    #[c2rust::src_loc = "31:3"]
    pub const ASIO_ONESHOT: C2RustUnnamed = 256;
    #[c2rust::src_loc = "30:3"]
    pub const ASIO_SIGNAL: C2RustUnnamed = 8;
    #[c2rust::src_loc = "29:3"]
    pub const ASIO_TIMER: C2RustUnnamed = 4;
    #[c2rust::src_loc = "28:3"]
    pub const ASIO_WRITE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "27:3"]
    pub const ASIO_READ: C2RustUnnamed = 1;
    #[c2rust::src_loc = "26:3"]
    pub const ASIO_DISPOSABLE: C2RustUnnamed = 0;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/event.h:7"]
pub mod event_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:16"]
    pub struct asio_event_t {
        pub magic: *mut asio_event_t,
        pub owner: *mut pony_actor_t,
        pub msg_id: u32,
        pub fd: libc::c_int,
        pub flags: u32,
        pub noisy: bool,
        pub readable: bool,
        pub writeable: bool,
        pub nsec: u64,
    }
    use super::pony_h::pony_actor_t;
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn pony_asio_event_create(
            owner: *mut pony_actor_t,
            fd: libc::c_int,
            flags: u32,
            nsec: u64,
            noisy: bool,
        ) -> *mut asio_event_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:23"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:23"]
pub mod _socklen_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:23"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "416:8"]
    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "464:8"]
    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: sa_family_t,
        pub __ss_pad1: [libc::c_char; 6],
        pub __ss_align: __int64_t,
        pub __ss_pad2: [libc::c_char; 112],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_ssize_t_h::ssize_t;
    use super::_types_h::{__int64_t, __uint8_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "724:1"]
        pub fn accept(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
        #[c2rust::src_loc = "726:1"]
        pub fn bind(_: libc::c_int, _: *const sockaddr, _: socklen_t) -> libc::c_int;
        #[c2rust::src_loc = "727:1"]
        pub fn connect(_: libc::c_int, _: *const sockaddr, _: socklen_t) -> libc::c_int;
        #[c2rust::src_loc = "728:1"]
        pub fn getpeername(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
        #[c2rust::src_loc = "730:1"]
        pub fn getsockname(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
        #[c2rust::src_loc = "732:1"]
        pub fn getsockopt(
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut libc::c_void,
            _: *mut socklen_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "733:1"]
        pub fn listen(_: libc::c_int, _: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "734:1"]
        pub fn recv(_: libc::c_int, _: *mut libc::c_void, _: size_t, _: libc::c_int) -> ssize_t;
        #[c2rust::src_loc = "735:1"]
        pub fn recvfrom(
            _: libc::c_int,
            _: *mut libc::c_void,
            _: size_t,
            _: libc::c_int,
            _: *mut sockaddr,
            _: *mut socklen_t,
        ) -> ssize_t;
        #[c2rust::src_loc = "738:1"]
        pub fn send(_: libc::c_int, _: *const libc::c_void, _: size_t, _: libc::c_int) -> ssize_t;
        #[c2rust::src_loc = "740:1"]
        pub fn sendto(
            _: libc::c_int,
            _: *const libc::c_void,
            _: size_t,
            _: libc::c_int,
            _: *const sockaddr,
            _: socklen_t,
        ) -> ssize_t;
        #[c2rust::src_loc = "742:1"]
        pub fn setsockopt(
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: *const libc::c_void,
            _: socklen_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "743:1"]
        pub fn shutdown(_: libc::c_int, _: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "745:1"]
        pub fn socket(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:24"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:8"]
    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "505:8"]
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:24"]
pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:16"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_0 {
        pub __u6_addr8: [u8; 16],
        pub __u6_addr16: [u16; 8],
        pub __u6_addr32: [u32; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:8"]
    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "537:8"]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: libc::c_uint,
    }
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
    extern "C" {
        #[c2rust::src_loc = "213:30"]
        pub static in6addr_any: in6_addr;
        #[c2rust::src_loc = "214:30"]
        pub static in6addr_loopback: in6_addr;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/tcp.h:25"]
pub mod tcp_h {
    #[c2rust::src_loc = "77:1"]
    pub type tcp_seq = u32;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "87:8"]
    pub struct tcphdr {
        pub th_sport: libc::c_ushort,
        pub th_dport: libc::c_ushort,
        pub th_seq: tcp_seq,
        pub th_ack: tcp_seq,
        #[bitfield(name = "th_x2", ty = "libc::c_uint", bits = "0..=3")]
        #[bitfield(name = "th_off", ty = "libc::c_uint", bits = "4..=7")]
        pub th_x2_th_off: [u8; 1],
        pub th_flags: libc::c_uchar,
        pub th_win: libc::c_ushort,
        pub th_sum: libc::c_ushort,
        pub th_urp: libc::c_ushort,
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:27"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut libc::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[c2rust::src_loc = "269:1"]
        pub fn freeaddrinfo(_: *mut addrinfo);
        #[c2rust::src_loc = "271:1"]
        pub fn getaddrinfo(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *const addrinfo,
            _: *mut *mut addrinfo,
        ) -> libc::c_int;
        #[c2rust::src_loc = "277:1"]
        pub fn getnameinfo(
            _: *const sockaddr,
            _: socklen_t,
            _: *mut libc::c_char,
            _: socklen_t,
            _: *mut libc::c_char,
            _: socklen_t,
            _: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/signal.h:4"]
pub mod include_signal_h {
    use super::signal_h::sigaction;
    extern "C" {
        #[c2rust::src_loc = "84:1"]
        pub fn sigaction(_: libc::c_int, _: *const sigaction, _: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libkern/i386/_OSByteOrder.h:4"]
pub mod _OSByteOrder_h {
    #[inline]
    #[c2rust::src_loc = "53:1"]
    pub unsafe extern "C" fn _OSSwapInt32(mut _data: u32) -> u32 {
        _data.swap_bytes()
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:4"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/uio.h:4"]
pub mod uio_h {
    use super::_iovec_t_h::iovec;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "99:1"]
        pub fn writev(_: libc::c_int, _: *const iovec, _: libc::c_int) -> ssize_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/fcntl.h:4"]
pub mod fcntl_h {
    extern "C" {
        #[c2rust::src_loc = "580:1"]
        pub fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "71:6"]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:26"]
pub mod inet_h {
    use super::_socklen_t_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn inet_ntop(
            _: libc::c_int,
            _: *const libc::c_void,
            _: *mut libc::c_char,
            _: socklen_t,
        ) -> *const libc::c_char;
        #[c2rust::src_loc = "78:1"]
        pub fn inet_pton(
            _: libc::c_int,
            _: *const libc::c_char,
            _: *mut libc::c_void,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:29"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
    }
}
pub use self::_OSByteOrder_h::_OSSwapInt32;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_iovec_t_h::iovec;
pub use self::_pid_t_h::pid_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_sigset_t_h::sigset_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_types_h::{
    __darwin_socklen_t, __darwin_ssize_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::asio_h::{
    C2RustUnnamed, ASIO_DESTROYED, ASIO_DISPOSABLE, ASIO_ONESHOT, ASIO_READ, ASIO_SIGNAL,
    ASIO_TIMER, ASIO_WRITE,
};
use self::errno_h::__error;
pub use self::event_h::{asio_event_t, pony_asio_event_create};
use self::fcntl_h::fcntl;
pub use self::in6_h::{
    in6_addr, in6addr_any, in6addr_loopback, ipv6_mreq, sockaddr_in6, C2RustUnnamed_0,
};
pub use self::in_h::{in_addr, ip_mreq, sockaddr_in};
use self::include_signal_h::sigaction;
use self::inet_h::{inet_ntop, inet_pton};
pub use self::netdb_h::{addrinfo, freeaddrinfo, getaddrinfo, getnameinfo};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_alloc, pony_ctx, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_error, pony_final_fn, pony_msg_t,
    pony_serialise_fn, pony_trace_fn, pony_type_t,
};
pub use self::signal_h::{__sigaction_u, __siginfo, sigaction, sigval};
pub use self::socket_h::{
    accept, bind, connect, getpeername, getsockname, getsockopt, listen, recv, recvfrom, send,
    sendto, setsockopt, shutdown, sockaddr, sockaddr_storage, socket,
};
pub use self::stddef_h::size_t;
use self::string_h::{memcmp, memcpy, memset, strlen};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_sigset_t, __darwin_uid_t};
pub use self::tcp_h::{tcp_seq, tcphdr};
use self::uio_h::writev;
use self::unistd_h::close;
#[c2rust::src_loc = "30:1"]
pub type SOCKET = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "93:9"]
pub struct ipaddress_t {
    pub type_0: *const pony_type_t,
    pub addr: sockaddr_storage,
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn ponyint_address_length(mut ipaddr: *mut ipaddress_t) -> socklen_t {
    match (*ipaddr).addr.ss_family as libc::c_int {
        2 => return ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        30 => return ::core::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
        _ => {}
    }
    return -(1 as libc::c_int) as socklen_t;
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn map_any_to_loopback(mut addr: *mut sockaddr) -> bool {
    match (*addr).sa_family as libc::c_int {
        2 => {
            let mut in_0: *mut sockaddr_in = addr as *mut sockaddr_in;
            if (*in_0).sin_addr.s_addr == 0 as libc::c_int as u32 {
                (*in_0).sin_addr.s_addr = if 0 != 0 {
                    (0x7f000001 as libc::c_int as u32 & 0xff000000 as libc::c_uint)
                        >> 24 as libc::c_int
                        | (0x7f000001 as libc::c_int as u32 & 0xff0000 as libc::c_uint)
                            >> 8 as libc::c_int
                        | (0x7f000001 as libc::c_int as u32 & 0xff00 as libc::c_uint)
                            << 8 as libc::c_int
                        | (0x7f000001 as libc::c_int as u32 & 0xff as libc::c_uint)
                            << 24 as libc::c_int
                } else {
                    _OSSwapInt32(0x7f000001 as libc::c_int as u32)
                };
                return 1 as libc::c_int != 0;
            }
        }
        30 => {
            let mut in_1: *mut sockaddr_in6 = addr as *mut sockaddr_in6;
            if memcmp(
                &mut (*in_1).sin6_addr as *mut in6_addr as *const libc::c_void,
                &in6addr_any as *const in6_addr as *const libc::c_void,
                ::core::mem::size_of::<in6_addr>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                memcpy(
                    &mut (*in_1).sin6_addr as *mut in6_addr as *mut libc::c_void,
                    &in6addr_loopback as *const in6_addr as *const libc::c_void,
                    ::core::mem::size_of::<in6_addr>() as libc::c_ulong,
                );
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn os_addrinfo_intern(
    mut family: libc::c_int,
    mut socktype: libc::c_int,
    mut proto: libc::c_int,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut passive: bool,
) -> *mut addrinfo {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_canonname: 0 as *mut libc::c_char,
        ai_addr: 0 as *mut sockaddr,
        ai_next: 0 as *mut addrinfo,
    };
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x400 as libc::c_int;
    hints.ai_family = family;
    hints.ai_socktype = socktype;
    hints.ai_protocol = proto;
    if passive {
        hints.ai_flags |= 0x1 as libc::c_int;
    }
    if !host.is_null() && *host.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        host = 0 as *const libc::c_char;
    }
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    if getaddrinfo(host, service, &mut hints, &mut result) != 0 as libc::c_int {
        return 0 as *mut addrinfo;
    }
    result
}
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn set_nonblocking(mut s: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = fcntl(s, 3 as libc::c_int, 0 as libc::c_int);
    return fcntl(s, 4 as libc::c_int, flags | 0x4 as libc::c_int);
}
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn socket_from_addrinfo(mut p: *mut addrinfo, mut reuse: bool) -> libc::c_int {
    let mut fd: libc::c_int = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut r: libc::c_int = 0 as libc::c_int;
    if reuse {
        let mut reuseaddr: libc::c_int = 1 as libc::c_int;
        r |= setsockopt(
            fd,
            0xffff as libc::c_int,
            0x4 as libc::c_int,
            &mut reuseaddr as *mut libc::c_int as *const libc::c_char as *const libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
    }
    r |= set_nonblocking(fd);
    if r == 0 as libc::c_int {
        return fd;
    }
    pony_os_socket_close(fd);
    return -(1 as libc::c_int);
}
#[c2rust::src_loc = "494:1"]
unsafe extern "C" fn os_listen(
    mut owner: *mut pony_actor_t,
    mut fd: libc::c_int,
    mut p: *mut addrinfo,
    mut _proto: libc::c_int,
) -> *mut asio_event_t {
    if bind(
        fd,
        (*p).ai_addr,
        (*p).ai_addrlen as libc::c_int as socklen_t,
    ) != 0 as libc::c_int
    {
        pony_os_socket_close(fd);
        return 0 as *mut asio_event_t;
    }
    if (*p).ai_socktype == 1 as libc::c_int {
        if listen(fd, 128 as libc::c_int) != 0 as libc::c_int {
            pony_os_socket_close(fd);
            return 0 as *mut asio_event_t;
        }
    }
    let mut ev: *mut asio_event_t = pony_asio_event_create(
        owner,
        fd,
        ASIO_READ as libc::c_int as u32,
        0 as libc::c_int as u64,
        1 as libc::c_int != 0,
    );
    return ev;
}
#[c2rust::src_loc = "533:1"]
unsafe extern "C" fn os_connect(
    mut owner: *mut pony_actor_t,
    mut fd: libc::c_int,
    mut p: *mut addrinfo,
    mut from: *const libc::c_char,
    mut asio_flags: u32,
) -> bool {
    map_any_to_loopback((*p).ai_addr);
    let mut need_bind: bool =
        !from.is_null() && *from.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32;
    if need_bind {
        let mut result: *mut addrinfo = os_addrinfo_intern(
            (*p).ai_family,
            0 as libc::c_int,
            0 as libc::c_int,
            from,
            0 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
        if result.is_null() {
            return 0 as libc::c_int != 0;
        }
        let mut lp: *mut addrinfo = result;
        let mut bound: bool = 0 as libc::c_int != 0;
        while !lp.is_null() {
            if bind(
                fd,
                (*lp).ai_addr,
                (*lp).ai_addrlen as libc::c_int as socklen_t,
            ) == 0 as libc::c_int
            {
                bound = 1 as libc::c_int != 0;
                break;
            } else {
                lp = (*lp).ai_next;
            }
        }
        freeaddrinfo(result);
        if !bound {
            pony_os_socket_close(fd);
            return 0 as libc::c_int != 0;
        }
    }
    let mut r: libc::c_int = connect(
        fd,
        (*p).ai_addr,
        (*p).ai_addrlen as libc::c_int as socklen_t,
    );
    if r != 0 as libc::c_int && *__error() != 36 as libc::c_int {
        pony_os_socket_close(fd);
        return 0 as libc::c_int != 0;
    }
    pony_asio_event_create(
        owner,
        fd,
        asio_flags,
        0 as libc::c_int as u64,
        1 as libc::c_int != 0,
    );
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn os_socket_listen(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut family: libc::c_int,
    mut socktype: libc::c_int,
    mut proto: libc::c_int,
) -> *mut asio_event_t {
    let mut result: *mut addrinfo = os_addrinfo_intern(
        family,
        socktype,
        proto,
        host,
        service,
        1 as libc::c_int != 0,
    );
    if result.is_null() {
        return 0 as *mut asio_event_t;
    }
    let mut p: *mut addrinfo = result;
    while !p.is_null() {
        let mut fd: libc::c_int = socket_from_addrinfo(p, 1 as libc::c_int != 0);
        if fd != -(1 as libc::c_int) {
            let mut ev: *mut asio_event_t = os_listen(owner, fd, p, proto);
            freeaddrinfo(result);
            return ev;
        }
        p = (*p).ai_next;
    }
    freeaddrinfo(result);
    return 0 as *mut asio_event_t;
}
#[c2rust::src_loc = "647:1"]
unsafe extern "C" fn os_socket_connect(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut from: *const libc::c_char,
    mut family: libc::c_int,
    mut socktype: libc::c_int,
    mut proto: libc::c_int,
    mut asio_flags: u32,
) -> libc::c_int {
    let mut reuse: bool =
        from.is_null() || *from.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32;
    let mut result: *mut addrinfo = os_addrinfo_intern(
        family,
        socktype,
        proto,
        host,
        service,
        0 as libc::c_int != 0,
    );
    if result.is_null() {
        return 0 as libc::c_int;
    }
    let mut p: *mut addrinfo = result;
    let mut count: libc::c_int = 0 as libc::c_int;
    while !p.is_null() {
        let mut fd: libc::c_int = socket_from_addrinfo(p, reuse);
        if fd != -(1 as libc::c_int) {
            if os_connect(owner, fd, p, from, asio_flags) {
                count += 1;
            }
        }
        p = (*p).ai_next;
    }
    freeaddrinfo(result);
    return count;
}
#[no_mangle]
#[c2rust::src_loc = "679:1"]
pub unsafe extern "C" fn pony_os_listen_tcp(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        0 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "686:1"]
pub unsafe extern "C" fn pony_os_listen_tcp4(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        2 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "693:1"]
pub unsafe extern "C" fn pony_os_listen_tcp6(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        30 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "700:1"]
pub unsafe extern "C" fn pony_os_listen_udp(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        0 as libc::c_int,
        2 as libc::c_int,
        17 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "707:1"]
pub unsafe extern "C" fn pony_os_listen_udp4(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        2 as libc::c_int,
        2 as libc::c_int,
        17 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "714:1"]
pub unsafe extern "C" fn pony_os_listen_udp6(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut asio_event_t {
    return os_socket_listen(
        owner,
        host,
        service,
        30 as libc::c_int,
        2 as libc::c_int,
        17 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "721:1"]
pub unsafe extern "C" fn pony_os_connect_tcp(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut from: *const libc::c_char,
    mut asio_flags: u32,
) -> libc::c_int {
    return os_socket_connect(
        owner,
        host,
        service,
        from,
        0 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
        asio_flags,
    );
}
#[no_mangle]
#[c2rust::src_loc = "728:1"]
pub unsafe extern "C" fn pony_os_connect_tcp4(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut from: *const libc::c_char,
    mut asio_flags: u32,
) -> libc::c_int {
    return os_socket_connect(
        owner,
        host,
        service,
        from,
        2 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
        asio_flags,
    );
}
#[no_mangle]
#[c2rust::src_loc = "735:1"]
pub unsafe extern "C" fn pony_os_connect_tcp6(
    mut owner: *mut pony_actor_t,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
    mut from: *const libc::c_char,
    mut asio_flags: u32,
) -> libc::c_int {
    return os_socket_connect(
        owner,
        host,
        service,
        from,
        30 as libc::c_int,
        1 as libc::c_int,
        6 as libc::c_int,
        asio_flags,
    );
}
#[no_mangle]
#[c2rust::src_loc = "742:1"]
pub unsafe extern "C" fn pony_os_accept(mut ev: *mut asio_event_t) -> libc::c_int {
    let mut ns: libc::c_int = accept((*ev).fd, 0 as *mut sockaddr, 0 as *mut socklen_t);
    if ns != -(1 as libc::c_int) {
        set_nonblocking(ns);
    } else if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
        ns = 0 as libc::c_int;
    }
    ns
}
#[no_mangle]
#[c2rust::src_loc = "767:1"]
pub unsafe extern "C" fn pony_os_connected(mut fd: libc::c_int) -> bool {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut len: socklen_t = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    if getsockopt(
        fd,
        0xffff as libc::c_int,
        0x1007 as libc::c_int,
        &mut val as *mut libc::c_int as *mut libc::c_char as *mut libc::c_void,
        &mut len,
    ) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    return val == 0 as libc::c_int;
}
#[c2rust::src_loc = "778:1"]
unsafe extern "C" fn address_family(mut length: libc::c_int) -> libc::c_int {
    match length {
        4 => return 2 as libc::c_int,
        16 => return 30 as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "792:1"]
pub unsafe extern "C" fn pony_os_nameinfo(
    mut ipaddr: *mut ipaddress_t,
    mut rhost: *mut *mut libc::c_char,
    mut rserv: *mut *mut libc::c_char,
    mut reversedns: bool,
    mut servicename: bool,
) -> bool {
    let mut host: [libc::c_char; 1025] = [0; 1025];
    let mut serv: [libc::c_char; 32] = [0; 32];
    let mut len: socklen_t = ponyint_address_length(ipaddr);
    if len == -(1 as libc::c_int) as socklen_t {
        return 0 as libc::c_int != 0;
    }
    let mut flags: libc::c_int = 0 as libc::c_int;
    if !reversedns {
        flags |= 0x2 as libc::c_int;
    }
    if !servicename {
        flags |= 0x8 as libc::c_int;
    }
    let mut r: libc::c_int = getnameinfo(
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr,
        len,
        host.as_mut_ptr(),
        1025 as libc::c_int as socklen_t,
        serv.as_mut_ptr(),
        32 as libc::c_int as socklen_t,
        flags,
    );
    if r != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    let mut hostlen: size_t = strlen(host.as_mut_ptr());
    *rhost = pony_alloc(ctx, hostlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy(
        *rhost as *mut libc::c_void,
        host.as_mut_ptr() as *const libc::c_void,
        hostlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let mut servlen: size_t = strlen(serv.as_mut_ptr());
    *rserv = pony_alloc(ctx, servlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy(
        *rserv as *mut libc::c_void,
        serv.as_mut_ptr() as *const libc::c_void,
        servlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "830:1"]
pub unsafe extern "C" fn pony_os_addrinfo(
    mut family: libc::c_int,
    mut host: *const libc::c_char,
    mut service: *const libc::c_char,
) -> *mut addrinfo {
    match family {
        0 => {
            family = 0 as libc::c_int;
        }
        1 => {
            family = 2 as libc::c_int;
        }
        2 => {
            family = 30 as libc::c_int;
        }
        _ => return 0 as *mut addrinfo,
    }
    return os_addrinfo_intern(
        family,
        0 as libc::c_int,
        0 as libc::c_int,
        host,
        service,
        1 as libc::c_int != 0,
    );
}
#[no_mangle]
#[c2rust::src_loc = "844:1"]
pub unsafe extern "C" fn pony_os_getaddr(mut addr: *mut addrinfo, mut ipaddr: *mut ipaddress_t) {
    memcpy(
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut libc::c_void,
        (*addr).ai_addr as *const libc::c_void,
        (*addr).ai_addrlen as libc::c_ulong,
    );
    map_any_to_loopback(&mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr);
}
#[no_mangle]
#[c2rust::src_loc = "850:1"]
pub unsafe extern "C" fn pony_os_nextaddr(mut addr: *mut addrinfo) -> *mut addrinfo {
    return (*addr).ai_next;
}
#[no_mangle]
#[c2rust::src_loc = "855:1"]
pub unsafe extern "C" fn pony_os_ip_string(
    mut src: *mut libc::c_void,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut dst: [libc::c_char; 46] = [0; 46];
    let mut family: libc::c_int = address_family(len);
    if family == -(1 as libc::c_int) {
        return 0 as *mut libc::c_char;
    }
    if !(inet_ntop(
        family,
        src,
        dst.as_mut_ptr(),
        46 as libc::c_int as socklen_t,
    ))
    .is_null()
    {
        return 0 as *mut libc::c_char;
    }
    let mut dstlen: size_t = strlen(dst.as_mut_ptr());
    let mut result: *mut libc::c_char = pony_alloc(
        pony_ctx(),
        dstlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        result as *mut libc::c_void,
        dst.as_mut_ptr() as *const libc::c_void,
        dstlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    result
}
#[no_mangle]
#[c2rust::src_loc = "873:1"]
pub unsafe extern "C" fn pony_os_ipv4(mut ipaddr: *mut ipaddress_t) -> bool {
    return (*ipaddr).addr.ss_family as libc::c_int == 2 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "878:1"]
pub unsafe extern "C" fn pony_os_ipv6(mut ipaddr: *mut ipaddress_t) -> bool {
    return (*ipaddr).addr.ss_family as libc::c_int == 30 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "883:1"]
pub unsafe extern "C" fn pony_os_sockname(
    mut fd: libc::c_int,
    mut ipaddr: *mut ipaddress_t,
) -> bool {
    let mut len: socklen_t =
        ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    if getsockname(
        fd,
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr,
        &mut len,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    map_any_to_loopback(&mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "894:1"]
pub unsafe extern "C" fn pony_os_peername(
    mut fd: libc::c_int,
    mut ipaddr: *mut ipaddress_t,
) -> bool {
    let mut len: socklen_t =
        ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    if getpeername(
        fd,
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr,
        &mut len,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    map_any_to_loopback(&mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "905:1"]
pub unsafe extern "C" fn pony_os_host_ip4(mut host: *const libc::c_char) -> bool {
    let mut addr: in_addr = in_addr { s_addr: 0 };
    return inet_pton(
        2 as libc::c_int,
        host,
        &mut addr as *mut in_addr as *mut libc::c_void,
    ) == 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "911:1"]
pub unsafe extern "C" fn pony_os_host_ip6(mut host: *const libc::c_char) -> bool {
    let mut addr: in6_addr = in6_addr {
        __u6_addr: C2RustUnnamed_0 {
            __u6_addr8: [0; 16],
        },
    };
    return inet_pton(
        30 as libc::c_int,
        host,
        &mut addr as *mut in6_addr as *mut libc::c_void,
    ) == 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "942:1"]
pub unsafe extern "C" fn pony_os_writev(
    mut ev: *mut asio_event_t,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> size_t {
    let mut sent: ssize_t = writev((*ev).fd, iov, iovcnt);
    if sent < 0 as libc::c_int as libc::c_long {
        if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
        pony_error();
    }
    sent as size_t
}
#[no_mangle]
#[c2rust::src_loc = "958:1"]
pub unsafe extern "C" fn pony_os_send(
    mut ev: *mut asio_event_t,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut sent: ssize_t = send((*ev).fd, buf as *const libc::c_void, len, 0 as libc::c_int);
    if sent < 0 as libc::c_int as libc::c_long {
        if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
        pony_error();
    }
    sent as size_t
}
#[no_mangle]
#[c2rust::src_loc = "1000:1"]
pub unsafe extern "C" fn pony_os_recv(
    mut ev: *mut asio_event_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut received: ssize_t = recv((*ev).fd, buf as *mut libc::c_void, len, 0 as libc::c_int);
    if received < 0 as libc::c_int as libc::c_long {
        if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
        pony_error();
    } else if received == 0 as libc::c_int as libc::c_long {
        pony_error();
    }
    return received as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "1024:1"]
pub unsafe extern "C" fn pony_os_sendto(
    mut fd: libc::c_int,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut ipaddr: *mut ipaddress_t,
) -> size_t {
    let mut addrlen: socklen_t = ponyint_address_length(ipaddr);
    if addrlen == -(1 as libc::c_int) as socklen_t {
        pony_error();
    }
    let mut sent: ssize_t = sendto(
        fd,
        buf as *const libc::c_void,
        len,
        0 as libc::c_int,
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr,
        addrlen,
    );
    if sent < 0 as libc::c_int as libc::c_long {
        if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
        pony_error();
    }
    sent as size_t
}
#[no_mangle]
#[c2rust::src_loc = "1053:1"]
pub unsafe extern "C" fn pony_os_recvfrom(
    mut ev: *mut asio_event_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut ipaddr: *mut ipaddress_t,
) -> size_t {
    let mut addrlen: socklen_t =
        ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    let mut recvd: ssize_t = recvfrom(
        (*ev).fd,
        buf as *mut libc::c_void,
        len,
        0 as libc::c_int,
        &mut (*ipaddr).addr as *mut sockaddr_storage as *mut sockaddr,
        &mut addrlen,
    );
    if recvd < 0 as libc::c_int as libc::c_long {
        if *__error() == 35 as libc::c_int || *__error() == 35 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
        pony_error();
    } else if recvd == 0 as libc::c_int as libc::c_long {
        pony_error();
    }
    recvd as size_t
}
#[no_mangle]
#[c2rust::src_loc = "1081:1"]
pub unsafe extern "C" fn pony_os_keepalive(mut fd: libc::c_int, mut secs: libc::c_int) {
    let mut s: SOCKET = fd;
    let mut on: libc::c_int = if secs > 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    setsockopt(
        s,
        0xffff as libc::c_int,
        0x8 as libc::c_int,
        &mut on as *mut libc::c_int as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if on == 0 as libc::c_int {
        return;
    }
    let mut probes: libc::c_int = secs / 2 as libc::c_int;
    setsockopt(
        s,
        6 as libc::c_int,
        0x102 as libc::c_int,
        &mut probes as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    let mut idle: libc::c_int = secs / 2 as libc::c_int;
    setsockopt(
        s,
        6 as libc::c_int,
        0x10 as libc::c_int,
        &mut idle as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    let mut intvl: libc::c_int = 1 as libc::c_int;
    setsockopt(
        s,
        6 as libc::c_int,
        0x101 as libc::c_int,
        &mut intvl as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1119:1"]
pub unsafe extern "C" fn pony_os_nodelay(mut fd: libc::c_int, mut state: bool) {
    let mut val: libc::c_int = state as libc::c_int;
    setsockopt(
        fd,
        6 as libc::c_int,
        0x1 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1127:1"]
pub unsafe extern "C" fn pony_os_socket_shutdown(mut fd: libc::c_int) {
    shutdown(fd, 1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1132:1"]
pub unsafe extern "C" fn pony_os_socket_close(mut fd: libc::c_int) {
    close(fd);
}
#[no_mangle]
#[c2rust::src_loc = "1142:1"]
pub unsafe extern "C" fn ponyint_os_sockets_init() -> bool {
    let mut sa: sigaction = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };
    sa.__sigaction_u.__sa_handler = ::core::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(1 as libc::c_int as libc::intptr_t);
    sa.sa_mask = 0 as libc::c_int as sigset_t;
    sa.sa_flags = 0 as libc::c_int;
    if sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction) == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1211:1"]
pub unsafe extern "C" fn ponyint_os_sockets_final() {}
#[no_mangle]
#[c2rust::src_loc = "1219:1"]
pub unsafe extern "C" fn pony_os_broadcast(mut fd: libc::c_int, mut state: bool) {
    let mut broadcast: libc::c_int = if state as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    setsockopt(
        fd,
        0xffff as libc::c_int,
        0x20 as libc::c_int,
        &mut broadcast as *mut libc::c_int as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1227:1"]
pub unsafe extern "C" fn pony_os_multicast_interface(
    mut fd: libc::c_int,
    mut from: *const libc::c_char,
) {
    let mut p: *mut addrinfo = os_addrinfo_intern(
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        from,
        0 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
    if !p.is_null() {
        setsockopt(
            fd,
            0 as libc::c_int,
            9 as libc::c_int,
            &mut (*p).ai_addr as *mut *mut sockaddr as *const libc::c_char as *const libc::c_void,
            (*p).ai_addrlen as libc::c_int as socklen_t,
        );
        freeaddrinfo(p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1241:1"]
pub unsafe extern "C" fn pony_os_multicast_loopback(mut fd: libc::c_int, mut loopback: bool) {
    let mut loop_0: u8 = (if loopback as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as u8;
    setsockopt(
        fd,
        0 as libc::c_int,
        11 as libc::c_int,
        &mut loop_0 as *mut u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<u8>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1250:1"]
pub unsafe extern "C" fn pony_os_multicast_ttl(mut fd: libc::c_int, mut ttl: u8) {
    setsockopt(
        fd,
        0 as libc::c_int,
        10 as libc::c_int,
        &mut ttl as *mut u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<u8>() as libc::c_ulong as socklen_t,
    );
}
#[c2rust::src_loc = "1256:1"]
unsafe extern "C" fn multicast_interface(
    mut family: libc::c_int,
    mut host: *const libc::c_char,
) -> u32 {
    if host.is_null() || *host.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int as u32;
    }
    let mut p: *mut addrinfo = os_addrinfo_intern(
        family,
        0 as libc::c_int,
        0 as libc::c_int,
        host,
        0 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
    if p.is_null() {
        return 0 as libc::c_int as u32;
    }
    let mut interface: u32 = 0 as libc::c_int as u32;
    match (*p).ai_family {
        2 => {
            interface = (*((*p).ai_addr as *mut sockaddr_in)).sin_addr.s_addr;
        }
        30 => {
            interface = (*((*p).ai_addr as *mut sockaddr_in6)).sin6_scope_id;
        }
        _ => {}
    }
    freeaddrinfo(p);
    return interface;
}
#[c2rust::src_loc = "1293:1"]
unsafe extern "C" fn multicast_change(
    mut fd: libc::c_int,
    mut group: *const libc::c_char,
    mut to: *const libc::c_char,
    mut join: bool,
) {
    let mut rg: *mut addrinfo = os_addrinfo_intern(
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        group,
        0 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
    if rg.is_null() {
        return;
    }
    let mut interface: u32 = multicast_interface((*rg).ai_family, to);
    let mut s: SOCKET = fd;
    match (*rg).ai_family {
        2 => {
            let mut req: ip_mreq = ip_mreq {
                imr_multiaddr: in_addr { s_addr: 0 },
                imr_interface: in_addr { s_addr: 0 },
            };
            req.imr_multiaddr = (*((*rg).ai_addr as *mut sockaddr_in)).sin_addr;
            req.imr_interface.s_addr = interface;
            if join {
                setsockopt(
                    s,
                    0 as libc::c_int,
                    12 as libc::c_int,
                    &mut req as *mut ip_mreq as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<ip_mreq>() as libc::c_ulong as socklen_t,
                );
            } else {
                setsockopt(
                    s,
                    0 as libc::c_int,
                    13 as libc::c_int,
                    &mut req as *mut ip_mreq as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<ip_mreq>() as libc::c_ulong as socklen_t,
                );
            }
        }
        30 => {
            let mut req_0: ipv6_mreq = ipv6_mreq {
                ipv6mr_multiaddr: in6_addr {
                    __u6_addr: C2RustUnnamed_0 {
                        __u6_addr8: [0; 16],
                    },
                },
                ipv6mr_interface: 0,
            };
            memcpy(
                &mut req_0.ipv6mr_multiaddr as *mut in6_addr as *mut libc::c_void,
                &mut (*((*rg).ai_addr as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                    as *const libc::c_void,
                ::core::mem::size_of::<in6_addr>() as libc::c_ulong,
            );
            req_0.ipv6mr_interface = interface;
            if join {
                setsockopt(
                    s,
                    41 as libc::c_int,
                    12 as libc::c_int,
                    &mut req_0 as *mut ipv6_mreq as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<ipv6_mreq>() as libc::c_ulong as socklen_t,
                );
            } else {
                setsockopt(
                    s,
                    41 as libc::c_int,
                    13 as libc::c_int,
                    &mut req_0 as *mut ipv6_mreq as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<ipv6_mreq>() as libc::c_ulong as socklen_t,
                );
            }
        }
        _ => {}
    }
    freeaddrinfo(rg);
}
#[no_mangle]
#[c2rust::src_loc = "1346:1"]
pub unsafe extern "C" fn pony_os_multicast_join(
    mut fd: libc::c_int,
    mut group: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    multicast_change(fd, group, to, 1 as libc::c_int != 0);
}
#[no_mangle]
#[c2rust::src_loc = "1351:1"]
pub unsafe extern "C" fn pony_os_multicast_leave(
    mut fd: libc::c_int,
    mut group: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    multicast_change(fd, group, to, 0 as libc::c_int != 0);
}
#[no_mangle]
#[c2rust::src_loc = "1375:1"]
pub unsafe extern "C" fn pony_os_sockopt_level(mut option: libc::c_int) -> libc::c_int {
    match option {
        4000 => return 34 as libc::c_int,
        4001 => return 68 as libc::c_int,
        4002 => return 51 as libc::c_int,
        4003 => return 61 as libc::c_int,
        4004 => return 99 as libc::c_int,
        4005 => return 13 as libc::c_int,
        4006 => return 93 as libc::c_int,
        4008 => return 49 as libc::c_int,
        4009 => return 30 as libc::c_int,
        4010 => return 76 as libc::c_int,
        4012 => return 62 as libc::c_int,
        4013 => return 16 as libc::c_int,
        4014 => return 38 as libc::c_int,
        4016 => return 73 as libc::c_int,
        4017 => return 72 as libc::c_int,
        4019 => return 37 as libc::c_int,
        4020 => return 86 as libc::c_int,
        4021 => return 254 as libc::c_int,
        4022 => return 257 as libc::c_int,
        4023 => return 60 as libc::c_int,
        4024 => return 8 as libc::c_int,
        4025 => return 14 as libc::c_int,
        4026 => return 98 as libc::c_int,
        4027 => return 80 as libc::c_int,
        4028 => return 50 as libc::c_int,
        4029 => return 97 as libc::c_int,
        4030 => return 44 as libc::c_int,
        4031 => return 3 as libc::c_int,
        4032 => return 100 as libc::c_int,
        4033 => return 47 as libc::c_int,
        4034 => return 63 as libc::c_int,
        4036 => return 20 as libc::c_int,
        4037 => return 0 as libc::c_int,
        4038 => return 1 as libc::c_int,
        4039 => return 58 as libc::c_int,
        4040 => return 22 as libc::c_int,
        4041 => return 35 as libc::c_int,
        4042 => return 45 as libc::c_int,
        4043 => return 2 as libc::c_int,
        4044 => return 85 as libc::c_int,
        4045 => return 88 as libc::c_int,
        4046 => return 40 as libc::c_int,
        4047 => return 52 as libc::c_int,
        4048 => return 32 as libc::c_int,
        4049 => return 0 as libc::c_int,
        4050 => return 108 as libc::c_int,
        4051 => return 71 as libc::c_int,
        4052 => return 94 as libc::c_int,
        4053 => return 4 as libc::c_int,
        4054 => return 67 as libc::c_int,
        4055 => return 4 as libc::c_int,
        4056 => return 41 as libc::c_int,
        4057 => return 28 as libc::c_int,
        4058 => return 65 as libc::c_int,
        4059 => return 91 as libc::c_int,
        4060 => return 25 as libc::c_int,
        4061 => return 26 as libc::c_int,
        4062 => return 256 as libc::c_int,
        4063 => return 51 as libc::c_int + 1 as libc::c_int,
        4064 => return 19 as libc::c_int,
        4066 => return 48 as libc::c_int,
        4067 => return 95 as libc::c_int,
        4070 => return 92 as libc::c_int,
        4071 => return 18 as libc::c_int,
        4072 => return 77 as libc::c_int,
        4073 => return 54 as libc::c_int,
        4074 => return 59 as libc::c_int,
        4075 => return 31 as libc::c_int,
        4076 => return 11 as libc::c_int,
        4078 => return 89 as libc::c_int,
        4080 => return 113 as libc::c_int,
        4081 => return 9 as libc::c_int,
        4082 => return 103 as libc::c_int,
        4083 => return 21 as libc::c_int,
        4084 => return 12 as libc::c_int,
        4085 => return 75 as libc::c_int,
        4086 => return 255 as libc::c_int,
        4087 => return 10 as libc::c_int,
        4088 => return 27 as libc::c_int,
        4091 => return 43 as libc::c_int,
        4092 => return 46 as libc::c_int,
        4093 => return 66 as libc::c_int,
        4094 => return 64 as libc::c_int,
        4095 => return 69 as libc::c_int,
        4096 => return 96 as libc::c_int,
        4097 => return 132 as libc::c_int,
        4098 => return 42 as libc::c_int,
        4100 => return 33 as libc::c_int,
        4104 => return 90 as libc::c_int,
        4105 => return 7 as libc::c_int,
        4106 => return 82 as libc::c_int,
        4107 => return 53 as libc::c_int,
        4108 => return 87 as libc::c_int,
        4109 => return 6 as libc::c_int,
        4111 => return 29 as libc::c_int,
        4112 => return 39 as libc::c_int,
        4113 => return 23 as libc::c_int,
        4114 => return 24 as libc::c_int,
        4115 => return 84 as libc::c_int,
        4116 => return 17 as libc::c_int,
        4118 => return 83 as libc::c_int,
        4119 => return 70 as libc::c_int,
        4120 => return 81 as libc::c_int,
        4121 => return 79 as libc::c_int,
        4122 => return 78 as libc::c_int,
        4123 => return 74 as libc::c_int,
        4124 => return 15 as libc::c_int,
        4125 => return 36 as libc::c_int,
        4131 => return 0 as libc::c_int,
        4132 => return 0 as libc::c_int,
        4138 => return 0xffff as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
#[c2rust::src_loc = "1820:1"]
pub unsafe extern "C" fn pony_os_sockopt_option(mut option: libc::c_int) -> libc::c_int {
    match option {
        0 => return 20 as libc::c_int,
        1 => return 2 as libc::c_int,
        2 => return 30 as libc::c_int,
        74 => return 13 as libc::c_int,
        76 => return 3 as libc::c_int,
        77 => return 9 as libc::c_int,
        78 => return 14 as libc::c_int,
        79 => return 1 as libc::c_int,
        80 => return 16 as libc::c_int,
        83 => return 11 as libc::c_int,
        84 => return 10 as libc::c_int,
        85 => return 15 as libc::c_int,
        86 => return 17 as libc::c_int,
        87 => return 5 as libc::c_int,
        88 => return 7 as libc::c_int,
        89 => return 6 as libc::c_int,
        90 => return 2 as libc::c_int,
        91 => return 8 as libc::c_int,
        92 => return 12 as libc::c_int,
        95 => return 49152 as libc::c_int,
        96 => return 65535 as libc::c_int,
        98 => return 1024 as libc::c_int,
        99 => return 600 as libc::c_int,
        100 => return 5000 as libc::c_int,
        101 => return 23 as libc::c_int,
        102 => return 20 as libc::c_int,
        103 => return 22 as libc::c_int,
        104 => return 19 as libc::c_int,
        105 => return 25 as libc::c_int,
        106 => return 24 as libc::c_int,
        112 => return 26 as libc::c_int,
        135 => return 28 as libc::c_int,
        141 => return 10 as libc::c_int,
        142 => return 9 as libc::c_int,
        143 => return 11 as libc::c_int,
        185 => return 35 as libc::c_int,
        189 => return 36 as libc::c_int,
        196 => return 4 as libc::c_int,
        199 => return 27 as libc::c_int,
        236 => return 12 as libc::c_int,
        237 => return 70 as libc::c_int,
        241 => return 72 as libc::c_int,
        242 => return 25 as libc::c_int,
        244 => return 1 as libc::c_int,
        245 => return 1 as libc::c_int,
        246 => return 28 as libc::c_int,
        247 => return 13 as libc::c_int,
        248 => return 71 as libc::c_int,
        250 => return 60 as libc::c_int,
        251 => return 61 as libc::c_int,
        252 => return 62 as libc::c_int,
        253 => return 64 as libc::c_int,
        254 => return 22 as libc::c_int,
        259 => return 40 as libc::c_int,
        260 => return 41 as libc::c_int,
        261 => return 42 as libc::c_int,
        262 => return 44 as libc::c_int,
        267 => return 45 as libc::c_int,
        273 => return 43 as libc::c_int,
        274 => return 2 as libc::c_int,
        275 => return 21 as libc::c_int,
        276 => return 512 as libc::c_int,
        277 => return 4095 as libc::c_int,
        278 => return 128 as libc::c_int,
        279 => return 128 as libc::c_int,
        282 => return 31 as libc::c_int,
        283 => return 74 as libc::c_int,
        287 => return 9 as libc::c_int,
        288 => return 66 as libc::c_int,
        289 => return 11 as libc::c_int,
        290 => return 10 as libc::c_int,
        291 => return 14 as libc::c_int,
        294 => return 50 as libc::c_int,
        295 => return 51 as libc::c_int,
        296 => return 52 as libc::c_int,
        297 => return 54 as libc::c_int,
        298 => return 56 as libc::c_int,
        299 => return 53 as libc::c_int,
        301 => return 1 as libc::c_int,
        304 => return 26 as libc::c_int,
        312 => return 19 as libc::c_int,
        313 => return 0 as libc::c_int,
        314 => return 1 as libc::c_int,
        315 => return 2 as libc::c_int,
        316 => return 7 as libc::c_int,
        319 => return 20 as libc::c_int,
        320 => return 5 as libc::c_int,
        322 => return 26 as libc::c_int,
        323 => return 6 as libc::c_int,
        325 => return 27 as libc::c_int,
        326 => return 24 as libc::c_int,
        327 => return 8 as libc::c_int,
        331 => return 16 as libc::c_int,
        332 => return 15 as libc::c_int,
        333 => return 18 as libc::c_int,
        334 => return 17 as libc::c_int,
        336 => return 23 as libc::c_int,
        337 => return 3 as libc::c_int,
        338 => return 65 as libc::c_int,
        340 => return 4 as libc::c_int,
        341 => return 73 as libc::c_int,
        346 => return 0x1 as libc::c_int,
        347 => return 0x3 as libc::c_int,
        348 => return 0x5 as libc::c_int,
        349 => return 0x2 as libc::c_int,
        350 => return 0x4 as libc::c_int,
        352 => return 40 as libc::c_int,
        353 => return 84 as libc::c_int,
        354 => return 2 as libc::c_int,
        355 => return 1 as libc::c_int,
        356 => return 80 as libc::c_int,
        357 => return 82 as libc::c_int,
        358 => return 81 as libc::c_int,
        359 => return 83 as libc::c_int,
        361 => return 85 as libc::c_int,
        362 => return 0 as libc::c_int,
        384 => return 0 as libc::c_int,
        385 => return 0x5 as libc::c_int,
        386 => return 0x2 as libc::c_int,
        387 => return 0x6 as libc::c_int,
        388 => return 4 as libc::c_int,
        389 => return 5 as libc::c_int,
        390 => return 6 as libc::c_int,
        391 => return 1024 as libc::c_int,
        392 => return 1 as libc::c_int,
        393 => return 0x4 as libc::c_int,
        435 => return 1 as libc::c_int,
        509 => return 0x2 as libc::c_int,
        793 => return 2 as libc::c_int,
        794 => return 255 as libc::c_int,
        796 => return 3 as libc::c_int,
        797 => return 4 as libc::c_int,
        798 => return 5 as libc::c_int,
        799 => return 1 as libc::c_int,
        800 => return 128 as libc::c_int,
        801 => return 0x1 as libc::c_int,
        802 => return 0x2 as libc::c_int,
        812 => return 0x20 as libc::c_int,
        820 => return 0x1 as libc::c_int,
        825 => return 0x10 as libc::c_int,
        826 => return 0x2000 as libc::c_int,
        827 => return 0x1007 as libc::c_int,
        835 => return 0x8 as libc::c_int,
        842 => return 0x1010 as libc::c_int,
        843 => return 0x80 as libc::c_int,
        844 => return 0x1080 as libc::c_int,
        854 => return 0x1119 as libc::c_int,
        855 => return 0x1116 as libc::c_int,
        856 => return 0x1021 as libc::c_int,
        857 => return 0x1023 as libc::c_int,
        859 => return 0x1022 as libc::c_int,
        860 => return 0x1026 as libc::c_int,
        864 => return 0x1083 as libc::c_int,
        865 => return 0x1020 as libc::c_int,
        866 => return 0x1112 as libc::c_int,
        867 => return 0x1024 as libc::c_int,
        868 => return 0x100 as libc::c_int,
        874 => return 0x1011 as libc::c_int,
        883 => return 0x1082 as libc::c_int,
        884 => return 0x1002 as libc::c_int,
        886 => return 0x1004 as libc::c_int,
        887 => return 0x1006 as libc::c_int,
        889 => return 0x4 as libc::c_int,
        890 => return 0x200 as libc::c_int,
        891 => return 0x1025 as libc::c_int,
        903 => return 0x1001 as libc::c_int,
        905 => return 0x1003 as libc::c_int,
        906 => return 0x1005 as libc::c_int,
        907 => return 0x400 as libc::c_int,
        910 => return 0x800 as libc::c_int,
        911 => return 0x1008 as libc::c_int,
        912 => return 0x1027 as libc::c_int,
        914 => return 0x40 as libc::c_int,
        924 => return 0x4000 as libc::c_int,
        925 => return 0x8000 as libc::c_int,
        927 => return 1024 as libc::c_int,
        928 => return 0x1 as libc::c_int,
        929 => return 0x2 as libc::c_int,
        930 => return 0x8 as libc::c_int,
        931 => return 0x2 as libc::c_int,
        932 => return 0x1 as libc::c_int,
        933 => return 0x4 as libc::c_int,
        946 => return 6 as libc::c_int,
        947 => return 6 as libc::c_int + 2 as libc::c_int,
        949 => return 2 as libc::c_int,
        953 => return 4 as libc::c_int,
        956 => return 8 as libc::c_int,
        958 => return 2 as libc::c_int,
        959 => return 18 as libc::c_int,
        960 => return 10 as libc::c_int,
        961 => return 10 as libc::c_int + 2 as libc::c_int,
        962 => return 3 as libc::c_int,
        963 => return 11 as libc::c_int,
        964 => return 13 as libc::c_int,
        965 => return 12 as libc::c_int,
        966 => return 0 as libc::c_int,
        967 => return 34 as libc::c_int,
        969 => return 2 as libc::c_int,
        971 => return 1 as libc::c_int,
        973 => return 5 as libc::c_int,
        974 => {
            return (1 as libc::c_int) << 24 as libc::c_int
                | (1 as libc::c_int) << 16 as libc::c_int
                | (5 as libc::c_int) << 8 as libc::c_int;
        }
        975 => return 4 as libc::c_int,
        976 => {
            return (1 as libc::c_int) << 24 as libc::c_int
                | (1 as libc::c_int) << 16 as libc::c_int
                | (4 as libc::c_int) << 8 as libc::c_int
                | 2 as libc::c_int;
        }
        977 => return 19 as libc::c_int,
        978 => return 8 as libc::c_int,
        979 => {
            return (1 as libc::c_int) << 24 as libc::c_int
                | (1 as libc::c_int) << 16 as libc::c_int
                | (8 as libc::c_int) << 8 as libc::c_int
                | 10 as libc::c_int;
        }
        980 => return 3 as libc::c_int,
        985 => return 0x20 as libc::c_int,
        986 => return 0x106 as libc::c_int,
        995 => return 0x104 as libc::c_int,
        996 => return 0x105 as libc::c_int,
        1000 => return 0x10 as libc::c_int,
        1001 => return 0x102 as libc::c_int,
        1004 => return 0x101 as libc::c_int,
        1007 => return (0xf as libc::c_int) << 2 as libc::c_int,
        1008 => {
            return (((0xf as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<tcphdr>() as libc::c_ulong)
                as libc::c_int;
        }
        1009 => return 0x2 as libc::c_int,
        1010 => return 65535 as libc::c_int,
        1011 => return 4 as libc::c_int,
        1012 => return 14 as libc::c_int,
        1015 => return 216 as libc::c_int,
        1016 => return 512 as libc::c_int,
        1019 => return 0x1 as libc::c_int,
        1020 => return 0x8 as libc::c_int,
        1021 => return 0x4 as libc::c_int,
        1022 => return 0x201 as libc::c_int,
        1030 => return 0x80 as libc::c_int,
        1031 => return 0x100 as libc::c_int,
        1034 => return 0x103 as libc::c_int,
        1086 => return 4096 as libc::c_int,
        1094 => return 0x1 as libc::c_int,
        1098 => return 20 as libc::c_int,
        1100 => return 21 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
