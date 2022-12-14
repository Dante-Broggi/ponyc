use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "46:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "51:1"]
    pub type __darwin_intptr_t = libc::c_long;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_pid_t = i32;
    #[c2rust::src_loc = "73:1"]
    pub type __darwin_sigset_t = u32;
    #[c2rust::src_loc = "75:1"]
    pub type __darwin_uid_t = u32;
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h:1"]
pub mod _intptr_t_h {
    #[c2rust::src_loc = "32:1"]
    pub type intptr_t = __darwin_intptr_t;
    use super::_types_h::__darwin_intptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "196:1"]
        pub fn pony_alloc_msg(index: u32, id: u32) -> *mut pony_msg_t;
        #[c2rust::src_loc = "449:1"]
        pub fn pony_register_thread();
        #[c2rust::src_loc = "457:1"]
        pub fn pony_unregister_thread();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:5"]
pub mod messageq_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6:16"]
    pub struct messageq_t {
        pub head: *mut pony_msg_t,
        pub tail: *mut pony_msg_t,
    }
    use super::pony_h::pony_msg_t;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn ponyint_messageq_init(q: *mut messageq_t);
        #[c2rust::src_loc = "21:1"]
        pub fn ponyint_messageq_destroy(q: *mut messageq_t, maybe_non_empty: bool);
        #[c2rust::src_loc = "59:1"]
        pub fn ponyint_thread_messageq_pop(q: *mut messageq_t) -> *mut pony_msg_t;
        #[c2rust::src_loc = "45:1"]
        pub fn ponyint_thread_messageq_push(
            q: *mut messageq_t,
            first: *mut pony_msg_t,
            last: *mut pony_msg_t,
        ) -> bool;
    }
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sigset_t.h:1"]
pub mod _sigset_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sigset_t = __darwin_sigset_t;
    use super::sys__types_h::__darwin_sigset_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:1"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/signal.h:1"]
pub mod signal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct __siginfo {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub si_pid: pid_t,
        pub si_uid: uid_t,
        pub si_status: libc::c_int,
        pub si_addr: *mut libc::c_void,
        pub si_value: sigval,
        pub si_band: libc::c_long,
        pub __pad: [libc::c_ulong; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "269:7"]
    pub union __sigaction_u {
        pub __sa_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
        pub __sa_sigaction:
            Option<unsafe extern "C" fn(libc::c_int, *mut __siginfo, *mut libc::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "286:9"]
    pub struct sigaction {
        pub __sigaction_u: __sigaction_u,
        pub sa_mask: sigset_t,
        pub sa_flags: libc::c_int,
    }
    use super::_pid_t_h::pid_t;
    use super::_sigset_t_h::sigset_t;
    use super::_uid_t_h::uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/asio.h:1"]
pub mod asio_h {
    #[c2rust::src_loc = "24:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "32:3"]
    pub const ASIO_DESTROYED: C2RustUnnamed = 4294967295;
    #[c2rust::src_loc = "31:3"]
    pub const ASIO_ONESHOT: C2RustUnnamed = 256;
    #[c2rust::src_loc = "30:3"]
    pub const ASIO_SIGNAL: C2RustUnnamed = 8;
    #[c2rust::src_loc = "29:3"]
    pub const ASIO_TIMER: C2RustUnnamed = 4;
    #[c2rust::src_loc = "28:3"]
    pub const ASIO_WRITE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "27:3"]
    pub const ASIO_READ: C2RustUnnamed = 1;
    #[c2rust::src_loc = "26:3"]
    pub const ASIO_DISPOSABLE: C2RustUnnamed = 0;
    use super::asio_backend_t;
    extern "C" {
        #[c2rust::src_loc = "113:1"]
        pub fn ponyint_asio_noisy_remove() -> u64;
        #[c2rust::src_loc = "109:1"]
        pub fn ponyint_asio_noisy_add() -> u64;
        #[c2rust::src_loc = "88:1"]
        pub fn ponyint_asio_get_cpu() -> u32;
        #[c2rust::src_loc = "71:1"]
        pub fn ponyint_asio_get_backend() -> *mut asio_backend_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/event.h:11"]
pub mod event_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed(4))]
    #[c2rust::src_loc = "86:8"]
    pub struct kevent {
        pub ident: uintptr_t,
        pub filter: i16,
        pub flags: u16,
        pub fflags: u32,
        pub data: intptr_t,
        pub udata: *mut libc::c_void,
    }
    use super::_intptr_t_h::intptr_t;
    use super::_timespec_h::timespec;
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "380:1"]
        pub fn kqueue() -> libc::c_int;
        #[c2rust::src_loc = "381:1"]
        pub fn kevent(
            kq: libc::c_int,
            changelist: *const kevent,
            nchanges: libc::c_int,
            eventlist: *mut kevent,
            nevents: libc::c_int,
            timeout: *const timespec,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/event.h:2"]
pub mod asio_event_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:16"]
    pub struct asio_event_t {
        pub magic: *mut asio_event_t,
        pub owner: *mut pony_actor_t,
        pub msg_id: u32,
        pub fd: libc::c_int,
        pub flags: u32,
        pub noisy: bool,
        pub readable: bool,
        pub writeable: bool,
        pub nsec: u64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:16"]
    pub struct asio_msg_t {
        pub msg: pony_msg_t,
        pub event: *mut asio_event_t,
        pub flags: u32,
        pub arg: u32,
    }
    use super::pony_h::{pony_actor_t, pony_msg_t};
    extern "C" {
        #[c2rust::src_loc = "58:1"]
        pub fn pony_asio_event_send(ev: *mut asio_event_t, flags: u32, arg: u32);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/signal.h:1"]
pub mod include_signal_h {
    use super::signal_h::sigaction;
    extern "C" {
        #[c2rust::src_loc = "84:1"]
        pub fn sigaction(_: libc::c_int, _: *const sigaction, _: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:5"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/scheduler.h:5"]
pub mod scheduler_h {
    extern "C" {
        #[c2rust::src_loc = "139:1"]
        pub fn ponyint_sched_noisy_asio(from: i32);
        #[c2rust::src_loc = "150:1"]
        pub fn ponyint_sched_maybe_wakeup_if_all_asleep(current_scheduler_id: i32);
        #[c2rust::src_loc = "143:1"]
        pub fn ponyint_sched_unnoisy_asio(from: i32);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:7"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_cpu_affinity(cpu: u32);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:10"]
pub mod ponyassert_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:13"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:15"]
pub mod unistd_h {
    use super::_ssize_t_h::ssize_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "470:1"]
        pub fn pipe(_: *mut libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "472:1"]
        pub fn read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
        #[c2rust::src_loc = "496:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    }
}
pub use self::_intptr_t_h::intptr_t;
pub use self::_pid_t_h::pid_t;
pub use self::_sigset_t_h::sigset_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_intptr_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint32_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::asio_event_h::{asio_event_t, asio_msg_t, pony_asio_event_send};
pub use self::asio_h::{
    ponyint_asio_get_backend, ponyint_asio_get_cpu, ponyint_asio_noisy_add,
    ponyint_asio_noisy_remove, C2RustUnnamed, ASIO_DESTROYED, ASIO_DISPOSABLE, ASIO_ONESHOT,
    ASIO_READ, ASIO_SIGNAL, ASIO_TIMER, ASIO_WRITE,
};
use self::cpu_h::ponyint_cpu_affinity;
pub use self::event_h::{kevent, kqueue};
use self::include_signal_h::sigaction;
pub use self::messageq_h::{
    messageq_t, ponyint_messageq_destroy, ponyint_messageq_init, ponyint_thread_messageq_pop,
    ponyint_thread_messageq_push,
};
pub use self::pony_h::{
    pony_actor_t, pony_alloc_msg, pony_msg_t, pony_register_thread, pony_unregister_thread,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::scheduler_h::{
    ponyint_sched_maybe_wakeup_if_all_asleep, ponyint_sched_noisy_asio, ponyint_sched_unnoisy_asio,
};
pub use self::signal_h::{__sigaction_u, __siginfo, sigaction, sigval};
pub use self::stddef_h::size_t;
use self::string_h::memset;
pub use self::sys__types_h::{__darwin_pid_t, __darwin_sigset_t, __darwin_uid_t};
use self::unistd_h::{close, pipe, read, write};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:8"]
pub struct asio_backend_t {
    pub kq: libc::c_int,
    pub wakeup: [libc::c_int; 2],
    pub q: messageq_t,
}
#[c2rust::src_loc = "22:1"]
pub type kevent_flag_t = u32;
#[inline]
#[c2rust::src_loc = "27:1"]
unsafe extern "C" fn macro__EV_SET(
    mut kevp: *mut kevent,
    mut ident: uintptr_t,
    mut filter: i16,
    mut flags: u16,
    mut fflags: u32,
    mut data: intptr_t,
    mut udata: *mut libc::c_void,
) {
    let mut __kevp__: *mut kevent = kevp;
    (*__kevp__).ident = ident;
    (*__kevp__).filter = filter;
    (*__kevp__).flags = flags;
    (*__kevp__).fflags = fflags;
    (*__kevp__).data = data;
    (*__kevp__).udata = udata;
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_asio_backend_init() -> *mut asio_backend_t {
    let mut b: *mut asio_backend_t =
        ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut asio_backend_t;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<asio_backend_t>() as libc::c_ulong,
    );
    ponyint_messageq_init(&mut (*b).q);
    (*b).kq = kqueue();
    if (*b).kq == -(1 as libc::c_int) {
        ponyint_pool_free(0 as libc::c_int as size_t, b as *mut libc::c_void);
        return 0 as *mut asio_backend_t;
    }
    pipe(((*b).wakeup).as_mut_ptr());
    let mut new_event: kevent = kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    };
    macro__EV_SET(
        &mut new_event,
        (*b).wakeup[0 as libc::c_int as usize] as uintptr_t,
        -(1 as libc::c_int) as i16,
        0x1 as libc::c_int as u16,
        0 as libc::c_int as u32,
        0 as libc::c_int as intptr_t,
        0 as *mut libc::c_void,
    );
    let mut t: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __darwin_time_t,
            tv_nsec: 0 as libc::c_int as libc::c_long,
        };
        init
    };
    kevent(
        (*b).kq,
        &mut new_event,
        1 as libc::c_int,
        0 as *mut kevent,
        0 as libc::c_int,
        &mut t,
    );
    return b;
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn ponyint_asio_backend_final(mut b: *mut asio_backend_t) {
    let mut c: libc::c_char = 1 as libc::c_int as libc::c_char;
    write(
        (*b).wakeup[1 as libc::c_int as usize],
        &mut c as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
#[c2rust::src_loc = "94:1"]
unsafe extern "C" fn handle_queue(mut b: *mut asio_backend_t) {
    let mut msg: *mut asio_msg_t = 0 as *mut asio_msg_t;
    loop {
        msg = ponyint_thread_messageq_pop(&mut (*b).q) as *mut asio_msg_t;
        if msg.is_null() {
            break;
        }
        pony_asio_event_send(
            (*msg).event,
            ASIO_DISPOSABLE as libc::c_int as u32,
            0 as libc::c_int as u32,
        );
    }
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn retry_loop(mut b: *mut asio_backend_t) {
    let mut c: libc::c_char = 0 as libc::c_int as libc::c_char;
    write(
        (*b).wakeup[1 as libc::c_int as usize],
        &mut c as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn pony_asio_event_resubscribe_read(mut ev: *mut asio_event_t) {
    if ev.is_null()
        || (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || (*ev).flags == ASIO_DESTROYED as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0"
                    as *const u8 as *const libc::c_char,
                121 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"pony_asio_event_resubscribe_read\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    let mut b: *mut asio_backend_t = ponyint_asio_get_backend();
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"pony_asio_event_resubscribe_read\0",
            ))
            .as_ptr(),
        );
    };
    let mut event: [kevent; 1] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 1];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut kqueue_flags: kevent_flag_t =
        (if (*ev).flags & ASIO_ONESHOT as libc::c_int as libc::c_uint != 0 {
            0x10 as libc::c_int
        } else {
            0x20 as libc::c_int
        }) as kevent_flag_t;
    if (*ev).flags & ASIO_READ as libc::c_int as libc::c_uint != 0 && !(*ev).readable {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(1 as libc::c_int) as i16,
            (0x1 as libc::c_int as libc::c_uint | kqueue_flags) as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    } else {
        return;
    }
    kevent(
        (*b).kq,
        event.as_mut_ptr(),
        i,
        0 as *mut kevent,
        0 as libc::c_int,
        0 as *const timespec,
    );
    if (*ev).fd == 0 as libc::c_int {
        retry_loop(b);
    }
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn pony_asio_event_resubscribe_write(mut ev: *mut asio_event_t) {
    if ev.is_null()
        || (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || (*ev).flags == ASIO_DESTROYED as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0"
                    as *const u8 as *const libc::c_char,
                152 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"pony_asio_event_resubscribe_write\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    let mut b: *mut asio_backend_t = ponyint_asio_get_backend();
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            157 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"pony_asio_event_resubscribe_write\0",
            ))
            .as_ptr(),
        );
    };
    let mut event: [kevent; 2] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut kqueue_flags: kevent_flag_t =
        (if (*ev).flags & ASIO_ONESHOT as libc::c_int as libc::c_uint != 0 {
            0x10 as libc::c_int
        } else {
            0x20 as libc::c_int
        }) as kevent_flag_t;
    if (*ev).flags & ASIO_WRITE as libc::c_int as libc::c_uint != 0 && !(*ev).writeable {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(2 as libc::c_int) as i16,
            (0x1 as libc::c_int as libc::c_uint | kqueue_flags) as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    } else {
        return;
    }
    kevent(
        (*b).kq,
        event.as_mut_ptr(),
        i,
        0 as *mut kevent,
        0 as libc::c_int,
        0 as *const timespec,
    );
    if (*ev).fd == 0 as libc::c_int {
        retry_loop(b);
    }
}
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn ponyint_asio_backend_dispatch(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    ponyint_cpu_affinity(ponyint_asio_get_cpu());
    pony_register_thread();
    let mut b: *mut asio_backend_t = arg as *mut asio_backend_t;
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            182 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"ponyint_asio_backend_dispatch\0",
            ))
            .as_ptr(),
        );
    };
    let mut fired: [kevent; 64] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 64];
    while (*b).kq != -(1 as libc::c_int) {
        let mut timeout: *mut timespec = 0 as *mut timespec;
        let mut count: libc::c_int = kevent(
            (*b).kq,
            0 as *const kevent,
            0 as libc::c_int,
            fired.as_mut_ptr(),
            64 as libc::c_int,
            timeout,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < count {
            let mut ep: *mut kevent = &mut *fired.as_mut_ptr().offset(i as isize) as *mut kevent;
            if (*ep).ident == (*b).wakeup[0 as libc::c_int as usize] as uintptr_t
                && (*ep).filter as libc::c_int == -(1 as libc::c_int)
            {
                let mut terminate: libc::c_char = 0;
                read(
                    (*b).wakeup[0 as libc::c_int as usize],
                    &mut terminate as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                );
                if terminate as libc::c_int == 1 as libc::c_int {
                    close((*b).kq);
                    close((*b).wakeup[0 as libc::c_int as usize]);
                    close((*b).wakeup[1 as libc::c_int as usize]);
                    (*b).kq = -(1 as libc::c_int);
                }
            } else {
                let mut ev: *mut asio_event_t = (*ep).udata as *mut asio_event_t;
                if !((*ep).flags as libc::c_int & 0x4000 as libc::c_int != 0) {
                    match (*ep).filter as libc::c_int {
                        -1 => {
                            if (*ev).flags & ASIO_READ as libc::c_int as libc::c_uint != 0 {
                                (*ev).readable = 1 as libc::c_int != 0;
                                pony_asio_event_send(
                                    ev,
                                    ASIO_READ as libc::c_int as u32,
                                    0 as libc::c_int as u32,
                                );
                            }
                        }
                        -2 => {
                            if (*ep).flags as libc::c_int & 0x8000 as libc::c_int != 0 {
                                if (*ev).flags
                                    & (ASIO_READ as libc::c_int | ASIO_WRITE as libc::c_int)
                                        as libc::c_uint
                                    != 0
                                {
                                    (*ev).readable = 1 as libc::c_int != 0;
                                    (*ev).writeable = 1 as libc::c_int != 0;
                                    pony_asio_event_send(
                                        ev,
                                        (ASIO_READ as libc::c_int | ASIO_WRITE as libc::c_int)
                                            as u32,
                                        0 as libc::c_int as u32,
                                    );
                                }
                            } else if (*ev).flags & ASIO_WRITE as libc::c_int as libc::c_uint != 0 {
                                (*ev).writeable = 1 as libc::c_int != 0;
                                pony_asio_event_send(
                                    ev,
                                    ASIO_WRITE as libc::c_int as u32,
                                    0 as libc::c_int as u32,
                                );
                            }
                        }
                        -7 => {
                            if (*ev).flags & ASIO_TIMER as libc::c_int as libc::c_uint != 0 {
                                pony_asio_event_send(
                                    ev,
                                    ASIO_TIMER as libc::c_int as u32,
                                    0 as libc::c_int as u32,
                                );
                            }
                        }
                        -6 => {
                            if (*ev).flags & ASIO_SIGNAL as libc::c_int as libc::c_uint != 0 {
                                pony_asio_event_send(
                                    ev,
                                    ASIO_SIGNAL as libc::c_int as u32,
                                    (*ep).data as u32,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            i += 1;
        }
        handle_queue(b);
    }
    ponyint_messageq_destroy(&mut (*b).q, 1 as libc::c_int != 0);
    ponyint_pool_free(0 as libc::c_int as size_t, b as *mut libc::c_void);
    pony_unregister_thread();
    return 0 as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "295:1"]
pub unsafe extern "C" fn pony_asio_event_subscribe(mut ev: *mut asio_event_t) {
    if ev.is_null()
        || (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || (*ev).flags == ASIO_DESTROYED as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0"
                    as *const u8 as *const libc::c_char,
                301 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"pony_asio_event_subscribe\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    let mut b: *mut asio_backend_t = ponyint_asio_get_backend();
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            306 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"pony_asio_event_subscribe\0",
            ))
            .as_ptr(),
        );
    };
    if (*ev).noisy {
        let mut old_count: u64 = ponyint_asio_noisy_add();
        if old_count == 0 as libc::c_int as libc::c_ulonglong {
            ponyint_sched_noisy_asio(-(10 as libc::c_int));
        }
    }
    let mut event: [kevent; 4] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut flags: kevent_flag_t =
        (if (*ev).flags & ASIO_ONESHOT as libc::c_int as libc::c_uint != 0 {
            0x10 as libc::c_int
        } else {
            0x20 as libc::c_int
        }) as kevent_flag_t;
    if (*ev).flags & ASIO_READ as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(1 as libc::c_int) as i16,
            (0x1 as libc::c_int as libc::c_uint | flags) as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_WRITE as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(2 as libc::c_int) as i16,
            (0x1 as libc::c_int as libc::c_uint | flags) as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_TIMER as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            ev as uintptr_t,
            -(7 as libc::c_int) as i16,
            (0x1 as libc::c_int | 0x10 as libc::c_int) as u16,
            0x4 as libc::c_int as u32,
            (*ev).nsec as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_SIGNAL as libc::c_int as libc::c_uint != 0 {
        let mut new_action: sigaction = sigaction {
            __sigaction_u: __sigaction_u { __sa_handler: None },
            sa_mask: 0,
            sa_flags: 0,
        };
        new_action.__sigaction_u.__sa_handler = ::core::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t);
        new_action.sa_mask = 0 as libc::c_int as sigset_t;
        new_action.sa_flags = 0x2 as libc::c_int;
        sigaction(
            (*ev).nsec as libc::c_int,
            &mut new_action,
            0 as *mut sigaction,
        );
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).nsec as uintptr_t,
            -(6 as libc::c_int) as i16,
            (0x1 as libc::c_int | 0x20 as libc::c_int) as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    kevent(
        (*b).kq,
        event.as_mut_ptr(),
        i,
        0 as *mut kevent,
        0 as libc::c_int,
        0 as *const timespec,
    );
    if (*ev).fd == 0 as libc::c_int {
        retry_loop(b);
    }
}
#[no_mangle]
#[c2rust::src_loc = "373:1"]
pub unsafe extern "C" fn pony_asio_event_setnsec(mut ev: *mut asio_event_t, mut nsec: u64) {
    if ev.is_null()
        || (*ev).magic != ev
        || (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || (*ev).flags == ASIO_DESTROYED as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0"
                    as *const u8 as *const libc::c_char,
                380 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"pony_asio_event_setnsec\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    let mut b: *mut asio_backend_t = ponyint_asio_get_backend();
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            385 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"pony_asio_event_setnsec\0",
            ))
            .as_ptr(),
        );
    };
    let mut event: [kevent; 1] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 1];
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*ev).flags & ASIO_TIMER as libc::c_int as libc::c_uint != 0 {
        (*ev).nsec = nsec;
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            ev as uintptr_t,
            -(7 as libc::c_int) as i16,
            (0x1 as libc::c_int | 0x10 as libc::c_int) as u16,
            0x4 as libc::c_int as u32,
            (*ev).nsec as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    kevent(
        (*b).kq,
        event.as_mut_ptr(),
        i,
        0 as *mut kevent,
        0 as libc::c_int,
        0 as *const timespec,
    );
}
#[no_mangle]
#[c2rust::src_loc = "407:1"]
pub unsafe extern "C" fn pony_asio_event_unsubscribe(mut ev: *mut asio_event_t) {
    if ev.is_null()
        || (*ev).magic != ev
        || (*ev).flags == ASIO_DISPOSABLE as libc::c_int as libc::c_uint
        || (*ev).flags == ASIO_DESTROYED as libc::c_uint
    {
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0"
                    as *const u8 as *const libc::c_char,
                414 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"pony_asio_event_unsubscribe\0",
                ))
                .as_ptr(),
            );
        };
        return;
    }
    let mut b: *mut asio_backend_t = ponyint_asio_get_backend();
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/asio/kqueue.c\0" as *const u8
                as *const libc::c_char,
            419 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"pony_asio_event_unsubscribe\0",
            ))
            .as_ptr(),
        );
    };
    if (*ev).noisy {
        let mut old_count: u64 = ponyint_asio_noisy_remove();
        if old_count == 1 as libc::c_int as libc::c_ulonglong {
            ponyint_sched_unnoisy_asio(-(10 as libc::c_int));
            ponyint_sched_maybe_wakeup_if_all_asleep(-(1 as libc::c_int));
        }
        (*ev).noisy = 0 as libc::c_int != 0;
    }
    let mut event: [kevent; 4] = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    }; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*ev).flags & ASIO_READ as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(1 as libc::c_int) as i16,
            0x2 as libc::c_int as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_WRITE as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).fd as uintptr_t,
            -(2 as libc::c_int) as i16,
            0x2 as libc::c_int as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_TIMER as libc::c_int as libc::c_uint != 0 {
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            ev as uintptr_t,
            -(7 as libc::c_int) as i16,
            0x2 as libc::c_int as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    if (*ev).flags & ASIO_SIGNAL as libc::c_int as libc::c_uint != 0 {
        let mut new_action: sigaction = sigaction {
            __sigaction_u: __sigaction_u { __sa_handler: None },
            sa_mask: 0,
            sa_flags: 0,
        };
        new_action.__sigaction_u.__sa_handler = None;
        new_action.sa_mask = 0 as libc::c_int as sigset_t;
        new_action.sa_flags = 0x2 as libc::c_int;
        sigaction(
            (*ev).nsec as libc::c_int,
            &mut new_action,
            0 as *mut sigaction,
        );
        macro__EV_SET(
            &mut *event.as_mut_ptr().offset(i as isize),
            (*ev).nsec as uintptr_t,
            -(6 as libc::c_int) as i16,
            0x2 as libc::c_int as u16,
            0 as libc::c_int as u32,
            0 as libc::c_int as intptr_t,
            ev as *mut libc::c_void,
        );
        i += 1;
    }
    kevent(
        (*b).kq,
        event.as_mut_ptr(),
        i,
        0 as *mut kevent,
        0 as libc::c_int,
        0 as *const timespec,
    );
    (*ev).flags = ASIO_DISPOSABLE as libc::c_int as u32;
    let mut msg: *mut asio_msg_t =
        pony_alloc_msg(0 as libc::c_int as u32, 0 as libc::c_int as u32)
            as *mut asio_msg_t;
    let ref mut fresh1 = (*msg).event;
    *fresh1 = ev;
    (*msg).flags = ASIO_DISPOSABLE as libc::c_int as u32;
    ponyint_thread_messageq_push(&mut (*b).q, msg as *mut pony_msg_t, msg as *mut pony_msg_t);
    retry_loop(b);
}
