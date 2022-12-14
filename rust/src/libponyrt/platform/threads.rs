use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:4"]
pub mod _types_h {
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = u64;
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
    #[c2rust::src_loc = "63:8"]
    pub struct _opaque_pthread_attr_t {
        pub __sig: libc::c_long,
        pub __opaque: [libc::c_char; 56],
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
    #[c2rust::src_loc = "109:1"]
    pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
    #[c2rust::src_loc = "110:1"]
    pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
    #[c2rust::src_loc = "113:1"]
    pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_pthread_t = *mut _opaque_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:4"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_attr_t.h:4"]
pub mod _pthread_attr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_attr_t = __darwin_pthread_attr_t;
    use super::_pthread_types_h::__darwin_pthread_attr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_cond_t.h:4"]
pub mod _pthread_cond_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_cond_t = __darwin_pthread_cond_t;
    use super::_pthread_types_h::__darwin_pthread_cond_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_mutex_t.h:4"]
pub mod _pthread_mutex_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_mutex_t = __darwin_pthread_mutex_t;
    use super::_pthread_types_h::__darwin_pthread_mutex_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:4"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/threads.h:4"]
pub mod threads_h {
    #[c2rust::src_loc = "19:1"]
    pub type thread_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/resource.h:4"]
pub mod resource_h {
    #[c2rust::src_loc = "89:1"]
    pub type rlim_t = u64;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "459:8"]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        #[c2rust::src_loc = "567:1"]
        pub fn getrlimit(_: libc::c_int, _: *mut rlimit) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pthread/pthread.h:4"]
pub mod pthread_h {
    use super::_pthread_attr_t_h::pthread_attr_t;
    use super::_pthread_cond_t_h::pthread_cond_t;
    use super::_pthread_mutex_t_h::pthread_mutex_t;
    use super::_pthread_t_h::pthread_t;

    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "229:1"]
        pub fn pthread_attr_destroy(_: *mut pthread_attr_t) -> libc::c_int;
        #[c2rust::src_loc = "262:1"]
        pub fn pthread_attr_init(_: *mut pthread_attr_t) -> libc::c_int;
        #[c2rust::src_loc = "290:1"]
        pub fn pthread_attr_setstacksize(_: *mut pthread_attr_t, _: usize) -> libc::c_int;
        #[c2rust::src_loc = "308:1"]
        pub fn pthread_cond_signal(_: *mut pthread_cond_t) -> libc::c_int;
        #[c2rust::src_loc = "318:1"]
        pub fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> libc::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn pthread_create(
            _: *mut pthread_t,
            _: *const pthread_attr_t,
            _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            _: *mut libc::c_void,
        ) -> libc::c_int;
        #[c2rust::src_loc = "348:1"]
        pub fn pthread_detach(_: pthread_t) -> libc::c_int;
        #[c2rust::src_loc = "369:1"]
        pub fn pthread_join(_: pthread_t, _: *mut *mut libc::c_void) -> libc::c_int;
        #[c2rust::src_loc = "493:1"]
        pub fn pthread_self() -> pthread_t;
    }
}
pub use self::_pthread_attr_t_h::pthread_attr_t;
pub use self::_pthread_cond_t_h::pthread_cond_t;
pub use self::_pthread_mutex_t_h::pthread_mutex_t;
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_attr_t, __darwin_pthread_cond_t, __darwin_pthread_handler_rec,
    __darwin_pthread_mutex_t, __darwin_pthread_t, _opaque_pthread_attr_t, _opaque_pthread_cond_t,
    _opaque_pthread_mutex_t, _opaque_pthread_t,
};
use self::pthread_h::{
    pthread_attr_destroy, pthread_attr_init, pthread_attr_setstacksize, pthread_cond_signal,
    pthread_cond_wait, pthread_create, pthread_detach, pthread_join, pthread_self,
};
pub use self::resource_h::{getrlimit, rlim_t, rlimit};
pub use self::stddef_h::size_t;
pub use self::threads_h::thread_fn;
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn ponyint_thread_create(
    mut thread: *mut pthread_t,
    mut start: thread_fn,
    mut _cpu: u32,
    mut arg: *mut libc::c_void,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut setstack_called: bool = 0 as libc::c_int != 0;
    let mut limit: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut attr: pthread_attr_t = pthread_attr_t {
        __sig: 0,
        __opaque: [0; 56],
    };
    let mut attr_p: *mut pthread_attr_t = &mut attr;
    pthread_attr_init(attr_p);
    if getrlimit(3 as libc::c_int, &mut limit) == 0 as libc::c_int
        && limit.rlim_cur
            != ((1 as libc::c_int as u64) << 63 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
        && limit.rlim_cur >= 8192 as libc::c_int as rlim_t
    {
        if !setstack_called {
            pthread_attr_setstacksize(&mut attr, limit.rlim_cur as usize);
        }
    } else {
        attr_p = 0 as *mut pthread_attr_t;
    }
    if pthread_create(thread, attr_p, start, arg) != 0 {
        ret = 0 as libc::c_int != 0;
    }
    pthread_attr_destroy(&mut attr);
    ret
}
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn ponyint_thread_join(mut thread: pthread_t) -> bool {
    if pthread_join(thread, 0 as *mut *mut libc::c_void) != 0 {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "235:1"]
pub unsafe extern "C" fn ponyint_thread_detach(mut thread: pthread_t) {
    pthread_detach(thread);
}
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn ponyint_thread_self() -> pthread_t {
    pthread_self()
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn ponyint_thread_suspend(
    mut signal: *mut pthread_cond_t,
    mut mut_0: *mut pthread_mutex_t,
) {
    let mut ret: libc::c_int = 0;
    ret = pthread_cond_wait(signal, mut_0);
    // TODO: What to do if `ret` is an unrecoverable error?
    let _ = ret;
}
#[no_mangle]
#[c2rust::src_loc = "277:1"]
pub unsafe extern "C" fn ponyint_thread_wake(
    mut _thread: pthread_t,
    mut signal: *mut pthread_cond_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = pthread_cond_signal(signal);
    ret
}
