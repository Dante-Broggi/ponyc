use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "43:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "70:1"]
    pub type __darwin_mode_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:1"]
pub mod _pthread_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:8"]
    pub struct _opaque_pthread_mutex_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 56],
    }
    #[c2rust::src_loc = "113:1"]
    pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/dirent.h:1"]
pub mod dirent_h {
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
pub mod include_dirent_h {
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
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "70:8"]
        pub type _telldir;
        #[c2rust::src_loc = "105:1"]
        pub fn closedir(_: *mut DIR) -> libc::c_int;
        #[c2rust::src_loc = "107:1"]
        pub fn opendir(_: *const libc::c_char) -> *mut DIR;
        #[c2rust::src_loc = "109:1"]
        pub fn readdir(_: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:1"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "227:1"]
        pub fn realpath(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:1"]
pub mod stat_h {
    use super::_mode_t_h::mode_t;
    extern "C" {
        #[c2rust::src_loc = "384:1"]
        pub fn mkdir(_: *const libc::c_char, _: mode_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:1"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:3"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "79:7"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "87:7"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach-o/dyld.h:9"]
pub mod dyld_h {
    extern "C" {
        #[c2rust::src_loc = "98:1"]
        pub fn _NSGetExecutablePath(buf: *mut libc::c_char, bufsize: *mut u32) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libgen.h:158"]
pub mod libgen_h {
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn basename(_: *mut libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::_mode_t_h::mode_t;
pub use self::_pthread_types_h::{__darwin_pthread_mutex_t, _opaque_pthread_mutex_t};
pub use self::_size_t_h::size_t;
pub use self::_types_h::{__darwin_size_t, __uint16_t, __uint64_t, __uint8_t};
pub use self::dirent_h::dirent;
use self::dyld_h::_NSGetExecutablePath;
use self::errno_h::__error;
pub use self::include_dirent_h::{_telldir, closedir, opendir, readdir, DIR};
use self::libgen_h::basename;
use self::pool_h::{ponyint_pool_alloc_size, ponyint_pool_free_size};
use self::stat_h::mkdir;
use self::stdlib_h::realpath;
use self::string_h::{strcpy, strlen, strrchr};
pub use self::sys__types_h::__darwin_mode_t;
#[no_mangle]
#[c2rust::src_loc = "23:1"]
pub unsafe extern "C" fn pony_opendir(
    mut path: *const libc::c_char,
    mut err: *mut u32,
) -> *mut DIR {
    let mut dir: *mut DIR = opendir(path);
    if dir.is_null() {
        *err = *__error() as u32;
        return 0 as *mut DIR;
    }
    dir
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn pony_realpath(
    mut path: *const libc::c_char,
    mut resolved: *mut libc::c_char,
) -> *mut libc::c_char {
    realpath(path, resolved)
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn pony_dir_info_name(mut info: *mut dirent) -> *mut libc::c_char {
    ((*info).d_name).as_mut_ptr()
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn pony_closedir(mut dir: *mut DIR) {
    closedir(dir);
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn pony_dir_entry_next(mut dir: *mut DIR) -> *mut dirent {
    readdir(dir)
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn pony_mkdir(mut path: *const libc::c_char) {
    let mut path_len: usize = libc::strlen(path);
    let mut buf: *mut libc::c_char =
        ponyint_pool_alloc_size(path_len.wrapping_add(1)) as *mut libc::c_char;
    let mut i: usize = 0;
    while i < path_len {
        *buf.offset(i as isize) = *path.offset(i as isize);
        if *path.offset(i as isize) as libc::c_int == '/' as i32 {
            *buf.offset(i.wrapping_add(1) as isize) = '\0' as i32 as libc::c_char;
            mkdir(buf, 0o777 as libc::c_int as mode_t);
        }
        i = i.wrapping_add(1);
    }
    mkdir(path, 0o777 as libc::c_int as mode_t);
    ponyint_pool_free_size(path_len.wrapping_add(1), buf as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub unsafe extern "C" fn get_file_name(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    basename(filename)
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn remove_ext(
    mut path: *const libc::c_char,
    mut dot: libc::c_char,
    mut sep: libc::c_char,
    mut allocated_size: *mut usize,
) -> *mut libc::c_char {
    let mut retstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastdot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastsep: *mut libc::c_char = 0 as *mut libc::c_char;
    if path.is_null() {
        return 0 as *mut libc::c_char;
    }
    *allocated_size = (libc::strlen(path)).wrapping_add(1);
    retstr = ponyint_pool_alloc_size(*allocated_size) as *mut libc::c_char;
    strcpy(retstr, path);
    lastdot = strrchr(retstr, dot as libc::c_int);
    lastsep = if sep as libc::c_int == 0 as libc::c_int {
        0 as *mut libc::c_char
    } else {
        strrchr(retstr, sep as libc::c_int)
    };
    if !lastdot.is_null() {
        if !lastsep.is_null() {
            if lastsep < lastdot {
                *lastdot = '\0' as i32 as libc::c_char;
            }
        } else {
            *lastdot = '\0' as i32 as libc::c_char;
        }
    }
    retstr
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn get_compiler_exe_path(
    mut output_path: *mut libc::c_char,
    mut argv0: *const libc::c_char,
) -> bool {
    let mut success: bool = 0 as libc::c_int != 0;
    success = if argv0.is_null() {
        success as libc::c_int
    } else {
        success as libc::c_int
    } != 0;
    let mut exec_path: [libc::c_char; 1024] = [0; 1024];
    let mut size: u32 = ::core::mem::size_of::<[libc::c_char; 1024]>() as u32;
    let mut r: libc::c_int = _NSGetExecutablePath(exec_path.as_mut_ptr(), &mut size);
    success = r == 0 as libc::c_int;
    if success {
        pony_realpath(exec_path.as_mut_ptr(), output_path);
    }
    success
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn get_compiler_exe_directory(
    mut output_path: *mut libc::c_char,
    mut argv0: *const libc::c_char,
) -> bool {
    let mut can_get_compiler_path: bool = get_compiler_exe_path(output_path, argv0);
    if can_get_compiler_path {
        let mut p: *mut libc::c_char = strrchr(output_path, '/' as i32);
        if p.is_null() {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        *p = '\0' as i32 as libc::c_char;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0;
    };
}
