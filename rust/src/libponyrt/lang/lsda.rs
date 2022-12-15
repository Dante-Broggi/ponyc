use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "51:1"]
    pub type __darwin_intptr_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h:1"]
pub mod _intptr_t_h {
    #[c2rust::src_loc = "32:1"]
    pub type intptr_t = __darwin_intptr_t;
    use super::_types_h::__darwin_intptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unwind.h:1"]
pub mod unwind_h {
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "56:16"]
        pub type _Unwind_Context;
        #[c2rust::src_loc = "265:1"]
        pub fn _Unwind_GetLanguageSpecificData(context: *mut _Unwind_Context) -> libc::uintptr_t;
        #[c2rust::src_loc = "264:1"]
        pub fn _Unwind_GetRegionStart(context: *mut _Unwind_Context) -> libc::uintptr_t;
        #[c2rust::src_loc = "220:1"]
        pub fn _Unwind_GetIP(context: *mut _Unwind_Context) -> libc::uintptr_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/lang/lsda.h:1"]
pub mod lsda_h {
    #[c2rust::src_loc = "16:1"]
    pub type exception_context_t = _Unwind_Context;
    use super::unwind_h::_Unwind_Context;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "131:7"]
        pub fn abort() -> !;
    }
}
pub use self::_intptr_t_h::intptr_t;
pub use self::_types_h::__darwin_intptr_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::lsda_h::exception_context_t;
use self::stdlib_h::abort;
use self::unwind_h::{_Unwind_GetIP, _Unwind_GetLanguageSpecificData, _Unwind_GetRegionStart};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "10:16"]
pub struct lsda_t {
    pub region_start: libc::uintptr_t,
    pub ip: libc::uintptr_t,
    pub ip_offset: libc::uintptr_t,
    pub landing_pads: libc::uintptr_t,
    pub type_table: *const u8,
    pub call_site_table: *const u8,
    pub action_table: *const u8,
    pub type_table_encoding: u8,
    pub call_site_encoding: u8,
}
#[c2rust::src_loc = "28:3"]
pub const DW_EH_PE_uleb128: C2RustUnnamed = 1;
#[c2rust::src_loc = "32:3"]
pub const DW_EH_PE_sleb128: C2RustUnnamed = 9;
#[c2rust::src_loc = "35:3"]
pub const DW_EH_PE_sdata8: C2RustUnnamed = 12;
#[c2rust::src_loc = "34:3"]
pub const DW_EH_PE_sdata4: C2RustUnnamed = 11;
#[c2rust::src_loc = "33:3"]
pub const DW_EH_PE_sdata2: C2RustUnnamed = 10;
#[c2rust::src_loc = "31:3"]
pub const DW_EH_PE_udata8: C2RustUnnamed = 4;
#[c2rust::src_loc = "30:3"]
pub const DW_EH_PE_udata4: C2RustUnnamed = 3;
#[c2rust::src_loc = "29:3"]
pub const DW_EH_PE_udata2: C2RustUnnamed = 2;
#[c2rust::src_loc = "27:3"]
pub const DW_EH_PE_absptr: C2RustUnnamed = 0;
#[c2rust::src_loc = "42:3"]
pub const DW_EH_PE_omit: C2RustUnnamed = 255;
#[c2rust::src_loc = "41:3"]
pub const DW_EH_PE_indirect: C2RustUnnamed = 128;
#[c2rust::src_loc = "40:3"]
pub const DW_EH_PE_aligned: C2RustUnnamed = 80;
#[c2rust::src_loc = "39:3"]
pub const DW_EH_PE_funcrel: C2RustUnnamed = 64;
#[c2rust::src_loc = "38:3"]
pub const DW_EH_PE_datarel: C2RustUnnamed = 48;
#[c2rust::src_loc = "37:3"]
pub const DW_EH_PE_textrel: C2RustUnnamed = 32;
#[c2rust::src_loc = "36:3"]
pub const DW_EH_PE_pcrel: C2RustUnnamed = 16;
#[c2rust::src_loc = "25:1"]
pub type C2RustUnnamed = libc::c_uint;
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn read_sleb128(mut data: *mut *const u8) -> intptr_t {
    let mut result: libc::uintptr_t = 0 as libc::c_int as libc::uintptr_t;
    let mut shift: libc::uintptr_t = 0 as libc::c_int as libc::uintptr_t;
    let mut byte: libc::c_uchar = 0;
    let mut p: *const u8 = *data;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        byte = *fresh0;
        result |= ((byte & 0x7f) << shift) as usize;
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as libc::uintptr_t as libc::uintptr_t;
        if !(byte as libc::c_int & 0x80 as libc::c_int != 0) {
            break;
        }
    }
    if byte as libc::c_int & 0x40 as libc::c_int != 0
        && shift < (::core::mem::size_of::<libc::uintptr_t>()) << 3 as libc::c_int
    {
        result |= !(0 as libc::c_int as libc::uintptr_t) << shift;
    }
    *data = p;
    result as intptr_t
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn read_uleb128(mut data: *mut *const u8) -> libc::uintptr_t {
    let mut result: libc::uintptr_t = 0 as libc::c_int as libc::uintptr_t;
    let mut shift: libc::uintptr_t = 0 as libc::c_int as libc::uintptr_t;
    let mut byte: libc::c_uchar = 0;
    let mut p: *const u8 = *data;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        byte = *fresh1;
        result |= ((byte & 0x7f) << shift) as usize;
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as libc::uintptr_t as libc::uintptr_t;
        if !(byte as libc::c_int & 0x80 as libc::c_int != 0) {
            break;
        }
    }
    *data = p;
    result
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn read_encoded_ptr(
    mut data: *mut *const u8,
    mut encoding: u8,
) -> libc::uintptr_t {
    let mut p: *const u8 = *data;
    if encoding as libc::c_int == DW_EH_PE_omit as libc::c_int {
        return 0 as libc::c_int as libc::uintptr_t;
    }
    let mut result: libc::uintptr_t = 0;
    match encoding as libc::c_int & 0xf as libc::c_int {
        0 => {
            result = *(p as *mut libc::uintptr_t);
            p = p.offset(::core::mem::size_of::<libc::uintptr_t>() as isize);
        }
        2 => {
            result = *(p as *mut u16) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<u16>() as isize);
        }
        3 => {
            result = *(p as *mut u32) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<u32>() as isize);
        }
        4 => {
            result = *(p as *mut u64) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<u64>() as isize);
        }
        10 => {
            result = *(p as *mut i16) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<i16>() as isize);
        }
        11 => {
            result = *(p as *mut i32) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<i32>() as isize);
        }
        12 => {
            result = *(p as *mut i64) as libc::uintptr_t;
            p = p.offset(::core::mem::size_of::<i64>() as isize);
        }
        9 => {
            result = read_sleb128(&mut p) as libc::uintptr_t;
        }
        1 => {
            result = read_uleb128(&mut p);
        }
        _ => {
            libc::abort();
        }
    }
    *data = p;
    result
}
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn read_with_encoding(
    mut data: *mut *const u8,
    mut def: libc::uintptr_t,
) -> libc::uintptr_t {
    let mut start: libc::uintptr_t = *data as libc::uintptr_t;
    let mut p: *const u8 = *data;
    let fresh2 = p;
    p = p.offset(1);
    let mut encoding: u8 = *fresh2;
    *data = p;
    if encoding as libc::c_int == DW_EH_PE_omit as libc::c_int {
        return def;
    }
    let mut result: libc::uintptr_t = read_encoded_ptr(data, encoding);
    match encoding as libc::c_int & 0x70 as libc::c_int {
        0 => {}
        16 => {
            result = (result as libc::c_ulong).wrapping_add(start.try_into().unwrap())
                as libc::uintptr_t as libc::uintptr_t;
        }
        32 | 48 | 64 | 80 | _ => {
            libc::abort();
        }
    }
    if encoding as libc::c_int & DW_EH_PE_indirect as libc::c_int != 0 {
        result = *(result as *mut libc::uintptr_t);
    }
    result
}
#[c2rust::src_loc = "186:1"]
unsafe extern "C" fn lsda_init(
    mut lsda: *mut lsda_t,
    mut context: *mut exception_context_t,
) -> bool {
    let mut data: *const u8 = _Unwind_GetLanguageSpecificData(context) as *const u8;
    if data.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*lsda).region_start = _Unwind_GetRegionStart(context);
    (*lsda).ip = (_Unwind_GetIP(context)).wrapping_sub(1);
    (*lsda).ip_offset = ((*lsda).ip).wrapping_sub((*lsda).region_start);
    (*lsda).landing_pads = read_with_encoding(&mut data, (*lsda).region_start);
    let fresh3 = data;
    data = data.offset(1);
    (*lsda).type_table_encoding = *fresh3;
    if (*lsda).type_table_encoding as libc::c_int != DW_EH_PE_omit as libc::c_int {
        let ref mut fresh4 = (*lsda).type_table;
        *fresh4 = read_uleb128(&mut data) as *const u8;
        let ref mut fresh5 = (*lsda).type_table;
        *fresh5 = (*fresh5).offset(data as libc::uintptr_t as isize);
    } else {
        let ref mut fresh6 = (*lsda).type_table;
        *fresh6 = 0 as *const u8;
    }
    let fresh7 = data;
    data = data.offset(1);
    (*lsda).call_site_encoding = *fresh7;
    let mut length: libc::uintptr_t = read_uleb128(&mut data);
    let ref mut fresh8 = (*lsda).call_site_table;
    *fresh8 = data;
    let ref mut fresh9 = (*lsda).action_table;
    *fresh9 = data.offset(length as isize);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "219:1"]
pub unsafe extern "C" fn ponyint_lsda_scan(
    mut context: *mut exception_context_t,
    mut lp: *mut libc::uintptr_t,
) -> bool {
    let mut lsda: lsda_t = lsda_t {
        region_start: 0,
        ip: 0,
        ip_offset: 0,
        landing_pads: 0,
        type_table: 0 as *const u8,
        call_site_table: 0 as *const u8,
        action_table: 0 as *const u8,
        type_table_encoding: 0,
        call_site_encoding: 0,
    };
    if !lsda_init(&mut lsda, context) {
        return 0 as libc::c_int != 0;
    }
    let mut p: *const u8 = lsda.call_site_table;
    while p < lsda.action_table {
        let mut start: libc::uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        let mut length: libc::uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        let mut landing_pad: libc::uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        read_uleb128(&mut p);
        if start <= lsda.ip_offset && lsda.ip_offset < start.wrapping_add(length) {
            if landing_pad == 0 {
                return 0 as libc::c_int != 0;
            }
            *lp = (lsda.landing_pads).wrapping_add(landing_pad);
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
