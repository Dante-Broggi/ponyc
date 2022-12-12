use ::libc;
use core::sync::atomic::{
    AtomicBool,
    AtomicU32,
    Ordering::{Relaxed, Release},
};
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:3"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int64_t.h:3"]
pub mod _int64_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int64_t = libc::c_longlong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:3"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:3"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:3"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
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
    #[c2rust::src_loc = "73:8"]
    pub struct _opaque_pthread_condattr_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:8"]
    pub struct _opaque_pthread_mutex_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 56],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:8"]
    pub struct _opaque_pthread_mutexattr_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct _opaque_pthread_once_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 8],
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
    #[c2rust::src_loc = "111:1"]
    pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
    #[c2rust::src_loc = "113:1"]
    pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
    #[c2rust::src_loc = "114:1"]
    pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
    #[c2rust::src_loc = "115:1"]
    pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_pthread_t = *mut _opaque_pthread_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:3"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h:3"]
pub mod _intptr_t_h {
    #[c2rust::src_loc = "32:1"]
    pub type intptr_t = __darwin_intptr_t;
    use super::_types_h::__darwin_intptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:3"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_condattr_t.h:3"]
pub mod _pthread_condattr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_condattr_t = __darwin_pthread_condattr_t;
    use super::_pthread_types_h::__darwin_pthread_condattr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_mutex_t.h:3"]
pub mod _pthread_mutex_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_mutex_t = __darwin_pthread_mutex_t;
    use super::_pthread_types_h::__darwin_pthread_mutex_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_mutexattr_t.h:3"]
pub mod _pthread_mutexattr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
    use super::_pthread_types_h::__darwin_pthread_mutexattr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_once_t.h:3"]
pub mod _pthread_once_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_once_t = __darwin_pthread_once_t;
    use super::_pthread_types_h::__darwin_pthread_once_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:3"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/threads.h:3"]
pub mod threads_h {
    #[c2rust::src_loc = "19:1"]
    pub type thread_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>;
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_t_h::pthread_t;
    
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_thread_create(
            thread: *mut pthread_t,
            start: thread_fn,
            cpu: uint32_t,
            arg: *mut libc::c_void,
        ) -> bool;
        #[c2rust::src_loc = "54:1"]
        pub fn ponyint_thread_join(thread: pthread_t) -> bool;
        #[c2rust::src_loc = "58:1"]
        pub fn ponyint_thread_self() -> pthread_t;
        #[c2rust::src_loc = "61:1"]
        pub fn ponyint_thread_suspend(signal: *mut pthread_cond_t, mut_0: *mut pthread_mutex_t);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_thread_wake(thread: pthread_t, signal: *mut pthread_cond_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:3"]
pub mod mpmcq_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct mpmcq_t {
        pub head: *mut mpmcq_node_t,
        pub tail: aba_protected_mpmcq_node_t,
    }
    use super::_uintptr_t_h::uintptr_t;
    use super::internal::__int128_t;
    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
        #[c2rust::src_loc = "23:1"]
        pub fn ponyint_mpmcq_init(q: *mut mpmcq_t);
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_mpmcq_destroy(q: *mut mpmcq_t);
        #[c2rust::src_loc = "27:1"]
        pub fn ponyint_mpmcq_push(q: *mut mpmcq_t, data: *mut libc::c_void);
        #[c2rust::src_loc = "29:1"]
        pub fn ponyint_mpmcq_push_single(q: *mut mpmcq_t, data: *mut libc::c_void);
        #[c2rust::src_loc = "31:1"]
        pub fn ponyint_mpmcq_pop(q: *mut mpmcq_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:3"]
pub mod scheduler_h {
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
    #[c2rust::src_loc = "35:16"]
    pub struct schedulerstats_t {
        pub mem_used: int64_t,
        pub mem_allocated: int64_t,
        pub mem_used_actors: int64_t,
        pub mem_allocated_actors: int64_t,
        pub created_actors_counter: size_t,
        pub destroyed_actors_counter: size_t,
        pub actor_app_cpu: size_t,
        pub actor_gc_mark_cpu: size_t,
        pub actor_gc_sweep_cpu: size_t,
        pub actor_system_cpu: size_t,
        pub msg_cpu: size_t,
        pub misc_cpu: size_t,
    }
    use super::_int32_t_h::int32_t;
    use super::_int64_t_h::int64_t;
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_t_h::pthread_t;
    use super::_uint32_t_h::uint32_t;
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
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_messageq_destroy(q: *mut messageq_t, maybe_non_empty: bool);
        #[c2rust::src_loc = "45:1"]
        pub fn ponyint_thread_messageq_push(
            q: *mut messageq_t,
            first: *mut pony_msg_t,
            last: *mut pony_msg_t,
        ) -> bool;
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_thread_messageq_pop(q: *mut messageq_t) -> *mut pony_msg_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:3"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:16"]
    pub struct pony_msgi_t {
        pub msg: pony_msg_t,
        pub i: intptr_t,
    }
    use super::_intptr_t_h::intptr_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "196:1"]
        pub fn pony_alloc_msg(index: uint32_t, id: uint32_t) -> *mut pony_msg_t;
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
    use super::actor_h::pony_actor_t;
    use super::hash_h::hashmap_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_muteref_free(mref: *mut muteref_t);
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_muteref_alloc(key: *mut pony_actor_t) -> *mut muteref_t;
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_mutemap_size(map: *mut mutemap_t) -> size_t;
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_mutemap_removeindex(map: *mut mutemap_t, index: size_t);
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_mutemap_putindex(map: *mut mutemap_t, entry: *mut muteref_t, index: size_t);
        #[c2rust::src_loc = "18:45"]
        pub fn ponyint_mutemap_get(
            map: *mut mutemap_t,
            key: *mut muteref_t,
            index: *mut size_t,
        ) -> *mut muteref_t;
        #[c2rust::src_loc = "10:45"]
        pub fn ponyint_muteset_next(map: *mut muteset_t, i: *mut size_t) -> *mut pony_actor_t;
        #[c2rust::src_loc = "10:1"]
        pub fn ponyint_muteset_putindex(
            map: *mut muteset_t,
            entry: *mut pony_actor_t,
            index: size_t,
        );
        #[c2rust::src_loc = "10:45"]
        pub fn ponyint_muteset_get(
            map: *mut muteset_t,
            key: *mut pony_actor_t,
            index: *mut size_t,
        ) -> *mut pony_actor_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:3"]
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
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/actor.h:7"]
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
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "113:1"]
        pub fn ponyint_actor_run(
            ctx: *mut pony_ctx_t,
            actor: *mut pony_actor_t,
            polling: bool,
        ) -> bool;
        #[c2rust::src_loc = "133:1"]
        pub fn ponyint_actor_getnoblock() -> bool;
        #[c2rust::src_loc = "141:1"]
        pub fn ponyint_unmute_actor(actor: *mut pony_actor_t);
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
        pub used: size_t,
        pub next_gc: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "32:1"]
        pub fn ponyint_pool_thread_cleanup();
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pthread/pthread.h:3"]
pub mod pthread_h {
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_condattr_t_h::pthread_condattr_t;
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_mutexattr_t_h::pthread_mutexattr_t;
    use super::_pthread_once_t_h::pthread_once_t;
    extern "C" {
        #[c2rust::src_loc = "299:1"]
        pub fn pthread_cond_destroy(_: *mut pthread_cond_t) -> libc::c_int;
        #[c2rust::src_loc = "302:1"]
        pub fn pthread_cond_init(
            _: *mut pthread_cond_t,
            _: *const pthread_condattr_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "387:1"]
        pub fn pthread_mutex_init(
            _: *mut pthread_mutex_t,
            _: *const pthread_mutexattr_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "391:1"]
        pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "399:1"]
        pub fn pthread_mutex_trylock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "403:1"]
        pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "448:1"]
        pub fn pthread_once(
            _: *mut pthread_once_t,
            _: Option<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:5"]
pub mod cpu_h {
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::scheduler_h::scheduler_t;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn ponyint_cpu_count() -> uint32_t;
        #[c2rust::src_loc = "15:1"]
        pub fn ponyint_cpu_assign(
            count: uint32_t,
            scheduler_0: *mut scheduler_t,
            nopin: bool,
            pinasio: bool,
        ) -> uint32_t;
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_cpu_affinity(cpu: uint32_t);
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_cpu_core_pause(tsc: uint64_t, tsc2: uint64_t, yield_0: bool);
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_cpu_tick() -> uint64_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.h:8"]
pub mod cycle_h {
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn ponyint_cycle_check_blocked(tsc: uint64_t, tsc2: uint64_t) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_cycle_terminate();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/asio.h:9"]
pub mod asio_h {
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_asio_init(cpu: uint32_t);
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_asio_start() -> bool;
        #[c2rust::src_loc = "98:1"]
        pub fn ponyint_asio_stop() -> bool;
        #[c2rust::src_loc = "105:1"]
        pub fn ponyint_asio_stoppable() -> bool;
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
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:13"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "6:1"]
        pub fn macro__DTRACE_ENABLED(_: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:14"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_int32_t_h::int32_t;
pub use self::_int64_t_h::int64_t;
pub use self::_intptr_t_h::intptr_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_condattr_t_h::pthread_condattr_t;
pub use self::_pthread_mutex_t_h::pthread_mutex_t;
pub use self::_pthread_mutexattr_t_h::pthread_mutexattr_t;
pub use self::_pthread_once_t_h::pthread_once_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_condattr_t, __darwin_pthread_handler_rec,
    __darwin_pthread_mutex_t, __darwin_pthread_mutexattr_t, __darwin_pthread_once_t,
    __darwin_pthread_t, _opaque_pthread_cond_t, _opaque_pthread_condattr_t,
    _opaque_pthread_mutex_t, _opaque_pthread_mutexattr_t, _opaque_pthread_once_t,
    _opaque_pthread_t,
};
pub use self::_types_h::__darwin_intptr_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::{
    pony_actor_t, ponyint_actor_getnoblock, ponyint_actor_run, ponyint_unmute_actor,
};
pub use self::actormap_h::actormap_t;
use self::asio_h::{
    ponyint_asio_init, ponyint_asio_start, ponyint_asio_stop, ponyint_asio_stoppable,
};
use self::cpu_h::{
    ponyint_cpu_affinity, ponyint_cpu_assign, ponyint_cpu_core_pause, ponyint_cpu_count,
    ponyint_cpu_tick,
};
use self::cycle_h::{ponyint_cycle_check_blocked, ponyint_cycle_terminate};
pub use self::delta_h::deltamap_t;
use self::dtrace_h::{macro__DTRACE, macro__DTRACE_ENABLED};
pub use self::gc_h::{gc_t, gcstack_t};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{chunk_t, heap_t};
pub use self::internal::__int128_t;
pub use self::messageq_h::{
    messageq_t, ponyint_messageq_destroy, ponyint_messageq_init, ponyint_thread_messageq_pop,
    ponyint_thread_messageq_push,
};
pub use self::mpmcq_h::{
    aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, ponyint_mpmcq_destroy, ponyint_mpmcq_init,
    ponyint_mpmcq_pop, ponyint_mpmcq_push, ponyint_mpmcq_push_single, C2RustUnnamed,
};
pub use self::mutemap_h::{
    mutemap_t, muteref_t, muteset_t, ponyint_mutemap_get, ponyint_mutemap_putindex,
    ponyint_mutemap_removeindex, ponyint_mutemap_size, ponyint_muteref_alloc, ponyint_muteref_free,
    ponyint_muteset_get, ponyint_muteset_next, ponyint_muteset_putindex,
};
pub use self::objectmap_h::objectmap_t;
pub use self::pony_h::{
    _pony_type_t, pony_alloc_msg, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_msgi_t, pony_serialise_fn, pony_trace_fn,
    pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
    ponyint_pool_thread_cleanup,
};
use self::pthread_h::{
    pthread_cond_destroy, pthread_cond_init, pthread_mutex_init, pthread_mutex_lock,
    pthread_mutex_trylock, pthread_mutex_unlock, pthread_once,
};
pub use self::scheduler_h::{
    pony_ctx_t, scheduler_t, schedulerstats_t, trace_actor_fn, trace_object_fn,
};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stack_h::{ponyint_stack_pop, ponyint_stack_push, Stack};
pub use self::stddef_h::size_t;
use self::string_h::memset;
pub use self::threads_h::{
    ponyint_thread_create, ponyint_thread_join, ponyint_thread_self, ponyint_thread_suspend,
    ponyint_thread_wake, thread_fn,
};
#[c2rust::src_loc = "25:9"]
pub type sched_msg_t = libc::c_uint;
#[c2rust::src_loc = "35:3"]
pub const SCHED_UNNOISY_ASIO: sched_msg_t = 52;
#[c2rust::src_loc = "34:3"]
pub const SCHED_NOISY_ASIO: sched_msg_t = 51;
#[c2rust::src_loc = "33:3"]
pub const SCHED_UNMUTE_ACTOR: sched_msg_t = 50;
#[c2rust::src_loc = "32:3"]
pub const SCHED_SUSPEND: sched_msg_t = 41;
#[c2rust::src_loc = "31:3"]
pub const SCHED_TERMINATE: sched_msg_t = 40;
#[c2rust::src_loc = "30:3"]
pub const SCHED_ACK: sched_msg_t = 31;
#[c2rust::src_loc = "29:3"]
pub const SCHED_CNF: sched_msg_t = 30;
#[c2rust::src_loc = "28:3"]
pub const SCHED_UNBLOCK: sched_msg_t = 21;
#[c2rust::src_loc = "27:3"]
pub const SCHED_BLOCK: sched_msg_t = 20;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1600:34"]
pub struct actorstack_t {}
#[c2rust::src_loc = "39:13"]
static mut pause_cycle_detection: bool = false;
#[c2rust::src_loc = "40:17"]
static mut last_cd_tsc: uint64_t = 0;
#[c2rust::src_loc = "41:17"]
static mut scheduler_count: uint32_t = 0;
#[c2rust::src_loc = "42:17"]
static mut min_scheduler_count: uint32_t = 0;
#[c2rust::src_loc = "43:17"]
static mut scheduler_suspend_threshold: uint64_t = 0;
#[c2rust::src_loc = "44:30"]
static mut active_scheduler_count: AtomicU32 = AtomicU32::new(0);
#[c2rust::src_loc = "45:30"]
static mut active_scheduler_count_check: AtomicU32 = AtomicU32::new(0);
#[c2rust::src_loc = "46:21"]
static mut scheduler: *mut scheduler_t = 0 as *const scheduler_t as *mut scheduler_t;
#[c2rust::src_loc = "47:20"]
static mut inject_context: *mut pony_ctx_t = 0 as *const pony_ctx_t as *mut pony_ctx_t;
#[c2rust::src_loc = "48:26"]
static mut detect_quiescence: AtomicBool = AtomicBool::new(false);
#[c2rust::src_loc = "49:13"]
static mut use_yield: bool = false;
#[c2rust::src_loc = "50:16"]
static mut inject: mpmcq_t = mpmcq_t {
    head: 0 as *const mpmcq_node_t as *mut mpmcq_node_t,
    tail: aba_protected_mpmcq_node_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *const mpmcq_node_t as *mut mpmcq_node_t,
            counter: 0,
        },
    },
};
#[thread_local]
#[c2rust::src_loc = "51:41"]
static mut this_scheduler: *mut scheduler_t = 0 as *const scheduler_t as *mut scheduler_t;
#[c2rust::src_loc = "54:24"]
static mut sched_mut: pthread_mutex_t = pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};
#[c2rust::src_loc = "56:23"]
static mut sched_mut_once: pthread_once_t = {
    let mut init = _opaque_pthread_once_t {
        __sig: 0x30b1bcba as libc::c_int as libc::c_long,
        __opaque: [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0],
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn sched_mut_init() {
    pthread_mutex_init(&mut sched_mut, 0 as *const pthread_mutexattr_t);
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn get_active_scheduler_count() -> uint32_t {
    active_scheduler_count.load(Relaxed)
}
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn get_active_scheduler_count_check() -> uint32_t {
    active_scheduler_count_check.load(Relaxed)
}
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn pop(mut sched: *mut scheduler_t) -> *mut pony_actor_t {
    return ponyint_mpmcq_pop(&mut (*sched).q) as *mut pony_actor_t;
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn push(mut sched: *mut scheduler_t, mut actor: *mut pony_actor_t) {
    ponyint_mpmcq_push_single(&mut (*sched).q, actor as *mut libc::c_void);
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn pop_global(mut sched: *mut scheduler_t) -> *mut pony_actor_t {
    let mut actor: *mut pony_actor_t = ponyint_mpmcq_pop(&mut inject) as *mut pony_actor_t;
    if !actor.is_null() {
        return actor;
    }
    if sched.is_null() {
        return 0 as *mut pony_actor_t;
    } else {
        return pop(sched);
    };
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn send_msg(
    mut _from: uint32_t,
    mut to: uint32_t,
    mut msg: sched_msg_t,
    mut arg: intptr_t,
) {
    let mut m: *mut pony_msgi_t =
        pony_alloc_msg(0 as libc::c_int as uint32_t, msg as uint32_t) as *mut pony_msgi_t;
    (*m).i = arg;
    ponyint_thread_messageq_push(
        &mut (*scheduler.offset(to as isize)).mq,
        &mut (*m).msg,
        &mut (*m).msg,
    );
}
#[c2rust::src_loc = "219:1"]
unsafe extern "C" fn send_msg_all_active(
    mut from: uint32_t,
    mut msg: sched_msg_t,
    mut arg: intptr_t,
) {
    let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < current_active_scheduler_count {
        send_msg(from, i, msg, arg);
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn send_msg_all(mut from: uint32_t, mut msg: sched_msg_t, mut arg: intptr_t) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < scheduler_count {
        send_msg(from, i, msg, arg);
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "233:1"]
unsafe extern "C" fn signal_suspended_threads(
    mut sched_count: uint32_t,
    mut curr_sched_id: int32_t,
) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < sched_count {
        if i as int32_t != curr_sched_id {
            ponyint_thread_wake(
                (*scheduler.offset(i as isize)).tid,
                (*scheduler.offset(i as isize)).sleep_object,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "246:1"]
unsafe extern "C" fn wake_suspended_threads(mut current_scheduler_id: int32_t) {
    let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
    loop {
        current_active_scheduler_count = get_active_scheduler_count();
        if !(current_active_scheduler_count < scheduler_count) {
            break;
        }
        if pthread_mutex_lock(&mut sched_mut) == 0 {
            current_active_scheduler_count = get_active_scheduler_count();
            if current_active_scheduler_count < scheduler_count {
                current_active_scheduler_count = scheduler_count;
                active_scheduler_count.store(current_active_scheduler_count, Relaxed);
            }
            pthread_mutex_unlock(&mut sched_mut);
            signal_suspended_threads(current_active_scheduler_count, current_scheduler_id);
        }
        while get_active_scheduler_count() != get_active_scheduler_count_check() {
            signal_suspended_threads(current_active_scheduler_count, current_scheduler_id);
        }
    }
}
#[c2rust::src_loc = "310:1"]
unsafe extern "C" fn maybe_start_cnf_ack_cycle(mut sched: *mut scheduler_t) {
    if detect_quiescence.load(Relaxed) && (*sched).block_count >= get_active_scheduler_count() {
        let ref mut fresh0 = (*sched).ack_token;
        *fresh0 += 1;
        (*sched).ack_count = 0 as libc::c_int as uint32_t;
        send_msg_all_active(
            (*sched).index as uint32_t,
            SCHED_CNF,
            (*sched).ack_token as intptr_t,
        );
    }
}
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn handle_sched_block(mut sched: *mut scheduler_t) {
    let ref mut fresh1 = (*sched).block_count;
    *fresh1 = (*fresh1).wrapping_add(1);
    maybe_start_cnf_ack_cycle(sched);
}
#[c2rust::src_loc = "336:1"]
unsafe extern "C" fn handle_sched_unblock(mut sched: *mut scheduler_t) {
    let ref mut fresh2 = (*sched).block_count;
    *fresh2 = (*fresh2).wrapping_sub(1);
    let ref mut fresh3 = (*sched).ack_token;
    *fresh3 += 1;
    (*sched).ack_count = 0 as libc::c_int as uint32_t;
}
#[c2rust::src_loc = "345:1"]
unsafe extern "C" fn read_msg(mut sched: *mut scheduler_t) -> bool {
    let mut m: *mut pony_msgi_t = 0 as *mut pony_msgi_t;
    let mut run_queue_changed: bool = 0 as libc::c_int != 0;
    loop {
        m = ponyint_thread_messageq_pop(&mut (*sched).mq) as *mut pony_msgi_t;
        if m.is_null() {
            break;
        }
        match (*m).msg.id {
            41 => {
                maybe_start_cnf_ack_cycle(sched);
            }
            20 => {
                handle_sched_block(sched);
            }
            21 => {
                handle_sched_unblock(sched);
            }
            30 => {
                send_msg(
                    (*sched).index as uint32_t,
                    0 as libc::c_int as uint32_t,
                    SCHED_ACK,
                    (*m).i,
                );
            }
            31 => {
                if (*m).i == (*sched).ack_token as libc::c_long {
                    let ref mut fresh4 = (*sched).ack_count;
                    *fresh4 = (*fresh4).wrapping_add(1);
                }
            }
            40 => {
                (*sched).terminate = 1 as libc::c_int != 0;
            }
            50 => {
                if ponyint_sched_unmute_senders(&mut (*sched).ctx, (*m).i as *mut pony_actor_t) {
                    run_queue_changed = 1 as libc::c_int != 0;
                }
            }
            51 => {
                (*sched).asio_noisy = 1 as libc::c_int != 0;
            }
            52 => {
                (*sched).asio_noisy = 0 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    return run_queue_changed;
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn quiescent(
    mut sched: *mut scheduler_t,
    mut tsc: uint64_t,
    mut tsc2: uint64_t,
) -> bool {
    if (*sched).terminate {
        return 1 as libc::c_int != 0;
    }
    let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
    if (*sched).ack_count >= current_active_scheduler_count {
        pause_cycle_detection = 1 as libc::c_int != 0;
        if (*sched).asio_stoppable as libc::c_int != 0 && ponyint_asio_stop() as libc::c_int != 0 {
            send_msg_all(
                (*sched).index as uint32_t,
                SCHED_TERMINATE,
                0 as libc::c_int as intptr_t,
            );
            wake_suspended_threads((*sched).index);
            let ref mut fresh5 = (*sched).ack_token;
            *fresh5 += 1;
            (*sched).ack_count = 0 as libc::c_int as uint32_t;
        } else if ponyint_asio_stoppable() {
            (*sched).asio_stoppable = 1 as libc::c_int != 0;
            let ref mut fresh6 = (*sched).ack_token;
            *fresh6 += 1;
            (*sched).ack_count = 0 as libc::c_int as uint32_t;
            send_msg_all_active(
                (*sched).index as uint32_t,
                SCHED_CNF,
                (*sched).ack_token as intptr_t,
            );
            pause_cycle_detection = 0 as libc::c_int != 0;
        } else {
            (*sched).asio_stoppable = 0 as libc::c_int != 0;
            pause_cycle_detection = 0 as libc::c_int != 0;
        }
    }
    ponyint_cpu_core_pause(tsc, tsc2, use_yield);
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "505:1"]
unsafe extern "C" fn choose_victim(mut sched: *mut scheduler_t) -> *mut scheduler_t {
    let mut victim: *mut scheduler_t = (*sched).last_victim;
    loop {
        victim = victim.offset(-1);
        let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
        if victim < scheduler {
            victim = &mut *scheduler.offset(
                current_active_scheduler_count.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut scheduler_t;
        }
        if victim == (*sched).last_victim
            || current_active_scheduler_count == 1 as libc::c_int as libc::c_uint
        {
            let ref mut fresh7 = (*sched).last_victim;
            *fresh7 = sched;
            break;
        } else {
            if victim == sched {
                continue;
            }
            let ref mut fresh8 = (*sched).last_victim;
            *fresh8 = victim;
            return victim;
        }
    }
    return 0 as *mut scheduler_t;
}
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn suspend_scheduler(
    mut sched: *mut scheduler_t,
    mut current_active_scheduler_count: uint32_t,
) -> *mut pony_actor_t {
    let mut actor: *mut pony_actor_t = 0 as *mut pony_actor_t;
    let mut sched_count: uint32_t = get_active_scheduler_count();
    if sched_count != current_active_scheduler_count {
        return actor;
    }
    active_scheduler_count.store(sched_count.wrapping_sub(1), Relaxed);
    let mut sched_count_check: uint32_t = get_active_scheduler_count_check();
    active_scheduler_count_check.store(sched_count_check.wrapping_sub(1), Relaxed);
    if sched_count == sched_count_check {
    } else {
        ponyint_assert_fail(
            b"sched_count == sched_count_check\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                as *const u8 as *const libc::c_char,
            586 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"suspend_scheduler\0"))
                .as_ptr(),
        );
    };
    if (*sched).index != 0 as libc::c_int {
        send_msg(
            (*sched).index as uint32_t,
            0 as libc::c_int as uint32_t,
            SCHED_SUSPEND,
            0 as libc::c_int as intptr_t,
        );
    }
    macro__DTRACE(
        b"THREAD_SUSPEND\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
    );
    while get_active_scheduler_count() <= (*sched).index as uint32_t {
        if (*sched).index == 0 as libc::c_int {
            actor = pop_global(0 as *mut scheduler_t);
            if !actor.is_null() {
                break;
            }
            if read_msg(sched) {
                actor = pop_global(sched);
                if !actor.is_null() {
                    break;
                }
            }
            if !(*sched).asio_noisy {
                break;
            }
        }
        ponyint_thread_suspend((*sched).sleep_object, &mut sched_mut);
    }
    macro__DTRACE(
        b"THREAD_RESUME\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
    );
    sched_count = get_active_scheduler_count();
    if sched_count == 0 as libc::c_int as libc::c_uint {
        sched_count = 1 as libc::c_int as uint32_t;
        active_scheduler_count.store(sched_count, Relaxed);
    }
    sched_count_check = get_active_scheduler_count_check();
    active_scheduler_count_check.store(sched_count_check.wrapping_add(1), Relaxed);
    actor
}
#[c2rust::src_loc = "711:1"]
unsafe extern "C" fn perhaps_suspend_scheduler(
    mut sched: *mut scheduler_t,
    mut current_active_scheduler_count: uint32_t,
    mut block_sent: *mut bool,
    mut steal_attempts: *mut uint32_t,
    mut sched_is_blocked: bool,
) -> *mut pony_actor_t {
    if current_active_scheduler_count > min_scheduler_count
        && sched
            == &mut *scheduler.offset(
                current_active_scheduler_count.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut scheduler_t
        && !(*sched).terminate
        && current_active_scheduler_count == get_active_scheduler_count_check()
        && pthread_mutex_trylock(&mut sched_mut) == 0
    {
        let mut actor: *mut pony_actor_t = 0 as *mut pony_actor_t;
        if (*sched).index > 0 as libc::c_int
            || (*sched).index == 0 as libc::c_int && (*sched).asio_noisy as libc::c_int != 0
        {
            if !sched_is_blocked {
                if (*sched).index == 0 as libc::c_int {
                    handle_sched_unblock(sched);
                } else {
                    send_msg(
                        (*sched).index as uint32_t,
                        0 as libc::c_int as uint32_t,
                        SCHED_UNBLOCK,
                        0 as libc::c_int as intptr_t,
                    );
                }
                *block_sent = 0 as libc::c_int != 0;
            }
            actor = suspend_scheduler(sched, current_active_scheduler_count);
            *steal_attempts = 0 as libc::c_int as uint32_t;
        } else {
            if (*sched).index == 0 as libc::c_int {
            } else {
                ponyint_assert_fail(
                    b"sched->index == 0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                        as *const u8 as *const libc::c_char,
                    759 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"perhaps_suspend_scheduler\0",
                    ))
                    .as_ptr(),
                );
            };
            if !(*sched).asio_noisy {
            } else {
                ponyint_assert_fail(
                    b"!sched->asio_noisy\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                        as *const u8 as *const libc::c_char,
                    760 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"perhaps_suspend_scheduler\0",
                    ))
                    .as_ptr(),
                );
            };
            if sched_is_blocked {
                handle_sched_block(sched);
                *block_sent = 1 as libc::c_int != 0;
            }
        }
        pthread_mutex_unlock(&mut sched_mut);
        if !actor.is_null() {
            return actor;
        }
    }
    return 0 as *mut pony_actor_t;
}
#[c2rust::src_loc = "789:1"]
unsafe extern "C" fn steal(mut sched: *mut scheduler_t) -> *mut pony_actor_t {
    let mut block_sent: bool = 0 as libc::c_int != 0;
    let mut steal_attempts: uint32_t = 0 as libc::c_int as uint32_t;
    let mut tsc: uint64_t = ponyint_cpu_tick();
    let mut actor: *mut pony_actor_t = 0 as *mut pony_actor_t;
    let mut victim: *mut scheduler_t = 0 as *mut scheduler_t;
    loop {
        victim = choose_victim(sched);
        actor = pop_global(victim);
        if !actor.is_null() {
            break;
        }
        let mut tsc2: uint64_t = ponyint_cpu_tick();
        if read_msg(sched) {
            actor = pop_global(sched);
            if !actor.is_null() {
                break;
            }
        }
        if quiescent(sched, tsc, tsc2) {
            macro__DTRACE(
                b"WORK_STEAL_FAILURE\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)victim\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut pony_actor_t;
        }
        let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
        let mut clocks_elapsed: uint64_t = tsc2.wrapping_sub(tsc);
        if !block_sent {
            if current_active_scheduler_count > (*sched).index as uint32_t {
            } else {
                ponyint_assert_fail(
                    b"current_active_scheduler_count > (uint32_t)sched->index\0" as *const u8
                        as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                        as *const u8 as *const libc::c_char,
                    864 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"steal\0")).as_ptr(),
                );
            };
            if steal_attempts < current_active_scheduler_count {
                steal_attempts = steal_attempts.wrapping_add(1);
            } else if clocks_elapsed > 1000000 as libc::c_int as libc::c_ulonglong
                && ponyint_mutemap_size(&mut (*sched).mute_mapping)
                    == 0 as libc::c_int as libc::c_ulong
            {
                if clocks_elapsed > scheduler_suspend_threshold {
                    current_active_scheduler_count = get_active_scheduler_count();
                    actor = perhaps_suspend_scheduler(
                        sched,
                        current_active_scheduler_count,
                        &mut block_sent,
                        &mut steal_attempts,
                        1 as libc::c_int != 0,
                    );
                    if !actor.is_null() {
                        break;
                    }
                }
                if !(*sched).asio_noisy {
                    if (*sched).index == 0 as libc::c_int {
                        handle_sched_block(sched);
                    } else {
                        send_msg(
                            (*sched).index as uint32_t,
                            0 as libc::c_int as uint32_t,
                            SCHED_BLOCK,
                            0 as libc::c_int as intptr_t,
                        );
                    }
                    block_sent = 1 as libc::c_int != 0;
                }
            }
        } else {
            if current_active_scheduler_count > (*sched).index as uint32_t {
            } else {
                ponyint_assert_fail(
                    b"current_active_scheduler_count > (uint32_t)sched->index\0" as *const u8
                        as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                        as *const u8 as *const libc::c_char,
                    906 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"steal\0")).as_ptr(),
                );
            };
            if clocks_elapsed > scheduler_suspend_threshold {
                actor = perhaps_suspend_scheduler(
                    sched,
                    current_active_scheduler_count,
                    &mut block_sent,
                    &mut steal_attempts,
                    0 as libc::c_int != 0,
                );
                if !actor.is_null() {
                    break;
                }
            }
        }
        if !(!ponyint_actor_getnoblock()
            && (*sched).index == 0 as libc::c_int
            && !pause_cycle_detection)
        {
            continue;
        }
        let mut current_tsc: uint64_t = ponyint_cpu_tick();
        if !ponyint_cycle_check_blocked(last_cd_tsc, current_tsc) {
            continue;
        }
        last_cd_tsc = current_tsc;
        actor = pop_global(sched);
        if !actor.is_null() {
            break;
        }
    }
    if block_sent {
        if (*sched).index == 0 as libc::c_int {
            handle_sched_unblock(sched);
        } else {
            send_msg(
                (*sched).index as uint32_t,
                0 as libc::c_int as uint32_t,
                SCHED_UNBLOCK,
                0 as libc::c_int as intptr_t,
            );
        }
    }
    macro__DTRACE(
        b"WORK_STEAL_SUCCESSFUL\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
        b"(uintptr_t)victim\0" as *const u8 as *const libc::c_char,
        b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
    );
    actor
}
#[c2rust::src_loc = "961:1"]
unsafe extern "C" fn run(mut sched: *mut scheduler_t) {
    if (*sched).index == 0 as libc::c_int {
        pause_cycle_detection = 0 as libc::c_int != 0;
        last_cd_tsc = 0 as libc::c_int as uint64_t;
    }
    let mut actor: *mut pony_actor_t = pop_global(sched);
    if macro__DTRACE_ENABLED(b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char)
        as libc::c_int
        != 0
        && !actor.is_null()
    {
        macro__DTRACE(
            b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        if (*sched).index == 0 as libc::c_int {
            if !ponyint_actor_getnoblock() {
                if !pause_cycle_detection {
                    let mut current_tsc: uint64_t = ponyint_cpu_tick();
                    if ponyint_cycle_check_blocked(last_cd_tsc, current_tsc) {
                        last_cd_tsc = current_tsc;
                        if actor.is_null() {
                            actor = pop_global(sched);
                        }
                    }
                }
            }
            let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
            let mut current_active_scheduler_count_check: uint32_t =
                get_active_scheduler_count_check();
            if current_active_scheduler_count != current_active_scheduler_count_check {
                signal_suspended_threads(current_active_scheduler_count, (*sched).index);
            }
        }
        if read_msg(sched) as libc::c_int != 0 && actor.is_null() {
            actor = pop_global(sched);
        }
        if actor.is_null() {
            actor = steal(sched);
            if actor.is_null() {
                if (pop(sched)).is_null() {
                } else {
                    ponyint_assert_fail(
                        b"pop(sched) == NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                            as *const u8 as *const libc::c_char,
                        1055 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 4],
                            &[libc::c_char; 4],
                        >(b"run\0"))
                            .as_ptr(),
                    );
                };
                return;
            }
            macro__DTRACE(
                b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
            );
        }
        if ponyint_mutemap_size(&mut (*sched).mute_mapping) > 0 as libc::c_int as libc::c_ulong {
            ponyint_sched_maybe_wakeup((*sched).index);
        }
        let mut reschedule: bool =
            ponyint_actor_run(&mut (*sched).ctx, actor, 0 as libc::c_int != 0);
        let ref mut fresh9 = (*sched).ctx.current;
        *fresh9 = 0 as *mut pony_actor_t;
        let mut next: *mut pony_actor_t = pop_global(sched);
        if reschedule {
            if !next.is_null() {
                push(sched, actor);
                macro__DTRACE(
                    b"ACTOR_DESCHEDULED\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                    b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                    b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                );
                actor = next;
                macro__DTRACE(
                    b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                    b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                    b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                );
                ponyint_sched_maybe_wakeup((*sched).index);
            }
        } else {
            macro__DTRACE(
                b"ACTOR_DESCHEDULED\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
            );
            actor = next;
            if macro__DTRACE_ENABLED(b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char)
                as libc::c_int
                != 0
                && !actor.is_null()
            {
                macro__DTRACE(
                    b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                    b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                    b"(uintptr_t)actor\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
}
#[c2rust::src_loc = "1112:1"]
unsafe extern "C" fn run_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sched: *mut scheduler_t = arg as *mut scheduler_t;
    this_scheduler = sched;
    ponyint_cpu_affinity((*sched).cpu);
    run(sched);
    ponyint_pool_thread_cleanup();
    return 0 as *mut libc::c_void;
}
#[c2rust::src_loc = "1133:1"]
unsafe extern "C" fn ponyint_sched_shutdown() {
    let mut start: uint32_t = 0;
    start = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = start;
    while i < scheduler_count {
        ponyint_thread_join((*scheduler.offset(i as isize)).tid);
        i = i.wrapping_add(1);
    }
    macro__DTRACE(
        b"RT_END\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    ponyint_cycle_terminate();
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < scheduler_count {
        while !(ponyint_thread_messageq_pop(&mut (*scheduler.offset(i_0 as isize)).mq)).is_null() {}
        ponyint_messageq_destroy(
            &mut (*scheduler.offset(i_0 as isize)).mq,
            0 as libc::c_int != 0,
        );
        ponyint_mpmcq_destroy(&mut (*scheduler.offset(i_0 as isize)).q);
        pthread_cond_destroy((*scheduler.offset(i_0 as isize)).sleep_object);
        ponyint_pool_free(
            1 as libc::c_int as size_t,
            (*scheduler.offset(i_0 as isize)).sleep_object as *mut libc::c_void,
        );
        let ref mut fresh10 = (*scheduler.offset(i_0 as isize)).sleep_object;
        *fresh10 = 0 as *mut pthread_cond_t;
        i_0 = i_0.wrapping_add(1);
    }
    ponyint_pool_free_size(
        (scheduler_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<scheduler_t>() as libc::c_ulong),
        scheduler as *mut libc::c_void,
    );
    scheduler = 0 as *mut scheduler_t;
    inject_context = 0 as *mut pony_ctx_t;
    scheduler_count = 0 as libc::c_int as uint32_t;
    active_scheduler_count.store(0, Relaxed);
    ponyint_mpmcq_destroy(&mut inject);
}
#[no_mangle]
#[c2rust::src_loc = "1185:1"]
pub unsafe extern "C" fn ponyint_sched_init(
    mut threads: uint32_t,
    mut noyield: bool,
    mut pin: bool,
    mut pinasio: bool,
    mut min_threads: uint32_t,
    mut thread_suspend_threshold: uint32_t,
    mut _stats_interval: uint32_t,
) -> *mut pony_ctx_t {
    pony_register_thread();
    use_yield = !noyield;
    if thread_suspend_threshold < 1 as libc::c_int as libc::c_uint {
        thread_suspend_threshold = 1 as libc::c_int as uint32_t;
    }
    if threads == 0 as libc::c_int as libc::c_uint {
        threads = ponyint_cpu_count();
    }
    if min_threads > threads {
        min_threads = threads;
    }
    scheduler_suspend_threshold =
        thread_suspend_threshold.wrapping_mul(1000000 as libc::c_int as libc::c_uint) as uint64_t;
    scheduler_count = threads;
    min_scheduler_count = min_threads;
    active_scheduler_count.store(scheduler_count, Relaxed);
    active_scheduler_count_check.store(scheduler_count, Relaxed);
    scheduler = ponyint_pool_alloc_size(
        (scheduler_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<scheduler_t>() as libc::c_ulong),
    ) as *mut scheduler_t;
    memset(
        scheduler as *mut libc::c_void,
        0 as libc::c_int,
        (scheduler_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<scheduler_t>() as libc::c_ulong),
    );
    let mut asio_cpu: uint32_t = ponyint_cpu_assign(scheduler_count, scheduler, pin, pinasio);
    pthread_once(
        &mut sched_mut_once,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(sched_mut_init))),
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < scheduler_count {
        let ref mut fresh11 = (*scheduler.offset(i as isize)).sleep_object;
        *fresh11 = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut pthread_cond_t;
        let mut ret: libc::c_int = pthread_cond_init(
            (*scheduler.offset(i as isize)).sleep_object,
            0 as *const pthread_condattr_t,
        );
        if ret != 0 as libc::c_int {
            ponyint_pool_free(
                1 as libc::c_int as size_t,
                (*scheduler.offset(i as isize)).sleep_object as *mut libc::c_void,
            );
            let ref mut fresh12 = (*scheduler.offset(i as isize)).sleep_object;
            *fresh12 = 0 as *mut pthread_cond_t;
        }
        let ref mut fresh13 = (*scheduler.offset(i as isize)).ctx.scheduler;
        *fresh13 = &mut *scheduler.offset(i as isize) as *mut scheduler_t;
        let ref mut fresh14 = (*scheduler.offset(i as isize)).last_victim;
        *fresh14 = &mut *scheduler.offset(i as isize) as *mut scheduler_t;
        (*scheduler.offset(i as isize)).index = i as int32_t;
        (*scheduler.offset(i as isize)).asio_noisy = 0 as libc::c_int != 0;
        ponyint_messageq_init(&mut (*scheduler.offset(i as isize)).mq);
        ponyint_mpmcq_init(&mut (*scheduler.offset(i as isize)).q);
        i = i.wrapping_add(1);
    }
    ponyint_mpmcq_init(&mut inject);
    ponyint_asio_init(asio_cpu);
    inject_context = pony_ctx();
    inject_context
}
#[no_mangle]
#[c2rust::src_loc = "1298:1"]
pub unsafe extern "C" fn ponyint_sched_start(mut library: bool) -> bool {
    pony_register_thread();
    if !ponyint_asio_start() {
        return 0 as libc::c_int != 0;
    }
    detect_quiescence.store(!library, Relaxed);
    macro__DTRACE(
        b"RT_START\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    let mut start: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = start;
    while i < scheduler_count {
        if ((*scheduler.offset(i as isize)).sleep_object).is_null() {
            return 0 as libc::c_int != 0;
        }
        if !ponyint_thread_create(
            &mut (*scheduler.offset(i as isize)).tid,
            Some(run_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            (*scheduler.offset(i as isize)).cpu,
            &mut *scheduler.offset(i as isize) as *mut scheduler_t as *mut libc::c_void,
        ) {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    if !library {
        ponyint_sched_shutdown();
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1336:1"]
pub unsafe extern "C" fn ponyint_sched_stop() {
    detect_quiescence.store(true, Release);
    ponyint_sched_shutdown();
}
#[no_mangle]
#[c2rust::src_loc = "1342:1"]
pub unsafe extern "C" fn ponyint_sched_add(mut ctx: *mut pony_ctx_t, mut actor: *mut pony_actor_t) {
    if !((*ctx).scheduler).is_null() {
        push((*ctx).scheduler, actor);
    } else {
        ponyint_mpmcq_push(&mut inject, actor as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1354:1"]
pub unsafe extern "C" fn pony_scheduler_stats() -> *mut schedulerstats_t {
    return 0 as *mut schedulerstats_t;
}
#[no_mangle]
#[c2rust::src_loc = "1364:1"]
pub unsafe extern "C" fn pony_scheduler_index() -> int32_t {
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    (*(*ctx).scheduler).index
}
#[no_mangle]
#[c2rust::src_loc = "1370:1"]
pub unsafe extern "C" fn pony_schedulers() -> uint32_t {
    scheduler_count
}
#[no_mangle]
#[c2rust::src_loc = "1375:1"]
pub unsafe extern "C" fn pony_active_schedulers() -> uint32_t {
    get_active_scheduler_count()
}
#[no_mangle]
#[c2rust::src_loc = "1380:1"]
pub unsafe extern "C" fn pony_min_schedulers() -> uint32_t {
    min_scheduler_count
}
#[no_mangle]
#[c2rust::src_loc = "1385:1"]
pub unsafe extern "C" fn pony_scheduler_yield() -> bool {
    use_yield
}
#[no_mangle]
#[c2rust::src_loc = "1390:1"]
pub unsafe extern "C" fn pony_register_thread() {
    if !this_scheduler.is_null() {
        return;
    }
    this_scheduler = ponyint_pool_alloc(4 as libc::c_int as size_t) as *mut scheduler_t;
    memset(
        this_scheduler as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<scheduler_t>() as libc::c_ulong,
    );
    let ref mut fresh15 = (*this_scheduler).tid;
    *fresh15 = ponyint_thread_self();
    (*this_scheduler).index = -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1402:1"]
pub unsafe extern "C" fn pony_unregister_thread() {
    if this_scheduler.is_null() {
        return;
    }
    ponyint_pool_free(
        4 as libc::c_int as size_t,
        this_scheduler as *mut libc::c_void,
    );
    this_scheduler = 0 as *mut scheduler_t;
    ponyint_pool_thread_cleanup();
}
#[no_mangle]
#[c2rust::src_loc = "1413:1"]
pub unsafe extern "C" fn pony_ctx() -> *mut pony_ctx_t {
    if !this_scheduler.is_null() {
    } else {
        ponyint_assert_fail(
            b"this_scheduler != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                as *const u8 as *const libc::c_char,
            1415 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"pony_ctx\0")).as_ptr(),
        );
    };
    &mut (*this_scheduler).ctx
}
#[no_mangle]
#[c2rust::src_loc = "1420:1"]
pub unsafe extern "C" fn ponyint_sched_noisy_asio(mut from: int32_t) {
    send_msg_all(
        from as uint32_t,
        SCHED_NOISY_ASIO,
        0 as libc::c_int as intptr_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1426:1"]
pub unsafe extern "C" fn ponyint_sched_unnoisy_asio(mut from: int32_t) {
    send_msg_all(
        from as uint32_t,
        SCHED_UNNOISY_ASIO,
        0 as libc::c_int as intptr_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1432:1"]
pub unsafe extern "C" fn ponyint_sched_maybe_wakeup_if_all_asleep(
    mut current_scheduler_id: int32_t,
) {
    let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
    loop {
        current_active_scheduler_count = get_active_scheduler_count();
        if !(current_active_scheduler_count == 0 as libc::c_int as libc::c_uint) {
            break;
        }
        ponyint_sched_maybe_wakeup(current_scheduler_id);
        current_active_scheduler_count = get_active_scheduler_count();
        if current_active_scheduler_count >= 1 as libc::c_int as libc::c_uint {
            while get_active_scheduler_count() != get_active_scheduler_count_check() {
                signal_suspended_threads(current_active_scheduler_count, current_scheduler_id);
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1462:1"]
pub unsafe extern "C" fn ponyint_sched_get_inject_context() -> *mut pony_ctx_t {
    inject_context
}
#[no_mangle]
#[c2rust::src_loc = "1467:1"]
pub unsafe extern "C" fn ponyint_sched_maybe_wakeup(mut current_scheduler_id: int32_t) {
    let mut current_active_scheduler_count: uint32_t = get_active_scheduler_count();
    if current_active_scheduler_count < scheduler_count
        && pthread_mutex_trylock(&mut sched_mut) == 0
    {
        current_active_scheduler_count = get_active_scheduler_count();
        if current_active_scheduler_count < scheduler_count {
            current_active_scheduler_count = current_active_scheduler_count.wrapping_add(1);
            active_scheduler_count.store(current_active_scheduler_count, Relaxed);
        }
        pthread_mutex_unlock(&mut sched_mut);
        signal_suspended_threads(current_active_scheduler_count, current_scheduler_id);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1536:1"]
pub unsafe extern "C" fn ponyint_sched_mute(
    mut ctx: *mut pony_ctx_t,
    mut sender: *mut pony_actor_t,
    mut recv: *mut pony_actor_t,
) {
    if sender != recv {
    } else {
        ponyint_assert_fail(
            b"sender != recv\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                as *const u8 as *const libc::c_char,
            1538 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"ponyint_sched_mute\0"))
                .as_ptr(),
        );
    };
    let mut sched: *mut scheduler_t = (*ctx).scheduler;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut key: muteref_t = muteref_t {
        key: 0 as *mut pony_actor_t,
        value: muteset_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
    };
    key.key = recv;
    let mut mref: *mut muteref_t =
        ponyint_mutemap_get(&mut (*sched).mute_mapping, &mut key, &mut index);
    if mref.is_null() {
        mref = ponyint_muteref_alloc(recv);
        ponyint_mutemap_putindex(&mut (*sched).mute_mapping, mref, index);
    }
    let mut index2: size_t = -(1 as libc::c_int) as size_t;
    let mut r: *mut pony_actor_t = ponyint_muteset_get(&mut (*mref).value, sender, &mut index2);
    if r.is_null() {
        ponyint_muteset_putindex(&mut (*mref).value, sender, index2);
        let ref mut fresh16 = (*sender).muted;
        *fresh16 = (*fresh16).wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1594:1"]
pub unsafe extern "C" fn ponyint_sched_start_global_unmute(
    mut from: uint32_t,
    mut actor: *mut pony_actor_t,
) {
    send_msg_all_active(from, SCHED_UNMUTE_ACTOR, actor as intptr_t);
}
#[no_mangle]
#[c2rust::src_loc = "1600:34"]
pub unsafe extern "C" fn ponyint_actorstack_pop(
    mut stack: *mut actorstack_t,
    mut data: *mut *mut pony_actor_t,
) -> *mut actorstack_t {
    ponyint_stack_pop(stack as *mut Stack, data as *mut *mut libc::c_void)
        as *mut actorstack_t
}
#[no_mangle]
#[c2rust::src_loc = "1600:34"]
pub unsafe extern "C" fn ponyint_actorstack_push(
    mut stack: *mut actorstack_t,
    mut data: *mut pony_actor_t,
) -> *mut actorstack_t {
    ponyint_stack_push(stack as *mut Stack, data as *mut libc::c_void) as *mut actorstack_t
}
#[no_mangle]
#[c2rust::src_loc = "1602:1"]
pub unsafe extern "C" fn ponyint_sched_unmute_senders(
    mut ctx: *mut pony_ctx_t,
    mut actor: *mut pony_actor_t,
) -> bool {
    let mut actors_rescheduled: size_t = 0 as libc::c_int as size_t;
    let mut sched: *mut scheduler_t = (*ctx).scheduler;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    let mut key: muteref_t = muteref_t {
        key: 0 as *mut pony_actor_t,
        value: muteset_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
    };
    key.key = actor;
    let mut mref: *mut muteref_t =
        ponyint_mutemap_get(&mut (*sched).mute_mapping, &mut key, &mut index);
    if !mref.is_null() {
        let mut i: size_t = -(1 as libc::c_int) as size_t;
        let mut muted: *mut pony_actor_t = 0 as *mut pony_actor_t;
        let mut needs_unmuting: *mut actorstack_t = 0 as *mut actorstack_t;
        loop {
            muted = ponyint_muteset_next(&mut (*mref).value, &mut i);
            if muted.is_null() {
                break;
            }
            if (*muted).muted > 0 as libc::c_int as libc::c_ulong {
            } else {
                ponyint_assert_fail(
                    b"muted->muted > 0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.c\0"
                        as *const u8 as *const libc::c_char,
                    1632 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                        b"ponyint_sched_unmute_senders\0",
                    ))
                    .as_ptr(),
                );
            };
            let ref mut fresh17 = (*muted).muted;
            *fresh17 = (*fresh17).wrapping_sub(1);
            if (*muted).muted == 0 as libc::c_int as libc::c_ulong {
                needs_unmuting = ponyint_actorstack_push(needs_unmuting, muted);
            }
        }
        ponyint_mutemap_removeindex(&mut (*sched).mute_mapping, index);
        ponyint_muteref_free(mref);
        let mut to_unmute: *mut pony_actor_t = 0 as *mut pony_actor_t;
        while !needs_unmuting.is_null() {
            needs_unmuting = ponyint_actorstack_pop(needs_unmuting, &mut to_unmute);
            ponyint_unmute_actor(to_unmute);
            ponyint_sched_add(ctx, to_unmute);
            macro__DTRACE(
                b"ACTOR_SCHEDULED\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"(uintptr_t)sched\0" as *const u8 as *const libc::c_char,
                b"(uintptr_t)to_unmute\0" as *const u8 as *const libc::c_char,
            );
            actors_rescheduled = actors_rescheduled.wrapping_add(1);
            ponyint_sched_start_global_unmute((*(*ctx).scheduler).index as uint32_t, to_unmute);
        }
    }
    return actors_rescheduled > 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
#[c2rust::src_loc = "1673:1"]
pub unsafe extern "C" fn pony_sched_index(mut ctx: *mut pony_ctx_t) -> int32_t {
    (*(*ctx).scheduler).index
}
