use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_ct_rune_t = libc::c_int;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> size_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: uint32_t,
        pub id: uint32_t,
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
        pub id: uint32_t,
        pub size: uint32_t,
        pub field_count: uint32_t,
        pub field_offset: uint32_t,
        pub instance: *mut libc::c_void,
        pub trace: pony_trace_fn,
        pub serialise_trace: pony_trace_fn,
        pub serialise: pony_serialise_fn,
        pub deserialise: pony_trace_fn,
        pub custom_serialise_space: pony_custom_serialise_space_fn,
        pub custom_deserialise: pony_custom_deserialise_fn,
        pub dispatch: pony_dispatch_fn,
        pub final_0: pony_final_fn,
        pub event_notify: uint32_t,
        pub traits: *mut *mut uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed = 0;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "394:1"]
        pub fn pony_traceknown(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            m: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = size_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: size_t,
        pub size: size_t,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    use super::pony_h::{pony_ctx_t, pony_type_t};
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: size_t);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
            index: *mut size_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
            index: size_t,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut size_t,
            count: size_t,
            item_bitmap: *mut bitmap_t,
            size: size_t,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "124:1"]
        pub fn ponyint_hashmap_serialise_trace(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
        #[c2rust::src_loc = "127:1"]
        pub fn ponyint_hashmap_serialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            buf: *mut libc::c_void,
            offset: size_t,
        );
        #[c2rust::src_loc = "130:1"]
        pub fn ponyint_hashmap_deserialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    #[c2rust::src_loc = "12:9"]
    pub type sym_status_t = libc::c_uint;
    #[c2rust::src_loc = "21:3"]
    pub const SYM_ERROR: sym_status_t = 7;
    #[c2rust::src_loc = "20:3"]
    pub const SYM_FFIDECL: sym_status_t = 6;
    #[c2rust::src_loc = "19:3"]
    pub const SYM_CONSUMED_SAME_EXPR: sym_status_t = 5;
    #[c2rust::src_loc = "18:3"]
    pub const SYM_CONSUMED: sym_status_t = 4;
    #[c2rust::src_loc = "17:3"]
    pub const SYM_UNDEFINED: sym_status_t = 3;
    #[c2rust::src_loc = "16:3"]
    pub const SYM_DEFINED: sym_status_t = 2;
    #[c2rust::src_loc = "15:3"]
    pub const SYM_NOCASE: sym_status_t = 1;
    #[c2rust::src_loc = "14:3"]
    pub const SYM_NONE: sym_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct symbol_t {
        pub name: *const libc::c_char,
        pub def: *mut ast_t,
        pub status: sym_status_t,
        pub branch_count: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:35"]
    pub struct symtab_t {
        pub contents: hashmap_t,
    }
    use super::_size_t_h::size_t;
    use super::hash_h::hashmap_t;
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
        #[c2rust::src_loc = "10:16"]
        pub type errors_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:2"]
pub mod stringtab_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn stringtab_consume(
            string: *const libc::c_char,
            buf_size: size_t,
        ) -> *const libc::c_char;
        #[c2rust::src_loc = "24:1"]
        pub fn string_trace(ctx: *mut pony_ctx_t, string: *const libc::c_char);
        #[c2rust::src_loc = "27:1"]
        pub fn string_deserialise_offset(
            ctx: *mut pony_ctx_t,
            offset: uintptr_t,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:3"]
pub mod ast_h {
    use super::pony_h::pony_type_t;
    use super::symtab_h::{ast_t, errors_t};
    extern "C" {
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "223:1"]
        pub fn ast_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:3"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:4"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn is_name_private(name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "39:1"]
        pub fn is_name_type(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:5"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:5"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::{pony_ctx_t, pony_type_t};
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> size_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:11"]
pub mod _ctype_h {
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "291:1"]
    pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
        __tolower(_c)
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "297:1"]
    pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
        __toupper(_c)
    }
    use super::_types_h::__darwin_ct_rune_t;
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
        #[c2rust::src_loc = "189:1"]
        pub fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    }
}
pub use self::_ctype_h::{__tolower, __toupper, tolower, toupper};
pub use self::_size_t_h::size_t;
pub use self::_types_h::{__darwin_ct_rune_t, __darwin_size_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uintptr_t_h::uintptr_t;
use self::ast_h::{ast_error, ast_pony_type};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_deserialise, ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio,
    ponyint_hashmap_get, ponyint_hashmap_init, ponyint_hashmap_mem_size, ponyint_hashmap_next,
    ponyint_hashmap_optimize, ponyint_hashmap_put, ponyint_hashmap_putindex,
    ponyint_hashmap_remove, ponyint_hashmap_removeindex, ponyint_hashmap_serialise,
    ponyint_hashmap_serialise_trace, ponyint_hashmap_size,
};
use self::id_h::{is_name_private, is_name_type};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, pony_type_t, C2RustUnnamed, PONY_TRACE_IMMUTABLE,
    PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free};
use self::serialise_h::{pony_deserialise_offset, pony_serialise_offset};
use self::stdio_h::printf;
use self::string_h::{memcpy, strcmp, strlen};
use self::stringtab_h::{string_deserialise_offset, string_trace, stringtab_consume};
pub use self::symtab_h::{
    ast_t, errors_t, sym_status_t, symbol_t, symtab_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR,
    SYM_DEFINED, SYM_ERROR, SYM_FFIDECL, SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};
#[c2rust::src_loc = "35:1"]
pub type symtab_free_fn = Option<unsafe extern "C" fn(*mut symbol_t) -> ()>;
#[c2rust::src_loc = "35:1"]
pub type symtab_cmp_fn = Option<unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool>;
#[c2rust::src_loc = "13:1"]
unsafe extern "C" fn sym_hash(mut sym: *mut symbol_t) -> size_t {
    ponyint_hash_ptr((*sym).name as *const libc::c_void)
}
#[c2rust::src_loc = "18:1"]
unsafe extern "C" fn sym_cmp(mut a: *mut symbol_t, mut b: *mut symbol_t) -> bool {
    (*a).name == (*b).name
}
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn sym_dup(mut sym: *mut symbol_t) -> *mut symbol_t {
    let mut s: *mut symbol_t = ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut symbol_t;
    memcpy(
        s as *mut libc::c_void,
        sym as *const libc::c_void,
        ::core::mem::size_of::<symbol_t>() as libc::c_ulong,
    );
    s
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn sym_free(mut sym: *mut symbol_t) {
    ponyint_pool_free(0 as libc::c_int as size_t, sym as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_alloc_size(mut map: *mut symtab_t) -> size_t {
    return ponyint_hashmap_alloc_size(map as *mut hashmap_t);
}
#[c2rust::src_loc = "35:1"]
static mut symtab_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as uint32_t,
            size: ::core::mem::size_of::<symtab_t>() as libc::c_ulong as uint32_t,
            field_count: 0 as libc::c_int as uint32_t,
            field_offset: 0 as libc::c_int as uint32_t,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                symtab_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                symtab_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                symtab_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as uint32_t,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_pony_type() -> *const pony_type_t {
    &symtab_pony
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_destroy(mut map: *mut symtab_t) {
    let mut freef: symtab_free_fn = Some(sym_free as unsafe extern "C" fn(*mut symbol_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<symtab_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_optimize(mut map: *mut symtab_t) {
    let mut cmpf: symtab_cmp_fn =
        Some(sym_cmp as unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<symtab_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_init(mut map: *mut symtab_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_putindex(
    mut map: *mut symtab_t,
    mut entry: *mut symbol_t,
    mut index: size_t,
) {
    let mut cmpf: symtab_cmp_fn =
        Some(sym_cmp as unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        sym_hash(entry),
        ::core::mem::transmute::<symtab_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_removeindex(mut map: *mut symtab_t, mut index: size_t) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_clearindex(mut map: *mut symtab_t, mut index: size_t) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_size(mut map: *mut symtab_t) -> size_t {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, symbol_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_mem_size(mut map: *mut symtab_t) -> size_t {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn symtab_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, symbol_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "35:44"]
pub unsafe extern "C" fn symtab_get(
    mut map: *mut symtab_t,
    mut key: *mut symbol_t,
    mut index: *mut size_t,
) -> *mut symbol_t {
    let mut cmpf: symtab_cmp_fn =
        Some(sym_cmp as unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        sym_hash(key),
        ::core::mem::transmute::<symtab_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut symbol_t
}
#[no_mangle]
#[c2rust::src_loc = "35:44"]
pub unsafe extern "C" fn symtab_remove(
    mut map: *mut symtab_t,
    mut entry: *mut symbol_t,
) -> *mut symbol_t {
    let mut cmpf: symtab_cmp_fn =
        Some(sym_cmp as unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        sym_hash(entry),
        ::core::mem::transmute::<symtab_cmp_fn, cmp_fn>(cmpf),
    ) as *mut symbol_t
}
#[no_mangle]
#[c2rust::src_loc = "35:44"]
pub unsafe extern "C" fn symtab_next(mut map: *mut symtab_t, mut i: *mut size_t) -> *mut symbol_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut symbol_t
}
#[no_mangle]
#[c2rust::src_loc = "35:44"]
pub unsafe extern "C" fn symtab_put(
    mut map: *mut symtab_t,
    mut entry: *mut symbol_t,
) -> *mut symbol_t {
    let mut cmpf: symtab_cmp_fn =
        Some(sym_cmp as unsafe extern "C" fn(*mut symbol_t, *mut symbol_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        sym_hash(entry),
        ::core::mem::transmute::<symtab_cmp_fn, cmp_fn>(cmpf),
    ) as *mut symbol_t
}
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn name_without_case(mut name: *const libc::c_char) -> *const libc::c_char {
    let mut len: size_t = (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buf: *mut libc::c_char = ponyint_pool_alloc_size(len) as *mut libc::c_char;
    if is_name_type(name) {
        let mut i: size_t = 0;
        while i < len {
            *buf.offset(i as isize) =
                toupper(*name.offset(i as isize) as libc::c_int) as libc::c_char;
            i = i.wrapping_add(1);
        }
    } else {
        let mut i_0: size_t = 0;
        while i_0 < len {
            *buf.offset(i_0 as isize) =
                tolower(*name.offset(i_0 as isize) as libc::c_int) as libc::c_char;
            i_0 = i_0.wrapping_add(1);
        }
    }
    stringtab_consume(buf, len)
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn symtab_new() -> *mut symtab_t {
    let mut symtab: *mut symtab_t = ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut symtab_t;
    symtab_init(symtab, 8 as libc::c_int as size_t);
    return symtab;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn symtab_dup(mut symtab: *mut symtab_t) -> *mut symtab_t {
    let mut n: *mut symtab_t = ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut symtab_t;
    symtab_init(n, symtab_size(symtab));
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next(symtab, &mut i);
        if sym.is_null() {
            break;
        }
        symtab_put(n, sym_dup(sym));
    }
    n
}
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn symtab_free(mut symtab: *mut symtab_t) {
    if symtab.is_null() {
        return;
    }
    symtab_destroy(symtab);
    ponyint_pool_free(0 as libc::c_int as size_t, symtab as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn symtab_add(
    mut symtab: *mut symtab_t,
    mut name: *const libc::c_char,
    mut def: *mut ast_t,
    mut status: sym_status_t,
) -> bool {
    let mut no_case: *const libc::c_char = name_without_case(name);
    if no_case != name {
        let mut s1: symbol_t = {
            let mut init = symbol_t {
                name: no_case,
                def: def,
                status: SYM_NOCASE,
                branch_count: 0 as libc::c_int as size_t,
            };
            init
        };
        let mut index: size_t = -(1 as libc::c_int) as size_t;
        let mut s2: *mut symbol_t = symtab_get(symtab, &mut s1, &mut index);
        if !s2.is_null() {
            return 0 as libc::c_int != 0;
        }
        symtab_putindex(symtab, sym_dup(&mut s1), index);
    }
    let mut s1_0: symbol_t = {
        let mut init = symbol_t {
            name: name,
            def: def,
            status: status,
            branch_count: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut index_0: size_t = -(1 as libc::c_int) as size_t;
    let mut s2_0: *mut symbol_t = symtab_get(symtab, &mut s1_0, &mut index_0);
    if !s2_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    symtab_putindex(symtab, sym_dup(&mut s1_0), index_0);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn symtab_find(
    mut symtab: *mut symtab_t,
    mut name: *const libc::c_char,
    mut status: *mut sym_status_t,
) -> *mut ast_t {
    let mut s1: symbol_t = {
        let mut init = symbol_t {
            name: name,
            def: 0 as *mut ast_t,
            status: SYM_NONE,
            branch_count: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut s2: *mut symbol_t = symtab_get(symtab, &mut s1, &mut index);
    if !s2.is_null() {
        if !status.is_null() {
            *status = (*s2).status;
        }
        if (*s2).status as libc::c_uint == SYM_NOCASE as libc::c_int as libc::c_uint {
            return 0 as *mut ast_t;
        }
        return (*s2).def;
    }
    if !status.is_null() {
        *status = SYM_NONE;
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn symtab_find_case(
    mut symtab: *mut symtab_t,
    mut name: *const libc::c_char,
    mut status: *mut sym_status_t,
) -> *mut ast_t {
    let mut s1: symbol_t = {
        let mut init = symbol_t {
            name: name,
            def: 0 as *mut ast_t,
            status: SYM_NONE,
            branch_count: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut s2: *mut symbol_t = symtab_get(symtab, &mut s1, &mut index);
    if !s2.is_null() {
        if !status.is_null() {
            *status = (*s2).status;
        }
        return (*s2).def;
    }
    let mut no_case: *const libc::c_char = name_without_case(name);
    if no_case != name {
        return symtab_find_case(symtab, no_case, status);
    }
    if !status.is_null() {
        *status = SYM_NONE;
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub unsafe extern "C" fn symtab_set_status(
    mut symtab: *mut symtab_t,
    mut name: *const libc::c_char,
    mut status: sym_status_t,
) {
    let mut s1: symbol_t = {
        let mut init = symbol_t {
            name: name,
            def: 0 as *mut ast_t,
            status: status,
            branch_count: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut s2: *mut symbol_t = symtab_get(symtab, &mut s1, &mut index);
    if !s2.is_null() {
        (*s2).status = status;
    } else {
        symtab_putindex(symtab, sym_dup(&mut s1), index);
    };
}
#[no_mangle]
#[c2rust::src_loc = "184:1"]
pub unsafe extern "C" fn symtab_inherit_status(mut dst: *mut symtab_t, mut src: *mut symtab_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    loop {
        sym = symtab_next(src, &mut i);
        if sym.is_null() {
            break;
        }
        if !((*sym).def).is_null() {
            continue;
        }
        let mut dsym: *mut symbol_t = symtab_get(dst, sym, &mut index);
        if !dsym.is_null() {
            (*dsym).status = (*sym).status;
        } else {
            symtab_putindex(dst, sym_dup(sym), index);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "211:1"]
pub unsafe extern "C" fn symtab_inherit_branch(mut dst: *mut symtab_t, mut src: *mut symtab_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    loop {
        sym = symtab_next(src, &mut i);
        if sym.is_null() {
            break;
        }
        if !((*sym).def).is_null() {
            continue;
        }
        let mut dsym: *mut symbol_t = symtab_get(dst, sym, &mut index);
        if !dsym.is_null() {
            if (*sym).status as libc::c_uint == SYM_DEFINED as libc::c_int as libc::c_uint {
                if (*dsym).status as libc::c_uint == SYM_UNDEFINED as libc::c_int as libc::c_uint {
                    let ref mut fresh0 = (*dsym).branch_count;
                    *fresh0 = (*fresh0).wrapping_add(1);
                }
            } else if (*sym).status as libc::c_uint == SYM_CONSUMED as libc::c_int as libc::c_uint {
                (*dsym).status = SYM_CONSUMED;
                (*dsym).branch_count = 0 as libc::c_int as size_t;
            } else if (*sym).status as libc::c_uint
                == SYM_CONSUMED_SAME_EXPR as libc::c_int as libc::c_uint
            {
                (*dsym).status = SYM_CONSUMED_SAME_EXPR;
                (*dsym).branch_count = 0 as libc::c_int as size_t;
            }
        } else {
            dsym = sym_dup(sym);
            if (*dsym).status as libc::c_uint == SYM_DEFINED as libc::c_int as libc::c_uint {
                (*dsym).status = SYM_UNDEFINED;
                (*dsym).branch_count = 1 as libc::c_int as size_t;
            }
            symtab_putindex(dst, dsym, index);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "259:1"]
pub unsafe extern "C" fn symtab_can_merge_public(
    mut dst: *mut symtab_t,
    mut src: *mut symtab_t,
) -> bool {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next(src, &mut i);
        if sym.is_null() {
            break;
        }
        if is_name_private((*sym).name) as libc::c_int != 0
            || (*sym).status as libc::c_uint == SYM_NOCASE as libc::c_int as libc::c_uint
            || strcmp((*sym).name, b"Main\0" as *const u8 as *const libc::c_char) == 0
        {
            continue;
        }
        if !(symtab_find_case(dst, (*sym).name, 0 as *mut sym_status_t)).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "278:1"]
pub unsafe extern "C" fn symtab_merge_public(
    mut dst: *mut symtab_t,
    mut src: *mut symtab_t,
) -> bool {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next(src, &mut i);
        if sym.is_null() {
            break;
        }
        if is_name_private((*sym).name) as libc::c_int != 0
            || (*sym).status as libc::c_uint == SYM_NOCASE as libc::c_int as libc::c_uint
            || strcmp((*sym).name, b"Main\0" as *const u8 as *const libc::c_char) == 0
        {
            continue;
        }
        if !symtab_add(dst, (*sym).name, (*sym).def, (*sym).status) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn symtab_check_all_defined(
    mut symtab: *mut symtab_t,
    mut errors: *mut errors_t,
) -> bool {
    let mut r: bool = 1 as libc::c_int != 0;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next(symtab, &mut i);
        if sym.is_null() {
            break;
        }
        if !((*sym).def).is_null()
            && (*sym).status as libc::c_uint == SYM_UNDEFINED as libc::c_int as libc::c_uint
        {
            ast_error(
                errors,
                (*sym).def,
                b"Local variable %s is not assigned a value in all code paths\0" as *const u8
                    as *const libc::c_char,
                (*sym).name,
            );
            r = 0 as libc::c_int != 0;
        }
    }
    r
}
#[no_mangle]
#[c2rust::src_loc = "318:1"]
pub unsafe extern "C" fn symtab_print(mut symtab: *mut symtab_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next(symtab, &mut i);
        if sym.is_null() {
            break;
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, (*sym).name);
        match (*sym).status as libc::c_uint {
            3 => {
                printf(b": undefined\n\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                printf(b": defined\n\0" as *const u8 as *const libc::c_char);
            }
            4 => {
                printf(b": consumed\n\0" as *const u8 as *const libc::c_char);
            }
            5 => {
                printf(b": consumed same expression\n\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                printf(b": UNKNOWN\n\0" as *const u8 as *const libc::c_char);
            }
        }
    }
}
#[c2rust::src_loc = "352:1"]
unsafe extern "C" fn symbol_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut sym: *mut symbol_t = object as *mut symbol_t;
    string_trace(ctx, (*sym).name as *mut libc::c_char);
    if !((*sym).def).is_null() {
        pony_traceknown(
            ctx,
            (*sym).def as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "362:1"]
unsafe extern "C" fn symbol_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut _mutability: libc::c_int,
) {
    let mut sym: *mut symbol_t = object as *mut symbol_t;
    let mut dst: *mut symbol_t = (buf as uintptr_t).wrapping_add(offset) as *mut symbol_t;
    let ref mut fresh1 = (*dst).name;
    *fresh1 = pony_serialise_offset(ctx, (*sym).name as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh2 = (*dst).def;
    *fresh2 = pony_serialise_offset(ctx, (*sym).def as *mut libc::c_void) as *mut ast_t;
    (*dst).status = (*sym).status;
    (*dst).branch_count = (*sym).branch_count;
}
#[c2rust::src_loc = "376:1"]
unsafe extern "C" fn symbol_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut sym: *mut symbol_t = object as *mut symbol_t;
    let ref mut fresh3 = (*sym).name;
    *fresh3 = string_deserialise_offset(ctx, (*sym).name as uintptr_t);
    let ref mut fresh4 = (*sym).def;
    *fresh4 = pony_deserialise_offset(ctx, ast_pony_type(), (*sym).def as uintptr_t) as *mut ast_t;
}
#[c2rust::src_loc = "386:20"]
static mut symbol_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as uint32_t,
            size: ::core::mem::size_of::<symbol_t>() as libc::c_ulong as uint32_t,
            field_count: 0 as libc::c_int as uint32_t,
            field_offset: 0 as libc::c_int as uint32_t,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                symbol_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                symbol_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                symbol_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as uint32_t,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "407:1"]
pub unsafe extern "C" fn symbol_pony_type() -> *const pony_type_t {
    &symbol_pony
}
