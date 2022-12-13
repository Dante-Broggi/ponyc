use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:1"]
pub mod stack_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct Stack {
        pub index: libc::c_int,
        pub data: [*mut libc::c_void; 62],
        pub prev: *mut Stack,
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:3"]
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
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::stack_h::Stack;
pub use self::stddef_h::size_t;
#[c2rust::src_loc = "5:1"]
unsafe extern "C" fn stack_new(mut prev: *mut Stack, mut data: *mut libc::c_void) -> *mut Stack {
    let mut stack: *mut Stack = ponyint_pool_alloc(4 as libc::c_int as size_t) as *mut Stack;
    (*stack).index = 1 as libc::c_int;
    let ref mut fresh0 = (*stack).data[0 as libc::c_int as usize];
    *fresh0 = data;
    let ref mut fresh1 = (*stack).prev;
    *fresh1 = prev;
    return stack;
}
#[no_mangle]
#[c2rust::src_loc = "15:1"]
pub unsafe extern "C" fn ponyint_stack_pop(
    mut stack: *mut Stack,
    mut data: *mut *mut libc::c_void,
) -> *mut Stack {
    if !stack.is_null() {
    } else {
        ponyint_assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.c\0" as *const u8
                as *const libc::c_char,
            17 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_stack_pop\0"))
                .as_ptr(),
        );
    };
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.c\0" as *const u8
                as *const libc::c_char,
            18 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ponyint_stack_pop\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh2 = (*stack).index;
    *fresh2 -= 1;
    *data = (*stack).data[(*stack).index as usize];
    if (*stack).index == 0 as libc::c_int {
        let mut prev: *mut Stack = (*stack).prev;
        ponyint_pool_free(4 as libc::c_int as size_t, stack as *mut libc::c_void);
        return prev;
    }
    return stack;
}
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn ponyint_stack_push(
    mut stack: *mut Stack,
    mut data: *mut libc::c_void,
) -> *mut Stack {
    if !stack.is_null() && (*stack).index < 62 as libc::c_int {
        let ref mut fresh3 = (*stack).data[(*stack).index as usize];
        *fresh3 = data;
        let ref mut fresh4 = (*stack).index;
        *fresh4 += 1;
    } else {
        stack = stack_new(stack, data);
    }
    return stack;
}
