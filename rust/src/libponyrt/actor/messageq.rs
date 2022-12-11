use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:3"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:3"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:3"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: uint32_t,
        pub id: uint32_t,
        pub next: *mut pony_msg_t,
    }
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:3"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:5"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/dtrace.h:7"]
pub mod dtrace_h {
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn macro__DTRACE(_: *const libc::c_char, _: libc::c_int, _: ...) -> bool;
    }
}
pub use self::_uint32_t_h::uint32_t;
pub use self::_uintptr_t_h::uintptr_t;
use self::atomics_h::f__atomic_thread_fence;
use self::dtrace_h::macro__DTRACE;
pub use self::messageq_h::messageq_t;
pub use self::pony_h::pony_msg_t;
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stddef_h::size_t;
#[c2rust::src_loc = "15:1"]
unsafe extern "C" fn messageq_size_debug(mut q: *mut messageq_t) -> size_t {
    let mut tail: *mut pony_msg_t = (*q).tail;
    let mut count: size_t = 0 as libc::c_int as size_t;
    while !({ ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next as *mut *mut pony_msg_t) })
        .is_null()
    {
        count = count.wrapping_add(1);
        tail = ({ ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next) });
    }
    return count;
}
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn messageq_push(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*last).next, 0 as *mut pony_msg_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    f__atomic_thread_fence(b"memory_order_release\0" as *const u8 as *const libc::c_char);
    let mut prev: *mut pony_msg_t =
        ({ ::core::intrinsics::atomic_xchg_relaxed(&mut (*q).head, last) });
    let mut was_empty: bool =
        prev as uintptr_t & 1 as libc::c_int as libc::c_ulong != 0 as libc::c_int as libc::c_ulong;
    prev = (prev as uintptr_t & !(1 as libc::c_int as uintptr_t)) as *mut pony_msg_t;
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*prev).next, first);
        // compile_error!("Builtin is not supposed to be used")
    });
    return was_empty;
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn messageq_push_single(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*last).next, 0 as *mut pony_msg_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    let mut prev: *mut pony_msg_t = ({ ::core::intrinsics::atomic_load_relaxed(&mut (*q).head) });
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*q).head, last);
        // compile_error!("Builtin is not supposed to be used")
    });
    let mut was_empty: bool =
        prev as uintptr_t & 1 as libc::c_int as libc::c_ulong != 0 as libc::c_int as libc::c_ulong;
    prev = (prev as uintptr_t & !(1 as libc::c_int as uintptr_t)) as *mut pony_msg_t;
    ({
        ::core::intrinsics::atomic_store_rel(&mut (*prev).next, first);
        // compile_error!("Builtin is not supposed to be used")
    });
    return was_empty;
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn ponyint_messageq_init(mut q: *mut messageq_t) {
    let mut stub: *mut pony_msg_t =
        ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut pony_msg_t;
    (*stub).index = 0 as libc::c_int as uint32_t;
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*stub).next, 0 as *mut pony_msg_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    ({
        ::core::intrinsics::atomic_store_relaxed(
            &mut (*q).head,
            (stub as uintptr_t | 1 as libc::c_int as libc::c_ulong) as *mut pony_msg_t,
        );
        // compile_error!("Builtin is not supposed to be used")
    });
    let ref mut fresh0 = (*q).tail;
    *fresh0 = stub;
    messageq_size_debug(q);
}
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn ponyint_messageq_destroy(
    mut q: *mut messageq_t,
    mut maybe_non_empty: bool,
) {
    let mut tail: *mut pony_msg_t = (*q).tail;
    if maybe_non_empty as libc::c_int != 0
        || ({ ::core::intrinsics::atomic_load_acq(&mut (*q).head as *mut *mut pony_msg_t) })
            as uintptr_t
            & !(1 as libc::c_int as uintptr_t)
            == tail as uintptr_t
    {
    } else {
        ponyint_assert_fail(
            b"maybe_non_empty || ( (((uintptr_t)atomic_load_explicit(&q->head, memory_order_acquire) & ~(uintptr_t)1)) == (uintptr_t)tail)\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/actor/messageq.c\0"
                as *const u8 as *const libc::c_char,
            108 as libc::c_int as size_t,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"ponyint_messageq_destroy\0"))
                .as_ptr(),
        );
    };
    ponyint_pool_free((*tail).index as size_t, tail as *mut libc::c_void);
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*q).head, 0 as *mut pony_msg_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    let ref mut fresh1 = (*q).tail;
    *fresh1 = 0 as *mut pony_msg_t;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn ponyint_actor_messageq_push(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    return messageq_push(q, first, last);
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn ponyint_thread_messageq_push(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    return messageq_push(q, first, last);
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn ponyint_actor_messageq_push_single(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    return messageq_push_single(q, first, last);
}
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn ponyint_thread_messageq_push_single(
    mut q: *mut messageq_t,
    mut first: *mut pony_msg_t,
    mut last: *mut pony_msg_t,
) -> bool {
    return messageq_push_single(q, first, last);
}
#[no_mangle]
#[c2rust::src_loc = "236:1"]
pub unsafe extern "C" fn ponyint_actor_messageq_pop(mut q: *mut messageq_t) -> *mut pony_msg_t {
    let mut tail: *mut pony_msg_t = (*q).tail;
    let mut next: *mut pony_msg_t = ({ ::core::intrinsics::atomic_load_acq(&mut (*tail).next) });
    if !next.is_null() {
        macro__DTRACE(
            b"ACTOR_MSG_POP\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int,
            b"sched->index\0" as *const u8 as *const libc::c_char,
            b"(uint32_t) next->id\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t) actor\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh2 = (*q).tail;
        *fresh2 = next;
        f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
        ponyint_pool_free((*tail).index as size_t, tail as *mut libc::c_void);
    }
    return next;
}
#[no_mangle]
#[c2rust::src_loc = "260:1"]
pub unsafe extern "C" fn ponyint_thread_messageq_pop(mut q: *mut messageq_t) -> *mut pony_msg_t {
    let mut tail: *mut pony_msg_t = (*q).tail;
    let mut next: *mut pony_msg_t =
        ({ ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next) });
    if !next.is_null() {
        macro__DTRACE(
            b"THREAD_MSG_POP\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            b"(uint32_t) next->id\0" as *const u8 as *const libc::c_char,
            b"(uintptr_t) thr\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh3 = (*q).tail;
        *fresh3 = next;
        f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
        ponyint_pool_free((*tail).index as size_t, tail as *mut libc::c_void);
    }
    return next;
}
#[no_mangle]
#[c2rust::src_loc = "284:1"]
pub unsafe extern "C" fn ponyint_messageq_markempty(mut q: *mut messageq_t) -> bool {
    let mut tail: *mut pony_msg_t = (*q).tail;
    let mut head: *mut pony_msg_t = ({ ::core::intrinsics::atomic_load_acq(&mut (*q).head) });
    if head as uintptr_t & 1 as libc::c_int as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if head != tail {
        return 0 as libc::c_int != 0;
    }
    head = (head as uintptr_t | 1 as libc::c_int as libc::c_ulong) as *mut pony_msg_t;
    return ({
        let fresh4 = ::core::intrinsics::atomic_cxchg_acqrel(&mut (*q).head, *&mut tail, head);
        *&mut tail = fresh4.0;
        fresh4.1
    });
}
#[no_mangle]
#[c2rust::src_loc = "304:1"]
pub unsafe extern "C" fn ponyint_messageq_isempty(mut q: *mut messageq_t) -> bool {
    let mut tail: *mut pony_msg_t = (*q).tail;
    let mut head: *mut pony_msg_t = ({ ::core::intrinsics::atomic_load_acq(&mut (*q).head) });
    if head as uintptr_t & 1 as libc::c_int as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    return head == tail;
}
