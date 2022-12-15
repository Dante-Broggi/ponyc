use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:5"]
pub mod _types_h {
    #[c2rust::src_loc = "51:1"]
    pub type __darwin_intptr_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:5"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:5"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h:5"]
pub mod _intptr_t_h {
    #[c2rust::src_loc = "32:1"]
    pub type intptr_t = __darwin_intptr_t;
    use super::_types_h::__darwin_intptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:5"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:5"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:5"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:5"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> usize;
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_hash_size(key: usize) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:6"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:16"]
    pub struct pony_actor_pad_t {
        pub type_0: *const pony_type_t,
        pub pad: [libc::c_char; 264],
    }
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_actor_destroy(actor: *mut pony_actor_t);
        #[c2rust::src_loc = "127:1"]
        pub fn ponyint_actor_sendrelease(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "125:1"]
        pub fn ponyint_actor_final(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "123:1"]
        pub fn ponyint_actor_setpendingdestroy(actor: *mut pony_actor_t);
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_actor_pendingdestroy(actor: *mut pony_actor_t) -> bool;
        #[c2rust::src_loc = "129:1"]
        pub fn ponyint_actor_setsystem(actor: *mut pony_actor_t);
        #[c2rust::src_loc = "157:1"]
        pub fn ponyint_release_cycle_detector_critical(actor: *mut pony_actor_t);
        #[c2rust::src_loc = "155:1"]
        pub fn ponyint_acquire_cycle_detector_critical(actor: *mut pony_actor_t) -> bool;
        #[c2rust::src_loc = "143:1"]
        pub fn ponyint_destroy(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_become(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:5"]
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
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
        #[c2rust::src_loc = "77:1"]
        pub fn ponyint_gc_delta(gc: *mut gc_t) -> *mut deltamap_t;
        #[c2rust::src_loc = "75:1"]
        pub fn ponyint_gc_rc(gc: *mut gc_t) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/delta.h:5"]
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
        #[c2rust::src_loc = "10:16"]
        pub type delta_t;
        #[c2rust::src_loc = "12:1"]
        pub fn ponyint_delta_actor(delta: *mut delta_t) -> *mut pony_actor_t;
        #[c2rust::src_loc = "14:1"]
        pub fn ponyint_delta_rc(delta: *mut delta_t) -> usize;
        #[c2rust::src_loc = "16:47"]
        pub fn ponyint_deltamap_next(map: *mut deltamap_t, i: *mut usize) -> *mut delta_t;
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_deltamap_free(map: *mut deltamap_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:5"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:5"]
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
        pub rc: usize,
        pub mark: u32,
        pub map: objectmap_t,
    }
    use super::actor_h::pony_actor_t;
    use super::hash_h::hashmap_t;
    use super::objectmap_h::objectmap_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn ponyint_actormap_size(map: *mut actormap_t) -> usize;
        #[c2rust::src_loc = "27:47"]
        pub fn ponyint_actormap_next(map: *mut actormap_t, i: *mut usize) -> *mut actorref_t;
        #[c2rust::src_loc = "29:1"]
        pub fn ponyint_actormap_getactor(
            map: *mut actormap_t,
            actor: *mut pony_actor_t,
            index: *mut usize,
        ) -> *mut actorref_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:5"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:5"]
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
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:6"]
pub mod messageq_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6:16"]
    pub struct messageq_t {
        pub head: *mut pony_msg_t,
        pub tail: *mut pony_msg_t,
    }
    use super::pony_h::pony_msg_t;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ponyint_messageq_markempty(q: *mut messageq_t) -> bool;
        #[c2rust::src_loc = "39:1"]
        pub fn ponyint_actor_messageq_pop(q: *mut messageq_t) -> *mut pony_msg_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:5"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:16"]
    pub struct pony_msgi_t {
        pub msg: pony_msg_t,
        pub i: intptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct pony_msgp_t {
        pub msg: pony_msg_t,
        pub p: *mut libc::c_void,
    }
    use super::_intptr_t_h::intptr_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "192:1"]
        pub fn pony_create(
            ctx: *mut pony_ctx_t,
            type_0: *const pony_type_t,
            orphaned: bool,
        ) -> *mut pony_actor_t;
        #[c2rust::src_loc = "196:1"]
        pub fn pony_alloc_msg(index: u32, id: u32) -> *mut pony_msg_t;
        #[c2rust::src_loc = "208:1"]
        pub fn pony_sendv(
            ctx: *mut pony_ctx_t,
            to: *mut pony_actor_t,
            first: *mut pony_msg_t,
            last: *mut pony_msg_t,
            has_app_msg: bool,
        );
        #[c2rust::src_loc = "241:1"]
        pub fn pony_send(ctx: *mut pony_ctx_t, to: *mut pony_actor_t, id: u32);
        #[c2rust::src_loc = "247:1"]
        pub fn pony_sendp(
            ctx: *mut pony_ctx_t,
            to: *mut pony_actor_t,
            id: u32,
            p: *mut libc::c_void,
        );
        #[c2rust::src_loc = "254:1"]
        pub fn pony_sendi(ctx: *mut pony_ctx_t, to: *mut pony_actor_t, id: u32, i: intptr_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:6"]
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
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "154:1"]
        pub fn ponyint_sched_get_inject_context() -> *mut pony_ctx_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:6"]
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
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:6"]
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
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:6"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:5"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:5"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:7"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_cpu_tick_diff(supposedly_earlier: u64, supposedly_later: u64) -> u64;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:12"]
pub mod ponyassert_h {
    use super::stddef_h::size_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:13"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:14"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_intptr_t_h::intptr_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_types_h::__darwin_intptr_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::{
    pony_actor_pad_t, pony_actor_t, ponyint_acquire_cycle_detector_critical, ponyint_actor_destroy,
    ponyint_actor_final, ponyint_actor_pendingdestroy, ponyint_actor_sendrelease,
    ponyint_actor_setpendingdestroy, ponyint_actor_setsystem, ponyint_become, ponyint_destroy,
    ponyint_release_cycle_detector_critical,
};
pub use self::actormap_h::{
    actormap_t, actorref_t, ponyint_actormap_getactor, ponyint_actormap_next, ponyint_actormap_size,
};
use self::cpu_h::ponyint_cpu_tick_diff;
pub use self::delta_h::{
    delta_t, deltamap_t, ponyint_delta_actor, ponyint_delta_rc, ponyint_deltamap_free,
    ponyint_deltamap_next,
};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr, ponyint_hash_size};
pub use self::gc_h::{gc_t, gcstack_t, ponyint_gc_delta, ponyint_gc_rc};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::heap_h::{chunk_t, heap_t};
pub use self::messageq_h::{messageq_t, ponyint_actor_messageq_pop, ponyint_messageq_markempty};
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
pub use self::pony_h::{
    _pony_type_t, pony_alloc_msg, pony_create, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_msgi_t,
    pony_msgp_t, pony_send, pony_sendi, pony_sendp, pony_sendv, pony_serialise_fn, pony_trace_fn,
    pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::scheduler_h::{
    pony_ctx_t, ponyint_sched_get_inject_context, scheduler_t, trace_actor_fn, trace_object_fn,
};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stack_h::{ponyint_stack_pop, ponyint_stack_push, Stack};
pub use self::stddef_h::size_t;
use self::stdio_h::printf;
use self::string_h::memset;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "285:16"]
pub struct detector_t {
    pub pad: pony_actor_pad_t,
    pub next_token: usize,
    pub detect_interval: usize,
    pub last_checked: usize,
    pub views: viewmap_t,
    pub deferred: viewmap_t,
    pub perceived: perceivedmap_t,
    pub attempted: usize,
    pub detected: usize,
    pub collected: usize,
    pub created: usize,
    pub destroyed: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "278:39"]
pub struct perceivedmap_t {
    pub contents: hashmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "245:34"]
pub struct viewmap_t {
    pub contents: hashmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "203:8"]
pub struct view_t {
    pub actor: *mut pony_actor_t,
    pub rc: usize,
    pub view_rc: u32,
    pub blocked: bool,
    pub deferred: bool,
    pub color: u8,
    pub map: viewrefmap_t,
    pub perceived: *mut perceived_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "248:8"]
pub struct perceived_t {
    pub token: usize,
    pub ack: usize,
    pub map: viewmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "178:37"]
pub struct viewrefmap_t {
    pub contents: hashmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "176:36"]
pub struct viewrefstack_t {}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "154:16"]
pub struct viewref_t {
    pub view: *mut view_t,
    pub rc: usize,
}
#[c2rust::src_loc = "200:3"]
pub const COLOR_WHITE: C2RustUnnamed_0 = 2;
#[c2rust::src_loc = "198:3"]
pub const COLOR_BLACK: C2RustUnnamed_0 = 0;
#[c2rust::src_loc = "199:3"]
pub const COLOR_GREY: C2RustUnnamed_0 = 1;
#[c2rust::src_loc = "246:1"]
pub type ponyint_viewmap_cmp_fn = Option<unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool>;
#[c2rust::src_loc = "279:1"]
pub type ponyint_perceivedmap_cmp_fn =
    Option<unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool>;
#[c2rust::src_loc = "246:1"]
pub type ponyint_viewmap_free_fn = Option<unsafe extern "C" fn(*mut view_t) -> ()>;
#[c2rust::src_loc = "179:1"]
pub type ponyint_viewrefmap_free_fn = Option<unsafe extern "C" fn(*mut viewref_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "143:16"]
pub struct block_msg_t {
    pub msg: pony_msg_t,
    pub actor: *mut pony_actor_t,
    pub rc: usize,
    pub delta: *mut deltamap_t,
}
#[c2rust::src_loc = "179:1"]
pub type ponyint_viewrefmap_cmp_fn =
    Option<unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool>;
#[c2rust::src_loc = "279:1"]
pub type ponyint_perceivedmap_free_fn = Option<unsafe extern "C" fn(*mut perceived_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "283:43"]
pub struct pendingdestroystack_t {}
#[c2rust::src_loc = "196:1"]
pub type C2RustUnnamed_0 = libc::c_uint;
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn viewref_hash(mut vref: *mut viewref_t) -> usize {
    ponyint_hash_ptr((*vref).view as *const libc::c_void)
}
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn viewref_cmp(mut a: *mut viewref_t, mut b: *mut viewref_t) -> bool {
    (*a).view == (*b).view
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn viewref_free(mut vref: *mut viewref_t) {
    ponyint_pool_free(0 as libc::c_int as usize, vref as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "176:36"]
pub unsafe extern "C" fn ponyint_viewrefstack_pop(
    mut stack: *mut viewrefstack_t,
    mut data: *mut *mut viewref_t,
) -> *mut viewrefstack_t {
    ponyint_stack_pop(stack as *mut Stack, data as *mut *mut libc::c_void) as *mut viewrefstack_t
}
#[no_mangle]
#[c2rust::src_loc = "176:36"]
pub unsafe extern "C" fn ponyint_viewrefstack_push(
    mut stack: *mut viewrefstack_t,
    mut data: *mut viewref_t,
) -> *mut viewrefstack_t {
    ponyint_stack_push(stack as *mut Stack, data as *mut libc::c_void) as *mut viewrefstack_t
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_alloc_size(mut map: *mut viewrefmap_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_removeindex(
    mut map: *mut viewrefmap_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_optimize(mut map: *mut viewrefmap_t) {
    let mut cmpf: ponyint_viewrefmap_cmp_fn =
        Some(viewref_cmp as unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_viewrefmap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_size(mut map: *mut viewrefmap_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_init(mut map: *mut viewrefmap_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_putindex(
    mut map: *mut viewrefmap_t,
    mut entry: *mut viewref_t,
    mut index: usize,
) {
    let mut cmpf: ponyint_viewrefmap_cmp_fn =
        Some(viewref_cmp as unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        viewref_hash(entry),
        ::core::mem::transmute::<ponyint_viewrefmap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_mem_size(mut map: *mut viewrefmap_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_destroy(mut map: *mut viewrefmap_t) {
    let mut freef: ponyint_viewrefmap_free_fn =
        Some(viewref_free as unsafe extern "C" fn(*mut viewref_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_viewrefmap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn ponyint_viewrefmap_clearindex(
    mut map: *mut viewrefmap_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "179:50"]
pub unsafe extern "C" fn ponyint_viewrefmap_remove(
    mut map: *mut viewrefmap_t,
    mut entry: *mut viewref_t,
) -> *mut viewref_t {
    let mut cmpf: ponyint_viewrefmap_cmp_fn =
        Some(viewref_cmp as unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        viewref_hash(entry),
        ::core::mem::transmute::<ponyint_viewrefmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut viewref_t
}
#[no_mangle]
#[c2rust::src_loc = "179:50"]
pub unsafe extern "C" fn ponyint_viewrefmap_get(
    mut map: *mut viewrefmap_t,
    mut key: *mut viewref_t,
    mut index: *mut usize,
) -> *mut viewref_t {
    let mut cmpf: ponyint_viewrefmap_cmp_fn =
        Some(viewref_cmp as unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        viewref_hash(key),
        ::core::mem::transmute::<ponyint_viewrefmap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut viewref_t
}
#[no_mangle]
#[c2rust::src_loc = "179:50"]
pub unsafe extern "C" fn ponyint_viewrefmap_put(
    mut map: *mut viewrefmap_t,
    mut entry: *mut viewref_t,
) -> *mut viewref_t {
    let mut cmpf: ponyint_viewrefmap_cmp_fn =
        Some(viewref_cmp as unsafe extern "C" fn(*mut viewref_t, *mut viewref_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        viewref_hash(entry),
        ::core::mem::transmute::<ponyint_viewrefmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut viewref_t
}
#[no_mangle]
#[c2rust::src_loc = "179:50"]
pub unsafe extern "C" fn ponyint_viewrefmap_next(
    mut map: *mut viewrefmap_t,
    mut i: *mut usize,
) -> *mut viewref_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets) as *mut viewref_t
}
#[no_mangle]
#[c2rust::src_loc = "201:3"]
pub static mut ponyint_color_t: C2RustUnnamed_0 = COLOR_BLACK;
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn view_hash(mut view: *mut view_t) -> usize {
    ponyint_hash_ptr((*view).actor as *const libc::c_void)
}
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn view_cmp(mut a: *mut view_t, mut b: *mut view_t) -> bool {
    (*a).actor == (*b).actor
}
#[c2rust::src_loc = "229:1"]
unsafe extern "C" fn view_free(mut view: *mut view_t) {
    let ref mut fresh0 = (*view).view_rc;
    *fresh0 = (*fresh0).wrapping_sub(1);
    if (*view).view_rc == 0 as libc::c_int as libc::c_uint {
        ponyint_viewrefmap_destroy(&mut (*view).map);
        ponyint_pool_free(1 as libc::c_int as usize, view as *mut libc::c_void);
    }
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_putindex(
    mut map: *mut viewmap_t,
    mut entry: *mut view_t,
    mut index: usize,
) {
    let mut cmpf: ponyint_viewmap_cmp_fn =
        Some(view_cmp as unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        view_hash(entry),
        ::core::mem::transmute::<ponyint_viewmap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_clearindex(mut map: *mut viewmap_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_optimize(mut map: *mut viewmap_t) {
    let mut cmpf: ponyint_viewmap_cmp_fn =
        Some(view_cmp as unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_viewmap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_size(mut map: *mut viewmap_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_destroy(mut map: *mut viewmap_t) {
    let mut freef: ponyint_viewmap_free_fn = None;
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_viewmap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_mem_size(mut map: *mut viewmap_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_alloc_size(mut map: *mut viewmap_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_init(mut map: *mut viewmap_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn ponyint_viewmap_removeindex(mut map: *mut viewmap_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "246:44"]
pub unsafe extern "C" fn ponyint_viewmap_get(
    mut map: *mut viewmap_t,
    mut key: *mut view_t,
    mut index: *mut usize,
) -> *mut view_t {
    let mut cmpf: ponyint_viewmap_cmp_fn =
        Some(view_cmp as unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        view_hash(key),
        ::core::mem::transmute::<ponyint_viewmap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut view_t
}
#[no_mangle]
#[c2rust::src_loc = "246:44"]
pub unsafe extern "C" fn ponyint_viewmap_put(
    mut map: *mut viewmap_t,
    mut entry: *mut view_t,
) -> *mut view_t {
    let mut cmpf: ponyint_viewmap_cmp_fn =
        Some(view_cmp as unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        view_hash(entry),
        ::core::mem::transmute::<ponyint_viewmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut view_t
}
#[no_mangle]
#[c2rust::src_loc = "246:44"]
pub unsafe extern "C" fn ponyint_viewmap_remove(
    mut map: *mut viewmap_t,
    mut entry: *mut view_t,
) -> *mut view_t {
    let mut cmpf: ponyint_viewmap_cmp_fn =
        Some(view_cmp as unsafe extern "C" fn(*mut view_t, *mut view_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        view_hash(entry),
        ::core::mem::transmute::<ponyint_viewmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut view_t
}
#[no_mangle]
#[c2rust::src_loc = "246:44"]
pub unsafe extern "C" fn ponyint_viewmap_next(
    mut map: *mut viewmap_t,
    mut i: *mut usize,
) -> *mut view_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets) as *mut view_t
}
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn perceived_hash(mut per: *mut perceived_t) -> usize {
    ponyint_hash_size((*per).token)
}
#[c2rust::src_loc = "260:1"]
unsafe extern "C" fn perceived_cmp(mut a: *mut perceived_t, mut b: *mut perceived_t) -> bool {
    (*a).token == (*b).token
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn perceived_free(mut per: *mut perceived_t) {
    ponyint_viewmap_destroy(&mut (*per).map);
    ponyint_pool_free(1 as libc::c_int as usize, per as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_optimize(mut map: *mut perceivedmap_t) {
    let mut cmpf: ponyint_perceivedmap_cmp_fn =
        Some(perceived_cmp as unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_perceivedmap_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_alloc_size(mut map: *mut perceivedmap_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_mem_size(mut map: *mut perceivedmap_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_fill_ratio(
    mut map: *mut hashmap_t,
) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_size(mut map: *mut perceivedmap_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_putindex(
    mut map: *mut perceivedmap_t,
    mut entry: *mut perceived_t,
    mut index: usize,
) {
    let mut cmpf: ponyint_perceivedmap_cmp_fn =
        Some(perceived_cmp as unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        perceived_hash(entry),
        ::core::mem::transmute::<ponyint_perceivedmap_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_clearindex(
    mut map: *mut perceivedmap_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_init(mut map: *mut perceivedmap_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_removeindex(
    mut map: *mut perceivedmap_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn ponyint_perceivedmap_destroy(mut map: *mut perceivedmap_t) {
    let mut freef: ponyint_perceivedmap_free_fn =
        Some(perceived_free as unsafe extern "C" fn(*mut perceived_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_perceivedmap_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "279:54"]
pub unsafe extern "C" fn ponyint_perceivedmap_put(
    mut map: *mut perceivedmap_t,
    mut entry: *mut perceived_t,
) -> *mut perceived_t {
    let mut cmpf: ponyint_perceivedmap_cmp_fn =
        Some(perceived_cmp as unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        perceived_hash(entry),
        ::core::mem::transmute::<ponyint_perceivedmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut perceived_t
}
#[no_mangle]
#[c2rust::src_loc = "279:54"]
pub unsafe extern "C" fn ponyint_perceivedmap_get(
    mut map: *mut perceivedmap_t,
    mut key: *mut perceived_t,
    mut index: *mut usize,
) -> *mut perceived_t {
    let mut cmpf: ponyint_perceivedmap_cmp_fn =
        Some(perceived_cmp as unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        perceived_hash(key),
        ::core::mem::transmute::<ponyint_perceivedmap_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut perceived_t
}
#[no_mangle]
#[c2rust::src_loc = "279:54"]
pub unsafe extern "C" fn ponyint_perceivedmap_next(
    mut map: *mut perceivedmap_t,
    mut i: *mut usize,
) -> *mut perceived_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut perceived_t
}
#[no_mangle]
#[c2rust::src_loc = "279:54"]
pub unsafe extern "C" fn ponyint_perceivedmap_remove(
    mut map: *mut perceivedmap_t,
    mut entry: *mut perceived_t,
) -> *mut perceived_t {
    let mut cmpf: ponyint_perceivedmap_cmp_fn =
        Some(perceived_cmp as unsafe extern "C" fn(*mut perceived_t, *mut perceived_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        perceived_hash(entry),
        ::core::mem::transmute::<ponyint_perceivedmap_cmp_fn, cmp_fn>(cmpf),
    ) as *mut perceived_t
}
#[no_mangle]
#[c2rust::src_loc = "283:43"]
pub unsafe extern "C" fn ponyint_pendingdestroystack_push(
    mut stack: *mut pendingdestroystack_t,
    mut data: *mut pony_actor_t,
) -> *mut pendingdestroystack_t {
    ponyint_stack_push(stack as *mut Stack, data as *mut libc::c_void) as *mut pendingdestroystack_t
}
#[no_mangle]
#[c2rust::src_loc = "283:43"]
pub unsafe extern "C" fn ponyint_pendingdestroystack_pop(
    mut stack: *mut pendingdestroystack_t,
    mut data: *mut *mut pony_actor_t,
) -> *mut pendingdestroystack_t {
    ponyint_stack_pop(stack as *mut Stack, data as *mut *mut libc::c_void)
        as *mut pendingdestroystack_t
}
#[c2rust::src_loc = "310:22"]
static mut cycle_detector: *mut pony_actor_t = 0 as *const pony_actor_t as *mut pony_actor_t;
#[c2rust::src_loc = "332:1"]
unsafe extern "C" fn get_view(
    mut d: *mut detector_t,
    mut actor: *mut pony_actor_t,
    mut create: bool,
) -> *mut view_t {
    if !actor.is_null() {
    } else {
        ponyint_assert_fail(
            b"actor != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            334 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"get_view\0")).as_ptr(),
        );
    };
    let mut key: view_t = view_t {
        actor: 0 as *mut pony_actor_t,
        rc: 0,
        view_rc: 0,
        blocked: false,
        deferred: false,
        color: 0,
        map: viewrefmap_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        perceived: 0 as *mut perceived_t,
    };
    key.actor = actor;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = ponyint_viewmap_get(&mut (*d).views, &mut key, &mut index);
    if view.is_null() && create as libc::c_int != 0 {
        view = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut view_t;
        memset(
            view as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<view_t>().try_into().unwrap(),
        );
        let ref mut fresh1 = (*view).actor;
        *fresh1 = actor;
        (*view).view_rc = 1 as libc::c_int as u32;
        ponyint_viewmap_putindex(&mut (*d).views, view, index);
        let ref mut fresh2 = (*d).created;
        *fresh2 = (*fresh2).wrapping_add(1);
    }
    view
}
#[c2rust::src_loc = "370:1"]
unsafe extern "C" fn apply_delta(
    mut d: *mut detector_t,
    mut view: *mut view_t,
    mut map: *mut deltamap_t,
) {
    if map.is_null() {
        return;
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut delta: *mut delta_t = 0 as *mut delta_t;
    loop {
        delta = ponyint_deltamap_next(map, &mut i);
        if delta.is_null() {
            break;
        }
        let mut actor: *mut pony_actor_t = ponyint_delta_actor(delta);
        let mut rc: usize = ponyint_delta_rc(delta);
        let mut find: *mut view_t = get_view(d, actor, rc > 0);
        if find.is_null() {
            continue;
        }
        let mut key: viewref_t = viewref_t {
            view: 0 as *mut view_t,
            rc: 0,
        };
        key.view = find;
        if rc > 0 {
            let mut index: usize = -(1 as libc::c_int) as usize;
            let mut ref_0: *mut viewref_t =
                ponyint_viewrefmap_get(&mut (*view).map, &mut key, &mut index);
            if ref_0.is_null() {
                ref_0 = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut viewref_t;
                let ref mut fresh3 = (*ref_0).view;
                *fresh3 = find;
                ponyint_viewrefmap_putindex(&mut (*view).map, ref_0, index);
                let ref mut fresh4 = (*find).view_rc;
                *fresh4 = (*fresh4).wrapping_add(1);
            }
            (*ref_0).rc = rc;
        } else {
            let mut ref_1: *mut viewref_t = ponyint_viewrefmap_remove(&mut (*view).map, &mut key);
            if !ref_1.is_null() {
                viewref_free(ref_1);
                view_free(find);
            }
        }
    }
    ponyint_deltamap_free(map);
}
#[c2rust::src_loc = "447:1"]
unsafe extern "C" fn mark_grey(
    mut d: *mut detector_t,
    mut view: *mut view_t,
    mut rc: usize,
) -> bool {
    if !(*view).blocked {
        return 0 as libc::c_int != 0;
    }
    if (*view).deferred {
        ponyint_viewmap_remove(&mut (*d).deferred, view);
        (*view).deferred = 0 as libc::c_int != 0;
    }
    let ref mut fresh5 = (*view).rc;
    *fresh5 = (*fresh5 as libc::c_ulong).wrapping_sub(rc.try_into().unwrap()) as usize as usize;
    if (*view).color as libc::c_int == COLOR_GREY as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*view).color as libc::c_int == COLOR_BLACK as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"view->color == COLOR_BLACK\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            465 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"mark_grey\0")).as_ptr(),
        );
    };
    (*view).color = COLOR_GREY as libc::c_int as u8;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn scan_grey(mut d: *mut detector_t, mut view: *mut view_t, mut rc: usize) {
    let mut ref_0: *mut viewref_t = 0 as *mut viewref_t;
    let mut head: viewref_t = {
        let mut init = viewref_t { view: view, rc: rc };
        init
    };
    let mut stack: *mut viewrefstack_t =
        ponyint_viewrefstack_push(0 as *mut viewrefstack_t, &mut head);
    while !stack.is_null() {
        stack = ponyint_viewrefstack_pop(stack, &mut ref_0);
        if mark_grey(d, (*ref_0).view, (*ref_0).rc) {
            let mut i: usize = -(1 as libc::c_int) as usize;
            let mut child: *mut viewref_t = 0 as *mut viewref_t;
            loop {
                child = ponyint_viewrefmap_next(&mut (*(*ref_0).view).map, &mut i);
                if child.is_null() {
                    break;
                }
                stack = ponyint_viewrefstack_push(stack, child);
            }
        }
    }
}
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn mark_black(
    mut view: *mut view_t,
    mut rc: usize,
    mut count: *mut libc::c_int,
) -> bool {
    if !(*view).blocked {
        if (*view).color as libc::c_int == COLOR_BLACK as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"view->color == COLOR_BLACK\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                495 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mark_black\0"))
                    .as_ptr(),
            );
        };
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh6 = (*view).rc;
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(rc.try_into().unwrap()) as usize as usize;
    if (*view).color as libc::c_int == COLOR_BLACK as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*view).color as libc::c_int == COLOR_WHITE as libc::c_int {
        *count = *count + 1 as libc::c_int;
    }
    (*view).color = COLOR_BLACK as libc::c_int as u8;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "512:1"]
unsafe extern "C" fn scan_black(mut view: *mut view_t, mut rc: usize) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ref_0: *mut viewref_t = 0 as *mut viewref_t;
    let mut head: viewref_t = {
        let mut init = viewref_t { view: view, rc: rc };
        init
    };
    let mut stack: *mut viewrefstack_t =
        ponyint_viewrefstack_push(0 as *mut viewrefstack_t, &mut head);
    while !stack.is_null() {
        stack = ponyint_viewrefstack_pop(stack, &mut ref_0);
        if mark_black((*ref_0).view, (*ref_0).rc, &mut count) {
            let mut i: usize = -(1 as libc::c_int) as usize;
            let mut child: *mut viewref_t = 0 as *mut viewref_t;
            loop {
                child = ponyint_viewrefmap_next(&mut (*(*ref_0).view).map, &mut i);
                if child.is_null() {
                    break;
                }
                stack = ponyint_viewrefstack_push(stack, child);
            }
        }
    }
    return count;
}
#[c2rust::src_loc = "537:1"]
unsafe extern "C" fn mark_white(mut view: *mut view_t, mut count: *mut libc::c_int) -> bool {
    if (*view).color as libc::c_int != COLOR_GREY as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*view).blocked as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"view->blocked\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            542 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mark_white\0")).as_ptr(),
        );
    };
    if (*view).rc > 0 {
        *count = *count - scan_black(view, 0 as libc::c_int as usize);
        return 0 as libc::c_int != 0;
    }
    if ((*view).perceived).is_null() {
    } else {
        ponyint_assert_fail(
            b"view->perceived == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            551 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mark_white\0")).as_ptr(),
        );
    };
    (*view).color = COLOR_WHITE as libc::c_int as u8;
    *count = *count + 1 as libc::c_int;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn scan_white(mut view: *mut view_t) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ref_0: *mut viewref_t = 0 as *mut viewref_t;
    let mut head: viewref_t = {
        let mut init = viewref_t {
            view: view,
            rc: 0 as libc::c_int as usize,
        };
        init
    };
    let mut stack: *mut viewrefstack_t =
        ponyint_viewrefstack_push(0 as *mut viewrefstack_t, &mut head);
    while !stack.is_null() {
        stack = ponyint_viewrefstack_pop(stack, &mut ref_0);
        if mark_white((*ref_0).view, &mut count) {
            let mut i: usize = -(1 as libc::c_int) as usize;
            let mut child: *mut viewref_t = 0 as *mut viewref_t;
            loop {
                child = ponyint_viewrefmap_next(&mut (*(*ref_0).view).map, &mut i);
                if child.is_null() {
                    break;
                }
                stack = ponyint_viewrefstack_push(stack, child);
            }
        }
    }
    return count;
}
#[c2rust::src_loc = "583:1"]
unsafe extern "C" fn collect_view(
    mut per: *mut perceived_t,
    mut view: *mut view_t,
    mut rc: usize,
    mut count: *mut libc::c_int,
) -> bool {
    if (*view).color as libc::c_int == COLOR_WHITE as libc::c_int {
        if (*view).deferred as libc::c_int == 0 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"view->deferred == false\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                587 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"collect_view\0"))
                    .as_ptr(),
            );
        };
        if ((*view).perceived).is_null() {
        } else {
            ponyint_assert_fail(
                b"view->perceived == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                588 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"collect_view\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh7 = (*view).perceived;
        *fresh7 = per;
        ponyint_viewmap_put(&mut (*per).map, view);
    }
    return mark_black(view, rc, count);
}
#[c2rust::src_loc = "611:1"]
unsafe extern "C" fn collect_white(
    mut per: *mut perceived_t,
    mut view: *mut view_t,
    mut rc: usize,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ref_0: *mut viewref_t = 0 as *mut viewref_t;
    let mut head: viewref_t = {
        let mut init = viewref_t { view: view, rc: rc };
        init
    };
    let mut stack: *mut viewrefstack_t =
        ponyint_viewrefstack_push(0 as *mut viewrefstack_t, &mut head);
    while !stack.is_null() {
        stack = ponyint_viewrefstack_pop(stack, &mut ref_0);
        if collect_view(per, (*ref_0).view, (*ref_0).rc, &mut count) {
            let mut i: usize = -(1 as libc::c_int) as usize;
            let mut child: *mut viewref_t = 0 as *mut viewref_t;
            loop {
                child = ponyint_viewrefmap_next(&mut (*(*ref_0).view).map, &mut i);
                if child.is_null() {
                    break;
                }
                stack = ponyint_viewrefstack_push(stack, child);
            }
        }
    }
    return count;
}
#[c2rust::src_loc = "636:1"]
unsafe extern "C" fn send_conf(mut ctx: *mut pony_ctx_t, mut per: *mut perceived_t) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*per).map, &mut i);
        if view.is_null() {
            break;
        }
        if ponyint_acquire_cycle_detector_critical((*view).actor) {
            if !ponyint_actor_pendingdestroy((*view).actor) {
                pony_sendi(
                    ctx,
                    (*view).actor,
                    (4294967295 as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint),
                    (*per).token as intptr_t,
                );
            }
            ponyint_release_cycle_detector_critical((*view).actor);
        }
    }
}
#[c2rust::src_loc = "663:1"]
unsafe extern "C" fn detect(
    mut ctx: *mut pony_ctx_t,
    mut d: *mut detector_t,
    mut view: *mut view_t,
) -> bool {
    if ((*view).perceived).is_null() {
    } else {
        ponyint_assert_fail(
            b"view->perceived == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            665 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"detect\0")).as_ptr(),
        );
    };
    scan_grey(d, view, 0 as libc::c_int as usize);
    let mut count: libc::c_int = scan_white(view);
    if count >= 0 as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"count >= 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            669 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"detect\0")).as_ptr(),
        );
    };
    if count == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh8 = (*d).detected;
    *fresh8 = (*fresh8).wrapping_add(1);
    let mut per: *mut perceived_t =
        ponyint_pool_alloc(1 as libc::c_int as usize) as *mut perceived_t;
    let ref mut fresh9 = (*d).next_token;
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).wrapping_add(1);
    (*per).token = fresh10;
    (*per).ack = 0 as libc::c_int as usize;
    ponyint_viewmap_init(&mut (*per).map, count as usize);
    ponyint_perceivedmap_put(&mut (*d).perceived, per);
    let mut count2: libc::c_int = collect_white(per, view, 0 as libc::c_int as usize);
    if count2 == count {
    } else {
        ponyint_assert_fail(
            b"count2 == count\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            702 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"detect\0")).as_ptr(),
        );
    };
    if ponyint_viewmap_size(&mut (*per).map) == count as usize {
    } else {
        ponyint_assert_fail(
            b"ponyint_viewmap_size(&per->map) == (size_t)count\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            703 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"detect\0")).as_ptr(),
        );
    };
    send_conf(ctx, per);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "709:1"]
unsafe extern "C" fn deferred(mut ctx: *mut pony_ctx_t, mut d: *mut detector_t) {
    let ref mut fresh11 = (*d).attempted;
    *fresh11 = (*fresh11).wrapping_add(1);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*d).deferred, &mut i);
        if view.is_null() {
            break;
        }
        if (*view).deferred as libc::c_int == 1 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"view->deferred == true\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                718 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"deferred\0")).as_ptr(),
            );
        };
        ponyint_viewmap_removeindex(&mut (*d).deferred, i);
        i = i.wrapping_sub(1);
        (*view).deferred = 0 as libc::c_int != 0;
        detect(ctx, d, view);
    }
}
#[c2rust::src_loc = "731:1"]
unsafe extern "C" fn expire(mut d: *mut detector_t, mut view: *mut view_t) {
    let mut per: *mut perceived_t = (*view).perceived;
    if per.is_null() {
        return;
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut pview: *mut view_t = 0 as *mut view_t;
    loop {
        pview = ponyint_viewmap_next(&mut (*per).map, &mut i);
        if pview.is_null() {
            break;
        }
        let ref mut fresh12 = (*pview).perceived;
        *fresh12 = 0 as *mut perceived_t;
    }
    ponyint_perceivedmap_remove(&mut (*d).perceived, per);
    perceived_free(per);
    let ref mut fresh13 = (*view).perceived;
    *fresh13 = 0 as *mut perceived_t;
}
#[c2rust::src_loc = "749:1"]
unsafe extern "C" fn collect(
    mut ctx: *mut pony_ctx_t,
    mut d: *mut detector_t,
    mut per: *mut perceived_t,
) {
    ponyint_perceivedmap_remove(&mut (*d).perceived, per);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*per).map, &mut i);
        if view.is_null() {
            break;
        }
        if !ponyint_actor_pendingdestroy((*view).actor) {
        } else {
            ponyint_assert_fail(
                b"!ponyint_actor_pendingdestroy(view->actor)\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                761 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"collect\0")).as_ptr(),
            );
        };
        if (*view).perceived == per {
        } else {
            ponyint_assert_fail(
                b"view->perceived == per\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                763 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"collect\0")).as_ptr(),
            );
        };
        if (*view).deferred {
            ponyint_viewmap_remove(&mut (*d).deferred, view);
        }
        ponyint_actor_setpendingdestroy((*view).actor);
        ponyint_actor_final(ctx, (*view).actor);
    }
    i = -(1 as libc::c_int) as usize;
    loop {
        view = ponyint_viewmap_next(&mut (*per).map, &mut i);
        if view.is_null() {
            break;
        }
        ponyint_actor_sendrelease(ctx, (*view).actor);
    }
    i = -(1 as libc::c_int) as usize;
    loop {
        view = ponyint_viewmap_next(&mut (*per).map, &mut i);
        if view.is_null() {
            break;
        }
        ponyint_actor_destroy((*view).actor);
        ponyint_viewmap_remove(&mut (*d).views, view);
        view_free(view);
    }
    let ref mut fresh14 = (*d).destroyed;
    *fresh14 = (*fresh14 as libc::c_ulong)
        .wrapping_add(ponyint_viewmap_size(&mut (*per).map).try_into().unwrap())
        as usize as usize;
    perceived_free(per);
    let ref mut fresh15 = (*d).collected;
    *fresh15 = (*fresh15).wrapping_add(1);
}
#[c2rust::src_loc = "798:1"]
unsafe extern "C" fn check_blocked(mut ctx: *mut pony_ctx_t, mut d: *mut detector_t) {
    let mut i: usize = (*d).last_checked;
    let mut total: usize = ponyint_viewmap_size(&mut (*d).views);
    let mut n: usize = 0;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*d).views, &mut i);
        if view.is_null() {
            break;
        }
        if !(*view).blocked {
            if ponyint_acquire_cycle_detector_critical((*view).actor) {
                if !ponyint_actor_pendingdestroy((*view).actor) {
                    pony_send(
                        ctx,
                        (*view).actor,
                        (4294967295 as libc::c_uint).wrapping_sub(7 as libc::c_int as libc::c_uint),
                    );
                }
                ponyint_release_cycle_detector_critical((*view).actor);
            }
        }
        n = n.wrapping_add(1);
        if n > (if total.wrapping_div((10 as libc::c_int as libc::c_ulong).try_into().unwrap())
            > (1000 as libc::c_int as libc::c_ulong).try_into().unwrap()
        {
            total.wrapping_div((10 as libc::c_int as libc::c_ulong).try_into().unwrap())
        } else {
            (1000 as libc::c_int as libc::c_ulong).try_into().unwrap()
        }) {
            break;
        }
    }
    if view.is_null() {
        (*d).last_checked = -(1 as libc::c_int) as usize;
    } else {
        (*d).last_checked = i;
    }
    deferred(ctx, d);
}
#[c2rust::src_loc = "850:1"]
unsafe extern "C" fn actor_destroyed(mut d: *mut detector_t, mut actor: *mut pony_actor_t) {
    let mut view: *mut view_t = get_view(d, actor, 0 as libc::c_int != 0);
    if !view.is_null() {
        ponyint_viewmap_remove(&mut (*d).views, view);
        view_free(view);
    }
}
#[c2rust::src_loc = "868:1"]
unsafe extern "C" fn block(
    mut d: *mut detector_t,
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut rc: usize,
    mut map: *mut deltamap_t,
) {
    if rc == 0 {
        let mut view: *mut view_t = get_view(d, actor, 0 as libc::c_int != 0);
        if !view.is_null() {
            if (*view).deferred {
                ponyint_viewmap_remove(&mut (*d).deferred, view);
            }
            expire(d, view);
            ponyint_viewmap_remove(&mut (*d).views, view);
            view_free(view);
        }
        if !map.is_null() {
            ponyint_deltamap_free(map);
        }
        if ponyint_actor_pendingdestroy(actor) as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"ponyint_actor_pendingdestroy(actor)\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                    as *const libc::c_char,
                906 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"block\0")).as_ptr(),
            );
        };
        ponyint_actor_final(ctx, actor);
        ponyint_actor_sendrelease(ctx, actor);
        ponyint_actor_destroy(actor);
        let ref mut fresh16 = (*d).destroyed;
        *fresh16 = (*fresh16).wrapping_add(1);
        return;
    }
    let mut view_0: *mut view_t = get_view(d, actor, 1 as libc::c_int != 0);
    (*view_0).rc = rc;
    if !map.is_null() {
        apply_delta(d, view_0, map);
    }
    (*view_0).blocked = 1 as libc::c_int != 0;
    expire(d, view_0);
    if !(*view_0).deferred {
        ponyint_viewmap_put(&mut (*d).deferred, view_0);
        (*view_0).deferred = 1 as libc::c_int != 0;
    }
}
#[c2rust::src_loc = "954:1"]
unsafe extern "C" fn unblock(mut d: *mut detector_t, mut actor: *mut pony_actor_t) {
    let mut key: view_t = view_t {
        actor: 0 as *mut pony_actor_t,
        rc: 0,
        view_rc: 0,
        blocked: false,
        deferred: false,
        color: 0,
        map: viewrefmap_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        perceived: 0 as *mut perceived_t,
    };
    key.actor = actor;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = ponyint_viewmap_get(&mut (*d).views, &mut key, &mut index);
    if !view.is_null() {
    } else {
        ponyint_assert_fail(
            b"view != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            962 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"unblock\0")).as_ptr(),
        );
    };
    (*view).blocked = 0 as libc::c_int != 0;
    if (*view).deferred {
        ponyint_viewmap_remove(&mut (*d).deferred, view);
        (*view).deferred = 0 as libc::c_int != 0;
    }
    expire(d, view);
}
#[c2rust::src_loc = "978:1"]
unsafe extern "C" fn ack(mut ctx: *mut pony_ctx_t, mut d: *mut detector_t, mut token: usize) {
    let mut key: perceived_t = perceived_t {
        token: 0,
        ack: 0,
        map: viewmap_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
    };
    key.token = token;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut per: *mut perceived_t =
        ponyint_perceivedmap_get(&mut (*d).perceived, &mut key, &mut index);
    if per.is_null() {
        return;
    }
    let ref mut fresh17 = (*per).ack;
    *fresh17 = (*fresh17).wrapping_add(1);
    if (*per).ack == ponyint_viewmap_size(&mut (*per).map) {
        collect(ctx, d, per);
        return;
    }
}
#[c2rust::src_loc = "1001:1"]
unsafe extern "C" fn final_0(mut ctx: *mut pony_ctx_t, mut self_0: *mut pony_actor_t) {
    let mut stack: *mut pendingdestroystack_t = 0 as *mut pendingdestroystack_t;
    let mut msg: *mut pony_msg_t = 0 as *mut pony_msg_t;
    loop {
        loop {
            msg = ponyint_actor_messageq_pop(&mut (*self_0).q);
            if msg.is_null() {
                break;
            }
            if (*msg).id
                == (4294967295 as libc::c_uint).wrapping_sub(6 as libc::c_int as libc::c_uint)
            {
                let mut m: *mut block_msg_t = msg as *mut block_msg_t;
                if !((*m).delta).is_null() {
                    ponyint_deltamap_free((*m).delta);
                }
                if !ponyint_actor_pendingdestroy((*m).actor) {
                    ponyint_actor_setpendingdestroy((*m).actor);
                    ponyint_actor_final(ctx, (*m).actor);
                    stack = ponyint_pendingdestroystack_push(stack, (*m).actor);
                }
            }
        }
        if ponyint_messageq_markempty(&mut (*self_0).q) {
            break;
        }
    }
    let mut d: *mut detector_t = self_0 as *mut detector_t;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*d).views, &mut i);
        if view.is_null() {
            break;
        }
        if !ponyint_actor_pendingdestroy((*view).actor) {
            ponyint_actor_setpendingdestroy((*view).actor);
            ponyint_actor_final(ctx, (*view).actor);
            stack = ponyint_pendingdestroystack_push(stack, (*view).actor);
        }
    }
    let mut actor: *mut pony_actor_t = 0 as *mut pony_actor_t;
    while !stack.is_null() {
        stack = ponyint_pendingdestroystack_pop(stack, &mut actor);
        ponyint_actor_destroy(actor);
    }
    i = -(1 as libc::c_int) as usize;
    loop {
        view = ponyint_viewmap_next(&mut (*d).deferred, &mut i);
        if view.is_null() {
            break;
        }
        ponyint_viewmap_removeindex(&mut (*d).deferred, i);
        i = i.wrapping_sub(1);
    }
    ponyint_viewmap_destroy(&mut (*d).deferred);
    ponyint_viewmap_destroy(&mut (*d).views);
    ponyint_perceivedmap_destroy(&mut (*d).perceived);
}
#[c2rust::src_loc = "1082:1"]
unsafe extern "C" fn dump_view(mut view: *mut view_t) {
    printf(
        b"%p: %zu/%zu (%s)%s%s\n\0" as *const u8 as *const libc::c_char,
        (*view).actor,
        (*view).rc,
        (*(*view).actor).gc.rc,
        if (*view).blocked as libc::c_int != 0 {
            b"blocked\0" as *const u8 as *const libc::c_char
        } else {
            b"unblocked\0" as *const u8 as *const libc::c_char
        },
        if (*view).rc == (*(*view).actor).gc.rc {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" ERROR\0" as *const u8 as *const libc::c_char
        },
        if ((*(*view).actor).gc.delta).is_null() {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" DELTA\0" as *const u8 as *const libc::c_char
        },
    );
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut p: *mut viewref_t = 0 as *mut viewref_t;
    let mut aref: *mut actorref_t = 0 as *mut actorref_t;
    let mut index: usize = -(1 as libc::c_int) as usize;
    loop {
        p = ponyint_viewrefmap_next(&mut (*view).map, &mut i);
        if p.is_null() {
            break;
        }
        aref = ponyint_actormap_getactor(
            &mut (*(*view).actor).gc.foreign,
            (*(*p).view).actor,
            &mut index,
        );
        if !aref.is_null() {
            printf(
                b"\t%p: %zu/%zu %s\n\0" as *const u8 as *const libc::c_char,
                (*(*p).view).actor,
                (*p).rc,
                (*aref).rc,
                if (*p).rc == (*aref).rc {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"ERROR\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            printf(
                b"\t%p: %zu ERROR\n\0" as *const u8 as *const libc::c_char,
                (*(*p).view).actor,
                (*p).rc,
            );
        }
    }
    if ponyint_actormap_size(&mut (*(*view).actor).gc.foreign)
        != ponyint_viewrefmap_size(&mut (*view).map)
    {
        printf(b"\t--- ERROR\n\0" as *const u8 as *const libc::c_char);
        i = -(1 as libc::c_int) as usize;
        loop {
            aref = ponyint_actormap_next(&mut (*(*view).actor).gc.foreign, &mut i);
            if aref.is_null() {
                break;
            }
            printf(
                b"\t%p: %zu\n\0" as *const u8 as *const libc::c_char,
                (*aref).actor,
                (*aref).rc,
            );
        }
    }
}
#[c2rust::src_loc = "1123:1"]
unsafe extern "C" fn dump_views() {
    let mut d: *mut detector_t = cycle_detector as *mut detector_t;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*d).views, &mut i);
        if view.is_null() {
            break;
        }
        dump_view(view);
    }
}
#[c2rust::src_loc = "1135:1"]
unsafe extern "C" fn check_view(mut d: *mut detector_t, mut view: *mut view_t) {
    if !((*view).perceived).is_null() {
        printf(
            b"%p: in a cycle\n\0" as *const u8 as *const libc::c_char,
            (*view).actor,
        );
        return;
    }
    scan_grey(d, view, 0 as libc::c_int as usize);
    let mut count: libc::c_int = scan_white(view);
    if count >= 0 as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"count >= 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            1145 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"check_view\0")).as_ptr(),
        );
    };
    printf(
        b"%p: %s\n\0" as *const u8 as *const libc::c_char,
        (*view).actor,
        if count > 0 as libc::c_int {
            b"COLLECTABLE\0" as *const u8 as *const libc::c_char
        } else {
            b"uncollectable\0" as *const u8 as *const libc::c_char
        },
    );
}
#[c2rust::src_loc = "1150:1"]
unsafe extern "C" fn check_views() {
    let mut d: *mut detector_t = cycle_detector as *mut detector_t;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut view: *mut view_t = 0 as *mut view_t;
    loop {
        view = ponyint_viewmap_next(&mut (*d).views, &mut i);
        if view.is_null() {
            break;
        }
        check_view(d, view);
    }
}
#[c2rust::src_loc = "1164:1"]
unsafe extern "C" fn cycle_dispatch(
    mut ctx: *mut pony_ctx_t,
    mut self_0: *mut pony_actor_t,
    mut msg: *mut pony_msg_t,
) {
    let mut d: *mut detector_t = self_0 as *mut detector_t;
    match (*msg).id {
        4294967285 => {
            check_blocked(ctx, d);
        }
        4294967286 => {
            let mut m: *mut pony_msgp_t = msg as *mut pony_msgp_t;
            actor_destroyed(d, (*m).p as *mut pony_actor_t);
        }
        4294967289 => {
            let mut m_0: *mut block_msg_t = msg as *mut block_msg_t;
            block(d, ctx, (*m_0).actor, (*m_0).rc, (*m_0).delta);
        }
        4294967290 => {
            let mut m_1: *mut pony_msgp_t = msg as *mut pony_msgp_t;
            unblock(d, (*m_1).p as *mut pony_actor_t);
        }
        4294967294 => {
            let mut m_2: *mut pony_msgi_t = msg as *mut pony_msgi_t;
            ack(ctx, d, (*m_2).i as usize);
        }
        _ => {
            dump_views();
            check_views();
        }
    };
}
#[c2rust::src_loc = "1222:20"]
static mut cycle_type: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<detector_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: None,
            serialise: None,
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: Some(
                cycle_dispatch
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut pony_actor_t,
                        *mut pony_msg_t,
                    ) -> (),
            ),
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
#[c2rust::src_loc = "1243:1"]
pub unsafe extern "C" fn ponyint_cycle_create(mut ctx: *mut pony_ctx_t, mut detect_interval: u32) {
    if detect_interval > 1000 as libc::c_int as libc::c_uint {
        detect_interval = 1000 as libc::c_int as u32;
    }
    if detect_interval < 10 as libc::c_int as libc::c_uint {
        detect_interval = 10 as libc::c_int as u32;
    }
    cycle_detector = 0 as *mut pony_actor_t;
    cycle_detector = pony_create(ctx, &cycle_type, 0 as libc::c_int != 0);
    ponyint_actor_setsystem(cycle_detector);
    let mut d: *mut detector_t = cycle_detector as *mut detector_t;
    (*d).detect_interval =
        detect_interval.wrapping_mul(2000000 as libc::c_int as libc::c_uint) as usize;
    (*d).last_checked = -(1 as libc::c_int) as usize;
}
#[no_mangle]
#[c2rust::src_loc = "1268:1"]
pub unsafe extern "C" fn ponyint_cycle_check_blocked(mut tsc: u64, mut tsc2: u64) -> bool {
    let mut d: *mut detector_t = cycle_detector as *mut detector_t;
    let mut diff: u64 = ponyint_cpu_tick_diff(tsc, tsc2);
    if diff > (*d).detect_interval as libc::c_ulonglong {
        let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
        pony_send(
            ctx,
            cycle_detector,
            (4294967295 as libc::c_uint).wrapping_sub(10 as libc::c_int as libc::c_uint),
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1284:1"]
pub unsafe extern "C" fn ponyint_cycle_actor_destroyed(mut actor: *mut pony_actor_t) {
    if !cycle_detector.is_null() && !ponyint_is_cycle(actor) {
        let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
        pony_sendp(
            ctx,
            cycle_detector,
            (4294967295 as libc::c_uint).wrapping_sub(9 as libc::c_int as libc::c_uint),
            actor as *mut libc::c_void,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1295:1"]
pub unsafe extern "C" fn ponyint_cycle_block(mut actor: *mut pony_actor_t, mut gc: *mut gc_t) {
    if &mut (*actor).gc as *mut gc_t == gc {
    } else {
        ponyint_assert_fail(
            b"&actor->gc == gc\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            1297 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"ponyint_cycle_block\0"))
                .as_ptr(),
        );
    };
    let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
    let mut m: *mut block_msg_t = pony_alloc_msg(
        1 as libc::c_int as u32,
        (4294967295 as libc::c_uint).wrapping_sub(6 as libc::c_int as libc::c_uint),
    ) as *mut block_msg_t;
    let ref mut fresh18 = (*m).actor;
    *fresh18 = actor;
    (*m).rc = ponyint_gc_rc(gc);
    let ref mut fresh19 = (*m).delta;
    *fresh19 = ponyint_gc_delta(gc);
    if ((*gc).delta).is_null() {
    } else {
        ponyint_assert_fail(
            b"gc->delta == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.c\0" as *const u8
                as *const libc::c_char,
            1311 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"ponyint_cycle_block\0"))
                .as_ptr(),
        );
    };
    pony_sendv(
        ctx,
        cycle_detector,
        &mut (*m).msg,
        &mut (*m).msg,
        0 as libc::c_int != 0,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1316:1"]
pub unsafe extern "C" fn ponyint_cycle_unblock(mut actor: *mut pony_actor_t) {
    let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
    pony_sendp(
        ctx,
        cycle_detector,
        (4294967295 as libc::c_uint).wrapping_sub(5 as libc::c_int as libc::c_uint),
        actor as *mut libc::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1322:1"]
pub unsafe extern "C" fn ponyint_cycle_ack(mut token: usize) {
    let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
    pony_sendi(
        ctx,
        cycle_detector,
        (4294967295 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint),
        token as intptr_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1328:1"]
pub unsafe extern "C" fn ponyint_cycle_terminate() {
    let mut ctx: *mut pony_ctx_t = ponyint_sched_get_inject_context();
    ponyint_become(ctx, cycle_detector);
    final_0(ctx, cycle_detector);
    ponyint_destroy(ctx, cycle_detector);
    cycle_detector = 0 as *mut pony_actor_t;
}
#[no_mangle]
#[c2rust::src_loc = "1338:1"]
pub unsafe extern "C" fn ponyint_is_cycle(mut actor: *mut pony_actor_t) -> bool {
    return actor == cycle_detector;
}
