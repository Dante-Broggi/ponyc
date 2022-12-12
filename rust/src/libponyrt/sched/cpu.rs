use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:4"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:4"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:4"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:4"]
pub mod _types_h {
    #[c2rust::src_loc = "52:1"]
    pub type __darwin_natural_t = libc::c_uint;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:4"]
pub mod sys__types_h {
    #[c2rust::src_loc = "68:1"]
    pub type __darwin_mach_port_name_t = __darwin_natural_t;
    #[c2rust::src_loc = "69:1"]
    pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
    use super::_types_h::__darwin_natural_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:4"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:4"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:4"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:4"]
pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: libc::c_long,
    }
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:4"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:4"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mach_port_t.h:4"]
pub mod _mach_port_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type mach_port_t = __darwin_mach_port_t;
    use super::sys__types_h::__darwin_mach_port_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/i386/kern_return.h:14"]
pub mod kern_return_h {
    #[c2rust::src_loc = "73:1"]
    pub type kern_return_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/i386/vm_types.h:14"]
pub mod vm_types_h {
    #[c2rust::src_loc = "95:1"]
    pub type natural_t = __darwin_natural_t;
    #[c2rust::src_loc = "96:1"]
    pub type integer_t = libc::c_int;
    use super::_types_h::__darwin_natural_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/message.h:14"]
pub mod message_h {
    #[c2rust::src_loc = "614:1"]
    pub type mach_msg_type_number_t = natural_t;
    use super::vm_types_h::natural_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/thread_policy.h:14"]
pub mod thread_policy_h {
    #[c2rust::src_loc = "51:1"]
    pub type thread_policy_flavor_t = natural_t;
    #[c2rust::src_loc = "52:1"]
    pub type thread_policy_t = *mut integer_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "210:8"]
    pub struct thread_affinity_policy {
        pub affinity_tag: integer_t,
    }
    #[c2rust::src_loc = "216:1"]
    pub type thread_affinity_policy_data_t = thread_affinity_policy;
    use super::vm_types_h::{integer_t, natural_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/mach_types.h:14"]
pub mod mach_types_h {
    #[c2rust::src_loc = "126:1"]
    pub type thread_act_t = mach_port_t;
    use super::_mach_port_t_h::mach_port_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:26"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:26"]
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
    use super::_int32_t_h::int32_t;
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_t_h::pthread_t;
    use super::_uint32_t_h::uint32_t;
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::{pony_actor_t, pony_type_t};
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:26"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:26"]
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
    use super::_uint32_t_h::uint32_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:26"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:26"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:26"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:26"]
pub mod actormap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:35"]
    pub struct actormap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:26"]
pub mod gc_h {
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:4"]
pub mod time_h {
    use super::_timespec_h::timespec;
    extern "C" {
        #[c2rust::src_loc = "142:1"]
        pub fn nanosleep(__rqtp: *const timespec, __rmtp: *mut timespec) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:5"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/thread_act.h:14"]
pub mod thread_act_h {
    use super::kern_return_h::kern_return_t;
    use super::mach_types_h::thread_act_t;
    use super::message_h::mach_msg_type_number_t;
    use super::thread_policy_h::{thread_policy_flavor_t, thread_policy_t};
    
    extern "C" {
        #[c2rust::src_loc = "320:1"]
        pub fn thread_policy_set(
            thread: thread_act_t,
            flavor: thread_policy_flavor_t,
            policy_info: thread_policy_t,
            policy_infoCnt: mach_msg_type_number_t,
        ) -> kern_return_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/mach_init.h:14"]
pub mod mach_init_h {
    use super::_mach_port_t_h::mach_port_t;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn mach_thread_self() -> mach_port_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/sysctl.h:38"]
pub mod sysctl_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "794:1"]
        pub fn sysctlbyname(
            _: *const libc::c_char,
            _: *mut libc::c_void,
            _: *mut size_t,
            _: *mut libc::c_void,
            _: size_t,
        ) -> libc::c_int;
    }
}
pub use self::_int32_t_h::int32_t;
pub use self::_mach_port_t_h::mach_port_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_timespec_h::timespec;
pub use self::_types_h::{__darwin_natural_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actormap_h::actormap_t;
use self::dtrace_h::macro__DTRACE;

pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::internal::__int128_t;
pub use self::kern_return_h::kern_return_t;
use self::mach_init_h::mach_thread_self;
pub use self::mach_types_h::thread_act_t;
pub use self::message_h::mach_msg_type_number_t;
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_type_t,
};
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
pub use self::stddef_h::size_t;
pub use self::sys__types_h::{__darwin_mach_port_name_t, __darwin_mach_port_t};
use self::sysctl_h::sysctlbyname;
use self::thread_act_h::thread_policy_set;
pub use self::thread_policy_h::{
    thread_affinity_policy, thread_affinity_policy_data_t, thread_policy_flavor_t, thread_policy_t,
};
use self::time_h::nanosleep;
pub use self::vm_types_h::{integer_t, natural_t};
use core::arch::asm;
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn property(mut key: *const libc::c_char) -> uint32_t {
    let mut value: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
    let mut err: libc::c_int = sysctlbyname(
        key,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        &mut len,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    return (if err == 0 as libc::c_int {
        value
    } else {
        0 as libc::c_int
    }) as uint32_t;
}
#[c2rust::src_loc = "60:17"]
static mut hw_cpu_count: uint32_t = 0;
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn ponyint_cpu_init() {
    hw_cpu_count = property(b"hw.physicalcpu\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn ponyint_cpu_count() -> uint32_t {
    hw_cpu_count
}
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn ponyint_cpu_assign(
    mut count: uint32_t,
    mut scheduler: *mut scheduler_t,
    mut pin: bool,
    mut pinasio: bool,
) -> uint32_t {
    let mut asio_cpu: uint32_t = -(1 as libc::c_int) as uint32_t;
    if !pin {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < count {
            (*scheduler.offset(i as isize)).cpu = -(1 as libc::c_int) as uint32_t;
            (*scheduler.offset(i as isize)).node = 0 as libc::c_int as uint32_t;
            i = i.wrapping_add(1);
        }
        return asio_cpu;
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < count {
        (*scheduler.offset(i_0 as isize)).cpu = i_0;
        (*scheduler.offset(i_0 as isize)).node = 0 as libc::c_int as uint32_t;
        i_0 = i_0.wrapping_add(1);
    }
    if pinasio {
        asio_cpu = count;
    }
    asio_cpu
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn ponyint_cpu_affinity(mut cpu: uint32_t) {
    if cpu == -(1 as libc::c_int) as uint32_t {
        return;
    }
    let mut policy: thread_affinity_policy_data_t =
        thread_affinity_policy_data_t { affinity_tag: 0 };
    policy.affinity_tag = cpu as integer_t;
    thread_policy_set(
        mach_thread_self(),
        4 as libc::c_int as thread_policy_flavor_t,
        &mut policy as *mut thread_affinity_policy_data_t as thread_policy_t,
        (::core::mem::size_of::<thread_affinity_policy_data_t>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<integer_t>() as libc::c_ulong)
            as mach_msg_type_number_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "325:1"]
pub unsafe extern "C" fn ponyint_cpu_core_pause(
    mut tsc: uint64_t,
    mut tsc2: uint64_t,
    mut yield_0: bool,
) {
    let mut diff: uint64_t = ponyint_cpu_tick_diff(tsc, tsc2);
    if diff < 10000000 as libc::c_int as libc::c_ulonglong {
        return;
    }
    if yield_0 {
        let mut ts: timespec = {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __darwin_time_t,
                tv_nsec: 0 as libc::c_int as libc::c_long,
            };
            init
        };
        if diff > 10000000000 as libc::c_long as libc::c_ulonglong {
            ts.tv_nsec = 30000000 as libc::c_int as libc::c_long;
        } else if diff > 3000000000 as libc::c_long as libc::c_ulonglong {
            ts.tv_nsec = 10000000 as libc::c_int as libc::c_long;
        } else if diff > 1000000000 as libc::c_int as libc::c_ulonglong {
            ts.tv_nsec = 1000000 as libc::c_int as libc::c_long;
        } else {
            ts.tv_nsec = 100000 as libc::c_int as libc::c_long;
        }
        macro__DTRACE(
            b"CPU_NANOSLEEP\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            b"ts.tv_nsec\0" as *const u8 as *const libc::c_char,
        );
        nanosleep(&mut ts, 0 as *mut timespec);
    }
}
#[no_mangle]
#[c2rust::src_loc = "387:1"]
pub unsafe extern "C" fn ponyint_cpu_relax() {
    asm!("pause", options(preserves_flags));
}
#[no_mangle]
#[c2rust::src_loc = "394:1"]
pub unsafe extern "C" fn ponyint_cpu_tick() -> uint64_t {
    return 0 as libc::c_int as uint64_t;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn ponyint_cpu_tick_diff(
    mut supposedly_earlier: uint64_t,
    mut supposedly_later: uint64_t,
) -> uint64_t {
    if supposedly_earlier == supposedly_later {
        return 0 as libc::c_int as uint64_t;
    } else if supposedly_earlier > supposedly_later {
        return (18446744073709551615 as libc::c_ulonglong)
            .wrapping_sub(supposedly_earlier)
            .wrapping_add(supposedly_later);
    } else {
        supposedly_later.wrapping_sub(supposedly_earlier)
    }
}
