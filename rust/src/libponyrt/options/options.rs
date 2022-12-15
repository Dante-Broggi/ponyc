use ::libc;
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/options/options.h:1"]
pub mod options_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct opt_arg_t {
        pub long_opt: *const libc::c_char,
        pub short_opt: libc::c_char,
        pub flag: u32,
        pub id: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:16"]
    pub struct opt_state_t {
        pub args: *const opt_arg_t,
        pub argc: *mut libc::c_int,
        pub argv: *mut *mut libc::c_char,
        pub arg_val: *mut libc::c_char,
        pub opt_start: *mut libc::c_char,
        pub opt_end: *mut libc::c_char,
        pub match_type: libc::c_int,
        pub idx: libc::c_int,
        pub remove: libc::c_int,
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:3"]
pub mod ponyassert_h {
    use super::stddef_h::size_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:4"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "73:7"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "84:6"]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:5"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
pub use self::options_h::{opt_arg_t, opt_state_t};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::stddef_h::size_t;
use self::stdio_h::printf;
use self::string_h::{memmove, strlen, strncmp};
#[c2rust::src_loc = "13:1"]
unsafe extern "C" fn end_reached(mut arg: *const opt_arg_t) -> bool {
    return ((*arg).long_opt).is_null()
        && (*arg).short_opt as libc::c_int == 0 as libc::c_int
        && (*arg).id == 4294967295 as libc::c_uint
        && (*arg).flag == 4294967295 as libc::c_uint;
}
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn has_argument(mut s: *mut opt_state_t) -> bool {
    let mut short_arg: bool = (*s).match_type == 2 as libc::c_int
        && (*((*s).opt_start).offset(1 as libc::c_int as isize) as libc::c_int != 0
            || (*s).idx < *(*s).argc - 1 as libc::c_int);
    let mut long_arg: bool = (*s).match_type == 1 as libc::c_int
        && (*(*s).opt_end as libc::c_int == '=' as i32 || (*s).idx < *(*s).argc - 1 as libc::c_int);
    return short_arg as libc::c_int | long_arg as libc::c_int != 0;
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn parse_option_name(mut s: *mut opt_state_t) {
    let mut is_long_opt: bool = *(*((*s).argv).offset((*s).idx as isize))
        .offset(1 as libc::c_int as isize) as libc::c_int
        == '-' as i32;
    (*s).match_type = if is_long_opt as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let ref mut fresh0 = (*s).opt_start;
    *fresh0 = (*((*s).argv).offset((*s).idx as isize))
        .offset(1 as libc::c_int as isize)
        .offset(is_long_opt as libc::c_int as isize);
    let ref mut fresh1 = (*s).opt_end;
    *fresh1 = (*s).opt_start;
    while *(*s).opt_end as libc::c_int != 0 && *(*s).opt_end as libc::c_int != '=' as i32 {
        let ref mut fresh2 = (*s).opt_end;
        *fresh2 = (*fresh2).offset(1);
    }
}
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn find_match(mut s: *mut opt_state_t) -> *const opt_arg_t {
    let mut match_length: usize = 0;
    let mut match_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut match_0: *const opt_arg_t = 0 as *const opt_arg_t;
    parse_option_name(s);
    let mut p: *const opt_arg_t = (*s).args;
    while !end_reached(p) {
        if (*s).match_type == 1 as libc::c_int {
            match_name = (*p).long_opt;
            match_length = ((*s).opt_end).offset_from((*s).opt_start) as libc::c_long as usize;
        } else {
            match_name = &(*p).short_opt;
            match_length = 1 as libc::c_int as usize;
        }
        if strncmp(match_name, (*s).opt_start, match_length.try_into().unwrap()) == 0 {
            if (*s).match_type == 2 as libc::c_int || match_length == libc::strlen(match_name) {
                if !match_0.is_null() && (*match_0).id != (*p).id {
                    return 1 as libc::c_int as *mut opt_arg_t;
                }
                match_0 = p;
            }
        }
        p = p.offset(1);
    }
    match_0
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn is_positional(mut s: *const opt_state_t) -> bool {
    return *(*((*s).argv).offset((*s).idx as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int
        != '-' as i32
        || *(*((*s).argv).offset((*s).idx as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int
            == '\0' as i32;
}
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn skip_non_options(mut s: *mut opt_state_t) -> bool {
    loop {
        if (*s).idx == *(*s).argc {
            return 0 as libc::c_int != 0;
        }
        if !is_positional(s) {
            return 1 as libc::c_int != 0;
        }
        let ref mut fresh3 = (*s).idx;
        *fresh3 += 1;
    }
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn strip_accepted_opts(mut s: *mut opt_state_t) {
    if (*s).remove > 0 as libc::c_int {
        *(*s).argc -= (*s).remove;
        if *(*s).argc >= (*s).idx {
        } else {
            ponyint_assert_fail(
                b"*s->argc >= s->idx\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/options/options.c\0"
                    as *const u8 as *const libc::c_char,
                107 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"strip_accepted_opts\0",
                ))
                .as_ptr(),
            );
        };
        memmove(
            &mut *((*s).argv).offset((*s).idx as isize) as *mut *mut libc::c_char
                as *mut libc::c_void,
            &mut *((*s).argv).offset(((*s).idx + (*s).remove) as isize) as *mut *mut libc::c_char
                as *const libc::c_void,
            ((*(*s).argc - (*s).idx) as libc::c_uint as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        );
        let ref mut fresh4 = (*s).idx;
        *fresh4 -= 1;
        (*s).remove = 0 as libc::c_int;
    }
}
#[c2rust::src_loc = "117:1"]
unsafe extern "C" fn parse_long_opt_arg(mut s: *mut opt_state_t) {
    if *(*s).opt_end as libc::c_int == '=' as i32 {
        let ref mut fresh5 = (*s).arg_val;
        *fresh5 = ((*s).opt_end).offset(1 as libc::c_int as isize);
        let ref mut fresh6 = (*s).opt_start;
        *fresh6 = (*fresh6).offset(libc::strlen((*s).opt_start) as isize);
    } else if *(*((*s).argv).offset(((*s).idx + 1 as libc::c_int) as isize))
        .offset(0 as libc::c_int as isize) as libc::c_int
        != '-' as i32
    {
        let ref mut fresh7 = (*s).arg_val;
        *fresh7 = *((*s).argv).offset(((*s).idx + 1 as libc::c_int) as isize);
        let ref mut fresh8 = (*s).opt_start;
        *fresh8 = (*fresh8).offset(libc::strlen((*s).opt_start) as isize);
        if *(*((*s).argv).offset(((*s).idx + 1 as libc::c_int) as isize))
            .offset(0 as libc::c_int as isize)
            != 0
        {
            let ref mut fresh9 = (*s).remove;
            *fresh9 += 1;
        }
    }
}
#[c2rust::src_loc = "135:1"]
unsafe extern "C" fn parse_short_opt_arg(mut s: *mut opt_state_t) {
    if *(*s).opt_end != 0 {
        let ref mut fresh10 = (*s).arg_val;
        *fresh10 = (*s).opt_end;
        let ref mut fresh11 = (*s).opt_start;
        *fresh11 = (*fresh11).offset(libc::strlen((*s).opt_start) as isize);
    } else if *(*s).opt_start as libc::c_int != '-' as i32 {
        let ref mut fresh12 = (*s).arg_val;
        *fresh12 = *((*s).argv).offset(((*s).idx + 1 as libc::c_int) as isize);
    } else {
        let ref mut fresh13 = (*s).arg_val;
        *fresh13 = ((*s).opt_start).offset(1 as libc::c_int as isize);
        let ref mut fresh14 = (*s).opt_start;
        *fresh14 = (*fresh14).offset(libc::strlen((*s).opt_start) as isize);
    }
    let ref mut fresh15 = (*s).remove;
    *fresh15 += 1;
}
#[c2rust::src_loc = "155:1"]
unsafe extern "C" fn parse_short_opt(mut s: *mut opt_state_t) {
    memmove(
        (*s).opt_start as *mut libc::c_void,
        ((*s).opt_start).offset(1 as libc::c_int as isize) as *const libc::c_void,
        libc::strlen((*s).opt_start).try_into().unwrap(),
    );
    if *(*s).opt_start != 0 {
        let ref mut fresh16 = (*s).opt_start;
        *fresh16 = *((*s).argv).offset((*s).idx as isize);
    } else {
        let ref mut fresh17 = (*s).remove;
        *fresh17 += 1;
    };
}
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn missing_argument(mut s: *mut opt_state_t) -> libc::c_int {
    printf(
        b"%s: '%s' option requires an argument!\n\0" as *const u8 as *const libc::c_char,
        *((*s).argv).offset(0 as libc::c_int as isize),
        *((*s).argv).offset((*s).idx as isize),
    );
    return -(2 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn ponyint_opt_init(
    mut args: *const opt_arg_t,
    mut s: *mut opt_state_t,
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let ref mut fresh18 = (*s).argc;
    *fresh18 = argc;
    let ref mut fresh19 = (*s).argv;
    *fresh19 = argv;
    let ref mut fresh20 = (*s).args;
    *fresh20 = args;
    let ref mut fresh21 = (*s).arg_val;
    *fresh21 = 0 as *mut libc::c_char;
    let ref mut fresh22 = (*s).opt_start;
    *fresh22 = 0 as *mut libc::c_char;
    let ref mut fresh23 = (*s).opt_end;
    *fresh23 = 0 as *mut libc::c_char;
    (*s).match_type = 0 as libc::c_int;
    (*s).idx = 0 as libc::c_int;
    (*s).remove = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "190:1"]
pub unsafe extern "C" fn ponyint_opt_next(mut s: *mut opt_state_t) -> libc::c_int {
    let ref mut fresh24 = (*s).arg_val;
    *fresh24 = 0 as *mut libc::c_char;
    if ((*s).opt_start).is_null() || *(*s).opt_start as libc::c_int == '\0' as i32 {
        let ref mut fresh25 = (*s).idx;
        *fresh25 += 1;
        if !skip_non_options(s) {
            return -(1 as libc::c_int);
        }
    }
    let mut m: *const opt_arg_t = find_match(s);
    if m.is_null() {
        let ref mut fresh26 = (*s).opt_start;
        *fresh26 = (*fresh26).offset(libc::strlen((*s).opt_start) as isize);
        return ponyint_opt_next(s);
    } else {
        if m == 1 as libc::c_int as *mut opt_arg_t as *const opt_arg_t {
            printf(
                b"%s: '%s' option is ambiguous!\n\0" as *const u8 as *const libc::c_char,
                *((*s).argv).offset(0 as libc::c_int as isize),
                *((*s).argv).offset((*s).idx as isize),
            );
            return -(2 as libc::c_int);
        }
    }
    if (*m).flag == ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint && !has_argument(s) {
        return missing_argument(s);
    }
    if (*s).match_type == 1 as libc::c_int {
        let ref mut fresh27 = (*s).remove;
        *fresh27 += 1;
        if (*m).flag
            & ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_uint
            != 0
            && has_argument(s) as libc::c_int != 0
        {
            parse_long_opt_arg(s);
            if ((*s).arg_val).is_null()
                && (*m).flag & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
            {
                return missing_argument(s);
            }
        }
        let ref mut fresh28 = (*s).opt_start;
        *fresh28 = 0 as *mut libc::c_char;
    } else if (*s).match_type == 2 as libc::c_int {
        parse_short_opt(s);
        if (*m).flag
            & ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_uint
            != 0
            && has_argument(s) as libc::c_int != 0
        {
            parse_short_opt_arg(s);
            if ((*s).arg_val).is_null()
                && (*m).flag & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
            {
                return missing_argument(s);
            }
        }
    }
    strip_accepted_opts(s);
    (*m).id as libc::c_int
}
