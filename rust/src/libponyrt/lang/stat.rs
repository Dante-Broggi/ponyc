use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
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
        #[c2rust::src_loc = "382:1"]
        pub fn fstat(_: libc::c_int, _: *mut stat) -> libc::c_int;
        #[c2rust::src_loc = "383:1"]
        pub fn lstat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
        #[c2rust::src_loc = "386:1"]
        pub fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:2"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
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
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;
    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;

    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_off_t_h::off_t;
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_time_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_type_t,
};
pub use self::stat_h::{fstat, lstat, stat};
pub use self::stddef_h::size_t;
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "7:16"]
pub struct pony_mode_t {
    pub desc: *const pony_type_t,
    pub setuid: bool,
    pub setgid: bool,
    pub sticky: bool,
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_exec: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_exec: bool,
    pub any_read: bool,
    pub any_write: bool,
    pub any_exec: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "24:16"]
pub struct pony_stat_t {
    pub desc: *const pony_type_t,
    pub path: *mut libc::c_void,
    pub mode: *mut pony_mode_t,
    pub hard_links: u32,
    pub device: u64,
    pub inode: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: usize,
    pub access_time: i64,
    pub access_time_nsec: i64,
    pub modified_time: i64,
    pub modified_time_nsec: i64,
    pub change_time: i64,
    pub change_time_nsec: i64,
    pub file: bool,
    pub directory: bool,
    pub pipe: bool,
    pub symlink: bool,
    pub broken: bool,
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn unix_file_type(mut p: *mut pony_stat_t, mut st: *mut stat) {
    (*p).file = ((*st).st_mode as libc::c_int & 0o170000 as libc::c_int == 0o100000 as libc::c_int)
        as libc::c_int
        != 0 as libc::c_int;
    (*p).directory = ((*st).st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o40000 as libc::c_int) as libc::c_int
        != 0 as libc::c_int;
    (*p).pipe = ((*st).st_mode as libc::c_int & 0o170000 as libc::c_int == 0o10000 as libc::c_int)
        as libc::c_int
        != 0 as libc::c_int;
    (*p).size = (*st).st_size as usize;
}
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn unix_stat(mut p: *mut pony_stat_t, mut st: *mut stat) {
    (*p).hard_links = (*st).st_nlink as u32;
    (*p).device = (*st).st_dev as u64;
    (*p).inode = (*st).st_ino;
    (*p).uid = (*st).st_uid;
    (*p).gid = (*st).st_gid;
    unix_file_type(p, st);
    (*p).symlink = ((*st).st_mode as libc::c_int & 0o170000 as libc::c_int
        == 0o120000 as libc::c_int) as libc::c_int
        != 0 as libc::c_int;
    (*p).broken = 0 as libc::c_int != 0;
    (*(*p).mode).setuid = (*st).st_mode as libc::c_int & 0o4000 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).setgid = (*st).st_mode as libc::c_int & 0o2000 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).sticky = (*st).st_mode as libc::c_int & 0o1000 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).owner_read =
        (*st).st_mode as libc::c_int & 0o400 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).owner_write =
        (*st).st_mode as libc::c_int & 0o200 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).owner_exec =
        (*st).st_mode as libc::c_int & 0o100 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).group_read =
        (*st).st_mode as libc::c_int & 0o40 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).group_write =
        (*st).st_mode as libc::c_int & 0o20 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).group_exec =
        (*st).st_mode as libc::c_int & 0o10 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).any_read = (*st).st_mode as libc::c_int & 0o4 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).any_write = (*st).st_mode as libc::c_int & 0o2 as libc::c_int != 0 as libc::c_int;
    (*(*p).mode).any_exec = (*st).st_mode as libc::c_int & 0o1 as libc::c_int != 0 as libc::c_int;
    (*p).access_time = (*st).st_atimespec.tv_sec as i64;
    (*p).modified_time = (*st).st_mtimespec.tv_sec as i64;
    (*p).change_time = (*st).st_ctimespec.tv_sec as i64;
    (*p).access_time_nsec = (*st).st_atimespec.tv_nsec as i64;
    (*p).modified_time_nsec = (*st).st_mtimespec.tv_nsec as i64;
    (*p).change_time_nsec = (*st).st_ctimespec.tv_nsec as i64;
}
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn unix_symlink(mut r: libc::c_int, mut p: *mut pony_stat_t, mut st: *mut stat) {
    if r != 0 as libc::c_int {
        (*p).broken = 1 as libc::c_int != 0;
        (*p).size = 0 as libc::c_int as usize;
    } else {
        unix_file_type(p, st);
    };
}
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn pony_os_fstatat(
    mut _fd: libc::c_int,
    mut _path: *const libc::c_char,
    mut _p: *mut pony_stat_t,
) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn pony_os_fstat(
    mut fd: libc::c_int,
    mut _path: *const libc::c_char,
    mut p: *mut pony_stat_t,
) -> bool {
    let mut st: stat = stat {
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
    if fstat(fd, &mut st) != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    unix_stat(p, &mut st);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "227:1"]
pub unsafe extern "C" fn pony_os_stat(
    mut path: *const libc::c_char,
    mut p: *mut pony_stat_t,
) -> bool {
    let mut st: stat = stat {
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
    if lstat(path, &mut st) != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    unix_stat(p, &mut st);
    if (*p).symlink {
        unix_symlink(
            (stat(path, &mut st) != 0 as libc::c_int) as libc::c_int,
            p,
            &mut st,
        );
    }
    return 1 as libc::c_int != 0;
}
