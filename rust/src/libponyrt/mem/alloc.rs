use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:4"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:4"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:4"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h:4"]
pub mod _off_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:4"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "131:7"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:6"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "174:1"]
        pub fn perror(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/mman.h:9"]
pub mod mman_h {
    use super::_off_t_h::off_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "238:1"]
        pub fn mmap(
            _: *mut libc::c_void,
            _: size_t,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: off_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "247:1"]
        pub fn munmap(_: *mut libc::c_void, _: size_t) -> libc::c_int;
    }
}
pub use self::_off_t_h::off_t;
use self::mman_h::{mmap, munmap};
pub use self::stddef_h::size_t;
use self::stdio_h::perror;
use self::stdlib_h::abort;
pub use self::sys__types_h::__darwin_off_t;
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn ponyint_virt_alloc(mut bytes: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ok: bool = 1 as libc::c_int != 0;
    p = mmap(
        0 as *mut libc::c_void,
        bytes,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x2 as libc::c_int | 0x1000 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as off_t,
    );
    if p == -(1 as libc::c_int) as *mut libc::c_void {
        ok = 0 as libc::c_int != 0;
    }
    if !ok {
        perror(b"out of memory: \0" as *const u8 as *const libc::c_char);
        abort();
    }
    p
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn ponyint_virt_free(mut p: *mut libc::c_void, mut bytes: size_t) {
    munmap(p, bytes);
}
