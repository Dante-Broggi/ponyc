use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
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
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
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
        pub muted: size_t,
        pub internal_flags: u8,
        pub gc: gc_t,
    }
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_actor_pendingdestroy(actor: *mut pony_actor_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:3"]
pub mod gc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:16"]
    pub struct gc_t {
        pub mark: u32,
        pub rc_mark: u32,
        pub rc: size_t,
        pub local: objectmap_t,
        pub foreign: actormap_t,
        pub delta: *mut deltamap_t,
    }
    use super::actormap_h::actormap_t;
    use super::delta_h::deltamap_t;
    use super::objectmap_h::objectmap_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
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
    use super::actor_h::pony_actor_t;
    use super::hash_h::hashmap_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_deltamap_update(
            map: *mut deltamap_t,
            actor: *mut pony_actor_t,
            rc: size_t,
        ) -> *mut deltamap_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: size_t,
        pub size: size_t,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: size_t,
    }
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = size_t;
    use super::fun_h::{cmp_fn, free_fn};
    use super::stddef_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:1"]
pub mod actormap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:35"]
    pub struct actormap_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct actorref_t {
        pub actor: *mut pony_actor_t,
        pub rc: size_t,
        pub mark: u32,
        pub map: objectmap_t,
    }
    use super::actor_h::pony_actor_t;
    use super::hash_h::hashmap_t;
    use super::objectmap_h::objectmap_t;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:1"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct object_t {
        pub address: *mut libc::c_void,
        pub rc: size_t,
        pub mark: u32,
        pub immutable: bool,
        pub type_0: *const pony_type_t,
    }
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_init(map: *mut objectmap_t, size: size_t);
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_destroy(map: *mut objectmap_t);
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_optimize(map: *mut objectmap_t);
        #[c2rust::src_loc = "18:49"]
        pub fn ponyint_objectmap_put(map: *mut objectmap_t, entry: *mut object_t) -> *mut object_t;
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_clearindex(map: *mut objectmap_t, index: size_t);
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_size(map: *mut objectmap_t) -> size_t;
        #[c2rust::src_loc = "18:49"]
        pub fn ponyint_objectmap_next(map: *mut objectmap_t, i: *mut size_t) -> *mut object_t;
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_objectmap_getobject(
            map: *mut objectmap_t,
            address: *mut libc::c_void,
            index: *mut size_t,
        ) -> *mut object_t;
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_objectmap_getorput(
            map: *mut objectmap_t,
            address: *mut libc::c_void,
            type_0: *const pony_type_t,
            mark: u32,
        ) -> *mut object_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:3"]
pub mod heap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:16"]
    pub struct heap_t {
        pub small_free: [*mut chunk_t; 5],
        pub small_full: [*mut chunk_t; 5],
        pub large: *mut chunk_t,
        pub used: size_t,
        pub next_gc: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
    }
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
        pub traits: *mut *mut uintptr_t,
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
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> size_t>;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
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
    use super::_uintptr_t_h::uintptr_t;
    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn pony_sendp(
            ctx: *mut pony_ctx_t,
            to: *mut pony_actor_t,
            id: u32,
            p: *mut libc::c_void,
        );
    }
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
        pub serialise_size: size_t,
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
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:4"]
pub mod serialise_h {
    #[c2rust::src_loc = "18:1"]
    pub type serialise_throw_fn = Option<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "16:1"]
    pub type serialise_alloc_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, size_t) -> *mut libc::c_void>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:36"]
    pub struct ponyint_serialise_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
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
        pub counter: uintptr_t,
    }
    use super::_uintptr_t_h::uintptr_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:9"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::{pony_actor_t, ponyint_actor_pendingdestroy};
pub use self::actormap_h::{actormap_t, actorref_t};
pub use self::delta_h::{deltamap_t, ponyint_deltamap_update};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::gc_h::{gc_t, gcstack_t};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::heap_h::{chunk_t, heap_t};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::{
    object_t, objectmap_t, ponyint_objectmap_clearindex, ponyint_objectmap_destroy,
    ponyint_objectmap_getobject, ponyint_objectmap_getorput, ponyint_objectmap_init,
    ponyint_objectmap_next, ponyint_objectmap_optimize, ponyint_objectmap_put,
    ponyint_objectmap_size,
};
pub use self::pony_h::{
    _pony_type_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn, pony_dispatch_fn,
    pony_final_fn, pony_msg_t, pony_sendp, pony_serialise_fn, pony_trace_fn, pony_type_t,
};
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
use self::string_h::memset;
#[c2rust::src_loc = "50:1"]
pub type ponyint_actormap_free_fn = Option<unsafe extern "C" fn(*mut actorref_t) -> ()>;
#[c2rust::src_loc = "50:1"]
pub type ponyint_actormap_cmp_fn =
    Option<unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool>;
#[c2rust::src_loc = "11:1"]
unsafe extern "C" fn actorref_hash(mut aref: *mut actorref_t) -> size_t {
    ponyint_hash_ptr((*aref).actor as *const libc::c_void)
}
#[c2rust::src_loc = "16:1"]
unsafe extern "C" fn actorref_cmp(mut a: *mut actorref_t, mut b: *mut actorref_t) -> bool {
    (*a).actor == (*b).actor
}
#[c2rust::src_loc = "21:1"]
unsafe extern "C" fn actorref_alloc(
    mut actor: *mut pony_actor_t,
    mut mark: u32,
) -> *mut actorref_t {
    let mut aref: *mut actorref_t =
        ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut actorref_t;
    memset(
        aref as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<actorref_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*aref).actor;
    *fresh0 = actor;
    (*aref).mark = mark.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return aref;
}
#[no_mangle]
#[c2rust::src_loc = "32:1"]
pub unsafe extern "C" fn ponyint_actorref_getobject(
    mut aref: *mut actorref_t,
    mut address: *mut libc::c_void,
) -> *mut object_t {
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    return ponyint_objectmap_getobject(&mut (*aref).map, address, &mut index);
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn ponyint_actorref_getorput(
    mut aref: *mut actorref_t,
    mut address: *mut libc::c_void,
    mut type_0: *const pony_type_t,
    mut mark: u32,
) -> *mut object_t {
    ponyint_objectmap_getorput(&mut (*aref).map, address, type_0, mark)
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn ponyint_actorref_free(mut aref: *mut actorref_t) {
    ponyint_objectmap_destroy(&mut (*aref).map);
    ponyint_pool_free(1 as libc::c_int as size_t, aref as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_destroy(mut map: *mut actormap_t) {
    let mut freef: ponyint_actormap_free_fn =
        Some(ponyint_actorref_free as unsafe extern "C" fn(*mut actorref_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_actormap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_init(mut map: *mut actormap_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_alloc_size(mut map: *mut actormap_t) -> size_t {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_mem_size(mut map: *mut actormap_t) -> size_t {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_optimize(mut map: *mut actormap_t) {
    let mut cmpf: ponyint_actormap_cmp_fn =
        Some(actorref_cmp as unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_actormap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_size(mut map: *mut actormap_t) -> size_t {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_clearindex(mut map: *mut actormap_t, mut index: size_t) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_removeindex(mut map: *mut actormap_t, mut index: size_t) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_actormap_putindex(
    mut map: *mut actormap_t,
    mut entry: *mut actorref_t,
    mut index: size_t,
) {
    let mut cmpf: ponyint_actormap_cmp_fn =
        Some(actorref_cmp as unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        actorref_hash(entry),
        ::core::mem::transmute::<ponyint_actormap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "50:46"]
pub unsafe extern "C" fn ponyint_actormap_remove(
    mut map: *mut actormap_t,
    mut entry: *mut actorref_t,
) -> *mut actorref_t {
    let mut cmpf: ponyint_actormap_cmp_fn =
        Some(actorref_cmp as unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        actorref_hash(entry),
        ::core::mem::transmute::<ponyint_actormap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut actorref_t
}
#[no_mangle]
#[c2rust::src_loc = "50:46"]
pub unsafe extern "C" fn ponyint_actormap_put(
    mut map: *mut actormap_t,
    mut entry: *mut actorref_t,
) -> *mut actorref_t {
    let mut cmpf: ponyint_actormap_cmp_fn =
        Some(actorref_cmp as unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        actorref_hash(entry),
        ::core::mem::transmute::<ponyint_actormap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut actorref_t
}
#[no_mangle]
#[c2rust::src_loc = "50:46"]
pub unsafe extern "C" fn ponyint_actormap_get(
    mut map: *mut actormap_t,
    mut key: *mut actorref_t,
    mut index: *mut size_t,
) -> *mut actorref_t {
    let mut cmpf: ponyint_actormap_cmp_fn =
        Some(actorref_cmp as unsafe extern "C" fn(*mut actorref_t, *mut actorref_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        actorref_hash(key),
        ::core::mem::transmute::<ponyint_actormap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut actorref_t
}
#[no_mangle]
#[c2rust::src_loc = "50:46"]
pub unsafe extern "C" fn ponyint_actormap_next(
    mut map: *mut actormap_t,
    mut i: *mut size_t,
) -> *mut actorref_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut actorref_t
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn move_unmarked_objects(
    mut from: *mut actorref_t,
    mut mark: u32,
) -> *mut actorref_t {
    let mut size: size_t = ponyint_objectmap_size(&mut (*from).map);
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut actorref_t;
    }
    let mut to: *mut actorref_t = 0 as *mut actorref_t;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut obj: *mut object_t = 0 as *mut object_t;
    let mut needs_optimize: bool = 0 as libc::c_int != 0;
    loop {
        obj = ponyint_objectmap_next(&mut (*from).map, &mut i);
        if obj.is_null() {
            break;
        }
        if (*obj).mark == mark {
            continue;
        }
        ponyint_objectmap_clearindex(&mut (*from).map, i);
        needs_optimize = 1 as libc::c_int != 0;
        if to.is_null() {
            to = actorref_alloc((*from).actor, mark);
            ponyint_objectmap_init(&mut (*to).map, size);
        }
        ponyint_objectmap_put(&mut (*to).map, obj);
    }
    if needs_optimize {
        ponyint_objectmap_optimize(&mut (*from).map);
    }
    to
}
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn send_release(mut ctx: *mut pony_ctx_t, mut aref: *mut actorref_t) {
    if aref.is_null() {
        return;
    }
    if ponyint_actor_pendingdestroy((*aref).actor) as libc::c_int != 0
        || (*aref).rc == 0 as libc::c_int as libc::c_ulong
            && ponyint_objectmap_size(&mut (*aref).map) == 0 as libc::c_int as libc::c_ulong
    {
        ponyint_actorref_free(aref);
        return;
    }
    pony_sendp(
        ctx,
        (*aref).actor,
        (4294967295 as libc::c_uint).wrapping_sub(3 as libc::c_int as libc::c_uint),
        aref as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn ponyint_actormap_getactor(
    mut map: *mut actormap_t,
    mut actor: *mut pony_actor_t,
    mut index: *mut size_t,
) -> *mut actorref_t {
    let mut key: actorref_t = actorref_t {
        actor: 0 as *mut pony_actor_t,
        rc: 0,
        mark: 0,
        map: objectmap_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
    };
    key.actor = actor;
    ponyint_actormap_get(map, &mut key, index)
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn ponyint_actormap_getorput(
    mut map: *mut actormap_t,
    mut actor: *mut pony_actor_t,
    mut mark: u32,
) -> *mut actorref_t {
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut aref: *mut actorref_t = ponyint_actormap_getactor(map, actor, &mut index);
    if !aref.is_null() {
        return aref;
    }
    aref = actorref_alloc(actor, mark);
    ponyint_actormap_putindex(map, aref, index);
    aref
}
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn ponyint_actormap_sweep(
    mut ctx: *mut pony_ctx_t,
    mut map: *mut actormap_t,
    mut mark: u32,
    mut delta: *mut deltamap_t,
    mut actor_noblock: bool,
) -> *mut deltamap_t {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut aref: *mut actorref_t = 0 as *mut actorref_t;
    let mut needs_optimize: bool = 0 as libc::c_int != 0;
    loop {
        aref = ponyint_actormap_next(map, &mut i);
        if aref.is_null() {
            break;
        }
        if (*aref).mark == mark {
            aref = move_unmarked_objects(aref, mark);
        } else {
            ponyint_actormap_clearindex(map, i);
            if !actor_noblock {
                delta = ponyint_deltamap_update(delta, (*aref).actor, 0 as libc::c_int as size_t);
            }
            needs_optimize = 1 as libc::c_int != 0;
        }
        send_release(ctx, aref);
    }
    if needs_optimize {
        ponyint_actormap_optimize(map);
    }
    delta
}
