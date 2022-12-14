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
    pub type uintptr_t = libc::c_ulong;
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
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;
    use super::stddef_h::size_t;
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
    use super::actormap_h::actormap_t;
    use super::delta_h::deltamap_t;
    use super::objectmap_h::objectmap_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
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
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:3"]
pub mod actormap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:35"]
    pub struct actormap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
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
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:3"]
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
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed_0 = 0;
    use super::_uintptr_t_h::uintptr_t;
    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
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
        #[c2rust::src_loc = "300:1"]
        pub fn pony_gc_send(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "307:1"]
        pub fn pony_gc_recv(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "335:1"]
        pub fn pony_send_done(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "341:1"]
        pub fn pony_recv_done(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "394:1"]
        pub fn pony_traceknown(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            m: libc::c_int,
        );
        #[c2rust::src_loc = "404:1"]
        pub fn pony_traceunknown(ctx: *mut pony_ctx_t, p: *mut libc::c_void, m: libc::c_int);
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
        #[c2rust::src_loc = "150:1"]
        pub fn ponyint_sched_maybe_wakeup_if_all_asleep(current_scheduler_id: i32);
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
    use super::stddef_h::size_t;
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
        pub counter: uintptr_t,
    }
    use super::_uintptr_t_h::uintptr_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/event.h:1"]
pub mod event_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:16"]
    pub struct asio_event_t {
        pub magic: *mut asio_event_t,
        pub owner: *mut pony_actor_t,
        pub msg_id: u32,
        pub fd: libc::c_int,
        pub flags: u32,
        pub noisy: bool,
        pub readable: bool,
        pub writeable: bool,
        pub nsec: u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:16"]
    pub struct asio_msg_t {
        pub msg: pony_msg_t,
        pub event: *mut asio_event_t,
        pub flags: u32,
        pub arg: u32,
    }
    use super::actor_h::pony_actor_t;
    use super::pony_h::pony_msg_t;
    extern "C" {
        #[c2rust::src_loc = "70:1"]
        pub fn pony_asio_event_subscribe(ev: *mut asio_event_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/asio.h:2"]
pub mod asio_h {
    #[c2rust::src_loc = "32:3"]
    pub const ASIO_DESTROYED: C2RustUnnamed_1 = 4294967295;
    #[c2rust::src_loc = "26:3"]
    pub const ASIO_DISPOSABLE: C2RustUnnamed_1 = 0;
    #[c2rust::src_loc = "24:1"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "31:3"]
    pub const ASIO_ONESHOT: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "30:3"]
    pub const ASIO_SIGNAL: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "29:3"]
    pub const ASIO_TIMER: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "28:3"]
    pub const ASIO_WRITE: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "27:3"]
    pub const ASIO_READ: C2RustUnnamed_1 = 1;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:6"]
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
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::pony_actor_t;
pub use self::actormap_h::actormap_t;
pub use self::asio_h::{
    C2RustUnnamed_1, ASIO_DESTROYED, ASIO_DISPOSABLE, ASIO_ONESHOT, ASIO_READ, ASIO_SIGNAL,
    ASIO_TIMER, ASIO_WRITE,
};
pub use self::delta_h::deltamap_t;
pub use self::event_h::{asio_event_t, asio_msg_t, pony_asio_event_subscribe};
pub use self::gc_h::{gc_t, gcstack_t};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{chunk_t, heap_t};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
pub use self::pony_h::{
    _pony_type_t, pony_alloc_msg, pony_ctx, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_gc_recv, pony_gc_send,
    pony_msg_t, pony_recv_done, pony_send_done, pony_sendv, pony_serialise_fn, pony_trace_fn,
    pony_traceknown, pony_traceunknown, pony_type_t, C2RustUnnamed_0, PONY_TRACE_IMMUTABLE,
    PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::scheduler_h::{
    pony_ctx_t, ponyint_sched_maybe_wakeup_if_all_asleep, scheduler_t, trace_actor_fn,
    trace_object_fn,
};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
#[no_mangle]
#[c2rust::src_loc = "9:1"]
pub unsafe extern "C" fn pony_asio_event_create(
    mut owner: *mut pony_actor_t,
    mut fd: libc::c_int,
    mut flags: u32,
    mut nsec: u64,
    mut noisy: bool,
) -> *mut asio_event_t {
    if flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || flags == ASIO_DESTROYED as libc::c_uint
    {
        return 0 as *mut asio_event_t;
    }
    let mut type_0: *const pony_type_t = *(owner as *mut *const pony_type_t);
    let mut msg_id: u32 = (*type_0).event_notify;
    if msg_id == -(1 as libc::c_int) as u32 {
        return 0 as *mut asio_event_t;
    }
    let mut ev: *mut asio_event_t =
        ponyint_pool_alloc(1 as libc::c_int as usize) as *mut asio_event_t;
    let ref mut fresh0 = (*ev).magic;
    *fresh0 = ev;
    let ref mut fresh1 = (*ev).owner;
    *fresh1 = owner;
    (*ev).msg_id = msg_id;
    (*ev).fd = fd;
    (*ev).flags = flags;
    (*ev).noisy = noisy;
    (*ev).nsec = nsec;
    (*ev).writeable = 0 as libc::c_int != 0;
    (*ev).readable = 0 as libc::c_int != 0;
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    pony_gc_send(ctx);
    pony_traceknown(
        ctx,
        owner as *mut libc::c_void,
        type_0,
        PONY_TRACE_OPAQUE as libc::c_int,
    );
    pony_send_done(ctx);
    pony_asio_event_subscribe(ev);
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn pony_asio_event_destroy(mut ev: *mut asio_event_t) {
    if ev.is_null()
        || (*ev).magic != ev
        || (*ev).flags != ASIO_DISPOSABLE as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/event.c\0"
                    as *const u8 as *const libc::c_char,
                47 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"pony_asio_event_destroy\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    (*ev).flags = ASIO_DESTROYED as libc::c_uint;
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    pony_gc_recv(ctx);
    pony_traceunknown(
        ctx,
        (*ev).owner as *mut libc::c_void,
        PONY_TRACE_OPAQUE as libc::c_int,
    );
    pony_recv_done(ctx);
    ponyint_pool_free(1 as libc::c_int as usize, ev as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn pony_asio_event_fd(mut ev: *mut asio_event_t) -> libc::c_int {
    if ev.is_null() {
        return -(1 as libc::c_int);
    }
    return (*ev).fd;
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn pony_asio_event_get_disposable(mut ev: *mut asio_event_t) -> bool {
    if ev.is_null() {
        return 0 as libc::c_int != 0;
    }
    return (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn pony_asio_event_get_writeable(mut ev: *mut asio_event_t) -> bool {
    if ev.is_null() {
        return 0 as libc::c_int != 0;
    }
    return (*ev).writeable;
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn pony_asio_event_set_writeable(
    mut ev: *mut asio_event_t,
    mut writeable: bool,
) {
    if !ev.is_null() {
        (*ev).writeable = writeable;
    }
}
#[no_mangle]
#[c2rust::src_loc = "93:1"]
pub unsafe extern "C" fn pony_asio_event_get_readable(mut ev: *mut asio_event_t) -> bool {
    if ev.is_null() {
        return 0 as libc::c_int != 0;
    }
    return (*ev).readable;
}
#[no_mangle]
#[c2rust::src_loc = "101:1"]
pub unsafe extern "C" fn pony_asio_event_set_readable(
    mut ev: *mut asio_event_t,
    mut readable: bool,
) {
    if !ev.is_null() {
        (*ev).readable = readable;
    }
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn pony_asio_event_nsec(mut ev: *mut asio_event_t) -> u64 {
    if ev.is_null() {
        return 0 as libc::c_int as u64;
    }
    return (*ev).nsec;
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn pony_asio_event_send(
    mut ev: *mut asio_event_t,
    mut flags: u32,
    mut arg: u32,
) {
    let mut m: *mut asio_msg_t =
        pony_alloc_msg(0 as libc::c_int as u32, (*ev).msg_id) as *mut asio_msg_t;
    let ref mut fresh2 = (*m).event;
    *fresh2 = ev;
    (*m).flags = flags;
    (*m).arg = arg;
    pony_sendv(
        pony_ctx(),
        (*ev).owner,
        &mut (*m).msg,
        &mut (*m).msg,
        0 as libc::c_int != 0,
    );
    ponyint_sched_maybe_wakeup_if_all_asleep(-(1 as libc::c_int));
}
