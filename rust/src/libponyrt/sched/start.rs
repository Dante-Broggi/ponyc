use ::libc;
use atomic_enum::atomic_enum;
use core::sync::atomic::{
    AtomicBool, AtomicI32,
    Ordering::{Acquire, Relaxed, Release},
};
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdint.h:3"]
pub mod stdint_h {
    #[c2rust::src_loc = "44:1"]
    pub type uint_fast8_t = u8;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
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
        pub counter: libc::uintptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct mpmcq_t {
        pub head: *mut mpmcq_node_t,
        pub tail: aba_protected_mpmcq_node_t,
    }

    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type mpmcq_node_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:3"]
pub mod scheduler_h {
    #[c2rust::src_loc = "84:8"]
    pub use crate::libponyrt::sched::scheduler::scheduler_t;
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

    extern "C" {
        #[c2rust::src_loc = "110:1"]
        pub fn ponyint_sched_init(
            threads: u32,
            noyield: bool,
            nopin: bool,
            pinasio: bool,
            min_threads: u32,
            thread_suspend_threshold: u32,
            stats_interval: u32,
        ) -> *mut pony_ctx_t;
        #[c2rust::src_loc = "119:1"]
        pub fn ponyint_sched_start(library: bool) -> bool;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_sched_stop();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:3"]
pub use crate::libponyrt::actor::messageq::messageq_h;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "159:16"]
    pub struct pony_language_features_init_t {
        pub init_network: bool,
        pub init_serialisation: bool,
        pub descriptor_table: *mut *const pony_type_t,
        pub descriptor_table_size: usize,
    }

    use super::actor_h::pony_actor_t;
    use super::scheduler_h::pony_ctx_t;

    extern "C" {
        #[c2rust::src_loc = "449:1"]
        pub fn pony_register_thread();
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:3"]
pub mod serialise_h {
    #[c2rust::src_loc = "18:1"]
    pub type serialise_throw_fn = Option<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "16:1"]
    pub type serialise_alloc_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, usize) -> *mut libc::c_void>;

    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    #[c2rust::src_loc = "24:36"]
    pub use crate::libponyrt::gc::serialise::ponyint_serialise_t;

    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_serialise_setup(table: *mut *const pony_type_t, table_size: usize) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:3"]
pub mod actormap_h {
    #[c2rust::src_loc = "27:35"]
    pub use crate::libponyrt::gc::actormap::actormap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:3"]
pub mod gc_h {
    #[c2rust::src_loc = "16:16"]
    pub use crate::libponyrt::gc::gc::gc_t;

    #[c2rust::src_loc = "28:32"]
    pub use crate::libponyrt::gc::gc::gcstack_t;
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
    use super::gc_h::gc_t;
    use super::heap_h::heap_t;
    use super::messageq_h::messageq_t;
    use super::pony_h::pony_type_t;

    extern "C" {
        #[c2rust::src_loc = "131:1"]
        pub fn ponyint_actor_setnoblock(state: bool);
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
        pub used: usize,
        pub next_gc: usize,
    }

    extern "C" {
        #[c2rust::src_loc = "18:16"]
        pub type chunk_t;
        #[c2rust::src_loc = "40:1"]
        pub fn ponyint_heap_setnextgcfactor(factor: libc::c_double);
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_heap_setinitialgc(size: usize);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/options/options.h:11"]
pub mod options_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct opt_arg_t {
        pub long_opt: *const libc::c_char,
        pub short_opt: libc::c_char,
        pub flag: u32,
        pub id: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:16"]
    pub struct opt_state_t {
        pub args: *const opt_arg_t,
        pub argc: *mut libc::c_int,
        pub argv: *mut *mut libc::c_char,
        pub arg_val: *mut libc::c_char,
        pub opt_start: *mut libc::c_char,
        pub opt_end: *mut libc::c_char,
        pub match_type: libc::c_int,
        pub idx: libc::c_int,
        pub remove: libc::c_int,
    }
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_opt_init(
            args_0: *const opt_arg_t,
            s: *mut opt_state_t,
            argc: *mut libc::c_int,
            argv: *mut *mut libc::c_char,
        );
        #[c2rust::src_loc = "89:1"]
        pub fn ponyint_opt_next(s: *mut opt_state_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:3"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:3"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "134:1"]
        pub fn atof(_: *const libc::c_char) -> libc::c_double;
        #[c2rust::src_loc = "135:1"]
        pub fn atoi(_: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "145:7"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:4"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn ponyint_cpu_init();
        #[c2rust::src_loc = "13:1"]
        pub fn ponyint_cpu_count() -> u32;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/cycle.h:7"]
pub mod cycle_h {
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn ponyint_cycle_create(ctx: *mut pony_ctx_t, detect_interval: u32);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/lang/socket.h:10"]
pub mod socket_h {
    extern "C" {
        #[c2rust::src_loc = "6:1"]
        pub fn ponyint_os_sockets_init() -> bool;
        #[c2rust::src_loc = "8:1"]
        pub fn ponyint_os_sockets_final();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:12"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:13"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:14"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:16"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actor_h::{pony_actor_t, ponyint_actor_setnoblock};
pub use self::actormap_h::actormap_t;
use self::atomics_h::f__atomic_thread_fence;
use self::cpu_h::{ponyint_cpu_count, ponyint_cpu_init};
use self::cycle_h::ponyint_cycle_create;
pub use self::delta_h::deltamap_t;
use self::dtrace_h::macro__DTRACE;
pub use self::gc_h::{gc_t, gcstack_t};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::heap_h::{chunk_t, heap_t, ponyint_heap_setinitialgc, ponyint_heap_setnextgcfactor};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::objectmap_h::objectmap_t;
pub use self::options_h::{opt_arg_t, opt_state_t, ponyint_opt_init, ponyint_opt_next};
pub use self::pony_h::{
    _pony_type_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn, pony_dispatch_fn,
    pony_final_fn, pony_language_features_init_t, pony_msg_t, pony_register_thread,
    pony_serialise_fn, pony_trace_fn, pony_type_t,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::scheduler_h::{
    pony_ctx_t, ponyint_sched_init, ponyint_sched_start, ponyint_sched_stop, scheduler_t,
    trace_actor_fn, trace_object_fn,
};
pub use self::serialise_h::{
    ponyint_serialise_setup, ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn,
};
use self::socket_h::{ponyint_os_sockets_final, ponyint_os_sockets_init};
pub use self::stddef_h::size_t;
pub use self::stdint_h::uint_fast8_t;
use self::stdio_h::printf;
use self::stdlib_h::{atof, atoi, exit};
use self::string_h::{memcpy, memset};
extern "C" {
    #[c2rust::src_loc = "105:1"]
    pub fn Main_runtime_override_defaults_oo(opt: *mut options_t);
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "22:16"]
pub struct options_t {
    pub threads: u32,
    pub min_threads: u32,
    pub noscale: bool,
    pub thread_suspend_threshold: u32,
    pub cd_detect_interval: u32,
    pub gc_initial: usize,
    pub gc_factor: libc::c_double,
    pub noyield: bool,
    pub noblock: bool,
    pub pin: bool,
    pub pinasio: bool,
    pub stats_interval: u32,
    pub version: bool,
    pub ponyhelp: bool,
}
#[c2rust::src_loc = "60:3"]
pub const OPT_MAXTHREADS: opt_num_t = 0;
#[c2rust::src_loc = "76:3"]
pub const OPT_PONYHELP: opt_num_t = 13;
#[c2rust::src_loc = "72:3"]
pub const OPT_VERSION: opt_num_t = 12;
#[c2rust::src_loc = "71:3"]
pub const OPT_STATSINTERVAL: opt_num_t = 11;
#[c2rust::src_loc = "70:3"]
pub const OPT_PINASIO: opt_num_t = 10;
#[c2rust::src_loc = "69:3"]
pub const OPT_PIN: opt_num_t = 9;
#[c2rust::src_loc = "68:3"]
pub const OPT_NOBLOCK: opt_num_t = 8;
#[c2rust::src_loc = "67:3"]
pub const OPT_NOYIELD: opt_num_t = 7;
#[c2rust::src_loc = "66:3"]
pub const OPT_GCFACTOR: opt_num_t = 6;
#[c2rust::src_loc = "65:3"]
pub const OPT_GCINITIAL: opt_num_t = 5;
#[c2rust::src_loc = "64:3"]
pub const OPT_CDINTERVAL: opt_num_t = 4;
#[c2rust::src_loc = "63:3"]
pub const OPT_SUSPENDTHRESHOLD: opt_num_t = 3;
#[c2rust::src_loc = "62:3"]
pub const OPT_NOSCALE: opt_num_t = 2;
#[c2rust::src_loc = "61:3"]
pub const OPT_MINTHREADS: opt_num_t = 1;
#[c2rust::src_loc = "44:9"]
#[atomic_enum]
#[derive(Eq, PartialEq)]
pub enum RunningKind {
    #[c2rust::src_loc = "46:3"]
    NOT_RUNNING = 0,
    #[c2rust::src_loc = "47:3"]
    RUNNING_DEFAULT = 1,
    #[c2rust::src_loc = "48:3"]
    RUNNING_LIBRARY = 2,
}
pub use RunningKind::*;
#[c2rust::src_loc = "58:1"]
pub type opt_num_t = uint_fast8_t;
#[c2rust::src_loc = "52:26"]
static mut initialised: AtomicBool = AtomicBool::new(false);
#[c2rust::src_loc = "53:36"]
static mut running: AtomicRunningKind = AtomicRunningKind::new(NOT_RUNNING);
#[c2rust::src_loc = "54:25"]
static mut rt_exit_code: AtomicI32 = AtomicI32::new(0);
#[c2rust::src_loc = "56:38"]
static mut language_init: pony_language_features_init_t = pony_language_features_init_t {
    init_network: false,
    init_serialisation: false,
    descriptor_table: 0 as *const *const pony_type_t as *mut *const pony_type_t,
    descriptor_table_size: 0,
};
#[c2rust::src_loc = "79:18"]
static mut args: [opt_arg_t; 15] = [
    {
        let mut init = opt_arg_t {
            long_opt: b"ponymaxthreads\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_MAXTHREADS as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponyminthreads\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_MINTHREADS as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponynoscale\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_NOSCALE as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponysuspendthreshold\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_SUSPENDTHRESHOLD as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponycdinterval\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_CDINTERVAL as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponygcinitial\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_GCINITIAL as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponygcfactor\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_GCFACTOR as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponynoyield\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_NOYIELD as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponynoblock\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_NOBLOCK as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponypin\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_PIN as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponypinasio\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_PINASIO as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponyprintstatsinterval\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as u32,
            id: OPT_STATSINTERVAL as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponyversion\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_VERSION as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: b"ponyhelp\0" as *const u8 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as u32,
            id: OPT_PONYHELP as uint_fast8_t as u32,
        };
        init
    },
    {
        let mut init = opt_arg_t {
            long_opt: 0 as *const libc::c_char,
            short_opt: 0 as libc::c_int as libc::c_char,
            flag: 4294967295 as libc::c_uint,
            id: 4294967295 as libc::c_uint,
        };
        init
    },
];
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn arg_name(id: libc::c_int) -> *const libc::c_char {
    args[id as usize].long_opt
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn err_out(mut id: libc::c_int, mut msg: *const libc::c_char) {
    printf(
        b"--%s %s\n\0" as *const u8 as *const libc::c_char,
        arg_name(id),
        msg,
    );
    exit(255 as libc::c_int);
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn parse_uint(
    mut target: *mut u32,
    mut min: libc::c_int,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut v: libc::c_int = atoi(value);
    if v < (if min < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        min
    }) {
        return 1 as libc::c_int;
    }
    *target = v as u32;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn parse_size(
    mut target: *mut usize,
    mut min: libc::c_int,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut v: libc::c_int = atoi(value);
    if v < (if min < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        min
    }) {
        return 1 as libc::c_int;
    }
    *target = v as usize;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn parse_udouble(
    mut target: *mut libc::c_double,
    mut min: libc::c_double,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut v: libc::c_double = atof(value);
    if v < (if min < 0 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else {
        min
    }) {
        return 1 as libc::c_int;
    }
    *target = v;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "162:1"]
unsafe extern "C" fn parse_opts(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut opt: *mut options_t,
) -> libc::c_int {
    let mut s: opt_state_t = opt_state_t {
        args: 0 as *const opt_arg_t,
        argc: 0 as *mut libc::c_int,
        argv: 0 as *mut *mut libc::c_char,
        arg_val: 0 as *mut libc::c_char,
        opt_start: 0 as *mut libc::c_char,
        opt_end: 0 as *mut libc::c_char,
        match_type: 0,
        idx: 0,
        remove: 0,
    };
    let mut id: libc::c_int = 0;
    ponyint_opt_init(args.as_mut_ptr(), &mut s, &mut argc, argv);
    let mut minthreads_set: bool = 0 as libc::c_int != 0;
    loop {
        id = ponyint_opt_next(&mut s);
        if !(id != -(1 as libc::c_int)) {
            break;
        }
        match id {
            0 => {
                if parse_uint(&mut (*opt).threads, 1 as libc::c_int, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 1\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            1 => {
                if parse_uint(&mut (*opt).min_threads, 0 as libc::c_int, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 0\0" as *const u8 as *const libc::c_char,
                    );
                }
                minthreads_set = 1 as libc::c_int != 0;
            }
            2 => {
                (*opt).noscale = 1 as libc::c_int != 0;
            }
            3 => {
                if parse_uint(
                    &mut (*opt).thread_suspend_threshold,
                    0 as libc::c_int,
                    s.arg_val,
                ) != 0
                {
                    err_out(
                        id,
                        b"can't be less than 0\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            4 => {
                if parse_uint(&mut (*opt).cd_detect_interval, 0 as libc::c_int, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 0\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            5 => {
                if parse_size(&mut (*opt).gc_initial, 0 as libc::c_int, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 0\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            6 => {
                if parse_udouble(&mut (*opt).gc_factor, 1.0f64, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 1.0\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            7 => {
                (*opt).noyield = 1 as libc::c_int != 0;
            }
            8 => {
                (*opt).noblock = 1 as libc::c_int != 0;
            }
            9 => {
                (*opt).pin = 1 as libc::c_int != 0;
            }
            10 => {
                (*opt).pinasio = 1 as libc::c_int != 0;
            }
            11 => {
                if parse_uint(&mut (*opt).stats_interval, 1 as libc::c_int, s.arg_val) != 0 {
                    err_out(
                        id,
                        b"can't be less than 1 second\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            12 => {
                (*opt).version = 1 as libc::c_int != 0;
            }
            13 => {
                (*opt).ponyhelp = 1 as libc::c_int != 0;
            }
            -2 => {
                exit(-(1 as libc::c_int));
            }
            _ => {
                exit(-(1 as libc::c_int));
            }
        }
    }
    if (*opt).noscale {
        if minthreads_set {
            printf(
                b"--%s & --%s are mutually exclusive\n\0" as *const u8 as *const libc::c_char,
                arg_name(OPT_MINTHREADS as uint_fast8_t as libc::c_int),
                arg_name(OPT_NOSCALE as uint_fast8_t as libc::c_int),
            );
            exit(-(1 as libc::c_int));
        }
        (*opt).min_threads = (*opt).threads;
    }
    let ref mut fresh0 = *argv.offset(argc as isize);
    *fresh0 = 0 as *mut libc::c_char;
    return argc;
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn pony_init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut prev_init: bool = initialised.swap(true, Relaxed);
    if !prev_init {
    } else {
        ponyint_assert_fail(
            b"!prev_init\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            221 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pony_init\0")).as_ptr(),
        );
    };
    if (running.load(Relaxed)) == NOT_RUNNING {
    } else {
        ponyint_assert_fail(
            b"atomic_load_explicit(&running, memory_order_relaxed) == NOT_RUNNING\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            223 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pony_init\0")).as_ptr(),
        );
    };
    f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
    macro__DTRACE(
        b"RT_INIT\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    let mut opt: options_t = options_t {
        threads: 0,
        min_threads: 0,
        noscale: false,
        thread_suspend_threshold: 0,
        cd_detect_interval: 0,
        gc_initial: 0,
        gc_factor: 0.,
        noyield: false,
        noblock: false,
        pin: false,
        pinasio: false,
        stats_interval: 0,
        version: false,
        ponyhelp: false,
    };
    memset(
        &mut opt as *mut options_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<options_t>().try_into().unwrap(),
    );
    opt.min_threads = 0 as libc::c_int as u32;
    opt.cd_detect_interval = 100 as libc::c_int as u32;
    opt.gc_initial = 14 as libc::c_int as usize;
    opt.gc_factor = 2.0f32 as libc::c_double;
    opt.pin = 0 as libc::c_int != 0;
    opt.stats_interval = 4294967295 as libc::c_uint;
    pony_register_thread();
    Main_runtime_override_defaults_oo(&mut opt);
    argc = parse_opts(argc, argv, &mut opt);
    if opt.ponyhelp {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            b"Runtime options for Pony programs (not for use with ponyc):\n  --ponymaxthreads Use N scheduler threads. Defaults to the number of\n                   cores (not hyperthreads) available.\n                   This can't be larger than the number of cores available.\n  --ponyminthreads Minimum number of active scheduler threads allowed.\n                   Defaults to 0, meaning that all scheduler threads are\n                   allowed to be suspended when no work is available.\n                   This can't be larger than --ponymaxthreads if provided,\n                   or the physical cores available\n  --ponynoscale    Don't scale down the scheduler threads.\n                   See --ponymaxthreads on how to specify the number of threads\n                   explicitly. Can't be used with --ponyminthreads.\n  --ponysuspendthreshold\n                   Amount of idle time before a scheduler thread suspends\n                   itself to minimize resource consumption (max 1000 ms,\n                   min 1 ms).\n                   Defaults to 1 ms.\n  --ponycdinterval Run cycle detection every N ms (max 1000 ms, min 10 ms).\n                   Defaults to 100 ms.\n  --ponygcinitial  Defer garbage collection until an actor is using at\n                   least 2^N bytes. Defaults to 2^14.\n  --ponygcfactor   After GC, an actor will next be GC'd at a heap memory\n                   usage N times its current value. This is a floating\n                   point value. Defaults to 2.0.\n  --ponynoyield    Do not yield the CPU when no work is available.\n  --ponynoblock    Do not send block messages to the cycle detector.\n  --ponypin        Pin scheduler threads to CPU cores. The ASIO thread\n                   can also be pinned if `--ponypinasio` is set.\n  --ponypinasio    Pin the ASIO thread to a CPU the way scheduler\n                   threads are pinned to CPUs. Requires `--ponypin` to\n                   be set to have any effect.\n  --ponyprintstatsinterval\n                   Print actor stats before an actor is destroyed and\n                   print scheduler stats every X seconds. Defaults to -1 (never).\n  --ponyversion    Print the version of the compiler and exit.\n  --ponyhelp       Print the runtime usage options and exit.\n\nNOTE: These can be programmatically overridden. See the docstring in the\n      `RuntimeOptions` struct in the `builtin` package.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if opt.version {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b"UNKNOWN\0" as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    ponyint_cpu_init();
    if opt.threads == 0 as libc::c_int as libc::c_uint {
        opt.threads = ponyint_cpu_count();
    } else if opt.threads > ponyint_cpu_count() {
        printf(
            b"Can't have --%s > physical cores, the number of threads you'd be running with (%u > %u)\n\0"
                as *const u8 as *const libc::c_char,
            arg_name(OPT_MAXTHREADS as uint_fast8_t as libc::c_int),
            opt.threads,
            ponyint_cpu_count(),
        );
        exit(-(1 as libc::c_int));
    }
    if opt.min_threads > opt.threads {
        printf(
            b"Can't have --%s > --%s (%u > %u)\n\0" as *const u8 as *const libc::c_char,
            arg_name(OPT_MINTHREADS as uint_fast8_t as libc::c_int),
            arg_name(OPT_MAXTHREADS as uint_fast8_t as libc::c_int),
            opt.min_threads,
            opt.threads,
        );
        exit(-(1 as libc::c_int));
    }
    if opt.stats_interval != 4294967295 as libc::c_uint {
        printf(
            b"Printing runtime stats requires building with RUNTIMESTATS enabled. Ignoring.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    ponyint_heap_setinitialgc(opt.gc_initial);
    ponyint_heap_setnextgcfactor(opt.gc_factor);
    ponyint_actor_setnoblock(opt.noblock);
    pony_exitcode(0 as libc::c_int);
    let mut ctx: *mut pony_ctx_t = ponyint_sched_init(
        opt.threads,
        opt.noyield,
        opt.pin,
        opt.pinasio,
        opt.min_threads,
        opt.thread_suspend_threshold,
        opt.stats_interval,
    );
    ponyint_cycle_create(ctx, opt.cd_detect_interval);
    argc
}
#[no_mangle]
#[c2rust::src_loc = "308:1"]
pub unsafe extern "C" fn pony_start(
    mut library: bool,
    mut exit_code: *mut libc::c_int,
    mut language_features: *const pony_language_features_init_t,
) -> bool {
    if (&mut initialised).load(Relaxed) {
    } else {
        ponyint_assert_fail(
            b"atomic_load_explicit(&initialised, memory_order_relaxed)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            311 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_start\0")).as_ptr(),
        );
    };
    let mut prev_running: RunningKind = running.swap(RUNNING_DEFAULT, Relaxed);
    if prev_running as libc::c_uint == NOT_RUNNING as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"prev_running == NOT_RUNNING\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            318 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pony_start\0")).as_ptr(),
        );
    };
    if !language_features.is_null() {
        memcpy(
            &mut language_init as *mut pony_language_features_init_t as *mut libc::c_void,
            language_features as *const libc::c_void,
            ::core::mem::size_of::<pony_language_features_init_t>()
                .try_into()
                .unwrap(),
        );
        if language_init.init_network as libc::c_int != 0 && !ponyint_os_sockets_init() {
            running.store(NOT_RUNNING, Relaxed);
            return 0 as libc::c_int != 0;
        }
        if language_init.init_serialisation as libc::c_int != 0
            && !ponyint_serialise_setup(
                language_init.descriptor_table,
                language_init.descriptor_table_size,
            )
        {
            running.store(NOT_RUNNING, Relaxed);
            return 0 as libc::c_int != 0;
        }
    } else {
        memset(
            &mut language_init as *mut pony_language_features_init_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<pony_language_features_init_t>()
                .try_into()
                .unwrap(),
        );
    }
    if !ponyint_sched_start(library) {
        running.store(NOT_RUNNING, Relaxed);
        return 0 as libc::c_int != 0;
    }
    if library {
        running.store(RUNNING_LIBRARY, Release);
        return 1 as libc::c_int != 0;
    }
    if language_init.init_network {
        ponyint_os_sockets_final();
    }
    let mut ec: libc::c_int = pony_get_exitcode();
    f__atomic_thread_fence(b"memory_order_acq_rel\0" as *const u8 as *const libc::c_char);
    initialised.store(false, Relaxed);
    running.store(NOT_RUNNING, Relaxed);
    if !exit_code.is_null() {
        *exit_code = ec;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "375:1"]
pub unsafe extern "C" fn pony_stop() -> libc::c_int {
    if (&mut initialised).load(Relaxed) {
    } else {
        ponyint_assert_fail(
            b"atomic_load_explicit(&initialised, memory_order_relaxed)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            377 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pony_stop\0")).as_ptr(),
        );
    };
    let mut loc_running: RunningKind = running.load(Acquire);
    if loc_running as libc::c_uint == RUNNING_LIBRARY as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"loc_running == RUNNING_LIBRARY\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/start.c\0" as *const u8
                as *const libc::c_char,
            385 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pony_stop\0")).as_ptr(),
        );
    };
    ponyint_sched_stop();
    if language_init.init_network {
        ponyint_os_sockets_final();
    }
    let mut ec: libc::c_int = pony_get_exitcode();
    f__atomic_thread_fence(b"memory_order_acq_rel\0" as *const u8 as *const libc::c_char);
    initialised.store(false, Relaxed);
    running.store(NOT_RUNNING, Relaxed);
    ec
}
#[no_mangle]
#[c2rust::src_loc = "403:1"]
pub unsafe extern "C" fn pony_exitcode(mut code: libc::c_int) {
    rt_exit_code.store(code, Release);
}
#[no_mangle]
#[c2rust::src_loc = "408:1"]
pub unsafe extern "C" fn pony_get_exitcode() -> libc::c_int {
    (&mut rt_exit_code).load(Acquire)
}
