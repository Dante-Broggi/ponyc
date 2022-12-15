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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:5"]
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

    extern "C" {
        #[c2rust::src_loc = "117:1"]
        pub fn ponyint_actor_gc(actor: *mut pony_actor_t) -> *mut gc_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:2"]
pub mod gc_h {
    use super::actor_h::pony_actor_t;
    #[c2rust::src_loc = "16:16"]
    pub use crate::libponyrt::gc::gc::gc_t;

    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;

    #[c2rust::src_loc = "28:32"]
    pub use crate::libponyrt::gc::gc::gcstack_t;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_gc_acquireactor(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "69:1"]
        pub fn ponyint_gc_sendrelease_manual(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_gc_handlestack(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "65:1"]
        pub fn ponyint_gc_sendacquire(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_gc_done(gc: *mut gc_t);
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_gc_releaseobject(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            mutability: libc::c_int,
        );
        #[c2rust::src_loc = "53:1"]
        pub fn ponyint_gc_releaseactor(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "39:1"]
        pub fn ponyint_gc_acquireobject(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            mutability: libc::c_int,
        );
        #[c2rust::src_loc = "33:1"]
        pub fn ponyint_gc_recvobject(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            mutability: libc::c_int,
        );
        #[c2rust::src_loc = "47:1"]
        pub fn ponyint_gc_recvactor(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_gc_sendobject(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            mutability: libc::c_int,
        );
        #[c2rust::src_loc = "45:1"]
        pub fn ponyint_gc_sendactor(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "49:1"]
        pub fn ponyint_gc_markactor(ctx: *mut pony_ctx_t, actor: *mut pony_actor_t);
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_gc_markobject(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            mutability: libc::c_int,
        );
        #[c2rust::src_loc = "63:1"]
        pub fn ponyint_gc_sweep(ctx: *mut pony_ctx_t, gc: *mut gc_t);
        #[c2rust::src_loc = "57:1"]
        pub fn ponyint_gc_markimmutable(ctx: *mut pony_ctx_t, gc: *mut gc_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/delta.h:2"]
pub mod delta_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "16:35"]
    pub struct deltamap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:2"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:2"]
pub mod actormap_h {
    #[c2rust::src_loc = "27:35"]
    pub use crate::libponyrt::gc::actormap::actormap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/objectmap.h:2"]
pub mod objectmap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:36"]
    pub struct objectmap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:2"]
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

    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:3"]
pub use crate::libponyrt::actor::messageq::messageq_h;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
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
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed_0 = 0;

    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:3"]
pub mod scheduler_h {
    #[c2rust::src_loc = "60:16"]
    pub use crate::libponyrt::sched::scheduler::pony_ctx_t;
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

    use super::pony_h::pony_type_t;

    #[c2rust::src_loc = "84:8"]
    pub use crate::libponyrt::sched::scheduler::scheduler_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:3"]
pub mod serialise_h {
    #[c2rust::src_loc = "18:1"]
    pub type serialise_throw_fn = Option<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "16:1"]
    pub type serialise_alloc_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, usize) -> *mut libc::c_void>;

    use super::scheduler_h::pony_ctx_t;
    #[c2rust::src_loc = "24:36"]
    pub use crate::libponyrt::gc::serialise::ponyint_serialise_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:6"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:7"]
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
pub use self::actor_h::{pony_actor_t, ponyint_actor_gc};
pub use self::actormap_h::actormap_t;
pub use self::delta_h::deltamap_t;
use self::dtrace_h::macro__DTRACE;
pub use self::gc_h::{
    gc_t, gcstack_t, ponyint_gc_acquireactor, ponyint_gc_acquireobject, ponyint_gc_done,
    ponyint_gc_handlestack, ponyint_gc_markactor, ponyint_gc_markimmutable, ponyint_gc_markobject,
    ponyint_gc_recvactor, ponyint_gc_recvobject, ponyint_gc_releaseactor, ponyint_gc_releaseobject,
    ponyint_gc_sendacquire, ponyint_gc_sendactor, ponyint_gc_sendobject,
    ponyint_gc_sendrelease_manual, ponyint_gc_sweep,
};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{chunk_t, heap_t};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
pub use self::pony_h::{
    _pony_type_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn, pony_dispatch_fn,
    pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_type_t, C2RustUnnamed_0,
    PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
#[no_mangle]
#[c2rust::src_loc = "9:1"]
pub unsafe extern "C" fn pony_gc_send(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            11 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"pony_gc_send\0")).as_ptr(),
        );
    };
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            12 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"pony_gc_send\0")).as_ptr(),
        );
    };
    let ref mut fresh0 = (*ctx).trace_object;
    *fresh0 = Some(
        ponyint_gc_sendobject
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh1 = (*ctx).trace_actor;
    *fresh1 = Some(
        ponyint_gc_sendactor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
    macro__DTRACE(
        b"GC_SEND_START\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "19:1"]
pub unsafe extern "C" fn pony_gc_recv(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            21 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"pony_gc_recv\0")).as_ptr(),
        );
    };
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            22 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"pony_gc_recv\0")).as_ptr(),
        );
    };
    let ref mut fresh2 = (*ctx).trace_object;
    *fresh2 = Some(
        ponyint_gc_recvobject
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh3 = (*ctx).trace_actor;
    *fresh3 = Some(
        ponyint_gc_recvactor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
    macro__DTRACE(
        b"GC_RECV_START\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn ponyint_gc_mark(mut ctx: *mut pony_ctx_t) {
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            31 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ponyint_gc_mark\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh4 = (*ctx).trace_object;
    *fresh4 = Some(
        ponyint_gc_markobject
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh5 = (*ctx).trace_actor;
    *fresh5 = Some(
        ponyint_gc_markactor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
}
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn pony_gc_acquire(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            38 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"pony_gc_acquire\0"))
                .as_ptr(),
        );
    };
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            39 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"pony_gc_acquire\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh6 = (*ctx).trace_object;
    *fresh6 = Some(
        ponyint_gc_acquireobject
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh7 = (*ctx).trace_actor;
    *fresh7 = Some(
        ponyint_gc_acquireactor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn pony_gc_release(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            46 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"pony_gc_release\0"))
                .as_ptr(),
        );
    };
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            47 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"pony_gc_release\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh8 = (*ctx).trace_object;
    *fresh8 = Some(
        ponyint_gc_releaseobject
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh9 = (*ctx).trace_actor;
    *fresh9 = Some(
        ponyint_gc_releaseactor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
}
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn pony_send_done(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            54 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pony_send_done\0"))
                .as_ptr(),
        );
    };
    ponyint_gc_handlestack(ctx);
    ponyint_gc_sendacquire(ctx);
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
    macro__DTRACE(
        b"GC_SEND_END\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn pony_recv_done(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            64 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pony_recv_done\0"))
                .as_ptr(),
        );
    };
    ponyint_gc_handlestack(ctx);
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
    macro__DTRACE(
        b"GC_RECV_END\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)ctx->scheduler\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn ponyint_mark_done(mut ctx: *mut pony_ctx_t) {
    ponyint_gc_markimmutable(ctx, ponyint_actor_gc((*ctx).current));
    ponyint_gc_handlestack(ctx);
    ponyint_gc_sendacquire(ctx);
    ponyint_gc_sweep(ctx, ponyint_actor_gc((*ctx).current));
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn pony_acquire_done(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            82 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pony_acquire_done\0"))
                .as_ptr(),
        );
    };
    ponyint_gc_handlestack(ctx);
    ponyint_gc_sendacquire(ctx);
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn pony_release_done(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            90 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pony_release_done\0"))
                .as_ptr(),
        );
    };
    ponyint_gc_handlestack(ctx);
    ponyint_gc_sendrelease_manual(ctx);
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
}
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn pony_send_next(mut ctx: *mut pony_ctx_t) {
    if !((*ctx).current).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->current != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/trace.c\0" as *const u8
                as *const libc::c_char,
            98 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pony_send_next\0"))
                .as_ptr(),
        );
    };
    ponyint_gc_handlestack(ctx);
    ponyint_gc_done(ponyint_actor_gc((*ctx).current));
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn pony_trace(mut ctx: *mut pony_ctx_t, mut p: *mut libc::c_void) {
    ((*ctx).trace_object).expect("non-null function pointer")(
        ctx,
        p,
        0 as *const pony_type_t,
        PONY_TRACE_OPAQUE as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub unsafe extern "C" fn pony_traceknown(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut m: libc::c_int,
) {
    if ((*t).dispatch).is_some() {
        ((*ctx).trace_actor).expect("non-null function pointer")(ctx, p as *mut pony_actor_t);
    } else {
        ((*ctx).trace_object).expect("non-null function pointer")(ctx, p, t, m);
    };
}
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn pony_traceunknown(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut m: libc::c_int,
) {
    let mut t: *const pony_type_t = *(p as *mut *const pony_type_t);
    if ((*t).dispatch).is_some() {
        ((*ctx).trace_actor).expect("non-null function pointer")(ctx, p as *mut pony_actor_t);
    } else {
        ((*ctx).trace_object).expect("non-null function pointer")(ctx, p, t, m);
    };
}
