use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int16_t.h:1"]
pub mod _int16_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int16_t = libc::c_short;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:1"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = libc::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int64_t.h:1"]
pub mod _int64_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int64_t = libc::c_longlong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:1"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:1"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = libc::c_ushort;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:1"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "51:1"]
    pub type __darwin_intptr_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
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
        pub fn _Unwind_GetLanguageSpecificData(context: *mut _Unwind_Context) -> uintptr_t;
        #[c2rust::src_loc = "264:1"]
        pub fn _Unwind_GetRegionStart(context: *mut _Unwind_Context) -> uintptr_t;
        #[c2rust::src_loc = "220:1"]
        pub fn _Unwind_GetIP(context: *mut _Unwind_Context) -> uintptr_t;
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
pub use self::_int16_t_h::int16_t;
pub use self::_int32_t_h::int32_t;
pub use self::_int64_t_h::int64_t;
pub use self::_intptr_t_h::intptr_t;
pub use self::_types_h::__darwin_intptr_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::lsda_h::exception_context_t;
use self::stdlib_h::abort;
use self::unwind_h::{
    _Unwind_Context, _Unwind_GetIP, _Unwind_GetLanguageSpecificData, _Unwind_GetRegionStart,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "10:16"]
pub struct lsda_t {
    pub region_start: uintptr_t,
    pub ip: uintptr_t,
    pub ip_offset: uintptr_t,
    pub landing_pads: uintptr_t,
    pub type_table: *const uint8_t,
    pub call_site_table: *const uint8_t,
    pub action_table: *const uint8_t,
    pub type_table_encoding: uint8_t,
    pub call_site_encoding: uint8_t,
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
unsafe extern "C" fn read_sleb128(mut data: *mut *const uint8_t) -> intptr_t {
    let mut result: uintptr_t = 0 as libc::c_int as uintptr_t;
    let mut shift: uintptr_t = 0 as libc::c_int as uintptr_t;
    let mut byte: libc::c_uchar = 0;
    let mut p: *const uint8_t = *data;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        byte = *fresh0;
        result |= ((byte as libc::c_int & 0x7f as libc::c_int) << shift) as libc::c_ulong;
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as uintptr_t as uintptr_t;
        if !(byte as libc::c_int & 0x80 as libc::c_int != 0) {
            break;
        }
    }
    if byte as libc::c_int & 0x40 as libc::c_int != 0
        && shift < (::core::mem::size_of::<uintptr_t>() as libc::c_ulong) << 3 as libc::c_int
    {
        result |= !(0 as libc::c_int as uintptr_t) << shift;
    }
    *data = p;
    return result as intptr_t;
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn read_uleb128(mut data: *mut *const uint8_t) -> uintptr_t {
    let mut result: uintptr_t = 0 as libc::c_int as uintptr_t;
    let mut shift: uintptr_t = 0 as libc::c_int as uintptr_t;
    let mut byte: libc::c_uchar = 0;
    let mut p: *const uint8_t = *data;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        byte = *fresh1;
        result |= ((byte as libc::c_int & 0x7f as libc::c_int) << shift) as libc::c_ulong;
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as uintptr_t as uintptr_t;
        if !(byte as libc::c_int & 0x80 as libc::c_int != 0) {
            break;
        }
    }
    *data = p;
    return result;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn read_encoded_ptr(
    mut data: *mut *const uint8_t,
    mut encoding: uint8_t,
) -> uintptr_t {
    let mut p: *const uint8_t = *data;
    if encoding as libc::c_int == DW_EH_PE_omit as libc::c_int {
        return 0 as libc::c_int as uintptr_t;
    }
    let mut result: uintptr_t = 0;
    match encoding as libc::c_int & 0xf as libc::c_int {
        0 => {
            result = *(p as *mut uintptr_t);
            p = p.offset(::core::mem::size_of::<uintptr_t>() as libc::c_ulong as isize);
        }
        2 => {
            result = *(p as *mut uint16_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
        }
        3 => {
            result = *(p as *mut uint32_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
        }
        4 => {
            result = *(p as *mut uint64_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<uint64_t>() as libc::c_ulong as isize);
        }
        10 => {
            result = *(p as *mut int16_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<int16_t>() as libc::c_ulong as isize);
        }
        11 => {
            result = *(p as *mut int32_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<int32_t>() as libc::c_ulong as isize);
        }
        12 => {
            result = *(p as *mut int64_t) as uintptr_t;
            p = p.offset(::core::mem::size_of::<int64_t>() as libc::c_ulong as isize);
        }
        9 => {
            result = read_sleb128(&mut p) as uintptr_t;
        }
        1 => {
            result = read_uleb128(&mut p);
        }
        _ => {
            abort();
        }
    }
    *data = p;
    return result;
}
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn read_with_encoding(
    mut data: *mut *const uint8_t,
    mut def: uintptr_t,
) -> uintptr_t {
    let mut start: uintptr_t = *data as uintptr_t;
    let mut p: *const uint8_t = *data;
    let fresh2 = p;
    p = p.offset(1);
    let mut encoding: uint8_t = *fresh2;
    *data = p;
    if encoding as libc::c_int == DW_EH_PE_omit as libc::c_int {
        return def;
    }
    let mut result: uintptr_t = read_encoded_ptr(data, encoding);
    match encoding as libc::c_int & 0x70 as libc::c_int {
        0 => {}
        16 => {
            result = (result as libc::c_ulong).wrapping_add(start) as uintptr_t as uintptr_t;
        }
        32 | 48 | 64 | 80 | _ => {
            abort();
        }
    }
    if encoding as libc::c_int & DW_EH_PE_indirect as libc::c_int != 0 {
        result = *(result as *mut uintptr_t);
    }
    return result;
}
#[c2rust::src_loc = "186:1"]
unsafe extern "C" fn lsda_init(
    mut lsda: *mut lsda_t,
    mut context: *mut exception_context_t,
) -> bool {
    let mut data: *const uint8_t = _Unwind_GetLanguageSpecificData(context) as *const uint8_t;
    if data.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*lsda).region_start = _Unwind_GetRegionStart(context);
    (*lsda).ip = (_Unwind_GetIP(context)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*lsda).ip_offset = ((*lsda).ip).wrapping_sub((*lsda).region_start);
    (*lsda).landing_pads = read_with_encoding(&mut data, (*lsda).region_start);
    let fresh3 = data;
    data = data.offset(1);
    (*lsda).type_table_encoding = *fresh3;
    if (*lsda).type_table_encoding as libc::c_int != DW_EH_PE_omit as libc::c_int {
        let ref mut fresh4 = (*lsda).type_table;
        *fresh4 = read_uleb128(&mut data) as *const uint8_t;
        let ref mut fresh5 = (*lsda).type_table;
        *fresh5 = (*fresh5).offset(data as uintptr_t as isize);
    } else {
        let ref mut fresh6 = (*lsda).type_table;
        *fresh6 = 0 as *const uint8_t;
    }
    let fresh7 = data;
    data = data.offset(1);
    (*lsda).call_site_encoding = *fresh7;
    let mut length: uintptr_t = read_uleb128(&mut data);
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
    mut lp: *mut uintptr_t,
) -> bool {
    let mut lsda: lsda_t = lsda_t {
        region_start: 0,
        ip: 0,
        ip_offset: 0,
        landing_pads: 0,
        type_table: 0 as *const uint8_t,
        call_site_table: 0 as *const uint8_t,
        action_table: 0 as *const uint8_t,
        type_table_encoding: 0,
        call_site_encoding: 0,
    };
    if !lsda_init(&mut lsda, context) {
        return 0 as libc::c_int != 0;
    }
    let mut p: *const uint8_t = lsda.call_site_table;
    while p < lsda.action_table {
        let mut start: uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        let mut length: uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        let mut landing_pad: uintptr_t = read_encoded_ptr(&mut p, lsda.call_site_encoding);
        read_uleb128(&mut p);
        if start <= lsda.ip_offset && lsda.ip_offset < start.wrapping_add(length) {
            if landing_pad == 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int != 0;
            }
            *lp = (lsda.landing_pads).wrapping_add(landing_pad);
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}