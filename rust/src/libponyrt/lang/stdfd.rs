use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "46:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "55:1"]
    pub type __darwin_blkcnt_t = i64;
    #[c2rust::src_loc = "56:1"]
    pub type __darwin_blksize_t = i32;
    #[c2rust::src_loc = "57:1"]
    pub type __darwin_dev_t = i32;
    #[c2rust::src_loc = "60:1"]
    pub type __darwin_gid_t = u32;
    #[c2rust::src_loc = "62:1"]
    pub type __darwin_ino64_t = u64;
    #[c2rust::src_loc = "70:1"]
    pub type __darwin_mode_t = u16;
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
    #[c2rust::src_loc = "75:1"]
    pub type __darwin_uid_t = u32;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:1"]
pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: libc::c_long,
    }
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_dev_t.h:1"]
pub mod _dev_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:1"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h:1"]
pub mod _off_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blkcnt_t.h:1"]
pub mod _blkcnt_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blksize_t.h:1"]
pub mod _blksize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:1"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_nlink_t.h:1"]
pub mod _nlink_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type nlink_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:1"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: u32,
        pub st_gen: u32,
        pub st_lspare: i32,
        pub st_qspare: [i64; 2],
    }
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;

    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {
        #[c2rust::src_loc = "382:1"]
        pub fn fstat(_: libc::c_int, _: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:2"]
pub mod _stdio_h {
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:16"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/poll.h:7"]
pub mod poll_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:8"]
    pub struct pollfd {
        pub fd: libc::c_int,
        pub events: libc::c_short,
        pub revents: libc::c_short,
    }
    #[c2rust::src_loc = "102:1"]
    pub type nfds_t = libc::c_uint;
    extern "C" {
        #[c2rust::src_loc = "113:1"]
        pub fn poll(_: *mut pollfd, _: nfds_t, _: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/termios.h:8"]
pub mod termios_h {
    #[c2rust::src_loc = "263:1"]
    pub type tcflag_t = libc::c_ulong;
    #[c2rust::src_loc = "264:1"]
    pub type cc_t = libc::c_uchar;
    #[c2rust::src_loc = "265:1"]
    pub type speed_t = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "267:8"]
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [cc_t; 20],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
    }
    extern "C" {
        #[c2rust::src_loc = "335:1"]
        pub fn tcgetattr(_: libc::c_int, _: *mut termios) -> libc::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn tcsetattr(_: libc::c_int, _: libc::c_int, _: *const termios) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "133:1"]
        pub fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:2"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "68:14"]
        pub static mut __stdoutp: *mut FILE;
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "146:1"]
        pub fn fflush(_: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "165:9"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[c2rust::src_loc = "184:1"]
        pub fn setvbuf(_: *mut FILE, _: *mut libc::c_char, _: libc::c_int, _: usize)
            -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:9"]
pub mod unistd_h {
    use super::_ssize_t_h::ssize_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "463:1"]
        pub fn isatty(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "472:1"]
        pub fn read(_: libc::c_int, _: *mut libc::c_void, _: usize) -> ssize_t;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_off_t_h::off_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_ssize_t, __darwin_time_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::poll_h::{nfds_t, poll, pollfd};
pub use self::stat_h::{fstat, stat};
pub use self::stddef_h::size_t;
use self::stdio_h::{__stderrp, __stdoutp, fflush, fprintf, fwrite, setvbuf};
use self::stdlib_h::atexit;
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t,
};
pub use self::termios_h::{cc_t, speed_t, tcflag_t, tcgetattr, tcsetattr, termios};
use self::unistd_h::{isatty, read};
#[c2rust::src_loc = "270:9"]
pub type fd_type_t = libc::c_uint;
#[c2rust::src_loc = "276:3"]
pub const FD_TYPE_FILE: fd_type_t = 4;
#[c2rust::src_loc = "275:3"]
pub const FD_TYPE_PIPE: fd_type_t = 3;
#[c2rust::src_loc = "274:3"]
pub const FD_TYPE_TTY: fd_type_t = 2;
#[c2rust::src_loc = "273:3"]
pub const FD_TYPE_DEVICE: fd_type_t = 1;
#[c2rust::src_loc = "272:3"]
pub const FD_TYPE_NONE: fd_type_t = 0;
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn pony_os_stdout() -> *mut FILE {
    __stdoutp
}
#[no_mangle]
#[c2rust::src_loc = "22:1"]
pub unsafe extern "C" fn pony_os_stderr() -> *mut FILE {
    __stderrp
}
#[c2rust::src_loc = "27:13"]
static mut is_stdout_tty: bool = 0 as libc::c_int != 0;
#[c2rust::src_loc = "28:13"]
static mut is_stderr_tty: bool = 0 as libc::c_int != 0;
#[c2rust::src_loc = "268:23"]
static mut orig_termios: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_cc: [0; 20],
    c_ispeed: 0,
    c_ospeed: 0,
};
#[c2rust::src_loc = "279:1"]
unsafe extern "C" fn fd_type(mut fd: libc::c_int) -> fd_type_t {
    let mut type_0: fd_type_t = FD_TYPE_NONE;
    let mut st: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    if fstat(fd, &mut st) != -(1 as libc::c_int) {
        match st.st_mode as libc::c_int & 0o170000 as libc::c_int {
            4096 | 49152 => {
                type_0 = FD_TYPE_PIPE;
            }
            8192 => {
                if libc::isatty(fd) != 0 {
                    type_0 = FD_TYPE_TTY;
                } else {
                    type_0 = FD_TYPE_DEVICE;
                }
            }
            32768 => {
                type_0 = FD_TYPE_FILE;
            }
            _ => {}
        }
    }
    type_0
}
#[c2rust::src_loc = "316:1"]
unsafe extern "C" fn stdin_tty_restore() {
    tcsetattr(0 as libc::c_int, 2 as libc::c_int, &mut orig_termios);
}
#[c2rust::src_loc = "321:1"]
unsafe extern "C" fn stdin_tty() {
    if tcgetattr(0 as libc::c_int, &mut orig_termios) == -(1 as libc::c_int) {
        return;
    }
    let mut io: termios = orig_termios;
    io.c_iflag &=
        !(0x2 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int | 0x200 as libc::c_int)
            as libc::c_ulong;
    io.c_cflag |= 0x300 as libc::c_int as libc::c_ulong;
    io.c_lflag &=
        !(0x8 as libc::c_int | 0x100 as libc::c_int | 0x400 as libc::c_int) as libc::c_ulong;
    io.c_cc[16 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    io.c_cc[17 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    tcsetattr(0 as libc::c_int, 2 as libc::c_int, &mut io);
    atexit(::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        Option<unsafe extern "C" fn() -> ()>,
    >(Some(::core::mem::transmute::<
        unsafe extern "C" fn() -> (),
        unsafe extern "C" fn() -> (),
    >(stdin_tty_restore))));
}
#[no_mangle]
#[c2rust::src_loc = "340:1"]
pub unsafe extern "C" fn ansi_parse(
    mut buffer: *const libc::c_char,
    mut pos: *mut usize,
    mut len: usize,
    mut argc: *mut libc::c_int,
    mut argv: *mut libc::c_int,
) -> libc::c_char {
    let mut n: usize = 0;
    let mut arg: libc::c_int = -(1 as libc::c_int);
    let mut code: libc::c_char = -(1 as libc::c_int) as libc::c_char;
    n = *pos;
    while n < len {
        let mut c: libc::c_char = *buffer.offset(n as isize);
        if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
            if arg < 0 as libc::c_int {
                arg = 0 as libc::c_int;
            }
            *argv.offset(arg as isize) =
                *argv.offset(arg as isize) * 10 as libc::c_int + (c as libc::c_int - '0' as i32);
        } else if c as libc::c_int == ';' as i32 {
            arg = arg + 1 as libc::c_int;
            if arg > 5 as libc::c_int {
                break;
            }
        } else {
            code = c;
            break;
        }
        n = n.wrapping_add(1);
    }
    *pos = n.wrapping_add(1);
    *argc = arg + 1 as libc::c_int;
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "374:1"]
pub unsafe extern "C" fn pony_os_stdout_setup() {
    is_stdout_tty =
        fd_type(1 as libc::c_int) as libc::c_uint == FD_TYPE_TTY as libc::c_int as libc::c_uint;
    is_stderr_tty =
        fd_type(2 as libc::c_int) as libc::c_uint == FD_TYPE_TTY as libc::c_int as libc::c_uint;
    if is_stdout_tty {
        setvbuf(
            __stdoutp,
            0 as *mut libc::c_char,
            2 as libc::c_int,
            0 as libc::c_int as usize,
        );
    } else {
        setvbuf(
            __stdoutp,
            0 as *mut libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int as usize,
        );
    }
    if is_stderr_tty {
        setvbuf(
            __stderrp,
            0 as *mut libc::c_char,
            2 as libc::c_int,
            0 as libc::c_int as usize,
        );
    } else {
        setvbuf(
            __stderrp,
            0 as *mut libc::c_char,
            1 as libc::c_int,
            0 as libc::c_int as usize,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "421:1"]
pub unsafe extern "C" fn pony_os_stdin_setup() -> bool {
    let mut stdin_event_based: bool = 1 as libc::c_int != 0;
    let mut stdin_type: fd_type_t = fd_type(0 as libc::c_int);
    if stdin_type as libc::c_uint == FD_TYPE_TTY as libc::c_int as libc::c_uint
        && is_stdout_tty as libc::c_int != 0
    {
        stdin_tty();
    }
    if stdin_type as libc::c_uint == FD_TYPE_FILE as libc::c_int as libc::c_uint {
        stdin_event_based = 0 as libc::c_int != 0;
    }
    stdin_event_based
}
#[no_mangle]
#[c2rust::src_loc = "450:1"]
pub unsafe extern "C" fn pony_os_stdin_read(
    mut buffer: *mut libc::c_char,
    mut space: usize,
) -> usize {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    pfd.fd = 0 as libc::c_int;
    pfd.events = 0x1 as libc::c_int as libc::c_short;
    pfd.revents = 0 as libc::c_int as libc::c_short;
    let mut n: libc::c_int = poll(&mut pfd, 1 as libc::c_int as nfds_t, 0 as libc::c_int);
    if n != 1 as libc::c_int {
        return -(1 as libc::c_int) as usize;
    }
    return read(0 as libc::c_int, buffer as *mut libc::c_void, space) as usize;
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn pony_os_std_print(
    mut fp: *mut FILE,
    mut buffer: *mut libc::c_char,
    mut len: usize,
) {
    fwrite(
        buffer as *const libc::c_void,
        len,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "522:1"]
pub unsafe extern "C" fn pony_os_std_write(
    mut fp: *mut FILE,
    mut buffer: *mut libc::c_char,
    mut len: usize,
) {
    if len == 0 {
        return;
    }
    fwrite(
        buffer as *const libc::c_void,
        len,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
}
#[no_mangle]
#[c2rust::src_loc = "751:1"]
pub unsafe extern "C" fn pony_os_std_flush(mut fp: *mut FILE) {
    if fp.is_null() {
        return;
    }
    fflush(fp);
}
