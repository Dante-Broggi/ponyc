use ::libc;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:2"]
pub mod pony_h {
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:2"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:2"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:2"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:34"]
    pub struct muteset_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct muteref_t {
        pub key: *mut pony_actor_t,
        pub value: muteset_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_actor_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:1"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::mutemap_h::{mutemap_t, muteref_t, muteset_t};
use self::pony_h::pony_actor_t;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::string_h::memset;
#[c2rust::src_loc = "17:1"]
pub type ponyint_muteset_free_fn = Option<unsafe extern "C" fn(*mut pony_actor_t) -> ()>;
#[c2rust::src_loc = "17:1"]
pub type ponyint_muteset_cmp_fn =
    Option<unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool>;
#[c2rust::src_loc = "45:1"]
pub type ponyint_mutemap_free_fn = Option<unsafe extern "C" fn(*mut muteref_t) -> ()>;
#[c2rust::src_loc = "45:1"]
pub type ponyint_mutemap_cmp_fn =
    Option<unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool>;
#[c2rust::src_loc = "7:1"]
unsafe extern "C" fn muteset_hash(mut actor: *mut pony_actor_t) -> size_t {
    ponyint_hash_ptr(actor as *const libc::c_void)
}
#[c2rust::src_loc = "12:1"]
unsafe extern "C" fn muteset_cmp(mut a: *mut pony_actor_t, mut b: *mut pony_actor_t) -> bool {
    a == b
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_putindex(
    mut map: *mut muteset_t,
    mut entry: *mut pony_actor_t,
    mut index: size_t,
) {
    let mut cmpf: ponyint_muteset_cmp_fn =
        Some(muteset_cmp as unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteset_hash(entry),
        ::core::mem::transmute::<ponyint_muteset_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_alloc_size(mut map: *mut muteset_t) -> size_t {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_init(mut map: *mut muteset_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_destroy(mut map: *mut muteset_t) {
    let mut freef: ponyint_muteset_free_fn = None;
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_muteset_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_optimize(mut map: *mut muteset_t) {
    let mut cmpf: ponyint_muteset_cmp_fn =
        Some(muteset_cmp as unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_muteset_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_removeindex(mut map: *mut muteset_t, mut index: size_t) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_clearindex(mut map: *mut muteset_t, mut index: size_t) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_size(mut map: *mut muteset_t) -> size_t {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_muteset_mem_size(mut map: *mut muteset_t) -> size_t {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "17:44"]
pub unsafe extern "C" fn ponyint_muteset_get(
    mut map: *mut muteset_t,
    mut key: *mut pony_actor_t,
    mut index: *mut size_t,
) -> *mut pony_actor_t {
    let mut cmpf: ponyint_muteset_cmp_fn =
        Some(muteset_cmp as unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        muteset_hash(key),
        ::core::mem::transmute::<ponyint_muteset_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut pony_actor_t
}
#[no_mangle]
#[c2rust::src_loc = "17:44"]
pub unsafe extern "C" fn ponyint_muteset_put(
    mut map: *mut muteset_t,
    mut entry: *mut pony_actor_t,
) -> *mut pony_actor_t {
    let mut cmpf: ponyint_muteset_cmp_fn =
        Some(muteset_cmp as unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteset_hash(entry),
        ::core::mem::transmute::<ponyint_muteset_cmp_fn, cmp_fn>(cmpf),
    ) as *mut pony_actor_t
}
#[no_mangle]
#[c2rust::src_loc = "17:44"]
pub unsafe extern "C" fn ponyint_muteset_next(
    mut map: *mut muteset_t,
    mut i: *mut size_t,
) -> *mut pony_actor_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut pony_actor_t
}
#[no_mangle]
#[c2rust::src_loc = "17:44"]
pub unsafe extern "C" fn ponyint_muteset_remove(
    mut map: *mut muteset_t,
    mut entry: *mut pony_actor_t,
) -> *mut pony_actor_t {
    let mut cmpf: ponyint_muteset_cmp_fn =
        Some(muteset_cmp as unsafe extern "C" fn(*mut pony_actor_t, *mut pony_actor_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteset_hash(entry),
        ::core::mem::transmute::<ponyint_muteset_cmp_fn, cmp_fn>(cmpf),
    ) as *mut pony_actor_t
}
#[c2rust::src_loc = "20:1"]
unsafe extern "C" fn muteref_hash(mut mref: *mut muteref_t) -> size_t {
    ponyint_hash_ptr((*mref).key as *const libc::c_void)
}
#[c2rust::src_loc = "25:1"]
unsafe extern "C" fn muteref_cmp(mut a: *mut muteref_t, mut b: *mut muteref_t) -> bool {
    (*a).key == (*b).key
}
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn ponyint_muteref_alloc(mut key: *mut pony_actor_t) -> *mut muteref_t {
    let mut mref: *mut muteref_t = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut muteref_t;
    memset(
        mref as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<muteref_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*mref).key;
    *fresh0 = key;
    mref
}
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn ponyint_muteref_free(mut mref: *mut muteref_t) {
    ponyint_muteset_destroy(&mut (*mref).value);
    ponyint_pool_free(1 as libc::c_int as size_t, mref as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_optimize(mut map: *mut mutemap_t) {
    let mut cmpf: ponyint_mutemap_cmp_fn =
        Some(muteref_cmp as unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_mutemap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_destroy(mut map: *mut mutemap_t) {
    let mut freef: ponyint_mutemap_free_fn =
        Some(ponyint_muteref_free as unsafe extern "C" fn(*mut muteref_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_mutemap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_removeindex(mut map: *mut mutemap_t, mut index: size_t) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_mem_size(mut map: *mut mutemap_t) -> size_t {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_clearindex(mut map: *mut mutemap_t, mut index: size_t) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_size(mut map: *mut mutemap_t) -> size_t {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_init(mut map: *mut mutemap_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_alloc_size(mut map: *mut mutemap_t) -> size_t {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn ponyint_mutemap_putindex(
    mut map: *mut mutemap_t,
    mut entry: *mut muteref_t,
    mut index: size_t,
) {
    let mut cmpf: ponyint_mutemap_cmp_fn =
        Some(muteref_cmp as unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteref_hash(entry),
        ::core::mem::transmute::<ponyint_mutemap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "45:44"]
pub unsafe extern "C" fn ponyint_mutemap_get(
    mut map: *mut mutemap_t,
    mut key: *mut muteref_t,
    mut index: *mut size_t,
) -> *mut muteref_t {
    let mut cmpf: ponyint_mutemap_cmp_fn =
        Some(muteref_cmp as unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        muteref_hash(key),
        ::core::mem::transmute::<ponyint_mutemap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut muteref_t
}
#[no_mangle]
#[c2rust::src_loc = "45:44"]
pub unsafe extern "C" fn ponyint_mutemap_next(
    mut map: *mut mutemap_t,
    mut i: *mut size_t,
) -> *mut muteref_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut muteref_t
}
#[no_mangle]
#[c2rust::src_loc = "45:44"]
pub unsafe extern "C" fn ponyint_mutemap_remove(
    mut map: *mut mutemap_t,
    mut entry: *mut muteref_t,
) -> *mut muteref_t {
    let mut cmpf: ponyint_mutemap_cmp_fn =
        Some(muteref_cmp as unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteref_hash(entry),
        ::core::mem::transmute::<ponyint_mutemap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut muteref_t
}
#[no_mangle]
#[c2rust::src_loc = "45:44"]
pub unsafe extern "C" fn ponyint_mutemap_put(
    mut map: *mut mutemap_t,
    mut entry: *mut muteref_t,
) -> *mut muteref_t {
    let mut cmpf: ponyint_mutemap_cmp_fn =
        Some(muteref_cmp as unsafe extern "C" fn(*mut muteref_t, *mut muteref_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        muteref_hash(entry),
        ::core::mem::transmute::<ponyint_mutemap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut muteref_t
}
