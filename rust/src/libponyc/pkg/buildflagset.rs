use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:2"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: usize,
        pub size: usize,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: usize,
    }
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = usize;
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: usize);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: usize,
        );
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: *mut usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut usize,
            count: usize,
            item_bitmap: *mut bitmap_t,
            size: usize,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:2"]
pub mod fun_h {
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:2"]
pub mod stringtab_h {
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:2"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "344:6"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:5"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:6"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_assert_fail(
            expr: *const libc::c_char,
            file: *const libc::c_char,
            line: usize,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
use self::stdio_h::snprintf;
use self::string_h::{memcpy, strlen};
use self::stringtab_h::stringtab;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "119:8"]
pub struct buildflagset_t {
    pub have_os_flags: bool,
    pub have_arch_flags: bool,
    pub have_size_flags: bool,
    pub have_endian_flags: bool,
    pub started_enum: bool,
    pub first_config_ready: bool,
    pub flags: *mut flagtab_t,
    pub enum_os_flags: u32,
    pub enum_arch_flags: u32,
    pub enum_size_flags: u32,
    pub enum_endian_flags: u32,
    pub text_buffer: *mut libc::c_char,
    pub buffer_size: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "115:26"]
pub struct flagtab_t {
    pub contents: hashmap_t,
}
#[c2rust::src_loc = "116:1"]
pub type flagtab_free_fn = Option<unsafe extern "C" fn(*mut flag_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "78:16"]
pub struct flag_t {
    pub name: *const libc::c_char,
    pub value: bool,
}
#[c2rust::src_loc = "116:1"]
pub type flagtab_cmp_fn = Option<unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool>;
#[c2rust::src_loc = "12:20"]
static mut _os_flags: [*const libc::c_char; 9] = [
    b"bsd\0" as *const u8 as *const libc::c_char,
    b"freebsd\0" as *const u8 as *const libc::c_char,
    b"dragonfly\0" as *const u8 as *const libc::c_char,
    b"openbsd\0" as *const u8 as *const libc::c_char,
    b"linux\0" as *const u8 as *const libc::c_char,
    b"osx\0" as *const u8 as *const libc::c_char,
    b"windows\0" as *const u8 as *const libc::c_char,
    b"unknown_OS\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[c2rust::src_loc = "25:20"]
static mut _arch_flags: [*const libc::c_char; 4] = [
    b"x86\0" as *const u8 as *const libc::c_char,
    b"arm\0" as *const u8 as *const libc::c_char,
    b"unknown_arch\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[c2rust::src_loc = "33:20"]
static mut _size_flags: [*const libc::c_char; 5] = [
    b"lp64\0" as *const u8 as *const libc::c_char,
    b"llp64\0" as *const u8 as *const libc::c_char,
    b"ilp32\0" as *const u8 as *const libc::c_char,
    b"unknown_size\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[c2rust::src_loc = "42:20"]
static mut _endian_flags: [*const libc::c_char; 4] = [
    b"bigendian\0" as *const u8 as *const libc::c_char,
    b"littleendian\0" as *const u8 as *const libc::c_char,
    b"unknown_endian\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[c2rust::src_loc = "50:13"]
static mut _stringtabed: bool = 0 as libc::c_int != 0;
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn stringtab_mutexgroups() {
    if _stringtabed {
        return;
    }
    let mut i: usize = 0;
    while !(_os_flags[i as usize]).is_null() {
        _os_flags[i as usize] = stringtab(_os_flags[i as usize]);
        i = i.wrapping_add(1);
    }
    let mut i_0: usize = 0;
    while !(_arch_flags[i_0 as usize]).is_null() {
        _arch_flags[i_0 as usize] = stringtab(_arch_flags[i_0 as usize]);
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: usize = 0;
    while !(_size_flags[i_1 as usize]).is_null() {
        _size_flags[i_1 as usize] = stringtab(_size_flags[i_1 as usize]);
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: usize = 0;
    while !(_endian_flags[i_2 as usize]).is_null() {
        _endian_flags[i_2 as usize] = stringtab(_endian_flags[i_2 as usize]);
        i_2 = i_2.wrapping_add(1);
    }
    _stringtabed = 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn flag_hash(mut flag: *mut flag_t) -> usize {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            88 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"flag_hash\0")).as_ptr(),
        );
    };
    return ponyint_hash_ptr((*flag).name as *const libc::c_void);
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn flag_cmp(mut a: *mut flag_t, mut b: *mut flag_t) -> bool {
    if !a.is_null() {
    } else {
        ponyint_assert_fail(
            b"a != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            94 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"flag_cmp\0")).as_ptr(),
        );
    };
    if !b.is_null() {
    } else {
        ponyint_assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            95 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"flag_cmp\0")).as_ptr(),
        );
    };
    return (*a).name == (*b).name;
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn flag_dup(mut flag: *mut flag_t) -> *mut flag_t {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            102 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"flag_dup\0")).as_ptr(),
        );
    };
    let mut f: *mut flag_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut flag_t;
    memcpy(
        f as *mut libc::c_void,
        flag as *const libc::c_void,
        ::core::mem::size_of::<flag_t>(),
    );
    f
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn flag_free(mut flag: *mut flag_t) {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            111 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"flag_free\0")).as_ptr(),
        );
    };
    ponyint_pool_free(0 as libc::c_int as usize, flag as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_size(mut map: *mut flagtab_t) -> usize {
    return ponyint_hashmap_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_destroy(mut map: *mut flagtab_t) {
    let mut freef: flagtab_free_fn = Some(flag_free as unsafe extern "C" fn(*mut flag_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<flagtab_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_optimize(mut map: *mut flagtab_t) {
    let mut cmpf: flagtab_cmp_fn =
        Some(flag_cmp as unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<flagtab_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_removeindex(mut map: *mut flagtab_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_clearindex(mut map: *mut flagtab_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_init(mut map: *mut flagtab_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_mem_size(mut map: *mut flagtab_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_putindex(
    mut map: *mut flagtab_t,
    mut entry: *mut flag_t,
    mut index: usize,
) {
    let mut cmpf: flagtab_cmp_fn =
        Some(flag_cmp as unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        flag_hash(entry),
        ::core::mem::transmute::<flagtab_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn flagtab_alloc_size(mut map: *mut flagtab_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "116:36"]
pub unsafe extern "C" fn flagtab_remove(
    mut map: *mut flagtab_t,
    mut entry: *mut flag_t,
) -> *mut flag_t {
    let mut cmpf: flagtab_cmp_fn =
        Some(flag_cmp as unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        flag_hash(entry),
        ::core::mem::transmute::<flagtab_cmp_fn, cmp_fn>(cmpf),
    ) as *mut flag_t
}
#[no_mangle]
#[c2rust::src_loc = "116:36"]
pub unsafe extern "C" fn flagtab_put(
    mut map: *mut flagtab_t,
    mut entry: *mut flag_t,
) -> *mut flag_t {
    let mut cmpf: flagtab_cmp_fn =
        Some(flag_cmp as unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        flag_hash(entry),
        ::core::mem::transmute::<flagtab_cmp_fn, cmp_fn>(cmpf),
    ) as *mut flag_t
}
#[no_mangle]
#[c2rust::src_loc = "116:36"]
pub unsafe extern "C" fn flagtab_next(mut map: *mut flagtab_t, mut i: *mut usize) -> *mut flag_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets) as *mut flag_t
}
#[no_mangle]
#[c2rust::src_loc = "116:36"]
pub unsafe extern "C" fn flagtab_get(
    mut map: *mut flagtab_t,
    mut key: *mut flag_t,
    mut index: *mut usize,
) -> *mut flag_t {
    let mut cmpf: flagtab_cmp_fn =
        Some(flag_cmp as unsafe extern "C" fn(*mut flag_t, *mut flag_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        flag_hash(key),
        ::core::mem::transmute::<flagtab_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut flag_t
}
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn buildflagset_create() -> *mut buildflagset_t {
    let mut p: *mut buildflagset_t =
        ponyint_pool_alloc(1 as libc::c_int as usize) as *mut buildflagset_t;
    if !p.is_null() {
    } else {
        ponyint_assert_fail(
            b"p != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            140 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"buildflagset_create\0"))
                .as_ptr(),
        );
    };
    (*p).have_os_flags = 0 as libc::c_int != 0;
    (*p).have_arch_flags = 0 as libc::c_int != 0;
    (*p).have_size_flags = 0 as libc::c_int != 0;
    (*p).have_endian_flags = 0 as libc::c_int != 0;
    (*p).started_enum = 0 as libc::c_int != 0;
    let ref mut fresh0 = (*p).flags;
    *fresh0 = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut flagtab_t;
    flagtab_init((*p).flags, 8 as libc::c_int as usize);
    let ref mut fresh1 = (*p).text_buffer;
    *fresh1 = 0 as *mut libc::c_char;
    (*p).buffer_size = 0 as libc::c_int as usize;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn buildflagset_free(mut set: *mut buildflagset_t) {
    if !set.is_null() {
        flagtab_destroy((*set).flags);
        ponyint_pool_free(0 as libc::c_int as usize, (*set).flags as *mut libc::c_void);
        if (*set).buffer_size > 0 {
            ponyint_pool_free_size((*set).buffer_size, (*set).text_buffer as *mut libc::c_void);
        }
        ponyint_pool_free(1 as libc::c_int as usize, set as *mut libc::c_void);
    }
}
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn os_index(mut flag: *const libc::c_char) -> ssize_t {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            175 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"os_index\0")).as_ptr(),
        );
    };
    stringtab_mutexgroups();
    let mut i: usize = 0;
    while !(_os_flags[i as usize]).is_null() {
        if flag == _os_flags[i as usize] {
            return i as ssize_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn arch_index(mut flag: *const libc::c_char) -> ssize_t {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            192 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"arch_index\0")).as_ptr(),
        );
    };
    stringtab_mutexgroups();
    let mut i: usize = 0;
    while !(_arch_flags[i as usize]).is_null() {
        if flag == _arch_flags[i as usize] {
            return i as ssize_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn size_index(mut flag: *const libc::c_char) -> ssize_t {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            209 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"size_index\0")).as_ptr(),
        );
    };
    stringtab_mutexgroups();
    let mut i: usize = 0;
    while !(_size_flags[i as usize]).is_null() {
        if flag == _size_flags[i as usize] {
            return i as ssize_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[c2rust::src_loc = "224:1"]
unsafe extern "C" fn endian_index(mut flag: *const libc::c_char) -> ssize_t {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            226 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"endian_index\0")).as_ptr(),
        );
    };
    stringtab_mutexgroups();
    let mut i: usize = 0;
    while !(_endian_flags[i as usize]).is_null() {
        if flag == _endian_flags[i as usize] {
            return i as ssize_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[no_mangle]
#[c2rust::src_loc = "239:1"]
pub unsafe extern "C" fn buildflagset_add(
    mut set: *mut buildflagset_t,
    mut flag: *const libc::c_char,
) {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            241 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_add\0"))
                .as_ptr(),
        );
    };
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            242 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_add\0"))
                .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            243 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_add\0"))
                .as_ptr(),
        );
    };
    if !(*set).started_enum {
    } else {
        ponyint_assert_fail(
            b"!set->started_enum\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            244 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_add\0"))
                .as_ptr(),
        );
    };
    if os_index(flag) >= 0 as libc::c_int as libc::c_long {
        (*set).have_os_flags = 1 as libc::c_int != 0;
        return;
    }
    if arch_index(flag) >= 0 as libc::c_int as libc::c_long {
        (*set).have_arch_flags = 1 as libc::c_int != 0;
        return;
    }
    if size_index(flag) >= 0 as libc::c_int as libc::c_long {
        (*set).have_size_flags = 1 as libc::c_int != 0;
        return;
    }
    if endian_index(flag) >= 0 as libc::c_int as libc::c_long {
        (*set).have_endian_flags = 1 as libc::c_int != 0;
        return;
    }
    let mut f1: flag_t = {
        let mut init = flag_t {
            name: flag,
            value: 0 as libc::c_int != 0,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut f2: *mut flag_t = flagtab_get((*set).flags, &mut f1, &mut index);
    if !f2.is_null() {
        return;
    }
    flagtab_putindex((*set).flags, flag_dup(&mut f1), index);
}
#[no_mangle]
#[c2rust::src_loc = "290:1"]
pub unsafe extern "C" fn buildflagset_configcount(mut set: *mut buildflagset_t) -> libc::c_double {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            292 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"buildflagset_configcount\0",
            ))
            .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            293 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"buildflagset_configcount\0",
            ))
            .as_ptr(),
        );
    };
    let mut r: libc::c_double = 1.0f64;
    if (*set).have_os_flags {
        let mut count: libc::c_int = 0 as libc::c_int;
        while !(_os_flags[count as usize]).is_null() {
            count += 1;
        }
        r *= count as libc::c_double;
    }
    if (*set).have_arch_flags {
        let mut count_0: libc::c_int = 0 as libc::c_int;
        while !(_arch_flags[count_0 as usize]).is_null() {
            count_0 += 1;
        }
        r *= count_0 as libc::c_double;
    }
    if (*set).have_size_flags {
        let mut count_1: libc::c_int = 0 as libc::c_int;
        while !(_size_flags[count_1 as usize]).is_null() {
            count_1 += 1;
        }
        r *= count_1 as libc::c_double;
    }
    if (*set).have_endian_flags {
        let mut count_2: libc::c_int = 0 as libc::c_int;
        while !(_endian_flags[count_2 as usize]).is_null() {
            count_2 += 1;
        }
        r *= count_2 as libc::c_double;
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    while !(flagtab_next((*set).flags, &mut i)).is_null() {
        r *= 2 as libc::c_int as libc::c_double;
    }
    return r;
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn buildflagset_startenum(mut set: *mut buildflagset_t) {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            346 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"buildflagset_startenum\0",
            ))
            .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            347 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"buildflagset_startenum\0",
            ))
            .as_ptr(),
        );
    };
    (*set).started_enum = 1 as libc::c_int != 0;
    (*set).first_config_ready = 1 as libc::c_int != 0;
    (*set).enum_os_flags = 0 as libc::c_int as u32;
    (*set).enum_arch_flags = 0 as libc::c_int as u32;
    (*set).enum_size_flags = 0 as libc::c_int as u32;
    (*set).enum_endian_flags = 0 as libc::c_int as u32;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut flag: *mut flag_t = 0 as *mut flag_t;
    loop {
        flag = flagtab_next((*set).flags, &mut i);
        if flag.is_null() {
            break;
        }
        (*flag).value = 0 as libc::c_int != 0;
    }
}
#[no_mangle]
#[c2rust::src_loc = "366:1"]
pub unsafe extern "C" fn buildflagset_next(mut set: *mut buildflagset_t) -> bool {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            368 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"buildflagset_next\0"))
                .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            369 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"buildflagset_next\0"))
                .as_ptr(),
        );
    };
    if (*set).started_enum as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"set->started_enum\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            370 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"buildflagset_next\0"))
                .as_ptr(),
        );
    };
    if (*set).first_config_ready {
        (*set).first_config_ready = 0 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    if (*set).have_os_flags {
        let ref mut fresh2 = (*set).enum_os_flags;
        *fresh2 = (*fresh2).wrapping_add(1);
        if !(_os_flags[(*set).enum_os_flags as usize]).is_null() {
            return 1 as libc::c_int != 0;
        }
        (*set).enum_os_flags = 0 as libc::c_int as u32;
    }
    if (*set).have_arch_flags {
        let ref mut fresh3 = (*set).enum_arch_flags;
        *fresh3 = (*fresh3).wrapping_add(1);
        if !(_arch_flags[(*set).enum_arch_flags as usize]).is_null() {
            return 1 as libc::c_int != 0;
        }
        (*set).enum_arch_flags = 0 as libc::c_int as u32;
    }
    if (*set).have_size_flags {
        let ref mut fresh4 = (*set).enum_size_flags;
        *fresh4 = (*fresh4).wrapping_add(1);
        if !(_size_flags[(*set).enum_size_flags as usize]).is_null() {
            return 1 as libc::c_int != 0;
        }
        (*set).enum_size_flags = 0 as libc::c_int as u32;
    }
    if (*set).have_endian_flags {
        let ref mut fresh5 = (*set).enum_endian_flags;
        *fresh5 = (*fresh5).wrapping_add(1);
        if !(_endian_flags[(*set).enum_endian_flags as usize]).is_null() {
            return 1 as libc::c_int != 0;
        }
        (*set).enum_endian_flags = 0 as libc::c_int as u32;
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut flag: *mut flag_t = 0 as *mut flag_t;
    loop {
        flag = flagtab_next((*set).flags, &mut i);
        if flag.is_null() {
            break;
        }
        if !(*flag).value {
            (*flag).value = 1 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
        (*flag).value = 0 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "445:1"]
pub unsafe extern "C" fn buildflagset_get(
    mut set: *mut buildflagset_t,
    mut flag: *const libc::c_char,
) -> bool {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            447 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_get\0"))
                .as_ptr(),
        );
    };
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            448 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_get\0"))
                .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            449 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_get\0"))
                .as_ptr(),
        );
    };
    if (*set).started_enum as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"set->started_enum\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            450 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_get\0"))
                .as_ptr(),
        );
    };
    let mut index: ssize_t = os_index(flag);
    if index >= 0 as libc::c_int as libc::c_long {
        return (*set).enum_os_flags == index as u32;
    }
    index = arch_index(flag);
    if index >= 0 as libc::c_int as libc::c_long {
        return (*set).enum_arch_flags == index as u32;
    }
    index = size_index(flag);
    if index >= 0 as libc::c_int as libc::c_long {
        return (*set).enum_size_flags == index as u32;
    }
    index = endian_index(flag);
    if index >= 0 as libc::c_int as libc::c_long {
        return (*set).enum_endian_flags == index as u32;
    }
    let mut f1: flag_t = {
        let mut init = flag_t {
            name: flag,
            value: 0 as libc::c_int != 0,
        };
        init
    };
    let mut h_index: usize = -(1 as libc::c_int) as usize;
    let mut f2: *mut flag_t = flagtab_get((*set).flags, &mut f1, &mut h_index);
    if !f2.is_null() {
    } else {
        ponyint_assert_fail(
            b"f2 != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            479 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"buildflagset_get\0"))
                .as_ptr(),
        );
    };
    return (*f2).value;
}
#[c2rust::src_loc = "488:1"]
unsafe extern "C" fn print_str(
    mut s: *const libc::c_char,
    mut set: *mut buildflagset_t,
    mut pointer: *mut *mut libc::c_char,
) {
    if !s.is_null() {
    } else {
        ponyint_assert_fail(
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            490 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"print_str\0")).as_ptr(),
        );
    };
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            491 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"print_str\0")).as_ptr(),
        );
    };
    if !pointer.is_null() {
    } else {
        ponyint_assert_fail(
            b"pointer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            492 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"print_str\0")).as_ptr(),
        );
    };
    let mut len: usize = libc::strlen(s);
    if (*pointer).offset(len as isize) < ((*set).text_buffer).offset((*set).buffer_size as isize) {
        snprintf(
            *pointer,
            len.wrapping_add(1),
            b"%s\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    *pointer = (*pointer).offset(len as isize);
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn print_flag(
    mut flag: *const libc::c_char,
    mut sense: bool,
    mut set: *mut buildflagset_t,
    mut pointer: *mut *mut libc::c_char,
) {
    if !flag.is_null() {
    } else {
        ponyint_assert_fail(
            b"flag != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            512 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"print_flag\0")).as_ptr(),
        );
    };
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            513 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"print_flag\0")).as_ptr(),
        );
    };
    if *pointer != (*set).text_buffer {
        print_str(b" \0" as *const u8 as *const libc::c_char, set, pointer);
    }
    if !sense {
        print_str(b"!\0" as *const u8 as *const libc::c_char, set, pointer);
    }
    print_str(flag, set, pointer);
}
#[no_mangle]
#[c2rust::src_loc = "525:1"]
pub unsafe extern "C" fn buildflagset_print(mut set: *mut buildflagset_t) -> *const libc::c_char {
    if !set.is_null() {
    } else {
        ponyint_assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            527 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"buildflagset_print\0"))
                .as_ptr(),
        );
    };
    if !((*set).flags).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            528 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"buildflagset_print\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut libc::c_char = (*set).text_buffer;
    if (*set).have_os_flags {
        print_flag(
            _os_flags[(*set).enum_os_flags as usize],
            1 as libc::c_int != 0,
            set,
            &mut p,
        );
    }
    if (*set).have_arch_flags {
        print_flag(
            _arch_flags[(*set).enum_arch_flags as usize],
            1 as libc::c_int != 0,
            set,
            &mut p,
        );
    }
    if (*set).have_size_flags {
        print_flag(
            _size_flags[(*set).enum_size_flags as usize],
            1 as libc::c_int != 0,
            set,
            &mut p,
        );
    }
    if (*set).have_endian_flags {
        print_flag(
            _endian_flags[(*set).enum_endian_flags as usize],
            1 as libc::c_int != 0,
            set,
            &mut p,
        );
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut flag: *mut flag_t = 0 as *mut flag_t;
    loop {
        flag = flagtab_next((*set).flags, &mut i);
        if flag.is_null() {
            break;
        }
        print_flag((*flag).name, (*flag).value, set, &mut p);
    }
    if p == (*set).text_buffer {
        print_str(
            b"all configs\0" as *const u8 as *const libc::c_char,
            set,
            &mut p,
        );
    }
    let mut size_needed: usize = (p.offset_from((*set).text_buffer) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as usize;
    if size_needed > (*set).buffer_size {
        if (*set).buffer_size > 0 {
            ponyint_pool_free_size((*set).buffer_size, (*set).text_buffer as *mut libc::c_void);
        }
        let ref mut fresh6 = (*set).text_buffer;
        *fresh6 = ponyint_pool_alloc_size(size_needed) as *mut libc::c_char;
        (*set).buffer_size = size_needed;
        *((*set).text_buffer).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        buildflagset_print(set);
    }
    if !((*set).text_buffer).is_null() {
    } else {
        ponyint_assert_fail(
            b"set->text_buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            571 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"buildflagset_print\0"))
                .as_ptr(),
        );
    };
    *((*set).text_buffer)
        .offset(size_needed.wrapping_sub(1) as isize) =
        '\0' as i32 as libc::c_char;
    return (*set).text_buffer;
}
#[c2rust::src_loc = "577:19"]
static mut _user_flags: *mut flagtab_t = 0 as *const flagtab_t as *mut flagtab_t;
#[no_mangle]
#[c2rust::src_loc = "579:1"]
pub unsafe extern "C" fn define_build_flag(mut name: *const libc::c_char) -> bool {
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            581 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"define_build_flag\0"))
                .as_ptr(),
        );
    };
    if _user_flags.is_null() {
        _user_flags = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut flagtab_t;
        flagtab_init(_user_flags, 8 as libc::c_int as usize);
    }
    let mut f1: flag_t = {
        let mut init = flag_t {
            name: stringtab(name),
            value: 0 as libc::c_int != 0,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut f2: *mut flag_t = flagtab_get(_user_flags, &mut f1, &mut index);
    if !f2.is_null() {
        return 0 as libc::c_int != 0;
    }
    flagtab_putindex(_user_flags, flag_dup(&mut f1), index);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "605:1"]
pub unsafe extern "C" fn remove_build_flags(mut flags: *mut *const libc::c_char) -> bool {
    if !flags.is_null() {
    } else {
        ponyint_assert_fail(
            b"flags != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            607 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"remove_build_flags\0"))
                .as_ptr(),
        );
    };
    if _user_flags.is_null() {
        _user_flags = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut flagtab_t;
        flagtab_init(_user_flags, 8 as libc::c_int as usize);
    }
    let mut removed: usize = 0;
    let mut next: *mut *const libc::c_char = flags;
    while !(*next).is_null() {
        let mut f1: flag_t = {
            let mut init = flag_t {
                name: stringtab(*next),
                value: 0 as libc::c_int != 0,
            };
            init
        };
        let mut found: *mut flag_t = flagtab_remove(_user_flags, &mut f1);
        if !found.is_null() {
            flag_free(found);
            removed = (removed as libc::c_ulong).wrapping_add(1)
                as usize as usize;
        }
        next = next.offset(1 as libc::c_int as isize);
    }
    return removed > 0;
}
#[no_mangle]
#[c2rust::src_loc = "632:1"]
pub unsafe extern "C" fn is_build_flag_defined(mut name: *const libc::c_char) -> bool {
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/buildflagset.c\0"
                as *const u8 as *const libc::c_char,
            634 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"is_build_flag_defined\0"))
                .as_ptr(),
        );
    };
    if _user_flags.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut f1: flag_t = {
        let mut init = flag_t {
            name: stringtab(name),
            value: 0 as libc::c_int != 0,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut f2: *mut flag_t = flagtab_get(_user_flags, &mut f1, &mut index);
    return !f2.is_null();
}
#[no_mangle]
#[c2rust::src_loc = "647:1"]
pub unsafe extern "C" fn clear_build_flags() {
    if !_user_flags.is_null() {
        flagtab_destroy(_user_flags);
        ponyint_pool_free(0 as libc::c_int as usize, _user_flags as *mut libc::c_void);
        _user_flags = 0 as *mut flagtab_t;
    }
}
