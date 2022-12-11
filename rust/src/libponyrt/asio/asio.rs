use ::libc;
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
    #[c2rust::src_loc = "103:8"]
    pub struct _opaque_pthread_t {
        pub __sig: libc::c_long,
        pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
        pub __opaque: [libc::c_char; 8176],
    }
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_pthread_t = *mut _opaque_pthread_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_t.h:8"]
pub mod _pthread_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_t = __darwin_pthread_t;
    use super::_pthread_types_h::__darwin_pthread_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/threads.h:8"]
pub mod threads_h {
    #[c2rust::src_loc = "19:1"]
    pub type thread_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>;
    use super::_pthread_t_h::pthread_t;
    
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_thread_create(
            thread: *mut pthread_t,
            start: thread_fn,
            cpu: uint32_t,
            arg: *mut libc::c_void,
        ) -> bool;
        #[c2rust::src_loc = "54:1"]
        pub fn ponyint_thread_join(thread: pthread_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/asio.h:8"]
pub mod asio_h {
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type asio_backend_t;
        #[c2rust::src_loc = "122:1"]
        pub fn ponyint_asio_backend_final(backend: *mut asio_backend_t);
        #[c2rust::src_loc = "132:1"]
        pub fn ponyint_asio_backend_dispatch(arg: *mut libc::c_void) -> *mut libc::c_void;
        #[c2rust::src_loc = "53:1"]
        pub fn ponyint_asio_backend_init() -> *mut asio_backend_t;
    }
}
pub use self::_pthread_t_h::pthread_t;
pub use self::_pthread_types_h::{
    __darwin_pthread_handler_rec, __darwin_pthread_t, _opaque_pthread_t,
};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
use self::asio_h::{
    asio_backend_t, ponyint_asio_backend_dispatch, ponyint_asio_backend_final,
    ponyint_asio_backend_init,
};
pub use self::threads_h::{ponyint_thread_create, ponyint_thread_join, thread_fn};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "12:8"]
pub struct asio_base_t {
    pub tid: pthread_t,
    pub backend: *mut asio_backend_t,
    pub noisy_count: uint64_t,
}
#[c2rust::src_loc = "22:20"]
static mut running_base: asio_base_t = asio_base_t {
    tid: 0 as *const _opaque_pthread_t as *mut _opaque_pthread_t,
    backend: 0 as *const asio_backend_t as *mut asio_backend_t,
    noisy_count: 0,
};
#[c2rust::src_loc = "23:17"]
static mut asio_cpu: uint32_t = 0;
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn ponyint_asio_get_backend() -> *mut asio_backend_t {
    return running_base.backend;
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn ponyint_asio_get_backend_tid() -> pthread_t {
    return running_base.tid;
}
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn ponyint_asio_get_cpu() -> uint32_t {
    return asio_cpu;
}
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn ponyint_asio_init(mut cpu: uint32_t) {
    asio_cpu = cpu;
    running_base.backend = ponyint_asio_backend_init();
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn ponyint_asio_start() -> bool {
    if (running_base.backend).is_null() {
        return 0 as libc::c_int != 0;
    }
    if !ponyint_thread_create(
        &mut running_base.tid,
        Some(
            ponyint_asio_backend_dispatch
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        asio_cpu,
        running_base.backend as *mut libc::c_void,
    ) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn ponyint_asio_stop() -> bool {
    if !ponyint_asio_stoppable() {
        return 0 as libc::c_int != 0;
    }
    if !(running_base.backend).is_null() {
        ponyint_asio_backend_final(running_base.backend);
        ponyint_thread_join(running_base.tid);
        running_base.backend = 0 as *mut asio_backend_t;
        running_base.tid = 0 as pthread_t;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn ponyint_asio_stoppable() -> bool {
    return ({
        ::core::intrinsics::atomic_load_acq(&mut running_base.noisy_count as *mut uint64_t)
    }) == 0 as libc::c_int as libc::c_ulonglong;
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn ponyint_asio_noisy_add() -> uint64_t {
    return ({
        ::core::intrinsics::atomic_xadd_rel(
            &mut running_base.noisy_count,
            1 as libc::c_int as uint64_t,
        )
    });
}
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub unsafe extern "C" fn ponyint_asio_noisy_remove() -> uint64_t {
    return ({
        ::core::intrinsics::atomic_xsub_rel(
            &mut running_base.noisy_count,
            1 as libc::c_int as uint64_t,
        )
    });
}
