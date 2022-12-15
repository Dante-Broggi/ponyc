use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "15:1"]
    pub type map_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
    #[c2rust::src_loc = "74:1"]
    pub type pony_trace_fn = Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "84:1"]
    pub type pony_serialise_fn = Option<
        unsafe extern "C" fn(
            *mut pony_ctx_t,
            *mut libc::c_void,
            *mut libc::c_void,
            usize,
            libc::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;
    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:22"]
    pub struct _pony_type_t {
        pub id: u32,
        pub size: u32,
        pub field_count: u32,
        pub field_offset: u32,
        pub instance: *mut libc::c_void,
        pub trace: pony_trace_fn,
        pub serialise_trace: pony_trace_fn,
        pub serialise: pony_serialise_fn,
        pub deserialise: pony_trace_fn,
        pub custom_serialise_space: pony_custom_serialise_space_fn,
        pub custom_deserialise: pony_custom_deserialise_fn,
        pub dispatch: pony_dispatch_fn,
        pub final_0: pony_final_fn,
        pub event_notify: u32,
        pub traits: *mut *mut libc::uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed = 0;

    #[c2rust::src_loc = "30:16"]
    pub use crate::libponyrt::actor::actor::pony_actor_t;
    #[c2rust::src_loc = "36:16"]
    pub use crate::libponyrt::sched::scheduler::pony_ctx_t;

    extern "C" {
        #[c2rust::src_loc = "394:1"]
        pub fn pony_traceknown(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            m: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/list.h:1"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct list_t {
        pub data: *mut libc::c_void,
        pub next: *mut list_t,
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:2"]
pub mod serialise_h {

    use super::pony_h::{pony_ctx_t, pony_type_t};

    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: libc::uintptr_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {

    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
    }
}
pub use self::_ssize_t_h::ssize_t;
pub use self::_types_h::__darwin_ssize_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::fun_h::{cmp_fn, free_fn, map_fn};
pub use self::list_h::list_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, pony_type_t, C2RustUnnamed, PONY_TRACE_IMMUTABLE,
    PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::serialise_h::{pony_deserialise_offset, pony_serialise_offset};
pub use self::stddef_h::size_t;
#[no_mangle]
#[c2rust::src_loc = "7:1"]
pub unsafe extern "C" fn ponyint_list_pop(
    mut list: *mut list_t,
    mut data: *mut *mut libc::c_void,
) -> *mut list_t {
    let mut l: *mut list_t = list;
    list = (*l).next;
    *data = (*l).data;
    ponyint_pool_free(0 as libc::c_int as usize, l as *mut libc::c_void);
    return list;
}
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ponyint_list_push(
    mut list: *mut list_t,
    mut data: *mut libc::c_void,
) -> *mut list_t {
    let mut l: *mut list_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut list_t;
    let ref mut fresh0 = (*l).data;
    *fresh0 = data;
    let ref mut fresh1 = (*l).next;
    *fresh1 = list;
    l
}
#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn ponyint_list_append(
    mut list: *mut list_t,
    mut data: *mut libc::c_void,
) -> *mut list_t {
    let mut l: *mut list_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut list_t;
    let ref mut fresh2 = (*l).data;
    *fresh2 = data;
    let ref mut fresh3 = (*l).next;
    *fresh3 = 0 as *mut list_t;
    if list.is_null() {
        return l;
    }
    let mut p: *mut list_t = list;
    while !((*p).next).is_null() {
        p = (*p).next;
    }
    let ref mut fresh4 = (*p).next;
    *fresh4 = l;
    list
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn ponyint_list_next(mut list: *mut list_t) -> *mut list_t {
    (*list).next
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn ponyint_list_index(
    mut list: *mut list_t,
    mut index: ssize_t,
) -> *mut list_t {
    if index < 0 as libc::c_int as libc::c_long {
        index = ponyint_list_length(list) as ssize_t + index;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while !list.is_null() && (i as libc::c_long) < index {
        list = (*list).next;
        i += 1;
    }
    list
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn ponyint_list_data(mut list: *mut list_t) -> *mut libc::c_void {
    (*list).data
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn ponyint_list_find(
    mut list: *mut list_t,
    mut f: cmp_fn,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    while !list.is_null() {
        if f.expect("non-null function pointer")(data, (*list).data) {
            return (*list).data;
        }
        list = (*list).next;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn ponyint_list_findindex(
    mut list: *mut list_t,
    mut f: cmp_fn,
    mut data: *mut libc::c_void,
) -> ssize_t {
    let mut index: usize = 0;
    while !list.is_null() {
        if f.expect("non-null function pointer")(data, (*list).data) {
            return index as ssize_t;
        }
        list = (*list).next;
        index = index.wrapping_add(1);
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn ponyint_list_subset(
    mut a: *mut list_t,
    mut b: *mut list_t,
    mut f: cmp_fn,
) -> bool {
    while !a.is_null() {
        if (ponyint_list_find(b, f, (*a).data)).is_null() {
            return 0 as libc::c_int != 0;
        }
        a = (*a).next;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn ponyint_list_equals(
    mut a: *mut list_t,
    mut b: *mut list_t,
    mut f: cmp_fn,
) -> bool {
    while !a.is_null() {
        if b.is_null() || !f.expect("non-null function pointer")((*a).data, (*b).data) {
            return 0 as libc::c_int != 0;
        }
        a = (*a).next;
        b = (*b).next;
    }
    return b.is_null();
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn ponyint_list_map(
    mut list: *mut list_t,
    mut f: map_fn,
    mut arg: *mut libc::c_void,
) -> *mut list_t {
    let mut to: *mut list_t = 0 as *mut list_t;
    while !list.is_null() {
        let mut result: *mut libc::c_void =
            f.expect("non-null function pointer")((*list).data, arg);
        if !result.is_null() {
            to = ponyint_list_push(to, result);
        }
        list = (*list).next;
    }
    return ponyint_list_reverse(to);
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn ponyint_list_reverse(mut list: *mut list_t) -> *mut list_t {
    let mut to: *mut list_t = 0 as *mut list_t;
    while !list.is_null() {
        let mut next: *mut list_t = (*list).next;
        let ref mut fresh5 = (*list).next;
        *fresh5 = to;
        to = list;
        list = next;
    }
    return to;
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn ponyint_list_length(mut list: *mut list_t) -> usize {
    let mut len: usize = 0;
    while !list.is_null() {
        len = len.wrapping_add(1);
        list = (*list).next;
    }
    len
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn ponyint_list_free(mut list: *mut list_t, mut f: free_fn) {
    let mut next: *mut list_t = 0 as *mut list_t;
    while !list.is_null() {
        next = (*list).next;
        if f.is_some() {
            f.expect("non-null function pointer")((*list).data);
        }
        ponyint_pool_free(0 as libc::c_int as usize, list as *mut libc::c_void);
        list = next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn ponyint_list_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut list_type: *const pony_type_t,
    mut elem_type: *const pony_type_t,
) {
    let mut list: *mut list_t = object as *mut list_t;
    if !((*list).data).is_null() {
        pony_traceknown(
            ctx,
            (*list).data,
            elem_type,
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*list).next).is_null() {
        pony_traceknown(
            ctx,
            (*list).next as *mut libc::c_void,
            list_type,
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn ponyint_list_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
) {
    let mut list: *mut list_t = object as *mut list_t;
    let mut dst: *mut list_t = (buf as libc::uintptr_t).wrapping_add(offset) as *mut list_t;
    let ref mut fresh6 = (*dst).data;
    *fresh6 = pony_serialise_offset(ctx, (*list).data) as *mut libc::c_void;
    let ref mut fresh7 = (*dst).next;
    *fresh7 = pony_serialise_offset(ctx, (*list).next as *mut libc::c_void) as *mut list_t;
}
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn ponyint_list_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut list_type: *const pony_type_t,
    mut elem_type: *const pony_type_t,
) {
    let mut list: *mut list_t = object as *mut list_t;
    let ref mut fresh8 = (*list).data;
    *fresh8 = pony_deserialise_offset(ctx, elem_type, (*list).data as libc::uintptr_t);
    let ref mut fresh9 = (*list).next;
    *fresh9 =
        pony_deserialise_offset(ctx, list_type, (*list).next as libc::uintptr_t) as *mut list_t;
}
