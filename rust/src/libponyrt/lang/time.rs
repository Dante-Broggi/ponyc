use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/llvm/src/clang/lib/Headers/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "74:1"]
    pub type wchar_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:1"]
pub mod _time_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:1"]
pub mod time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:8"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *mut libc::c_char,
    }
    use super::_time_t_h::time_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strftime(
            _: *mut libc::c_char,
            _: size_t,
            _: *const libc::c_char,
            _: *const tm,
        ) -> size_t;
        #[c2rust::src_loc = "127:1"]
        pub fn gmtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
        #[c2rust::src_loc = "138:1"]
        pub fn timegm(_: *mut tm) -> time_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:2"]
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
            size_t,
            libc::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> size_t>;
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
        pub traits: *mut *mut uintptr_t,
        pub fields: *mut libc::c_void,
        pub vtable: *mut libc::c_void,
    }
    #[c2rust::src_loc = "133:1"]
    pub type pony_type_t = _pony_type_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "183:1"]
        pub fn pony_ctx() -> *mut pony_ctx_t;
        #[c2rust::src_loc = "262:1"]
        pub fn pony_alloc(ctx: *mut pony_ctx_t, size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "508:1"]
        pub fn pony_error();
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:4"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_time_t_h::time_t;
pub use self::_types_h::__darwin_time_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_alloc, pony_ctx, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_error, pony_final_fn, pony_msg_t,
    pony_serialise_fn, pony_trace_fn, pony_type_t,
};
pub use self::stddef_h::{size_t, wchar_t};
use self::string_h::{memset, strcmp};
pub use self::time_h::{gmtime_r, strftime, timegm, tm};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "8:9"]
pub struct date_t {
    pub type_0: *const pony_type_t,
    pub nsec: libc::c_int,
    pub sec: libc::c_int,
    pub min: libc::c_int,
    pub hour: libc::c_int,
    pub day_of_month: libc::c_int,
    pub month: libc::c_int,
    pub year: libc::c_int,
    pub day_of_week: libc::c_int,
    pub day_of_year: libc::c_int,
}
#[c2rust::src_loc = "22:1"]
unsafe extern "C" fn date_to_tm(mut date: *mut date_t, mut tm: *mut tm) {
    memset(
        tm as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tm>() as libc::c_ulong,
    );
    (*tm).tm_sec = (*date).sec;
    (*tm).tm_min = (*date).min;
    (*tm).tm_hour = (*date).hour;
    (*tm).tm_mday = (*date).day_of_month;
    (*tm).tm_mon = (*date).month - 1 as libc::c_int;
    (*tm).tm_year = (*date).year - 1900 as libc::c_int;
    (*tm).tm_wday = (*date).day_of_week;
    (*tm).tm_yday = (*date).day_of_year;
    (*tm).tm_isdst = -(1 as libc::c_int);
}
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn tm_to_date(mut tm: *mut tm, mut nsec: libc::c_int, mut date: *mut date_t) {
    (*date).nsec = nsec;
    (*date).sec = (*tm).tm_sec;
    (*date).min = (*tm).tm_min;
    (*date).hour = (*tm).tm_hour;
    (*date).day_of_month = (*tm).tm_mday;
    (*date).month = (*tm).tm_mon + 1 as libc::c_int;
    (*date).year = (*tm).tm_year + 1900 as libc::c_int;
    (*date).day_of_week = (*tm).tm_wday;
    (*date).day_of_year = (*tm).tm_yday;
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn ponyint_timegm(mut date: *mut date_t) -> i64 {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *mut libc::c_char,
    };
    date_to_tm(date, &mut tm);
    return timegm(&mut tm) as i64;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn ponyint_gmtime(
    mut date: *mut date_t,
    mut sec: i64,
    mut nsec: i64,
) {
    let mut overflow_sec: time_t = (nsec / 1000000000 as libc::c_int as libc::c_longlong) as time_t;
    nsec -= (overflow_sec * 1000000000 as libc::c_int as libc::c_long) as libc::c_longlong;
    if nsec < 0 as libc::c_int as libc::c_longlong {
        nsec += 1000000000 as libc::c_int as libc::c_longlong;
        overflow_sec -= 1;
    }
    let mut t: time_t = sec as time_t + overflow_sec;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *mut libc::c_char,
    };
    gmtime_r(&mut t, &mut tm);
    tm_to_date(&mut tm, nsec as libc::c_int, date);
}
#[no_mangle]
#[c2rust::src_loc = "86:1"]
pub unsafe extern "C" fn format_invalid_parameter_handler(
    mut _expression: *const wchar_t,
    mut _function: *const wchar_t,
    mut _file: *const wchar_t,
    mut _line: libc::c_uint,
    mut _p_reserved: uintptr_t,
) {
    pony_error();
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn ponyint_formattime(
    mut date: *mut date_t,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ctx: *mut pony_ctx_t = pony_ctx();
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || strcmp(fmt, b"%p\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(fmt, b"%P\0" as *const u8 as *const libc::c_char) == 0
    {
        buffer = pony_alloc(ctx, 1 as libc::c_int as size_t) as *mut libc::c_char;
        *buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return buffer;
    }
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *mut libc::c_char,
    };
    date_to_tm(date, &mut tm);
    let mut len: size_t = 64 as libc::c_int as size_t;
    let mut r: size_t = 0;
    while r == 0 as libc::c_int as libc::c_ulong {
        buffer = pony_alloc(ctx, len) as *mut libc::c_char;
        r = strftime(buffer, len, fmt, &mut tm);
        len <<= 1 as libc::c_int;
    }
    buffer
}
