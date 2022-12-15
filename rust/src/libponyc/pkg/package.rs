use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "43:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "46:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "55:1"]
    pub type __darwin_blkcnt_t = i64;
    #[c2rust::src_loc = "56:1"]
    pub type __darwin_blksize_t = i32;
    #[c2rust::src_loc = "57:1"]
    pub type __darwin_dev_t = i32;
    #[c2rust::src_loc = "60:1"]
    pub type __darwin_gid_t = u32;
    #[c2rust::src_loc = "62:1"]
    pub type __darwin_ino64_t = u64;
    #[c2rust::src_loc = "70:1"]
    pub type __darwin_mode_t = u16;
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
    #[c2rust::src_loc = "75:1"]
    pub type __darwin_uid_t = u32;
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
    #[c2rust::src_loc = "78:8"]
    pub struct _opaque_pthread_mutex_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 56],
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
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_pthread_t = *mut _opaque_pthread_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:1"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/dirent.h:1"]
pub mod sys_dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:8"]
    pub struct dirent {
        pub d_ino: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [libc::c_char; 1024],
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/dirent.h:1"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:9"]
    pub struct DIR {
        pub __dd_fd: libc::c_int,
        pub __dd_loc: libc::c_long,
        pub __dd_size: libc::c_long,
        pub __dd_buf: *mut libc::c_char,
        pub __dd_len: libc::c_int,
        pub __dd_seek: libc::c_long,
        pub __padding: libc::c_long,
        pub __dd_flags: libc::c_int,
        pub __dd_lock: __darwin_pthread_mutex_t,
        pub __dd_td: *mut _telldir,
    }
    use super::_pthread_types_h::__darwin_pthread_mutex_t;
    extern "C" {
        #[c2rust::src_loc = "70:8"]
        pub type _telldir;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_dev_t.h:1"]
pub mod _dev_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:1"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h:1"]
pub mod _off_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blkcnt_t.h:1"]
pub mod _blkcnt_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blksize_t.h:1"]
pub mod _blksize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:1"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_nlink_t.h:1"]
pub mod _nlink_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type nlink_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:1"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: u32,
        pub st_gen: u32,
        pub st_lspare: i32,
        pub st_qspare: [i64; 2],
    }
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;

    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {
        #[c2rust::src_loc = "386:1"]
        pub fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "15:1"]
    pub type map_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_hash_str(str: *const libc::c_char) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
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
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::scheduler_h::pony_ctx_t;
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:12"]
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
    use super::_size_t_h::size_t;
    use super::actormap_h::actormap_t;
    use super::gc_h::gcstack_t;
    use super::messageq_h::messageq_t;
    use super::mpmcq_h::mpmcq_t;
    use super::mutemap_h::mutemap_t;
    use super::pony_h::{pony_actor_t, pony_type_t};
    use super::serialise_h::{ponyint_serialise_t, serialise_alloc_fn, serialise_throw_fn};
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:10"]
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
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::hash_h::hashmap_t;
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn pony_serialise_reserve(ctx: *mut pony_ctx_t, p: *mut libc::c_void, size: usize);
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: libc::uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
        #[c2rust::src_loc = "33:1"]
        pub fn pony_serialise(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            out: *mut ponyint_array_t,
            alloc_fn: serialise_alloc_fn,
            throw_fn: serialise_throw_fn,
        );
        #[c2rust::src_loc = "42:1"]
        pub fn pony_deserialise_block(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
            size: usize,
        ) -> *mut libc::c_void;
    }
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
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "127:1"]
        pub fn ponyint_hashmap_serialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            buf: *mut libc::c_void,
            offset: usize,
        );
        #[c2rust::src_loc = "124:1"]
        pub fn ponyint_hashmap_serialise_trace(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
        #[c2rust::src_loc = "130:1"]
        pub fn ponyint_hashmap_deserialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: usize);
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: usize,
        );
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: *mut usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut usize,
            count: usize,
            item_bitmap: *mut bitmap_t,
            size: usize,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/actormap.h:10"]
pub mod actormap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:35"]
    pub struct actormap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/gc.h:10"]
pub mod gc_h {
    extern "C" {
        #[c2rust::src_loc = "28:32"]
        pub type gcstack_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:12"]
pub use crate::libponyrt::actor::messageq::messageq_h;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:12"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mutemap.h:12"]
pub mod mutemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:34"]
    pub struct mutemap_t {
        pub contents: hashmap_t,
    }
    use super::hash_h::hashmap_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/list.h:1"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct list_t {
        pub data: *mut libc::c_void,
        pub next: *mut list_t,
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::fun_h::{cmp_fn, free_fn, map_fn};
    use super::pony_h::pony_type_t;
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_list_pop(list: *mut list_t, data: *mut *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_list_push(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_list_append(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_list_next(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_list_index(list: *mut list_t, index: ssize_t) -> *mut list_t;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_list_data(list: *mut list_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_list_find(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "32:1"]
        pub fn ponyint_list_findindex(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> ssize_t;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_list_subset(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_list_equals(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_list_map(
            list: *mut list_t,
            f: map_fn,
            arg: *mut libc::c_void,
        ) -> *mut list_t;
        #[c2rust::src_loc = "40:1"]
        pub fn ponyint_list_reverse(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_list_length(list: *mut list_t) -> usize;
        #[c2rust::src_loc = "44:1"]
        pub fn ponyint_list_free(list: *mut list_t, f: free_fn);
        #[c2rust::src_loc = "46:1"]
        pub fn ponyint_list_serialise_trace(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            list_type: *const pony_type_t,
            elem_type: *const pony_type_t,
        );
        #[c2rust::src_loc = "49:1"]
        pub fn ponyint_list_serialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            buf: *mut libc::c_void,
            offset: usize,
        );
        #[c2rust::src_loc = "52:1"]
        pub fn ponyint_list_deserialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            list_type: *const pony_type_t,
            elem_type: *const pony_type_t,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:1"]
pub mod _stdio_h {
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:16"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:1"]
pub mod package_h {
    #[c2rust::src_loc = "20:1"]
    pub type package_group_list_cmp_fn =
        Option<unsafe extern "C" fn(*mut package_group_t, *mut package_group_t) -> bool>;
    #[c2rust::src_loc = "20:1"]
    pub type package_group_list_map_fn = Option<
        unsafe extern "C" fn(*mut package_group_t, *mut libc::c_void) -> *mut package_group_t,
    >;
    #[c2rust::src_loc = "20:1"]
    pub type package_group_list_free_fn = Option<unsafe extern "C" fn(*mut package_group_t) -> ()>;
    #[c2rust::src_loc = "24:1"]
    pub type path_fn = Option<unsafe extern "C" fn(*const libc::c_char, *mut pass_opt_t) -> bool>;
    use super::package_group_t;
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "17:16"]
        pub type ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.h:2"]
pub mod pass_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "274:16"]
    pub struct pass_opt_t {
        pub limit: pass_id,
        pub program_pass: pass_id,
        pub release: bool,
        pub library: bool,
        pub runtimebc: bool,
        pub staticbin: bool,
        pub pic: bool,
        pub print_stats: bool,
        pub verify: bool,
        pub extfun: bool,
        pub strip_debug: bool,
        pub print_filenames: bool,
        pub check_tree: bool,
        pub lint_llvm: bool,
        pub docs: bool,
        pub docs_private: bool,
        pub verbosity: verbosity_level,
        pub ast_print_width: usize,
        pub allow_test_symbols: bool,
        pub parse_trace: bool,
        pub package_search_paths: *mut strlist_t,
        pub safe_packages: *mut strlist_t,
        pub magic_packages: *mut magic_package_t,
        pub argv0: *const libc::c_char,
        pub all_args: *const libc::c_char,
        pub output: *const libc::c_char,
        pub bin_name: *const libc::c_char,
        pub link_arch: *mut libc::c_char,
        pub linker: *mut libc::c_char,
        pub link_ldcmd: *mut libc::c_char,
        pub llvm_args: *const libc::c_char,
        pub triple: *mut libc::c_char,
        pub abi: *mut libc::c_char,
        pub cpu: *mut libc::c_char,
        pub features: *mut libc::c_char,
        pub check: typecheck_t,
        pub plugins: *mut plugins_t,
        pub data: *mut libc::c_void,
    }
    #[c2rust::src_loc = "205:9"]
    pub type verbosity_level = libc::c_uint;
    #[c2rust::src_loc = "211:3"]
    pub const VERBOSITY_ALL: verbosity_level = 4;
    #[c2rust::src_loc = "210:3"]
    pub const VERBOSITY_TOOL_INFO: verbosity_level = 3;
    #[c2rust::src_loc = "209:3"]
    pub const VERBOSITY_INFO: verbosity_level = 2;
    #[c2rust::src_loc = "208:3"]
    pub const VERBOSITY_MINIMAL: verbosity_level = 1;
    #[c2rust::src_loc = "207:3"]
    pub const VERBOSITY_QUIET: verbosity_level = 0;
    #[c2rust::src_loc = "216:9"]
    pub type pass_id = libc::c_uint;
    #[c2rust::src_loc = "239:3"]
    pub const PASS_ALL: pass_id = 21;
    #[c2rust::src_loc = "238:3"]
    pub const PASS_OBJ: pass_id = 20;
    #[c2rust::src_loc = "237:3"]
    pub const PASS_ASM: pass_id = 19;
    #[c2rust::src_loc = "236:3"]
    pub const PASS_BITCODE: pass_id = 18;
    #[c2rust::src_loc = "235:3"]
    pub const PASS_LLVM_IR: pass_id = 17;
    #[c2rust::src_loc = "234:3"]
    pub const PASS_PAINT: pass_id = 16;
    #[c2rust::src_loc = "233:3"]
    pub const PASS_REACH: pass_id = 15;
    #[c2rust::src_loc = "232:3"]
    pub const PASS_SERIALISER: pass_id = 14;
    #[c2rust::src_loc = "231:3"]
    pub const PASS_FINALISER: pass_id = 13;
    #[c2rust::src_loc = "230:3"]
    pub const PASS_VERIFY: pass_id = 12;
    #[c2rust::src_loc = "229:3"]
    pub const PASS_COMPLETENESS: pass_id = 11;
    #[c2rust::src_loc = "228:3"]
    pub const PASS_EXPR: pass_id = 10;
    #[c2rust::src_loc = "227:3"]
    pub const PASS_REFER: pass_id = 9;
    #[c2rust::src_loc = "226:3"]
    pub const PASS_DOCS: pass_id = 8;
    #[c2rust::src_loc = "225:3"]
    pub const PASS_TRAITS: pass_id = 7;
    #[c2rust::src_loc = "224:3"]
    pub const PASS_FLATTEN: pass_id = 6;
    #[c2rust::src_loc = "223:3"]
    pub const PASS_NAME_RESOLUTION: pass_id = 5;
    #[c2rust::src_loc = "222:3"]
    pub const PASS_IMPORT: pass_id = 4;
    #[c2rust::src_loc = "221:3"]
    pub const PASS_SCOPE: pass_id = 3;
    #[c2rust::src_loc = "220:3"]
    pub const PASS_SUGAR: pass_id = 2;
    #[c2rust::src_loc = "219:3"]
    pub const PASS_SYNTAX: pass_id = 1;
    #[c2rust::src_loc = "218:3"]
    pub const PASS_PARSE: pass_id = 0;
    use super::_size_t_h::size_t;
    use super::frame_h::typecheck_t;
    use super::magic_package_t;
    use super::package_h::ast_t;
    use super::source_h::source_t;
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "363:1"]
        pub fn ast_passes_program(program: *mut ast_t, options: *mut pass_opt_t) -> bool;
        #[c2rust::src_loc = "390:1"]
        pub fn ast_passes_subtree(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
        #[c2rust::src_loc = "358:1"]
        pub fn module_passes(
            package: *mut ast_t,
            options: *mut pass_opt_t,
            source: *mut source_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/frame.h:2"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct typecheck_t {
        pub frame: *mut typecheck_frame_t,
        pub stats: typecheck_stats_t,
        pub errors: *mut errors_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct typecheck_stats_t {
        pub names_count: usize,
        pub default_caps_count: usize,
        pub heap_alloc: usize,
        pub stack_alloc: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct typecheck_frame_t {
        pub package: *mut ast_t,
        pub module: *mut ast_t,
        pub type_0: *mut ast_t,
        pub constraint: *mut ast_t,
        pub provides: *mut ast_t,
        pub method: *mut ast_t,
        pub def_arg: *mut ast_t,
        pub method_body: *mut ast_t,
        pub method_params: *mut ast_t,
        pub method_type: *mut ast_t,
        pub ffi_type: *mut ast_t,
        pub local_type: *mut ast_t,
        pub as_type: *mut ast_t,
        pub the_case: *mut ast_t,
        pub pattern: *mut ast_t,
        pub loop_0: *mut ast_t,
        pub loop_cond: *mut ast_t,
        pub loop_body: *mut ast_t,
        pub loop_else: *mut ast_t,
        pub try_expr: *mut ast_t,
        pub recover: *mut ast_t,
        pub ifdef_cond: *mut ast_t,
        pub ifdef_clause: *mut ast_t,
        pub iftype_constraint: *mut ast_t,
        pub iftype_body: *mut ast_t,
        pub prev: *mut typecheck_frame_t,
    }
    use super::_size_t_h::size_t;
    use super::error_h::errors_t;
    use super::package_h::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:2"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "102:1"]
        pub fn errorf(
            errors: *mut errors_t,
            file: *const libc::c_char,
            fmt: *const libc::c_char,
            _: ...
        );
        #[c2rust::src_loc = "110:1"]
        pub fn errorf_continue(
            errors: *mut errors_t,
            file: *const libc::c_char,
            fmt: *const libc::c_char,
            _: ...
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:2"]
pub mod stringtab_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::scheduler_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "27:1"]
        pub fn string_deserialise_offset(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
        ) -> *const libc::c_char;
        #[c2rust::src_loc = "24:1"]
        pub fn string_trace(ctx: *mut pony_ctx_t, string: *const libc::c_char);
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_push(list: *mut strlist_t, data: *const libc::c_char) -> *mut strlist_t;
        #[c2rust::src_loc = "9:1"]
        pub fn strlist_free(list: *mut strlist_t);
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "9:34"]
        pub fn strlist_data(list: *mut strlist_t) -> *const libc::c_char;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_next(list: *mut strlist_t) -> *mut strlist_t;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_pop(list: *mut strlist_t, data: *mut *const libc::c_char) -> *mut strlist_t;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_append(list: *mut strlist_t, data: *const libc::c_char) -> *mut strlist_t;
        #[c2rust::src_loc = "9:34"]
        pub fn strlist_find(list: *mut strlist_t, data: *const libc::c_char)
            -> *const libc::c_char;
        #[c2rust::src_loc = "20:1"]
        pub fn stringtab_consume(
            string: *const libc::c_char,
            buf_size: usize,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.h:2"]
pub mod token_h {
    #[c2rust::src_loc = "185:3"]
    pub const TK_PACKAGE: token_id = 137;
    #[c2rust::src_loc = "20:9"]
    pub type token_id = libc::c_uint;
    #[c2rust::src_loc = "275:3"]
    pub const TK_TEST_EXTRA: token_id = 214;
    #[c2rust::src_loc = "274:3"]
    pub const TK_TEST_UPDATEARG: token_id = 213;
    #[c2rust::src_loc = "273:3"]
    pub const TK_TEST_ALIASED: token_id = 212;
    #[c2rust::src_loc = "272:3"]
    pub const TK_TEST_TRY_NO_CHECK: token_id = 211;
    #[c2rust::src_loc = "271:3"]
    pub const TK_TEST_SEQ_SCOPE: token_id = 210;
    #[c2rust::src_loc = "270:3"]
    pub const TK_TEST_NO_SEQ: token_id = 209;
    #[c2rust::src_loc = "267:3"]
    pub const TK_FLATTEN: token_id = 208;
    #[c2rust::src_loc = "266:3"]
    pub const TK_NEWLINE: token_id = 207;
    #[c2rust::src_loc = "263:3"]
    pub const TK_DISPOSING_BLOCK: token_id = 206;
    #[c2rust::src_loc = "261:3"]
    pub const TK_ANNOTATION: token_id = 205;
    #[c2rust::src_loc = "259:3"]
    pub const TK_FUNCHAIN: token_id = 204;
    #[c2rust::src_loc = "258:3"]
    pub const TK_BECHAIN: token_id = 203;
    #[c2rust::src_loc = "257:3"]
    pub const TK_FUNAPP: token_id = 202;
    #[c2rust::src_loc = "256:3"]
    pub const TK_BEAPP: token_id = 201;
    #[c2rust::src_loc = "255:3"]
    pub const TK_NEWAPP: token_id = 200;
    #[c2rust::src_loc = "254:3"]
    pub const TK_DONTCAREREF: token_id = 199;
    #[c2rust::src_loc = "253:3"]
    pub const TK_PARAMREF: token_id = 198;
    #[c2rust::src_loc = "252:3"]
    pub const TK_LETREF: token_id = 197;
    #[c2rust::src_loc = "251:3"]
    pub const TK_VARREF: token_id = 196;
    #[c2rust::src_loc = "250:3"]
    pub const TK_TUPLEELEMREF: token_id = 195;
    #[c2rust::src_loc = "249:3"]
    pub const TK_EMBEDREF: token_id = 194;
    #[c2rust::src_loc = "248:3"]
    pub const TK_FLETREF: token_id = 193;
    #[c2rust::src_loc = "247:3"]
    pub const TK_FVARREF: token_id = 192;
    #[c2rust::src_loc = "246:3"]
    pub const TK_FUNREF: token_id = 191;
    #[c2rust::src_loc = "245:3"]
    pub const TK_BEREF: token_id = 190;
    #[c2rust::src_loc = "244:3"]
    pub const TK_NEWBEREF: token_id = 189;
    #[c2rust::src_loc = "243:3"]
    pub const TK_NEWREF: token_id = 188;
    #[c2rust::src_loc = "242:3"]
    pub const TK_TYPEPARAMREF: token_id = 187;
    #[c2rust::src_loc = "241:3"]
    pub const TK_TYPEREF: token_id = 186;
    #[c2rust::src_loc = "240:3"]
    pub const TK_PACKAGEREF: token_id = 185;
    #[c2rust::src_loc = "239:3"]
    pub const TK_REFERENCE: token_id = 184;
    #[c2rust::src_loc = "237:3"]
    pub const TK_MATCH_DONTCARE: token_id = 183;
    #[c2rust::src_loc = "236:3"]
    pub const TK_MATCH_CAPTURE: token_id = 182;
    #[c2rust::src_loc = "235:3"]
    pub const TK_CASE: token_id = 181;
    #[c2rust::src_loc = "234:3"]
    pub const TK_CASES: token_id = 180;
    #[c2rust::src_loc = "233:3"]
    pub const TK_ARRAY: token_id = 179;
    #[c2rust::src_loc = "232:3"]
    pub const TK_TUPLE: token_id = 178;
    #[c2rust::src_loc = "231:3"]
    pub const TK_CALL: token_id = 177;
    #[c2rust::src_loc = "230:3"]
    pub const TK_QUALIFY: token_id = 176;
    #[c2rust::src_loc = "229:3"]
    pub const TK_SEQ: token_id = 175;
    #[c2rust::src_loc = "227:3"]
    pub const TK_LAMBDACAPTURE: token_id = 174;
    #[c2rust::src_loc = "226:3"]
    pub const TK_LAMBDACAPTURES: token_id = 173;
    #[c2rust::src_loc = "225:3"]
    pub const TK_UPDATEARG: token_id = 172;
    #[c2rust::src_loc = "224:3"]
    pub const TK_NAMEDARG: token_id = 171;
    #[c2rust::src_loc = "223:3"]
    pub const TK_NAMEDARGS: token_id = 170;
    #[c2rust::src_loc = "222:3"]
    pub const TK_POSITIONALARGS: token_id = 169;
    #[c2rust::src_loc = "221:3"]
    pub const TK_VALUEFORMALARG: token_id = 168;
    #[c2rust::src_loc = "220:3"]
    pub const TK_VALUEFORMALPARAM: token_id = 167;
    #[c2rust::src_loc = "219:3"]
    pub const TK_TYPEARGS: token_id = 166;
    #[c2rust::src_loc = "218:3"]
    pub const TK_PARAM: token_id = 165;
    #[c2rust::src_loc = "217:3"]
    pub const TK_PARAMS: token_id = 164;
    #[c2rust::src_loc = "216:3"]
    pub const TK_TYPEPARAM: token_id = 163;
    #[c2rust::src_loc = "215:3"]
    pub const TK_TYPEPARAMS: token_id = 162;
    #[c2rust::src_loc = "213:3"]
    pub const TK_OPERATORLITERAL: token_id = 161;
    #[c2rust::src_loc = "212:3"]
    pub const TK_LITERALBRANCH: token_id = 160;
    #[c2rust::src_loc = "211:3"]
    pub const TK_LITERAL: token_id = 159;
    #[c2rust::src_loc = "209:3"]
    pub const TK_ERRORTYPE: token_id = 158;
    #[c2rust::src_loc = "208:3"]
    pub const TK_INFERTYPE: token_id = 157;
    #[c2rust::src_loc = "207:3"]
    pub const TK_DONTCARETYPE: token_id = 156;
    #[c2rust::src_loc = "206:3"]
    pub const TK_BARELAMBDATYPE: token_id = 155;
    #[c2rust::src_loc = "205:3"]
    pub const TK_LAMBDATYPE: token_id = 154;
    #[c2rust::src_loc = "204:3"]
    pub const TK_FUNTYPE: token_id = 153;
    #[c2rust::src_loc = "203:3"]
    pub const TK_THISTYPE: token_id = 152;
    #[c2rust::src_loc = "202:3"]
    pub const TK_NOMINAL: token_id = 151;
    #[c2rust::src_loc = "201:3"]
    pub const TK_TUPLETYPE: token_id = 150;
    #[c2rust::src_loc = "200:3"]
    pub const TK_UNIONTYPE: token_id = 149;
    #[c2rust::src_loc = "199:3"]
    pub const TK_PROVIDES: token_id = 148;
    #[c2rust::src_loc = "197:3"]
    pub const TK_IFDEFFLAG: token_id = 147;
    #[c2rust::src_loc = "196:3"]
    pub const TK_IFDEFNOT: token_id = 146;
    #[c2rust::src_loc = "195:3"]
    pub const TK_IFDEFOR: token_id = 145;
    #[c2rust::src_loc = "194:3"]
    pub const TK_IFDEFAND: token_id = 144;
    #[c2rust::src_loc = "192:3"]
    pub const TK_FFICALL: token_id = 143;
    #[c2rust::src_loc = "191:3"]
    pub const TK_FFIDECL: token_id = 142;
    #[c2rust::src_loc = "190:3"]
    pub const TK_FLET: token_id = 141;
    #[c2rust::src_loc = "189:3"]
    pub const TK_FVAR: token_id = 140;
    #[c2rust::src_loc = "188:3"]
    pub const TK_MEMBERS: token_id = 139;
    #[c2rust::src_loc = "186:3"]
    pub const TK_MODULE: token_id = 138;
    #[c2rust::src_loc = "184:3"]
    pub const TK_PROGRAM: token_id = 136;
    #[c2rust::src_loc = "181:3"]
    pub const TK_LOCATION: token_id = 135;
    #[c2rust::src_loc = "180:3"]
    pub const TK_ADDRESS: token_id = 134;
    #[c2rust::src_loc = "179:3"]
    pub const TK_DIGESTOF: token_id = 133;
    #[c2rust::src_loc = "177:3"]
    pub const TK_XOR: token_id = 132;
    #[c2rust::src_loc = "176:3"]
    pub const TK_OR: token_id = 131;
    #[c2rust::src_loc = "175:3"]
    pub const TK_AND: token_id = 130;
    #[c2rust::src_loc = "174:3"]
    pub const TK_NOT: token_id = 129;
    #[c2rust::src_loc = "172:3"]
    pub const TK_COMPILE_ERROR: token_id = 128;
    #[c2rust::src_loc = "171:3"]
    pub const TK_ERROR: token_id = 127;
    #[c2rust::src_loc = "170:3"]
    pub const TK_WITH: token_id = 126;
    #[c2rust::src_loc = "169:3"]
    pub const TK_TRY_NO_CHECK: token_id = 125;
    #[c2rust::src_loc = "168:3"]
    pub const TK_TRY: token_id = 124;
    #[c2rust::src_loc = "167:3"]
    pub const TK_WHERE: token_id = 123;
    #[c2rust::src_loc = "166:3"]
    pub const TK_MATCH: token_id = 122;
    #[c2rust::src_loc = "165:3"]
    pub const TK_IN: token_id = 121;
    #[c2rust::src_loc = "164:3"]
    pub const TK_FOR: token_id = 120;
    #[c2rust::src_loc = "163:3"]
    pub const TK_UNTIL: token_id = 119;
    #[c2rust::src_loc = "162:3"]
    pub const TK_REPEAT: token_id = 118;
    #[c2rust::src_loc = "161:3"]
    pub const TK_DO: token_id = 117;
    #[c2rust::src_loc = "160:3"]
    pub const TK_WHILE: token_id = 116;
    #[c2rust::src_loc = "159:3"]
    pub const TK_END: token_id = 115;
    #[c2rust::src_loc = "158:3"]
    pub const TK_ELSEIF: token_id = 114;
    #[c2rust::src_loc = "157:3"]
    pub const TK_ELSE: token_id = 113;
    #[c2rust::src_loc = "156:3"]
    pub const TK_THEN: token_id = 112;
    #[c2rust::src_loc = "155:3"]
    pub const TK_IFTYPE_SET: token_id = 111;
    #[c2rust::src_loc = "154:3"]
    pub const TK_IFTYPE: token_id = 110;
    #[c2rust::src_loc = "153:3"]
    pub const TK_IFDEF: token_id = 109;
    #[c2rust::src_loc = "152:3"]
    pub const TK_IF: token_id = 108;
    #[c2rust::src_loc = "150:3"]
    pub const TK_RECOVER: token_id = 107;
    #[c2rust::src_loc = "149:3"]
    pub const TK_CONSUME: token_id = 106;
    #[c2rust::src_loc = "148:3"]
    pub const TK_CONTINUE: token_id = 105;
    #[c2rust::src_loc = "147:3"]
    pub const TK_BREAK: token_id = 104;
    #[c2rust::src_loc = "146:3"]
    pub const TK_RETURN: token_id = 103;
    #[c2rust::src_loc = "145:3"]
    pub const TK_THIS: token_id = 102;
    #[c2rust::src_loc = "143:3"]
    pub const TK_CAP_ANY: token_id = 101;
    #[c2rust::src_loc = "142:3"]
    pub const TK_CAP_ALIAS: token_id = 100;
    #[c2rust::src_loc = "141:3"]
    pub const TK_CAP_SHARE: token_id = 99;
    #[c2rust::src_loc = "140:3"]
    pub const TK_CAP_SEND: token_id = 98;
    #[c2rust::src_loc = "139:3"]
    pub const TK_CAP_READ: token_id = 97;
    #[c2rust::src_loc = "137:3"]
    pub const TK_TAG: token_id = 96;
    #[c2rust::src_loc = "136:3"]
    pub const TK_BOX: token_id = 95;
    #[c2rust::src_loc = "135:3"]
    pub const TK_VAL: token_id = 94;
    #[c2rust::src_loc = "134:3"]
    pub const TK_REF: token_id = 93;
    #[c2rust::src_loc = "133:3"]
    pub const TK_TRN: token_id = 92;
    #[c2rust::src_loc = "132:3"]
    pub const TK_ISO: token_id = 91;
    #[c2rust::src_loc = "130:3"]
    pub const TK_BE: token_id = 90;
    #[c2rust::src_loc = "129:3"]
    pub const TK_FUN: token_id = 89;
    #[c2rust::src_loc = "128:3"]
    pub const TK_NEW: token_id = 88;
    #[c2rust::src_loc = "127:3"]
    pub const TK_DONTCARE: token_id = 87;
    #[c2rust::src_loc = "126:3"]
    pub const TK_EMBED: token_id = 86;
    #[c2rust::src_loc = "125:3"]
    pub const TK_LET: token_id = 85;
    #[c2rust::src_loc = "124:3"]
    pub const TK_VAR: token_id = 84;
    #[c2rust::src_loc = "122:3"]
    pub const TK_ISNT: token_id = 83;
    #[c2rust::src_loc = "121:3"]
    pub const TK_IS: token_id = 82;
    #[c2rust::src_loc = "120:3"]
    pub const TK_AS: token_id = 81;
    #[c2rust::src_loc = "118:3"]
    pub const TK_BARELAMBDA: token_id = 80;
    #[c2rust::src_loc = "117:3"]
    pub const TK_LAMBDA: token_id = 79;
    #[c2rust::src_loc = "116:3"]
    pub const TK_OBJECT: token_id = 78;
    #[c2rust::src_loc = "115:3"]
    pub const TK_ACTOR: token_id = 77;
    #[c2rust::src_loc = "114:3"]
    pub const TK_CLASS: token_id = 76;
    #[c2rust::src_loc = "113:3"]
    pub const TK_STRUCT: token_id = 75;
    #[c2rust::src_loc = "112:3"]
    pub const TK_PRIMITIVE: token_id = 74;
    #[c2rust::src_loc = "111:3"]
    pub const TK_TRAIT: token_id = 73;
    #[c2rust::src_loc = "110:3"]
    pub const TK_INTERFACE: token_id = 72;
    #[c2rust::src_loc = "109:3"]
    pub const TK_TYPE: token_id = 71;
    #[c2rust::src_loc = "108:3"]
    pub const TK_USE: token_id = 70;
    #[c2rust::src_loc = "106:3"]
    pub const TK_COMPILE_INTRINSIC: token_id = 69;
    #[c2rust::src_loc = "103:3"]
    pub const TK_MINUS_TILDE_NEW: token_id = 68;
    #[c2rust::src_loc = "102:3"]
    pub const TK_MINUS_NEW: token_id = 67;
    #[c2rust::src_loc = "101:3"]
    pub const TK_LSQUARE_NEW: token_id = 66;
    #[c2rust::src_loc = "100:3"]
    pub const TK_LPAREN_NEW: token_id = 65;
    #[c2rust::src_loc = "97:3"]
    pub const TK_CONSTANT: token_id = 64;
    #[c2rust::src_loc = "96:3"]
    pub const TK_ELLIPSIS: token_id = 63;
    #[c2rust::src_loc = "95:3"]
    pub const TK_UNARY_MINUS_TILDE: token_id = 62;
    #[c2rust::src_loc = "94:3"]
    pub const TK_UNARY_MINUS: token_id = 61;
    #[c2rust::src_loc = "93:3"]
    pub const TK_QUESTION: token_id = 60;
    #[c2rust::src_loc = "91:3"]
    pub const TK_SUBTYPE: token_id = 59;
    #[c2rust::src_loc = "90:3"]
    pub const TK_ALIASED: token_id = 58;
    #[c2rust::src_loc = "89:3"]
    pub const TK_EPHEMERAL: token_id = 57;
    #[c2rust::src_loc = "88:3"]
    pub const TK_ISECTTYPE: token_id = 56;
    #[c2rust::src_loc = "87:3"]
    pub const TK_PIPE: token_id = 55;
    #[c2rust::src_loc = "85:3"]
    pub const TK_NE_TILDE: token_id = 54;
    #[c2rust::src_loc = "84:3"]
    pub const TK_NE: token_id = 53;
    #[c2rust::src_loc = "83:3"]
    pub const TK_EQ_TILDE: token_id = 52;
    #[c2rust::src_loc = "82:3"]
    pub const TK_EQ: token_id = 51;
    #[c2rust::src_loc = "80:3"]
    pub const TK_GT_TILDE: token_id = 50;
    #[c2rust::src_loc = "79:3"]
    pub const TK_GT: token_id = 49;
    #[c2rust::src_loc = "78:3"]
    pub const TK_GE_TILDE: token_id = 48;
    #[c2rust::src_loc = "77:3"]
    pub const TK_GE: token_id = 47;
    #[c2rust::src_loc = "76:3"]
    pub const TK_LE_TILDE: token_id = 46;
    #[c2rust::src_loc = "75:3"]
    pub const TK_LE: token_id = 45;
    #[c2rust::src_loc = "74:3"]
    pub const TK_LT_TILDE: token_id = 44;
    #[c2rust::src_loc = "73:3"]
    pub const TK_LT: token_id = 43;
    #[c2rust::src_loc = "71:3"]
    pub const TK_RSHIFT_TILDE: token_id = 42;
    #[c2rust::src_loc = "70:3"]
    pub const TK_RSHIFT: token_id = 41;
    #[c2rust::src_loc = "69:3"]
    pub const TK_LSHIFT_TILDE: token_id = 40;
    #[c2rust::src_loc = "68:3"]
    pub const TK_LSHIFT: token_id = 39;
    #[c2rust::src_loc = "66:3"]
    pub const TK_AT_LBRACE: token_id = 38;
    #[c2rust::src_loc = "65:3"]
    pub const TK_AT: token_id = 37;
    #[c2rust::src_loc = "64:3"]
    pub const TK_MOD_TILDE: token_id = 36;
    #[c2rust::src_loc = "63:3"]
    pub const TK_MOD: token_id = 35;
    #[c2rust::src_loc = "62:3"]
    pub const TK_REM_TILDE: token_id = 34;
    #[c2rust::src_loc = "61:3"]
    pub const TK_REM: token_id = 33;
    #[c2rust::src_loc = "60:3"]
    pub const TK_DIVIDE_TILDE: token_id = 32;
    #[c2rust::src_loc = "59:3"]
    pub const TK_DIVIDE: token_id = 31;
    #[c2rust::src_loc = "58:3"]
    pub const TK_MULTIPLY_TILDE: token_id = 30;
    #[c2rust::src_loc = "57:3"]
    pub const TK_MULTIPLY: token_id = 29;
    #[c2rust::src_loc = "56:3"]
    pub const TK_MINUS_TILDE: token_id = 28;
    #[c2rust::src_loc = "55:3"]
    pub const TK_MINUS: token_id = 27;
    #[c2rust::src_loc = "54:3"]
    pub const TK_PLUS_TILDE: token_id = 26;
    #[c2rust::src_loc = "53:3"]
    pub const TK_PLUS: token_id = 25;
    #[c2rust::src_loc = "51:3"]
    pub const TK_ASSIGN: token_id = 24;
    #[c2rust::src_loc = "50:3"]
    pub const TK_SEMI: token_id = 23;
    #[c2rust::src_loc = "49:3"]
    pub const TK_COLON: token_id = 22;
    #[c2rust::src_loc = "48:3"]
    pub const TK_CHAIN: token_id = 21;
    #[c2rust::src_loc = "47:3"]
    pub const TK_TILDE: token_id = 20;
    #[c2rust::src_loc = "46:3"]
    pub const TK_DOT: token_id = 19;
    #[c2rust::src_loc = "45:3"]
    pub const TK_DBLARROW: token_id = 18;
    #[c2rust::src_loc = "44:3"]
    pub const TK_ARROW: token_id = 17;
    #[c2rust::src_loc = "43:3"]
    pub const TK_COMMA: token_id = 16;
    #[c2rust::src_loc = "41:3"]
    pub const TK_BACKSLASH: token_id = 15;
    #[c2rust::src_loc = "40:3"]
    pub const TK_RSQUARE: token_id = 14;
    #[c2rust::src_loc = "39:3"]
    pub const TK_LSQUARE: token_id = 13;
    #[c2rust::src_loc = "38:3"]
    pub const TK_RPAREN: token_id = 12;
    #[c2rust::src_loc = "37:3"]
    pub const TK_LPAREN: token_id = 11;
    #[c2rust::src_loc = "36:3"]
    pub const TK_RBRACE: token_id = 10;
    #[c2rust::src_loc = "35:3"]
    pub const TK_LBRACE: token_id = 9;
    #[c2rust::src_loc = "32:3"]
    pub const TK_ID: token_id = 8;
    #[c2rust::src_loc = "31:3"]
    pub const TK_FLOAT: token_id = 7;
    #[c2rust::src_loc = "30:3"]
    pub const TK_INT: token_id = 6;
    #[c2rust::src_loc = "29:3"]
    pub const TK_STRING: token_id = 5;
    #[c2rust::src_loc = "28:3"]
    pub const TK_FALSE: token_id = 4;
    #[c2rust::src_loc = "27:3"]
    pub const TK_TRUE: token_id = 3;
    #[c2rust::src_loc = "24:3"]
    pub const TK_NONE: token_id = 2;
    #[c2rust::src_loc = "23:3"]
    pub const TK_LEX_ERROR: token_id = 1;
    #[c2rust::src_loc = "22:3"]
    pub const TK_EOF: token_id = 0;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:2"]
pub mod symtab_h {
    #[c2rust::src_loc = "12:9"]
    pub type sym_status_t = libc::c_uint;
    #[c2rust::src_loc = "21:3"]
    pub const SYM_ERROR: sym_status_t = 7;
    #[c2rust::src_loc = "20:3"]
    pub const SYM_FFIDECL: sym_status_t = 6;
    #[c2rust::src_loc = "19:3"]
    pub const SYM_CONSUMED_SAME_EXPR: sym_status_t = 5;
    #[c2rust::src_loc = "18:3"]
    pub const SYM_CONSUMED: sym_status_t = 4;
    #[c2rust::src_loc = "17:3"]
    pub const SYM_UNDEFINED: sym_status_t = 3;
    #[c2rust::src_loc = "16:3"]
    pub const SYM_DEFINED: sym_status_t = 2;
    #[c2rust::src_loc = "15:3"]
    pub const SYM_NOCASE: sym_status_t = 1;
    #[c2rust::src_loc = "14:3"]
    pub const SYM_NONE: sym_status_t = 0;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:2"]
pub mod ast_h {
    #[c2rust::src_loc = "39:3"]
    pub const AST_FLAG_PRESERVE: C2RustUnnamed_1 = 8192;
    #[c2rust::src_loc = "29:1"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "51:3"]
    pub const AST_FLAG_FCNSM_REASGN: C2RustUnnamed_1 = 33554432;
    #[c2rust::src_loc = "50:3"]
    pub const AST_FLAG_CNSM_REASGN: C2RustUnnamed_1 = 16777216;
    #[c2rust::src_loc = "49:3"]
    pub const AST_FLAG_MAY_BREAK: C2RustUnnamed_1 = 8388608;
    #[c2rust::src_loc = "48:3"]
    pub const AST_FLAG_IMPORT: C2RustUnnamed_1 = 4194304;
    #[c2rust::src_loc = "47:3"]
    pub const AST_FLAG_INCOMPLETE: C2RustUnnamed_1 = 2097152;
    #[c2rust::src_loc = "46:3"]
    pub const AST_FLAG_JUMPS_AWAY: C2RustUnnamed_1 = 1048576;
    #[c2rust::src_loc = "45:3"]
    pub const AST_FLAG_ERROR_2: C2RustUnnamed_1 = 524288;
    #[c2rust::src_loc = "44:3"]
    pub const AST_FLAG_DONE_2: C2RustUnnamed_1 = 262144;
    #[c2rust::src_loc = "43:3"]
    pub const AST_FLAG_RECURSE_2: C2RustUnnamed_1 = 131072;
    #[c2rust::src_loc = "42:3"]
    pub const AST_FLAG_ERROR_1: C2RustUnnamed_1 = 65536;
    #[c2rust::src_loc = "41:3"]
    pub const AST_FLAG_DONE_1: C2RustUnnamed_1 = 32768;
    #[c2rust::src_loc = "40:3"]
    pub const AST_FLAG_RECURSE_1: C2RustUnnamed_1 = 16384;
    #[c2rust::src_loc = "38:3"]
    pub const AST_FLAG_MISSING_SEMI: C2RustUnnamed_1 = 4096;
    #[c2rust::src_loc = "37:3"]
    pub const AST_FLAG_BAD_SEMI: C2RustUnnamed_1 = 2048;
    #[c2rust::src_loc = "36:3"]
    pub const AST_FLAG_AMBIGUOUS: C2RustUnnamed_1 = 1024;
    #[c2rust::src_loc = "35:3"]
    pub const AST_FLAG_IN_PARENS: C2RustUnnamed_1 = 512;
    #[c2rust::src_loc = "34:3"]
    pub const AST_FLAG_MIGHT_SEND: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "33:3"]
    pub const AST_FLAG_CAN_SEND: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "32:3"]
    pub const AST_FLAG_CAN_ERROR: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "31:3"]
    pub const AST_FLAG_PASS_MASK: C2RustUnnamed_1 = 31;
    use super::error_h::errors_t;
    use super::package_h::ast_t;
    use super::pony_h::pony_type_t;
    use super::symtab_h::sym_status_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "223:1"]
        pub fn ast_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "57:1"]
        pub fn ast_blank(id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: u32);
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "122:1"]
        pub fn ast_set(
            ast: *mut ast_t,
            name: *const libc::c_char,
            value: *mut ast_t,
            status: sym_status_t,
            allow_shadowing: bool,
        ) -> bool;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
        #[c2rust::src_loc = "79:1"]
        pub fn ast_setdata(ast: *mut ast_t, data: *mut libc::c_void) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "219:1"]
        pub fn ast_signature_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:2"]
pub mod source_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn source_open(
            file: *const libc::c_char,
            error_msgp: *mut *const libc::c_char,
        ) -> *mut source_t;
        #[c2rust::src_loc = "30:1"]
        pub fn source_open_string(source_code: *const libc::c_char) -> *mut source_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:4"]
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
        #[c2rust::src_loc = "19:1"]
        pub fn ponyint_stack_push(list: *mut Stack, data: *mut libc::c_void) -> *mut Stack;
        #[c2rust::src_loc = "17:1"]
        pub fn ponyint_stack_pop(stack: *mut Stack, data: *mut *mut libc::c_void) -> *mut Stack;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/paths.h:1"]
pub mod paths_h {
    use super::dirent_h::DIR;
    use super::sys_dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn pony_opendir(path: *const libc::c_char, err: *mut u32) -> *mut DIR;
        #[c2rust::src_loc = "34:1"]
        pub fn pony_realpath(
            path: *const libc::c_char,
            resolved: *mut libc::c_char,
        ) -> *mut libc::c_char;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_dir_info_name(info: *mut dirent) -> *mut libc::c_char;
        #[c2rust::src_loc = "38:1"]
        pub fn pony_closedir(dir: *mut DIR);
        #[c2rust::src_loc = "40:1"]
        pub fn pony_dir_entry_next(dir: *mut DIR) -> *mut dirent;
        #[c2rust::src_loc = "51:1"]
        pub fn get_compiler_exe_directory(
            output_path: *mut libc::c_char,
            argv0: *const libc::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "147:1"]
        pub fn getenv(_: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "160:1"]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nel: usize,
            __width: usize,
            __compar: Option<
                unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
            >,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "68:14"]
        pub static mut __stdoutp: *mut FILE;
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "157:1"]
        pub fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "177:1"]
        pub fn putchar(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "178:1"]
        pub fn puts(_: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "344:6"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.h:2"]
pub mod program_h {
    use super::package_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn program_assign_pkg_id(ast: *mut ast_t) -> u32;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:10"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_pool_realloc_size(
            old_size: usize,
            new_size: usize,
            p: *mut libc::c_void,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:13"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/blake2/blake2.h:14"]
pub mod blake2_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "82:3"]
        pub fn blake2b(
            out: *mut libc::c_void,
            outlen: usize,
            in_0: *const libc::c_void,
            inlen: usize,
            key: *const libc::c_void,
            keylen: usize,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:18"]
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
        #[c2rust::src_loc = "75:7"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "76:7"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "79:7"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "84:6"]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;
        #[c2rust::src_loc = "87:7"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_off_t_h::off_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_cond_t, __darwin_pthread_handler_rec, __darwin_pthread_mutex_t,
    __darwin_pthread_t, _opaque_pthread_cond_t, _opaque_pthread_mutex_t, _opaque_pthread_t,
};
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __int64_t, __uint16_t,
    __uint32_t, __uint64_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::actormap_h::actormap_t;
pub use self::ast_h::{
    ast_append, ast_blank, ast_child, ast_data, ast_error, ast_free, ast_from_string, ast_get,
    ast_id, ast_name, ast_nearest, ast_parent, ast_pony_type, ast_pop, ast_scope, ast_set,
    ast_setdata, ast_setflag, ast_sibling, ast_signature_pony_type, C2RustUnnamed_1,
    AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
use self::blake2_h::blake2b;
pub use self::dirent_h::{_telldir, DIR};
use self::error_h::{errorf, errorf_continue, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, map_fn, ponyint_hash_str};
use self::gc_h::gcstack_t;
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_deserialise, ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio,
    ponyint_hashmap_get, ponyint_hashmap_init, ponyint_hashmap_mem_size, ponyint_hashmap_next,
    ponyint_hashmap_optimize, ponyint_hashmap_put, ponyint_hashmap_putindex,
    ponyint_hashmap_remove, ponyint_hashmap_removeindex, ponyint_hashmap_serialise,
    ponyint_hashmap_serialise_trace, ponyint_hashmap_size,
};
pub use self::list_h::{
    list_t, ponyint_list_append, ponyint_list_data, ponyint_list_deserialise, ponyint_list_equals,
    ponyint_list_find, ponyint_list_findindex, ponyint_list_free, ponyint_list_index,
    ponyint_list_length, ponyint_list_map, ponyint_list_next, ponyint_list_pop, ponyint_list_push,
    ponyint_list_reverse, ponyint_list_serialise, ponyint_list_serialise_trace,
    ponyint_list_subset,
};
pub use self::messageq_h::messageq_t;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_node_t, mpmcq_t, C2RustUnnamed};
pub use self::mutemap_h::mutemap_t;
pub use self::package_h::{
    ast_t, package_group_list_cmp_fn, package_group_list_free_fn, package_group_list_map_fn,
    path_fn,
};
pub use self::pass_h::{
    ast_passes_program, ast_passes_subtree, module_passes, pass_id, pass_opt_t, plugins_t,
    verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR,
    PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ,
    PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR,
    PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL,
    VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::paths_h::{
    get_compiler_exe_directory, pony_closedir, pony_dir_entry_next, pony_dir_info_name,
    pony_opendir, pony_realpath,
};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_custom_deserialise_fn, pony_custom_serialise_space_fn,
    pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn, pony_trace_fn, pony_traceknown,
    pony_type_t, C2RustUnnamed_0, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
    ponyint_pool_realloc_size,
};
use self::program_h::program_assign_pkg_id;
pub use self::scheduler_h::{pony_ctx_t, scheduler_t, trace_actor_fn, trace_object_fn};
pub use self::serialise_h::{
    pony_deserialise_block, pony_deserialise_offset, pony_serialise, pony_serialise_offset,
    pony_serialise_reserve, ponyint_array_t, ponyint_serialise_t, serialise_alloc_fn,
    serialise_throw_fn,
};
pub use self::source_h::{source_open, source_open_string, source_t};
pub use self::stack_h::{ponyint_stack_pop, ponyint_stack_push, Stack};
pub use self::stat_h::stat;
use self::stdio_h::{__stderrp, __stdoutp, fprintf, fputs, printf, putchar, puts, snprintf};
use self::stdlib_h::{getenv, qsort};
use self::string_h::{memcpy, memset, strcat, strchr, strcmp, strcpy, strlen, strncmp, strrchr};
use self::stringtab_h::{
    string_deserialise_offset, string_trace, stringtab, stringtab_consume, strlist_append,
    strlist_data, strlist_find, strlist_free, strlist_next, strlist_pop, strlist_push, strlist_t,
};
pub use self::symtab_h::{
    sym_status_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL,
    SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t,
};
pub use self::sys_dirent_h::dirent;
pub use self::token_h::{
    token_id, TK_ACTOR, TK_ADDRESS, TK_ALIASED, TK_AND, TK_ANNOTATION, TK_ARRAY, TK_ARROW, TK_AS,
    TK_ASSIGN, TK_AT, TK_AT_LBRACE, TK_BACKSLASH, TK_BARELAMBDA, TK_BARELAMBDATYPE, TK_BE,
    TK_BEAPP, TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK, TK_CALL, TK_CAP_ALIAS, TK_CAP_ANY,
    TK_CAP_READ, TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES, TK_CHAIN, TK_CLASS, TK_COLON,
    TK_COMMA, TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT, TK_CONSUME, TK_CONTINUE,
    TK_DBLARROW, TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE, TK_DIVIDE_TILDE, TK_DO, TK_DONTCARE,
    TK_DONTCAREREF, TK_DONTCARETYPE, TK_DOT, TK_ELLIPSIS, TK_ELSE, TK_ELSEIF, TK_EMBED,
    TK_EMBEDREF, TK_END, TK_EOF, TK_EPHEMERAL, TK_EQ, TK_EQ_TILDE, TK_ERROR, TK_ERRORTYPE,
    TK_FALSE, TK_FFICALL, TK_FFIDECL, TK_FLATTEN, TK_FLET, TK_FLETREF, TK_FLOAT, TK_FOR, TK_FUN,
    TK_FUNAPP, TK_FUNCHAIN, TK_FUNREF, TK_FUNTYPE, TK_FVAR, TK_FVARREF, TK_GE, TK_GE_TILDE, TK_GT,
    TK_GT_TILDE, TK_ID, TK_IF, TK_IFDEF, TK_IFDEFAND, TK_IFDEFFLAG, TK_IFDEFNOT, TK_IFDEFOR,
    TK_IFTYPE, TK_IFTYPE_SET, TK_IN, TK_INFERTYPE, TK_INT, TK_INTERFACE, TK_IS, TK_ISECTTYPE,
    TK_ISNT, TK_ISO, TK_LAMBDA, TK_LAMBDACAPTURE, TK_LAMBDACAPTURES, TK_LAMBDATYPE, TK_LBRACE,
    TK_LE, TK_LET, TK_LETREF, TK_LEX_ERROR, TK_LE_TILDE, TK_LITERAL, TK_LITERALBRANCH, TK_LOCATION,
    TK_LPAREN, TK_LPAREN_NEW, TK_LSHIFT, TK_LSHIFT_TILDE, TK_LSQUARE, TK_LSQUARE_NEW, TK_LT,
    TK_LT_TILDE, TK_MATCH, TK_MATCH_CAPTURE, TK_MATCH_DONTCARE, TK_MEMBERS, TK_MINUS, TK_MINUS_NEW,
    TK_MINUS_TILDE, TK_MINUS_TILDE_NEW, TK_MOD, TK_MODULE, TK_MOD_TILDE, TK_MULTIPLY,
    TK_MULTIPLY_TILDE, TK_NAMEDARG, TK_NAMEDARGS, TK_NE, TK_NEW, TK_NEWAPP, TK_NEWBEREF,
    TK_NEWLINE, TK_NEWREF, TK_NE_TILDE, TK_NOMINAL, TK_NONE, TK_NOT, TK_OBJECT, TK_OPERATORLITERAL,
    TK_OR, TK_PACKAGE, TK_PACKAGEREF, TK_PARAM, TK_PARAMREF, TK_PARAMS, TK_PIPE, TK_PLUS,
    TK_PLUS_TILDE, TK_POSITIONALARGS, TK_PRIMITIVE, TK_PROGRAM, TK_PROVIDES, TK_QUALIFY,
    TK_QUESTION, TK_RBRACE, TK_RECOVER, TK_REF, TK_REFERENCE, TK_REM, TK_REM_TILDE, TK_REPEAT,
    TK_RETURN, TK_RPAREN, TK_RSHIFT, TK_RSHIFT_TILDE, TK_RSQUARE, TK_SEMI, TK_SEQ, TK_STRING,
    TK_STRUCT, TK_SUBTYPE, TK_TAG, TK_TEST_ALIASED, TK_TEST_EXTRA, TK_TEST_NO_SEQ,
    TK_TEST_SEQ_SCOPE, TK_TEST_TRY_NO_CHECK, TK_TEST_UPDATEARG, TK_THEN, TK_THIS, TK_THISTYPE,
    TK_TILDE, TK_TRAIT, TK_TRN, TK_TRUE, TK_TRY, TK_TRY_NO_CHECK, TK_TUPLE, TK_TUPLEELEMREF,
    TK_TUPLETYPE, TK_TYPE, TK_TYPEARGS, TK_TYPEPARAM, TK_TYPEPARAMREF, TK_TYPEPARAMS, TK_TYPEREF,
    TK_UNARY_MINUS, TK_UNARY_MINUS_TILDE, TK_UNIONTYPE, TK_UNTIL, TK_UPDATEARG, TK_USE, TK_VAL,
    TK_VALUEFORMALARG, TK_VALUEFORMALPARAM, TK_VAR, TK_VARREF, TK_WHERE, TK_WHILE, TK_WITH, TK_XOR,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:8"]
pub struct package_t {
    pub path: *const libc::c_char,
    pub qualified_name: *const libc::c_char,
    pub id: *const libc::c_char,
    pub filename: *const libc::c_char,
    pub symbol: *const libc::c_char,
    pub ast: *mut ast_t,
    pub dependencies: package_set_t,
    pub group: *mut package_group_t,
    pub group_index: usize,
    pub next_hygienic_id: usize,
    pub low_index: usize,
    pub allow_ffi: bool,
    pub on_stack: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "74:8"]
pub struct package_group_t {
    pub signature: *mut libc::c_char,
    pub members: package_set_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:40"]
pub struct package_set_t {
    pub contents: hashmap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "81:8"]
pub struct magic_package_t {
    pub path: *const libc::c_char,
    pub src: *const libc::c_char,
    pub mapped_path: *const libc::c_char,
    pub next: *mut magic_package_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "92:43"]
pub struct package_group_list_t {
    pub contents: list_t,
}
#[c2rust::src_loc = "111:1"]
pub type package_set_free_fn = Option<unsafe extern "C" fn(*mut package_t) -> ()>;
#[c2rust::src_loc = "111:1"]
pub type package_set_cmp_fn = Option<unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "66:16"]
pub struct package_signature_t {
    pub filename: *const libc::c_char,
    pub group: *mut package_group_t,
    pub group_index: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "90:29"]
pub struct package_stack_t {}
#[no_mangle]
#[c2rust::src_loc = "90:29"]
pub unsafe extern "C" fn package_stack_push(
    mut stack: *mut package_stack_t,
    mut data: *mut package_t,
) -> *mut package_stack_t {
    ponyint_stack_push(stack as *mut Stack, data as *mut libc::c_void) as *mut package_stack_t
}
#[no_mangle]
#[c2rust::src_loc = "90:29"]
pub unsafe extern "C" fn package_stack_pop(
    mut stack: *mut package_stack_t,
    mut data: *mut *mut package_t,
) -> *mut package_stack_t {
    ponyint_stack_pop(stack as *mut Stack, data as *mut *mut libc::c_void) as *mut package_stack_t
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_equals(
    mut a: *mut package_group_list_t,
    mut b: *mut package_group_list_t,
) -> bool {
    let mut cmp: package_group_list_cmp_fn = None;
    ponyint_list_equals(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<package_group_list_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_pony_type() -> *const pony_type_t {
    &package_group_list_pony
}
#[c2rust::src_loc = "92:1"]
static mut package_group_list_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_group_list_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_group_list_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_group_list_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                package_group_list_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_list_deserialise(
        ctx,
        object,
        package_group_list_pony_type(),
        package_group_pony_type(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_list_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_list_serialise_trace(
        ctx,
        object,
        package_group_list_pony_type(),
        package_group_pony_type(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_free(mut list: *mut package_group_list_t) {
    let mut free: package_group_list_free_fn =
        Some(package_group_free as unsafe extern "C" fn(*mut package_group_t) -> ());
    ponyint_list_free(
        list as *mut list_t,
        ::core::mem::transmute::<package_group_list_free_fn, free_fn>(free),
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_length(mut list: *mut package_group_list_t) -> usize {
    ponyint_list_length(list as *mut list_t)
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_findindex(
    mut list: *mut package_group_list_t,
    mut data: *mut package_group_t,
) -> ssize_t {
    let mut cmp: package_group_list_cmp_fn = None;
    ponyint_list_findindex(
        list as *mut list_t,
        ::core::mem::transmute::<package_group_list_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    )
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn package_group_list_subset(
    mut a: *mut package_group_list_t,
    mut b: *mut package_group_list_t,
) -> bool {
    let mut cmp: package_group_list_cmp_fn = None;
    ponyint_list_subset(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<package_group_list_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_index(
    mut list: *mut package_group_list_t,
    mut index: ssize_t,
) -> *mut package_group_list_t {
    ponyint_list_index(list as *mut list_t, index) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_map(
    mut list: *mut package_group_list_t,
    mut f: package_group_list_map_fn,
    mut arg: *mut libc::c_void,
) -> *mut package_group_list_t {
    ponyint_list_map(
        list as *mut list_t,
        ::core::mem::transmute::<package_group_list_map_fn, map_fn>(f),
        arg,
    ) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_reverse(
    mut list: *mut package_group_list_t,
) -> *mut package_group_list_t {
    ponyint_list_reverse(list as *mut list_t) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_next(
    mut list: *mut package_group_list_t,
) -> *mut package_group_list_t {
    ponyint_list_next(list as *mut list_t) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_append(
    mut list: *mut package_group_list_t,
    mut data: *mut package_group_t,
) -> *mut package_group_list_t {
    ponyint_list_append(list as *mut list_t, data as *mut libc::c_void) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_push(
    mut list: *mut package_group_list_t,
    mut data: *mut package_group_t,
) -> *mut package_group_list_t {
    ponyint_list_push(list as *mut list_t, data as *mut libc::c_void) as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:43"]
pub unsafe extern "C" fn package_group_list_pop(
    mut list: *mut package_group_list_t,
    mut data: *mut *mut package_group_t,
) -> *mut package_group_list_t {
    ponyint_list_pop(list as *mut list_t, data as *mut *mut libc::c_void)
        as *mut package_group_list_t
}
#[no_mangle]
#[c2rust::src_loc = "92:65"]
pub unsafe extern "C" fn package_group_list_find(
    mut list: *mut package_group_list_t,
    mut data: *mut package_group_t,
) -> *mut package_group_t {
    let mut cmp: package_group_list_cmp_fn = None;
    ponyint_list_find(
        list as *mut list_t,
        ::core::mem::transmute::<package_group_list_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    ) as *mut package_group_t
}
#[no_mangle]
#[c2rust::src_loc = "92:65"]
pub unsafe extern "C" fn package_group_list_data(
    mut list: *mut package_group_list_t,
) -> *mut package_group_t {
    ponyint_list_data(list as *mut list_t) as *mut package_group_t
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn package_hash(mut pkg: *mut package_t) -> usize {
    ponyint_hash_str((*pkg).qualified_name)
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn package_cmp(mut a: *mut package_t, mut b: *mut package_t) -> bool {
    (*a).qualified_name == (*b).qualified_name
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_alloc_size(mut map: *mut package_set_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_destroy(mut map: *mut package_set_t) {
    let mut freef: package_set_free_fn = None;
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<package_set_free_fn, free_fn>(freef),
    );
}
#[c2rust::src_loc = "111:1"]
static mut package_set_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_set_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_set_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_set_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                package_set_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_pony_type() -> *const pony_type_t {
    &package_set_pony
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, package_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_mem_size(mut map: *mut package_set_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, package_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_putindex(
    mut map: *mut package_set_t,
    mut entry: *mut package_t,
    mut index: usize,
) {
    let mut cmpf: package_set_cmp_fn =
        Some(package_cmp as unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        package_hash(entry),
        ::core::mem::transmute::<package_set_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_size(mut map: *mut package_set_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_init(mut map: *mut package_set_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_clearindex(mut map: *mut package_set_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_optimize(mut map: *mut package_set_t) {
    let mut cmpf: package_set_cmp_fn =
        Some(package_cmp as unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<package_set_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn package_set_removeindex(mut map: *mut package_set_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "111:54"]
pub unsafe extern "C" fn package_set_remove(
    mut map: *mut package_set_t,
    mut entry: *mut package_t,
) -> *mut package_t {
    let mut cmpf: package_set_cmp_fn =
        Some(package_cmp as unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        package_hash(entry),
        ::core::mem::transmute::<package_set_cmp_fn, cmp_fn>(cmpf),
    ) as *mut package_t
}
#[no_mangle]
#[c2rust::src_loc = "111:54"]
pub unsafe extern "C" fn package_set_get(
    mut map: *mut package_set_t,
    mut key: *mut package_t,
    mut index: *mut usize,
) -> *mut package_t {
    let mut cmpf: package_set_cmp_fn =
        Some(package_cmp as unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        package_hash(key),
        ::core::mem::transmute::<package_set_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut package_t
}
#[no_mangle]
#[c2rust::src_loc = "111:54"]
pub unsafe extern "C" fn package_set_next(
    mut map: *mut package_set_t,
    mut i: *mut usize,
) -> *mut package_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets) as *mut package_t
}
#[no_mangle]
#[c2rust::src_loc = "111:54"]
pub unsafe extern "C" fn package_set_put(
    mut map: *mut package_set_t,
    mut entry: *mut package_t,
) -> *mut package_t {
    let mut cmpf: package_set_cmp_fn =
        Some(package_cmp as unsafe extern "C" fn(*mut package_t, *mut package_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        package_hash(entry),
        ::core::mem::transmute::<package_set_cmp_fn, cmp_fn>(cmpf),
    ) as *mut package_t
}
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn find_magic_package(
    mut path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> *mut magic_package_t {
    let mut p: *mut magic_package_t = (*opt).magic_packages;
    while !p.is_null() {
        if path == (*p).path {
            return p;
        }
        p = (*p).next;
    }
    return 0 as *mut magic_package_t;
}
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn parse_source_file(
    mut package: *mut ast_t,
    mut file_path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_file\0"))
                .as_ptr(),
        );
    };
    if !file_path.is_null() {
    } else {
        ponyint_assert_fail(
            b"file_path != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            134 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_file\0"))
                .as_ptr(),
        );
    };
    if !opt.is_null() {
    } else {
        ponyint_assert_fail(
            b"opt != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            135 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_file\0"))
                .as_ptr(),
        );
    };
    if (*opt).print_filenames {
        printf(
            b"Opening %s\n\0" as *const u8 as *const libc::c_char,
            file_path,
        );
    }
    let mut error_msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut source: *mut source_t = source_open(file_path, &mut error_msg);
    if source.is_null() {
        if error_msg.is_null() {
            error_msg = b"couldn't open file\0" as *const u8 as *const libc::c_char;
        }
        errorf(
            (*opt).check.errors,
            file_path,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            error_msg,
            file_path,
        );
        return 0 as libc::c_int != 0;
    }
    module_passes(package, opt, source)
}
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn parse_source_code(
    mut package: *mut ast_t,
    mut src: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !src.is_null() {
    } else {
        ponyint_assert_fail(
            b"src != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            161 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_code\0"))
                .as_ptr(),
        );
    };
    if !opt.is_null() {
    } else {
        ponyint_assert_fail(
            b"opt != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            162 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_code\0"))
                .as_ptr(),
        );
    };
    if (*opt).print_filenames {
        printf(b"Opening magic source\n\0" as *const u8 as *const libc::c_char);
    }
    let mut source: *mut source_t = source_open_string(src);
    if !source.is_null() {
    } else {
        ponyint_assert_fail(
            b"source != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            168 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"parse_source_code\0"))
                .as_ptr(),
        );
    };
    module_passes(package, opt, source)
}
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn path_cat(
    mut part1: *const libc::c_char,
    mut part2: *const libc::c_char,
    mut result: *mut libc::c_char,
) {
    let mut len1: usize = 0;
    let mut lensep: usize = 0;
    if !part1.is_null() {
        len1 = libc::strlen(part1);
        lensep = 1 as libc::c_int as usize;
    }
    let mut len2: usize = libc::strlen(part2);
    if len1.wrapping_add(lensep).wrapping_add(len2) >= (1024 as libc::c_int as libc::c_ulong).try_into().unwrap() {
        *result.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return;
    }
    if !part1.is_null() {
        memcpy(
            result as *mut libc::c_void,
            part1 as *const libc::c_void,
            len1.try_into().unwrap(),
        );
        *result.offset(len1 as isize) = '/' as i32 as libc::c_char;
        memcpy(
            &mut *result.offset(len1.wrapping_add(1) as isize) as *mut libc::c_char
                as *mut libc::c_void,
            part2 as *const libc::c_void,
            len2.wrapping_add(1).try_into().unwrap(),
        );
    } else {
        memcpy(
            result as *mut libc::c_void,
            part2 as *const libc::c_void,
            len2.wrapping_add(1).try_into().unwrap(),
        );
    };
}
#[c2rust::src_loc = "204:1"]
unsafe extern "C" fn string_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    strcmp(
        *(a as *mut *const libc::c_char),
        *(b as *mut *const libc::c_char),
    )
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn parse_files_in_dir(
    mut package: *mut ast_t,
    mut dir_path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut err: u32 = 0 as libc::c_int as u32;
    let mut dir: *mut DIR = pony_opendir(dir_path, &mut err);
    let mut errors: *mut errors_t = (*opt).check.errors;
    if dir.is_null() {
        match err {
            13 => {
                errorf(
                    errors,
                    dir_path,
                    b"permission denied\0" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                errorf(
                    errors,
                    dir_path,
                    b"does not exist\0" as *const u8 as *const libc::c_char,
                );
            }
            20 => {
                errorf(
                    errors,
                    dir_path,
                    b"not a directory\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                errorf(
                    errors,
                    dir_path,
                    b"unknown error\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return 0 as libc::c_int != 0;
    }
    let mut count: usize = 0;
    let mut buf_size: usize = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong).try_into().unwrap();
    let mut entries: *mut *const libc::c_char =
        ponyint_pool_alloc_size(buf_size) as *mut *const libc::c_char;
    let mut d: *mut dirent = 0 as *mut dirent;
    loop {
        d = pony_dir_entry_next(dir);
        if d.is_null() {
            break;
        }
        let mut name: *const libc::c_char = stringtab(pony_dir_info_name(d));
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            continue;
        }
        let mut p: *const libc::c_char = strrchr(name, '.' as i32);
        if !p.is_null()
            && strcmp(p, b".pony\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            if count.wrapping_mul((::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong).try_into().unwrap())
                == buf_size
            {
                let mut new_buf_size: usize =
                    buf_size.wrapping_mul((2 as libc::c_int as libc::c_ulong).try_into().unwrap());
                entries =
                    ponyint_pool_realloc_size(buf_size, new_buf_size, entries as *mut libc::c_void)
                        as *mut *const libc::c_char;
                buf_size = new_buf_size;
            }
            let fresh0 = count;
            count = count.wrapping_add(1);
            let ref mut fresh1 = *entries.offset(fresh0 as isize);
            *fresh1 = name;
        }
    }
    pony_closedir(dir);
    qsort(
        entries as *mut libc::c_void,
        count,
        (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong).try_into().unwrap(),
        Some(
            string_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    let mut r: bool = 1 as libc::c_int != 0;
    let mut i: usize = 0;
    while i < count {
        let mut fullpath: [libc::c_char; 1024] = [0; 1024];
        path_cat(dir_path, *entries.offset(i as isize), fullpath.as_mut_ptr());
        r = (r as libc::c_int
            & parse_source_file(package, fullpath.as_mut_ptr(), opt) as libc::c_int)
            != 0;
        i = i.wrapping_add(1);
    }
    ponyint_pool_free_size(buf_size, entries as *mut libc::c_void);
    r
}
#[c2rust::src_loc = "286:1"]
unsafe extern "C" fn try_path(
    mut base: *const libc::c_char,
    mut path: *const libc::c_char,
    mut out_found_notdir: *mut bool,
) -> *const libc::c_char {
    let mut composite: [libc::c_char; 1024] = [0; 1024];
    let mut file: [libc::c_char; 1024] = [0; 1024];
    path_cat(base, path, composite.as_mut_ptr());
    if pony_realpath(composite.as_mut_ptr(), file.as_mut_ptr()) != file.as_mut_ptr() {
        return 0 as *const libc::c_char;
    }
    let mut s: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut err: libc::c_int = stat(file.as_mut_ptr(), &mut s);
    if err == -(1 as libc::c_int) {
        return 0 as *const libc::c_char;
    }
    if !(s.st_mode as libc::c_int & 0o170000 as libc::c_int == 0o40000 as libc::c_int) {
        if !out_found_notdir.is_null() {
            *out_found_notdir = 1 as libc::c_int != 0;
        }
        return 0 as *const libc::c_char;
    }
    return stringtab(file.as_mut_ptr());
}
#[c2rust::src_loc = "315:1"]
unsafe extern "C" fn is_root(mut path: *const libc::c_char) -> bool {
    if !path.is_null() {
    } else {
        ponyint_assert_fail(
            b"path != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            317 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"is_root\0")).as_ptr(),
        );
    };
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *path.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn try_package_path(
    mut base: *const libc::c_char,
    mut path: *const libc::c_char,
    mut out_found_notdir: *mut bool,
) -> *const libc::c_char {
    let mut path1: [libc::c_char; 1024] = [0; 1024];
    let mut path2: [libc::c_char; 1024] = [0; 1024];
    path_cat(0 as *const libc::c_char, base, path1.as_mut_ptr());
    loop {
        path_cat(
            path1.as_mut_ptr(),
            b"..\0" as *const u8 as *const libc::c_char,
            path2.as_mut_ptr(),
        );
        if pony_realpath(path2.as_mut_ptr(), path1.as_mut_ptr()) != path1.as_mut_ptr() {
            break;
        }
        path_cat(
            path1.as_mut_ptr(),
            b"pony_packages\0" as *const u8 as *const libc::c_char,
            path2.as_mut_ptr(),
        );
        let mut result: *const libc::c_char = try_path(path2.as_mut_ptr(), path, out_found_notdir);
        if !result.is_null() {
            return result;
        }
        if is_root(path1.as_mut_ptr()) {
            break;
        }
    }
    return 0 as *const libc::c_char;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn find_path(
    mut from: *mut ast_t,
    mut path: *const libc::c_char,
    mut out_is_relative: *mut bool,
    mut out_found_notdir: *mut bool,
    mut opt: *mut pass_opt_t,
) -> *const libc::c_char {
    if !out_is_relative.is_null() {
        *out_is_relative = 0 as libc::c_int != 0;
    }
    if !out_found_notdir.is_null() {
        *out_found_notdir = 0 as libc::c_int != 0;
    }
    if is_path_absolute(path) {
        return try_path(0 as *const libc::c_char, path, out_found_notdir);
    }
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    if from.is_null() || ast_id(from) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
        base = 0 as *const libc::c_char;
    } else {
        from = ast_nearest(from, TK_PACKAGE);
        let mut pkg: *mut package_t = ast_data(from) as *mut package_t;
        base = (*pkg).path;
    }
    let mut result: *const libc::c_char = try_path(base, path, out_found_notdir);
    if !result.is_null() {
        if !out_is_relative.is_null() {
            *out_is_relative = 1 as libc::c_int != 0;
        }
        return result;
    }
    if !is_path_relative(path) {
        if !base.is_null() {
            result = try_package_path(base, path, out_found_notdir);
            if !result.is_null() {
                return result;
            }
            if !from.is_null()
                && ast_id(from) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint
            {
                let mut target: *mut ast_t = ast_child(ast_parent(from));
                let mut pkg_0: *mut package_t = ast_data(target) as *mut package_t;
                base = (*pkg_0).path;
                result = try_package_path(base, path, out_found_notdir);
                if !result.is_null() {
                    return result;
                }
            }
        }
        let mut p: *mut strlist_t = (*opt).package_search_paths;
        while !p.is_null() {
            result = try_path(strlist_data(p), path, out_found_notdir);
            if !result.is_null() {
                return result;
            }
            p = strlist_next(p);
        }
    }
    return 0 as *const libc::c_char;
}
#[c2rust::src_loc = "446:1"]
unsafe extern "C" fn id_to_string(
    mut prefix: *const libc::c_char,
    mut id: usize,
) -> *const libc::c_char {
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    let mut len: usize = libc::strlen(prefix);
    let mut buf_size: usize = len.wrapping_add((32 as libc::c_int as libc::c_ulong).try_into().unwrap());
    let mut buffer: *mut libc::c_char = ponyint_pool_alloc_size(buf_size) as *mut libc::c_char;
    snprintf(
        buffer,
        buf_size.try_into().unwrap(),
        b"%s$%zu\0" as *const u8 as *const libc::c_char,
        prefix,
        id,
    );
    return stringtab_consume(buffer, buf_size);
}
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn symbol_in_use(
    mut program: *mut ast_t,
    mut symbol: *const libc::c_char,
) -> bool {
    let mut package: *mut ast_t = ast_child(program);
    while !package.is_null() {
        let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
        if (*pkg).symbol == symbol {
            return 1 as libc::c_int != 0;
        }
        package = ast_sibling(package);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "477:1"]
unsafe extern "C" fn string_to_symbol(mut string: *const libc::c_char) -> *const libc::c_char {
    let mut prefix: bool = 0 as libc::c_int != 0;
    if !(*string.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
        && *string.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32)
        && !(*string.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
            && *string.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32)
    {
        prefix = 1 as libc::c_int != 0;
    }
    let mut len: usize = libc::strlen(string);
    let mut buf_size: usize = len.wrapping_add((prefix as libc::c_ulong).try_into().unwrap()).wrapping_add(1);
    let mut buf: *mut libc::c_char = ponyint_pool_alloc_size(buf_size) as *mut libc::c_char;
    memcpy(
        buf.offset(prefix as libc::c_int as isize) as *mut libc::c_void,
        string as *const libc::c_void,
        len.wrapping_add(1).try_into().unwrap(),
    );
    if prefix {
        *buf.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
    }
    let mut i: usize = prefix as usize;
    while i < len {
        if !(*buf.offset(i as isize) as libc::c_int == '_' as i32
            || *buf.offset(i as isize) as libc::c_int >= 'a' as i32
                && *buf.offset(i as isize) as libc::c_int <= 'z' as i32
            || *buf.offset(i as isize) as libc::c_int >= '0' as i32
                && *buf.offset(i as isize) as libc::c_int <= '9' as i32)
        {
            if *buf.offset(i as isize) as libc::c_int >= 'A' as i32
                && *buf.offset(i as isize) as libc::c_int <= 'Z' as i32
            {
                let ref mut fresh2 = *buf.offset(i as isize);
                *fresh2 = (*fresh2 as libc::c_int | 0x20 as libc::c_int) as libc::c_char;
            } else {
                *buf.offset(i as isize) = '_' as i32 as libc::c_char;
            }
        }
        i = i.wrapping_add(1);
    }
    return stringtab_consume(buf, buf_size);
}
#[c2rust::src_loc = "518:1"]
unsafe extern "C" fn symbol_suffix(
    mut symbol: *const libc::c_char,
    mut suffix: usize,
) -> *const libc::c_char {
    let mut len: usize = libc::strlen(symbol);
    let mut buf_size: usize = len.wrapping_add((32 as libc::c_int as libc::c_ulong).try_into().unwrap());
    let mut buf: *mut libc::c_char = ponyint_pool_alloc_size(buf_size) as *mut libc::c_char;
    snprintf(
        buf,
        buf_size.try_into().unwrap(),
        b"%s%zu\0" as *const u8 as *const libc::c_char,
        symbol,
        suffix,
    );
    stringtab_consume(buf, buf_size)
}
#[c2rust::src_loc = "529:1"]
unsafe extern "C" fn create_package_symbol(
    mut program: *mut ast_t,
    mut filename: *const libc::c_char,
) -> *const libc::c_char {
    let mut symbol: *const libc::c_char = string_to_symbol(filename);
    let mut suffix: usize = 1 as libc::c_int as usize;
    while symbol_in_use(program, symbol) {
        symbol = symbol_suffix(symbol, suffix);
        suffix = suffix.wrapping_add(1);
    }
    return symbol;
}
#[no_mangle]
#[c2rust::src_loc = "545:1"]
pub unsafe extern "C" fn create_package(
    mut program: *mut ast_t,
    mut name: *const libc::c_char,
    mut qualified_name: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> *mut ast_t {
    let mut package: *mut ast_t = ast_blank(TK_PACKAGE);
    let mut pkg_id: u32 = program_assign_pkg_id(program);
    let mut pkg: *mut package_t = ponyint_pool_alloc(2 as libc::c_int as usize) as *mut package_t;
    let ref mut fresh3 = (*pkg).path;
    *fresh3 = name;
    let ref mut fresh4 = (*pkg).qualified_name;
    *fresh4 = qualified_name;
    let ref mut fresh5 = (*pkg).id;
    *fresh5 = id_to_string(0 as *const libc::c_char, pkg_id as usize);
    let mut p: *const libc::c_char = strrchr((*pkg).path, '/' as i32);
    if p.is_null() {
        p = (*pkg).path;
    } else {
        p = p.offset(1 as libc::c_int as isize);
    }
    let ref mut fresh6 = (*pkg).filename;
    *fresh6 = stringtab(p);
    if pkg_id > 1 as libc::c_int as libc::c_uint {
        let ref mut fresh7 = (*pkg).symbol;
        *fresh7 = create_package_symbol(program, (*pkg).filename);
    } else {
        let ref mut fresh8 = (*pkg).symbol;
        *fresh8 = 0 as *const libc::c_char;
    }
    let ref mut fresh9 = (*pkg).ast;
    *fresh9 = package;
    package_set_init(&mut (*pkg).dependencies, 1 as libc::c_int as usize);
    let ref mut fresh10 = (*pkg).group;
    *fresh10 = 0 as *mut package_group_t;
    (*pkg).group_index = -(1 as libc::c_int) as usize;
    (*pkg).next_hygienic_id = 0 as libc::c_int as usize;
    (*pkg).low_index = -(1 as libc::c_int) as usize;
    ast_setdata(package, pkg as *mut libc::c_void);
    ast_scope(package);
    ast_append(program, package);
    ast_set(
        program,
        (*pkg).path,
        package,
        SYM_NONE,
        0 as libc::c_int != 0,
    );
    ast_set(program, (*pkg).id, package, SYM_NONE, 0 as libc::c_int != 0);
    let mut safe: *mut strlist_t = (*opt).safe_packages;
    if !safe.is_null() && (strlist_find(safe, (*pkg).path)).is_null() {
        (*pkg).allow_ffi = 0 as libc::c_int != 0;
    } else {
        (*pkg).allow_ffi = 1 as libc::c_int != 0;
    }
    (*pkg).on_stack = 0 as libc::c_int != 0;
    return package;
}
#[c2rust::src_loc = "597:1"]
unsafe extern "C" fn add_path(mut path: *const libc::c_char, mut opt: *mut pass_opt_t) -> bool {
    let mut s: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut err: libc::c_int = stat(path, &mut s);
    if err != -(1 as libc::c_int)
        && s.st_mode as libc::c_int & 0o170000 as libc::c_int == 0o40000 as libc::c_int
    {
        path = stringtab(path);
        let mut search: *mut strlist_t = (*opt).package_search_paths;
        if (strlist_find(search, path)).is_null() {
            let ref mut fresh11 = (*opt).package_search_paths;
            *fresh11 = strlist_append(search, path);
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn add_relative_path(
    mut path: *const libc::c_char,
    mut relpath: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if (libc::strlen(path)).wrapping_add(libc::strlen(relpath))
        >= (1024 as libc::c_int as libc::c_ulong).try_into().unwrap()
    {
        return 0 as libc::c_int != 0;
    }
    strcpy(buf.as_mut_ptr(), path);
    strcat(buf.as_mut_ptr(), relpath);
    return add_path(buf.as_mut_ptr(), opt);
}
#[c2rust::src_loc = "644:1"]
unsafe extern "C" fn add_safe(mut path: *const libc::c_char, mut opt: *mut pass_opt_t) -> bool {
    path = stringtab(path);
    let mut safe: *mut strlist_t = (*opt).safe_packages;
    if (strlist_find(safe, path)).is_null() {
        let ref mut fresh12 = (*opt).safe_packages;
        *fresh12 = strlist_append(safe, path);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "658:1"]
unsafe extern "C" fn add_pony_installation_dir(
    mut path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut success: bool = 1 as libc::c_int != 0;
    add_path(path, opt);
    let mut link_arch: *const libc::c_char = if !((*opt).link_arch).is_null() {
        (*opt).link_arch as *const libc::c_char
    } else {
        b"native\0" as *const u8 as *const libc::c_char
    };
    let mut lib_len: usize =
        (8 as libc::c_int as libc::c_ulong).wrapping_add(libc::strlen(link_arch).try_into().unwrap()).try_into().unwrap();
    let mut lib_path: *mut libc::c_char = ponyint_pool_alloc_size(lib_len) as *mut libc::c_char;
    snprintf(
        lib_path,
        lib_len.try_into().unwrap(),
        b"../lib/%s\0" as *const u8 as *const libc::c_char,
        link_arch,
    );
    success = add_relative_path(path, lib_path, opt);
    ponyint_pool_free_size(lib_len, lib_path as *mut libc::c_void);
    if !success {
        return 0 as libc::c_int != 0;
    }
    lib_len = (5 as libc::c_int as libc::c_ulong).wrapping_add(libc::strlen(link_arch).try_into().unwrap());
    lib_path = ponyint_pool_alloc_size(lib_len) as *mut libc::c_char;
    snprintf(
        lib_path,
        lib_len.try_into().unwrap(),
        b"lib/%s\0" as *const u8 as *const libc::c_char,
        link_arch,
    );
    success = add_relative_path(path, lib_path, opt);
    ponyint_pool_free_size(lib_len, lib_path as *mut libc::c_void);
    if !success {
        return 0 as libc::c_int != 0;
    }
    success = add_relative_path(
        path,
        b"../packages\0" as *const u8 as *const libc::c_char,
        opt,
    );
    if !success {
        return 0 as libc::c_int != 0;
    }
    success = add_relative_path(
        path,
        b"../../packages\0" as *const u8 as *const libc::c_char,
        opt,
    );
    if !success {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "722:1"]
unsafe extern "C" fn add_exec_dir(mut opt: *mut pass_opt_t) -> bool {
    let mut path: [libc::c_char; 1024] = [0; 1024];
    let mut success: bool = get_compiler_exe_directory(path.as_mut_ptr(), (*opt).argv0);
    let mut errors: *mut errors_t = (*opt).check.errors;
    if !success {
        errorf(
            errors,
            0 as *const libc::c_char,
            b"Error determining executable path or directory.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    add_path(path.as_mut_ptr(), opt);
    return add_pony_installation_dir(path.as_mut_ptr(), opt);
}
#[no_mangle]
#[c2rust::src_loc = "739:1"]
pub unsafe extern "C" fn package_init(mut opt: *mut pass_opt_t) -> bool {
    if !add_exec_dir(opt) {
        errorf(
            (*opt).check.errors,
            0 as *const libc::c_char,
            b"Error adding package paths relative to ponyc binary location\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    package_add_paths(
        getenv(b"PONYPATH\0" as *const u8 as *const libc::c_char),
        opt,
    );
    add_path(b"/usr/local/lib\0" as *const u8 as *const libc::c_char, opt);
    add_path(b"/opt/local/lib\0" as *const u8 as *const libc::c_char, opt);
    let mut full_safe: *mut strlist_t = 0 as *mut strlist_t;
    let mut safe: *mut strlist_t = (*opt).safe_packages;
    while !safe.is_null() {
        let mut path: *const libc::c_char = 0 as *const libc::c_char;
        safe = strlist_pop(safe, &mut path);
        path = find_path(0 as *mut ast_t, path, 0 as *mut bool, 0 as *mut bool, opt);
        if path.is_null() {
            strlist_free(full_safe);
            strlist_free(safe);
            let ref mut fresh13 = (*opt).safe_packages;
            *fresh13 = 0 as *mut strlist_t;
            return 0 as libc::c_int != 0;
        }
        full_safe = strlist_push(full_safe, path);
    }
    let ref mut fresh14 = (*opt).safe_packages;
    *fresh14 = full_safe;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "791:1"]
pub unsafe extern "C" fn package_init_lib(
    mut opt: *mut pass_opt_t,
    mut pony_installation: *const libc::c_char,
) -> bool {
    package_add_paths(
        getenv(b"PONYPATH\0" as *const u8 as *const libc::c_char),
        opt,
    );
    if !add_pony_installation_dir(pony_installation, opt) {
        errorf(
            (*opt).check.errors,
            0 as *const libc::c_char,
            b"Error adding package paths relative to ponyc installation location\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "804:1"]
pub unsafe extern "C" fn handle_path_list(
    mut paths: *const libc::c_char,
    mut f: path_fn,
    mut opt: *mut pass_opt_t,
) -> bool {
    if paths.is_null() {
        return 1 as libc::c_int != 0;
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    loop {
        let mut p: *const libc::c_char = strchr(paths, ':' as i32);
        let mut len: usize = 0;
        if !p.is_null() {
            len = p.offset_from(paths) as libc::c_long as usize;
        } else {
            len = libc::strlen(paths);
        }
        if len >= (1024 as libc::c_int as libc::c_ulong).try_into().unwrap() {
            errorf(
                (*opt).check.errors,
                0 as *const libc::c_char,
                b"Path too long in %s\0" as *const u8 as *const libc::c_char,
                paths,
            );
        } else {
            let mut path: [libc::c_char; 1024] = [0; 1024];
            memcpy(
                path.as_mut_ptr() as *mut libc::c_void,
                paths as *const libc::c_void,
                len.try_into().unwrap(),
            );
            path[len as usize] = '\0' as i32 as libc::c_char;
            ok = f.expect("non-null function pointer")(path.as_mut_ptr(), opt) as libc::c_int != 0
                && ok as libc::c_int != 0;
        }
        if p.is_null() {
            break;
        }
        paths = p.offset(1 as libc::c_int as isize);
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "846:1"]
pub unsafe extern "C" fn package_add_paths(
    mut paths: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) {
    handle_path_list(
        paths,
        Some(add_path as unsafe extern "C" fn(*const libc::c_char, *mut pass_opt_t) -> bool),
        opt,
    );
}
#[no_mangle]
#[c2rust::src_loc = "852:1"]
pub unsafe extern "C" fn package_add_safe(
    mut paths: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> bool {
    add_safe(b"builtin\0" as *const u8 as *const libc::c_char, opt);
    return handle_path_list(
        paths,
        Some(add_safe as unsafe extern "C" fn(*const libc::c_char, *mut pass_opt_t) -> bool),
        opt,
    );
}
#[no_mangle]
#[c2rust::src_loc = "859:1"]
pub unsafe extern "C" fn package_add_magic_src(
    mut path: *const libc::c_char,
    mut src: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) {
    let mut n: *mut magic_package_t =
        ponyint_pool_alloc(0 as libc::c_int as usize) as *mut magic_package_t;
    let ref mut fresh15 = (*n).path;
    *fresh15 = stringtab(path);
    let ref mut fresh16 = (*n).src;
    *fresh16 = src;
    let ref mut fresh17 = (*n).mapped_path;
    *fresh17 = 0 as *const libc::c_char;
    let ref mut fresh18 = (*n).next;
    *fresh18 = (*opt).magic_packages;
    let ref mut fresh19 = (*opt).magic_packages;
    *fresh19 = n;
}
#[no_mangle]
#[c2rust::src_loc = "870:1"]
pub unsafe extern "C" fn package_add_magic_path(
    mut path: *const libc::c_char,
    mut mapped_path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) {
    let mut n: *mut magic_package_t =
        ponyint_pool_alloc(0 as libc::c_int as usize) as *mut magic_package_t;
    let ref mut fresh20 = (*n).path;
    *fresh20 = stringtab(path);
    let ref mut fresh21 = (*n).src;
    *fresh21 = 0 as *const libc::c_char;
    let ref mut fresh22 = (*n).mapped_path;
    *fresh22 = stringtab(mapped_path);
    let ref mut fresh23 = (*n).next;
    *fresh23 = (*opt).magic_packages;
    let ref mut fresh24 = (*opt).magic_packages;
    *fresh24 = n;
}
#[no_mangle]
#[c2rust::src_loc = "882:1"]
pub unsafe extern "C" fn package_clear_magic(mut opt: *mut pass_opt_t) {
    let mut p: *mut magic_package_t = (*opt).magic_packages;
    while !p.is_null() {
        let mut next: *mut magic_package_t = (*p).next;
        ponyint_pool_free(0 as libc::c_int as usize, p as *mut libc::c_void);
        p = next;
    }
    let ref mut fresh25 = (*opt).magic_packages;
    *fresh25 = 0 as *mut magic_package_t;
}
#[no_mangle]
#[c2rust::src_loc = "897:1"]
pub unsafe extern "C" fn program_load(
    mut path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> *mut ast_t {
    let mut program: *mut ast_t = ast_blank(TK_PROGRAM);
    ast_scope(program);
    (*opt).program_pass = PASS_PARSE;
    if (package_load(
        program,
        stringtab(b"builtin\0" as *const u8 as *const libc::c_char),
        opt,
    ))
    .is_null()
        || (package_load(program, path, opt)).is_null()
    {
        ast_free(program);
        return 0 as *mut ast_t;
    }
    let mut builtin: *mut ast_t = ast_pop(program);
    ast_append(program, builtin);
    if !ast_passes_program(program, opt) {
        ast_free(program);
        return 0 as *mut ast_t;
    }
    return program;
}
#[no_mangle]
#[c2rust::src_loc = "925:1"]
pub unsafe extern "C" fn package_load(
    mut from: *mut ast_t,
    mut path: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> *mut ast_t {
    if !from.is_null() {
    } else {
        ponyint_assert_fail(
            b"from != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            927 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"package_load\0")).as_ptr(),
        );
    };
    let mut magic: *mut magic_package_t = find_magic_package(path, opt);
    let mut full_path: *const libc::c_char = path;
    let mut qualified_name: *const libc::c_char = path;
    let mut program: *mut ast_t = ast_nearest(from, TK_PROGRAM);
    if magic.is_null() {
        let mut is_relative: bool = 0 as libc::c_int != 0;
        let mut found_notdir: bool = 0 as libc::c_int != 0;
        full_path = find_path(from, path, &mut is_relative, &mut found_notdir, opt);
        if full_path.is_null() {
            errorf(
                (*opt).check.errors,
                path,
                b"couldn't locate this path\0" as *const u8 as *const libc::c_char,
            );
            if found_notdir {
                errorf_continue(
                    (*opt).check.errors,
                    path,
                    b"note that a compiler invocation or a 'use' directive must refer to a directory\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return 0 as *mut ast_t;
        }
        if from != program && is_relative as libc::c_int != 0 {
            let mut parent: *mut ast_t = ast_nearest(from, TK_PACKAGE);
            let mut parent_pkg: *mut package_t = ast_data(parent) as *mut package_t;
            if !parent_pkg.is_null() {
                let mut package_path_0: *const libc::c_char = path;
                let mut relatives: usize = 0;
                loop {
                    if strncmp(
                        b"../\0" as *const u8 as *const libc::c_char,
                        package_path_0,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        package_path_0 = package_path_0.offset(3 as libc::c_int as isize);
                        relatives = (relatives as libc::c_ulong).wrapping_add(1) as usize as usize;
                    } else {
                        if !(strncmp(
                            b"./\0" as *const u8 as *const libc::c_char,
                            package_path_0,
                            2 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int)
                        {
                            break;
                        }
                        package_path_0 = package_path_0.offset(2 as libc::c_int as isize);
                    }
                }
                let mut base_name: *const libc::c_char = (*parent_pkg).qualified_name;
                let mut base_name_len: usize = libc::strlen(base_name);
                while relatives > 0 && base_name_len > 0 {
                    if *base_name.offset(base_name_len.wrapping_sub(1) as isize) as libc::c_int
                        == '/' as i32
                    {
                        relatives = (relatives as libc::c_ulong).wrapping_sub(1) as usize as usize;
                    }
                    base_name_len =
                        (base_name_len as libc::c_ulong).wrapping_sub(1) as usize as usize;
                }
                let mut package_path_len: usize = libc::strlen(package_path_0);
                let mut len: usize = base_name_len
                    .wrapping_add(package_path_len)
                    .wrapping_add((2 as libc::c_int as libc::c_ulong).try_into().unwrap());
                let mut q_name: *mut libc::c_char =
                    ponyint_pool_alloc_size(len) as *mut libc::c_char;
                memcpy(
                    q_name as *mut libc::c_void,
                    base_name as *const libc::c_void,
                    base_name_len.try_into().unwrap(),
                );
                *q_name.offset(base_name_len as isize) = '/' as i32 as libc::c_char;
                memcpy(
                    q_name
                        .offset(base_name_len as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    package_path_0 as *const libc::c_void,
                    package_path_len.try_into().unwrap(),
                );
                *q_name.offset(len.wrapping_sub(1) as isize) = '\0' as i32 as libc::c_char;
                qualified_name = stringtab_consume(q_name, len);
            }
        }
        if from == program {
            let mut basepath: *const libc::c_char = strrchr(full_path, '/' as i32);
            if basepath.is_null() {
                basepath = full_path;
            } else {
                basepath = basepath.offset(1 as libc::c_int as isize);
            }
            qualified_name = basepath;
        }
    }
    let mut package: *mut ast_t = ast_get(program, full_path, 0 as *mut sym_status_t);
    if !package.is_null() {
        return package;
    }
    package = create_package(program, full_path, qualified_name, opt);
    if (*opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"Building %s -> %s\n\0" as *const u8 as *const libc::c_char,
            path,
            full_path,
        );
    }
    if !magic.is_null() {
        if !((*magic).src).is_null() {
            if !parse_source_code(package, (*magic).src, opt) {
                return 0 as *mut ast_t;
            }
        } else if !((*magic).mapped_path).is_null() {
            if !parse_files_in_dir(package, (*magic).mapped_path, opt) {
                return 0 as *mut ast_t;
            }
        } else {
            return 0 as *mut ast_t;
        }
    } else if !parse_files_in_dir(package, full_path, opt) {
        return 0 as *mut ast_t;
    }
    if (ast_child(package)).is_null() {
        ast_error(
            (*opt).check.errors,
            package,
            b"no source files in package '%s'\0" as *const u8 as *const libc::c_char,
            path,
        );
        return 0 as *mut ast_t;
    }
    if !ast_passes_subtree(&mut package, opt, (*opt).program_pass) {
        ast_setflag(package, AST_FLAG_PRESERVE as libc::c_int as u32);
        return 0 as *mut ast_t;
    }
    return package;
}
#[no_mangle]
#[c2rust::src_loc = "1068:1"]
pub unsafe extern "C" fn package_free(mut package: *mut package_t) {
    if !package.is_null() {
        package_set_destroy(&mut (*package).dependencies);
        ponyint_pool_free(2 as libc::c_int as usize, package as *mut libc::c_void);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1078:1"]
pub unsafe extern "C" fn package_name(mut ast: *mut ast_t) -> *const libc::c_char {
    let mut pkg: *mut package_t = ast_data(ast_nearest(ast, TK_PACKAGE)) as *mut package_t;
    return if pkg.is_null() {
        0 as *const libc::c_char
    } else {
        (*pkg).id
    };
}
#[no_mangle]
#[c2rust::src_loc = "1085:1"]
pub unsafe extern "C" fn package_id(mut ast: *mut ast_t) -> *mut ast_t {
    return ast_from_string(ast, package_name(ast));
}
#[no_mangle]
#[c2rust::src_loc = "1091:1"]
pub unsafe extern "C" fn package_path(mut package: *mut ast_t) -> *const libc::c_char {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1093 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"package_path\0")).as_ptr(),
        );
    };
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1094 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"package_path\0")).as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    return (*pkg).path;
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn package_qualified_name(mut package: *mut ast_t) -> *const libc::c_char {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1103 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_qualified_name\0",
            ))
            .as_ptr(),
        );
    };
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1104 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_qualified_name\0",
            ))
            .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    return (*pkg).qualified_name;
}
#[no_mangle]
#[c2rust::src_loc = "1111:1"]
pub unsafe extern "C" fn package_filename(mut package: *mut ast_t) -> *const libc::c_char {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1113 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"package_filename\0"))
                .as_ptr(),
        );
    };
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1114 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"package_filename\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    if !pkg.is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1116 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"package_filename\0"))
                .as_ptr(),
        );
    };
    return (*pkg).filename;
}
#[no_mangle]
#[c2rust::src_loc = "1122:1"]
pub unsafe extern "C" fn package_symbol(mut package: *mut ast_t) -> *const libc::c_char {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1124 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"package_symbol\0"))
                .as_ptr(),
        );
    };
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1125 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"package_symbol\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    if !pkg.is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1127 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"package_symbol\0"))
                .as_ptr(),
        );
    };
    return (*pkg).symbol;
}
#[no_mangle]
#[c2rust::src_loc = "1133:1"]
pub unsafe extern "C" fn package_hygienic_id(mut t: *mut typecheck_t) -> *const libc::c_char {
    if !((*(*t).frame).package).is_null() {
    } else {
        ponyint_assert_fail(
            b"t->frame->package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1135 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"package_hygienic_id\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data((*(*t).frame).package) as *mut package_t;
    let ref mut fresh26 = (*pkg).next_hygienic_id;
    let fresh27 = *fresh26;
    *fresh26 = (*fresh26).wrapping_add(1);
    let mut id: usize = fresh27;
    return id_to_string((*pkg).id, id);
}
#[no_mangle]
#[c2rust::src_loc = "1143:1"]
pub unsafe extern "C" fn package_allow_ffi(mut t: *mut typecheck_t) -> bool {
    if !((*(*t).frame).package).is_null() {
    } else {
        ponyint_assert_fail(
            b"t->frame->package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1145 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"package_allow_ffi\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data((*(*t).frame).package) as *mut package_t;
    return (*pkg).allow_ffi;
}
#[no_mangle]
#[c2rust::src_loc = "1151:1"]
pub unsafe extern "C" fn package_alias_from_id(
    mut module: *mut ast_t,
    mut id: *const libc::c_char,
) -> *const libc::c_char {
    if ast_id(module) as libc::c_uint == TK_MODULE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(module) == TK_MODULE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1153 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"package_alias_from_id\0"))
                .as_ptr(),
        );
    };
    let mut strtab_id: *const libc::c_char = stringtab(id);
    let mut use_0: *mut ast_t = ast_child(module);
    while ast_id(use_0) as libc::c_uint == TK_USE as libc::c_int as libc::c_uint {
        let mut imported: *mut ast_t = ast_data(use_0) as *mut ast_t;
        if !imported.is_null()
            && ast_id(imported) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint
        {
        } else {
            ponyint_assert_fail(
                b"(imported != NULL) && (ast_id(imported) == TK_PACKAGE)\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0"
                    as *const u8 as *const libc::c_char,
                1161 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"package_alias_from_id\0",
                ))
                .as_ptr(),
            );
        };
        let mut pkg: *mut package_t = ast_data(imported) as *mut package_t;
        if !pkg.is_null() {
        } else {
            ponyint_assert_fail(
                b"pkg != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0"
                    as *const u8 as *const libc::c_char,
                1164 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"package_alias_from_id\0",
                ))
                .as_ptr(),
            );
        };
        if (*pkg).id == strtab_id {
            let mut alias: *mut ast_t = ast_child(use_0);
            if ast_id(alias) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                return 0 as *const libc::c_char;
            }
            return ast_name(alias);
        }
        use_0 = ast_sibling(use_0);
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"false\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1178 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"package_alias_from_id\0"))
                .as_ptr(),
        );
    };
    return 0 as *const libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "1183:1"]
pub unsafe extern "C" fn package_add_dependency(mut package: *mut ast_t, mut dep: *mut ast_t) {
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1185 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_add_dependency\0",
            ))
            .as_ptr(),
        );
    };
    if ast_id(dep) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(dep) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1186 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_add_dependency\0",
            ))
            .as_ptr(),
        );
    };
    if package == dep {
        return;
    }
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    let mut pkg_dep: *mut package_t = ast_data(dep) as *mut package_t;
    if !pkg.is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1194 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_add_dependency\0",
            ))
            .as_ptr(),
        );
    };
    if !pkg_dep.is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg_dep != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1195 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"package_add_dependency\0",
            ))
            .as_ptr(),
        );
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut in_set: *mut package_t = package_set_get(&mut (*pkg).dependencies, pkg_dep, &mut index);
    if !in_set.is_null() {
        return;
    }
    package_set_putindex(&mut (*pkg).dependencies, pkg_dep, index);
}
#[no_mangle]
#[c2rust::src_loc = "1207:1"]
pub unsafe extern "C" fn package_signature(mut package: *mut ast_t) -> *const libc::c_char {
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1209 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"package_signature\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    if !((*pkg).group).is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg->group != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1212 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"package_signature\0"))
                .as_ptr(),
        );
    };
    return package_group_signature((*pkg).group);
}
#[no_mangle]
#[c2rust::src_loc = "1218:1"]
pub unsafe extern "C" fn package_group_index(mut package: *mut ast_t) -> usize {
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1220 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"package_group_index\0"))
                .as_ptr(),
        );
    };
    let mut pkg: *mut package_t = ast_data(package) as *mut package_t;
    if (*pkg).group_index != -(1 as libc::c_int) as usize {
    } else {
        ponyint_assert_fail(
            b"pkg->group_index != (size_t)-1\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1223 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"package_group_index\0"))
                .as_ptr(),
        );
    };
    return (*pkg).group_index;
}
#[no_mangle]
#[c2rust::src_loc = "1229:1"]
pub unsafe extern "C" fn package_group_new() -> *mut package_group_t {
    let mut group: *mut package_group_t =
        ponyint_pool_alloc(1 as libc::c_int as usize) as *mut package_group_t;
    let ref mut fresh28 = (*group).signature;
    *fresh28 = 0 as *mut libc::c_char;
    package_set_init(&mut (*group).members, 1 as libc::c_int as usize);
    return group;
}
#[no_mangle]
#[c2rust::src_loc = "1238:1"]
pub unsafe extern "C" fn package_group_free(mut group: *mut package_group_t) {
    if !((*group).signature).is_null() {
        ponyint_pool_free_size(
            64 as libc::c_int as usize,
            (*group).signature as *mut libc::c_void,
        );
    }
    package_set_destroy(&mut (*group).members);
    ponyint_pool_free(1 as libc::c_int as usize, group as *mut libc::c_void);
}
#[c2rust::src_loc = "1248:1"]
unsafe extern "C" fn make_dependency_group(
    mut package: *mut package_t,
    mut groups: *mut *mut package_group_list_t,
    mut stack: *mut *mut package_stack_t,
    mut index: *mut usize,
) {
    if !(*package).on_stack {
    } else {
        ponyint_assert_fail(
            b"!package->on_stack\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1251 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"make_dependency_group\0"))
                .as_ptr(),
        );
    };
    let fresh29 = *index;
    *index = (*index).wrapping_add(1);
    let ref mut fresh30 = (*package).low_index;
    *fresh30 = fresh29;
    (*package).group_index = *fresh30;
    *stack = package_stack_push(*stack, package);
    (*package).on_stack = 1 as libc::c_int != 0;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut dep: *mut package_t = 0 as *mut package_t;
    loop {
        dep = package_set_next(&mut (*package).dependencies, &mut i);
        if dep.is_null() {
            break;
        }
        if (*dep).group_index == -(1 as libc::c_int) as usize {
            make_dependency_group(dep, groups, stack, index);
            if (*dep).low_index < (*package).low_index {
                (*package).low_index = (*dep).low_index;
            }
        } else if (*dep).on_stack as libc::c_int != 0 && (*dep).group_index < (*package).low_index {
            (*package).low_index = (*dep).group_index;
        }
    }
    if (*package).group_index == (*package).low_index {
        let mut group: *mut package_group_t = package_group_new();
        let mut member: *mut package_t = 0 as *mut package_t;
        let mut i_0: usize = 0;
        loop {
            *stack = package_stack_pop(*stack, &mut member);
            (*member).on_stack = 0 as libc::c_int != 0;
            let ref mut fresh31 = (*member).group;
            *fresh31 = group;
            let fresh32 = i_0;
            i_0 = i_0.wrapping_add(1);
            (*member).group_index = fresh32;
            package_set_put(&mut (*group).members, member);
            if !(package != member) {
                break;
            }
        }
        *groups = package_group_list_push(*groups, group);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1293:1"]
pub unsafe extern "C" fn package_dependency_groups(
    mut first_package: *mut ast_t,
) -> *mut package_group_list_t {
    let mut groups: *mut package_group_list_t = 0 as *mut package_group_list_t;
    let mut stack: *mut package_stack_t = 0 as *mut package_stack_t;
    let mut index: usize = 0;
    while !first_package.is_null() {
        if ast_id(first_package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(first_package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0"
                    as *const u8 as *const libc::c_char,
                1301 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"package_dependency_groups\0",
                ))
                .as_ptr(),
            );
        };
        let mut package: *mut package_t = ast_data(first_package) as *mut package_t;
        if (*package).group_index == -(1 as libc::c_int) as usize {
            make_dependency_group(package, &mut groups, &mut stack, &mut index);
        }
        first_package = ast_sibling(first_package);
    }
    if stack.is_null() {
    } else {
        ponyint_assert_fail(
            b"stack == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1310 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"package_dependency_groups\0",
            ))
            .as_ptr(),
        );
    };
    return package_group_list_reverse(groups);
}
#[c2rust::src_loc = "1315:1"]
unsafe extern "C" fn print_signature(mut sig: *const libc::c_char) {
    let mut i: usize = 0;
    while i < (64 as libc::c_int as libc::c_ulong).try_into().unwrap() {
        printf(
            b"%02hhX\0" as *const u8 as *const libc::c_char,
            *sig.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1322:1"]
pub unsafe extern "C" fn package_group_dump(mut group: *mut package_group_t) {
    let mut deps: package_set_t = package_set_t {
        contents: hashmap_t {
            count: 0,
            size: 0,
            item_bitmap: 0 as *mut bitmap_t,
            buckets: 0 as *mut hashmap_entry_t,
        },
    };
    package_set_init(&mut deps, 1 as libc::c_int as usize);
    fputs(
        b"Signature: \0" as *const u8 as *const libc::c_char,
        __stdoutp,
    );
    if !((*group).signature).is_null() {
        print_signature((*group).signature);
    } else {
        fputs(b"(NONE)\0" as *const u8 as *const libc::c_char, __stdoutp);
    }
    putchar('\n' as i32);
    puts(b"Members:\0" as *const u8 as *const libc::c_char);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut member: *mut package_t = 0 as *mut package_t;
    loop {
        member = package_set_next(&mut (*group).members, &mut i);
        if member.is_null() {
            break;
        }
        printf(
            b"  %s\n\0" as *const u8 as *const libc::c_char,
            (*member).filename,
        );
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut dep: *mut package_t = 0 as *mut package_t;
        loop {
            dep = package_set_next(&mut (*member).dependencies, &mut j);
            if dep.is_null() {
                break;
            }
            let mut k: usize = -(1 as libc::c_int) as usize;
            let mut in_set: *mut package_t = package_set_get(&mut (*group).members, dep, &mut k);
            if in_set.is_null() {
                k = -(1 as libc::c_int) as usize;
                in_set = package_set_get(&mut deps, dep, &mut k);
                if in_set.is_null() {
                    package_set_putindex(&mut deps, dep, k);
                }
            }
        }
    }
    puts(b"Dependencies:\0" as *const u8 as *const libc::c_char);
    i = -(1 as libc::c_int) as usize;
    loop {
        member = package_set_next(&mut deps, &mut i);
        if member.is_null() {
            break;
        }
        printf(
            b"  %s\n\0" as *const u8 as *const libc::c_char,
            (*member).filename,
        );
    }
    package_set_destroy(&mut deps);
}
#[c2rust::src_loc = "1382:1"]
unsafe extern "C" fn package_dep_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut package: *mut package_t = object as *mut package_t;
    string_trace(ctx, (*package).filename);
    pony_traceknown(
        ctx,
        (*package).group as *mut libc::c_void,
        package_group_dep_signature_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1392:1"]
unsafe extern "C" fn package_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut package: *mut package_t = object as *mut package_t;
    string_trace(ctx, (*package).filename);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut dep: *mut package_t = 0 as *mut package_t;
    loop {
        dep = package_set_next(&mut (*package).dependencies, &mut i);
        if dep.is_null() {
            break;
        }
        pony_traceknown(
            ctx,
            dep as *mut libc::c_void,
            package_dep_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    pony_traceknown(
        ctx,
        (*package).ast as *mut libc::c_void,
        ast_signature_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1412:1"]
unsafe extern "C" fn package_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut package: *mut package_t = object as *mut package_t;
    let mut dst: *mut package_signature_t =
        (buf as libc::uintptr_t).wrapping_add(offset) as *mut package_signature_t;
    let ref mut fresh33 = (*dst).filename;
    *fresh33 = pony_serialise_offset(
        ctx,
        (*package).filename as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh34 = (*dst).group;
    *fresh34 =
        pony_serialise_offset(ctx, (*package).group as *mut libc::c_void) as *mut package_group_t;
    (*dst).group_index = (*package).group_index;
}
#[c2rust::src_loc = "1427:20"]
static mut package_dep_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_signature_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_dep_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1449:1"]
pub unsafe extern "C" fn package_dep_signature_pony_type() -> *const pony_type_t {
    return &package_dep_signature_pony;
}
#[c2rust::src_loc = "1455:20"]
static mut package_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_signature_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1477:1"]
pub unsafe extern "C" fn package_signature_pony_type() -> *const pony_type_t {
    return &package_signature_pony;
}
#[c2rust::src_loc = "1483:1"]
unsafe extern "C" fn package_group_dep_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    if !((*group).signature).is_null() {
    } else {
        ponyint_assert_fail(
            b"group->signature != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1488 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"package_group_dep_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    pony_serialise_reserve(
        ctx,
        (*group).signature as *mut libc::c_void,
        64 as libc::c_int as usize,
    );
}
#[c2rust::src_loc = "1493:1"]
unsafe extern "C" fn package_group_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    if ((*group).signature).is_null() {
    } else {
        ponyint_assert_fail(
            b"group->signature == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1498 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"package_group_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut member: *mut package_t = 0 as *mut package_t;
    loop {
        member = package_set_next(&mut (*group).members, &mut i);
        if member.is_null() {
            break;
        }
        pony_traceknown(
            ctx,
            member as *mut libc::c_void,
            package_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "1511:1"]
unsafe extern "C" fn package_group_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    let mut dst: *mut package_group_t =
        (buf as libc::uintptr_t).wrapping_add(offset) as *mut package_group_t;
    if !((*group).signature).is_null() {
        let mut ptr_offset: libc::uintptr_t =
            pony_serialise_offset(ctx, (*group).signature as *mut libc::c_void);
        let mut dst_sig: *mut libc::c_char =
            (buf as libc::uintptr_t).wrapping_add(ptr_offset) as *mut libc::c_char;
        memcpy(
            dst_sig as *mut libc::c_void,
            (*group).signature as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        let ref mut fresh35 = (*dst).signature;
        *fresh35 = ptr_offset as *mut libc::c_char;
    } else {
        let ref mut fresh36 = (*dst).signature;
        *fresh36 = 0 as *mut libc::c_char;
    };
}
#[c2rust::src_loc = "1532:20"]
static mut package_group_dep_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_group_dep_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_group_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1554:1"]
pub unsafe extern "C" fn package_group_dep_signature_pony_type() -> *const pony_type_t {
    return &package_group_dep_signature_pony;
}
#[c2rust::src_loc = "1560:20"]
static mut package_group_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_group_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_group_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1582:1"]
pub unsafe extern "C" fn package_group_signature_pony_type() -> *const pony_type_t {
    return &package_group_signature_pony;
}
#[c2rust::src_loc = "1588:1"]
unsafe extern "C" fn s_alloc_fn(mut _ctx: *mut pony_ctx_t, mut size: usize) -> *mut libc::c_void {
    return ponyint_pool_alloc_size(size);
}
#[c2rust::src_loc = "1595:1"]
unsafe extern "C" fn s_throw_fn() {
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"false\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0" as *const u8
                as *const libc::c_char,
            1597 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"s_throw_fn\0")).as_ptr(),
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "1602:1"]
pub unsafe extern "C" fn package_group_signature(
    mut group: *mut package_group_t,
) -> *const libc::c_char {
    if ((*group).signature).is_null() {
        let mut ctx: pony_ctx_t = pony_ctx_t {
            scheduler: 0 as *mut scheduler_t,
            current: 0 as *mut pony_actor_t,
            trace_object: None,
            trace_actor: None,
            stack: 0 as *mut gcstack_t,
            acquire: actormap_t {
                contents: hashmap_t {
                    count: 0,
                    size: 0,
                    item_bitmap: 0 as *mut bitmap_t,
                    buckets: 0 as *mut hashmap_entry_t,
                },
            },
            serialise_buffer: 0 as *mut libc::c_void,
            serialise_size: 0,
            serialise: ponyint_serialise_t {
                contents: hashmap_t {
                    count: 0,
                    size: 0,
                    item_bitmap: 0 as *mut bitmap_t,
                    buckets: 0 as *mut hashmap_entry_t,
                },
            },
            serialise_alloc: None,
            serialise_alloc_final: None,
            serialise_throw: None,
        };
        memset(
            &mut ctx as *mut pony_ctx_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<pony_ctx_t>().try_into().unwrap(),
        );
        let mut array: ponyint_array_t = ponyint_array_t {
            t: 0 as *const pony_type_t,
            size: 0,
            alloc: 0,
            ptr: 0 as *mut libc::c_char,
        };
        memset(
            &mut array as *mut ponyint_array_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ponyint_array_t>().try_into().unwrap(),
        );
        let mut buf: *mut libc::c_char =
            ponyint_pool_alloc_size(64 as libc::c_int as usize) as *mut libc::c_char;
        pony_serialise(
            &mut ctx,
            group as *mut libc::c_void,
            package_group_signature_pony_type(),
            &mut array,
            Some(s_alloc_fn as unsafe extern "C" fn(*mut pony_ctx_t, usize) -> *mut libc::c_void),
            Some(::core::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(s_throw_fn)),
        );
        let mut status: libc::c_int = blake2b(
            buf as *mut libc::c_void,
            64 as libc::c_int as usize,
            array.ptr as *const libc::c_void,
            array.size,
            0 as *const libc::c_void,
            0 as libc::c_int as usize,
        );
        if status == 0 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"status == 0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.c\0"
                    as *const u8 as *const libc::c_char,
                1616 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"package_group_signature\0",
                ))
                .as_ptr(),
            );
        };
        let ref mut fresh37 = (*group).signature;
        *fresh37 = buf;
        ponyint_pool_free_size(array.size, array.ptr as *mut libc::c_void);
    }
    return (*group).signature;
}
#[no_mangle]
#[c2rust::src_loc = "1626:1"]
pub unsafe extern "C" fn package_done(mut opt: *mut pass_opt_t) {
    strlist_free((*opt).package_search_paths);
    let ref mut fresh38 = (*opt).package_search_paths;
    *fresh38 = 0 as *mut strlist_t;
    strlist_free((*opt).safe_packages);
    let ref mut fresh39 = (*opt).safe_packages;
    *fresh39 = 0 as *mut strlist_t;
    package_clear_magic(opt);
}
#[c2rust::src_loc = "1638:1"]
unsafe extern "C" fn package_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut package: *mut package_t = object as *mut package_t;
    string_trace(ctx, (*package).path);
    string_trace(ctx, (*package).qualified_name);
    string_trace(ctx, (*package).id);
    string_trace(ctx, (*package).filename);
    if !((*package).symbol).is_null() {
        string_trace(ctx, (*package).symbol);
    }
    pony_traceknown(
        ctx,
        (*package).ast as *mut libc::c_void,
        ast_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    package_set_serialise_trace(
        ctx,
        &mut (*package).dependencies as *mut package_set_t as *mut libc::c_void,
    );
    if !((*package).group).is_null() {
        pony_traceknown(
            ctx,
            (*package).group as *mut libc::c_void,
            package_group_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "1659:1"]
unsafe extern "C" fn package_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut package: *mut package_t = object as *mut package_t;
    let mut dst: *mut package_t = (buf as libc::uintptr_t).wrapping_add(offset) as *mut package_t;
    let ref mut fresh40 = (*dst).path;
    *fresh40 = pony_serialise_offset(
        ctx,
        (*package).path as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh41 = (*dst).qualified_name;
    *fresh41 = pony_serialise_offset(
        ctx,
        (*package).qualified_name as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh42 = (*dst).id;
    *fresh42 = pony_serialise_offset(ctx, (*package).id as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh43 = (*dst).filename;
    *fresh43 = pony_serialise_offset(
        ctx,
        (*package).filename as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh44 = (*dst).symbol;
    *fresh44 = pony_serialise_offset(
        ctx,
        (*package).symbol as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh45 = (*dst).ast;
    *fresh45 = pony_serialise_offset(ctx, (*package).ast as *mut libc::c_void) as *mut ast_t;
    package_set_serialise(
        ctx,
        &mut (*package).dependencies as *mut package_set_t as *mut libc::c_void,
        buf,
        offset.wrapping_add((48 as libc::c_ulong).try_into().unwrap()),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    let ref mut fresh46 = (*dst).group;
    *fresh46 =
        pony_serialise_offset(ctx, (*package).group as *mut libc::c_void) as *mut package_group_t;
    (*dst).group_index = (*package).group_index;
    (*dst).next_hygienic_id = (*package).next_hygienic_id;
    (*dst).low_index = (*package).low_index;
    (*dst).allow_ffi = (*package).allow_ffi;
    (*dst).on_stack = (*package).on_stack;
}
#[c2rust::src_loc = "1688:1"]
unsafe extern "C" fn package_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut package: *mut package_t = object as *mut package_t;
    let ref mut fresh47 = (*package).path;
    *fresh47 = string_deserialise_offset(ctx, (*package).path as libc::uintptr_t);
    let ref mut fresh48 = (*package).qualified_name;
    *fresh48 = string_deserialise_offset(ctx, (*package).qualified_name as libc::uintptr_t);
    let ref mut fresh49 = (*package).id;
    *fresh49 = string_deserialise_offset(ctx, (*package).id as libc::uintptr_t);
    let ref mut fresh50 = (*package).filename;
    *fresh50 = string_deserialise_offset(ctx, (*package).filename as libc::uintptr_t);
    let ref mut fresh51 = (*package).symbol;
    *fresh51 = string_deserialise_offset(ctx, (*package).symbol as libc::uintptr_t);
    let ref mut fresh52 = (*package).ast;
    *fresh52 = pony_deserialise_offset(ctx, ast_pony_type(), (*package).ast as libc::uintptr_t)
        as *mut ast_t;
    package_set_deserialise(
        ctx,
        &mut (*package).dependencies as *mut package_set_t as *mut libc::c_void,
    );
    let ref mut fresh53 = (*package).group;
    *fresh53 = pony_deserialise_offset(
        ctx,
        package_group_pony_type(),
        (*package).group as libc::uintptr_t,
    ) as *mut package_group_t;
}
#[c2rust::src_loc = "1708:20"]
static mut package_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                package_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1730:1"]
pub unsafe extern "C" fn package_pony_type() -> *const pony_type_t {
    return &package_pony;
}
#[c2rust::src_loc = "1736:1"]
unsafe extern "C" fn package_group_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    if !((*group).signature).is_null() {
        pony_serialise_reserve(
            ctx,
            (*group).signature as *mut libc::c_void,
            64 as libc::c_int as usize,
        );
    }
    package_set_serialise_trace(
        ctx,
        &mut (*group).members as *mut package_set_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1747:1"]
unsafe extern "C" fn package_group_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    let mut dst: *mut package_group_t =
        (buf as libc::uintptr_t).wrapping_add(offset) as *mut package_group_t;
    let mut ptr_offset: libc::uintptr_t =
        pony_serialise_offset(ctx, (*group).signature as *mut libc::c_void);
    let ref mut fresh54 = (*dst).signature;
    *fresh54 = ptr_offset as *mut libc::c_char;
    if !((*group).signature).is_null() {
        let mut dst_sig: *mut libc::c_char =
            (buf as libc::uintptr_t).wrapping_add(ptr_offset) as *mut libc::c_char;
        memcpy(
            dst_sig as *mut libc::c_void,
            (*group).signature as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
    }
    package_set_serialise(
        ctx,
        &mut (*group).members as *mut package_set_t as *mut libc::c_void,
        buf,
        offset.wrapping_add((8 as libc::c_ulong).try_into().unwrap()),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1770:1"]
unsafe extern "C" fn package_group_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut group: *mut package_group_t = object as *mut package_group_t;
    let ref mut fresh55 = (*group).signature;
    *fresh55 = pony_deserialise_block(
        ctx,
        (*group).signature as libc::uintptr_t,
        64 as libc::c_int as usize,
    ) as *mut libc::c_char;
    package_set_deserialise(
        ctx,
        &mut (*group).members as *mut package_set_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1780:20"]
static mut package_group_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<package_group_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                package_group_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                package_group_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                package_group_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
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
#[c2rust::src_loc = "1802:1"]
pub unsafe extern "C" fn package_group_pony_type() -> *const pony_type_t {
    return &package_group_pony;
}
#[no_mangle]
#[c2rust::src_loc = "1808:1"]
pub unsafe extern "C" fn is_path_absolute(mut path: *const libc::c_char) -> bool {
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1827:1"]
pub unsafe extern "C" fn is_path_relative(mut path: *const libc::c_char) -> bool {
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            return 1 as libc::c_int != 0;
        }
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            if *path.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
