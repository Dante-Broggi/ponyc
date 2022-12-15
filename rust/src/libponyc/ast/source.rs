use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:3"]
pub mod pony_h {
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
        pub traits: *mut *mut libc::uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "74:1"]
    pub type pony_trace_fn = Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "84:1"]
    pub type pony_serialise_fn = Option<
        unsafe extern "C" fn(
            *mut pony_ctx_t,
            *mut libc::c_void,
            *mut libc::c_void,
            usize,
            libc::c_int,
        ) -> (),
    >;
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[c2rust::src_loc = "9:1"]
    pub type pony_type_t = _pony_type_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
    use super::_size_t_h::size_t;
    use super::pony_h::_pony_type_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:2"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:2"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "143:1"]
        pub fn fclose(_: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "153:7"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
        #[c2rust::src_loc = "158:9"]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
        #[c2rust::src_loc = "162:1"]
        pub fn fseek(_: *mut FILE, _: libc::c_long, _: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "164:1"]
        pub fn ftell(_: *mut FILE) -> libc::c_long;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:3"]
pub mod stringtab_h {
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn string_trace(ctx: *mut pony_ctx_t, string: *const libc::c_char);
        #[c2rust::src_loc = "27:1"]
        pub fn string_deserialise_offset(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
        ) -> *const libc::c_char;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:4"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
        #[c2rust::src_loc = "42:1"]
        pub fn pony_deserialise_block(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
            size: usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "37:1"]
        pub fn pony_serialise_reserve(ctx: *mut pony_ctx_t, p: *mut libc::c_void, size: usize);
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
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t, __int64_t};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn,
};
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
use self::serialise_h::{pony_deserialise_block, pony_serialise_offset, pony_serialise_reserve};
pub use self::source_h::{pony_type_t, source_t};
use self::stdio_h::{fclose, fopen, fread, fseek, ftell};
use self::string_h::{memcpy, strlen};
use self::stringtab_h::{string_deserialise_offset, string_trace, stringtab};
pub use self::sys__types_h::__darwin_off_t;
#[no_mangle]
#[c2rust::src_loc = "10:1"]
pub unsafe extern "C" fn source_open(
    mut file: *const libc::c_char,
    mut error_msgp: *mut *const libc::c_char,
) -> *mut source_t {
    let mut fp: *mut FILE = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        *error_msgp = b"can't open file\0" as *const u8 as *const libc::c_char;
        return 0 as *mut source_t;
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let mut size: ssize_t = ftell(fp);
    if size < 0 as libc::c_int as libc::c_long {
        *error_msgp = b"can't determine length of file\0" as *const u8 as *const libc::c_char;
        fclose(fp);
        return 0 as *mut source_t;
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    let mut source: *mut source_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut source_t;
    let ref mut fresh0 = (*source).file;
    *fresh0 = stringtab(file);
    let ref mut fresh1 = (*source).m;
    *fresh1 = ponyint_pool_alloc_size((size + 1 as libc::c_int as libc::c_long) as usize)
        as *mut libc::c_char;
    (*source).len = (size + 1 as libc::c_int as libc::c_long) as usize;
    let mut read: ssize_t = fread(
        (*source).m as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>(),
        size as libc::c_ulong,
        fp,
    ) as ssize_t;
    *((*source).m).offset(size as isize) = '\0' as i32 as libc::c_char;
    if read < size {
        *error_msgp = b"failed to read entire file\0" as *const u8 as *const libc::c_char;
        ponyint_pool_free_size((*source).len, (*source).m as *mut libc::c_void);
        ponyint_pool_free(0 as libc::c_int as usize, source as *mut libc::c_void);
        fclose(fp);
        return 0 as *mut source_t;
    }
    fclose(fp);
    return source;
}
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn source_open_string(mut source_code: *const libc::c_char) -> *mut source_t {
    let mut source: *mut source_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut source_t;
    let ref mut fresh2 = (*source).file;
    *fresh2 = 0 as *const libc::c_char;
    (*source).len = (libc::strlen(source_code)).wrapping_add(1);
    let ref mut fresh3 = (*source).m;
    *fresh3 = ponyint_pool_alloc_size((*source).len) as *mut libc::c_char;
    memcpy(
        (*source).m as *mut libc::c_void,
        source_code as *const libc::c_void,
        (*source).len,
    );
    return source;
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn source_close(mut source: *mut source_t) {
    if source.is_null() {
        return;
    }
    if !((*source).m).is_null() {
        ponyint_pool_free_size((*source).len, (*source).m as *mut libc::c_void);
    }
    ponyint_pool_free(0 as libc::c_int as usize, source as *mut libc::c_void);
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn source_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut source: *mut source_t = object as *mut source_t;
    if !((*source).file).is_null() {
        string_trace(ctx, (*source).file);
    }
    pony_serialise_reserve(ctx, (*source).m as *mut libc::c_void, (*source).len);
}
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn source_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut source: *mut source_t = object as *mut source_t;
    let mut dst: *mut source_t = (buf as libc::uintptr_t).wrapping_add(offset) as *mut source_t;
    let ref mut fresh4 = (*dst).file;
    *fresh4 = pony_serialise_offset(
        ctx,
        (*source).file as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh5 = (*dst).m;
    *fresh5 = pony_serialise_offset(ctx, (*source).m as *mut libc::c_void) as *mut libc::c_char;
    (*dst).len = (*source).len;
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn source_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut source: *mut source_t = object as *mut source_t;
    let ref mut fresh6 = (*source).file;
    *fresh6 = string_deserialise_offset(ctx, (*source).file as libc::uintptr_t);
    let ref mut fresh7 = (*source).m;
    *fresh7 =
        pony_deserialise_block(ctx, (*source).m as libc::uintptr_t, (*source).len) as *mut libc::c_char;
}
#[c2rust::src_loc = "111:20"]
static mut source_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<source_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                source_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                source_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                source_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as u32,
            traits: 0 as *const *mut libc::uintptr_t as *mut *mut libc::uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn source_pony_type() -> *const pony_type_t {
    &source_pony
}
