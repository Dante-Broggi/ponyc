use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:1"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
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
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_next_pow2(i: size_t) -> size_t;
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
    use super::_uint32_t_h::uint32_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::stddef_h::size_t;
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
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/platform.h:1"]
pub mod platform_h {
    #[inline]
    #[c2rust::src_loc = "258:1"]
    pub unsafe extern "C" fn __pony_ffsll(mut x: uint64_t) -> uint32_t {
        return (if x as libc::c_longlong == 0 {
            0
        } else {
            (x as libc::c_longlong).trailing_zeros() as i32 + 1
        }) as uint32_t;
    }
    #[inline]
    #[c2rust::src_loc = "319:1"]
    pub unsafe extern "C" fn __pony_ffszu(mut x: size_t) -> uint32_t {
        return __pony_ffsll(x as uint64_t);
    }
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_pool_used_size(index: size_t) -> size_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:2"]
pub mod serialise_h {
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::{pony_ctx_t, pony_type_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn pony_serialise_reserve(ctx: *mut pony_ctx_t, p: *mut libc::c_void, size: size_t);
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> size_t;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "42:1"]
        pub fn pony_deserialise_block(
            ctx: *mut pony_ctx_t,
            offset: uintptr_t,
            size: size_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:3"]
pub mod ponyassert_h {
    use super::stddef_h::size_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:5"]
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
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_next_pow2};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::platform_h::{__pony_ffsll, __pony_ffszu};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, pony_type_t, C2RustUnnamed, PONY_TRACE_IMMUTABLE,
    PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc_size, ponyint_pool_free_size, ponyint_pool_used_size};
use self::serialise_h::{
    pony_deserialise_block, pony_deserialise_offset, pony_serialise_offset, pony_serialise_reserve,
};
pub use self::stddef_h::size_t;
use self::string_h::{memcpy, memset};
#[c2rust::src_loc = "10:1"]
unsafe extern "C" fn get_probe_length(
    mut map: *mut hashmap_t,
    mut hash: size_t,
    mut current: size_t,
    mut mask: size_t,
) -> size_t {
    return current.wrapping_add((*map).size).wrapping_sub(hash & mask) & mask;
}
#[c2rust::src_loc = "16:1"]
unsafe extern "C" fn search(
    mut map: *mut hashmap_t,
    mut pos: *mut size_t,
    mut key: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
    mut probe_length: *mut size_t,
    mut oi_probe_length: *mut size_t,
) -> *mut libc::c_void {
    let mut mask: size_t = ((*map).size).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut p_length: size_t = *probe_length;
    let mut oi_p_length: size_t = 0 as libc::c_int as size_t;
    let mut index: size_t = (hash & mask).wrapping_add(p_length) & mask;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut elem_hash: size_t = 0;
    let mut ib_index: size_t = 0;
    let mut ib_offset: size_t = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i <= mask {
        ib_index = index >> 6 as libc::c_int;
        ib_offset =
            index & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
        if *((*map).item_bitmap).offset(ib_index as isize)
            & (1 as libc::c_int as bitmap_t) << ib_offset
            == 0 as libc::c_int as libc::c_ulong
        {
            *pos = index;
            *probe_length = p_length;
            return 0 as *mut libc::c_void;
        }
        elem_hash = (*((*map).buckets).offset(index as isize)).hash;
        oi_p_length = get_probe_length(map, elem_hash, index, mask);
        if p_length > oi_p_length {
            *pos = index;
            *probe_length = p_length;
            *oi_probe_length = oi_p_length;
            return 0 as *mut libc::c_void;
        }
        if hash == elem_hash {
            elem = (*((*map).buckets).offset(index as isize)).ptr;
            if cmp.expect("non-null function pointer")(key, elem) {
                *pos = index;
                *probe_length = p_length;
                return elem;
            }
        }
        index = index.wrapping_add(1 as libc::c_int as libc::c_ulong) & mask;
        p_length = p_length.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    *pos = (*map).size;
    *probe_length = p_length;
    return 0 as *mut libc::c_void;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn resize(mut map: *mut hashmap_t, mut cmp: cmp_fn) {
    let mut s: size_t = (*map).size;
    let mut c: size_t = (*map).count;
    let mut b: *mut hashmap_entry_t = (*map).buckets;
    let mut old_item_bitmap: *mut bitmap_t = (*map).item_bitmap;
    let mut curr: *mut libc::c_void = 0 as *mut libc::c_void;
    (*map).count = 0 as libc::c_int as size_t;
    (*map).size = if s < 8 as libc::c_int as libc::c_ulong {
        8 as libc::c_int as libc::c_ulong
    } else {
        s << 3 as libc::c_int
    };
    let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
        (if (*map).size
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    let mut mem_alloc: *mut libc::c_void = ponyint_pool_alloc_size(
        bitmap_size
            .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
            .wrapping_add(
                ((*map).size)
                    .wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
            ),
    );
    memset(
        mem_alloc,
        0 as libc::c_int,
        bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong),
    );
    let ref mut fresh0 = (*map).item_bitmap;
    *fresh0 = mem_alloc as *mut bitmap_t;
    let ref mut fresh1 = (*map).buckets;
    *fresh1 = (mem_alloc as *mut libc::c_char).offset(
        bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong) as isize,
    ) as *mut hashmap_entry_t;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    loop {
        curr = ponyint_hashmap_next(&mut i, c, old_item_bitmap, s, b);
        if curr.is_null() {
            break;
        }
        ponyint_hashmap_put(map, curr, (*b.offset(i as isize)).hash, cmp);
    }
    if !b.is_null() {
        let mut old_bitmap_size: size_t = (s >> 6 as libc::c_int).wrapping_add(
            (if s & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_ulong,
        );
        ponyint_pool_free_size(
            old_bitmap_size
                .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
                .wrapping_add(
                    s.wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
                ),
            old_item_bitmap as *mut libc::c_void,
        );
    }
    if (*map).count == c {
    } else {
        ponyint_assert_fail(
            b"map->count == c\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0" as *const u8
                as *const libc::c_char,
            111 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"resize\0")).as_ptr(),
        );
    };
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn optimize_item(
    mut map: *mut hashmap_t,
    mut cmp: cmp_fn,
    mut old_index: size_t,
) -> size_t {
    let mut mask: size_t = ((*map).size).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut h: size_t = (*((*map).buckets).offset(old_index as isize)).hash;
    let mut entry: *mut libc::c_void = (*((*map).buckets).offset(old_index as isize)).ptr;
    let mut index: size_t = h & mask;
    let mut ib_index: size_t = 0;
    let mut ib_offset: size_t = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i <= mask {
        if index == old_index {
            break;
        }
        ib_index = index >> 6 as libc::c_int;
        ib_offset =
            index & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
        if *((*map).item_bitmap).offset(ib_index as isize)
            & (1 as libc::c_int as bitmap_t) << ib_offset
            == 0 as libc::c_int as libc::c_ulong
        {
            ponyint_hashmap_clearindex(map, old_index);
            ponyint_hashmap_putindex(map, entry, h, cmp, index);
            return 1 as libc::c_int as size_t;
        } else {
            let mut item_probe_length: size_t = get_probe_length(map, h, index, mask);
            let mut there_probe_length: size_t = get_probe_length(
                map,
                (*((*map).buckets).offset(index as isize)).hash,
                index,
                mask,
            );
            if item_probe_length > there_probe_length {
                ponyint_hashmap_clearindex(map, old_index);
                ponyint_hashmap_putindex(map, entry, h, cmp, index);
                return 1 as libc::c_int as size_t;
            }
        }
        index = index.wrapping_add(1 as libc::c_int as libc::c_ulong) & mask;
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub unsafe extern "C" fn ponyint_hashmap_init(mut map: *mut hashmap_t, mut size: size_t) {
    size <<= 1 as libc::c_int;
    if size < 8 as libc::c_int as libc::c_ulong {
        size = 8 as libc::c_int as size_t;
    } else {
        size = ponyint_next_pow2(size);
    }
    (*map).count = 0 as libc::c_int as size_t;
    (*map).size = size;
    let mut bitmap_size: size_t = (size >> 6 as libc::c_int).wrapping_add(
        (if size & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    let mut mem_alloc: *mut libc::c_void = ponyint_pool_alloc_size(
        bitmap_size
            .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
            .wrapping_add(
                size.wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
            ),
    );
    memset(
        mem_alloc,
        0 as libc::c_int,
        bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong),
    );
    let ref mut fresh2 = (*map).item_bitmap;
    *fresh2 = mem_alloc as *mut bitmap_t;
    let ref mut fresh3 = (*map).buckets;
    *fresh3 = (mem_alloc as *mut libc::c_char).offset(
        bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong) as isize,
    ) as *mut hashmap_entry_t;
}
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn ponyint_hashmap_destroy(mut map: *mut hashmap_t, mut free_elem: free_fn) {
    if free_elem.is_some() {
        let mut curr: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut i: size_t = -(1 as libc::c_int) as size_t;
        loop {
            curr = ponyint_hashmap_next(
                &mut i,
                (*map).count,
                (*map).item_bitmap,
                (*map).size,
                (*map).buckets,
            );
            if curr.is_null() {
                break;
            }
            free_elem.expect("non-null function pointer")(curr);
        }
    }
    if (*map).size > 0 as libc::c_int as libc::c_ulong {
        let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
            (if (*map).size
                & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_ulong,
        );
        ponyint_pool_free_size(
            bitmap_size
                .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
                .wrapping_add(
                    ((*map).size)
                        .wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
                ),
            (*map).item_bitmap as *mut libc::c_void,
        );
    }
    (*map).count = 0 as libc::c_int as size_t;
    (*map).size = 0 as libc::c_int as size_t;
    let ref mut fresh4 = (*map).buckets;
    *fresh4 = 0 as *mut hashmap_entry_t;
    let ref mut fresh5 = (*map).item_bitmap;
    *fresh5 = 0 as *mut bitmap_t;
}
#[no_mangle]
#[c2rust::src_loc = "233:1"]
pub unsafe extern "C" fn ponyint_hashmap_get(
    mut map: *mut hashmap_t,
    mut key: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
    mut pos: *mut size_t,
) -> *mut libc::c_void {
    if (*map).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut probe_length: size_t = 0 as libc::c_int as size_t;
    let mut oi_probe_length: size_t = 0 as libc::c_int as size_t;
    return search(
        map,
        pos,
        key,
        hash,
        cmp,
        &mut probe_length,
        &mut oi_probe_length,
    );
}
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn shift_put(
    mut map: *mut hashmap_t,
    mut entry: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
    mut index: size_t,
    mut pl: size_t,
    mut oi_pl: size_t,
    mut e: *mut libc::c_void,
) {
    let mut elem: *mut libc::c_void = e;
    let mut ci_hash: size_t = hash;
    let mut ci_entry: *mut libc::c_void = entry;
    let mut oi_hash: size_t = 0 as libc::c_int as size_t;
    let mut oi_entry: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pos: size_t = index;
    let mut probe_length: size_t = pl;
    let mut oi_probe_length: size_t = oi_pl;
    let mut ib_index: size_t = pos >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        pos & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    if probe_length > oi_probe_length
        || probe_length == oi_probe_length && probe_length == 0 as libc::c_int as libc::c_ulong
    {
    } else {
        ponyint_assert_fail(
            b"probe_length > oi_probe_length || (probe_length == oi_probe_length && probe_length == 0)\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0"
                as *const u8 as *const libc::c_char,
            260 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"shift_put\0"))
                .as_ptr(),
        );
    };
    loop {
        if pos < (*map).size {
        } else {
            ponyint_assert_fail(
                b"pos < map->size\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0" as *const u8
                    as *const libc::c_char,
                263 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"shift_put\0"))
                    .as_ptr(),
            );
        };
        if elem.is_null()
            && *((*map).item_bitmap).offset(ib_index as isize)
                & (1 as libc::c_int as bitmap_t) << ib_offset
                != 0 as libc::c_int as libc::c_ulong
        {
            oi_entry = (*((*map).buckets).offset(pos as isize)).ptr;
            oi_hash = (*((*map).buckets).offset(pos as isize)).hash;
            let ref mut fresh6 = (*((*map).buckets).offset(pos as isize)).ptr;
            *fresh6 = ci_entry;
            (*((*map).buckets).offset(pos as isize)).hash = ci_hash;
            ci_entry = oi_entry;
            ci_hash = oi_hash;
            probe_length = oi_probe_length;
            elem = search(
                map,
                &mut pos,
                ci_entry,
                ci_hash,
                cmp,
                &mut probe_length,
                &mut oi_probe_length,
            );
            ib_index = pos >> 6 as libc::c_int;
            ib_offset = pos
                & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
        } else {
            let ref mut fresh7 = (*((*map).buckets).offset(pos as isize)).ptr;
            *fresh7 = ci_entry;
            (*((*map).buckets).offset(pos as isize)).hash = ci_hash;
            if elem.is_null() {
                let ref mut fresh8 = (*map).count;
                *fresh8 = (*fresh8).wrapping_add(1);
                let ref mut fresh9 = *((*map).item_bitmap).offset(ib_index as isize);
                *fresh9 |= (1 as libc::c_int as bitmap_t) << ib_offset;
                if (*map).count << 1 as libc::c_int > (*map).size {
                    resize(map, cmp);
                }
            }
            return;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn ponyint_hashmap_put(
    mut map: *mut hashmap_t,
    mut entry: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
) -> *mut libc::c_void {
    if (*map).size == 0 as libc::c_int as libc::c_ulong {
        ponyint_hashmap_init(map, 4 as libc::c_int as size_t);
    }
    let mut pos: size_t = 0;
    let mut probe_length: size_t = 0 as libc::c_int as size_t;
    let mut oi_probe_length: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut libc::c_void = search(
        map,
        &mut pos,
        entry,
        hash,
        cmp,
        &mut probe_length,
        &mut oi_probe_length,
    );
    shift_put(
        map,
        entry,
        hash,
        cmp,
        pos,
        probe_length,
        oi_probe_length,
        elem,
    );
    return elem;
}
#[no_mangle]
#[c2rust::src_loc = "332:1"]
pub unsafe extern "C" fn ponyint_hashmap_putindex(
    mut map: *mut hashmap_t,
    mut entry: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
    mut pos: size_t,
) {
    if pos == -(1 as libc::c_int) as size_t {
        ponyint_hashmap_put(map, entry, hash, cmp);
        return;
    }
    if (*map).size == 0 as libc::c_int as libc::c_ulong {
        ponyint_hashmap_init(map, 4 as libc::c_int as size_t);
    }
    if pos < (*map).size {
    } else {
        ponyint_assert_fail(
            b"pos < map->size\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0" as *const u8
                as *const libc::c_char,
            344 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"ponyint_hashmap_putindex\0",
            ))
            .as_ptr(),
        );
    };
    let mut ib_index: size_t = pos >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        pos & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    if *((*map).item_bitmap).offset(ib_index as isize) & (1 as libc::c_int as bitmap_t) << ib_offset
        == 0 as libc::c_int as libc::c_ulong
    {
        let ref mut fresh10 = (*((*map).buckets).offset(pos as isize)).ptr;
        *fresh10 = entry;
        (*((*map).buckets).offset(pos as isize)).hash = hash;
        let ref mut fresh11 = (*map).count;
        *fresh11 = (*fresh11).wrapping_add(1);
        let ref mut fresh12 = *((*map).item_bitmap).offset(ib_index as isize);
        *fresh12 |= (1 as libc::c_int as bitmap_t) << ib_offset;
        if (*map).count << 1 as libc::c_int > (*map).size {
            resize(map, cmp);
        }
    } else {
        let mut mask: size_t = ((*map).size).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut oi_hash: size_t = (*((*map).buckets).offset(pos as isize)).hash;
        let mut oi_probe_length: size_t = get_probe_length(map, oi_hash, pos, mask);
        let mut ci_probe_length: size_t = get_probe_length(map, hash, pos, mask);
        if ci_probe_length > oi_probe_length {
            shift_put(
                map,
                entry,
                hash,
                cmp,
                pos,
                ci_probe_length,
                oi_probe_length,
                0 as *mut libc::c_void,
            );
        } else {
            ponyint_hashmap_put(map, entry, hash, cmp);
        }
    };
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn shift_delete(mut map: *mut hashmap_t, mut index: size_t) {
    if index < (*map).size {
    } else {
        ponyint_assert_fail(
            b"index < map->size\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0" as *const u8
                as *const libc::c_char,
            387 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"shift_delete\0")).as_ptr(),
        );
    };
    let mut pos: size_t = index;
    let mut mask: size_t = ((*map).size).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut next_pos: size_t = pos.wrapping_add(1 as libc::c_int as libc::c_ulong) & mask;
    let mut ni_hash: size_t = (*((*map).buckets).offset(next_pos as isize)).hash;
    let mut ni_ib_index: size_t = next_pos >> 6 as libc::c_int;
    let mut ni_ib_offset: size_t =
        next_pos & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    while *((*map).item_bitmap).offset(ni_ib_index as isize)
        & (1 as libc::c_int as bitmap_t) << ni_ib_offset
        != 0 as libc::c_int as libc::c_ulong
        && get_probe_length(map, ni_hash, next_pos, mask) != 0 as libc::c_int as libc::c_ulong
    {
        let ref mut fresh13 = (*((*map).buckets).offset(pos as isize)).ptr;
        *fresh13 = (*((*map).buckets).offset(next_pos as isize)).ptr;
        (*((*map).buckets).offset(pos as isize)).hash =
            (*((*map).buckets).offset(next_pos as isize)).hash;
        pos = next_pos;
        next_pos = pos.wrapping_add(1 as libc::c_int as libc::c_ulong) & mask;
        ni_hash = (*((*map).buckets).offset(next_pos as isize)).hash;
        ni_ib_index = next_pos >> 6 as libc::c_int;
        ni_ib_offset = next_pos
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    }
    let ref mut fresh14 = (*((*map).buckets).offset(pos as isize)).ptr;
    *fresh14 = 0 as *mut libc::c_void;
    let ref mut fresh15 = (*map).count;
    *fresh15 = (*fresh15).wrapping_sub(1);
    let mut ib_index: size_t = pos >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        pos & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    let ref mut fresh16 = *((*map).item_bitmap).offset(ib_index as isize);
    *fresh16 &= !((1 as libc::c_int as bitmap_t) << ib_offset);
}
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn ponyint_hashmap_remove(
    mut map: *mut hashmap_t,
    mut entry: *mut libc::c_void,
    mut hash: size_t,
    mut cmp: cmp_fn,
) -> *mut libc::c_void {
    if (*map).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut pos: size_t = 0;
    let mut probe_length: size_t = 0 as libc::c_int as size_t;
    let mut oi_probe_length: size_t = 0 as libc::c_int as size_t;
    let mut elem: *mut libc::c_void = search(
        map,
        &mut pos,
        entry,
        hash,
        cmp,
        &mut probe_length,
        &mut oi_probe_length,
    );
    if !elem.is_null() {
        shift_delete(map, pos);
    }
    return elem;
}
#[no_mangle]
#[c2rust::src_loc = "442:1"]
pub unsafe extern "C" fn ponyint_hashmap_removeindex(mut map: *mut hashmap_t, mut index: size_t) {
    if (*map).size <= index {
        return;
    }
    let mut ib_index: size_t = index >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        index & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    if *((*map).item_bitmap).offset(ib_index as isize) & (1 as libc::c_int as bitmap_t) << ib_offset
        != 0 as libc::c_int as libc::c_ulong
    {
        shift_delete(map, index);
    }
}
#[no_mangle]
#[c2rust::src_loc = "455:1"]
pub unsafe extern "C" fn ponyint_hashmap_next(
    mut i: *mut size_t,
    mut count: size_t,
    mut item_bitmap: *mut bitmap_t,
    mut size: size_t,
    mut buckets: *mut hashmap_entry_t,
) -> *mut libc::c_void {
    if count == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut index: size_t = (*i).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut ib_index: size_t = index >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        index & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    let mut ffs_offset: size_t = 0 as libc::c_int as size_t;
    let mut ib: bitmap_t = *item_bitmap.offset(ib_index as isize) >> ib_offset;
    while index < size {
        ffs_offset = __pony_ffszu(ib) as size_t;
        if ffs_offset == 0 as libc::c_int as libc::c_ulong {
            index = (index as libc::c_ulong).wrapping_add(
                (((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong).wrapping_sub(ib_offset),
            ) as size_t as size_t;
            ib_index = ib_index.wrapping_add(1);
            ib_offset = 0 as libc::c_int as size_t;
            ib = *item_bitmap.offset(ib_index as isize);
        } else {
            index = (index as libc::c_ulong)
                .wrapping_add(ffs_offset.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            if !((*buckets.offset(index as isize)).ptr).is_null() {
            } else {
                ponyint_assert_fail(
                    b"buckets[index].ptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.c\0"
                        as *const u8 as *const libc::c_char,
                    488 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"ponyint_hashmap_next\0",
                    ))
                    .as_ptr(),
                );
            };
            *i = index;
            return (*buckets.offset(index as isize)).ptr;
        }
    }
    *i = size;
    return 0 as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn ponyint_hashmap_size(mut map: *mut hashmap_t) -> size_t {
    return (*map).count;
}
#[no_mangle]
#[c2rust::src_loc = "506:1"]
pub unsafe extern "C" fn ponyint_hashmap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    return (*map).count as libc::c_double / (*map).size as libc::c_double;
}
#[no_mangle]
#[c2rust::src_loc = "513:1"]
pub unsafe extern "C" fn ponyint_hashmap_mem_size(mut map: *mut hashmap_t) -> size_t {
    let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
        (if (*map).size
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    return bitmap_size
        .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
        .wrapping_add(
            ((*map).size).wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
        );
}
#[no_mangle]
#[c2rust::src_loc = "522:1"]
pub unsafe extern "C" fn ponyint_hashmap_alloc_size(mut map: *mut hashmap_t) -> size_t {
    let mut size: size_t = ponyint_hashmap_mem_size(map);
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    return ponyint_pool_used_size(size);
}
#[no_mangle]
#[c2rust::src_loc = "531:1"]
pub unsafe extern "C" fn ponyint_hashmap_clearindex(mut map: *mut hashmap_t, mut index: size_t) {
    if (*map).size <= index {
        return;
    }
    let mut ib_index: size_t = index >> 6 as libc::c_int;
    let mut ib_offset: size_t =
        index & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong;
    if *((*map).item_bitmap).offset(ib_index as isize) & (1 as libc::c_int as bitmap_t) << ib_offset
        == 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    let ref mut fresh17 = (*((*map).buckets).offset(index as isize)).ptr;
    *fresh17 = 0 as *mut libc::c_void;
    let ref mut fresh18 = (*map).count;
    *fresh18 = (*fresh18).wrapping_sub(1);
    let ref mut fresh19 = *((*map).item_bitmap).offset(ib_index as isize);
    *fresh19 &= !((1 as libc::c_int as bitmap_t) << ib_offset);
}
#[no_mangle]
#[c2rust::src_loc = "550:1"]
pub unsafe extern "C" fn ponyint_hashmap_optimize(mut map: *mut hashmap_t, mut cmp: cmp_fn) {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut num_iters: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        count = 0 as libc::c_int as size_t;
        i = -(1 as libc::c_int) as size_t;
        loop {
            elem = ponyint_hashmap_next(
                &mut i,
                (*map).count,
                (*map).item_bitmap,
                (*map).size,
                (*map).buckets,
            );
            if elem.is_null() {
                break;
            }
            count = (count as libc::c_ulong).wrapping_add(optimize_item(map, cmp, i)) as size_t
                as size_t;
        }
        num_iters = num_iters.wrapping_add(1);
        if !(count > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "570:1"]
pub unsafe extern "C" fn ponyint_hashmap_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut elem_type: *const pony_type_t,
) {
    let mut map: *mut hashmap_t = object as *mut hashmap_t;
    let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
        (if (*map).size
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    pony_serialise_reserve(
        ctx,
        (*map).item_bitmap as *mut libc::c_void,
        bitmap_size
            .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
            .wrapping_add(
                ((*map).size)
                    .wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
            ),
    );
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        elem = ponyint_hashmap_next(
            &mut i,
            (*map).count,
            (*map).item_bitmap,
            (*map).size,
            (*map).buckets,
        );
        if elem.is_null() {
            break;
        }
        pony_traceknown(ctx, elem, elem_type, PONY_TRACE_MUTABLE as libc::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "590:1"]
pub unsafe extern "C" fn ponyint_hashmap_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
) {
    let mut map: *mut hashmap_t = object as *mut hashmap_t;
    let mut dst: *mut hashmap_t = (buf as uintptr_t).wrapping_add(offset) as *mut hashmap_t;
    let mut bitmap_offset: uintptr_t =
        pony_serialise_offset(ctx, (*map).item_bitmap as *mut libc::c_void);
    (*dst).count = (*map).count;
    (*dst).size = (*map).size;
    let ref mut fresh20 = (*dst).item_bitmap;
    *fresh20 = bitmap_offset as *mut bitmap_t;
    let ref mut fresh21 = (*dst).buckets;
    *fresh21 = 0 as *mut hashmap_entry_t;
    let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
        (if (*map).size
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    memcpy(
        (buf as uintptr_t).wrapping_add(bitmap_offset) as *mut libc::c_void,
        (*map).item_bitmap as *const libc::c_void,
        bitmap_size
            .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
            .wrapping_add(
                ((*map).size)
                    .wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
            ),
    );
    let mut dst_buckets: *mut hashmap_entry_t = (buf as uintptr_t)
        .wrapping_add(bitmap_offset)
        .wrapping_add(bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong))
        as *mut hashmap_entry_t;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        elem = ponyint_hashmap_next(
            &mut i,
            (*map).count,
            (*map).item_bitmap,
            (*map).size,
            (*map).buckets,
        );
        if elem.is_null() {
            break;
        }
        let ref mut fresh22 = (*dst_buckets.offset(i as isize)).ptr;
        *fresh22 = pony_serialise_offset(ctx, elem) as *mut libc::c_void;
        (*dst_buckets.offset(i as isize)).hash = (*((*map).buckets).offset(i as isize)).hash;
    }
}
#[no_mangle]
#[c2rust::src_loc = "622:1"]
pub unsafe extern "C" fn ponyint_hashmap_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut elem_type: *const pony_type_t,
) {
    let mut map: *mut hashmap_t = object as *mut hashmap_t;
    let mut bitmap_size: size_t = ((*map).size >> 6 as libc::c_int).wrapping_add(
        (if (*map).size
            & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_ulong,
    );
    let ref mut fresh23 = (*map).item_bitmap;
    *fresh23 = pony_deserialise_block(
        ctx,
        (*map).item_bitmap as uintptr_t,
        bitmap_size
            .wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong)
            .wrapping_add(
                ((*map).size)
                    .wrapping_mul(::core::mem::size_of::<hashmap_entry_t>() as libc::c_ulong),
            ),
    ) as *mut bitmap_t;
    let ref mut fresh24 = (*map).buckets;
    *fresh24 = ((*map).item_bitmap as *mut libc::c_char).offset(
        bitmap_size.wrapping_mul(::core::mem::size_of::<bitmap_t>() as libc::c_ulong) as isize,
    ) as *mut hashmap_entry_t;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        elem = ponyint_hashmap_next(
            &mut i,
            (*map).count,
            (*map).item_bitmap,
            (*map).size,
            (*map).buckets,
        );
        if elem.is_null() {
            break;
        }
        let ref mut fresh25 = (*((*map).buckets).offset(i as isize)).ptr;
        *fresh25 = pony_deserialise_offset(ctx, elem_type, elem as uintptr_t);
    }
}