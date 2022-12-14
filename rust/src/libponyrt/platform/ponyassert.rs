use ::libc;
use core::sync::atomic::{AtomicBool, Ordering::AcqRel};
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:3"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:3"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:3"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:4"]
pub mod _stdio_h {
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
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
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:3"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "131:7"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:3"]
pub mod time_h {
    use super::_timespec_h::timespec;
    extern "C" {
        #[c2rust::src_loc = "142:1"]
        pub fn nanosleep(__rqtp: *const timespec, __rmtp: *mut timespec) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:4"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "146:1"]
        pub fn fflush(_: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "157:1"]
        pub fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    }
}
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_timespec_h::timespec;
pub use self::_types_h::{__darwin_time_t, __int64_t};
pub use self::stddef_h::size_t;
use self::stdio_h::{__stderrp, fflush, fprintf, fputs};
use self::stdlib_h::abort;
pub use self::sys__types_h::__darwin_off_t;
use self::time_h::nanosleep;
#[c2rust::src_loc = "19:31"]
static mut assert_guard: AtomicBool = AtomicBool::new(false);
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn ponyint_assert_fail(
    mut expr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: size_t,
    mut func: *const libc::c_char,
) {
    while assert_guard.swap(true, AcqRel) {
        let mut ts: timespec = {
            let mut init = timespec {
                tv_sec: 1 as libc::c_int as __darwin_time_t,
                tv_nsec: 0 as libc::c_int as libc::c_long,
            };
            init
        };
        nanosleep(&mut ts, 0 as *mut timespec);
    }
    fprintf(
        __stderrp,
        b"%s:%zu: %s: Assertion `%s` failed.\n\n\0" as *const u8 as *const libc::c_char,
        file,
        line,
        func,
        expr,
    );
    fputs(
        b"Backtrace functionality not available.\n\0" as *const u8 as *const libc::c_char,
        __stderrp,
    );
    fflush(__stderrp);
    abort();
}
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub unsafe extern "C" fn ponyint_assert_disable_popups() {}
