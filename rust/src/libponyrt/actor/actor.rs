use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:3"]
pub mod _types_h {
    #[c2rust::src_loc = "51:1"]
    pub type __darwin_intptr_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:3"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:3"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h:3"]
pub mod _intptr_t_h {
    #[c2rust::src_loc = "32:1"]
    pub type intptr_t = __darwin_intptr_t;
    use super::_types_h::__darwin_intptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:3"]
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
    #[c2rust::src_loc = "31:16"]
    pub struct actorstats_t {
        pub heap_mem_allocated: usize,
        pub heap_mem_used: usize,
        pub heap_num_allocated: usize,
        pub heap_realloc_counter: usize,
        pub heap_alloc_counter: usize,
        pub heap_free_counter: usize,
        pub heap_gc_counter: usize,
        pub system_cpu: usize,
        pub app_cpu: usize,
        pub gc_mark_cpu: usize,
        pub gc_sweep_cpu: usize,
        pub messages_sent_counter: usize,
        pub system_messages_processed_counter: usize,
        pub app_messages_processed_counter: usize,
        pub foreign_actormap_objectmap_mem_used: usize,
        pub foreign_actormap_objectmap_mem_allocated: usize,
    }
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:3"]
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
    use super::actor_h::pony_actor_t;
    use super::actormap_h::{actormap_t, actorref_t};
    use super::delta_h::deltamap_t;
    use super::objectmap_h::objectmap_t;
    use super::scheduler_h::pony_ctx_t;

    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
        #[c2rust::src_loc = "55:1"]
        pub fn ponyint_gc_createactor(current: *mut pony_actor_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_gc_done(gc: *mut gc_t);
        #[c2rust::src_loc = "81:1"]
        pub fn ponyint_gc_destroy(gc: *mut gc_t);
        #[c2rust::src_loc = "67:1"]
        pub fn ponyint_gc_sendrelease(ctx: *mut pony_ctx_t, gc: *mut gc_t);
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_gc_release(gc: *mut gc_t, aref: *mut actorref_t) -> bool;
        #[c2rust::src_loc = "71:1"]
        pub fn ponyint_gc_acquire(gc: *mut gc_t, aref: *mut actorref_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/delta.h:3"]
pub mod delta_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:35"]
    pub struct deltamap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:3"]
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:3"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
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
        pub used: usize,
        pub next_gc: usize,
    }
    #[c2rust::src_loc = "32:3"]
    pub const TRACK_NO_FINALISERS: C2RustUnnamed_0 = 0;
    #[c2rust::src_loc = "33:3"]
    pub const TRACK_ALL_FINALISERS: C2RustUnnamed_0 = 4294967295;
    #[c2rust::src_loc = "30:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    use super::actor_h::pony_actor_t;

    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_heap_init(heap: *mut heap_t);
        #[c2rust::src_loc = "49:3"]
        pub fn ponyint_heap_alloc(
            actor: *mut pony_actor_t,
            heap: *mut heap_t,
            size: usize,
            track_finalisers_mask: u32,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "54:1"]
        pub fn ponyint_heap_alloc_small(
            actor: *mut pony_actor_t,
            heap: *mut heap_t,
            sizeclass: u32,
            track_finalisers_mask: u32,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_heap_alloc_large(
            actor: *mut pony_actor_t,
            heap: *mut heap_t,
            size: usize,
            track_finalisers_mask: u32,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "63:1"]
        pub fn ponyint_heap_realloc(
            actor: *mut pony_actor_t,
            heap: *mut heap_t,
            p: *mut libc::c_void,
            size: usize,
            copy: usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "96:1"]
        pub fn ponyint_heap_endgc(heap: *mut heap_t);
        #[c2rust::src_loc = "72:1"]
        pub fn ponyint_heap_startgc(heap: *mut heap_t) -> bool;
        #[c2rust::src_loc = "44:1"]
        pub fn ponyint_heap_destroy(heap: *mut heap_t);
        #[c2rust::src_loc = "46:1"]
        pub fn ponyint_heap_final(heap: *mut heap_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:3"]
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
        #[c2rust::src_loc = "19:1"]
        pub fn ponyint_messageq_init(q: *mut messageq_t);
        #[c2rust::src_loc = "23:1"]
        pub fn ponyint_actor_messageq_push(
            q: *mut messageq_t,
            first: *mut pony_msg_t,
            last: *mut pony_msg_t,
        ) -> bool;
        #[c2rust::src_loc = "31:1"]
        pub fn ponyint_actor_messageq_push_single(
            q: *mut messageq_t,
            first: *mut pony_msg_t,
            last: *mut pony_msg_t,
        ) -> bool;
        #[c2rust::src_loc = "65:1"]
        pub fn ponyint_messageq_markempty(q: *mut messageq_t) -> bool;
        #[c2rust::src_loc = "67:1"]
        pub fn ponyint_messageq_isempty(q: *mut messageq_t) -> bool;
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_messageq_destroy(q: *mut messageq_t, maybe_non_empty: bool);
        #[c2rust::src_loc = "39:1"]
        pub fn ponyint_actor_messageq_pop(q: *mut messageq_t) -> *mut pony_msg_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:3"]
pub mod pony_h {
    #[c2rust::src_loc = "46:8"]
    pub use crate::libponyrt::pony_msg_t;
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
    #[derive(Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:16"]
    pub struct pony_msgi_t {
        pub msg: pony_msg_t,
        pub i: intptr_t,
    }
    #[derive(Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct pony_msgp_t {
        pub msg: pony_msg_t,
        pub p: *mut libc::c_void,
    }
    use super::_intptr_t_h::intptr_t;

    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;

    extern "C" {
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:3"]
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

    use super::actor_h::pony_actor_t;
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;

    use super::pony_h::pony_type_t;
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
    #[c2rust::src_loc = "84:8"]
    pub use crate::libponyrt::sched::scheduler::scheduler_t;

    extern "C" {
        #[c2rust::src_loc = "123:1"]
        pub fn ponyint_sched_add(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "125:1"]
        pub fn ponyint_sched_mute(
            ctx: *mut pony_ctx_t,
            sender: *mut pony_actor_t,
            recv: *mut pony_actor_t,
        );
        #[c2rust::src_loc = "127:1"]
        pub fn ponyint_sched_start_global_unmute(from: u32, actor: *mut pony_actor_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:3"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:3"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:3"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:3"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {

    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_pool_index(size: usize) -> usize;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.h:7"]
pub mod cycle_h {
    use super::actor_h::pony_actor_t;
    use super::gc_h::gc_t;

    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_cycle_block(actor: *mut pony_actor_t, gc: *mut gc_t);
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_cycle_unblock(actor: *mut pony_actor_t);
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_cycle_ack(token: usize);
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_is_cycle(actor: *mut pony_actor_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.h:8"]
pub mod trace_h {
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn ponyint_gc_mark(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "11:1"]
        pub fn ponyint_mark_done(ctx: *mut pony_ctx_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:9"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:11"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:12"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "6:1"]
        pub fn macro__DTRACE_ENABLED(_: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
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
pub use self::actor_h::{actorstats_t, pony_actor_t};
pub use self::actormap_h::{actormap_t, actorref_t};
use self::atomics_h::f__atomic_thread_fence;
use self::cycle_h::{
    ponyint_cycle_ack, ponyint_cycle_block, ponyint_cycle_unblock, ponyint_is_cycle,
};
pub use self::delta_h::deltamap_t;
use self::dtrace_h::{macro__DTRACE, macro__DTRACE_ENABLED};
pub use self::gc_h::{
    gc_t, gcstack_t, ponyint_gc_acquire, ponyint_gc_createactor, ponyint_gc_destroy,
    ponyint_gc_done, ponyint_gc_release, ponyint_gc_sendrelease,
};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{
    chunk_t, heap_t, ponyint_heap_alloc, ponyint_heap_alloc_large, ponyint_heap_alloc_small,
    ponyint_heap_destroy, ponyint_heap_endgc, ponyint_heap_final, ponyint_heap_init,
    ponyint_heap_realloc, ponyint_heap_startgc, C2RustUnnamed_0, TRACK_ALL_FINALISERS,
    TRACK_NO_FINALISERS,
};
pub use self::messageq_h::{
    messageq_t, ponyint_actor_messageq_pop, ponyint_actor_messageq_push,
    ponyint_actor_messageq_push_single, ponyint_messageq_destroy, ponyint_messageq_init,
    ponyint_messageq_isempty, ponyint_messageq_markempty,
};
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
pub use self::pony_h::{
    _pony_type_t, pony_ctx, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_msgi_t, pony_msgp_t, pony_serialise_fn,
    pony_trace_fn, pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free_size, ponyint_pool_index,
};
pub use self::scheduler_h::{
    pony_ctx_t, ponyint_sched_add, ponyint_sched_mute, ponyint_sched_start_global_unmute,
    scheduler_t, trace_actor_fn, trace_object_fn,
};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
use self::string_h::memset;
use self::trace_h::{ponyint_gc_mark, ponyint_mark_done};
#[c2rust::src_loc = "46:3"]
pub const SYNC_FLAG_MUTED: C2RustUnnamed_2 = 8;
#[c2rust::src_loc = "45:3"]
pub const SYNC_FLAG_UNDER_PRESSURE: C2RustUnnamed_2 = 4;
#[c2rust::src_loc = "44:3"]
pub const SYNC_FLAG_OVERLOADED: C2RustUnnamed_2 = 2;
#[c2rust::src_loc = "43:3"]
pub const SYNC_FLAG_PENDINGDESTROY: C2RustUnnamed_2 = 1;
#[c2rust::src_loc = "37:3"]
pub const FLAG_CD_CONTACTED: C2RustUnnamed_1 = 16;
#[c2rust::src_loc = "34:3"]
pub const FLAG_BLOCKED_SENT: C2RustUnnamed_1 = 2;
#[c2rust::src_loc = "38:3"]
pub const FLAG_RC_OVER_ZERO_SEEN: C2RustUnnamed_1 = 32;
#[c2rust::src_loc = "33:3"]
pub const FLAG_BLOCKED: C2RustUnnamed_1 = 1;
#[c2rust::src_loc = "35:3"]
pub const FLAG_SYSTEM: C2RustUnnamed_1 = 4;
#[c2rust::src_loc = "31:1"]
pub type C2RustUnnamed_1 = libc::c_uint;
#[c2rust::src_loc = "36:3"]
pub const FLAG_UNSCHEDULED: C2RustUnnamed_1 = 8;
#[c2rust::src_loc = "41:1"]
pub type C2RustUnnamed_2 = libc::c_uint;
#[c2rust::src_loc = "29:13"]
static mut actor_noblock: bool = 0 as libc::c_int != 0;
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn has_sync_flag_any(mut actor: *mut pony_actor_t, mut check_flags: u8) -> bool {
    let mut flags: u8 = { ::core::intrinsics::atomic_load_acq(&mut (*actor).sync_flags) };
    return flags as libc::c_int & check_flags as libc::c_int != 0 as libc::c_int;
}
#[c2rust::src_loc = "94:1"]
unsafe extern "C" fn has_sync_flag(mut actor: *mut pony_actor_t, mut flag: u8) -> bool {
    has_sync_flag_any(actor, flag)
}
#[c2rust::src_loc = "99:1"]
unsafe extern "C" fn set_sync_flag(mut actor: *mut pony_actor_t, mut flag: u8) {
    let mut flags: u8 = { ::core::intrinsics::atomic_load_acq(&mut (*actor).sync_flags) };
    ({
        ::core::intrinsics::atomic_store_rel(
            &mut (*actor).sync_flags,
            (flags as libc::c_int | flag as libc::c_int) as u8,
        );
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn unset_sync_flag(mut actor: *mut pony_actor_t, mut flag: u8) {
    let mut flags: u8 = { ::core::intrinsics::atomic_load_acq(&mut (*actor).sync_flags) };
    ({
        ::core::intrinsics::atomic_store_rel(
            &mut (*actor).sync_flags,
            (flags as libc::c_int & !(flag as libc::c_int) as u8 as libc::c_int) as u8,
        );
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn has_internal_flag(mut actor: *mut pony_actor_t, mut flag: u8) -> bool {
    return (*actor).internal_flags as libc::c_int & flag as libc::c_int != 0 as libc::c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn set_internal_flag(mut actor: *mut pony_actor_t, mut flag: u8) {
    (*actor).internal_flags = ((*actor).internal_flags as libc::c_int | flag as libc::c_int) as u8;
}
#[c2rust::src_loc = "124:1"]
unsafe extern "C" fn unset_internal_flag(mut actor: *mut pony_actor_t, mut flag: u8) {
    (*actor).internal_flags = ((*actor).internal_flags as libc::c_int
        & !(flag as libc::c_int) as u8 as libc::c_int) as u8;
}
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn mute_actor(mut actor: *mut pony_actor_t) {
    set_sync_flag(actor, SYNC_FLAG_MUTED as libc::c_int as u8);
    macro__DTRACE(
        b"ACTOR_MUTED\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn ponyint_unmute_actor(mut actor: *mut pony_actor_t) {
    unset_sync_flag(actor, SYNC_FLAG_MUTED as libc::c_int as u8);
    macro__DTRACE(
        b"ACTOR_UNMUTED\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn triggers_muting(mut actor: *mut pony_actor_t) -> bool {
    return has_sync_flag_any(
        actor,
        (SYNC_FLAG_OVERLOADED as libc::c_int
            | SYNC_FLAG_UNDER_PRESSURE as libc::c_int
            | SYNC_FLAG_MUTED as libc::c_int) as u8,
    );
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn actor_setoverloaded(mut actor: *mut pony_actor_t) {
    if !ponyint_is_cycle(actor) {
    } else {
        ponyint_assert_fail(
            b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            172 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"actor_setoverloaded\0"))
                .as_ptr(),
        );
    };
    set_sync_flag(actor, SYNC_FLAG_OVERLOADED as libc::c_int as u8);
    macro__DTRACE(
        b"ACTOR_OVERLOADED\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn actor_unsetoverloaded(mut actor: *mut pony_actor_t) {
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    unset_sync_flag(actor, SYNC_FLAG_OVERLOADED as libc::c_int as u8);
    macro__DTRACE(
        b"ACTOR_OVERLOADED_CLEARED\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
    if !has_sync_flag(actor, SYNC_FLAG_UNDER_PRESSURE as libc::c_int as u8) {
        ponyint_sched_start_global_unmute((*(*ctx).scheduler).index as u32, actor);
    }
}
#[c2rust::src_loc = "188:1"]
unsafe extern "C" fn maybe_mark_should_mute(mut ctx: *mut pony_ctx_t, mut to: *mut pony_actor_t) {
    if !((*ctx).current).is_null() {
        if triggers_muting(to) as libc::c_int != 0
            && !has_sync_flag_any(
                (*ctx).current,
                (SYNC_FLAG_OVERLOADED as libc::c_int | SYNC_FLAG_UNDER_PRESSURE as libc::c_int)
                    as u8,
            )
            && (*ctx).current != to
        {
            ponyint_sched_mute(ctx, (*ctx).current, to);
        }
    }
}
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn well_formed_msg_chain(
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    if first.is_null()
        || last.is_null()
        || !({ ::core::intrinsics::atomic_load_relaxed(&mut (*last).next as *mut *mut pony_msg_t) })
            .is_null()
    {
        return 0 as libc::c_int != 0;
    }
    let mut m1: *mut pony_msg_t = first;
    let mut m2: *mut pony_msg_t = first;
    while !m1.is_null() && !m2.is_null() {
        if m2 == last {
            return 1 as libc::c_int != 0;
        }
        m2 = ::core::intrinsics::atomic_load_relaxed(&mut (*m2).next);
        if m2 == last {
            return 1 as libc::c_int != 0;
        }
        if m2.is_null() {
            return 0 as libc::c_int != 0;
        }
        m1 = ::core::intrinsics::atomic_load_relaxed(&mut (*m1).next);
        m2 = ::core::intrinsics::atomic_load_relaxed(&mut (*m2).next);
        if m1 == m2 {
            return 0 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn try_gc(mut ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    if !ponyint_heap_startgc(&mut (*actor).heap) {
        return;
    }
    macro__DTRACE(
        b"GC_START\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
    ponyint_gc_mark(ctx);
    if ((*(*actor).type_0).trace).is_some() {
        ((*(*actor).type_0).trace).expect("non-null function pointer")(
            ctx,
            actor as *mut libc::c_void,
        );
    }
    ponyint_mark_done(ctx);
    ponyint_heap_endgc(&mut (*actor).heap);
    macro__DTRACE(
        b"GC_END\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn send_unblock(mut actor: *mut pony_actor_t) {
    unset_internal_flag(
        actor,
        (FLAG_BLOCKED as libc::c_int | FLAG_BLOCKED_SENT as libc::c_int) as u8,
    );
    ponyint_cycle_unblock(actor);
}
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn send_block(mut ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    if (*ctx).current == actor {
    } else {
        ponyint_assert_fail(
            b"ctx->current == actor\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            300 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"send_block\0")).as_ptr(),
        );
    };
    pony_triggergc(ctx);
    try_gc(ctx, actor);
    set_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8);
    set_internal_flag(actor, FLAG_CD_CONTACTED as libc::c_int as u8);
    ponyint_cycle_block(actor, &mut (*actor).gc);
}
#[c2rust::src_loc = "319:1"]
unsafe extern "C" fn handle_message(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut msg: *mut pony_msg_t,
) -> bool {
    match (*msg).id {
        4294967291 => {
            if !ponyint_is_cycle(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    335 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut m: *mut pony_msgp_t = msg as *mut pony_msgp_t;
            if ponyint_gc_acquire(&mut (*actor).gc, (*m).p as *mut actorref_t) as libc::c_int != 0
                && has_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8) as libc::c_int
                    != 0
            {
                send_unblock(actor);
            }
            return 0 as libc::c_int != 0;
        }
        4294967292 => {
            if !ponyint_is_cycle(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    362 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut m_0: *mut pony_msgp_t = msg as *mut pony_msgp_t;
            if ponyint_gc_release(&mut (*actor).gc, (*m_0).p as *mut actorref_t) as libc::c_int != 0
                && has_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8) as libc::c_int
                    != 0
            {
                send_unblock(actor);
            }
            return 0 as libc::c_int != 0;
        }
        4294967294 => {
            if ponyint_is_cycle(actor) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    389 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 0 as libc::c_int != 0;
        }
        4294967293 => {
            if !ponyint_is_cycle(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    402 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            if has_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8) {
                let mut m_1: *mut pony_msgi_t = msg as *mut pony_msgi_t;
                ponyint_cycle_ack((*m_1).i as usize);
            }
            return 0 as libc::c_int != 0;
        }
        4294967288 => {
            if !ponyint_actor_pendingdestroy(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_actor_pendingdestroy(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    424 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            if !ponyint_is_cycle(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    426 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            if has_internal_flag(actor, FLAG_BLOCKED as libc::c_int as u8) as libc::c_int != 0
                && !has_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8)
                && (*actor).gc.rc > 0
            {
                send_block(ctx, actor);
            }
            return 0 as libc::c_int != 0;
        }
        4294967289 => {
            if ponyint_is_cycle(actor) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    447 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 0 as libc::c_int != 0;
        }
        4294967290 => {
            if ponyint_is_cycle(actor) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    460 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 0 as libc::c_int != 0;
        }
        4294967286 => {
            if ponyint_is_cycle(actor) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    473 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 0 as libc::c_int != 0;
        }
        4294967285 => {
            if ponyint_is_cycle(actor) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    486 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 0 as libc::c_int != 0;
        }
        _ => {
            if !ponyint_is_cycle(actor) {
            } else {
                ponyint_assert_fail(
                    b"!ponyint_is_cycle(actor)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                        as *const u8 as *const libc::c_char,
                    499 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"handle_message\0",
                    ))
                    .as_ptr(),
                );
            };
            if has_internal_flag(actor, FLAG_BLOCKED_SENT as libc::c_int as u8) {
                send_unblock(actor);
            }
            macro__DTRACE(
                b"ACTOR_MSG_RUN\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                b"msg->id\0" as *const u8 as *const libc::c_char,
            );
            ((*(*actor).type_0).dispatch).expect("non-null function pointer")(ctx, actor, msg);
            return 1 as libc::c_int != 0;
        }
    };
}
#[c2rust::src_loc = "514:1"]
unsafe extern "C" fn maybe_should_mute(mut actor: *mut pony_actor_t) -> bool {
    if (*actor).muted > 0 {
        mute_actor(actor);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "543:1"]
unsafe extern "C" fn batch_limit_reached(mut actor: *mut pony_actor_t, mut polling: bool) -> bool {
    if !has_sync_flag(actor, SYNC_FLAG_OVERLOADED as libc::c_int as u8) && !polling {
        actor_setoverloaded(actor);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "557:1"]
pub unsafe extern "C" fn ponyint_actor_run(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
    mut polling: bool,
) -> bool {
    if !has_sync_flag(actor, SYNC_FLAG_MUTED as libc::c_int as u8) {
    } else {
        ponyint_assert_fail(
            b"!has_sync_flag(actor, SYNC_FLAG_MUTED)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            559 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_actor_run\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh0 = (*ctx).current;
    *fresh0 = actor;
    let mut batch: usize = 100 as libc::c_int as usize;
    let mut msg: *mut pony_msg_t = 0 as *mut pony_msg_t;
    let mut app: usize = 0;
    if !actor_noblock && (*actor).gc.rc > 0 {
        set_internal_flag(actor, FLAG_RC_OVER_ZERO_SEEN as libc::c_int as u8);
    }
    let mut head: *mut pony_msg_t = { ::core::intrinsics::atomic_load_acq(&mut (*actor).q.head) };
    loop {
        msg = ponyint_actor_messageq_pop(&mut (*actor).q);
        if msg.is_null() {
            break;
        }
        let mut app_msg: bool = handle_message(ctx, actor, msg);
        if !actor_noblock && (*actor).gc.rc > 0 {
            set_internal_flag(actor, FLAG_RC_OVER_ZERO_SEEN as libc::c_int as u8);
        }
        if app_msg {
            app = app.wrapping_add(1);
            try_gc(ctx, actor);
            if maybe_should_mute(actor) {
                return 0 as libc::c_int != 0;
            }
            if app == batch || polling as libc::c_int != 0 {
                return batch_limit_reached(actor, polling);
            }
        }
        if msg == head {
            break;
        }
    }
    if app < batch {
    } else {
        ponyint_assert_fail(
            b"app < batch\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            642 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_actor_run\0"))
                .as_ptr(),
        );
    };
    if !has_sync_flag(actor, SYNC_FLAG_MUTED as libc::c_int as u8) {
    } else {
        ponyint_assert_fail(
            b"!has_sync_flag(actor, SYNC_FLAG_MUTED)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            643 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_actor_run\0"))
                .as_ptr(),
        );
    };
    if has_sync_flag(actor, SYNC_FLAG_OVERLOADED as libc::c_int as u8) {
        actor_unsetoverloaded(actor);
    }
    try_gc(ctx, actor);
    if app > 0 {
        return 1 as libc::c_int != 0;
    }
    if !has_internal_flag(
        actor,
        (FLAG_BLOCKED as libc::c_int
            | FLAG_SYSTEM as libc::c_int
            | FLAG_BLOCKED_SENT as libc::c_int) as u8,
    ) {
        set_internal_flag(actor, FLAG_BLOCKED as libc::c_int as u8);
    }
    if has_internal_flag(actor, FLAG_BLOCKED as libc::c_int as u8) {
        if (*actor).gc.rc == 0 {
            if actor_noblock as libc::c_int != 0
                || !has_internal_flag(actor, FLAG_RC_OVER_ZERO_SEEN as libc::c_int as u8)
            {
                if ponyint_messageq_isempty(&mut (*actor).q) {
                    let mut empty: bool = ponyint_messageq_markempty(&mut (*actor).q);
                    if empty as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"empty\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                                as *const u8 as *const libc::c_char,
                            698 as libc::c_int as usize,
                            (*::core::mem::transmute::<
                                &[u8; 18],
                                &[libc::c_char; 18],
                            >(b"ponyint_actor_run\0"))
                                .as_ptr(),
                        );
                    };
                    ponyint_actor_setpendingdestroy(actor);
                    ponyint_actor_final(ctx, actor);
                    ponyint_actor_sendrelease(ctx, actor);
                    ponyint_actor_destroy(actor);
                    return !empty;
                }
            } else if ponyint_acquire_cycle_detector_critical(actor) {
                if ponyint_messageq_isempty(&mut (*actor).q) {
                    ponyint_actor_setpendingdestroy(actor);
                    send_block(ctx, actor);
                    let mut empty_0: bool = ponyint_messageq_markempty(&mut (*actor).q);
                    if empty_0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"empty\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0"
                                as *const u8 as *const libc::c_char,
                            750 as libc::c_int as usize,
                            (*::core::mem::transmute::<
                                &[u8; 18],
                                &[libc::c_char; 18],
                            >(b"ponyint_actor_run\0"))
                                .as_ptr(),
                        );
                    };
                    ponyint_release_cycle_detector_critical(actor);
                    return !empty_0;
                } else {
                    ponyint_release_cycle_detector_critical(actor);
                }
            }
        } else if !actor_noblock
            && !has_internal_flag(actor, FLAG_CD_CONTACTED as libc::c_int as u8)
        {
            if ponyint_acquire_cycle_detector_critical(actor) {
                send_block(ctx, actor);
                ponyint_release_cycle_detector_critical(actor);
            }
        }
    }
    return !ponyint_messageq_markempty(&mut (*actor).q);
}
#[no_mangle]
#[c2rust::src_loc = "786:1"]
pub unsafe extern "C" fn ponyint_actor_destroy(mut actor: *mut pony_actor_t) {
    if has_sync_flag(actor, SYNC_FLAG_PENDINGDESTROY as libc::c_int as u8) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"has_sync_flag(actor, SYNC_FLAG_PENDINGDESTROY)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            788 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"ponyint_actor_destroy\0"))
                .as_ptr(),
        );
    };
    let mut head: *mut pony_msg_t = 0 as *mut pony_msg_t;
    loop {
        head = ::core::intrinsics::atomic_load_relaxed(&mut (*actor).q.head);
        if !(head as libc::uintptr_t & 1 as libc::c_int as libc::uintptr_t
            != 1 as libc::c_int as libc::uintptr_t)
        {
            break;
        }
    }
    f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
    ponyint_messageq_destroy(&mut (*actor).q, 0 as libc::c_int != 0);
    ponyint_gc_destroy(&mut (*actor).gc);
    ponyint_heap_destroy(&mut (*actor).heap);
    ponyint_pool_free_size((*(*actor).type_0).size as usize, actor as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "821:1"]
pub unsafe extern "C" fn ponyint_actor_gc(mut actor: *mut pony_actor_t) -> *mut gc_t {
    return &mut (*actor).gc;
}
#[no_mangle]
#[c2rust::src_loc = "826:1"]
pub unsafe extern "C" fn ponyint_actor_heap(mut actor: *mut pony_actor_t) -> *mut heap_t {
    return &mut (*actor).heap;
}
#[no_mangle]
#[c2rust::src_loc = "831:1"]
pub unsafe extern "C" fn ponyint_actor_pendingdestroy(mut actor: *mut pony_actor_t) -> bool {
    return has_sync_flag(actor, SYNC_FLAG_PENDINGDESTROY as libc::c_int as u8);
}
#[no_mangle]
#[c2rust::src_loc = "836:1"]
pub unsafe extern "C" fn ponyint_actor_setpendingdestroy(mut actor: *mut pony_actor_t) {
    set_sync_flag(actor, SYNC_FLAG_PENDINGDESTROY as libc::c_int as u8);
}
#[no_mangle]
#[c2rust::src_loc = "846:1"]
pub unsafe extern "C" fn ponyint_actor_final(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    let mut prev: *mut pony_actor_t = (*ctx).current;
    let ref mut fresh1 = (*ctx).current;
    *fresh1 = actor;
    if ((*(*actor).type_0).final_0).is_some() {
        ((*(*actor).type_0).final_0).expect("non-null function pointer")(
            actor as *mut libc::c_void,
        );
    }
    ponyint_heap_final(&mut (*actor).heap);
    let ref mut fresh2 = (*ctx).current;
    *fresh2 = prev;
}
#[no_mangle]
#[c2rust::src_loc = "864:1"]
pub unsafe extern "C" fn ponyint_actor_sendrelease(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) {
    ponyint_gc_sendrelease(ctx, &mut (*actor).gc);
}
#[no_mangle]
#[c2rust::src_loc = "869:1"]
pub unsafe extern "C" fn ponyint_actor_setsystem(mut actor: *mut pony_actor_t) {
    set_internal_flag(actor, FLAG_SYSTEM as libc::c_int as u8);
}
#[no_mangle]
#[c2rust::src_loc = "874:1"]
pub unsafe extern "C" fn ponyint_actor_setnoblock(mut state: bool) {
    actor_noblock = state;
}
#[no_mangle]
#[c2rust::src_loc = "879:1"]
pub unsafe extern "C" fn ponyint_actor_getnoblock() -> bool {
    return actor_noblock;
}
#[no_mangle]
#[c2rust::src_loc = "884:1"]
pub unsafe extern "C" fn pony_create(
    mut ctx: *mut pony_ctx_t,
    mut type_0: *const pony_type_t,
    mut orphaned: bool,
) -> *mut pony_actor_t {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            887 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"pony_create\0")).as_ptr(),
        );
    };
    let mut actor: *mut pony_actor_t =
        ponyint_pool_alloc_size((*type_0).size as usize) as *mut pony_actor_t;
    memset(
        actor as *mut libc::c_void,
        0 as libc::c_int,
        (*type_0).size as libc::c_ulong,
    );
    let ref mut fresh3 = (*actor).type_0;
    *fresh3 = type_0;
    ponyint_messageq_init(&mut (*actor).q);
    ponyint_heap_init(&mut (*actor).heap);
    ponyint_gc_done(&mut (*actor).gc);
    if !((*ctx).current).is_null() && !orphaned {
        (*actor).gc.rc = 256 as libc::c_int as usize;
        ponyint_gc_createactor((*ctx).current, actor);
    } else {
        (*actor).gc.rc = 0 as libc::c_int as usize;
    }
    macro__DTRACE(
        b"ACTOR_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
    return actor;
}
#[no_mangle]
#[c2rust::src_loc = "928:1"]
pub unsafe extern "C" fn ponyint_destroy(mut _ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    ponyint_actor_setpendingdestroy(actor);
    ponyint_actor_destroy(actor);
}
#[no_mangle]
#[c2rust::src_loc = "938:1"]
pub unsafe extern "C" fn pony_alloc_msg(mut index: u32, mut id: u32) -> *mut pony_msg_t {
    let mut msg: *mut pony_msg_t = ponyint_pool_alloc(index as usize) as *mut pony_msg_t;
    (*msg).index = index;
    (*msg).id = id;
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*msg).next, 0 as *mut pony_msg_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    return msg;
}
#[no_mangle]
#[c2rust::src_loc = "961:1"]
pub unsafe extern "C" fn pony_alloc_msg_size(mut size: usize, mut id: u32) -> *mut pony_msg_t {
    return pony_alloc_msg(ponyint_pool_index(size) as u32, id);
}
#[no_mangle]
#[c2rust::src_loc = "966:1"]
pub unsafe extern "C" fn pony_sendv(
    mut ctx: *mut pony_ctx_t,
    mut to: *mut pony_actor_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
    mut has_app_msg: bool,
) {
    if well_formed_msg_chain(first, last) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"well_formed_msg_chain(first, last)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            972 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_sendv\0")).as_ptr(),
        );
    };
    if !ponyint_actor_pendingdestroy(to) {
    } else {
        ponyint_assert_fail(
            b"!ponyint_actor_pendingdestroy(to)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            976 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_sendv\0")).as_ptr(),
        );
    };
    if macro__DTRACE_ENABLED(b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char) {
        let mut m: *mut pony_msg_t = first;
        while m != last {
            macro__DTRACE(
                b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"m->id\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)to\0" as *const u8 as *const libc::c_char,
            );
            m = ::core::intrinsics::atomic_load_relaxed(&mut (*m).next);
        }
        macro__DTRACE(
            b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
            b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
            b"last->id\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t)to\0" as *const u8 as *const libc::c_char,
        );
    }
    if has_app_msg {
        maybe_mark_should_mute(ctx, to);
    }
    if ponyint_actor_messageq_push(&mut (*to).q, first, last) {
        if !has_sync_flag(to, SYNC_FLAG_MUTED as libc::c_int as u8) {
            ponyint_sched_add(ctx, to);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1009:1"]
pub unsafe extern "C" fn pony_sendv_single(
    mut ctx: *mut pony_ctx_t,
    mut to: *mut pony_actor_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
    mut has_app_msg: bool,
) {
    if well_formed_msg_chain(first, last) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"well_formed_msg_chain(first, last)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1015 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pony_sendv_single\0"))
                .as_ptr(),
        );
    };
    if !ponyint_actor_pendingdestroy(to) {
    } else {
        ponyint_assert_fail(
            b"!ponyint_actor_pendingdestroy(to)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1019 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pony_sendv_single\0"))
                .as_ptr(),
        );
    };
    if macro__DTRACE_ENABLED(b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char) {
        let mut m: *mut pony_msg_t = first;
        while m != last {
            macro__DTRACE(
                b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
                b"m->id\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)to\0" as *const u8 as *const libc::c_char,
            );
            m = ::core::intrinsics::atomic_load_relaxed(&mut (*m).next);
        }
        macro__DTRACE(
            b"ACTOR_MSG_SEND\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
            b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
            b"last->id\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t)to\0" as *const u8 as *const libc::c_char,
        );
    }
    if has_app_msg {
        maybe_mark_should_mute(ctx, to);
    }
    if ponyint_actor_messageq_push_single(&mut (*to).q, first, last) {
        if !has_sync_flag(to, SYNC_FLAG_MUTED as libc::c_int as u8) {
            ponyint_sched_add(ctx, to);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1054:1"]
pub unsafe extern "C" fn pony_chain(mut prev: *mut pony_msg_t, mut next: *mut pony_msg_t) {
    if ({ ::core::intrinsics::atomic_load_relaxed(&mut (*prev).next as *mut *mut pony_msg_t) })
        .is_null()
    {
    } else {
        ponyint_assert_fail(
            b"atomic_load_explicit(&prev->next, memory_order_relaxed) == NULL\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1056 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_chain\0")).as_ptr(),
        );
    };
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*prev).next, next);
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[no_mangle]
#[c2rust::src_loc = "1060:1"]
pub unsafe extern "C" fn pony_send(
    mut ctx: *mut pony_ctx_t,
    mut to: *mut pony_actor_t,
    mut id: u32,
) {
    let mut m: *mut pony_msg_t = pony_alloc_msg(0 as libc::c_int as u32, id);
    pony_sendv(
        ctx,
        to,
        m,
        m,
        id <= (4294967295 as libc::c_uint).wrapping_sub(11 as libc::c_int as libc::c_uint),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1071:1"]
pub unsafe extern "C" fn pony_sendp(
    mut ctx: *mut pony_ctx_t,
    mut to: *mut pony_actor_t,
    mut id: u32,
    mut p: *mut libc::c_void,
) {
    let mut m: *mut pony_msgp_t = pony_alloc_msg(0 as libc::c_int as u32, id) as *mut pony_msgp_t;
    let ref mut fresh4 = (*m).p;
    *fresh4 = p;
    pony_sendv(
        ctx,
        to,
        &mut (*m).msg,
        &mut (*m).msg,
        id <= (4294967295 as libc::c_uint).wrapping_sub(11 as libc::c_int as libc::c_uint),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1086:1"]
pub unsafe extern "C" fn pony_sendi(
    mut ctx: *mut pony_ctx_t,
    mut to: *mut pony_actor_t,
    mut id: u32,
    mut i: intptr_t,
) {
    let mut m: *mut pony_msgi_t = pony_alloc_msg(0 as libc::c_int as u32, id) as *mut pony_msgi_t;
    (*m).i = i;
    pony_sendv(
        ctx,
        to,
        &mut (*m).msg,
        &mut (*m).msg,
        id <= (4294967295 as libc::c_uint).wrapping_sub(11 as libc::c_int as libc::c_uint),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn pony_alloc(
    mut ctx: *mut pony_ctx_t,
    mut size: usize,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1103 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_alloc\0")).as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        size,
        TRACK_NO_FINALISERS as libc::c_int as u32,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1110:1"]
pub unsafe extern "C" fn pony_alloc_small(
    mut ctx: *mut pony_ctx_t,
    mut sizeclass: u32,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1112 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"pony_alloc_small\0"))
                .as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"HEAP_MIN << sizeclass\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc_small(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        sizeclass,
        TRACK_NO_FINALISERS as libc::c_int as u32,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1119:1"]
pub unsafe extern "C" fn pony_alloc_large(
    mut ctx: *mut pony_ctx_t,
    mut size: usize,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1121 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"pony_alloc_large\0"))
                .as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc_large(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        size,
        TRACK_NO_FINALISERS as libc::c_int as u32,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1128:1"]
pub unsafe extern "C" fn pony_realloc(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut size: usize,
    mut copy: usize,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1130 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"pony_realloc\0")).as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_realloc((*ctx).current, &mut (*(*ctx).current).heap, p, size, copy);
}
#[no_mangle]
#[c2rust::src_loc = "1136:1"]
pub unsafe extern "C" fn pony_alloc_final(
    mut ctx: *mut pony_ctx_t,
    mut size: usize,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1138 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"pony_alloc_final\0"))
                .as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        size,
        TRACK_ALL_FINALISERS as libc::c_uint,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1145:1"]
pub unsafe extern "C" fn pony_alloc_small_final(
    mut ctx: *mut pony_ctx_t,
    mut sizeclass: u32,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1147 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"pony_alloc_small_final\0",
            ))
            .as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"HEAP_MIN << sizeclass\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc_small(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        sizeclass,
        TRACK_ALL_FINALISERS as libc::c_uint,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1154:1"]
pub unsafe extern "C" fn pony_alloc_large_final(
    mut ctx: *mut pony_ctx_t,
    mut size: usize,
) -> *mut libc::c_void {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1156 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"pony_alloc_large_final\0",
            ))
            .as_ptr(),
        );
    };
    macro__DTRACE(
        b"HEAP_ALLOC\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
    );
    return ponyint_heap_alloc_large(
        (*ctx).current,
        &mut (*(*ctx).current).heap,
        size,
        TRACK_ALL_FINALISERS as libc::c_uint,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1163:1"]
pub unsafe extern "C" fn pony_triggergc(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1165 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pony_triggergc\0"))
                .as_ptr(),
        );
    };
    (*(*ctx).current).heap.next_gc = 0 as libc::c_int as usize;
}
#[no_mangle]
#[c2rust::src_loc = "1169:1"]
pub unsafe extern "C" fn ponyint_become(mut ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    let ref mut fresh5 = (*ctx).current;
    *fresh5 = actor;
}
#[no_mangle]
#[c2rust::src_loc = "1174:1"]
pub unsafe extern "C" fn pony_poll(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.c\0" as *const u8
                as *const libc::c_char,
            1178 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pony_poll\0")).as_ptr(),
        );
    };
    ponyint_actor_run(ctx, (*ctx).current, 1 as libc::c_int != 0);
}
#[no_mangle]
#[c2rust::src_loc = "1182:1"]
pub unsafe extern "C" fn pony_apply_backpressure() {
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    set_sync_flag(
        (*ctx).current,
        SYNC_FLAG_UNDER_PRESSURE as libc::c_int as u8,
    );
    macro__DTRACE(
        b"ACTOR_UNDER_PRESSURE\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1189:1"]
pub unsafe extern "C" fn pony_release_backpressure() {
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    unset_sync_flag(
        (*ctx).current,
        SYNC_FLAG_UNDER_PRESSURE as libc::c_int as u8,
    );
    macro__DTRACE(
        b"ACTOR_PRESSURE_RELEASED\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->current\0" as *const u8 as *const libc::c_char,
    );
    if !has_sync_flag((*ctx).current, SYNC_FLAG_OVERLOADED as libc::c_int as u8) {
        ponyint_sched_start_global_unmute((*(*ctx).scheduler).index as u32, (*ctx).current);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1198:1"]
pub unsafe extern "C" fn pony_actor_stats() -> *mut actorstats_t {
    return 0 as *mut actorstats_t;
}
#[no_mangle]
#[c2rust::src_loc = "1257:1"]
pub unsafe extern "C" fn ponyint_acquire_cycle_detector_critical(
    mut actor: *mut pony_actor_t,
) -> bool {
    let mut expected: u8 = 0 as libc::c_int as u8;
    return {
        let fresh6 = ::core::intrinsics::atomic_cxchg_acqrel(
            &mut (*actor).cycle_detector_critical,
            *&mut expected,
            1 as libc::c_int as u8,
        );
        *&mut expected = fresh6.0;
        fresh6.1
    };
}
#[no_mangle]
#[c2rust::src_loc = "1263:1"]
pub unsafe extern "C" fn ponyint_release_cycle_detector_critical(mut actor: *mut pony_actor_t) {
    (*actor).cycle_detector_critical = 0 as libc::c_int as u8;
}
