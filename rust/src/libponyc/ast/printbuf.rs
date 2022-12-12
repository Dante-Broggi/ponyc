use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/usr/local/Cellar/llvm@13/13.0.1_2/lib/clang/13.0.1/include/stdarg.h:1"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/printbuf.h:1"]
pub mod printbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct printbuf_t {
        pub m: *mut libc::c_char,
        pub size: size_t,
        pub offset: size_t,
    }
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_pool_realloc_size(
            old_size: size_t,
            new_size: size_t,
            p: *mut libc::c_void,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:3"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:5"]
pub mod stdio_h {

    extern "C" {
        #[c2rust::src_loc = "347:6"]
        pub fn vsnprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ::core::ffi::VaList,
        ) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::internal::{__builtin_va_list, __va_list_tag};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
    ponyint_pool_realloc_size,
};
pub use self::printbuf_h::printbuf_t;
pub use self::stdarg_h::va_list;
use self::stdio_h::vsnprintf;
#[no_mangle]
#[c2rust::src_loc = "7:1"]
pub unsafe extern "C" fn printbuf_new() -> *mut printbuf_t {
    let mut buf: *mut printbuf_t =
        ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut printbuf_t;
    let ref mut fresh0 = (*buf).m;
    *fresh0 = ponyint_pool_alloc_size(32 as libc::c_int as size_t) as *mut libc::c_char;
    *((*buf).m).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*buf).size = 32 as libc::c_int as size_t;
    (*buf).offset = 0 as libc::c_int as size_t;
    return buf;
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn printbuf_free(mut buf: *mut printbuf_t) {
    ponyint_pool_free_size((*buf).size, (*buf).m as *mut libc::c_void);
    ponyint_pool_free(0 as libc::c_int as size_t, buf as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "23:1"]
pub unsafe extern "C" fn printbuf(
    mut buf: *mut printbuf_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut avail: size_t = ((*buf).size).wrapping_sub((*buf).offset);
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut r: libc::c_int = vsnprintf(
        ((*buf).m).offset((*buf).offset as isize),
        avail,
        fmt,
        ap.as_va_list(),
    );
    if r < 0 as libc::c_int {
        return;
    }
    if r as size_t >= avail {
        let mut new_size: size_t = ((*buf).size)
            .wrapping_add(r as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let ref mut fresh1 = (*buf).m;
        *fresh1 = ponyint_pool_realloc_size((*buf).size, new_size, (*buf).m as *mut libc::c_void)
            as *mut libc::c_char;
        (*buf).size = new_size;
        avail = ((*buf).size).wrapping_sub((*buf).offset);
        ap = args.clone();
        r = vsnprintf(
            ((*buf).m).offset((*buf).offset as isize),
            avail,
            fmt,
            ap.as_va_list(),
        );
        if r > 0 as libc::c_int && (r as size_t) < (*buf).size {
        } else {
            ponyint_assert_fail(
                b"(r > 0) && ((size_t)r < buf->size)\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/printbuf.c\0"
                    as *const u8 as *const libc::c_char,
                57 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"printbuf\0")).as_ptr(),
            );
        };
    }
    let ref mut fresh2 = (*buf).offset;
    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(r as libc::c_ulong) as size_t as size_t;
}
