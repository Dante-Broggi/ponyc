use ::libc;
use core::mem::ManuallyDrop;
use core::sync::atomic::{
    AtomicBool, AtomicPtr,
    Ordering::{Acquire, Relaxed, Release},
    AtomicUsize,
};
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __int128_t = i128;
}
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:3"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:3"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/platform.h:3"]
pub mod platform_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn __pony_clzll(mut x: uint64_t) -> uint32_t {
        x.leading_zeros() as i32 as uint32_t
    }
    #[inline]
    #[c2rust::src_loc = "327:1"]
    pub unsafe extern "C" fn __pony_clzzu(mut x: size_t) -> uint32_t {
        __pony_clzll(x as uint64_t)
    }
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/alloc.h:4"]
pub mod alloc_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn ponyint_virt_alloc(bytes: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:6"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/sched/cpu.h:6"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_cpu_relax();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:7"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:12"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uintptr_t_h::uintptr_t;
use self::alloc_h::ponyint_virt_alloc;
use self::atomics_h::f__atomic_thread_fence;
use self::cpu_h::ponyint_cpu_relax;
pub use self::internal::__int128_t;
pub use self::platform_h::{__pony_clzll, __pony_clzzu};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::stddef_h::size_t;
use self::string_h::memcpy;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:16"]
pub struct pool_local_t {
    pub pool: *mut pool_item_t,
    pub length: size_t,
    pub start: *mut libc::c_char,
    pub end: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "39:16"]
pub struct pool_item_t {
    pub next: *mut pool_item_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "64:16"]
pub struct pool_global_t {
    pub size: size_t,
    pub count: size_t,
    pub central: aba_protected_pool_central_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:1"]
pub union aba_protected_pool_central_t {
    pub c2rust_unnamed: C2RustUnnamed,
    pub raw: __int128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:1"]
pub struct C2RustUnnamed {
    pub object: *mut pool_central_t,
    pub counter: uintptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "54:16"]
pub struct pool_central_t {
    pub next: *mut pool_item_t,
    pub length: uintptr_t,
    pub central: *mut pool_central_t,
}
#[repr(C)]
#[c2rust::src_loc = "72:16"]
pub struct pool_block_t {
    pub prev: *mut pool_block_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub size: size_t,
    pub acquired: AtomicBool,
}
#[repr(C)]
#[c2rust::src_loc = "75:3"]
pub union C2RustUnnamed_0 {
    pub next: *mut pool_block_t,
    pub global: ManuallyDrop<AtomicPtr<pool_block_t>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "89:16"]
pub struct pool_block_header_t {
    pub head: *mut pool_block_t,
    pub total_size: size_t,
    pub largest_size: size_t,
}
#[c2rust::src_loc = "98:22"]
static mut pool_global: [pool_global_t; 16] = [
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 0 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 0 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 1 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 1 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 2 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 2 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 3 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 3 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 4 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 4 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 5 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 5 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0_i32 as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 6 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 6 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 7 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 7 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 8 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 8 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 9 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 9 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 10 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 10 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 11 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 11 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 12 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 12 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 13 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 13 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 14 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 14 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
    {
        let mut init = pool_global_t {
            size: (((1 as libc::c_int) << 5 as libc::c_int) << 15 as libc::c_int) as size_t,
            count: (((1 as libc::c_int) << 20 as libc::c_int)
                / (((1 as libc::c_int) << 5 as libc::c_int) << 15 as libc::c_int))
                as size_t,
            central: aba_protected_pool_central_t {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed {
                        object: 0 as *const pool_central_t as *mut pool_central_t,
                        counter: 0 as libc::c_int as uintptr_t,
                    };
                    init
                },
            },
        };
        init
    },
];
#[c2rust::src_loc = "118:21"]
static mut pool_block_global: pool_block_t = pool_block_t {
    prev: 0 as *const pool_block_t as *mut pool_block_t,
    c2rust_unnamed: C2RustUnnamed_0 {
        next: 0 as *const pool_block_t as *mut pool_block_t,
    },
    size: 0,
    acquired: AtomicBool::new(false),
};
#[c2rust::src_loc = "119:28"]
static mut in_pool_block_global: AtomicUsize = AtomicUsize::new(0);
#[thread_local]
#[c2rust::src_loc = "121:41"]
static mut pool_local: [pool_local_t; 16] = [pool_local_t {
    pool: 0 as *const pool_item_t as *mut pool_item_t,
    length: 0,
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
}; 16];
#[thread_local]
#[c2rust::src_loc = "122:48"]
static mut pool_block_header: pool_block_header_t = pool_block_header_t {
    head: 0 as *const pool_block_t as *mut pool_block_t,
    total_size: 0,
    largest_size: 0,
};
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn pool_block_remove(mut block: *mut pool_block_t) {
    let mut prev: *mut pool_block_t = (*block).prev;
    let mut next: *mut pool_block_t = (*block).c2rust_unnamed.next;
    if !prev.is_null() {
        let ref mut fresh0 = (*prev).c2rust_unnamed.next;
        *fresh0 = next;
    } else {
        pool_block_header.head = next;
    }
    if !next.is_null() {
        let ref mut fresh1 = (*next).prev;
        *fresh1 = prev;
    }
}
#[c2rust::src_loc = "382:1"]
unsafe extern "C" fn pool_block_insert(mut block: *mut pool_block_t) {
    let mut next: *mut pool_block_t = pool_block_header.head;
    let mut prev: *mut pool_block_t = 0 as *mut pool_block_t;
    while !next.is_null() {
        if (*block).size <= (*next).size {
            break;
        }
        prev = next;
        next = (*next).c2rust_unnamed.next;
    }
    let ref mut fresh2 = (*block).prev;
    *fresh2 = prev;
    let ref mut fresh3 = (*block).c2rust_unnamed.next;
    *fresh3 = next;
    if !prev.is_null() {
        let ref mut fresh4 = (*prev).c2rust_unnamed.next;
        *fresh4 = block;
    } else {
        pool_block_header.head = block;
    }
    if !next.is_null() {
        let ref mut fresh5 = (*next).prev;
        *fresh5 = block;
    }
}
#[c2rust::src_loc = "408:1"]
unsafe extern "C" fn pool_block_push(mut block: *mut pool_block_t) {
    &mut (*block).acquired.store(false, Relaxed);
    in_pool_block_global.fetch_add(1, Acquire);
    loop {
        let mut pos: *mut pool_block_t = (&mut *pool_block_global.c2rust_unnamed.global).load(Acquire);
        let mut prev: *mut pool_block_t = &mut pool_block_global;
        while !pos.is_null() && (*block).size > (*pos).size {
            prev = pos;
            pos = (&mut *(*pos).c2rust_unnamed.global).load(Acquire);
        }
        if (*prev).acquired.swap(true, Acquire) {
            continue;
        }
        let mut check_pos: *mut pool_block_t = (&mut (*prev).c2rust_unnamed.global).load(Relaxed);
        if pos != check_pos {
            &mut (*prev).acquired.store(false, Relaxed);
        } else {
            (&mut (*block).c2rust_unnamed.global).store(pos, Relaxed);
            (&mut (*prev).c2rust_unnamed.global).store(block, Release);
            (&mut (*prev).acquired).store(false, Release);
            break;
        }
    }
    in_pool_block_global.fetch_sub(1, Release);
}
#[c2rust::src_loc = "471:1"]
unsafe extern "C" fn pool_block_pull(mut size: size_t) -> *mut pool_block_t {
    let mut block: *mut pool_block_t = (&mut pool_block_global.c2rust_unnamed.global).load(Relaxed);
    if block.is_null() {
        return 0 as *mut pool_block_t;
    }
    in_pool_block_global.fetch_add(1, Acquire);
    loop {
        block = (&mut pool_block_global.c2rust_unnamed.global).load(Relaxed);
        if block.is_null() {
            in_pool_block_global.fetch_sub(1, Relaxed);
            return 0 as *mut pool_block_t;
        }
        f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
        let mut prev: *mut pool_block_t = &mut pool_block_global;
        while !block.is_null() && size > (*block).size {
            prev = block;
            block = (&mut (*block).c2rust_unnamed.global).load(Acquire);
        }
        if block.is_null() {
            in_pool_block_global.fetch_sub(1, Relaxed);
            return 0 as *mut pool_block_t;
        }
        if (*prev).acquired.swap(true, Acquire) {
            continue;
        }
        let mut check_block: *mut pool_block_t = (&mut (*prev).c2rust_unnamed.global).load(Relaxed);
        if block != check_block || (*block).acquired.swap(true, Relaxed) {
            (*prev).acquired.store(false, Relaxed);
        } else {
            f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
            let mut next: *mut pool_block_t = (&mut (*block).c2rust_unnamed.global).load(Relaxed);
            (&mut (*prev).c2rust_unnamed.global).store(next, Relaxed);
            (&mut (*prev).acquired).store(false, Release);
            break;
        }
    }
    in_pool_block_global.fetch_sub(1, Release);
    while in_pool_block_global.load(Relaxed) != 0 {
        ponyint_cpu_relax();
    }
    f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
    if size <= (*block).size {
    } else {
        ponyint_assert_fail(
            b"size <= block->size\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                as *const libc::c_char,
            570 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"pool_block_pull\0"))
                .as_ptr(),
        );
    };
    block
}
#[c2rust::src_loc = "580:1"]
unsafe extern "C" fn pool_block_get(mut size: size_t) -> *mut libc::c_void {
    if pool_block_header.largest_size >= size {
        let mut block: *mut pool_block_t = pool_block_header.head;
        while !block.is_null() {
            if (*block).size > size {
                let mut rem: size_t = ((*block).size).wrapping_sub(size);
                (*block).size = rem;
                pool_block_header.total_size = (pool_block_header.total_size as libc::c_ulong)
                    .wrapping_sub(size) as size_t
                    as size_t;
                if !((*block).prev).is_null() && (*(*block).prev).size > (*block).size {
                    if ((*block).c2rust_unnamed.next).is_null() {
                        pool_block_header.largest_size = (*(*block).prev).size;
                    }
                    pool_block_remove(block);
                    pool_block_insert(block);
                } else if ((*block).c2rust_unnamed.next).is_null() {
                    pool_block_header.largest_size = rem;
                }
                return (block as *mut libc::c_char).offset(rem as isize) as *mut libc::c_void;
            } else {
                if (*block).size == size {
                    if ((*block).c2rust_unnamed.next).is_null() {
                        pool_block_header.largest_size = if ((*block).prev).is_null() {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*(*block).prev).size
                        };
                    }
                    pool_block_remove(block);
                    pool_block_header.total_size = (pool_block_header.total_size as libc::c_ulong)
                        .wrapping_sub(size)
                        as size_t as size_t;
                    return block as *mut libc::c_void;
                }
            }
            block = (*block).c2rust_unnamed.next;
        }
        if 0 as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"false\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                    as *const libc::c_char,
                629 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pool_block_get\0"))
                    .as_ptr(),
            );
        };
    }
    let mut block_0: *mut pool_block_t = pool_block_pull(size);
    if block_0.is_null() {
        return 0 as *mut libc::c_void;
    }
    if size == (*block_0).size {
        return block_0 as *mut libc::c_void;
    }
    let mut rem_0: size_t = ((*block_0).size).wrapping_sub(size);
    (*block_0).size = rem_0;
    pool_block_insert(block_0);
    pool_block_header.total_size =
        (pool_block_header.total_size as libc::c_ulong).wrapping_add(rem_0) as size_t as size_t;
    if pool_block_header.largest_size < rem_0 {
        pool_block_header.largest_size = rem_0;
    }
    (block_0 as *mut libc::c_char).offset(rem_0 as isize) as *mut libc::c_void
}
#[c2rust::src_loc = "651:1"]
unsafe extern "C" fn pool_alloc_pages(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = pool_block_get(size);
    if !p.is_null() {
        return p;
    }
    if size >= (128 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        return ponyint_virt_alloc(size);
    }
    let mut block: *mut pool_block_t = ponyint_virt_alloc(
        (128 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as size_t,
    ) as *mut pool_block_t;
    let mut rem: size_t = ((128 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
        as libc::c_ulong)
        .wrapping_sub(size);
    (*block).size = rem;
    let ref mut fresh6 = (*block).c2rust_unnamed.next;
    *fresh6 = 0 as *mut pool_block_t;
    let ref mut fresh7 = (*block).prev;
    *fresh7 = 0 as *mut pool_block_t;
    pool_block_insert(block);
    pool_block_header.total_size =
        (pool_block_header.total_size as libc::c_ulong).wrapping_add(rem) as size_t as size_t;
    if pool_block_header.largest_size < rem {
        pool_block_header.largest_size = rem;
    }
    (block as *mut libc::c_char).offset(rem as isize) as *mut libc::c_void
}
#[c2rust::src_loc = "676:1"]
unsafe extern "C" fn todo() {}
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn pool_free_pages(mut p: *mut libc::c_void, mut size: size_t) {
    if pool_block_header.total_size
        >= (128 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    {
        todo();
    }
    let mut block: *mut pool_block_t = p as *mut pool_block_t;
    let ref mut fresh8 = (*block).prev;
    *fresh8 = 0 as *mut pool_block_t;
    let ref mut fresh9 = (*block).c2rust_unnamed.next;
    *fresh9 = 0 as *mut pool_block_t;
    (*block).size = size;
    pool_block_insert(block);
    pool_block_header.total_size =
        (pool_block_header.total_size as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    if pool_block_header.largest_size < size {
        pool_block_header.largest_size = size;
    }
}
#[c2rust::src_loc = "697:1"]
unsafe extern "C" fn pool_push(mut thread: *mut pool_local_t, mut global: *mut pool_global_t) {
    let mut p: *mut pool_central_t = (*thread).pool as *mut pool_central_t;
    (*p).length = (*thread).length;
    let ref mut fresh10 = (*thread).pool;
    *fresh10 = 0 as *mut pool_item_t;
    (*thread).length = 0 as libc::c_int as size_t;
    if (*p).length > 0 as libc::c_int as libc::c_ulong && (*p).length <= (*global).count {
    } else {
        ponyint_assert_fail(
            b"(p->length > 0) && (p->length <= global->count)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                as *const libc::c_char,
            705 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pool_push\0")).as_ptr(),
        );
    };
    let mut top: *mut pool_central_t = 0 as *mut pool_central_t;
    let mut cmp: aba_protected_pool_central_t = aba_protected_pool_central_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *const pool_central_t as *mut pool_central_t,
            counter: 0,
        },
    };
    let mut xchg: aba_protected_pool_central_t = aba_protected_pool_central_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *const pool_central_t as *mut pool_central_t,
            counter: 0,
        },
    };
    cmp.c2rust_unnamed.object = (*global).central.c2rust_unnamed.object;
    cmp.c2rust_unnamed.counter = (*global).central.c2rust_unnamed.counter;
    xchg.c2rust_unnamed.object = p;
    loop {
        top = cmp.c2rust_unnamed.object;
        xchg.c2rust_unnamed.counter =
            (cmp.c2rust_unnamed.counter).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let ref mut fresh11 = (*p).central;
        *fresh11 = top;
        if {
            let fresh12 = ::core::intrinsics::atomic_cxchg_acqrel(
                &mut (*global).central.raw as *mut __int128_t,
                *(&mut cmp.raw as *mut __int128_t),
                xchg.raw,
            );
            *(&mut cmp.raw as *mut __int128_t) = fresh12.0;
            fresh12.1
        } {
            break;
        }
    }
}
#[c2rust::src_loc = "730:1"]
unsafe extern "C" fn pool_pull(
    mut thread: *mut pool_local_t,
    mut global: *mut pool_global_t,
) -> *mut pool_item_t {
    let mut top: *mut pool_central_t = 0 as *mut pool_central_t;
    let mut cmp: aba_protected_pool_central_t = aba_protected_pool_central_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *const pool_central_t as *mut pool_central_t,
            counter: 0,
        },
    };
    let mut xchg: aba_protected_pool_central_t = aba_protected_pool_central_t {
        c2rust_unnamed: C2RustUnnamed {
            object: 0 as *const pool_central_t as *mut pool_central_t,
            counter: 0,
        },
    };
    cmp.c2rust_unnamed.object = (*global).central.c2rust_unnamed.object;
    cmp.c2rust_unnamed.counter = (*global).central.c2rust_unnamed.counter;
    let mut next: *mut pool_central_t = 0 as *mut pool_central_t;
    loop {
        top = cmp.c2rust_unnamed.object;
        if top.is_null() {
            return 0 as *mut pool_item_t;
        }
        f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
        next = (*top).central;
        xchg.c2rust_unnamed.object = next;
        xchg.c2rust_unnamed.counter =
            (cmp.c2rust_unnamed.counter).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if {
            let fresh13 = ::core::intrinsics::atomic_cxchg_acqrel(
                &mut (*global).central.raw as *mut __int128_t,
                *(&mut cmp.raw as *mut __int128_t),
                xchg.raw,
            );
            *(&mut cmp.raw as *mut __int128_t) = fresh13.0;
            fresh13.1
        } {
            break;
        }
    }
    let mut p: *mut pool_item_t = top as *mut pool_item_t;
    if (*top).length > 0 as libc::c_int as libc::c_ulong && (*top).length <= (*global).count {
    } else {
        ponyint_assert_fail(
            b"(top->length > 0) && (top->length <= global->count)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                as *const libc::c_char,
            764 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pool_pull\0")).as_ptr(),
        );
    };
    let ref mut fresh14 = (*thread).pool;
    *fresh14 = (*p).next;
    (*thread).length = ((*top).length).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    return p;
}
#[c2rust::src_loc = "773:1"]
unsafe extern "C" fn pool_get(mut pool: *mut pool_local_t, mut index: size_t) -> *mut libc::c_void {
    let mut thread: *mut pool_local_t = &mut *pool.offset(index as isize) as *mut pool_local_t;
    let mut global: *mut pool_global_t =
        &mut *pool_global.as_mut_ptr().offset(index as isize) as *mut pool_global_t;
    let mut p: *mut pool_item_t = (*thread).pool;
    if !p.is_null() {
        let ref mut fresh15 = (*thread).pool;
        *fresh15 = (*p).next;
        let ref mut fresh16 = (*thread).length;
        *fresh16 = (*fresh16).wrapping_sub(1);
        return p as *mut libc::c_void;
    }
    p = pool_pull(thread, global);
    if !p.is_null() {
        return p as *mut libc::c_void;
    }
    if (*global).size < ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong {
        if (*thread).start < (*thread).end {
            let mut p_0: *mut libc::c_void = (*thread).start as *mut libc::c_void;
            let ref mut fresh17 = (*thread).start;
            *fresh17 = (*fresh17).offset((*global).size as isize);
            return p_0;
        }
        let mut mem: *mut libc::c_char =
            pool_get(pool, (10 as libc::c_int - 5 as libc::c_int) as size_t) as *mut libc::c_char;
        let ref mut fresh18 = (*thread).start;
        *fresh18 = mem.offset((*global).size as isize);
        let ref mut fresh19 = (*thread).end;
        *fresh19 = mem.offset(((1 as libc::c_int) << 10 as libc::c_int) as isize);
        return mem as *mut libc::c_void;
    }
    pool_alloc_pages((*global).size)
}
#[no_mangle]
#[c2rust::src_loc = "819:1"]
pub unsafe extern "C" fn ponyint_pool_alloc(mut index: size_t) -> *mut libc::c_void {
    let mut pool: *mut pool_local_t = pool_local.as_mut_ptr();
    let mut p: *mut libc::c_void = pool_get(pool, index);
    p
}
#[no_mangle]
#[c2rust::src_loc = "838:1"]
pub unsafe extern "C" fn ponyint_pool_free(mut index: size_t, mut p: *mut libc::c_void) {
    if index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"index < POOL_COUNT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                as *const libc::c_char,
            845 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_pool_free\0"))
                .as_ptr(),
        );
    };
    let mut thread: *mut pool_local_t =
        &mut *pool_local.as_mut_ptr().offset(index as isize) as *mut pool_local_t;
    let mut global: *mut pool_global_t =
        &mut *pool_global.as_mut_ptr().offset(index as isize) as *mut pool_global_t;
    if (*thread).length == (*global).count {
        pool_push(thread, global);
    }
    if (*thread).length < (*global).count {
    } else {
        ponyint_assert_fail(
            b"thread->length < global->count\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.c\0" as *const u8
                as *const libc::c_char,
            854 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_pool_free\0"))
                .as_ptr(),
        );
    };
    let mut lp: *mut pool_item_t = p as *mut pool_item_t;
    let ref mut fresh20 = (*lp).next;
    *fresh20 = (*thread).pool;
    let ref mut fresh21 = (*thread).pool;
    *fresh21 = lp;
    let ref mut fresh22 = (*thread).length;
    *fresh22 = (*fresh22).wrapping_add(1);
}
#[c2rust::src_loc = "866:1"]
unsafe extern "C" fn pool_alloc_size(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = pool_alloc_pages(size);
    p
}
#[no_mangle]
#[c2rust::src_loc = "884:1"]
pub unsafe extern "C" fn ponyint_pool_alloc_size(mut size: size_t) -> *mut libc::c_void {
    let mut index: size_t = ponyint_pool_index(size);
    if index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        return ponyint_pool_alloc(index);
    }
    size = ponyint_pool_adjust_size(size);
    let mut p: *mut libc::c_void = pool_alloc_size(size);
    p
}
#[c2rust::src_loc = "897:1"]
unsafe extern "C" fn pool_free_size(mut size: size_t, mut p: *mut libc::c_void) {
    pool_free_pages(p, size);
}
#[no_mangle]
#[c2rust::src_loc = "913:1"]
pub unsafe extern "C" fn ponyint_pool_free_size(mut size: size_t, mut p: *mut libc::c_void) {
    let mut index: size_t = ponyint_pool_index(size);
    if index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        return ponyint_pool_free(index, p);
    }
    size = ponyint_pool_adjust_size(size);
    pool_free_size(size, p);
}
#[no_mangle]
#[c2rust::src_loc = "924:1"]
pub unsafe extern "C" fn ponyint_pool_realloc_size(
    mut old_size: size_t,
    mut new_size: size_t,
    mut p: *mut libc::c_void,
) -> *mut libc::c_void {
    if p.is_null() {
        return ponyint_pool_alloc_size(new_size);
    }
    let mut old_index: size_t = ponyint_pool_index(old_size);
    let mut new_index: size_t = ponyint_pool_index(new_size);
    let mut old_adj_size: size_t = 0 as libc::c_int as size_t;
    let mut new_p: *mut libc::c_void = 0 as *mut libc::c_void;
    if new_index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        if old_index == new_index {
            return p;
        }
        new_p = ponyint_pool_alloc(new_index);
    } else {
        let mut new_adj_size: size_t = ponyint_pool_adjust_size(new_size);
        if old_index >= (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
            old_adj_size = ponyint_pool_adjust_size(old_size);
            if old_adj_size == new_adj_size {
                return p;
            }
        }
        new_p = pool_alloc_size(new_adj_size);
    }
    memcpy(
        new_p,
        p,
        if old_size < new_size {
            old_size
        } else {
            new_size
        },
    );
    if old_index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        ponyint_pool_free(old_index, p);
    } else {
        pool_free_size(old_adj_size, p);
    }
    new_p
}
#[no_mangle]
#[c2rust::src_loc = "968:1"]
pub unsafe extern "C" fn ponyint_pool_thread_cleanup() {
    let mut index: size_t = 0 as libc::c_int as size_t;
    while index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        let mut thread: *mut pool_local_t =
            &mut *pool_local.as_mut_ptr().offset(index as isize) as *mut pool_local_t;
        let mut global: *mut pool_global_t =
            &mut *pool_global.as_mut_ptr().offset(index as isize) as *mut pool_global_t;
        while (*thread).start < (*thread).end {
            if (*thread).length == (*global).count {
                pool_push(thread, global);
            }
            let mut item: *mut pool_item_t = (*thread).start as *mut pool_item_t;
            let ref mut fresh23 = (*thread).start;
            *fresh23 = (*fresh23).offset((*global).size as isize);
            let ref mut fresh24 = (*item).next;
            *fresh24 = (*thread).pool;
            let ref mut fresh25 = (*thread).pool;
            *fresh25 = item;
            let ref mut fresh26 = (*thread).length;
            *fresh26 = (*fresh26).wrapping_add(1);
        }
        if (*thread).length > 0 as libc::c_int as libc::c_ulong {
            pool_push(thread, global);
        }
        index = index.wrapping_add(1);
    }
    let mut block: *mut pool_block_t = pool_block_header.head;
    while !block.is_null() {
        let mut next: *mut pool_block_t = (*block).c2rust_unnamed.next;
        pool_block_remove(block);
        pool_block_push(block);
        block = next;
    }
    pool_block_header.total_size = 0 as libc::c_int as size_t;
    pool_block_header.largest_size = 0 as libc::c_int as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "1004:1"]
pub unsafe extern "C" fn ponyint_pool_index(mut size: size_t) -> size_t {
    if size > ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong {
        return ((64 as libc::c_int - 5 as libc::c_int) as size_t)
            .wrapping_sub(__pony_clzzu(size) as libc::c_ulong)
            .wrapping_sub(
                (size & size.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0) as libc::c_int
                    as libc::c_ulong,
            );
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "1020:1"]
pub unsafe extern "C" fn ponyint_pool_used_size(mut size: size_t) -> size_t {
    let mut index: size_t = ponyint_pool_index(size);
    if index < (20 as libc::c_int - 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        return (1 as libc::c_int as size_t)
            << (5 as libc::c_int as libc::c_ulong).wrapping_add(index);
    }
    ponyint_pool_adjust_size(size)
}
#[no_mangle]
#[c2rust::src_loc = "1030:1"]
pub unsafe extern "C" fn ponyint_pool_adjust_size(mut size: size_t) -> size_t {
    if size & (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        size = (size
            & !(((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong);
    }
    if size == 0 as libc::c_int as libc::c_ulong {
        size = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    size
}
