use ::libc;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pagemap.h:3"]
pub mod pagemap_h {
    extern "C" {
        #[c2rust::src_loc = "8:16"]
        pub type chunk_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:5"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:6"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/pony/detail/atomics.h:9"]
pub mod atomics_h {
    extern "C" {
        #[c2rust::src_loc = "247:1"]
        pub fn f__atomic_thread_fence(_: *const libc::c_char);
    }
}
pub use self::_uintptr_t_h::uintptr_t;
use self::atomics_h::f__atomic_thread_fence;
use self::pagemap_h::chunk_t;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stddef_h::size_t;
use self::string_h::memset;
#[c2rust::src_loc = "42:1"]
pub type pagemap_node_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "34:16"]
pub struct pagemap_level_t {
    pub shift: libc::c_int,
    pub mask: libc::c_int,
    pub size: usize,
    pub size_index: usize,
}
#[c2rust::src_loc = "53:30"]
static mut level: [pagemap_level_t; 3] = [pagemap_level_t {
    shift: 0,
    mask: 0,
    size: 0,
    size_index: 0,
}; 3];
#[c2rust::src_loc = "69:36"]
static mut root: pagemap_node_t = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "90:1"]
pub unsafe extern "C" fn ponyint_pagemap_get(mut addr: *const libc::c_void) -> *mut chunk_t {
    let mut next_node: *mut pagemap_node_t = &mut root;
    let mut node: pagemap_node_t = { ::core::intrinsics::atomic_load_acq(next_node) };
    let mut i: usize = 0;
    while i < 3 as libc::c_int as libc::c_ulong {
        if node.is_null() {
            return 0 as *mut chunk_t;
        }
        let mut ix: uintptr_t =
            addr as uintptr_t >> level[i as usize].shift & level[i as usize].mask as libc::c_ulong;
        next_node = (node as *mut pagemap_node_t).offset(ix as isize) as *mut pagemap_node_t;
        node = ::core::intrinsics::atomic_load_acq(next_node);
        i = i.wrapping_add(1);
    }
    node as *mut chunk_t
}
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub unsafe extern "C" fn ponyint_pagemap_set(
    mut addr: *const libc::c_void,
    mut chunk: *mut chunk_t,
) {
    let mut next_node: *mut pagemap_node_t = &mut root;
    let mut i: usize = 0;
    while i < 3 as libc::c_int as libc::c_ulong {
        let mut node: pagemap_node_t = { ::core::intrinsics::atomic_load_relaxed(next_node) };
        if node.is_null() {
            let mut new_node: pagemap_node_t = ponyint_pool_alloc(level[i as usize].size_index);
            memset(new_node, 0 as libc::c_int, level[i as usize].size);
            if !({
                let fresh0 = ::core::intrinsics::atomic_cxchg_acqrel(
                    next_node,
                    *(&mut node as *mut pagemap_node_t),
                    new_node,
                );
                *(&mut node as *mut pagemap_node_t) = fresh0.0;
                fresh0.1
            }) {
                ponyint_pool_free(level[i as usize].size_index, new_node);
            } else {
                node = new_node;
            }
        } else {
            f__atomic_thread_fence(b"memory_order_acquire\0" as *const u8 as *const libc::c_char);
        }
        let mut ix: uintptr_t =
            addr as uintptr_t >> level[i as usize].shift & level[i as usize].mask as libc::c_ulong;
        next_node = (node as *mut pagemap_node_t).offset(ix as isize) as *mut pagemap_node_t;
        i = i.wrapping_add(1);
    }
    ({
        ::core::intrinsics::atomic_store_rel(next_node, chunk as pagemap_node_t);
        // compile_error!("Builtin is not supposed to be used")
    });
}
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn ponyint_pagemap_set_bulk(
    mut addr: *const libc::c_void,
    mut chunk: *mut chunk_t,
    mut size: usize,
) {
    let mut next_node: *mut pagemap_node_t = 0 as *mut pagemap_node_t;
    let mut node: pagemap_node_t = 0 as *mut libc::c_void;
    let mut ix: uintptr_t = 0 as libc::c_int as uintptr_t;
    let mut addr_ptr: uintptr_t = addr as uintptr_t;
    let mut addr_end: uintptr_t = (addr as uintptr_t).wrapping_add(size);
    while addr_ptr < addr_end {
        next_node = &mut root;
        let mut i: usize = 0;
        while i < 3 as libc::c_int as libc::c_ulong {
            node = ::core::intrinsics::atomic_load_relaxed(next_node);
            if node.is_null() {
                let mut new_node: *mut libc::c_void =
                    ponyint_pool_alloc(level[i as usize].size_index);
                memset(new_node, 0 as libc::c_int, level[i as usize].size);
                if !({
                    let fresh1 = ::core::intrinsics::atomic_cxchg_acqrel(
                        next_node,
                        *(&mut node as *mut pagemap_node_t),
                        new_node,
                    );
                    *(&mut node as *mut pagemap_node_t) = fresh1.0;
                    fresh1.1
                }) {
                    ponyint_pool_free(level[i as usize].size_index, new_node);
                } else {
                    node = new_node;
                }
            } else {
                f__atomic_thread_fence(
                    b"memory_order_acquire\0" as *const u8 as *const libc::c_char,
                );
            }
            ix = addr_ptr >> level[i as usize].shift & level[i as usize].mask as libc::c_ulong;
            next_node = (node as *mut pagemap_node_t).offset(ix as isize) as *mut pagemap_node_t;
            i = i.wrapping_add(1);
        }
        loop {
            ({
                ::core::intrinsics::atomic_store_rel(next_node, chunk as pagemap_node_t);
                // compile_error!("Builtin is not supposed to be used")
            });
            addr_ptr = (addr_ptr as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong)
                as uintptr_t as uintptr_t;
            ix = ix.wrapping_add(1);
            next_node = (node as *mut pagemap_node_t).offset(ix as isize) as *mut pagemap_node_t;
            if !(addr_ptr < addr_end
                && ix <= level[(3 as libc::c_int - 1 as libc::c_int) as usize].mask as uintptr_t)
            {
                break;
            }
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    level = [
        {
            let mut init = pagemap_level_t {
                shift: 10 as libc::c_int
                    + (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                    + ((48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                        + ((48 as libc::c_int - 10 as libc::c_int) % 3 as libc::c_int
                            > 1 as libc::c_int) as libc::c_int),
                mask: ((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                        + ((48 as libc::c_int - 10 as libc::c_int) % 3 as libc::c_int
                            > 0 as libc::c_int) as libc::c_int)
                    - 1 as libc::c_int,
                size: (((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                        + ((48 as libc::c_int - 10 as libc::c_int) % 3 as libc::c_int
                            > 0 as libc::c_int) as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pagemap_node_t>() as libc::c_ulong),
                size_index: 11 as libc::c_int as usize,
            };
            init
        },
        {
            let mut init = pagemap_level_t {
                shift: 10 as libc::c_int
                    + (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int,
                mask: ((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                        + ((48 as libc::c_int - 10 as libc::c_int) % 3 as libc::c_int
                            > 1 as libc::c_int) as libc::c_int)
                    - 1 as libc::c_int,
                size: (((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int
                        + ((48 as libc::c_int - 10 as libc::c_int) % 3 as libc::c_int
                            > 1 as libc::c_int) as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pagemap_node_t>() as libc::c_ulong),
                size_index: 11 as libc::c_int as usize,
            };
            init
        },
        {
            let mut init = pagemap_level_t {
                shift: 10 as libc::c_int,
                mask: ((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int)
                    - 1 as libc::c_int,
                size: (((1 as libc::c_int)
                    << (48 as libc::c_int - 10 as libc::c_int) / 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pagemap_node_t>() as libc::c_ulong),
                size_index: 10 as libc::c_int as usize,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
