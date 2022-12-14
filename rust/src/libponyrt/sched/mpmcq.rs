use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:3"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/mpmcq.h:3"]
pub mod mpmcq_h {
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
        pub counter: uintptr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:16"]
    pub struct mpmcq_t {
        pub head: *mut mpmcq_node_t,
        pub tail: aba_protected_mpmcq_node_t,
    }
    use super::_uintptr_t_h::uintptr_t;
    use super::mpmcq_node_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:3"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:5"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_cpu_relax();
    }
}
pub use self::_uintptr_t_h::uintptr_t;
use self::atomics_h::f__atomic_thread_fence;
use self::cpu_h::ponyint_cpu_relax;
pub use self::mpmcq_h::{aba_protected_mpmcq_node_t, mpmcq_t, C2RustUnnamed};
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stddef_h::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "13:8"]
pub struct mpmcq_node_t {
    pub next: *mut mpmcq_node_t,
    pub data: *mut libc::c_void,
}
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn node_alloc(mut data: *mut libc::c_void) -> *mut mpmcq_node_t {
    let mut node: *mut mpmcq_node_t =
        ponyint_pool_alloc(0 as libc::c_int as usize) as *mut mpmcq_node_t;
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*node).next, 0 as *mut mpmcq_node_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*node).data, data);
        // compile_error!("Builtin is not supposed to be used")
    });
    node
}
#[c2rust::src_loc = "28:1"]
unsafe extern "C" fn node_free(mut node: *mut mpmcq_node_t) {
    ponyint_pool_free(0 as libc::c_int as usize, node as *mut libc::c_void);
}
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn mpmcq_size_debug(mut q: *mut mpmcq_t) -> usize {
    let mut count: usize = 0;
    let mut tail: *mut mpmcq_node_t = (*q).tail.c2rust_unnamed.object;
    while !({
        ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next as *mut *mut mpmcq_node_t)
    })
    .is_null()
    {
        count = count.wrapping_add(1);
        tail = ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next);
    }
    count
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn ponyint_mpmcq_init(mut q: *mut mpmcq_t) {
    let mut node: *mut mpmcq_node_t = node_alloc(0 as *mut libc::c_void);
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*q).head, node);
        // compile_error!("Builtin is not supposed to be used")
    });
    let ref mut fresh0 = (*q).tail.c2rust_unnamed.object;
    *fresh0 = node;
    (*q).tail.c2rust_unnamed.counter = 0 as libc::c_int as uintptr_t;
    mpmcq_size_debug(q);
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn ponyint_mpmcq_destroy(mut q: *mut mpmcq_t) {
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*q).head, 0 as *mut mpmcq_node_t);
        // compile_error!("Builtin is not supposed to be used")
    });
    node_free((*q).tail.c2rust_unnamed.object);
    let ref mut fresh1 = (*q).tail.c2rust_unnamed.object;
    *fresh1 = 0 as *mut mpmcq_node_t;
}
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn ponyint_mpmcq_push(mut q: *mut mpmcq_t, mut data: *mut libc::c_void) {
    let mut node: *mut mpmcq_node_t = node_alloc(data);
    f__atomic_thread_fence(b"memory_order_release\0" as *const u8 as *const libc::c_char);
    let mut prev: *mut mpmcq_node_t =
        { ::core::intrinsics::atomic_xchg_relaxed(&mut (*q).head, node) };
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*prev).next, node);
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn ponyint_mpmcq_push_single(
    mut q: *mut mpmcq_t,
    mut data: *mut libc::c_void,
) {
    let mut node: *mut mpmcq_node_t = node_alloc(data);
    let mut prev: *mut mpmcq_node_t = { ::core::intrinsics::atomic_load_relaxed(&mut (*q).head) };
    ({
        ::core::intrinsics::atomic_store_relaxed(&mut (*q).head, node);
        // compile_error!("Builtin is not supposed to be used")
    });
    ({
        ::core::intrinsics::atomic_store_rel(&mut (*prev).next, node);
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn ponyint_mpmcq_pop(mut q: *mut mpmcq_t) -> *mut libc::c_void {
    let mut cmp: aba_protected_mpmcq_node_t = aba_protected_mpmcq_node_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *mut mpmcq_node_t,
            counter: 0,
        },
    };
    let mut xchg: aba_protected_mpmcq_node_t = aba_protected_mpmcq_node_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *mut mpmcq_node_t,
            counter: 0,
        },
    };
    let mut tail: *mut mpmcq_node_t = 0 as *mut mpmcq_node_t;
    cmp.c2rust_unnamed.object = (*q).tail.c2rust_unnamed.object;
    cmp.c2rust_unnamed.counter = (*q).tail.c2rust_unnamed.counter;
    let mut next: *mut mpmcq_node_t = 0 as *mut mpmcq_node_t;
    loop {
        tail = cmp.c2rust_unnamed.object;
        next = ::core::intrinsics::atomic_load_relaxed(&mut (*tail).next);
        if next.is_null() {
            return 0 as *mut libc::c_void;
        }
        xchg.c2rust_unnamed.object = next;
        xchg.c2rust_unnamed.counter =
            (cmp.c2rust_unnamed.counter).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if {
            let fresh2 = ::core::intrinsics::atomic_cxchg_acqrel(
                &mut (*q).tail.raw as *mut i128,
                *(&mut cmp.raw as *mut i128),
                xchg.raw,
            );
            *(&mut cmp.raw as *mut i128) = fresh2.0;
            fresh2.1
        } {
            break;
        }
    }
    f__atomic_thread_fence(b"memory_order_acq_rel\0" as *const u8 as *const libc::c_char);
    let mut data: *mut libc::c_void = { ::core::intrinsics::atomic_load_acq(&mut (*next).data) };
    ({
        ::core::intrinsics::atomic_store_rel(&mut (*next).data, 0 as *mut libc::c_void);
        // compile_error!("Builtin is not supposed to be used")
    });
    while !({ ::core::intrinsics::atomic_load_acq(&mut (*tail).data as *mut *mut libc::c_void) })
        .is_null()
    {
        ponyint_cpu_relax();
    }
    f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
    node_free(tail);
    data
}
