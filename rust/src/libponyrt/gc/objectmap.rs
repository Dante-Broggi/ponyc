use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> usize;
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
    use super::_uintptr_t_h::uintptr_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = usize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: usize,
        pub size: usize,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    use super::fun_h::{cmp_fn, free_fn};
    use super::stddef_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:1"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct object_t {
        pub address: *mut libc::c_void,
        pub rc: usize,
        pub mark: u32,
        pub immutable: bool,
        pub type_0: *const pony_type_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:2"]
pub mod heap_h {
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
        #[c2rust::src_loc = "89:1"]
        pub fn ponyint_heap_mark_shallow(chunk: *mut chunk_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pagemap.h:6"]
pub mod pagemap_h {
    use super::heap_h::chunk_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn ponyint_pagemap_get(addr: *const libc::c_void) -> *mut chunk_t;
    }
}
pub use self::_uintptr_t_h::uintptr_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
use self::heap_h::{chunk_t, ponyint_heap_mark_shallow};
pub use self::objectmap_h::{object_t, objectmap_t};
use self::pagemap_h::ponyint_pagemap_get;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_type_t,
};
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stddef_h::size_t;
#[c2rust::src_loc = "37:1"]
pub type ponyint_objectmap_free_fn = Option<unsafe extern "C" fn(*mut object_t) -> ()>;
#[c2rust::src_loc = "37:1"]
pub type ponyint_objectmap_cmp_fn =
    Option<unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool>;
#[c2rust::src_loc = "9:1"]
unsafe extern "C" fn object_hash(mut obj: *mut object_t) -> usize {
    ponyint_hash_ptr((*obj).address)
}
#[c2rust::src_loc = "14:1"]
unsafe extern "C" fn object_cmp(mut a: *mut object_t, mut b: *mut object_t) -> bool {
    (*a).address == (*b).address
}
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn object_alloc(
    mut address: *mut libc::c_void,
    mut type_0: *const pony_type_t,
    mut mark: u32,
) -> *mut object_t {
    let mut obj: *mut object_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut object_t;
    let ref mut fresh0 = (*obj).address;
    *fresh0 = address;
    (*obj).rc = 0 as libc::c_int as usize;
    (*obj).immutable = 0 as libc::c_int != 0;
    let ref mut fresh1 = (*obj).type_0;
    *fresh1 = type_0;
    (*obj).mark = mark.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return obj;
}
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn object_free(mut obj: *mut object_t) {
    ponyint_pool_free(0 as libc::c_int as usize, obj as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_init(mut map: *mut objectmap_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_alloc_size(mut map: *mut objectmap_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_mem_size(mut map: *mut objectmap_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_optimize(mut map: *mut objectmap_t) {
    let mut cmpf: ponyint_objectmap_cmp_fn =
        Some(object_cmp as unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_objectmap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_destroy(mut map: *mut objectmap_t) {
    let mut freef: ponyint_objectmap_free_fn =
        Some(object_free as unsafe extern "C" fn(*mut object_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_objectmap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_size(mut map: *mut objectmap_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_clearindex(mut map: *mut objectmap_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_removeindex(
    mut map: *mut objectmap_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn ponyint_objectmap_putindex(
    mut map: *mut objectmap_t,
    mut entry: *mut object_t,
    mut index: usize,
) {
    let mut cmpf: ponyint_objectmap_cmp_fn =
        Some(object_cmp as unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        object_hash(entry),
        ::core::mem::transmute::<ponyint_objectmap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "37:48"]
pub unsafe extern "C" fn ponyint_objectmap_put(
    mut map: *mut objectmap_t,
    mut entry: *mut object_t,
) -> *mut object_t {
    let mut cmpf: ponyint_objectmap_cmp_fn =
        Some(object_cmp as unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        object_hash(entry),
        ::core::mem::transmute::<ponyint_objectmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut object_t
}
#[no_mangle]
#[c2rust::src_loc = "37:48"]
pub unsafe extern "C" fn ponyint_objectmap_remove(
    mut map: *mut objectmap_t,
    mut entry: *mut object_t,
) -> *mut object_t {
    let mut cmpf: ponyint_objectmap_cmp_fn =
        Some(object_cmp as unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        object_hash(entry),
        ::core::mem::transmute::<ponyint_objectmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut object_t
}
#[no_mangle]
#[c2rust::src_loc = "37:48"]
pub unsafe extern "C" fn ponyint_objectmap_get(
    mut map: *mut objectmap_t,
    mut key: *mut object_t,
    mut index: *mut usize,
) -> *mut object_t {
    let mut cmpf: ponyint_objectmap_cmp_fn =
        Some(object_cmp as unsafe extern "C" fn(*mut object_t, *mut object_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        object_hash(key),
        ::core::mem::transmute::<ponyint_objectmap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut object_t
}
#[no_mangle]
#[c2rust::src_loc = "37:48"]
pub unsafe extern "C" fn ponyint_objectmap_next(
    mut map: *mut objectmap_t,
    mut i: *mut usize,
) -> *mut object_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets) as *mut object_t
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn ponyint_objectmap_getobject(
    mut map: *mut objectmap_t,
    mut address: *mut libc::c_void,
    mut index: *mut usize,
) -> *mut object_t {
    let mut obj: object_t = object_t {
        address: 0 as *mut libc::c_void,
        rc: 0,
        mark: 0,
        immutable: false,
        type_0: 0 as *const pony_type_t,
    };
    obj.address = address;
    ponyint_objectmap_get(map, &mut obj, index)
}
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn ponyint_objectmap_getorput(
    mut map: *mut objectmap_t,
    mut address: *mut libc::c_void,
    mut type_0: *const pony_type_t,
    mut mark: u32,
) -> *mut object_t {
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut obj: *mut object_t = ponyint_objectmap_getobject(map, address, &mut index);
    if !obj.is_null() {
        return obj;
    }
    obj = object_alloc(address, type_0, mark);
    ponyint_objectmap_putindex(map, obj, index);
    obj
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn ponyint_objectmap_sweep(mut map: *mut objectmap_t) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut obj: *mut object_t = 0 as *mut object_t;
    let mut needs_optimize: bool = 0 as libc::c_int != 0;
    loop {
        obj = ponyint_objectmap_next(map, &mut i);
        if obj.is_null() {
            break;
        }
        let mut p: *mut libc::c_void = (*obj).address;
        if (*obj).rc > 0 {
            let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
            ponyint_heap_mark_shallow(chunk, p);
        } else {
            ponyint_objectmap_clearindex(map, i);
            needs_optimize = 1 as libc::c_int != 0;
            object_free(obj);
        }
    }
    if needs_optimize {
        ponyint_objectmap_optimize(map);
    }
}
