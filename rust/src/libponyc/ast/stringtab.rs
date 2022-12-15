use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "15:1"]
    pub type map_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_hash_block(p: *const libc::c_void, len: usize) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
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
            usize,
            libc::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;
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
        pub traits: *mut *mut libc::uintptr_t,
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/list.h:1"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct list_t {
        pub data: *mut libc::c_void,
        pub next: *mut list_t,
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::fun_h::{cmp_fn, free_fn, map_fn};
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_list_equals(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_list_pop(list: *mut list_t, data: *mut *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_list_push(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_list_append(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_list_next(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_list_index(list: *mut list_t, index: ssize_t) -> *mut list_t;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_list_data(list: *mut list_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_list_find(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "32:1"]
        pub fn ponyint_list_findindex(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> ssize_t;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_list_subset(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_list_map(
            list: *mut list_t,
            f: map_fn,
            arg: *mut libc::c_void,
        ) -> *mut list_t;
        #[c2rust::src_loc = "40:1"]
        pub fn ponyint_list_reverse(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_list_length(list: *mut list_t) -> usize;
        #[c2rust::src_loc = "44:1"]
        pub fn ponyint_list_free(list: *mut list_t, f: free_fn);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    #[c2rust::src_loc = "9:1"]
    pub type strlist_cmp_fn =
        Option<unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> bool>;
    #[c2rust::src_loc = "9:1"]
    pub type strlist_map_fn =
        Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> *const libc::c_char>;
    #[c2rust::src_loc = "9:1"]
    pub type strlist_free_fn = Option<unsafe extern "C" fn(*const libc::c_char) -> ()>;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:2"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: usize,
        pub size: usize,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: usize,
    }
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = usize;
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: usize);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: *mut usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: usize,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut usize,
            count: usize,
            item_bitmap: *mut bitmap_t,
            size: usize,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:3"]
pub mod serialise_h {
    #[c2rust::src_loc = "20:1"]
    pub type deserialise_raw_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::{pony_ctx_t, pony_type_t};
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn pony_deserialise_raw(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
            ds_fn: deserialise_raw_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: libc::uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:7"]
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
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::fun_h::{cmp_fn, free_fn, map_fn, ponyint_hash_block};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::list_h::{
    list_t, ponyint_list_append, ponyint_list_data, ponyint_list_equals, ponyint_list_find,
    ponyint_list_findindex, ponyint_list_free, ponyint_list_index, ponyint_list_length,
    ponyint_list_map, ponyint_list_next, ponyint_list_pop, ponyint_list_push, ponyint_list_reverse,
    ponyint_list_subset,
};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, pony_type_t, C2RustUnnamed, PONY_TRACE_IMMUTABLE,
    PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
pub use self::serialise_h::{
    deserialise_raw_fn, pony_deserialise_offset, pony_deserialise_raw, pony_serialise_offset,
};
use self::string_h::{memcmp, memcpy, memset, strlen};
pub use self::stringtab_h::{strlist_cmp_fn, strlist_free_fn, strlist_map_fn};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "16:22"]
pub struct strlist_t {
    pub contents: list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:27"]
pub struct strtable_t {
    pub contents: hashmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "18:16"]
pub struct stringtab_entry_t {
    pub str_0: *const libc::c_char,
    pub len: usize,
    pub buf_size: usize,
}
#[c2rust::src_loc = "42:1"]
pub type strtable_cmp_fn =
    Option<unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool>;
#[c2rust::src_loc = "42:1"]
pub type strtable_free_fn = Option<unsafe extern "C" fn(*mut stringtab_entry_t) -> ()>;
#[c2rust::src_loc = "11:1"]
unsafe extern "C" fn ptr_cmp(mut a: *const libc::c_char, mut b: *const libc::c_char) -> bool {
    a == b
}
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn strlist_subset(mut a: *mut strlist_t, mut b: *mut strlist_t) -> bool {
    let mut cmp: strlist_cmp_fn =
        Some(ptr_cmp as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> bool);
    ponyint_list_subset(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<strlist_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn strlist_length(mut list: *mut strlist_t) -> usize {
    ponyint_list_length(list as *mut list_t)
}
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn strlist_free(mut list: *mut strlist_t) {
    let mut free: strlist_free_fn = None;
    ponyint_list_free(
        list as *mut list_t,
        ::core::mem::transmute::<strlist_free_fn, free_fn>(free),
    );
}
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn strlist_findindex(
    mut list: *mut strlist_t,
    mut data: *const libc::c_char,
) -> ssize_t {
    let mut cmp: strlist_cmp_fn =
        Some(ptr_cmp as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> bool);
    ponyint_list_findindex(
        list as *mut list_t,
        ::core::mem::transmute::<strlist_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    )
}
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn strlist_equals(mut a: *mut strlist_t, mut b: *mut strlist_t) -> bool {
    let mut cmp: strlist_cmp_fn =
        Some(ptr_cmp as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> bool);
    ponyint_list_equals(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<strlist_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_index(
    mut list: *mut strlist_t,
    mut index: ssize_t,
) -> *mut strlist_t {
    ponyint_list_index(list as *mut list_t, index) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_next(mut list: *mut strlist_t) -> *mut strlist_t {
    ponyint_list_next(list as *mut list_t) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_append(
    mut list: *mut strlist_t,
    mut data: *const libc::c_char,
) -> *mut strlist_t {
    ponyint_list_append(list as *mut list_t, data as *mut libc::c_void) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_map(
    mut list: *mut strlist_t,
    mut f: strlist_map_fn,
    mut arg: *mut libc::c_void,
) -> *mut strlist_t {
    ponyint_list_map(
        list as *mut list_t,
        ::core::mem::transmute::<strlist_map_fn, map_fn>(f),
        arg,
    ) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_reverse(mut list: *mut strlist_t) -> *mut strlist_t {
    ponyint_list_reverse(list as *mut list_t) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_pop(
    mut list: *mut strlist_t,
    mut data: *mut *const libc::c_char,
) -> *mut strlist_t {
    ponyint_list_pop(list as *mut list_t, data as *mut *mut libc::c_void) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:22"]
pub unsafe extern "C" fn strlist_push(
    mut list: *mut strlist_t,
    mut data: *const libc::c_char,
) -> *mut strlist_t {
    ponyint_list_push(list as *mut list_t, data as *mut libc::c_void) as *mut strlist_t
}
#[no_mangle]
#[c2rust::src_loc = "16:33"]
pub unsafe extern "C" fn strlist_data(mut list: *mut strlist_t) -> *const libc::c_char {
    ponyint_list_data(list as *mut list_t) as *const libc::c_char
}
#[no_mangle]
#[c2rust::src_loc = "16:33"]
pub unsafe extern "C" fn strlist_find(
    mut list: *mut strlist_t,
    mut data: *const libc::c_char,
) -> *const libc::c_char {
    let mut cmp: strlist_cmp_fn =
        Some(ptr_cmp as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> bool);
    ponyint_list_find(
        list as *mut list_t,
        ::core::mem::transmute::<strlist_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    ) as *const libc::c_char
}
#[c2rust::src_loc = "25:1"]
unsafe extern "C" fn stringtab_hash(mut a: *mut stringtab_entry_t) -> usize {
    ponyint_hash_block((*a).str_0 as *const libc::c_void, (*a).len)
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn stringtab_cmp(
    mut a: *mut stringtab_entry_t,
    mut b: *mut stringtab_entry_t,
) -> bool {
    return (*a).len == (*b).len
        && memcmp(
            (*a).str_0 as *const libc::c_void,
            (*b).str_0 as *const libc::c_void,
            (*a).len,
        ) == 0 as libc::c_int;
}
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn stringtab_free(mut a: *mut stringtab_entry_t) {
    ponyint_pool_free_size(
        (*a).buf_size,
        (*a).str_0 as *mut libc::c_char as *mut libc::c_void,
    );
    ponyint_pool_free(0 as libc::c_int as usize, a as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_destroy(mut map: *mut strtable_t) {
    let mut freef: strtable_free_fn =
        Some(stringtab_free as unsafe extern "C" fn(*mut stringtab_entry_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<strtable_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_init(mut map: *mut strtable_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_removeindex(mut map: *mut strtable_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_clearindex(mut map: *mut strtable_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_size(mut map: *mut strtable_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_optimize(mut map: *mut strtable_t) {
    let mut cmpf: strtable_cmp_fn = Some(
        stringtab_cmp
            as unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<strtable_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_alloc_size(mut map: *mut strtable_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_putindex(
    mut map: *mut strtable_t,
    mut entry: *mut stringtab_entry_t,
    mut index: usize,
) {
    let mut cmpf: strtable_cmp_fn = Some(
        stringtab_cmp
            as unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        stringtab_hash(entry),
        ::core::mem::transmute::<strtable_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn strtable_mem_size(mut map: *mut strtable_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "42:38"]
pub unsafe extern "C" fn strtable_next(
    mut map: *mut strtable_t,
    mut i: *mut usize,
) -> *mut stringtab_entry_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut stringtab_entry_t
}
#[no_mangle]
#[c2rust::src_loc = "42:38"]
pub unsafe extern "C" fn strtable_remove(
    mut map: *mut strtable_t,
    mut entry: *mut stringtab_entry_t,
) -> *mut stringtab_entry_t {
    let mut cmpf: strtable_cmp_fn = Some(
        stringtab_cmp
            as unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        stringtab_hash(entry),
        ::core::mem::transmute::<strtable_cmp_fn, cmp_fn>(cmpf),
    ) as *mut stringtab_entry_t
}
#[no_mangle]
#[c2rust::src_loc = "42:38"]
pub unsafe extern "C" fn strtable_get(
    mut map: *mut strtable_t,
    mut key: *mut stringtab_entry_t,
    mut index: *mut usize,
) -> *mut stringtab_entry_t {
    let mut cmpf: strtable_cmp_fn = Some(
        stringtab_cmp
            as unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        stringtab_hash(key),
        ::core::mem::transmute::<strtable_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut stringtab_entry_t
}
#[no_mangle]
#[c2rust::src_loc = "42:38"]
pub unsafe extern "C" fn strtable_put(
    mut map: *mut strtable_t,
    mut entry: *mut stringtab_entry_t,
) -> *mut stringtab_entry_t {
    let mut cmpf: strtable_cmp_fn = Some(
        stringtab_cmp
            as unsafe extern "C" fn(*mut stringtab_entry_t, *mut stringtab_entry_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        stringtab_hash(entry),
        ::core::mem::transmute::<strtable_cmp_fn, cmp_fn>(cmpf),
    ) as *mut stringtab_entry_t
}
#[c2rust::src_loc = "45:19"]
static mut table: strtable_t = strtable_t {
    contents: hashmap_t {
        count: 0,
        size: 0,
        item_bitmap: 0 as *const bitmap_t as *mut bitmap_t,
        buckets: 0 as *const hashmap_entry_t as *mut hashmap_entry_t,
    },
};
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn stringtab_init() {
    strtable_init(&mut table, 4096 as libc::c_int as usize);
}
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn stringtab(mut string: *const libc::c_char) -> *const libc::c_char {
    if string.is_null() {
        return 0 as *const libc::c_char;
    }
    stringtab_len(string, strlen(string))
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn stringtab_len(
    mut string: *const libc::c_char,
    mut len: usize,
) -> *const libc::c_char {
    if string.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut key: stringtab_entry_t = {
        let mut init = stringtab_entry_t {
            str_0: string,
            len: len,
            buf_size: 0 as libc::c_int as usize,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut n: *mut stringtab_entry_t = strtable_get(&mut table, &mut key, &mut index);
    if !n.is_null() {
        return (*n).str_0;
    }
    let mut dst: *mut libc::c_char =
        ponyint_pool_alloc_size(len.wrapping_add(1))
            as *mut libc::c_char;
    memcpy(dst as *mut libc::c_void, string as *const libc::c_void, len);
    *dst.offset(len as isize) = '\0' as i32 as libc::c_char;
    n = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut stringtab_entry_t;
    let ref mut fresh0 = (*n).str_0;
    *fresh0 = dst;
    (*n).len = len;
    (*n).buf_size = len.wrapping_add(1);
    strtable_putindex(&mut table, n, index);
    (*n).str_0
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn stringtab_consume(
    mut string: *const libc::c_char,
    mut buf_size: usize,
) -> *const libc::c_char {
    if string.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut len: usize = libc::strlen(string);
    let mut key: stringtab_entry_t = {
        let mut init = stringtab_entry_t {
            str_0: string,
            len: len,
            buf_size: 0 as libc::c_int as usize,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut n: *mut stringtab_entry_t = strtable_get(&mut table, &mut key, &mut index);
    if !n.is_null() {
        ponyint_pool_free_size(buf_size, string as *mut libc::c_void);
        return (*n).str_0;
    }
    n = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut stringtab_entry_t;
    let ref mut fresh1 = (*n).str_0;
    *fresh1 = string;
    (*n).len = len;
    (*n).buf_size = buf_size;
    strtable_putindex(&mut table, n, index);
    (*n).str_0
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn stringtab_done() {
    strtable_destroy(&mut table);
    memset(
        &mut table as *mut strtable_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<strtable_t>(),
    );
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn string_serialise(
    mut _ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut string: *const libc::c_char = object as *const libc::c_char;
    memcpy(
        (buf as libc::uintptr_t).wrapping_add(offset) as *mut libc::c_void,
        object,
        (libc::strlen(string)).wrapping_add(1),
    );
}
#[thread_local]
#[c2rust::src_loc = "131:48"]
static mut string_pony: _pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: 0 as libc::c_int as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: None,
            serialise: Some(
                string_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
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
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn string_trace(mut ctx: *mut pony_ctx_t, mut string: *const libc::c_char) {
    string_trace_len(ctx, string, strlen(string));
}
#[no_mangle]
#[c2rust::src_loc = "157:1"]
pub unsafe extern "C" fn string_trace_len(
    mut ctx: *mut pony_ctx_t,
    mut string: *const libc::c_char,
    mut len: usize,
) {
    string_pony.size = len.wrapping_add(1) as u32;
    pony_traceknown(
        ctx,
        string as *mut libc::c_char as *mut libc::c_void,
        &mut string_pony,
        PONY_TRACE_OPAQUE as libc::c_int,
    );
}
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn string_deserialise(
    mut buf: *mut libc::c_void,
    mut remaining_size: usize,
) -> *mut libc::c_void {
    let mut len: usize = 1 as libc::c_int as usize;
    loop {
        if len >= remaining_size {
            return 0 as *mut libc::c_void;
        }
        let fresh2 = len;
        len = len.wrapping_add(1);
        if !(*(buf as *mut libc::c_char).offset(fresh2 as isize) as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return stringtab_len(
        buf as *const libc::c_char,
        len.wrapping_sub(1),
    ) as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "176:1"]
pub unsafe extern "C" fn string_deserialise_offset(
    mut ctx: *mut pony_ctx_t,
    mut offset: libc::uintptr_t,
) -> *const libc::c_char {
    return pony_deserialise_raw(
        ctx,
        offset,
        Some(
            string_deserialise
                as unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void,
        ),
    ) as *const libc::c_char;
}
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn strlist_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut list: *mut strlist_t = object as *mut strlist_t;
    if !((*list).contents.data).is_null() {
        string_trace(ctx, (*list).contents.data as *const libc::c_char);
    }
    if !((*list).contents.next).is_null() {
        pony_traceknown(
            ctx,
            (*list).contents.next as *mut libc::c_void,
            strlist_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "193:1"]
unsafe extern "C" fn strlist_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut list: *mut strlist_t = object as *mut strlist_t;
    let mut dst: *mut strlist_t = (buf as libc::uintptr_t).wrapping_add(offset) as *mut strlist_t;
    let ref mut fresh3 = (*dst).contents.data;
    *fresh3 = pony_serialise_offset(ctx, (*list).contents.data) as *mut libc::c_void;
    let ref mut fresh4 = (*dst).contents.next;
    *fresh4 = pony_serialise_offset(ctx, (*list).contents.next as *mut libc::c_void) as *mut list_t;
}
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn strlist_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut list: *mut strlist_t = object as *mut strlist_t;
    let ref mut fresh5 = (*list).contents.data;
    *fresh5 =
        string_deserialise_offset(ctx, (*list).contents.data as libc::uintptr_t) as *mut libc::c_void;
    let ref mut fresh6 = (*list).contents.next;
    *fresh6 = pony_deserialise_offset(ctx, strlist_pony_type(), (*list).contents.next as libc::uintptr_t)
        as *mut list_t;
}
#[c2rust::src_loc = "216:20"]
static mut strlist_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<strlist_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                strlist_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                strlist_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                strlist_deserialise
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
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn strlist_pony_type() -> *const pony_type_t {
    &strlist_pony
}
