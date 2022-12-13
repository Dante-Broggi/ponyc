use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
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
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> size_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
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
    use super::fun_h::{cmp_fn, free_fn};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut size_t,
            count: size_t,
            item_bitmap: *mut bitmap_t,
            size: size_t,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
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
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
            index: *mut size_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: size_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/delta.h:1"]
pub mod delta_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:35"]
    pub struct deltamap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
pub use self::delta_h::deltamap_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
use self::pony_h::pony_actor_t;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stddef_h::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "4:16"]
pub struct delta_t {
    pub actor: *mut pony_actor_t,
    pub rc: size_t,
}
#[c2rust::src_loc = "35:1"]
pub type ponyint_deltamap_free_fn = Option<unsafe extern "C" fn(*mut delta_t) -> ()>;
#[c2rust::src_loc = "35:1"]
pub type ponyint_deltamap_cmp_fn = Option<unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool>;
#[c2rust::src_loc = "10:1"]
unsafe extern "C" fn delta_hash(mut delta: *mut delta_t) -> size_t {
    return ponyint_hash_ptr((*delta).actor as *const libc::c_void);
}
#[c2rust::src_loc = "15:1"]
unsafe extern "C" fn delta_cmp(mut a: *mut delta_t, mut b: *mut delta_t) -> bool {
    return (*a).actor == (*b).actor;
}
#[c2rust::src_loc = "20:1"]
unsafe extern "C" fn delta_free(mut delta: *mut delta_t) {
    ponyint_pool_free(0 as libc::c_int as size_t, delta as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "25:1"]
pub unsafe extern "C" fn ponyint_delta_actor(mut delta: *mut delta_t) -> *mut pony_actor_t {
    return (*delta).actor;
}
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn ponyint_delta_rc(mut delta: *mut delta_t) -> size_t {
    return (*delta).rc;
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_alloc_size(mut map: *mut deltamap_t) -> size_t {
    return ponyint_hashmap_alloc_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_putindex(
    mut map: *mut deltamap_t,
    mut entry: *mut delta_t,
    mut index: size_t,
) {
    let mut cmpf: ponyint_deltamap_cmp_fn =
        Some(delta_cmp as unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        delta_hash(entry),
        ::core::mem::transmute::<ponyint_deltamap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_destroy(mut map: *mut deltamap_t) {
    let mut freef: ponyint_deltamap_free_fn =
        Some(delta_free as unsafe extern "C" fn(*mut delta_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_deltamap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_optimize(mut map: *mut deltamap_t) {
    let mut cmpf: ponyint_deltamap_cmp_fn =
        Some(delta_cmp as unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool);
    return ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_deltamap_cmp_fn, cmp_fn>(cmpf),
    );
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_init(mut map: *mut deltamap_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_mem_size(mut map: *mut deltamap_t) -> size_t {
    return ponyint_hashmap_mem_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    return ponyint_hashmap_fill_ratio(map);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_size(mut map: *mut deltamap_t) -> size_t {
    return ponyint_hashmap_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_clearindex(mut map: *mut deltamap_t, mut index: size_t) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ponyint_deltamap_removeindex(mut map: *mut deltamap_t, mut index: size_t) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "35:46"]
pub unsafe extern "C" fn ponyint_deltamap_get(
    mut map: *mut deltamap_t,
    mut key: *mut delta_t,
    mut index: *mut size_t,
) -> *mut delta_t {
    let mut cmpf: ponyint_deltamap_cmp_fn =
        Some(delta_cmp as unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool);
    return ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        delta_hash(key),
        ::core::mem::transmute::<ponyint_deltamap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut delta_t;
}
#[no_mangle]
#[c2rust::src_loc = "35:46"]
pub unsafe extern "C" fn ponyint_deltamap_put(
    mut map: *mut deltamap_t,
    mut entry: *mut delta_t,
) -> *mut delta_t {
    let mut cmpf: ponyint_deltamap_cmp_fn =
        Some(delta_cmp as unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool);
    return ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        delta_hash(entry),
        ::core::mem::transmute::<ponyint_deltamap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut delta_t;
}
#[no_mangle]
#[c2rust::src_loc = "35:46"]
pub unsafe extern "C" fn ponyint_deltamap_next(
    mut map: *mut deltamap_t,
    mut i: *mut size_t,
) -> *mut delta_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    return ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut delta_t;
}
#[no_mangle]
#[c2rust::src_loc = "35:46"]
pub unsafe extern "C" fn ponyint_deltamap_remove(
    mut map: *mut deltamap_t,
    mut entry: *mut delta_t,
) -> *mut delta_t {
    let mut cmpf: ponyint_deltamap_cmp_fn =
        Some(delta_cmp as unsafe extern "C" fn(*mut delta_t, *mut delta_t) -> bool);
    return ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        delta_hash(entry),
        ::core::mem::transmute::<ponyint_deltamap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut delta_t;
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn ponyint_deltamap_update(
    mut map: *mut deltamap_t,
    mut actor: *mut pony_actor_t,
    mut rc: size_t,
) -> *mut deltamap_t {
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    if map.is_null() {
        map = ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut deltamap_t;
        ponyint_deltamap_init(map, 1 as libc::c_int as size_t);
    } else {
        let mut key: delta_t = delta_t {
            actor: 0 as *mut pony_actor_t,
            rc: 0,
        };
        key.actor = actor;
        let mut delta: *mut delta_t = ponyint_deltamap_get(map, &mut key, &mut index);
        if !delta.is_null() {
            (*delta).rc = rc;
            return map;
        }
    }
    let mut delta_0: *mut delta_t = ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut delta_t;
    let ref mut fresh0 = (*delta_0).actor;
    *fresh0 = actor;
    (*delta_0).rc = rc;
    if index == -(1 as libc::c_int) as size_t {
        ponyint_deltamap_put(map, delta_0);
    } else {
        ponyint_deltamap_putindex(map, delta_0, index);
    }
    return map;
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn ponyint_deltamap_free(mut map: *mut deltamap_t) {
    ponyint_deltamap_destroy(map);
    ponyint_pool_free(0 as libc::c_int as size_t, map as *mut libc::c_void);
}
