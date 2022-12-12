use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:1"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:1"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:2"]
pub mod actor_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct pony_actor_t {
        pub type_0: *const pony_type_t,
        pub q: messageq_t,
        pub sync_flags: uint8_t,
        pub cycle_detector_critical: uint8_t,
        pub heap: heap_t,
        pub muted: size_t,
        pub internal_flags: uint8_t,
        pub gc: gc_t,
    }
    use super::_uint8_t_h::uint8_t;
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "117:1"]
        pub fn ponyint_actor_gc(actor: *mut pony_actor_t) -> *mut gc_t;
        #[c2rust::src_loc = "119:1"]
        pub fn ponyint_actor_heap(actor: *mut pony_actor_t) -> *mut heap_t;
        #[c2rust::src_loc = "133:1"]
        pub fn ponyint_actor_getnoblock() -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:1"]
pub mod gc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:16"]
    pub struct gc_t {
        pub mark: uint32_t,
        pub rc_mark: uint32_t,
        pub rc: size_t,
        pub local: objectmap_t,
        pub foreign: actormap_t,
        pub delta: *mut deltamap_t,
    }
    use super::_uint32_t_h::uint32_t;
    use super::actormap_h::actormap_t;
    use super::delta_h::deltamap_t;
    use super::objectmap_h::objectmap_t;
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_deltamap_free(map: *mut deltamap_t);
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
    use super::stddef_h::size_t;
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
        pub mark: uint32_t,
        pub map: objectmap_t,
    }
    use super::_uint32_t_h::uint32_t;
    use super::actor_h::pony_actor_t;
    use super::delta_h::deltamap_t;
    use super::hash_h::hashmap_t;
    use super::objectmap_h::{object_t, objectmap_t};
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_actorref_getorput(
            aref: *mut actorref_t,
            address: *mut libc::c_void,
            type_0: *const pony_type_t,
            mark: uint32_t,
        ) -> *mut object_t;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_actorref_free(aref: *mut actorref_t);
        #[c2rust::src_loc = "27:1"]
        pub fn ponyint_actormap_destroy(map: *mut actormap_t);
        #[c2rust::src_loc = "27:1"]
        pub fn ponyint_actormap_clearindex(map: *mut actormap_t, index: size_t);
        #[c2rust::src_loc = "27:1"]
        pub fn ponyint_actormap_size(map: *mut actormap_t) -> size_t;
        #[c2rust::src_loc = "27:47"]
        pub fn ponyint_actormap_next(map: *mut actormap_t, i: *mut size_t) -> *mut actorref_t;
        #[c2rust::src_loc = "31:1"]
        pub fn ponyint_actormap_getorput(
            map: *mut actormap_t,
            actor: *mut pony_actor_t,
            mark: uint32_t,
        ) -> *mut actorref_t;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_actormap_sweep(
            ctx: *mut pony_ctx_t,
            map: *mut actormap_t,
            mark: uint32_t,
            delta: *mut deltamap_t,
            actor_noblock: bool,
        ) -> *mut deltamap_t;
    }
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
        pub mark: uint32_t,
        pub immutable: bool,
        pub type_0: *const pony_type_t,
    }
    use super::_uint32_t_h::uint32_t;
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_objectmap_destroy(map: *mut objectmap_t);
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
            mark: uint32_t,
        ) -> *mut object_t;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_objectmap_sweep(map: *mut objectmap_t);
    }
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
        pub used: size_t,
        pub next_gc: size_t,
    }
    use super::actor_h::pony_actor_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
        #[c2rust::src_loc = "70:1"]
        pub fn ponyint_heap_used(heap: *mut heap_t, size: size_t);
        #[c2rust::src_loc = "83:1"]
        pub fn ponyint_heap_mark(chunk: *mut chunk_t, p: *mut libc::c_void) -> bool;
        #[c2rust::src_loc = "89:1"]
        pub fn ponyint_heap_mark_shallow(chunk: *mut chunk_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_heap_owner(chunk: *mut chunk_t) -> *mut pony_actor_t;
        #[c2rust::src_loc = "105:1"]
        pub fn ponyint_heap_size(chunk: *mut chunk_t) -> size_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:2"]
pub mod messageq_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6:16"]
    pub struct messageq_t {
        pub head: *mut pony_msg_t,
        pub tail: *mut pony_msg_t,
    }
    use super::pony_h::pony_msg_t;
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
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
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
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed_0 = 0;
    use super::_uint32_t_h::uint32_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn pony_sendp(
            ctx: *mut pony_ctx_t,
            to: *mut pony_actor_t,
            id: uint32_t,
            p: *mut libc::c_void,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:2"]
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
        pub index: int32_t,
        pub cpu: uint32_t,
        pub node: uint32_t,
        pub terminate: bool,
        pub asio_stoppable: bool,
        pub asio_noisy: bool,
        pub sleep_object: *mut pthread_cond_t,
        pub last_victim: *mut scheduler_t,
        pub ctx: pony_ctx_t,
        pub block_count: uint32_t,
        pub ack_token: int32_t,
        pub ack_count: uint32_t,
        pub mute_mapping: mutemap_t,
        pub q: mpmcq_t,
        pub mq: messageq_t,
    }
    use super::_int32_t_h::int32_t;
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_t_h::pthread_t;
    use super::_uint32_t_h::uint32_t;
    use super::actor_h::pony_actor_t;
    use super::actormap_h::actormap_t;
    use super::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::pony_type_t;
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:2"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:2"]
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
        pub raw: __int128_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:1"]
    pub struct C2RustUnnamed {
        pub object: *mut mpmcq_node_t,
        pub counter: uintptr_t,
    }
    use super::_uintptr_t_h::uintptr_t;
    use super::internal::__int128_t;
    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:2"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:1"]
pub mod stack_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct Stack {
        pub index: libc::c_int,
        pub data: [*mut libc::c_void; 62],
        pub prev: *mut Stack,
    }
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn ponyint_stack_pop(stack: *mut Stack, data: *mut *mut libc::c_void) -> *mut Stack;
        #[c2rust::src_loc = "19:1"]
        pub fn ponyint_stack_push(list: *mut Stack, data: *mut libc::c_void) -> *mut Stack;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pagemap.h:4"]
pub mod pagemap_h {
    use super::heap_h::chunk_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn ponyint_pagemap_get(addr: *const libc::c_void) -> *mut chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:5"]
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
pub use self::_int32_t_h::int32_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::{
    pony_actor_t, ponyint_actor_gc, ponyint_actor_getnoblock, ponyint_actor_heap,
};
pub use self::actormap_h::{
    actormap_t, actorref_t, ponyint_actormap_clearindex, ponyint_actormap_destroy,
    ponyint_actormap_getorput, ponyint_actormap_next, ponyint_actormap_size,
    ponyint_actormap_sweep, ponyint_actorref_free, ponyint_actorref_getorput,
};
pub use self::delta_h::{deltamap_t, ponyint_deltamap_free, ponyint_deltamap_update};
pub use self::gc_h::gc_t;
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{
    chunk_t, heap_t, ponyint_heap_mark, ponyint_heap_mark_shallow, ponyint_heap_owner,
    ponyint_heap_size, ponyint_heap_used,
};
pub use self::internal::__int128_t;
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::{
    object_t, objectmap_t, ponyint_objectmap_destroy, ponyint_objectmap_getobject,
    ponyint_objectmap_getorput, ponyint_objectmap_next, ponyint_objectmap_sweep,
};
use self::pagemap_h::ponyint_pagemap_get;
pub use self::pony_h::{
    _pony_type_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn, pony_dispatch_fn,
    pony_final_fn, pony_msg_t, pony_sendp, pony_serialise_fn, pony_trace_fn, pony_type_t,
    C2RustUnnamed_0, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stack_h::{ponyint_stack_pop, ponyint_stack_push, Stack};
pub use self::stddef_h::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "11:31"]
pub struct gcstack_t {}
#[no_mangle]
#[c2rust::src_loc = "11:31"]
pub unsafe extern "C" fn ponyint_gcstack_pop(
    mut stack: *mut gcstack_t,
    mut data: *mut *mut libc::c_void,
) -> *mut gcstack_t {
    ponyint_stack_pop(stack as *mut Stack, data) as *mut gcstack_t
}
#[no_mangle]
#[c2rust::src_loc = "11:31"]
pub unsafe extern "C" fn ponyint_gcstack_push(
    mut stack: *mut gcstack_t,
    mut data: *mut libc::c_void,
) -> *mut gcstack_t {
    ponyint_stack_push(stack as *mut Stack, data) as *mut gcstack_t
}
#[c2rust::src_loc = "13:1"]
unsafe extern "C" fn acquire_actor(mut ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*ctx).acquire, actor, 0 as libc::c_int as uint32_t);
    let ref mut fresh0 = (*aref).rc;
    *fresh0 = (*fresh0 as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
}
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn acquire_object(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut address: *mut libc::c_void,
    mut immutable: bool,
) {
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*ctx).acquire, actor, 0 as libc::c_int as uint32_t);
    let mut type_0: *const pony_type_t = *(address as *mut *const pony_type_t);
    let mut obj: *mut object_t =
        ponyint_actorref_getorput(aref, address, type_0, 0 as libc::c_int as uint32_t);
    let ref mut fresh1 = (*obj).rc;
    *fresh1 = (*fresh1 as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    (*obj).immutable = immutable;
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn recurse(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut f: *mut libc::c_void,
) {
    if !f.is_null() {
        let ref mut fresh2 = (*ctx).stack;
        *fresh2 = ponyint_gcstack_push((*ctx).stack, p);
        let ref mut fresh3 = (*ctx).stack;
        *fresh3 = ponyint_gcstack_push((*ctx).stack, f);
    }
}
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn send_local_actor(mut gc: *mut gc_t) {
    if (*gc).rc_mark != (*gc).mark {
        (*gc).rc_mark = (*gc).mark;
        let ref mut fresh4 = (*gc).rc;
        *fresh4 = (*fresh4).wrapping_add(1);
    }
}
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn recv_local_actor(mut gc: *mut gc_t) {
    if (*gc).rc_mark != (*gc).mark {
        (*gc).rc_mark = (*gc).mark;
        if (*gc).rc > 0 as libc::c_int as libc::c_ulong {
        } else {
            ponyint_assert_fail(
                b"gc->rc > 0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                    as *const libc::c_char,
                55 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"recv_local_actor\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh5 = (*gc).rc;
        *fresh5 = (*fresh5).wrapping_sub(1);
    }
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn acquire_local_actor(mut gc: *mut gc_t) {
    let ref mut fresh6 = (*gc).rc;
    *fresh6 = (*fresh6).wrapping_add(1);
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn release_local_actor(mut gc: *mut gc_t) {
    if (*gc).rc > 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"gc->rc > 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            67 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"release_local_actor\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh7 = (*gc).rc;
    *fresh7 = (*fresh7).wrapping_sub(1);
}
#[c2rust::src_loc = "71:1"]
unsafe extern "C" fn send_remote_actor(
    mut ctx: *mut pony_ctx_t,
    mut gc: *mut gc_t,
    mut aref: *mut actorref_t,
) {
    if (*aref).mark == (*gc).mark {
        return;
    }
    (*aref).mark = (*gc).mark;
    if (*aref).rc <= 1 as libc::c_int as libc::c_ulong {
        let ref mut fresh8 = (*aref).rc;
        *fresh8 = (*fresh8 as libc::c_ulong)
            .wrapping_add((256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        acquire_actor(ctx, (*aref).actor);
    } else {
        let ref mut fresh9 = (*aref).rc;
        *fresh9 = (*fresh9).wrapping_sub(1);
    }
    if !ponyint_actor_getnoblock() {
        let ref mut fresh10 = (*gc).delta;
        *fresh10 = ponyint_deltamap_update((*gc).delta, (*aref).actor, (*aref).rc);
    }
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn recv_remote_actor(
    mut ctx: *mut pony_ctx_t,
    mut gc: *mut gc_t,
    mut aref: *mut actorref_t,
) {
    if (*aref).mark == (*gc).mark {
        return;
    }
    if (*aref).rc == 0 as libc::c_int as libc::c_ulong {
        ponyint_heap_used(
            ponyint_actor_heap((*ctx).current),
            1024 as libc::c_int as size_t,
        );
    }
    (*aref).mark = (*gc).mark;
    let ref mut fresh11 = (*aref).rc;
    *fresh11 = (*fresh11).wrapping_add(1);
    if !ponyint_actor_getnoblock() {
        let ref mut fresh12 = (*gc).delta;
        *fresh12 = ponyint_deltamap_update((*gc).delta, (*aref).actor, (*aref).rc);
    }
}
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn mark_remote_actor(
    mut ctx: *mut pony_ctx_t,
    mut gc: *mut gc_t,
    mut aref: *mut actorref_t,
) {
    if (*aref).mark == (*gc).mark {
        return;
    }
    ponyint_heap_used(
        ponyint_actor_heap((*ctx).current),
        1024 as libc::c_int as size_t,
    );
    (*aref).mark = (*gc).mark;
    if (*aref).rc == 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh13 = (*aref).rc;
        *fresh13 = (*fresh13 as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        acquire_actor(ctx, (*aref).actor);
        if !ponyint_actor_getnoblock() {
            let ref mut fresh14 = (*gc).delta;
            *fresh14 = ponyint_deltamap_update((*gc).delta, (*aref).actor, (*aref).rc);
        }
    }
}
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn acq_or_rel_remote_actor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*ctx).acquire, actor, 0 as libc::c_int as uint32_t);
    let ref mut fresh15 = (*aref).rc;
    *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
}
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn send_local_object(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut obj: *mut object_t = ponyint_objectmap_getorput(&mut (*gc).local, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    send_local_actor(gc);
    let ref mut fresh16 = (*obj).rc;
    *fresh16 = (*fresh16).wrapping_add(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn recv_local_object(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut obj: *mut object_t = ponyint_objectmap_getobject(&mut (*gc).local, p, &mut index);
    if !obj.is_null() {
    } else {
        ponyint_assert_fail(
            b"obj != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            172 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"recv_local_object\0"))
                .as_ptr(),
        );
    };
    if (*obj).mark == (*gc).mark {
        return;
    }
    recv_local_actor(gc);
    let ref mut fresh17 = (*obj).rc;
    *fresh17 = (*fresh17).wrapping_sub(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "194:1"]
unsafe extern "C" fn mark_local_object(
    mut ctx: *mut pony_ctx_t,
    mut chunk: *mut chunk_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    if mutability != PONY_TRACE_OPAQUE as libc::c_int {
        if !ponyint_heap_mark(chunk, p) {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
    } else {
        ponyint_heap_mark_shallow(chunk, p);
    };
}
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn acquire_local_object(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut obj: *mut object_t = ponyint_objectmap_getorput(&mut (*gc).local, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    acquire_local_actor(gc);
    let ref mut fresh18 = (*obj).rc;
    *fresh18 = (*fresh18).wrapping_add(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn release_local_object(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut obj: *mut object_t = ponyint_objectmap_getobject(&mut (*gc).local, p, &mut index);
    if !obj.is_null() {
    } else {
        ponyint_assert_fail(
            b"obj != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            240 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"release_local_object\0"))
                .as_ptr(),
        );
    };
    if (*obj).mark == (*gc).mark {
        return;
    }
    release_local_actor(gc);
    let ref mut fresh19 = (*obj).rc;
    *fresh19 = (*fresh19).wrapping_sub(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "261:1"]
unsafe extern "C" fn send_remote_object(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
    let mut obj: *mut object_t = ponyint_actorref_getorput(aref, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    send_remote_actor(ctx, gc, aref);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int
        && !(*obj).immutable
        && (*obj).rc > 0 as libc::c_int as libc::c_ulong
    {
        let ref mut fresh20 = (*obj).rc;
        *fresh20 = (*fresh20 as libc::c_ulong)
            .wrapping_add((256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        (*obj).immutable = 1 as libc::c_int != 0;
        acquire_object(ctx, actor, p, 1 as libc::c_int != 0);
        mutability = PONY_TRACE_MUTABLE as libc::c_int;
    } else if (*obj).rc <= 1 as libc::c_int as libc::c_ulong {
        if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
            (*obj).immutable = 1 as libc::c_int != 0;
        }
        let ref mut fresh21 = (*obj).rc;
        *fresh21 = (*fresh21 as libc::c_ulong)
            .wrapping_add((256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        acquire_object(ctx, actor, p, (*obj).immutable);
    } else {
        let ref mut fresh22 = (*obj).rc;
        *fresh22 = (*fresh22).wrapping_sub(1);
    }
    if mutability == PONY_TRACE_MUTABLE as libc::c_int {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn recv_remote_object(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
    mut chunk: *mut chunk_t,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
    let mut obj: *mut object_t = ponyint_actorref_getorput(aref, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    recv_remote_actor(ctx, gc, aref);
    if (*obj).rc == 0 as libc::c_int as libc::c_ulong {
        ponyint_heap_used(ponyint_actor_heap((*ctx).current), ponyint_heap_size(chunk));
        if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
            ponyint_heap_used(
                ponyint_actor_heap((*ctx).current),
                1024 as libc::c_int as size_t,
            );
        }
    }
    let ref mut fresh23 = (*obj).rc;
    *fresh23 = (*fresh23).wrapping_add(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "377:1"]
unsafe extern "C" fn mark_remote_object(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
    mut chunk: *mut chunk_t,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
    let mut obj: *mut object_t = ponyint_actorref_getorput(aref, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    ponyint_heap_used(ponyint_actor_heap((*ctx).current), ponyint_heap_size(chunk));
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        ponyint_heap_used(
            ponyint_actor_heap((*ctx).current),
            1024 as libc::c_int as size_t,
        );
    }
    mark_remote_actor(ctx, gc, aref);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int
        && !(*obj).immutable
        && (*obj).rc > 0 as libc::c_int as libc::c_ulong
    {
        let ref mut fresh24 = (*obj).rc;
        *fresh24 = (*fresh24 as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        (*obj).immutable = 1 as libc::c_int != 0;
        acquire_object(ctx, actor, p, 1 as libc::c_int != 0);
        mutability = PONY_TRACE_MUTABLE as libc::c_int;
    } else if (*obj).rc == 0 as libc::c_int as libc::c_ulong {
        if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
            (*obj).immutable = 1 as libc::c_int != 0;
        }
        let ref mut fresh25 = (*obj).rc;
        *fresh25 = (*fresh25 as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        acquire_object(ctx, actor, p, (*obj).immutable);
    }
    if mutability == PONY_TRACE_MUTABLE as libc::c_int {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn acq_or_rel_remote_object(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*ctx).acquire, actor, 0 as libc::c_int as uint32_t);
    let mut obj: *mut object_t = ponyint_actorref_getorput(aref, p, t, (*gc).mark);
    if (*obj).mark == (*gc).mark {
        return;
    }
    acq_or_rel_remote_actor(ctx, actor);
    let ref mut fresh26 = (*obj).rc;
    *fresh26 = (*fresh26).wrapping_add(1);
    (*obj).mark = (*gc).mark;
    if mutability == PONY_TRACE_OPAQUE as libc::c_int {
        return;
    }
    if mutability == PONY_TRACE_IMMUTABLE as libc::c_int {
        (*obj).immutable = 1 as libc::c_int != 0;
    }
    if !(*obj).immutable {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "471:1"]
pub unsafe extern "C" fn ponyint_gc_sendobject(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if chunk.is_null() {
        if mutability != PONY_TRACE_OPAQUE as libc::c_int {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
        return;
    }
    let mut actor: *mut pony_actor_t = ponyint_heap_owner(chunk);
    if actor == (*ctx).current {
        send_local_object(ctx, p, t, mutability);
    } else {
        send_remote_object(ctx, actor, p, t, mutability);
    };
}
#[no_mangle]
#[c2rust::src_loc = "492:1"]
pub unsafe extern "C" fn ponyint_gc_recvobject(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if chunk.is_null() {
        if mutability != PONY_TRACE_OPAQUE as libc::c_int {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
        return;
    }
    let mut actor: *mut pony_actor_t = ponyint_heap_owner(chunk);
    if actor == (*ctx).current {
        recv_local_object(ctx, p, t, mutability);
    } else {
        recv_remote_object(ctx, actor, p, t, mutability, chunk);
    };
}
#[no_mangle]
#[c2rust::src_loc = "513:1"]
pub unsafe extern "C" fn ponyint_gc_markobject(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if chunk.is_null() {
        if mutability != PONY_TRACE_OPAQUE as libc::c_int {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
        return;
    }
    let mut actor: *mut pony_actor_t = ponyint_heap_owner(chunk);
    if actor == (*ctx).current {
        mark_local_object(ctx, chunk, p, t, mutability);
    } else {
        mark_remote_object(ctx, actor, p, t, mutability, chunk);
    };
}
#[no_mangle]
#[c2rust::src_loc = "534:1"]
pub unsafe extern "C" fn ponyint_gc_acquireobject(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if chunk.is_null() {
        if mutability != PONY_TRACE_OPAQUE as libc::c_int {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
        return;
    }
    let mut actor: *mut pony_actor_t = ponyint_heap_owner(chunk);
    if actor == (*ctx).current {
        acquire_local_object(ctx, p, t, mutability);
    } else {
        acq_or_rel_remote_object(ctx, actor, p, t, mutability);
    };
}
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn ponyint_gc_releaseobject(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
    if chunk.is_null() {
        if mutability != PONY_TRACE_OPAQUE as libc::c_int {
            recurse(
                ctx,
                p,
                ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).trace),
            );
        }
        return;
    }
    let mut actor: *mut pony_actor_t = ponyint_heap_owner(chunk);
    if actor == (*ctx).current {
        release_local_object(ctx, p, t, mutability);
    } else {
        acq_or_rel_remote_object(ctx, actor, p, t, mutability);
    };
}
#[no_mangle]
#[c2rust::src_loc = "577:1"]
pub unsafe extern "C" fn ponyint_gc_sendactor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    if actor == (*ctx).current {
        send_local_actor(gc);
    } else {
        let mut aref: *mut actorref_t =
            ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
        send_remote_actor(ctx, gc, aref);
    };
}
#[no_mangle]
#[c2rust::src_loc = "591:1"]
pub unsafe extern "C" fn ponyint_gc_recvactor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    if actor == (*ctx).current {
        recv_local_actor(gc);
    } else {
        let mut aref: *mut actorref_t =
            ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
        recv_remote_actor(ctx, gc, aref);
    };
}
#[no_mangle]
#[c2rust::src_loc = "605:1"]
pub unsafe extern "C" fn ponyint_gc_markactor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    if actor == (*ctx).current {
        return;
    }
    let mut gc: *mut gc_t = ponyint_actor_gc((*ctx).current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
    mark_remote_actor(ctx, gc, aref);
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn ponyint_gc_acquireactor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    if actor == (*ctx).current {
        acquire_local_actor(ponyint_actor_gc((*ctx).current));
    } else {
        acq_or_rel_remote_actor(ctx, actor);
    };
}
#[no_mangle]
#[c2rust::src_loc = "623:1"]
pub unsafe extern "C" fn ponyint_gc_releaseactor(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    if actor == (*ctx).current {
        release_local_actor(ponyint_actor_gc((*ctx).current));
    } else {
        acq_or_rel_remote_actor(ctx, actor);
    };
}
#[no_mangle]
#[c2rust::src_loc = "631:1"]
pub unsafe extern "C" fn ponyint_gc_createactor(
    mut current: *mut pony_actor_t,
    mut actor: *mut pony_actor_t,
) {
    let mut gc: *mut gc_t = ponyint_actor_gc(current);
    let mut aref: *mut actorref_t =
        ponyint_actormap_getorput(&mut (*gc).foreign, actor, (*gc).mark);
    (*aref).rc = 256 as libc::c_int as size_t;
    if !ponyint_actor_getnoblock() {
        let ref mut fresh27 = (*gc).delta;
        *fresh27 = ponyint_deltamap_update((*gc).delta, actor, (*aref).rc);
    }
    ponyint_heap_used(ponyint_actor_heap(current), 1024 as libc::c_int as size_t);
}
#[no_mangle]
#[c2rust::src_loc = "645:1"]
pub unsafe extern "C" fn ponyint_gc_markimmutable(mut ctx: *mut pony_ctx_t, mut gc: *mut gc_t) {
    let mut map: *mut objectmap_t = &mut (*gc).local;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut obj: *mut object_t = 0 as *mut object_t;
    loop {
        obj = ponyint_objectmap_next(map, &mut i);
        if obj.is_null() {
            break;
        }
        if (*obj).immutable as libc::c_int != 0 && (*obj).rc > 0 as libc::c_int as libc::c_ulong {
            let mut p: *mut libc::c_void = (*obj).address;
            let mut chunk: *mut chunk_t = ponyint_pagemap_get(p);
            mark_local_object(
                ctx,
                chunk,
                p,
                (*obj).type_0,
                PONY_TRACE_IMMUTABLE as libc::c_int,
            );
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn ponyint_gc_handlestack(mut ctx: *mut pony_ctx_t) {
    let mut f: pony_trace_fn = None;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    while !((*ctx).stack).is_null() {
        let ref mut fresh28 = (*ctx).stack;
        *fresh28 = ponyint_gcstack_pop(
            (*ctx).stack,
            &mut f as *mut pony_trace_fn as *mut *mut libc::c_void,
        );
        let ref mut fresh29 = (*ctx).stack;
        *fresh29 = ponyint_gcstack_pop((*ctx).stack, &mut p);
        f.expect("non-null function pointer")(ctx, p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "676:1"]
pub unsafe extern "C" fn ponyint_gc_discardstack(mut ctx: *mut pony_ctx_t) {
    let mut f: pony_trace_fn = None;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    while !((*ctx).stack).is_null() {
        let ref mut fresh30 = (*ctx).stack;
        *fresh30 = ponyint_gcstack_pop(
            (*ctx).stack,
            &mut f as *mut pony_trace_fn as *mut *mut libc::c_void,
        );
        let ref mut fresh31 = (*ctx).stack;
        *fresh31 = ponyint_gcstack_pop((*ctx).stack, &mut p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "688:1"]
pub unsafe extern "C" fn ponyint_gc_sweep(mut ctx: *mut pony_ctx_t, mut gc: *mut gc_t) {
    ponyint_objectmap_sweep(&mut (*gc).local);
    let ref mut fresh32 = (*gc).delta;
    *fresh32 = ponyint_actormap_sweep(
        ctx,
        &mut (*gc).foreign,
        (*gc).mark,
        (*gc).delta,
        ponyint_actor_getnoblock(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "718:1"]
pub unsafe extern "C" fn ponyint_gc_acquire(mut gc: *mut gc_t, mut aref: *mut actorref_t) -> bool {
    let mut rc: size_t = (*aref).rc;
    let ref mut fresh33 = (*gc).rc;
    *fresh33 = (*fresh33 as libc::c_ulong).wrapping_add(rc) as size_t as size_t;
    let mut map: *mut objectmap_t = &mut (*aref).map;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut obj: *mut object_t = 0 as *mut object_t;
    loop {
        obj = ponyint_objectmap_next(map, &mut i);
        if obj.is_null() {
            break;
        }
        let mut obj_local: *mut object_t =
            ponyint_objectmap_getorput(&mut (*gc).local, (*obj).address, (*obj).type_0, (*gc).mark);
        let ref mut fresh34 = (*obj_local).rc;
        *fresh34 = (*fresh34 as libc::c_ulong).wrapping_add((*obj).rc) as size_t as size_t;
        if (*obj).immutable {
            (*obj_local).immutable = 1 as libc::c_int != 0;
        }
    }
    ponyint_actorref_free(aref);
    return rc > 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
#[c2rust::src_loc = "744:1"]
pub unsafe extern "C" fn ponyint_gc_release(mut gc: *mut gc_t, mut aref: *mut actorref_t) -> bool {
    let mut rc: size_t = (*aref).rc;
    if (*gc).rc >= rc {
    } else {
        ponyint_assert_fail(
            b"gc->rc >= rc\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            747 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"ponyint_gc_release\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh35 = (*gc).rc;
    *fresh35 = (*fresh35 as libc::c_ulong).wrapping_sub(rc) as size_t as size_t;
    let mut map: *mut objectmap_t = &mut (*aref).map;
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut obj: *mut object_t = 0 as *mut object_t;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    loop {
        obj = ponyint_objectmap_next(map, &mut i);
        if obj.is_null() {
            break;
        }
        let mut p: *mut libc::c_void = (*obj).address;
        let mut obj_local: *mut object_t =
            ponyint_objectmap_getobject(&mut (*gc).local, p, &mut index);
        if (*obj_local).rc >= (*obj).rc {
        } else {
            ponyint_assert_fail(
                b"obj_local->rc >= obj->rc\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                    as *const libc::c_char,
                760 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"ponyint_gc_release\0",
                ))
                .as_ptr(),
            );
        };
        let ref mut fresh36 = (*obj_local).rc;
        *fresh36 = (*fresh36 as libc::c_ulong).wrapping_sub((*obj).rc) as size_t as size_t;
    }
    ponyint_actorref_free(aref);
    return rc > 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
#[c2rust::src_loc = "768:1"]
pub unsafe extern "C" fn ponyint_gc_rc(mut gc: *mut gc_t) -> size_t {
    return (*gc).rc;
}
#[no_mangle]
#[c2rust::src_loc = "773:1"]
pub unsafe extern "C" fn ponyint_gc_delta(mut gc: *mut gc_t) -> *mut deltamap_t {
    let mut delta: *mut deltamap_t = (*gc).delta;
    let ref mut fresh37 = (*gc).delta;
    *fresh37 = 0 as *mut deltamap_t;
    return delta;
}
#[no_mangle]
#[c2rust::src_loc = "780:1"]
pub unsafe extern "C" fn ponyint_gc_sendacquire(mut ctx: *mut pony_ctx_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut aref: *mut actorref_t = 0 as *mut actorref_t;
    loop {
        aref = ponyint_actormap_next(&mut (*ctx).acquire, &mut i);
        if aref.is_null() {
            break;
        }
        ponyint_actormap_clearindex(&mut (*ctx).acquire, i);
        pony_sendp(
            ctx,
            (*aref).actor,
            (4294967295 as libc::c_uint).wrapping_sub(4 as libc::c_int as libc::c_uint),
            aref as *mut libc::c_void,
        );
    }
    if ponyint_actormap_size(&mut (*ctx).acquire) == 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"ponyint_actormap_size(&ctx->acquire) == 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            798 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"ponyint_gc_sendacquire\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "801:1"]
pub unsafe extern "C" fn ponyint_gc_sendrelease(mut ctx: *mut pony_ctx_t, mut gc: *mut gc_t) {
    let ref mut fresh38 = (*gc).delta;
    *fresh38 = ponyint_actormap_sweep(
        ctx,
        &mut (*gc).foreign,
        (*gc).mark,
        (*gc).delta,
        ponyint_actor_getnoblock(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "822:1"]
pub unsafe extern "C" fn ponyint_gc_sendrelease_manual(mut ctx: *mut pony_ctx_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut aref: *mut actorref_t = 0 as *mut actorref_t;
    loop {
        aref = ponyint_actormap_next(&mut (*ctx).acquire, &mut i);
        if aref.is_null() {
            break;
        }
        ponyint_actormap_clearindex(&mut (*ctx).acquire, i);
        pony_sendp(
            ctx,
            (*aref).actor,
            (4294967295 as libc::c_uint).wrapping_sub(3 as libc::c_int as libc::c_uint),
            aref as *mut libc::c_void,
        );
    }
    if ponyint_actormap_size(&mut (*ctx).acquire) == 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"ponyint_actormap_size(&ctx->acquire) == 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.c\0" as *const u8
                as *const libc::c_char,
            840 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"ponyint_gc_sendrelease_manual\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn ponyint_gc_done(mut gc: *mut gc_t) {
    let ref mut fresh39 = (*gc).mark;
    *fresh39 = (*fresh39).wrapping_add(1);
}
#[no_mangle]
#[c2rust::src_loc = "848:1"]
pub unsafe extern "C" fn ponyint_gc_destroy(mut gc: *mut gc_t) {
    ponyint_objectmap_destroy(&mut (*gc).local);
    ponyint_actormap_destroy(&mut (*gc).foreign);
    if !((*gc).delta).is_null() {
        ponyint_deltamap_free((*gc).delta);
        let ref mut fresh40 = (*gc).delta;
        *fresh40 = 0 as *mut deltamap_t;
    }
}
