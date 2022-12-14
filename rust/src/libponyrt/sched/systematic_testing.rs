use ::libc;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:3"]
pub mod mpmcq_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct mpmcq_t {
        pub head: *mut mpmcq_node_t,
        pub tail: aba_protected_mpmcq_node_t,
    }
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:3"]
pub mod scheduler_h {
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
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_t_h::pthread_t;
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::{pony_actor_t, pony_type_t};
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "131:1"]
        pub fn pony_active_schedulers() -> u32;
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:3"]
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
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
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
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pthread/pthread.h:3"]
pub mod pthread_h {
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_mutexattr_t_h::pthread_mutexattr_t;
    use super::_pthread_once_t_h::pthread_once_t;
    use super::_pthread_t_h::pthread_t;

    extern "C" {
        #[c2rust::src_loc = "351:1"]
        pub fn pthread_equal(_: pthread_t, _: pthread_t) -> libc::c_int;
        #[c2rust::src_loc = "387:1"]
        pub fn pthread_mutex_init(
            _: *mut pthread_mutex_t,
            _: *const pthread_mutexattr_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "391:1"]
        pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "403:1"]
        pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "448:1"]
        pub fn pthread_once(
            _: *mut pthread_once_t,
            _: Option<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/threads.h:3"]
pub mod threads_h {
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_t_h::pthread_t;

    extern "C" {
        #[c2rust::src_loc = "61:1"]
        pub fn ponyint_thread_suspend(signal: *mut pthread_cond_t, mut_0: *mut pthread_mutex_t);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_thread_wake(thread: pthread_t, signal: *mut pthread_cond_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:3"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "162:1"]
        pub fn rand() -> libc::c_int;
        #[c2rust::src_loc = "164:1"]
        pub fn srand(_: libc::c_uint);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:6"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:7"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_cpu_core_pause(tsc: u64, tsc2: u64, yield_0: bool);
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_cpu_tick() -> u64;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:8"]
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
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_mutex_t_h::pthread_mutex_t;
pub use self::_pthread_mutexattr_t_h::pthread_mutexattr_t;
pub use self::_pthread_once_t_h::pthread_once_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_mutex_t,
    __darwin_pthread_mutexattr_t, __darwin_pthread_once_t, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_mutex_t, _opaque_pthread_mutexattr_t,
    _opaque_pthread_once_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actormap_h::actormap_t;
use self::cpu_h::{ponyint_cpu_core_pause, ponyint_cpu_tick};

pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc_size, ponyint_pool_free_size};
use self::pthread_h::{
    pthread_equal, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, pthread_once,
};
pub use self::scheduler_h::{
    pony_active_schedulers, pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn,
};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
use self::stdlib_h::{rand, srand};
use self::string_h::memset;
use self::threads_h::{ponyint_thread_suspend, ponyint_thread_wake};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "10:16"]
pub struct systematic_testing_thread_t {
    pub tid: pthread_t,
    pub sleep_object: *mut pthread_cond_t,
    pub stopped: bool,
}
#[c2rust::src_loc = "17:37"]
static mut active_thread: *mut systematic_testing_thread_t =
    0 as *const systematic_testing_thread_t as *mut systematic_testing_thread_t;
#[c2rust::src_loc = "18:37"]
static mut threads_to_track: *mut systematic_testing_thread_t =
    0 as *const systematic_testing_thread_t as *mut systematic_testing_thread_t;
#[c2rust::src_loc = "19:17"]
static mut total_threads: u32 = 0 as libc::c_int as u32;
#[c2rust::src_loc = "20:17"]
static mut stopped_threads: u32 = 0 as libc::c_int as u32;
#[c2rust::src_loc = "21:30"]
static mut waiting_to_start_count: AtomicU32 = AtomicU32::new(0);
#[c2rust::src_loc = "24:24"]
static mut systematic_testing_mut: pthread_mutex_t = pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};
#[c2rust::src_loc = "26:23"]
static mut systematic_testing_mut_once: pthread_once_t = {
    let mut init = _opaque_pthread_once_t {
        __sig: 0x30b1bcba as libc::c_int as libc::c_long,
        __opaque: [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0],
    };
    init
};
#[no_mangle]
#[c2rust::src_loc = "28:1"]
pub unsafe extern "C" fn systematic_testing_mut_init() {
    pthread_mutex_init(&mut systematic_testing_mut, 0 as *const pthread_mutexattr_t);
}
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_init(
    mut random_seed: u64,
    mut max_threads: u32,
) {
    pthread_once(
        &mut systematic_testing_mut_once,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(systematic_testing_mut_init))),
    );
    waiting_to_start_count.store(0, Relaxed);
    if 0 as libc::c_int as libc::c_ulonglong == random_seed {
        random_seed = ponyint_cpu_tick();
    }
    srand(random_seed as libc::c_int as libc::c_uint);
    total_threads = max_threads.wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut mem_needed: size_t = (total_threads as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<systematic_testing_thread_t>() as libc::c_ulong);
    threads_to_track = ponyint_pool_alloc_size(mem_needed) as *mut systematic_testing_thread_t;
    memset(
        threads_to_track as *mut libc::c_void,
        0 as libc::c_int,
        mem_needed,
    );
    active_thread = threads_to_track;
}
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_wait_start(
    mut thread: pthread_t,
    mut signal: *mut pthread_cond_t,
) {
    waiting_to_start_count.fetch_add(1, Relaxed);
    while 0 as libc::c_int == pthread_equal((*active_thread).tid, thread) {
        ponyint_thread_suspend(signal, &mut systematic_testing_mut);
    }
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_start(
    mut schedulers: *mut scheduler_t,
    mut asio_thread: pthread_t,
    mut asio_signal: *mut pthread_cond_t,
) {
    let ref mut fresh0 = (*threads_to_track.offset(0 as libc::c_int as isize)).tid;
    *fresh0 = asio_thread;
    let ref mut fresh1 = (*threads_to_track.offset(0 as libc::c_int as isize)).sleep_object;
    *fresh1 = asio_signal;
    (*threads_to_track.offset(0 as libc::c_int as isize)).stopped = 0 as libc::c_int != 0;
    let mut i: u32 = 1 as libc::c_int as u32;
    while i < total_threads {
        let ref mut fresh2 = (*threads_to_track.offset(i as isize)).tid;
        *fresh2 =
            (*schedulers.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)).tid;
        let ref mut fresh3 = (*threads_to_track.offset(i as isize)).sleep_object;
        *fresh3 = (*schedulers.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .sleep_object;
        (*threads_to_track.offset(i as isize)).stopped = 0 as libc::c_int != 0;
        i = i.wrapping_add(1);
    }
    active_thread = &mut *threads_to_track.offset(1 as libc::c_int as isize)
        as *mut systematic_testing_thread_t;
    while total_threads != waiting_to_start_count.load(Relaxed) {
        ponyint_cpu_core_pause(
            1 as libc::c_int as u64,
            10000002 as libc::c_int as u64,
            1 as libc::c_int != 0,
        );
    }
    ponyint_thread_wake((*active_thread).tid, (*active_thread).sleep_object);
}
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn get_next_index() -> u32 {
    let mut active_scheduler_count: u32 = pony_active_schedulers();
    let mut active_count: u32 =
        active_scheduler_count.wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut next_index: u32 = 0 as libc::c_int as u32;
    loop {
        next_index = (rand() as libc::c_uint).wrapping_rem(active_count);
        if next_index <= total_threads {
        } else {
            ponyint_assert_fail(
                b"next_index <= total_threads\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/systematic_testing.c\0"
                    as *const u8 as *const libc::c_char,
                148 as libc::c_int as size_t,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"get_next_index\0"))
                    .as_ptr(),
            );
        };
        if !(*threads_to_track.offset(next_index as isize)).stopped {
            break;
        }
    }
    next_index
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_yield() {
    if stopped_threads == total_threads {
        let mut mem_needed: size_t = (total_threads as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<systematic_testing_thread_t>() as libc::c_ulong);
        ponyint_pool_free_size(mem_needed, threads_to_track as *mut libc::c_void);
        active_thread = 0 as *mut systematic_testing_thread_t;
        threads_to_track = 0 as *mut systematic_testing_thread_t;
        total_threads = 0 as libc::c_int as u32;
        stopped_threads = 0 as libc::c_int as u32;
        waiting_to_start_count.store(0, Relaxed);
        pthread_mutex_unlock(&mut systematic_testing_mut);
        return;
    }
    let mut next_index: u32 = get_next_index();
    let mut next_thread: *mut systematic_testing_thread_t =
        &mut *threads_to_track.offset(next_index as isize) as *mut systematic_testing_thread_t;
    let mut current_thread: *mut systematic_testing_thread_t = active_thread;
    if 0 as libc::c_int == pthread_equal((*active_thread).tid, (*next_thread).tid) {
        active_thread = next_thread;
        ponyint_thread_wake((*active_thread).tid, (*active_thread).sleep_object);
        if !(*current_thread).stopped {
            ponyint_thread_suspend((*current_thread).sleep_object, &mut systematic_testing_mut);
        } else {
            pthread_mutex_unlock(&mut systematic_testing_mut);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_suspend(mut mut_0: *mut pthread_mutex_t) {
    pthread_mutex_unlock(mut_0);
    ponyint_systematic_testing_yield();
    pthread_mutex_lock(mut_0);
}
#[no_mangle]
#[c2rust::src_loc = "234:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_asio_stopped() -> bool {
    return (*threads_to_track.offset(0 as libc::c_int as isize)).stopped;
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn ponyint_systematic_testing_stop_thread() {
    (*active_thread).stopped = 1 as libc::c_int != 0;
    stopped_threads = stopped_threads.wrapping_add(1);
    ponyint_systematic_testing_yield();
}
