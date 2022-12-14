use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __uint128_t = u128;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexint.h:1"]
pub mod lexint_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct lexint_t {
        pub low: u64,
        pub high: u64,
    }
}
pub use self::lexint_h::lexint_t;
#[no_mangle]
#[c2rust::src_loc = "14:1"]
pub unsafe extern "C" fn lexint_zero(mut i: *mut lexint_t) {
    (*i).low = 0 as libc::c_int as u64;
    (*i).high = 0 as libc::c_int as u64;
}
#[no_mangle]
#[c2rust::src_loc = "20:1"]
pub unsafe extern "C" fn lexint_cmp(mut a: *const lexint_t, mut b: *const lexint_t) -> libc::c_int {
    if (*a).high > (*b).high {
        return 1 as libc::c_int;
    }
    if (*a).high < (*b).high {
        return -(1 as libc::c_int);
    }
    if (*a).low > (*b).low {
        return 1 as libc::c_int;
    }
    if (*a).low < (*b).low {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn lexint_cmp64(mut a: *mut lexint_t, mut b: u64) -> libc::c_int {
    if (*a).high > 0 as libc::c_int as libc::c_ulonglong {
        return 1 as libc::c_int;
    }
    if (*a).low > b {
        return 1 as libc::c_int;
    }
    if (*a).low < b {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn lexint_shl(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    if b >= 128 as libc::c_int as libc::c_ulonglong {
        lexint_zero(dst);
    } else if b > 64 as libc::c_int as libc::c_ulonglong {
        (*dst).high = (*a).low << b.wrapping_sub(64 as libc::c_int as libc::c_ulonglong);
        (*dst).low = 0 as libc::c_int as u64;
    } else if b == 64 as libc::c_int as libc::c_ulonglong {
        (*dst).high = (*a).low;
        (*dst).low = 0 as libc::c_int as u64;
    } else if b > 0 as libc::c_int as libc::c_ulonglong {
        (*dst).high = ((*a).high << b)
            .wrapping_add((*a).low >> (64 as libc::c_int as libc::c_ulonglong).wrapping_sub(b));
        (*dst).low = (*a).low << b;
    } else {
        (*dst).high = (*a).high;
        (*dst).low = (*a).low;
    };
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn lexint_shr(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    if b >= 128 as libc::c_int as libc::c_ulonglong {
        lexint_zero(dst);
    } else if b > 64 as libc::c_int as libc::c_ulonglong {
        (*dst).low = (*a).high >> b.wrapping_sub(64 as libc::c_int as libc::c_ulonglong);
        (*dst).high = 0 as libc::c_int as u64;
    } else if b == 64 as libc::c_int as libc::c_ulonglong {
        (*dst).low = (*a).high;
        (*dst).high = 0 as libc::c_int as u64;
    } else if b > 0 as libc::c_int as libc::c_ulonglong {
        (*dst).low = ((*a).high << (64 as libc::c_int as libc::c_ulonglong).wrapping_sub(b))
            .wrapping_add((*a).low >> b);
        (*dst).high = (*a).high >> b;
    } else {
        (*dst).high = (*a).high;
        (*dst).low = (*a).low;
    };
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn lexint_testbit(mut a: *mut lexint_t, mut b: u8) -> u64 {
    if b as libc::c_int >= 64 as libc::c_int {
        return (*a).high >> b as libc::c_int - 64 as libc::c_int
            & 1 as libc::c_int as libc::c_ulonglong;
    }
    return (*a).low >> b as libc::c_int & 1 as libc::c_int as libc::c_ulonglong;
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn lexint_setbit(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u8) {
    *dst = *a;
    if b as libc::c_int >= 64 as libc::c_int {
        let ref mut fresh0 = (*dst).high;
        *fresh0 |= (1 as libc::c_int as u64) << b as libc::c_int - 64 as libc::c_int;
    } else {
        let ref mut fresh1 = (*dst).low;
        *fresh1 |= (1 as libc::c_int as u64) << b as libc::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn lexint_add(
    mut dst: *mut lexint_t,
    mut a: *mut lexint_t,
    mut b: *mut lexint_t,
) {
    (*dst).high = ((*a).high).wrapping_add((*b).high).wrapping_add(
        (((*a).low).wrapping_add((*b).low) < (*a).low) as libc::c_int as libc::c_ulonglong,
    );
    (*dst).low = ((*a).low).wrapping_add((*b).low);
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn lexint_add64(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    (*dst).high = ((*a).high)
        .wrapping_add((((*a).low).wrapping_add(b) < (*a).low) as libc::c_int as libc::c_ulonglong);
    (*dst).low = ((*a).low).wrapping_add(b);
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn lexint_sub(
    mut dst: *mut lexint_t,
    mut a: *mut lexint_t,
    mut b: *mut lexint_t,
) {
    (*dst).high = ((*a).high).wrapping_sub((*b).high).wrapping_sub(
        (((*a).low).wrapping_sub((*b).low) > (*a).low) as libc::c_int as libc::c_ulonglong,
    );
    (*dst).low = ((*a).low).wrapping_sub((*b).low);
}
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn lexint_sub64(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    (*dst).high = ((*a).high)
        .wrapping_sub((((*a).low).wrapping_sub(b) > (*a).low) as libc::c_int as libc::c_ulonglong);
    (*dst).low = ((*a).low).wrapping_sub(b);
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn lexint_mul64(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    let mut v1: u128 = ((*a).high as u128) << 64 as libc::c_int | (*a).low as u128;
    let mut v2: u128 = v1.wrapping_mul(b as u128);
    (*dst).low = v2 as u64;
    (*dst).high = (v2 >> 64 as libc::c_int) as u64;
}
#[no_mangle]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn lexint_div64(mut dst: *mut lexint_t, mut a: *mut lexint_t, mut b: u64) {
    let mut v1: u128 = ((*a).high as u128) << 64 as libc::c_int | (*a).low as u128;
    let mut v2: u128 = v1.wrapping_div(b as u128);
    (*dst).low = v2 as u64;
    (*dst).high = (v2 >> 64 as libc::c_int) as u64;
}
#[no_mangle]
#[c2rust::src_loc = "191:1"]
pub unsafe extern "C" fn lexint_char(mut i: *mut lexint_t, mut c: libc::c_int) {
    (*i).high = (*i).high << 8 as libc::c_int | (*i).low >> 56 as libc::c_int;
    (*i).low = (*i).low << 8 as libc::c_int | c as libc::c_ulonglong;
}
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn lexint_accum(mut i: *mut lexint_t, mut digit: u64, mut base: u64) -> bool {
    let mut v1: u128 = ((*i).high as u128) << 64 as libc::c_int | (*i).low as u128;
    let mut v2: u128 = v1.wrapping_mul(base as u128);
    if v2.wrapping_div(base as u128) != v1 {
        return 0 as libc::c_int != 0;
    }
    v2 = (v2 as u128).wrapping_add(digit as u128) as u128 as u128;
    if v2 < v1 {
        return 0 as libc::c_int != 0;
    }
    (*i).low = v2 as u64;
    (*i).high = (v2 >> 64 as libc::c_int) as u64;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "238:1"]
unsafe extern "C" fn count_leading_zeros(mut n: u64) -> libc::c_int {
    if n == 0 as libc::c_int as libc::c_ulonglong {
        return 64 as libc::c_int;
    }
    let mut count: libc::c_int = 0 as libc::c_int;
    if n >> 32 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 32 as libc::c_int;
        n <<= 32 as libc::c_int;
    }
    if n >> 48 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 16 as libc::c_int;
        n <<= 16 as libc::c_int;
    }
    if n >> 56 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 8 as libc::c_int;
        n <<= 8 as libc::c_int;
    }
    if n >> 60 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 4 as libc::c_int;
        n <<= 4 as libc::c_int;
    }
    if n >> 62 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 2 as libc::c_int;
        n <<= 2 as libc::c_int;
    }
    if n >> 63 as libc::c_int == 0 as libc::c_int as libc::c_ulonglong {
        count += 1 as libc::c_int;
        n <<= 1 as libc::c_int;
    }
    return count;
}
#[no_mangle]
#[c2rust::src_loc = "255:1"]
pub unsafe extern "C" fn lexint_double(mut i: *mut lexint_t) -> libc::c_double {
    if (*i).low == 0 as libc::c_int as libc::c_ulonglong
        && (*i).high == 0 as libc::c_int as libc::c_ulonglong
    {
        return 0 as libc::c_int as libc::c_double;
    }
    let mut sig_bit_count: libc::c_int = 128 as libc::c_int - count_leading_zeros((*i).high);
    if (*i).high == 0 as libc::c_int as libc::c_ulonglong {
        sig_bit_count = 64 as libc::c_int - count_leading_zeros((*i).low);
    }
    let mut exponent: u64 = (sig_bit_count - 1 as libc::c_int) as u64;
    let mut mantissa: u64 = (*i).low;
    if sig_bit_count <= 53 as libc::c_int {
        mantissa <<= 53 as libc::c_int - sig_bit_count;
    } else {
        if sig_bit_count == 54 as libc::c_int {
            mantissa <<= 1 as libc::c_int;
        } else if sig_bit_count > 55 as libc::c_int {
            let mut t: lexint_t = lexint_t { low: 0, high: 0 };
            lexint_shr(&mut t, i, (sig_bit_count - 55 as libc::c_int) as u64);
            mantissa = t.low;
            lexint_shl(&mut t, &mut t, (sig_bit_count - 55 as libc::c_int) as u64);
            if lexint_cmp(&mut t, i) != 0 as libc::c_int {
                mantissa |= 1 as libc::c_int as libc::c_ulonglong;
            }
        }
        if mantissa & 4 as libc::c_int as libc::c_ulonglong != 0 as libc::c_int as libc::c_ulonglong
        {
            mantissa |= 1 as libc::c_int as libc::c_ulonglong;
        }
        mantissa = mantissa.wrapping_add(1 as libc::c_int as libc::c_ulonglong) >> 2 as libc::c_int;
        if mantissa & (1 as libc::c_ulonglong) << 53 as libc::c_int
            != 0 as libc::c_int as libc::c_ulonglong
        {
            mantissa >>= 1 as libc::c_int;
            exponent = (exponent as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong) as u64
                as u64;
        }
    }
    let mut raw_bits: u64 = exponent.wrapping_add(1023 as libc::c_int as libc::c_ulonglong)
        << 52 as libc::c_int
        | mantissa & 0xfffffffffffff as libc::c_long as libc::c_ulonglong;
    let mut fp_bits: *mut libc::c_double = &mut raw_bits as *mut u64 as *mut libc::c_double;
    *fp_bits
}
