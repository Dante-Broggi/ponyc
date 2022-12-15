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
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/usr/local/Cellar/llvm@13/13.0.1_2/lib/clang/13.0.1/include/stdarg.h:1"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:1"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct errormsg_t {
        pub file: *const libc::c_char,
        pub line: usize,
        pub pos: usize,
        pub msg: *const libc::c_char,
        pub source: *const libc::c_char,
        pub frame: *mut errormsg_t,
        pub next: *mut errormsg_t,
    }
    #[c2rust::src_loc = "49:1"]
    pub type errorframe_t = *mut errormsg_t;
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;

    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "347:6"]
        pub fn vsnprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ::core::ffi::VaList,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:2"]
pub mod stringtab_h {
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:4"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: usize,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __int64_t};
pub use self::error_h::{errorframe_t, errormsg_t};
pub use self::internal::{__builtin_va_list, __va_list_tag};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::source_h::source_t;
pub use self::stdarg_h::va_list;
use self::stdio_h::{__stderrp, fprintf, vsnprintf};
use self::string_h::{memcpy, memset};
use self::stringtab_h::stringtab;
pub use self::sys__types_h::__darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "12:16"]
pub struct errors_t {
    pub head: *mut errormsg_t,
    pub tail: *mut errormsg_t,
    pub count: usize,
    pub immediate_report: bool,
    pub output_stream: *mut FILE,
}
#[no_mangle]
#[c2rust::src_loc = "22:1"]
pub unsafe extern "C" fn error_alloc() -> *mut errormsg_t {
    let mut e: *mut errormsg_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut errormsg_t;
    memset(
        e as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<errormsg_t>().try_into().unwrap(),
    );
    e
}
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn error_free(mut e: *mut errormsg_t) {
    while !e.is_null() {
        let mut next: *mut errormsg_t = (*e).frame;
        ponyint_pool_free(1 as libc::c_int as usize, e as *mut libc::c_void);
        e = next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn errors_alloc() -> *mut errors_t {
    let mut errors: *mut errors_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut errors_t;
    memset(
        errors as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<errors_t>().try_into().unwrap(),
    );
    let ref mut fresh0 = (*errors).output_stream;
    *fresh0 = __stderrp;
    return errors;
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn errors_free(mut errors: *mut errors_t) {
    let mut e: *mut errormsg_t = (*errors).head;
    while !e.is_null() {
        let mut next: *mut errormsg_t = (*e).next;
        error_free(e);
        e = next;
    }
    ponyint_pool_free(1 as libc::c_int as usize, errors as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn errors_get_first(mut errors: *mut errors_t) -> *mut errormsg_t {
    return (*errors).head;
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn errors_get_count(mut errors: *mut errors_t) -> usize {
    return (*errors).count;
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn errors_set_immediate(mut errors: *mut errors_t, mut immediate: bool) {
    (*errors).immediate_report = immediate;
}
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn errors_set_output_stream(mut errors: *mut errors_t, mut fp: *mut FILE) {
    let ref mut fresh1 = (*errors).output_stream;
    *fresh1 = fp;
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn error_print_msg(
    mut e: *mut errormsg_t,
    mut fp: *mut FILE,
    mut indent: *const libc::c_char,
) {
    if !((*e).file).is_null() {
        fprintf(
            fp,
            b"%s%s:\0" as *const u8 as *const libc::c_char,
            indent,
            (*e).file,
        );
        if (*e).line != 0 {
            fprintf(
                fp,
                b"%zu:%zu: \0" as *const u8 as *const libc::c_char,
                (*e).line,
                (*e).pos,
            );
        } else {
            fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
        }
    }
    fprintf(fp, b"%s\n\0" as *const u8 as *const libc::c_char, (*e).msg);
    if !((*e).source).is_null() {
        fprintf(
            fp,
            b"%s%s\n\0" as *const u8 as *const libc::c_char,
            indent,
            (*e).source,
        );
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, indent);
        let mut i: usize = 0;
        while i < ((*e).pos).wrapping_sub(1) {
            if *((*e).source).offset(i as isize) as libc::c_int == '\t' as i32 {
                fprintf(fp, b"\t\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
            }
            i = i.wrapping_add(1);
        }
        fprintf(fp, b"^\n\0" as *const u8 as *const libc::c_char);
    }
}
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn error_print(mut e: *mut errormsg_t, mut fp: *mut FILE) {
    fprintf(fp, b"Error:\n\0" as *const u8 as *const libc::c_char);
    error_print_msg(e, fp, b"\0" as *const u8 as *const libc::c_char);
    if !((*e).frame).is_null() {
        fprintf(fp, b"    Info:\n\0" as *const u8 as *const libc::c_char);
        let mut ef: *mut errormsg_t = (*e).frame;
        while !ef.is_null() {
            error_print_msg(ef, fp, b"    \0" as *const u8 as *const libc::c_char);
            ef = (*ef).frame;
        }
    }
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn errors_push(mut errors: *mut errors_t, mut e: *mut errormsg_t) {
    if (*errors).immediate_report {
        error_print(e, (*errors).output_stream);
    }
    if ((*errors).head).is_null() {
        let ref mut fresh2 = (*errors).head;
        *fresh2 = e;
        let ref mut fresh3 = (*errors).tail;
        *fresh3 = e;
    } else {
        let ref mut fresh4 = (*(*errors).tail).next;
        *fresh4 = e;
        let ref mut fresh5 = (*errors).tail;
        *fresh5 = e;
    }
    let ref mut fresh6 = (*e).next;
    *fresh6 = 0 as *mut errormsg_t;
    let ref mut fresh7 = (*errors).count;
    *fresh7 = (*fresh7).wrapping_add(1);
}
#[c2rust::src_loc = "147:1"]
unsafe extern "C" fn append_to_frame(mut frame: *mut errorframe_t, mut e: *mut errormsg_t) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"append_to_frame\0"))
                .as_ptr(),
        );
    };
    if (*frame).is_null() {
        *frame = e;
    } else {
        let mut p: *mut errormsg_t = *frame;
        while !((*p).frame).is_null() {
            p = (*p).frame;
        }
        let ref mut fresh8 = (*p).frame;
        *fresh8 = e;
    };
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn errors_print(mut errors: *mut errors_t) {
    if (*errors).immediate_report {
        return;
    }
    let mut e: *mut errormsg_t = (*errors).head;
    while !e.is_null() {
        error_print(e, (*errors).output_stream);
        e = (*e).next;
    }
}
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn make_errorv(
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut errormsg_t {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(
        buf.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    let mut e: *mut errormsg_t = error_alloc();
    if !source.is_null() {
        let ref mut fresh9 = (*e).file;
        *fresh9 = (*source).file;
    }
    (*e).line = line;
    (*e).pos = pos;
    let ref mut fresh10 = (*e).msg;
    *fresh10 = stringtab(buf.as_mut_ptr());
    if !source.is_null() && line != 0 {
        let mut tline: usize = 1 as libc::c_int as usize;
        let mut tpos: usize = 0;
        while tline < (*e).line && tpos < (*source).len {
            if *((*source).m).offset(tpos as isize) as libc::c_int == '\n' as i32 {
                tline = tline.wrapping_add(1);
            }
            tpos = tpos.wrapping_add(1);
        }
        let mut start: usize = tpos;
        while *((*source).m).offset(tpos as isize) as libc::c_int != '\n' as i32
            && tpos < (*source).len
        {
            tpos = tpos.wrapping_add(1);
        }
        let mut len: usize = tpos.wrapping_sub(start);
        if len >= ::core::mem::size_of::<[libc::c_char; 1024]>() {
            len = (::core::mem::size_of::<[libc::c_char; 1024]>()).wrapping_sub(1);
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *((*source).m).offset(start as isize) as *mut libc::c_char as *const libc::c_void,
            len.try_into().unwrap(),
        );
        buf[len as usize] = '\0' as i32 as libc::c_char;
        let ref mut fresh11 = (*e).source;
        *fresh11 = stringtab(buf.as_mut_ptr());
    }
    return e;
}
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn errorv(
    mut errors: *mut errors_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    errors_push(errors, make_errorv(source, line, pos, fmt, ap.as_va_list()));
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn errorv_continue(
    mut errors: *mut errors_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    if ((*errors).tail).is_null() {
        return errorv(errors, source, line, pos, fmt, ap.as_va_list());
    }
    let mut p: *mut errormsg_t = (*errors).tail;
    while !((*p).frame).is_null() {
        p = (*p).frame;
    }
    let ref mut fresh12 = (*p).frame;
    *fresh12 = make_errorv(source, line, pos, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn error(
    mut errors: *mut errors_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv(errors, source, line, pos, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "253:1"]
pub unsafe extern "C" fn error_continue(
    mut errors: *mut errors_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv_continue(errors, source, line, pos, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn errorframev(
    mut frame: *mut errorframe_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"errorframev\0")).as_ptr(),
        );
    };
    append_to_frame(frame, make_errorv(source, line, pos, fmt, ap.as_va_list()));
}
#[no_mangle]
#[c2rust::src_loc = "269:1"]
pub unsafe extern "C" fn errorframe(
    mut frame: *mut errorframe_t,
    mut source: *mut source_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"errorframe\0")).as_ptr(),
        );
    };
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorframev(frame, source, line, pos, fmt, ap.as_va_list());
}
#[c2rust::src_loc = "280:1"]
unsafe extern "C" fn make_errorfv(
    mut file: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut errormsg_t {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(
        buf.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    let mut e: *mut errormsg_t = error_alloc();
    let ref mut fresh13 = (*e).file;
    *fresh13 = stringtab(file);
    let ref mut fresh14 = (*e).msg;
    *fresh14 = stringtab(buf.as_mut_ptr());
    return e;
}
#[no_mangle]
#[c2rust::src_loc = "292:1"]
pub unsafe extern "C" fn errorfv(
    mut errors: *mut errors_t,
    mut file: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    errors_push(errors, make_errorfv(file, fmt, ap.as_va_list()));
}
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn errorfv_continue(
    mut errors: *mut errors_t,
    mut file: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    if ((*errors).tail).is_null() {
        return errorfv(errors, file, fmt, ap.as_va_list());
    }
    let mut p: *mut errormsg_t = (*errors).tail;
    while !((*p).frame).is_null() {
        p = (*p).frame;
    }
    let ref mut fresh15 = (*p).frame;
    *fresh15 = make_errorfv(file, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn errorf(
    mut errors: *mut errors_t,
    mut file: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorfv(errors, file, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "318:1"]
pub unsafe extern "C" fn errorf_continue(
    mut errors: *mut errors_t,
    mut file: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorfv_continue(errors, file, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "326:1"]
pub unsafe extern "C" fn errorframe_append(
    mut first: *mut errorframe_t,
    mut second: *mut errorframe_t,
) {
    if !first.is_null() {
    } else {
        ponyint_assert_fail(
            b"first != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            328 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"errorframe_append\0"))
                .as_ptr(),
        );
    };
    if !second.is_null() {
    } else {
        ponyint_assert_fail(
            b"second != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            329 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"errorframe_append\0"))
                .as_ptr(),
        );
    };
    append_to_frame(first, *second);
    *second = 0 as errorframe_t;
}
#[no_mangle]
#[c2rust::src_loc = "335:1"]
pub unsafe extern "C" fn errorframe_has_errors(mut frame: *mut errorframe_t) -> bool {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            337 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"errorframe_has_errors\0"))
                .as_ptr(),
        );
    };
    return !frame.is_null();
}
#[no_mangle]
#[c2rust::src_loc = "341:1"]
pub unsafe extern "C" fn errorframe_report(
    mut frame: *mut errorframe_t,
    mut errors: *mut errors_t,
) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            343 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"errorframe_report\0"))
                .as_ptr(),
        );
    };
    if !(*frame).is_null() {
        errors_push(errors, *frame);
    }
    *frame = 0 as errorframe_t;
}
#[no_mangle]
#[c2rust::src_loc = "351:1"]
pub unsafe extern "C" fn errorframe_discard(mut frame: *mut errorframe_t) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.c\0" as *const u8
                as *const libc::c_char,
            353 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"errorframe_discard\0"))
                .as_ptr(),
        );
    };
    error_free(*frame);
    *frame = 0 as errorframe_t;
}
