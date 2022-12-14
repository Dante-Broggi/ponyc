use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/platform.h:1"]
pub mod platform_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn __pony_clzll(mut x: u64) -> u32 {
        x.leading_zeros() as i32 as u32
    }
    #[inline]
    #[c2rust::src_loc = "327:1"]
    pub unsafe extern "C" fn __pony_clzzu(mut x: size_t) -> u32 {
        __pony_clzll(x as u64)
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:2"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_uintptr_t_h::uintptr_t;
pub use self::platform_h::{__pony_clzll, __pony_clzzu};
pub use self::stddef_h::size_t;
use self::string_h::strlen;
#[c2rust::src_loc = "6:28"]
static mut the_key: [libc::c_uchar; 16] = [
    0xfe as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn siphash24(
    mut key: *const libc::c_uchar,
    mut in_0: *const libc::c_uchar,
    mut len: size_t,
) -> u64 {
    let mut k0: u64 = *(key as *mut u64);
    let mut k1: u64 = *(key.offset(8 as libc::c_int as isize) as *mut u64);
    let mut b: u64 = (len as u64) << 56 as libc::c_int;
    let mut v0: u64 = k0 ^ 0x736f6d6570736575 as libc::c_ulonglong;
    let mut v1: u64 = k1 ^ 0x646f72616e646f6d as libc::c_ulonglong;
    let mut v2: u64 = k0 ^ 0x6c7967656e657261 as libc::c_ulonglong;
    let mut v3: u64 = k1 ^ 0x7465646279746573 as libc::c_ulonglong;
    let mut end: *const libc::c_uchar = in_0
        .offset(len as isize)
        .offset(-(len.wrapping_rem(8 as libc::c_int as libc::c_ulong) as isize));
    while in_0 != end {
        let mut m: u64 = *(in_0 as *mut u64);
        v3 ^= m;
        v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
        v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
        v1 ^= v0;
        v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
        v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
        v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
        v3 ^= v2;
        v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
        v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
        v3 ^= v0;
        v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
        v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
        v1 ^= v2;
        v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
        v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
        v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
        v1 ^= v0;
        v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
        v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
        v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
        v3 ^= v2;
        v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
        v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
        v3 ^= v0;
        v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
        v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
        v1 ^= v2;
        v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
        v0 ^= m;
        in_0 = in_0.offset(8 as libc::c_int as isize);
    }
    let mut current_block_42: u64;
    match len & 7 as libc::c_int as libc::c_ulong {
        7 => {
            b |= (*in_0.offset(6 as libc::c_int as isize) as u64) << 48 as libc::c_int;
            current_block_42 = 12975228715441869902;
        }
        6 => {
            current_block_42 = 12975228715441869902;
        }
        5 => {
            current_block_42 = 5966796404194383908;
        }
        4 => {
            current_block_42 = 15551203804450972332;
        }
        3 => {
            current_block_42 = 1273030355561950405;
        }
        2 => {
            current_block_42 = 3753403667802422685;
        }
        1 => {
            current_block_42 = 12300049035009383903;
        }
        _ => {
            current_block_42 = 12381812505308290051;
        }
    }
    match current_block_42 {
        12975228715441869902 => {
            b |= (*in_0.offset(5 as libc::c_int as isize) as u64) << 40 as libc::c_int;
            current_block_42 = 5966796404194383908;
        }
        _ => {}
    }
    match current_block_42 {
        5966796404194383908 => {
            b |= (*in_0.offset(4 as libc::c_int as isize) as u64) << 32 as libc::c_int;
            current_block_42 = 15551203804450972332;
        }
        _ => {}
    }
    match current_block_42 {
        15551203804450972332 => {
            b |= (*in_0.offset(3 as libc::c_int as isize) as u64) << 24 as libc::c_int;
            current_block_42 = 1273030355561950405;
        }
        _ => {}
    }
    match current_block_42 {
        1273030355561950405 => {
            b |= (*in_0.offset(2 as libc::c_int as isize) as u64) << 16 as libc::c_int;
            current_block_42 = 3753403667802422685;
        }
        _ => {}
    }
    match current_block_42 {
        3753403667802422685 => {
            b |= (*in_0.offset(1 as libc::c_int as isize) as u64) << 8 as libc::c_int;
            current_block_42 = 12300049035009383903;
        }
        _ => {}
    }
    match current_block_42 {
        12300049035009383903 => {
            b |= *in_0.offset(0 as libc::c_int as isize) as u64;
        }
        _ => {}
    }
    v3 ^= b;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 ^= b;
    v2 ^= 0xff as libc::c_int as libc::c_ulonglong;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulonglong).wrapping_add(v3) as u64 as u64;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulonglong).wrapping_add(v1) as u64 as u64;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    return v0 ^ v1 ^ v2 ^ v3;
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn ponyint_hash_block(mut p: *const libc::c_void, mut len: size_t) -> size_t {
    return siphash24(the_key.as_ptr(), p as *const libc::c_uchar, len) as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn ponyint_hash_block64(
    mut p: *const libc::c_void,
    mut len: size_t,
) -> u64 {
    return siphash24(the_key.as_ptr(), p as *const libc::c_uchar, len);
}
#[no_mangle]
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn ponyint_hash_str(mut str: *const libc::c_char) -> size_t {
    return siphash24(the_key.as_ptr(), str as *const libc::c_uchar, strlen(str)) as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub unsafe extern "C" fn ponyint_hash_ptr(mut p: *const libc::c_void) -> size_t {
    return ponyint_hash_int64(p as uintptr_t as u64) as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn ponyint_hash_int64(mut key: u64) -> u64 {
    key = (!key).wrapping_add(key << 21 as libc::c_int);
    key = key ^ key >> 24 as libc::c_int;
    key = key
        .wrapping_add(key << 3 as libc::c_int)
        .wrapping_add(key << 8 as libc::c_int);
    key = key ^ key >> 14 as libc::c_int;
    key = key
        .wrapping_add(key << 2 as libc::c_int)
        .wrapping_add(key << 4 as libc::c_int);
    key = key ^ key >> 28 as libc::c_int;
    key = key.wrapping_add(key << 31 as libc::c_int);
    return key;
}
#[no_mangle]
#[c2rust::src_loc = "183:1"]
pub unsafe extern "C" fn ponyint_hash_int32(mut key: u32) -> u32 {
    key = (!key).wrapping_add(key << 15 as libc::c_int);
    key = key ^ key >> 12 as libc::c_int;
    key = key.wrapping_add(key << 2 as libc::c_int);
    key = key ^ key >> 4 as libc::c_int;
    key = key
        .wrapping_add(key << 3 as libc::c_int)
        .wrapping_add(key << 11 as libc::c_int);
    key = key ^ key >> 16 as libc::c_int;
    return key;
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn ponyint_hash_size(mut key: size_t) -> size_t {
    return ponyint_hash_int64(key as u64) as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "203:1"]
pub unsafe extern "C" fn ponyint_next_pow2(mut i: size_t) -> size_t {
    i = i.wrapping_sub(1);
    if i == 0 as libc::c_int as libc::c_ulong {
        i = 64 as libc::c_int as size_t;
    } else {
        i = __pony_clzzu(i) as size_t;
    }
    return (1 as libc::c_int as size_t)
        << (if i == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_ulong
        } else {
            (64 as libc::c_int as libc::c_ulong).wrapping_sub(i)
        });
}
