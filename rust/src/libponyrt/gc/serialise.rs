use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
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
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_hash_size(key: usize) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:5"]
pub mod pony_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
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
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
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
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::{pony_actor_t, pony_type_t};
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:5"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:9"]
    pub struct ponyint_array_t {
        pub t: *const pony_type_t,
        pub size: usize,
        pub alloc: usize,
        pub ptr: *mut libc::c_char,
    }
    #[c2rust::src_loc = "20:1"]
    pub type deserialise_raw_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    use super::stddef_h::size_t;
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
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:5"]
pub mod gc_h {
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
        #[c2rust::src_loc = "28:32"]
        pub fn ponyint_gcstack_push(
            stack: *mut gcstack_t,
            data: *mut libc::c_void,
        ) -> *mut gcstack_t;
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_gc_handlestack(ctx: *mut pony_ctx_t);
        #[c2rust::src_loc = "61:1"]
        pub fn ponyint_gc_discardstack(ctx: *mut pony_ctx_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:6"]
pub use crate::libponyrt::actor::messageq::messageq_h;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:5"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "131:7"]
        pub fn abort() -> !;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:7"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:8"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_t,
    _opaque_pthread_cond_t, _opaque_pthread_t,
};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actormap_h::actormap_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_size};
use self::gc_h::{ponyint_gc_discardstack, ponyint_gc_handlestack, ponyint_gcstack_push};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_traceknown,
    pony_traceunknown, pony_type_t, C2RustUnnamed_0, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE,
    PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{
    deserialise_raw_fn, ponyint_array_t, ponyint_serialise_t, serialise_alloc_fn,
    serialise_throw_fn,
};
pub use self::stddef_h::size_t;
use self::stdlib_h::abort;
use self::string_h::{memcpy, memset};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "26:8"]
pub struct serialise_t {
    pub key: libc::uintptr_t,
    pub value: libc::uintptr_t,
    pub t: *const pony_type_t,
    pub mutability: libc::c_int,
    pub block: bool,
}
#[c2rust::src_loc = "50:1"]
pub type ponyint_serialise_free_fn = Option<unsafe extern "C" fn(*mut serialise_t) -> ()>;
#[c2rust::src_loc = "50:1"]
pub type ponyint_serialise_cmp_fn =
    Option<unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool>;
#[c2rust::src_loc = "21:15"]
static mut desc_table_size: usize = 0 as libc::c_int as usize;
#[c2rust::src_loc = "22:22"]
static mut desc_table: *mut *const pony_type_t =
    0 as *const *const pony_type_t as *mut *const pony_type_t;
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn serialise_hash(mut p: *mut serialise_t) -> usize {
    ponyint_hash_size((*p).key)
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn serialise_cmp(mut a: *mut serialise_t, mut b: *mut serialise_t) -> bool {
    (*a).key == (*b).key
}
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn serialise_free(mut p: *mut serialise_t) {
    ponyint_pool_free(0 as libc::c_int as usize, p as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_removeindex(
    mut map: *mut ponyint_serialise_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_mem_size(mut map: *mut ponyint_serialise_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_putindex(
    mut map: *mut ponyint_serialise_t,
    mut entry: *mut serialise_t,
    mut index: usize,
) {
    let mut cmpf: ponyint_serialise_cmp_fn =
        Some(serialise_cmp as unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        serialise_hash(entry),
        ::core::mem::transmute::<ponyint_serialise_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_alloc_size(mut map: *mut ponyint_serialise_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_clearindex(
    mut map: *mut ponyint_serialise_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_optimize(mut map: *mut ponyint_serialise_t) {
    let mut cmpf: ponyint_serialise_cmp_fn =
        Some(serialise_cmp as unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_serialise_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_size(mut map: *mut ponyint_serialise_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_destroy(mut map: *mut ponyint_serialise_t) {
    let mut freef: ponyint_serialise_free_fn =
        Some(serialise_free as unsafe extern "C" fn(*mut serialise_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ponyint_serialise_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_serialise_init(
    mut map: *mut ponyint_serialise_t,
    mut size: usize,
) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "51:3"]
pub unsafe extern "C" fn ponyint_serialise_next(
    mut map: *mut ponyint_serialise_t,
    mut i: *mut usize,
) -> *mut serialise_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut serialise_t
}
#[no_mangle]
#[c2rust::src_loc = "51:3"]
pub unsafe extern "C" fn ponyint_serialise_put(
    mut map: *mut ponyint_serialise_t,
    mut entry: *mut serialise_t,
) -> *mut serialise_t {
    let mut cmpf: ponyint_serialise_cmp_fn =
        Some(serialise_cmp as unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        serialise_hash(entry),
        ::core::mem::transmute::<ponyint_serialise_cmp_fn, cmp_fn>(cmpf),
    ) as *mut serialise_t
}
#[no_mangle]
#[c2rust::src_loc = "51:3"]
pub unsafe extern "C" fn ponyint_serialise_get(
    mut map: *mut ponyint_serialise_t,
    mut key: *mut serialise_t,
    mut index: *mut usize,
) -> *mut serialise_t {
    let mut cmpf: ponyint_serialise_cmp_fn =
        Some(serialise_cmp as unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        serialise_hash(key),
        ::core::mem::transmute::<ponyint_serialise_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut serialise_t
}
#[no_mangle]
#[c2rust::src_loc = "51:3"]
pub unsafe extern "C" fn ponyint_serialise_remove(
    mut map: *mut ponyint_serialise_t,
    mut entry: *mut serialise_t,
) -> *mut serialise_t {
    let mut cmpf: ponyint_serialise_cmp_fn =
        Some(serialise_cmp as unsafe extern "C" fn(*mut serialise_t, *mut serialise_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        serialise_hash(entry),
        ::core::mem::transmute::<ponyint_serialise_cmp_fn, cmp_fn>(cmpf),
    ) as *mut serialise_t
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn recurse(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut f: *mut libc::c_void,
) {
    if !f.is_null() {
        let ref mut fresh0 = (*ctx).stack;
        *fresh0 = ponyint_gcstack_push((*ctx).stack, p);
        let ref mut fresh1 = (*ctx).stack;
        *fresh1 = ponyint_gcstack_push((*ctx).stack, f);
    }
}
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn serialise_cleanup(mut ctx: *mut pony_ctx_t) {
    ponyint_gc_discardstack(ctx);
    (*ctx).serialise_size = 0 as libc::c_int as usize;
    ponyint_serialise_destroy(&mut (*ctx).serialise);
}
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn custom_deserialise(mut ctx: *mut pony_ctx_t) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = 0 as *mut serialise_t;
    loop {
        s = ponyint_serialise_next(&mut (*ctx).serialise, &mut i);
        if s.is_null() {
            break;
        }
        if !((*s).t).is_null() && ((*(*s).t).custom_deserialise).is_some() {
            ((*(*s).t).custom_deserialise).expect("non-null function pointer")(
                (*s).value as *mut libc::c_void,
                ((*ctx).serialise_buffer as libc::uintptr_t)
                    .wrapping_add((*s).key)
                    .wrapping_add((*(*s).t).size as libc::c_ulong)
                    as *mut libc::c_void,
            );
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn ponyint_serialise_setup(
    mut table: *mut *const pony_type_t,
    mut table_size: usize,
) -> bool {
    let mut i: u32 = 0 as libc::c_int as u32;
    while (i as libc::c_ulong) < table_size {
        if !(*table.offset(i as isize)).is_null() {
            if (**table.offset(i as isize)).id == i {
            } else {
                ponyint_assert_fail(
                    b"table[i]->id == i\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.c\0"
                        as *const u8 as *const libc::c_char,
                    90 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"ponyint_serialise_setup\0",
                    ))
                    .as_ptr(),
                );
            };
        }
        i = i.wrapping_add(1);
    }
    desc_table = table;
    desc_table_size = table_size;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn ponyint_serialise_object(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut mutability: libc::c_int,
) {
    let mut k: serialise_t = serialise_t {
        key: 0,
        value: 0,
        t: 0 as *const pony_type_t,
        mutability: 0,
        block: false,
    };
    k.key = p as libc::uintptr_t;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = ponyint_serialise_get(&mut (*ctx).serialise, &mut k, &mut index);
    if !s.is_null() {
        if (*s).mutability != PONY_TRACE_OPAQUE as libc::c_int
            || mutability == PONY_TRACE_OPAQUE as libc::c_int
        {
            return;
        }
    } else {
        s = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut serialise_t;
        (*s).key = p as libc::uintptr_t;
        (*s).value = (*ctx).serialise_size;
        let ref mut fresh2 = (*s).t;
        *fresh2 = t;
        (*s).block = 0 as libc::c_int != 0;
        ponyint_serialise_putindex(&mut (*ctx).serialise, s, index);
        let ref mut fresh3 = (*ctx).serialise_size;
        *fresh3 =
            (*fresh3 as libc::c_ulong).wrapping_add((*t).size as libc::c_ulong) as usize as usize;
        if ((*t).custom_serialise_space).is_some() {
            let ref mut fresh4 = (*ctx).serialise_size;
            *fresh4 = (*fresh4 as libc::c_ulong)
                .wrapping_add(((*t).custom_serialise_space).expect("non-null function pointer")(p))
                as usize as usize;
        }
    }
    (*s).mutability = mutability;
    if mutability != PONY_TRACE_OPAQUE as libc::c_int {
        recurse(
            ctx,
            p,
            ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).serialise_trace),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn ponyint_serialise_actor(
    mut ctx: *mut pony_ctx_t,
    mut _actor: *mut pony_actor_t,
) {
    serialise_cleanup(ctx);
    ::core::mem::transmute::<_, fn()>(((*ctx).serialise_throw).expect("non-null function pointer"))(
    );
    abort();
}
#[no_mangle]
#[c2rust::src_loc = "148:1"]
pub unsafe extern "C" fn pony_serialise_reserve(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut size: usize,
) {
    let mut k: serialise_t = serialise_t {
        key: 0,
        value: 0,
        t: 0 as *const pony_type_t,
        mutability: 0,
        block: false,
    };
    k.key = p as libc::uintptr_t;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = ponyint_serialise_get(&mut (*ctx).serialise, &mut k, &mut index);
    if !s.is_null() {
        return;
    }
    s = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut serialise_t;
    (*s).key = p as libc::uintptr_t;
    (*s).value = (*ctx).serialise_size;
    let ref mut fresh5 = (*s).t;
    *fresh5 = 0 as *const pony_type_t;
    (*s).mutability = PONY_TRACE_OPAQUE as libc::c_int;
    (*s).block = 1 as libc::c_int != 0;
    ponyint_serialise_putindex(&mut (*ctx).serialise, s, index);
    let ref mut fresh6 = (*ctx).serialise_size;
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(size) as usize as usize;
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn pony_serialise_offset(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
) -> usize {
    if p.is_null() {
        return !(0 as libc::c_int) as usize;
    }
    let mut k: serialise_t = serialise_t {
        key: 0,
        value: 0,
        t: 0 as *const pony_type_t,
        mutability: 0,
        block: false,
    };
    k.key = p as libc::uintptr_t;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = ponyint_serialise_get(&mut (*ctx).serialise, &mut k, &mut index);
    if !s.is_null() {
        if (*s).block as libc::c_int != 0 || !((*s).t).is_null() && ((*(*s).t).serialise).is_some()
        {
            return (*s).value;
        } else {
            return !(0 as libc::c_int) as usize;
        }
    }
    let mut t: *const pony_type_t = *(p as *mut *const pony_type_t);
    return (*t).id as usize
        | (1 as libc::c_int as usize)
            << (::core::mem::size_of::<usize>())
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1);
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn pony_serialise(
    mut ctx: *mut pony_ctx_t,
    mut p: *mut libc::c_void,
    mut t: *const pony_type_t,
    mut out: *mut ponyint_array_t,
    mut alloc_fn: serialise_alloc_fn,
    mut throw_fn: serialise_throw_fn,
) {
    if ((*ctx).stack).is_null() {
    } else {
        ponyint_assert_fail(
            b"ctx->stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.c\0" as *const u8
                as *const libc::c_char,
            205 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pony_serialise\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh7 = (*ctx).trace_object;
    *fresh7 = Some(
        ponyint_serialise_object
            as unsafe extern "C" fn(
                *mut pony_ctx_t,
                *mut libc::c_void,
                *const pony_type_t,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh8 = (*ctx).trace_actor;
    *fresh8 = Some(
        ponyint_serialise_actor as unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t) -> (),
    );
    (*ctx).serialise_size = 0 as libc::c_int as usize;
    let ref mut fresh9 = (*ctx).serialise_alloc;
    *fresh9 = alloc_fn;
    let ref mut fresh10 = (*ctx).serialise_throw;
    *fresh10 = throw_fn;
    if !t.is_null() {
        pony_traceknown(ctx, p, t, PONY_TRACE_MUTABLE as libc::c_int);
    } else {
        pony_traceunknown(ctx, p, PONY_TRACE_MUTABLE as libc::c_int);
    }
    ponyint_gc_handlestack(ctx);
    (*out).size = (*ctx).serialise_size;
    (*out).alloc = (*out).size;
    let ref mut fresh11 = (*out).ptr;
    *fresh11 = alloc_fn.expect("non-null function pointer")(ctx, (*out).size) as *mut libc::c_char;
    memset(
        (*out).ptr as *mut libc::c_void,
        0 as libc::c_int,
        (*out).size,
    );
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = 0 as *mut serialise_t;
    loop {
        s = ponyint_serialise_next(&mut (*ctx).serialise, &mut i);
        if s.is_null() {
            break;
        }
        if !(*s).block && !((*s).t).is_null() && ((*(*s).t).serialise).is_some() {
            ((*(*s).t).serialise).expect("non-null function pointer")(
                ctx,
                (*s).key as *mut libc::c_void,
                (*out).ptr as *mut libc::c_void,
                (*s).value,
                (*s).mutability,
            );
        }
    }
    serialise_cleanup(ctx);
}
#[no_mangle]
#[c2rust::src_loc = "236:1"]
pub unsafe extern "C" fn pony_deserialise_offset(
    mut ctx: *mut pony_ctx_t,
    mut t: *const pony_type_t,
    mut offset: libc::uintptr_t,
) -> *mut libc::c_void {
    if offset == !(0 as libc::c_int) as usize {
        return 0 as *mut libc::c_void;
    }
    if offset
        & (1 as libc::c_int as usize)
            << (::core::mem::size_of::<usize>())
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1)
        != 0
    {
        offset &= !((1 as libc::c_int as usize)
            << (::core::mem::size_of::<usize>())
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1));
        if offset > desc_table_size {
            return 0 as *mut libc::c_void;
        }
        t = *desc_table.offset(offset as isize);
        return (*t).instance;
    }
    let mut k: serialise_t = serialise_t {
        key: 0,
        value: 0,
        t: 0 as *const pony_type_t,
        mutability: 0,
        block: false,
    };
    k.key = offset;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = ponyint_serialise_get(&mut (*ctx).serialise, &mut k, &mut index);
    if !s.is_null() {
        return (*s).value as *mut libc::c_void;
    }
    if t.is_null() {
        if offset.wrapping_add(::core::mem::size_of::<uintptr_t>())
            > (*ctx).serialise_size
        {
            serialise_cleanup(ctx);
            ::core::mem::transmute::<_, fn()>(
                ((*ctx).serialise_throw).expect("non-null function pointer"),
            )();
            abort();
        }
        let mut id: libc::uintptr_t =
            *(((*ctx).serialise_buffer as libc::uintptr_t).wrapping_add(offset) as *mut libc::uintptr_t);
        t = *desc_table.offset(id as isize);
    }
    if !((*t).instance).is_null() {
        return (*t).instance;
    }
    if offset.wrapping_add((*t).size as libc::c_ulong) > (*ctx).serialise_size {
        serialise_cleanup(ctx);
        ::core::mem::transmute::<_, fn()>(
            ((*ctx).serialise_throw).expect("non-null function pointer"),
        )();
        abort();
    }
    let mut object: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*t).final_0).is_none() {
        object =
            ((*ctx).serialise_alloc).expect("non-null function pointer")(ctx, (*t).size as usize);
    } else {
        object = ((*ctx).serialise_alloc_final).expect("non-null function pointer")(
            ctx,
            (*t).size as usize,
        );
    }
    memcpy(
        object,
        ((*ctx).serialise_buffer as libc::uintptr_t).wrapping_add(offset) as *mut libc::c_void,
        (*t).size as libc::c_ulong,
    );
    s = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut serialise_t;
    (*s).key = offset;
    (*s).value = object as libc::uintptr_t;
    let ref mut fresh12 = (*s).t;
    *fresh12 = t;
    ponyint_serialise_putindex(&mut (*ctx).serialise, s, index);
    recurse(
        ctx,
        object,
        ::core::mem::transmute::<pony_trace_fn, *mut libc::c_void>((*t).deserialise),
    );
    object
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn pony_deserialise_block(
    mut ctx: *mut pony_ctx_t,
    mut offset: libc::uintptr_t,
    mut size: usize,
) -> *mut libc::c_void {
    if offset == !(0 as libc::c_int) as usize {
        return 0 as *mut libc::c_void;
    }
    if offset.wrapping_add(size) > (*ctx).serialise_size {
        serialise_cleanup(ctx);
        ::core::mem::transmute::<_, fn()>(
            ((*ctx).serialise_throw).expect("non-null function pointer"),
        )();
        abort();
    }
    let mut block: *mut libc::c_void =
        ((*ctx).serialise_alloc).expect("non-null function pointer")(ctx, size);
    memcpy(
        block,
        ((*ctx).serialise_buffer as libc::uintptr_t).wrapping_add(offset) as *mut libc::c_void,
        size,
    );
    block
}
#[no_mangle]
#[c2rust::src_loc = "339:1"]
pub unsafe extern "C" fn pony_deserialise_raw(
    mut ctx: *mut pony_ctx_t,
    mut offset: libc::uintptr_t,
    mut ds_fn: deserialise_raw_fn,
) -> *mut libc::c_void {
    if offset == !(0 as libc::c_int) as usize {
        return 0 as *mut libc::c_void;
    }
    let mut k: serialise_t = serialise_t {
        key: 0,
        value: 0,
        t: 0 as *const pony_type_t,
        mutability: 0,
        block: false,
    };
    k.key = offset;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut s: *mut serialise_t = ponyint_serialise_get(&mut (*ctx).serialise, &mut k, &mut index);
    if !s.is_null() {
        return (*s).value as *mut libc::c_void;
    }
    let mut object: *mut libc::c_void = ds_fn.expect("non-null function pointer")(
        ((*ctx).serialise_buffer as libc::uintptr_t).wrapping_add(offset) as *mut libc::c_void,
        ((*ctx).serialise_size).wrapping_sub(offset),
    );
    if object.is_null() {
        serialise_cleanup(ctx);
        ::core::mem::transmute::<_, fn()>(
            ((*ctx).serialise_throw).expect("non-null function pointer"),
        )();
        abort();
    }
    s = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut serialise_t;
    (*s).key = offset;
    (*s).value = object as libc::uintptr_t;
    let ref mut fresh13 = (*s).t;
    *fresh13 = 0 as *const pony_type_t;
    ponyint_serialise_putindex(&mut (*ctx).serialise, s, index);
    object
}
#[no_mangle]
#[c2rust::src_loc = "376:1"]
pub unsafe extern "C" fn pony_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut t: *const pony_type_t,
    mut in_0: *mut ponyint_array_t,
    mut alloc_fn: serialise_alloc_fn,
    mut alloc_final_fn: serialise_alloc_fn,
    mut throw_fn: serialise_throw_fn,
) -> *mut libc::c_void {
    let ref mut fresh14 = (*ctx).serialise_buffer;
    *fresh14 = (*in_0).ptr as *mut libc::c_void;
    (*ctx).serialise_size = (*in_0).size;
    let ref mut fresh15 = (*ctx).serialise_alloc;
    *fresh15 = alloc_fn;
    let ref mut fresh16 = (*ctx).serialise_alloc_final;
    *fresh16 = alloc_final_fn;
    let ref mut fresh17 = (*ctx).serialise_throw;
    *fresh17 = throw_fn;
    let mut object: *mut libc::c_void =
        pony_deserialise_offset(ctx, t, 0 as libc::c_int as libc::uintptr_t);
    ponyint_gc_handlestack(ctx);
    custom_deserialise(ctx);
    serialise_cleanup(ctx);
    object
}
