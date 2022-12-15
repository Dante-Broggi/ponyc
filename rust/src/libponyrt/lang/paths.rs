use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:2"]
pub mod pony_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
        #[c2rust::src_loc = "262:1"]
        pub fn pony_alloc(ctx: *mut pony_ctx_t, size: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "227:1"]
        pub fn realpath(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:3"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
use self::pony_h::{pony_alloc, pony_ctx};
pub use self::stddef_h::size_t;
use self::stdlib_h::realpath;
use self::string_h::{memcpy, strlen};
#[no_mangle]
#[c2rust::src_loc = "7:1"]
pub unsafe extern "C" fn pony_os_realpath(mut path: *const libc::c_char) -> *mut libc::c_char {
    let mut resolved: [libc::c_char; 1024] = [0; 1024];
    if (realpath(path, resolved.as_mut_ptr())).is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut len: usize = (libc::strlen(resolved.as_mut_ptr())).wrapping_add(1);
    let mut cstring: *mut libc::c_char = pony_alloc(pony_ctx(), len) as *mut libc::c_char;
    memcpy(
        cstring as *mut libc::c_void,
        resolved.as_mut_ptr() as *const libc::c_void,
        len.try_into().unwrap(),
    );
    cstring
}
