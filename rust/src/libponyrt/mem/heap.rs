use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:1"]
pub mod _pthread_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:8"]
    pub struct __darwin_pthread_handler_rec {
        pub __routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        pub __arg: *mut libc::c_void,
        pub __next: *mut __darwin_pthread_handler_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct _opaque_pthread_cond_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 40],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "103:8"]
    pub struct _opaque_pthread_t {
        pub __sig: libc::c_long,
        pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
        pub __opaque: [libc::c_char; 8176],
    }
    #[c2rust::src_loc = "110:1"]
    pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_pthread_t = *mut _opaque_pthread_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:1"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:1"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:4"]
pub mod actor_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct pony_actor_t {
        pub type_0: *const pony_type_t,
        pub q: messageq_t,
        pub sync_flags: u8,
        pub cycle_detector_critical: u8,
        pub heap: heap_t,
        pub muted: usize,
        pub internal_flags: u8,
        pub gc: gc_t,
    }
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:4"]
pub mod gc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:16"]
    pub struct gc_t {
        pub mark: u32,
        pub rc_mark: u32,
        pub rc: usize,
        pub local: objectmap_t,
        pub foreign: actormap_t,
        pub delta: *mut deltamap_t,
    }
    use super::actormap_h::actormap_t;
    use super::delta_h::deltamap_t;
    use super::objectmap_h::objectmap_t;

    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/delta.h:4"]
pub mod delta_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:35"]
    pub struct deltamap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:4"]
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:4"]
pub mod actormap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:35"]
    pub struct actormap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:4"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:1"]
pub mod heap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:16"]
    pub struct heap_t {
        pub small_free: [*mut chunk_t; 5],
        pub small_full: [*mut chunk_t; 5],
        pub large: *mut chunk_t,
        pub used: usize,
        pub next_gc: usize,
    }
    #[c2rust::src_loc = "30:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "33:3"]
    pub const TRACK_ALL_FINALISERS: C2RustUnnamed_0 = 4294967295;
    #[c2rust::src_loc = "32:3"]
    pub const TRACK_NO_FINALISERS: C2RustUnnamed_0 = 0;
    use super::chunk_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:4"]
pub use crate::libponyrt::actor::messageq::messageq_h;
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
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
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

    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:4"]
pub mod scheduler_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:16"]
    pub struct pony_ctx_t {
        pub scheduler: *mut scheduler_t,
        pub current: *mut pony_actor_t,
        pub trace_object: trace_object_fn,
        pub trace_actor: trace_actor_fn,
        pub stack: *mut gcstack_t,
        pub acquire: actormap_t,
        pub serialise_buffer: *mut libc::c_void,
        pub serialise_size: usize,
        pub serialise: ponyint_serialise_t,
        pub serialise_alloc: serialise_alloc_fn,
        pub serialise_alloc_final: serialise_alloc_fn,
        pub serialise_throw: serialise_throw_fn,
    }
    #[c2rust::src_loc = "33:1"]
    pub type trace_actor_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> ()>;
    #[c2rust::src_loc = "30:1"]
    pub type trace_object_fn = Option<
        unsafe extern "C" fn(
            *mut pony_ctx_t,
            *mut libc::c_void,
            *const pony_type_t,
            libc::c_int,
        ) -> (),
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:8"]
    pub struct scheduler_t {
        pub tid: pthread_t,
        pub index: i32,
        pub cpu: u32,
        pub node: u32,
        pub terminate: bool,
        pub asio_stoppable: bool,
        pub asio_noisy: bool,
        pub sleep_object: *mut pthread_cond_t,
        pub last_victim: *mut scheduler_t,
        pub ctx: pony_ctx_t,
        pub block_count: u32,
        pub ack_token: i32,
        pub ack_count: u32,
        pub mute_mapping: mutemap_t,
        pub q: mpmcq_t,
        pub mq: messageq_t,
    }
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_t_h::pthread_t;
    use super::actor_h::pony_actor_t;
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::pony_type_t;
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:4"]
pub mod serialise_h {
    #[c2rust::src_loc = "18:1"]
    pub type serialise_throw_fn = Option<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "16:1"]
    pub type serialise_alloc_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, usize) -> *mut libc::c_void>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:36"]
    pub struct ponyint_serialise_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
    use super::scheduler_h::pony_ctx_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:4"]
pub mod mpmcq_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct mpmcq_t {
        pub head: *mut mpmcq_node_t,
        pub tail: aba_protected_mpmcq_node_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub union aba_protected_mpmcq_node_t {
        pub c2rust_unnamed: C2RustUnnamed,
        pub raw: i128,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub struct C2RustUnnamed {
        pub object: *mut mpmcq_node_t,
        pub counter: libc::uintptr_t,
    }

    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:4"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/platform.h:1"]
pub mod platform_h {
    #[inline]
    #[c2rust::src_loc = "250:1"]
    pub unsafe extern "C" fn __pony_popcount(mut x: u32) -> u32 {
        x.count_ones() as i32 as u32
    }
    #[inline]
    #[c2rust::src_loc = "254:1"]
    pub unsafe extern "C" fn __pony_ffs(mut x: u32) -> u32 {
        (if x as libc::c_int == 0 {
            0
        } else {
            (x as libc::c_int).trailing_zeros() as i32 + 1
        }) as u32
    }
    #[inline]
    #[c2rust::src_loc = "258:1"]
    pub unsafe extern "C" fn __pony_ffsll(mut x: u64) -> u32 {
        (if x as libc::c_longlong == 0 {
            0
        } else {
            (x as libc::c_longlong).trailing_zeros() as i32 + 1
        }) as u32
    }
    #[inline]
    #[c2rust::src_loc = "262:1"]
    pub unsafe extern "C" fn __pony_ctz(mut x: u32) -> u32 {
        x.trailing_zeros() as i32 as u32
    }
    #[inline]
    #[c2rust::src_loc = "266:1"]
    pub unsafe extern "C" fn __pony_clz(mut x: u32) -> u32 {
        x.leading_zeros() as i32 as u32
    }
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn __pony_clzll(mut x: u64) -> u32 {
        x.leading_zeros() as i32 as u32
    }
    #[inline]
    #[c2rust::src_loc = "319:1"]
    pub unsafe extern "C" fn __pony_ffszu(mut x: usize) -> u32 {
        __pony_ffsll(x as u64)
    }
    #[inline]
    #[c2rust::src_loc = "327:1"]
    pub unsafe extern "C" fn __pony_clzzu(mut x: usize) -> u32 {
        __pony_clzll(x as u64)
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:1"]
pub mod pool_h {

    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_pool_adjust_size(size: usize) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pagemap.h:2"]
pub mod pagemap_h {
    use super::chunk_t;

    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn ponyint_pagemap_get(addr: *const libc::c_void) -> *mut chunk_t;
        #[c2rust::src_loc = "12:1"]
        pub fn ponyint_pagemap_set(addr: *const libc::c_void, chunk: *mut chunk_t);
        #[c2rust::src_loc = "14:1"]
        pub fn ponyint_pagemap_set_bulk(
            addr: *const libc::c_void,
            chunk: *mut chunk_t,
            size: usize,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:5"]
pub mod ponyassert_h {

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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:6"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:9"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
    }
}
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::pony_actor_t;
pub use self::actormap_h::actormap_t;
pub use self::delta_h::deltamap_t;
use self::dtrace_h::macro__DTRACE;
pub use self::gc_h::{gc_t, gcstack_t};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{heap_t, C2RustUnnamed_0, TRACK_ALL_FINALISERS, TRACK_NO_FINALISERS};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
use self::pagemap_h::{ponyint_pagemap_get, ponyint_pagemap_set, ponyint_pagemap_set_bulk};
pub use self::platform_h::{
    __pony_clz, __pony_clzll, __pony_clzzu, __pony_ctz, __pony_ffs, __pony_ffsll, __pony_ffszu,
    __pony_popcount,
};
pub use self::pony_h::{
    _pony_type_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn, pony_dispatch_fn,
    pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_adjust_size, ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free,
    ponyint_pool_free_size,
};
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
use self::string_h::{memcpy, memset};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "11:16"]
pub struct chunk_t {
    pub actor: *mut pony_actor_t,
    pub m: *mut libc::c_char,
    pub size: usize,
    pub slots: u32,
    pub shallow: u32,
    pub finalisers: u32,
    pub next: *mut chunk_t,
}
#[c2rust::src_loc = "37:1"]
pub type block_t = [libc::c_char; 1024];
#[c2rust::src_loc = "34:3"]
pub const FORCE_ALL_FINALISERS: C2RustUnnamed_2 = 4294967295;
#[c2rust::src_loc = "33:3"]
pub const FORCE_NO_FINALISERS: C2RustUnnamed_2 = 0;
#[c2rust::src_loc = "38:1"]
pub type chunk_fn = Option<unsafe extern "C" fn(*mut chunk_t, u32) -> ()>;
#[c2rust::src_loc = "28:3"]
pub const CHUNK_NEEDS_TO_BE_CLEARED: C2RustUnnamed_1 = 4294967295;
#[c2rust::src_loc = "26:1"]
pub type C2RustUnnamed_1 = libc::c_uint;
#[c2rust::src_loc = "31:1"]
pub type C2RustUnnamed_2 = libc::c_uint;
#[c2rust::src_loc = "49:23"]
static mut sizeclass_empty: [u32; 5] = [
    0xffffffff as libc::c_uint,
    0x55555555 as libc::c_int as u32,
    0x11111111 as libc::c_int as u32,
    0x1010101 as libc::c_int as u32,
    0x10001 as libc::c_int as u32,
];
#[c2rust::src_loc = "58:23"]
static mut sizeclass_init: [u32; 5] = [
    0xfffffffe as libc::c_uint,
    0x55555554 as libc::c_int as u32,
    0x11111110 as libc::c_int as u32,
    0x1010100 as libc::c_int as u32,
    0x10000 as libc::c_int as u32,
];
#[c2rust::src_loc = "74:22"]
static mut sizeclass_table: [u8; 16] = [
    0 as libc::c_int as u8,
    1 as libc::c_int as u8,
    2 as libc::c_int as u8,
    2 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    3 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
    4 as libc::c_int as u8,
];
#[c2rust::src_loc = "80:15"]
static mut heap_initialgc: usize = ((1 as libc::c_int) << 14 as libc::c_int) as usize;
#[c2rust::src_loc = "81:15"]
static mut heap_nextgc_factor: libc::c_double = 2.0f64;
#[c2rust::src_loc = "131:1"]
unsafe extern "C" fn large_pagemap(
    mut m: *mut libc::c_char,
    mut size: usize,
    mut chunk: *mut chunk_t,
) {
    ponyint_pagemap_set_bulk(m as *const libc::c_void, chunk, size);
}
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn clear_chunk(mut chunk: *mut chunk_t, mut mark: u32) {
    (*chunk).slots = mark;
    (*chunk).shallow = mark;
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn maybe_clear_chunk(mut chunk: *mut chunk_t) {
    if (*chunk).size != 0 && (*chunk).shallow == CHUNK_NEEDS_TO_BE_CLEARED as libc::c_uint {
        if (*chunk).size
            >= ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .try_into()
                .unwrap()
        {
            (*chunk).slots = 1 as libc::c_int as u32;
            (*chunk).shallow = 1 as libc::c_int as u32;
        } else {
            (*chunk).slots = sizeclass_empty[(*chunk).size as usize];
            (*chunk).shallow = sizeclass_empty[(*chunk).size as usize];
        }
    }
}
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn final_small(mut chunk: *mut chunk_t, mut force_finalisers_mask: u32) {
    if force_finalisers_mask == FORCE_NO_FINALISERS as libc::c_int as libc::c_uint
        || force_finalisers_mask == FORCE_ALL_FINALISERS as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"force_finalisers_mask == FORCE_NO_FINALISERS || force_finalisers_mask == FORCE_ALL_FINALISERS\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0"
                as *const u8 as *const libc::c_char,
            174 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"final_small\0"))
                .as_ptr(),
        );
    };
    let mut finalisers: u32 = (*chunk).finalisers & ((*chunk).slots | force_finalisers_mask);
    if finalisers == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*chunk).finalisers = (*chunk).finalisers & !((*chunk).slots | force_finalisers_mask);
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bit: u64 = 0 as libc::c_int as u64;
    while finalisers != 0 as libc::c_int as libc::c_uint {
        bit = __pony_ctz(finalisers) as u64;
        p = ((*chunk).m).offset((bit << 5 as libc::c_int) as isize) as *mut libc::c_void;
        if ((**(p as *mut *const pony_type_t)).final_0).is_some() {
        } else {
            ponyint_assert_fail(
                b"(*(pony_type_t**)p)->final != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0" as *const u8
                    as *const libc::c_char,
                196 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"final_small\0"))
                    .as_ptr(),
            );
        };
        ((**(p as *mut *const pony_type_t)).final_0).expect("non-null function pointer")(p);
        finalisers &= finalisers.wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
}
#[c2rust::src_loc = "204:1"]
unsafe extern "C" fn final_large(mut chunk: *mut chunk_t, mut _mark: u32) {
    if (*chunk).finalisers == 1 as libc::c_int as libc::c_uint {
        if ((**((*chunk).m as *mut *const pony_type_t)).final_0).is_some() {
        } else {
            ponyint_assert_fail(
                b"(*(pony_type_t**)chunk->m)->final != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0" as *const u8
                    as *const libc::c_char,
                209 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"final_large\0"))
                    .as_ptr(),
            );
        };
        ((**((*chunk).m as *mut *const pony_type_t)).final_0).expect("non-null function pointer")(
            (*chunk).m as *mut libc::c_void,
        );
        (*chunk).finalisers = 0 as libc::c_int as u32;
    }
}
#[c2rust::src_loc = "216:1"]
unsafe extern "C" fn destroy_small(mut chunk: *mut chunk_t, mut _mark: u32) {
    final_small(chunk, FORCE_ALL_FINALISERS as libc::c_uint);
    ponyint_pagemap_set((*chunk).m as *const libc::c_void, 0 as *mut chunk_t);
    ponyint_pool_free(5 as libc::c_int as usize, (*chunk).m as *mut libc::c_void);
    ponyint_pool_free(1 as libc::c_int as usize, chunk as *mut libc::c_void);
}
#[c2rust::src_loc = "228:1"]
unsafe extern "C" fn destroy_large(mut chunk: *mut chunk_t, mut mark: u32) {
    final_large(chunk, mark);
    large_pagemap((*chunk).m, (*chunk).size, 0 as *mut chunk_t);
    if !((*chunk).m).is_null() {
        ponyint_pool_free_size((*chunk).size, (*chunk).m as *mut libc::c_void);
    }
    ponyint_pool_free(1 as libc::c_int as usize, chunk as *mut libc::c_void);
}
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn sweep_small(
    mut chunk: *mut chunk_t,
    mut avail: *mut *mut chunk_t,
    mut full: *mut *mut chunk_t,
    mut empty: u32,
    mut size: usize,
) -> usize {
    let mut used: usize = 0;
    let mut next: *mut chunk_t = 0 as *mut chunk_t;
    while !chunk.is_null() {
        next = (*chunk).next;
        maybe_clear_chunk(chunk);
        let ref mut fresh0 = (*chunk).slots;
        *fresh0 &= (*chunk).shallow;
        (*chunk).shallow = CHUNK_NEEDS_TO_BE_CLEARED as libc::c_uint;
        if (*chunk).slots == 0 as libc::c_int as libc::c_uint {
            used = (used as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<block_t>().try_into().unwrap())
                as usize as usize;
            let ref mut fresh1 = (*chunk).next;
            *fresh1 = *full;
            *full = chunk;
        } else if (*chunk).slots == empty {
            destroy_small(chunk, 0 as libc::c_int as u32);
        } else {
            used = (used as libc::c_ulong).wrapping_add(
                (::core::mem::size_of::<block_t>())
                    .wrapping_sub(
                        (__pony_popcount((*chunk).slots) as libc::c_ulong)
                            .wrapping_mul(size.try_into().unwrap())
                            .try_into()
                            .unwrap(),
                    )
                    .try_into()
                    .unwrap(),
            ) as usize as usize;
            final_small(chunk, FORCE_NO_FINALISERS as libc::c_int as u32);
            let ref mut fresh2 = (*chunk).next;
            *fresh2 = *avail;
            *avail = chunk;
        }
        chunk = next;
    }
    used
}
#[c2rust::src_loc = "322:1"]
unsafe extern "C" fn sweep_large(mut chunk: *mut chunk_t, mut used: *mut usize) -> *mut chunk_t {
    let mut list: *mut chunk_t = 0 as *mut chunk_t;
    let mut next: *mut chunk_t = 0 as *mut chunk_t;
    while !chunk.is_null() {
        next = (*chunk).next;
        maybe_clear_chunk(chunk);
        let ref mut fresh3 = (*chunk).slots;
        *fresh3 &= (*chunk).shallow;
        (*chunk).shallow = CHUNK_NEEDS_TO_BE_CLEARED as libc::c_uint;
        if (*chunk).slots == 0 as libc::c_int as libc::c_uint {
            let ref mut fresh4 = (*chunk).next;
            *fresh4 = list;
            list = chunk;
            *used = (*used as libc::c_ulong).wrapping_add((*chunk).size.try_into().unwrap())
                as usize as usize;
        } else {
            destroy_large(chunk, 0 as libc::c_int as u32);
        }
        chunk = next;
    }
    list
}
#[c2rust::src_loc = "370:1"]
unsafe extern "C" fn chunk_list(mut f: chunk_fn, mut current: *mut chunk_t, mut mark: u32) {
    let mut next: *mut chunk_t = 0 as *mut chunk_t;
    while !current.is_null() {
        next = (*current).next;
        f.expect("non-null function pointer")(current, mark);
        current = next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "382:1"]
pub unsafe extern "C" fn ponyint_heap_index(mut size: usize) -> u32 {
    return sizeclass_table[(size.wrapping_sub(1) >> 5 as libc::c_int) as usize] as u32;
}
#[no_mangle]
#[c2rust::src_loc = "389:1"]
pub unsafe extern "C" fn ponyint_heap_setinitialgc(mut size: usize) {
    heap_initialgc = (1 as libc::c_int as usize) << size;
}
#[no_mangle]
#[c2rust::src_loc = "394:1"]
pub unsafe extern "C" fn ponyint_heap_setnextgcfactor(mut factor: libc::c_double) {
    if factor < 1.0f64 {
        factor = 1.0f64;
    }
    macro__DTRACE(
        b"GC_THRESHOLD\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"factor\0" as *const u8 as *const libc::c_char,
    );
    heap_nextgc_factor = factor;
}
#[no_mangle]
#[c2rust::src_loc = "403:1"]
pub unsafe extern "C" fn ponyint_heap_init(mut heap: *mut heap_t) {
    memset(
        heap as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<heap_t>().try_into().unwrap(),
    );
    (*heap).next_gc = heap_initialgc;
}
#[no_mangle]
#[c2rust::src_loc = "409:1"]
pub unsafe extern "C" fn ponyint_heap_destroy(mut heap: *mut heap_t) {
    chunk_list(
        Some(destroy_large as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
        (*heap).large,
        0 as libc::c_int as u32,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int {
        chunk_list(
            Some(destroy_small as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
            (*heap).small_free[i as usize],
            0 as libc::c_int as u32,
        );
        chunk_list(
            Some(destroy_small as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
            (*heap).small_full[i as usize],
            0 as libc::c_int as u32,
        );
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "420:1"]
pub unsafe extern "C" fn ponyint_heap_final(mut heap: *mut heap_t) {
    chunk_list(
        Some(final_large as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
        (*heap).large,
        0 as libc::c_int as u32,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int {
        chunk_list(
            Some(final_small as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
            (*heap).small_free[i as usize],
            FORCE_ALL_FINALISERS as libc::c_uint,
        );
        chunk_list(
            Some(final_small as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
            (*heap).small_full[i as usize],
            FORCE_ALL_FINALISERS as libc::c_uint,
        );
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "431:1"]
pub unsafe extern "C" fn ponyint_heap_alloc(
    mut actor: *mut pony_actor_t,
    mut heap: *mut heap_t,
    mut size: usize,
    mut track_finalisers_mask: u32,
) -> *mut libc::c_void {
    if track_finalisers_mask == TRACK_NO_FINALISERS as libc::c_int as libc::c_uint
        || track_finalisers_mask == TRACK_ALL_FINALISERS as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"track_finalisers_mask == TRACK_NO_FINALISERS || track_finalisers_mask == TRACK_ALL_FINALISERS\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0"
                as *const u8 as *const libc::c_char,
            439 as libc::c_int as usize,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"ponyint_heap_alloc\0"))
                .as_ptr(),
        );
    };
    if size == 0 {
        return 0 as *mut libc::c_void;
    } else if size
        <= (((1 as libc::c_int) << 10 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        return ponyint_heap_alloc_small(
            actor,
            heap,
            ponyint_heap_index(size),
            track_finalisers_mask,
        );
    } else {
        return ponyint_heap_alloc_large(actor, heap, size, track_finalisers_mask);
    };
}
#[no_mangle]
#[c2rust::src_loc = "452:1"]
pub unsafe extern "C" fn ponyint_heap_alloc_small(
    mut actor: *mut pony_actor_t,
    mut heap: *mut heap_t,
    mut sizeclass: u32,
    mut track_finalisers_mask: u32,
) -> *mut libc::c_void {
    if track_finalisers_mask == TRACK_NO_FINALISERS as libc::c_int as libc::c_uint
        || track_finalisers_mask == TRACK_ALL_FINALISERS as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"track_finalisers_mask == TRACK_NO_FINALISERS || track_finalisers_mask == TRACK_ALL_FINALISERS\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0"
                as *const u8 as *const libc::c_char,
            460 as libc::c_int as usize,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"ponyint_heap_alloc_small\0"))
                .as_ptr(),
        );
    };
    let mut chunk: *mut chunk_t = (*heap).small_free[sizeclass as usize];
    let mut m: *mut libc::c_void = 0 as *mut libc::c_void;
    if !chunk.is_null() {
        let mut slots: u32 = (*chunk).slots;
        let mut bit: u32 = __pony_ctz(slots);
        slots &= !((1 as libc::c_int as u32) << bit);
        m = ((*chunk).m).offset((bit << 5 as libc::c_int) as isize) as *mut libc::c_void;
        (*chunk).slots = slots;
        let ref mut fresh5 = (*chunk).finalisers;
        *fresh5 |= track_finalisers_mask & (1 as libc::c_int as u32) << bit;
        if slots == 0 as libc::c_int as libc::c_uint {
            let ref mut fresh6 = (*heap).small_free[sizeclass as usize];
            *fresh6 = (*chunk).next;
            let ref mut fresh7 = (*chunk).next;
            *fresh7 = (*heap).small_full[sizeclass as usize];
            let ref mut fresh8 = (*heap).small_full[sizeclass as usize];
            *fresh8 = chunk;
        }
    } else {
        let mut n: *mut chunk_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut chunk_t;
        let ref mut fresh9 = (*n).actor;
        *fresh9 = actor;
        let ref mut fresh10 = (*n).m;
        *fresh10 =
            ponyint_pool_alloc(5 as libc::c_int as usize) as *mut block_t as *mut libc::c_char;
        (*n).size = sizeclass as usize;
        (*n).finalisers = track_finalisers_mask & 1 as libc::c_int as libc::c_uint;
        (*n).slots = sizeclass_init[sizeclass as usize];
        let ref mut fresh11 = (*n).next;
        *fresh11 = 0 as *mut chunk_t;
        (*n).shallow = CHUNK_NEEDS_TO_BE_CLEARED as libc::c_uint;
        ponyint_pagemap_set((*n).m as *const libc::c_void, n);
        let ref mut fresh12 = (*heap).small_free[sizeclass as usize];
        *fresh12 = n;
        chunk = n;
        m = (*chunk).m as *mut libc::c_void;
    }
    let ref mut fresh13 = (*heap).used;
    *fresh13 = (*fresh13 as libc::c_ulong)
        .wrapping_add((((1 as libc::c_int) << 5 as libc::c_int) << sizeclass) as libc::c_ulong)
        as usize as usize;
    return m;
}
#[no_mangle]
#[c2rust::src_loc = "527:1"]
pub unsafe extern "C" fn ponyint_heap_alloc_large(
    mut actor: *mut pony_actor_t,
    mut heap: *mut heap_t,
    mut size: usize,
    mut track_finalisers_mask: u32,
) -> *mut libc::c_void {
    if track_finalisers_mask == TRACK_NO_FINALISERS as libc::c_int as libc::c_uint
        || track_finalisers_mask == TRACK_ALL_FINALISERS as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"track_finalisers_mask == TRACK_NO_FINALISERS || track_finalisers_mask == TRACK_ALL_FINALISERS\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0"
                as *const u8 as *const libc::c_char,
            535 as libc::c_int as usize,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"ponyint_heap_alloc_large\0"))
                .as_ptr(),
        );
    };
    size = ponyint_pool_adjust_size(size);
    let mut chunk: *mut chunk_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut chunk_t;
    let ref mut fresh14 = (*chunk).actor;
    *fresh14 = actor;
    (*chunk).size = size;
    let ref mut fresh15 = (*chunk).m;
    *fresh15 = ponyint_pool_alloc_size(size) as *mut libc::c_char;
    (*chunk).slots = 0 as libc::c_int as u32;
    (*chunk).shallow = CHUNK_NEEDS_TO_BE_CLEARED as libc::c_uint;
    (*chunk).finalisers = track_finalisers_mask & 1 as libc::c_int as libc::c_uint;
    large_pagemap((*chunk).m, size, chunk);
    let ref mut fresh16 = (*chunk).next;
    *fresh16 = (*heap).large;
    let ref mut fresh17 = (*heap).large;
    *fresh17 = chunk;
    let ref mut fresh18 = (*heap).used;
    *fresh18 = (*fresh18 as libc::c_ulong).wrapping_add((*chunk).size.try_into().unwrap()) as usize
        as usize;
    return (*chunk).m as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "571:1"]
pub unsafe extern "C" fn ponyint_heap_realloc(
    mut actor: *mut pony_actor_t,
    mut heap: *mut heap_t,
    mut p: *mut libc::c_void,
    mut size: usize,
    mut copy: usize,
) -> *mut libc::c_void {
    if p.is_null() {
        return ponyint_heap_alloc(actor, heap, size, TRACK_NO_FINALISERS as libc::c_int as u32);
    }
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if !chunk.is_null() {
    } else {
        ponyint_assert_fail(
            b"chunk != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0" as *const u8
                as *const libc::c_char,
            585 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ponyint_heap_realloc\0"))
                .as_ptr(),
        );
    };
    let mut oldsize: usize = 0;
    if (*chunk).size
        < ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        let mut ext: *mut libc::c_void = (p as libc::uintptr_t
            & !((((1) << 5) << (*chunk).size) - 1) as usize)
            as *mut libc::c_void;
        oldsize = ((((1) << 5) << (*chunk).size) as usize).wrapping_sub(
            (p as libc::uintptr_t)
                .wrapping_sub(ext as libc::uintptr_t)
                .try_into()
                .unwrap(),
        );
    } else {
        oldsize = ((*chunk).size)
            .wrapping_sub((p as libc::uintptr_t).wrapping_sub((*chunk).m as libc::uintptr_t));
    }
    if copy <= size {
    } else {
        ponyint_assert_fail(
            b"copy <= size\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0" as *const u8
                as *const libc::c_char,
            601 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ponyint_heap_realloc\0"))
                .as_ptr(),
        );
    };
    if copy > size {
        copy = size;
    }
    if oldsize > copy {
        oldsize = copy;
    }
    let mut q: *mut libc::c_void =
        ponyint_heap_alloc(actor, heap, size, TRACK_NO_FINALISERS as libc::c_int as u32);
    memcpy(q, p, oldsize.try_into().unwrap());
    q
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn ponyint_heap_used(mut heap: *mut heap_t, mut size: usize) {
    let ref mut fresh19 = (*heap).used;
    *fresh19 = (*fresh19 as libc::c_ulong).wrapping_add(size.try_into().unwrap()) as usize as usize;
}
#[no_mangle]
#[c2rust::src_loc = "620:1"]
pub unsafe extern "C" fn ponyint_heap_startgc(mut heap: *mut heap_t) -> bool {
    if (*heap).used <= (*heap).next_gc {
        return 0 as libc::c_int != 0;
    }
    let mut mark: u32 = sizeclass_empty[0 as libc::c_int as usize];
    if mark == 0xffffffff as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"mark == 0xFFFFFFFF\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.c\0" as *const u8
                as *const libc::c_char,
            633 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ponyint_heap_startgc\0"))
                .as_ptr(),
        );
    };
    chunk_list(
        Some(clear_chunk as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
        (*heap).small_free[0 as libc::c_int as usize],
        mark,
    );
    chunk_list(
        Some(clear_chunk as unsafe extern "C" fn(*mut chunk_t, u32) -> ()),
        (*heap).small_full[0 as libc::c_int as usize],
        mark,
    );
    (*heap).used = 0 as libc::c_int as usize;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn ponyint_heap_mark(
    mut chunk: *mut chunk_t,
    mut p: *mut libc::c_void,
) -> bool {
    let mut marked: bool = false;
    maybe_clear_chunk(chunk);
    if (*chunk).size
        >= ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        marked = (*chunk).slots == 0 as libc::c_int as libc::c_uint;
        if p == (*chunk).m as *mut libc::c_void {
            (*chunk).slots = 0 as libc::c_int as u32;
        } else {
            (*chunk).shallow = 0 as libc::c_int as u32;
        }
    } else {
        let mut ext: *mut libc::c_void = (p as libc::uintptr_t
            & !((((1) << 5) << (*chunk).size) - 1) as usize)
            as *mut libc::c_void;
        let mut slot: u32 = ((1 as libc::c_int)
            << ((ext as *mut libc::c_char).offset_from((*chunk).m) as libc::c_long
                as libc::uintptr_t
                >> 5 as libc::c_int)) as u32;
        marked = (*chunk).slots & slot == 0 as libc::c_int as libc::c_uint;
        if p == ext {
            let ref mut fresh20 = (*chunk).slots;
            *fresh20 &= !slot;
        } else {
            let ref mut fresh21 = (*chunk).shallow;
            *fresh21 &= !slot;
        }
    }
    marked
}
#[no_mangle]
#[c2rust::src_loc = "683:1"]
pub unsafe extern "C" fn ponyint_heap_mark_shallow(
    mut chunk: *mut chunk_t,
    mut p: *mut libc::c_void,
) {
    maybe_clear_chunk(chunk);
    if (*chunk).size
        >= ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        (*chunk).shallow = 0 as libc::c_int as u32;
    } else {
        let mut ext: *mut libc::c_void = (p as libc::uintptr_t
            & !((((1) << 5) << (*chunk).size) - 1) as usize)
            as *mut libc::c_void;
        let mut slot: u32 = ((1 as libc::c_int)
            << ((ext as *mut libc::c_char).offset_from((*chunk).m) as libc::c_long
                as libc::uintptr_t
                >> 5 as libc::c_int)) as u32;
        let ref mut fresh22 = (*chunk).shallow;
        *fresh22 &= !slot;
    };
}
#[no_mangle]
#[c2rust::src_loc = "702:1"]
pub unsafe extern "C" fn ponyint_heap_free(mut chunk: *mut chunk_t, mut p: *mut libc::c_void) {
    if (*chunk).size
        >= ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        if p == (*chunk).m as *mut libc::c_void {
            final_large(chunk, 0 as libc::c_int as u32);
            ponyint_pool_free_size((*chunk).size, (*chunk).m as *mut libc::c_void);
            let ref mut fresh23 = (*chunk).m;
            *fresh23 = 0 as *mut libc::c_char;
            (*chunk).slots = 1 as libc::c_int as u32;
        }
        return;
    }
    let mut ext: *mut libc::c_void =
        (p as libc::uintptr_t & !((((1) << 5) << (*chunk).size) - 1) as usize) as *mut libc::c_void;
    if p == ext {
        let mut slot: u32 = ((1 as libc::c_int)
            << ((ext as *mut libc::c_char).offset_from((*chunk).m) as libc::c_long
                as libc::uintptr_t
                >> 5 as libc::c_int)) as u32;
        if (*chunk).finalisers & slot != 0 as libc::c_int as libc::c_uint {
            ((**(p as *mut *const pony_type_t)).final_0).expect("non-null function pointer")(p);
            let ref mut fresh24 = (*chunk).finalisers;
            *fresh24 &= !slot;
        }
        let ref mut fresh25 = (*chunk).slots;
        *fresh25 |= slot;
    }
}
#[no_mangle]
#[c2rust::src_loc = "741:1"]
pub unsafe extern "C" fn ponyint_heap_endgc(mut heap: *mut heap_t) {
    let mut used: usize = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int {
        let mut list1: *mut chunk_t = (*heap).small_free[i as usize];
        let mut list2: *mut chunk_t = (*heap).small_full[i as usize];
        let ref mut fresh26 = (*heap).small_free[i as usize];
        *fresh26 = 0 as *mut chunk_t;
        let ref mut fresh27 = (*heap).small_full[i as usize];
        *fresh27 = 0 as *mut chunk_t;
        let mut avail: *mut *mut chunk_t =
            &mut *((*heap).small_free).as_mut_ptr().offset(i as isize) as *mut *mut chunk_t;
        let mut full: *mut *mut chunk_t =
            &mut *((*heap).small_full).as_mut_ptr().offset(i as isize) as *mut *mut chunk_t;
        let mut size: usize = (((1 as libc::c_int) << 5 as libc::c_int) << i) as usize;
        let mut empty: u32 = sizeclass_empty[i as usize];
        used = (used as libc::c_ulong).wrapping_add(
            sweep_small(list1, avail, full, empty, size)
                .try_into()
                .unwrap(),
        ) as usize as usize;
        used = (used as libc::c_ulong).wrapping_add(
            sweep_small(list2, avail, full, empty, size)
                .try_into()
                .unwrap(),
        ) as usize as usize;
        i += 1;
    }
    let ref mut fresh28 = (*heap).large;
    *fresh28 = sweep_large((*heap).large, &mut used);
    let ref mut fresh29 = (*heap).used;
    *fresh29 = (*fresh29 as libc::c_ulong).wrapping_add(used.try_into().unwrap()) as usize as usize;
    (*heap).next_gc = ((*heap).used as libc::c_double * heap_nextgc_factor) as usize;
    if (*heap).next_gc < heap_initialgc {
        (*heap).next_gc = heap_initialgc;
    }
}
#[no_mangle]
#[c2rust::src_loc = "804:1"]
pub unsafe extern "C" fn ponyint_heap_owner(mut chunk: *mut chunk_t) -> *mut pony_actor_t {
    (*chunk).actor
}
#[no_mangle]
#[c2rust::src_loc = "816:1"]
pub unsafe extern "C" fn ponyint_heap_size(mut chunk: *mut chunk_t) -> usize {
    if (*chunk).size
        >= ((10 as libc::c_int - 1 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong)
            .try_into()
            .unwrap()
    {
        return (*chunk).size;
    }
    return (((1 as libc::c_int) << 5 as libc::c_int) << (*chunk).size) as usize;
}
