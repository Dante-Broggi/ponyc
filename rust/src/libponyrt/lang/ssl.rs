use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:1"]
pub mod _pthread_types_h {
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
    #[c2rust::src_loc = "113:1"]
    pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
    #[c2rust::src_loc = "114:1"]
    pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_mutex_t.h:1"]
pub mod _pthread_mutex_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_mutex_t = __darwin_pthread_mutex_t;
    use super::_pthread_types_h::__darwin_pthread_mutex_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_mutexattr_t.h:1"]
pub mod _pthread_mutexattr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
    use super::_pthread_types_h::__darwin_pthread_mutexattr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pthread/pthread.h:1"]
pub mod pthread_h {
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_mutexattr_t_h::pthread_mutexattr_t;
    extern "C" {
        #[c2rust::src_loc = "387:1"]
        pub fn pthread_mutex_init(
            _: *mut pthread_mutex_t,
            _: *const pthread_mutexattr_t,
        ) -> libc::c_int;
        #[c2rust::src_loc = "391:1"]
        pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "403:1"]
        pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
    }
}
pub use self::_pthread_mutex_t_h::pthread_mutex_t;
pub use self::_pthread_mutexattr_t_h::pthread_mutexattr_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_mutex_t, __darwin_pthread_mutexattr_t, _opaque_pthread_mutex_t,
    _opaque_pthread_mutexattr_t,
};
use self::pool_h::ponyint_pool_alloc_size;
use self::pthread_h::{pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock};
pub use self::stddef_h::size_t;
#[c2rust::src_loc = "7:25"]
static mut locks: *mut pthread_mutex_t = 0 as *const pthread_mutex_t as *mut pthread_mutex_t;
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn locking_callback(
    mut mode: libc::c_int,
    mut type_0: libc::c_int,
    mut _file: *const libc::c_char,
    mut _line: libc::c_int,
) {
    if mode & 1 as libc::c_int != 0 {
        pthread_mutex_lock(&mut *locks.offset(type_0 as isize));
    } else {
        pthread_mutex_unlock(&mut *locks.offset(type_0 as isize));
    };
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn ponyint_ssl_multithreading(mut count: u32) -> *mut libc::c_void {
    locks = ponyint_pool_alloc_size(
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<pthread_mutex_t>()),
    ) as *mut pthread_mutex_t;
    let mut i: u32 = 0 as libc::c_int as u32;
    while i < count {
        pthread_mutex_init(
            &mut *locks.offset(i as isize),
            0 as *const pthread_mutexattr_t,
        );
        i = i.wrapping_add(1);
    }
    ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_char, libc::c_int) -> (),
        >,
        *mut libc::c_void,
    >(Some(
        locking_callback
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
                libc::c_int,
            ) -> (),
    ))
}
