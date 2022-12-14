use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:3"]
pub mod _types_h {
    #[c2rust::src_loc = "43:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/dirent.h:3"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/dirent.h:3"]
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
        #[c2rust::src_loc = "109:1"]
        pub fn readdir(_: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:4"]
pub mod pony_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
        #[c2rust::src_loc = "262:1"]
        pub fn pony_alloc(ctx: *mut pony_ctx_t, size: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:3"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "42:7"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:4"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:5"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "117:7"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:13"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "449:1"]
        pub fn getcwd(_: *mut libc::c_char, _: size_t) -> *mut libc::c_char;
    }
}
use self::_malloc_h::free;
pub use self::_pthread_types_h::{__darwin_pthread_mutex_t, _opaque_pthread_mutex_t};
pub use self::_types_h::{__uint16_t, __uint64_t, __uint8_t};
use self::atomics_h::f__atomic_thread_fence;
pub use self::dirent_h::dirent;
pub use self::include_dirent_h::{_telldir, readdir, DIR};
use self::pony_h::{pony_alloc, pony_ctx};
pub use self::stddef_h::size_t;
use self::string_h::{memcpy, strdup, strlen};
use self::unistd_h::getcwd;
#[no_mangle]
#[c2rust::src_loc = "24:26"]
pub static mut cwd_cache: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn pony_os_eexist() -> libc::c_int {
    return 17 as libc::c_int;
}
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn skip_entry(mut entry: *const libc::c_char, mut len: size_t) -> bool {
    if len == 1 as libc::c_int as libc::c_ulong
        && *entry.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        return 1 as libc::c_int != 0;
    }
    if len == 2 as libc::c_int as libc::c_ulong
        && *entry.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *entry.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn pony_os_cwd() -> *mut libc::c_char {
    let mut cwd: *const libc::c_char = { ::core::intrinsics::atomic_load_relaxed(&mut cwd_cache) };
    if cwd.is_null() {
        let mut cwd_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
        cwd_alloc = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
        if cwd_alloc.is_null() {
            cwd_alloc = strdup(b".\0" as *const u8 as *const libc::c_char);
        }
        if !({
            let fresh0 = ::core::intrinsics::atomic_cxchg_acqrel(
                &mut cwd_cache as *mut *const libc::c_char,
                *(&mut cwd as *mut *const libc::c_char),
                cwd_alloc as *const libc::c_char,
            );
            *(&mut cwd as *mut *const libc::c_char) = fresh0.0;
            fresh0.1
        }) {
            free(cwd_alloc as *mut libc::c_void);
        } else {
            cwd = cwd_alloc;
        }
    } else {
        f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
    }
    let mut len: size_t = (strlen(cwd)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut cstring: *mut libc::c_char = pony_alloc(pony_ctx(), len) as *mut libc::c_char;
    memcpy(
        cstring as *mut libc::c_void,
        cwd as *const libc::c_void,
        len,
    );
    cstring
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn ponyint_o_rdonly() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn ponyint_o_rdwr() -> libc::c_int {
    return 0x2 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn ponyint_o_creat() -> libc::c_int {
    return 0x200 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn ponyint_o_trunc() -> libc::c_int {
    return 0x400 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn ponyint_o_directory() -> libc::c_int {
    return 0x100000 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub unsafe extern "C" fn ponyint_o_cloexec() -> libc::c_int {
    return 0x1000000 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub unsafe extern "C" fn ponyint_unix_readdir(mut dir: *mut DIR) -> *const libc::c_char {
    let mut d: *mut dirent = 0 as *mut dirent;
    loop {
        d = readdir(dir);
        if d.is_null() {
            break;
        }
        let mut len: size_t = (*d).d_namlen as size_t;
        if skip_entry(((*d).d_name).as_mut_ptr(), len) {
            continue;
        }
        let mut cstring: *mut libc::c_char = pony_alloc(
            pony_ctx(),
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            cstring as *mut libc::c_void,
            ((*d).d_name).as_mut_ptr() as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        return cstring;
    }
    return 0 as *const libc::c_char;
}
