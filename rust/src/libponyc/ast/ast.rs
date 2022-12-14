use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[c2rust::src_loc = "100:1"]
    pub type __darwin_va_list = __builtin_va_list;
    #[c2rust::src_loc = "121:1"]
    pub type __darwin_ssize_t = libc::c_long;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:1"]
pub mod _va_list_h {
    #[c2rust::src_loc = "32:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:1"]
pub mod _stdio_h {
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:16"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
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
    #[c2rust::src_loc = "124:1"]
    pub type pony_final_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "114:1"]
    pub type pony_dispatch_fn =
        Option<unsafe extern "C" fn(*mut pony_ctx_t, *mut pony_actor_t, *mut pony_msg_t) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct pony_msg_t {
        pub index: u32,
        pub id: u32,
        pub next: *mut pony_msg_t,
    }
    #[c2rust::src_loc = "105:1"]
    pub type pony_custom_deserialise_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> size_t>;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
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
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed = 0;
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "30:16"]
        pub type pony_actor_t;
        #[c2rust::src_loc = "36:16"]
        pub type pony_ctx_t;
        #[c2rust::src_loc = "394:1"]
        pub fn pony_traceknown(
            ctx: *mut pony_ctx_t,
            p: *mut libc::c_void,
            t: *const pony_type_t,
            m: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[c2rust::src_loc = "9:1"]
    pub type pony_type_t = _pony_type_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: size_t,
    }
    use super::_size_t_h::size_t;
    use super::pony_h::_pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn source_close(source: *mut source_t);
        #[c2rust::src_loc = "37:1"]
        pub fn source_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct errormsg_t {
        pub file: *const libc::c_char,
        pub line: size_t,
        pub pos: size_t,
        pub msg: *const libc::c_char,
        pub source: *const libc::c_char,
        pub frame: *mut errormsg_t,
        pub next: *mut errormsg_t,
    }
    #[c2rust::src_loc = "49:1"]
    pub type errorframe_t = *mut errormsg_t;
    use super::_size_t_h::size_t;

    use super::source_h::source_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "75:1"]
        pub fn errorv(
            errors: *mut errors_t,
            source: *mut source_t,
            line: size_t,
            pos: size_t,
            fmt: *const libc::c_char,
            ap: ::core::ffi::VaList,
        );
        #[c2rust::src_loc = "83:1"]
        pub fn errorv_continue(
            errors: *mut errors_t,
            source: *mut source_t,
            line: size_t,
            pos: size_t,
            fmt: *const libc::c_char,
            ap: ::core::ffi::VaList,
        );
        #[c2rust::src_loc = "91:1"]
        pub fn errorframev(
            frame: *mut errorframe_t,
            source: *mut source_t,
            line: size_t,
            pos: size_t,
            fmt: *const libc::c_char,
            ap: ::core::ffi::VaList,
        );
    }
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.h:1"]
pub mod token_h {
    #[c2rust::src_loc = "20:9"]
    pub type token_id = libc::c_uint;
    #[c2rust::src_loc = "275:3"]
    pub const TK_TEST_EXTRA: token_id = 214;
    #[c2rust::src_loc = "274:3"]
    pub const TK_TEST_UPDATEARG: token_id = 213;
    #[c2rust::src_loc = "273:3"]
    pub const TK_TEST_ALIASED: token_id = 212;
    #[c2rust::src_loc = "272:3"]
    pub const TK_TEST_TRY_NO_CHECK: token_id = 211;
    #[c2rust::src_loc = "271:3"]
    pub const TK_TEST_SEQ_SCOPE: token_id = 210;
    #[c2rust::src_loc = "270:3"]
    pub const TK_TEST_NO_SEQ: token_id = 209;
    #[c2rust::src_loc = "267:3"]
    pub const TK_FLATTEN: token_id = 208;
    #[c2rust::src_loc = "266:3"]
    pub const TK_NEWLINE: token_id = 207;
    #[c2rust::src_loc = "263:3"]
    pub const TK_DISPOSING_BLOCK: token_id = 206;
    #[c2rust::src_loc = "261:3"]
    pub const TK_ANNOTATION: token_id = 205;
    #[c2rust::src_loc = "259:3"]
    pub const TK_FUNCHAIN: token_id = 204;
    #[c2rust::src_loc = "258:3"]
    pub const TK_BECHAIN: token_id = 203;
    #[c2rust::src_loc = "257:3"]
    pub const TK_FUNAPP: token_id = 202;
    #[c2rust::src_loc = "256:3"]
    pub const TK_BEAPP: token_id = 201;
    #[c2rust::src_loc = "255:3"]
    pub const TK_NEWAPP: token_id = 200;
    #[c2rust::src_loc = "254:3"]
    pub const TK_DONTCAREREF: token_id = 199;
    #[c2rust::src_loc = "253:3"]
    pub const TK_PARAMREF: token_id = 198;
    #[c2rust::src_loc = "252:3"]
    pub const TK_LETREF: token_id = 197;
    #[c2rust::src_loc = "251:3"]
    pub const TK_VARREF: token_id = 196;
    #[c2rust::src_loc = "250:3"]
    pub const TK_TUPLEELEMREF: token_id = 195;
    #[c2rust::src_loc = "249:3"]
    pub const TK_EMBEDREF: token_id = 194;
    #[c2rust::src_loc = "248:3"]
    pub const TK_FLETREF: token_id = 193;
    #[c2rust::src_loc = "247:3"]
    pub const TK_FVARREF: token_id = 192;
    #[c2rust::src_loc = "246:3"]
    pub const TK_FUNREF: token_id = 191;
    #[c2rust::src_loc = "245:3"]
    pub const TK_BEREF: token_id = 190;
    #[c2rust::src_loc = "244:3"]
    pub const TK_NEWBEREF: token_id = 189;
    #[c2rust::src_loc = "243:3"]
    pub const TK_NEWREF: token_id = 188;
    #[c2rust::src_loc = "242:3"]
    pub const TK_TYPEPARAMREF: token_id = 187;
    #[c2rust::src_loc = "241:3"]
    pub const TK_TYPEREF: token_id = 186;
    #[c2rust::src_loc = "240:3"]
    pub const TK_PACKAGEREF: token_id = 185;
    #[c2rust::src_loc = "239:3"]
    pub const TK_REFERENCE: token_id = 184;
    #[c2rust::src_loc = "237:3"]
    pub const TK_MATCH_DONTCARE: token_id = 183;
    #[c2rust::src_loc = "236:3"]
    pub const TK_MATCH_CAPTURE: token_id = 182;
    #[c2rust::src_loc = "235:3"]
    pub const TK_CASE: token_id = 181;
    #[c2rust::src_loc = "234:3"]
    pub const TK_CASES: token_id = 180;
    #[c2rust::src_loc = "233:3"]
    pub const TK_ARRAY: token_id = 179;
    #[c2rust::src_loc = "232:3"]
    pub const TK_TUPLE: token_id = 178;
    #[c2rust::src_loc = "231:3"]
    pub const TK_CALL: token_id = 177;
    #[c2rust::src_loc = "230:3"]
    pub const TK_QUALIFY: token_id = 176;
    #[c2rust::src_loc = "229:3"]
    pub const TK_SEQ: token_id = 175;
    #[c2rust::src_loc = "227:3"]
    pub const TK_LAMBDACAPTURE: token_id = 174;
    #[c2rust::src_loc = "226:3"]
    pub const TK_LAMBDACAPTURES: token_id = 173;
    #[c2rust::src_loc = "225:3"]
    pub const TK_UPDATEARG: token_id = 172;
    #[c2rust::src_loc = "224:3"]
    pub const TK_NAMEDARG: token_id = 171;
    #[c2rust::src_loc = "223:3"]
    pub const TK_NAMEDARGS: token_id = 170;
    #[c2rust::src_loc = "222:3"]
    pub const TK_POSITIONALARGS: token_id = 169;
    #[c2rust::src_loc = "221:3"]
    pub const TK_VALUEFORMALARG: token_id = 168;
    #[c2rust::src_loc = "220:3"]
    pub const TK_VALUEFORMALPARAM: token_id = 167;
    #[c2rust::src_loc = "219:3"]
    pub const TK_TYPEARGS: token_id = 166;
    #[c2rust::src_loc = "218:3"]
    pub const TK_PARAM: token_id = 165;
    #[c2rust::src_loc = "217:3"]
    pub const TK_PARAMS: token_id = 164;
    #[c2rust::src_loc = "216:3"]
    pub const TK_TYPEPARAM: token_id = 163;
    #[c2rust::src_loc = "215:3"]
    pub const TK_TYPEPARAMS: token_id = 162;
    #[c2rust::src_loc = "213:3"]
    pub const TK_OPERATORLITERAL: token_id = 161;
    #[c2rust::src_loc = "212:3"]
    pub const TK_LITERALBRANCH: token_id = 160;
    #[c2rust::src_loc = "211:3"]
    pub const TK_LITERAL: token_id = 159;
    #[c2rust::src_loc = "209:3"]
    pub const TK_ERRORTYPE: token_id = 158;
    #[c2rust::src_loc = "208:3"]
    pub const TK_INFERTYPE: token_id = 157;
    #[c2rust::src_loc = "207:3"]
    pub const TK_DONTCARETYPE: token_id = 156;
    #[c2rust::src_loc = "206:3"]
    pub const TK_BARELAMBDATYPE: token_id = 155;
    #[c2rust::src_loc = "205:3"]
    pub const TK_LAMBDATYPE: token_id = 154;
    #[c2rust::src_loc = "204:3"]
    pub const TK_FUNTYPE: token_id = 153;
    #[c2rust::src_loc = "203:3"]
    pub const TK_THISTYPE: token_id = 152;
    #[c2rust::src_loc = "202:3"]
    pub const TK_NOMINAL: token_id = 151;
    #[c2rust::src_loc = "201:3"]
    pub const TK_TUPLETYPE: token_id = 150;
    #[c2rust::src_loc = "200:3"]
    pub const TK_UNIONTYPE: token_id = 149;
    #[c2rust::src_loc = "199:3"]
    pub const TK_PROVIDES: token_id = 148;
    #[c2rust::src_loc = "197:3"]
    pub const TK_IFDEFFLAG: token_id = 147;
    #[c2rust::src_loc = "196:3"]
    pub const TK_IFDEFNOT: token_id = 146;
    #[c2rust::src_loc = "195:3"]
    pub const TK_IFDEFOR: token_id = 145;
    #[c2rust::src_loc = "194:3"]
    pub const TK_IFDEFAND: token_id = 144;
    #[c2rust::src_loc = "192:3"]
    pub const TK_FFICALL: token_id = 143;
    #[c2rust::src_loc = "191:3"]
    pub const TK_FFIDECL: token_id = 142;
    #[c2rust::src_loc = "190:3"]
    pub const TK_FLET: token_id = 141;
    #[c2rust::src_loc = "189:3"]
    pub const TK_FVAR: token_id = 140;
    #[c2rust::src_loc = "188:3"]
    pub const TK_MEMBERS: token_id = 139;
    #[c2rust::src_loc = "186:3"]
    pub const TK_MODULE: token_id = 138;
    #[c2rust::src_loc = "185:3"]
    pub const TK_PACKAGE: token_id = 137;
    #[c2rust::src_loc = "184:3"]
    pub const TK_PROGRAM: token_id = 136;
    #[c2rust::src_loc = "181:3"]
    pub const TK_LOCATION: token_id = 135;
    #[c2rust::src_loc = "180:3"]
    pub const TK_ADDRESS: token_id = 134;
    #[c2rust::src_loc = "179:3"]
    pub const TK_DIGESTOF: token_id = 133;
    #[c2rust::src_loc = "177:3"]
    pub const TK_XOR: token_id = 132;
    #[c2rust::src_loc = "176:3"]
    pub const TK_OR: token_id = 131;
    #[c2rust::src_loc = "175:3"]
    pub const TK_AND: token_id = 130;
    #[c2rust::src_loc = "174:3"]
    pub const TK_NOT: token_id = 129;
    #[c2rust::src_loc = "172:3"]
    pub const TK_COMPILE_ERROR: token_id = 128;
    #[c2rust::src_loc = "171:3"]
    pub const TK_ERROR: token_id = 127;
    #[c2rust::src_loc = "170:3"]
    pub const TK_WITH: token_id = 126;
    #[c2rust::src_loc = "169:3"]
    pub const TK_TRY_NO_CHECK: token_id = 125;
    #[c2rust::src_loc = "168:3"]
    pub const TK_TRY: token_id = 124;
    #[c2rust::src_loc = "167:3"]
    pub const TK_WHERE: token_id = 123;
    #[c2rust::src_loc = "166:3"]
    pub const TK_MATCH: token_id = 122;
    #[c2rust::src_loc = "165:3"]
    pub const TK_IN: token_id = 121;
    #[c2rust::src_loc = "164:3"]
    pub const TK_FOR: token_id = 120;
    #[c2rust::src_loc = "163:3"]
    pub const TK_UNTIL: token_id = 119;
    #[c2rust::src_loc = "162:3"]
    pub const TK_REPEAT: token_id = 118;
    #[c2rust::src_loc = "161:3"]
    pub const TK_DO: token_id = 117;
    #[c2rust::src_loc = "160:3"]
    pub const TK_WHILE: token_id = 116;
    #[c2rust::src_loc = "159:3"]
    pub const TK_END: token_id = 115;
    #[c2rust::src_loc = "158:3"]
    pub const TK_ELSEIF: token_id = 114;
    #[c2rust::src_loc = "157:3"]
    pub const TK_ELSE: token_id = 113;
    #[c2rust::src_loc = "156:3"]
    pub const TK_THEN: token_id = 112;
    #[c2rust::src_loc = "155:3"]
    pub const TK_IFTYPE_SET: token_id = 111;
    #[c2rust::src_loc = "154:3"]
    pub const TK_IFTYPE: token_id = 110;
    #[c2rust::src_loc = "153:3"]
    pub const TK_IFDEF: token_id = 109;
    #[c2rust::src_loc = "152:3"]
    pub const TK_IF: token_id = 108;
    #[c2rust::src_loc = "150:3"]
    pub const TK_RECOVER: token_id = 107;
    #[c2rust::src_loc = "149:3"]
    pub const TK_CONSUME: token_id = 106;
    #[c2rust::src_loc = "148:3"]
    pub const TK_CONTINUE: token_id = 105;
    #[c2rust::src_loc = "147:3"]
    pub const TK_BREAK: token_id = 104;
    #[c2rust::src_loc = "146:3"]
    pub const TK_RETURN: token_id = 103;
    #[c2rust::src_loc = "145:3"]
    pub const TK_THIS: token_id = 102;
    #[c2rust::src_loc = "143:3"]
    pub const TK_CAP_ANY: token_id = 101;
    #[c2rust::src_loc = "142:3"]
    pub const TK_CAP_ALIAS: token_id = 100;
    #[c2rust::src_loc = "141:3"]
    pub const TK_CAP_SHARE: token_id = 99;
    #[c2rust::src_loc = "140:3"]
    pub const TK_CAP_SEND: token_id = 98;
    #[c2rust::src_loc = "139:3"]
    pub const TK_CAP_READ: token_id = 97;
    #[c2rust::src_loc = "137:3"]
    pub const TK_TAG: token_id = 96;
    #[c2rust::src_loc = "136:3"]
    pub const TK_BOX: token_id = 95;
    #[c2rust::src_loc = "135:3"]
    pub const TK_VAL: token_id = 94;
    #[c2rust::src_loc = "134:3"]
    pub const TK_REF: token_id = 93;
    #[c2rust::src_loc = "133:3"]
    pub const TK_TRN: token_id = 92;
    #[c2rust::src_loc = "132:3"]
    pub const TK_ISO: token_id = 91;
    #[c2rust::src_loc = "130:3"]
    pub const TK_BE: token_id = 90;
    #[c2rust::src_loc = "129:3"]
    pub const TK_FUN: token_id = 89;
    #[c2rust::src_loc = "128:3"]
    pub const TK_NEW: token_id = 88;
    #[c2rust::src_loc = "127:3"]
    pub const TK_DONTCARE: token_id = 87;
    #[c2rust::src_loc = "126:3"]
    pub const TK_EMBED: token_id = 86;
    #[c2rust::src_loc = "125:3"]
    pub const TK_LET: token_id = 85;
    #[c2rust::src_loc = "124:3"]
    pub const TK_VAR: token_id = 84;
    #[c2rust::src_loc = "122:3"]
    pub const TK_ISNT: token_id = 83;
    #[c2rust::src_loc = "121:3"]
    pub const TK_IS: token_id = 82;
    #[c2rust::src_loc = "120:3"]
    pub const TK_AS: token_id = 81;
    #[c2rust::src_loc = "118:3"]
    pub const TK_BARELAMBDA: token_id = 80;
    #[c2rust::src_loc = "117:3"]
    pub const TK_LAMBDA: token_id = 79;
    #[c2rust::src_loc = "116:3"]
    pub const TK_OBJECT: token_id = 78;
    #[c2rust::src_loc = "115:3"]
    pub const TK_ACTOR: token_id = 77;
    #[c2rust::src_loc = "114:3"]
    pub const TK_CLASS: token_id = 76;
    #[c2rust::src_loc = "113:3"]
    pub const TK_STRUCT: token_id = 75;
    #[c2rust::src_loc = "112:3"]
    pub const TK_PRIMITIVE: token_id = 74;
    #[c2rust::src_loc = "111:3"]
    pub const TK_TRAIT: token_id = 73;
    #[c2rust::src_loc = "110:3"]
    pub const TK_INTERFACE: token_id = 72;
    #[c2rust::src_loc = "109:3"]
    pub const TK_TYPE: token_id = 71;
    #[c2rust::src_loc = "108:3"]
    pub const TK_USE: token_id = 70;
    #[c2rust::src_loc = "106:3"]
    pub const TK_COMPILE_INTRINSIC: token_id = 69;
    #[c2rust::src_loc = "103:3"]
    pub const TK_MINUS_TILDE_NEW: token_id = 68;
    #[c2rust::src_loc = "102:3"]
    pub const TK_MINUS_NEW: token_id = 67;
    #[c2rust::src_loc = "101:3"]
    pub const TK_LSQUARE_NEW: token_id = 66;
    #[c2rust::src_loc = "100:3"]
    pub const TK_LPAREN_NEW: token_id = 65;
    #[c2rust::src_loc = "97:3"]
    pub const TK_CONSTANT: token_id = 64;
    #[c2rust::src_loc = "96:3"]
    pub const TK_ELLIPSIS: token_id = 63;
    #[c2rust::src_loc = "95:3"]
    pub const TK_UNARY_MINUS_TILDE: token_id = 62;
    #[c2rust::src_loc = "94:3"]
    pub const TK_UNARY_MINUS: token_id = 61;
    #[c2rust::src_loc = "93:3"]
    pub const TK_QUESTION: token_id = 60;
    #[c2rust::src_loc = "91:3"]
    pub const TK_SUBTYPE: token_id = 59;
    #[c2rust::src_loc = "90:3"]
    pub const TK_ALIASED: token_id = 58;
    #[c2rust::src_loc = "89:3"]
    pub const TK_EPHEMERAL: token_id = 57;
    #[c2rust::src_loc = "88:3"]
    pub const TK_ISECTTYPE: token_id = 56;
    #[c2rust::src_loc = "87:3"]
    pub const TK_PIPE: token_id = 55;
    #[c2rust::src_loc = "85:3"]
    pub const TK_NE_TILDE: token_id = 54;
    #[c2rust::src_loc = "84:3"]
    pub const TK_NE: token_id = 53;
    #[c2rust::src_loc = "83:3"]
    pub const TK_EQ_TILDE: token_id = 52;
    #[c2rust::src_loc = "82:3"]
    pub const TK_EQ: token_id = 51;
    #[c2rust::src_loc = "80:3"]
    pub const TK_GT_TILDE: token_id = 50;
    #[c2rust::src_loc = "79:3"]
    pub const TK_GT: token_id = 49;
    #[c2rust::src_loc = "78:3"]
    pub const TK_GE_TILDE: token_id = 48;
    #[c2rust::src_loc = "77:3"]
    pub const TK_GE: token_id = 47;
    #[c2rust::src_loc = "76:3"]
    pub const TK_LE_TILDE: token_id = 46;
    #[c2rust::src_loc = "75:3"]
    pub const TK_LE: token_id = 45;
    #[c2rust::src_loc = "74:3"]
    pub const TK_LT_TILDE: token_id = 44;
    #[c2rust::src_loc = "73:3"]
    pub const TK_LT: token_id = 43;
    #[c2rust::src_loc = "71:3"]
    pub const TK_RSHIFT_TILDE: token_id = 42;
    #[c2rust::src_loc = "70:3"]
    pub const TK_RSHIFT: token_id = 41;
    #[c2rust::src_loc = "69:3"]
    pub const TK_LSHIFT_TILDE: token_id = 40;
    #[c2rust::src_loc = "68:3"]
    pub const TK_LSHIFT: token_id = 39;
    #[c2rust::src_loc = "66:3"]
    pub const TK_AT_LBRACE: token_id = 38;
    #[c2rust::src_loc = "65:3"]
    pub const TK_AT: token_id = 37;
    #[c2rust::src_loc = "64:3"]
    pub const TK_MOD_TILDE: token_id = 36;
    #[c2rust::src_loc = "63:3"]
    pub const TK_MOD: token_id = 35;
    #[c2rust::src_loc = "62:3"]
    pub const TK_REM_TILDE: token_id = 34;
    #[c2rust::src_loc = "61:3"]
    pub const TK_REM: token_id = 33;
    #[c2rust::src_loc = "60:3"]
    pub const TK_DIVIDE_TILDE: token_id = 32;
    #[c2rust::src_loc = "59:3"]
    pub const TK_DIVIDE: token_id = 31;
    #[c2rust::src_loc = "58:3"]
    pub const TK_MULTIPLY_TILDE: token_id = 30;
    #[c2rust::src_loc = "57:3"]
    pub const TK_MULTIPLY: token_id = 29;
    #[c2rust::src_loc = "56:3"]
    pub const TK_MINUS_TILDE: token_id = 28;
    #[c2rust::src_loc = "55:3"]
    pub const TK_MINUS: token_id = 27;
    #[c2rust::src_loc = "54:3"]
    pub const TK_PLUS_TILDE: token_id = 26;
    #[c2rust::src_loc = "53:3"]
    pub const TK_PLUS: token_id = 25;
    #[c2rust::src_loc = "51:3"]
    pub const TK_ASSIGN: token_id = 24;
    #[c2rust::src_loc = "50:3"]
    pub const TK_SEMI: token_id = 23;
    #[c2rust::src_loc = "49:3"]
    pub const TK_COLON: token_id = 22;
    #[c2rust::src_loc = "48:3"]
    pub const TK_CHAIN: token_id = 21;
    #[c2rust::src_loc = "47:3"]
    pub const TK_TILDE: token_id = 20;
    #[c2rust::src_loc = "46:3"]
    pub const TK_DOT: token_id = 19;
    #[c2rust::src_loc = "45:3"]
    pub const TK_DBLARROW: token_id = 18;
    #[c2rust::src_loc = "44:3"]
    pub const TK_ARROW: token_id = 17;
    #[c2rust::src_loc = "43:3"]
    pub const TK_COMMA: token_id = 16;
    #[c2rust::src_loc = "41:3"]
    pub const TK_BACKSLASH: token_id = 15;
    #[c2rust::src_loc = "40:3"]
    pub const TK_RSQUARE: token_id = 14;
    #[c2rust::src_loc = "39:3"]
    pub const TK_LSQUARE: token_id = 13;
    #[c2rust::src_loc = "38:3"]
    pub const TK_RPAREN: token_id = 12;
    #[c2rust::src_loc = "37:3"]
    pub const TK_LPAREN: token_id = 11;
    #[c2rust::src_loc = "36:3"]
    pub const TK_RBRACE: token_id = 10;
    #[c2rust::src_loc = "35:3"]
    pub const TK_LBRACE: token_id = 9;
    #[c2rust::src_loc = "32:3"]
    pub const TK_ID: token_id = 8;
    #[c2rust::src_loc = "31:3"]
    pub const TK_FLOAT: token_id = 7;
    #[c2rust::src_loc = "30:3"]
    pub const TK_INT: token_id = 6;
    #[c2rust::src_loc = "29:3"]
    pub const TK_STRING: token_id = 5;
    #[c2rust::src_loc = "28:3"]
    pub const TK_FALSE: token_id = 4;
    #[c2rust::src_loc = "27:3"]
    pub const TK_TRUE: token_id = 3;
    #[c2rust::src_loc = "24:3"]
    pub const TK_NONE: token_id = 2;
    #[c2rust::src_loc = "23:3"]
    pub const TK_LEX_ERROR: token_id = 1;
    #[c2rust::src_loc = "22:3"]
    pub const TK_EOF: token_id = 0;
    use super::_size_t_h::size_t;
    use super::lexint_h::lexint_t;
    use super::source_h::{pony_type_t, source_t};
    extern "C" {
        #[c2rust::src_loc = "16:16"]
        pub type token_t;
        #[c2rust::src_loc = "18:16"]
        pub type token_signature_t;
        #[c2rust::src_loc = "282:1"]
        pub fn token_new(id: token_id) -> *mut token_t;
        #[c2rust::src_loc = "287:1"]
        pub fn token_dup(token: *mut token_t) -> *mut token_t;
        #[c2rust::src_loc = "293:1"]
        pub fn token_dup_new_id(token: *mut token_t, id: token_id) -> *mut token_t;
        #[c2rust::src_loc = "298:1"]
        pub fn token_free(token: *mut token_t);
        #[c2rust::src_loc = "301:1"]
        pub fn token_freeze(token: *mut token_t);
        #[c2rust::src_loc = "304:1"]
        pub fn token_signature_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "308:1"]
        pub fn token_docstring_signature_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "311:1"]
        pub fn token_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "317:1"]
        pub fn token_get_id(token: *mut token_t) -> token_id;
        #[c2rust::src_loc = "324:1"]
        pub fn token_string(token: *mut token_t) -> *const libc::c_char;
        #[c2rust::src_loc = "329:1"]
        pub fn token_string_len(token: *mut token_t) -> size_t;
        #[c2rust::src_loc = "332:1"]
        pub fn token_float(token: *mut token_t) -> libc::c_double;
        #[c2rust::src_loc = "335:1"]
        pub fn token_int(token: *mut token_t) -> *mut lexint_t;
        #[c2rust::src_loc = "341:1"]
        pub fn token_print(token: *mut token_t) -> *const libc::c_char;
        #[c2rust::src_loc = "346:1"]
        pub fn token_print_escaped(token: *mut token_t) -> *mut libc::c_char;
        #[c2rust::src_loc = "354:1"]
        pub fn token_source(token: *mut token_t) -> *mut source_t;
        #[c2rust::src_loc = "357:1"]
        pub fn token_line_number(token: *mut token_t) -> size_t;
        #[c2rust::src_loc = "360:1"]
        pub fn token_line_position(token: *mut token_t) -> size_t;
        #[c2rust::src_loc = "371:1"]
        pub fn token_set_id(token: *mut token_t, id: token_id);
        #[c2rust::src_loc = "380:1"]
        pub fn token_set_string(token: *mut token_t, value: *const libc::c_char, length_0: size_t);
        #[c2rust::src_loc = "383:1"]
        pub fn token_set_float(token: *mut token_t, value: libc::c_double);
        #[c2rust::src_loc = "386:1"]
        pub fn token_set_int(token: *mut token_t, value: *mut lexint_t);
        #[c2rust::src_loc = "391:1"]
        pub fn token_set_pos(token: *mut token_t, source: *mut source_t, line: size_t, pos: size_t);
    }
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = size_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: size_t,
        pub size: size_t,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:35"]
    pub struct symtab_t {
        pub contents: hashmap_t,
    }
    #[c2rust::src_loc = "12:9"]
    pub type sym_status_t = libc::c_uint;
    #[c2rust::src_loc = "21:3"]
    pub const SYM_ERROR: sym_status_t = 7;
    #[c2rust::src_loc = "20:3"]
    pub const SYM_FFIDECL: sym_status_t = 6;
    #[c2rust::src_loc = "19:3"]
    pub const SYM_CONSUMED_SAME_EXPR: sym_status_t = 5;
    #[c2rust::src_loc = "18:3"]
    pub const SYM_CONSUMED: sym_status_t = 4;
    #[c2rust::src_loc = "17:3"]
    pub const SYM_UNDEFINED: sym_status_t = 3;
    #[c2rust::src_loc = "16:3"]
    pub const SYM_DEFINED: sym_status_t = 2;
    #[c2rust::src_loc = "15:3"]
    pub const SYM_NOCASE: sym_status_t = 1;
    #[c2rust::src_loc = "14:3"]
    pub const SYM_NONE: sym_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct symbol_t {
        pub name: *const libc::c_char,
        pub def: *mut ast_t,
        pub status: sym_status_t,
        pub branch_count: size_t,
    }
    use super::_size_t_h::size_t;
    use super::ast_t;
    use super::hash_h::hashmap_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "32:45"]
        pub fn symtab_next(map: *mut symtab_t, i: *mut size_t) -> *mut symbol_t;
        #[c2rust::src_loc = "32:1"]
        pub fn symtab_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "36:1"]
        pub fn symtab_new() -> *mut symtab_t;
        #[c2rust::src_loc = "38:1"]
        pub fn symtab_dup(symtab: *mut symtab_t) -> *mut symtab_t;
        #[c2rust::src_loc = "40:1"]
        pub fn symtab_free(symtab: *mut symtab_t);
        #[c2rust::src_loc = "42:1"]
        pub fn symtab_add(
            symtab: *mut symtab_t,
            name: *const libc::c_char,
            def: *mut ast_t,
            status: sym_status_t,
        ) -> bool;
        #[c2rust::src_loc = "45:1"]
        pub fn symtab_find(
            symtab: *mut symtab_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "47:1"]
        pub fn symtab_find_case(
            symtab: *mut symtab_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "52:1"]
        pub fn symtab_set_status(
            symtab: *mut symtab_t,
            name: *const libc::c_char,
            status: sym_status_t,
        );
        #[c2rust::src_loc = "55:1"]
        pub fn symtab_inherit_status(dst: *mut symtab_t, src: *mut symtab_t);
        #[c2rust::src_loc = "57:1"]
        pub fn symtab_inherit_branch(dst: *mut symtab_t, src: *mut symtab_t);
        #[c2rust::src_loc = "59:1"]
        pub fn symtab_can_merge_public(dst: *mut symtab_t, src: *mut symtab_t) -> bool;
        #[c2rust::src_loc = "61:1"]
        pub fn symtab_merge_public(dst: *mut symtab_t, src: *mut symtab_t) -> bool;
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
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::fun_h::{cmp_fn, free_fn, map_fn};
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn ponyint_list_pop(list: *mut list_t, data: *mut *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "20:1"]
        pub fn ponyint_list_push(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "22:1"]
        pub fn ponyint_list_append(list: *mut list_t, data: *mut libc::c_void) -> *mut list_t;
        #[c2rust::src_loc = "24:1"]
        pub fn ponyint_list_next(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "26:1"]
        pub fn ponyint_list_index(list: *mut list_t, index: ssize_t) -> *mut list_t;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_list_data(list: *mut list_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_list_find(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "32:1"]
        pub fn ponyint_list_findindex(
            list: *mut list_t,
            f: cmp_fn,
            data: *mut libc::c_void,
        ) -> ssize_t;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_list_subset(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_list_equals(a: *mut list_t, b: *mut list_t, f: cmp_fn) -> bool;
        #[c2rust::src_loc = "38:1"]
        pub fn ponyint_list_map(
            list: *mut list_t,
            f: map_fn,
            arg: *mut libc::c_void,
        ) -> *mut list_t;
        #[c2rust::src_loc = "40:1"]
        pub fn ponyint_list_reverse(list: *mut list_t) -> *mut list_t;
        #[c2rust::src_loc = "42:1"]
        pub fn ponyint_list_length(list: *mut list_t) -> size_t;
        #[c2rust::src_loc = "44:1"]
        pub fn ponyint_list_free(list: *mut list_t, f: free_fn);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "29:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "51:3"]
    pub const AST_FLAG_FCNSM_REASGN: C2RustUnnamed_0 = 33554432;
    #[c2rust::src_loc = "50:3"]
    pub const AST_FLAG_CNSM_REASGN: C2RustUnnamed_0 = 16777216;
    #[c2rust::src_loc = "49:3"]
    pub const AST_FLAG_MAY_BREAK: C2RustUnnamed_0 = 8388608;
    #[c2rust::src_loc = "48:3"]
    pub const AST_FLAG_IMPORT: C2RustUnnamed_0 = 4194304;
    #[c2rust::src_loc = "47:3"]
    pub const AST_FLAG_INCOMPLETE: C2RustUnnamed_0 = 2097152;
    #[c2rust::src_loc = "46:3"]
    pub const AST_FLAG_JUMPS_AWAY: C2RustUnnamed_0 = 1048576;
    #[c2rust::src_loc = "45:3"]
    pub const AST_FLAG_ERROR_2: C2RustUnnamed_0 = 524288;
    #[c2rust::src_loc = "44:3"]
    pub const AST_FLAG_DONE_2: C2RustUnnamed_0 = 262144;
    #[c2rust::src_loc = "43:3"]
    pub const AST_FLAG_RECURSE_2: C2RustUnnamed_0 = 131072;
    #[c2rust::src_loc = "42:3"]
    pub const AST_FLAG_ERROR_1: C2RustUnnamed_0 = 65536;
    #[c2rust::src_loc = "41:3"]
    pub const AST_FLAG_DONE_1: C2RustUnnamed_0 = 32768;
    #[c2rust::src_loc = "40:3"]
    pub const AST_FLAG_RECURSE_1: C2RustUnnamed_0 = 16384;
    #[c2rust::src_loc = "39:3"]
    pub const AST_FLAG_PRESERVE: C2RustUnnamed_0 = 8192;
    #[c2rust::src_loc = "38:3"]
    pub const AST_FLAG_MISSING_SEMI: C2RustUnnamed_0 = 4096;
    #[c2rust::src_loc = "37:3"]
    pub const AST_FLAG_BAD_SEMI: C2RustUnnamed_0 = 2048;
    #[c2rust::src_loc = "36:3"]
    pub const AST_FLAG_AMBIGUOUS: C2RustUnnamed_0 = 1024;
    #[c2rust::src_loc = "35:3"]
    pub const AST_FLAG_IN_PARENS: C2RustUnnamed_0 = 512;
    #[c2rust::src_loc = "34:3"]
    pub const AST_FLAG_MIGHT_SEND: C2RustUnnamed_0 = 256;
    #[c2rust::src_loc = "33:3"]
    pub const AST_FLAG_CAN_SEND: C2RustUnnamed_0 = 128;
    #[c2rust::src_loc = "32:3"]
    pub const AST_FLAG_CAN_ERROR: C2RustUnnamed_0 = 64;
    #[c2rust::src_loc = "31:3"]
    pub const AST_FLAG_PASS_MASK: C2RustUnnamed_0 = 31;
    #[c2rust::src_loc = "54:1"]
    pub type astlist_cmp_fn = Option<unsafe extern "C" fn(*mut ast_t, *mut ast_t) -> bool>;
    #[c2rust::src_loc = "54:1"]
    pub type astlist_map_fn =
        Option<unsafe extern "C" fn(*mut ast_t, *mut libc::c_void) -> *mut ast_t>;
    #[c2rust::src_loc = "54:1"]
    pub type astlist_free_fn = Option<unsafe extern "C" fn(*mut ast_t) -> ()>;
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.h:9"]
pub mod program_h {
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "10:16"]
        pub type program_t;
        #[c2rust::src_loc = "13:1"]
        pub fn program_create() -> *mut program_t;
        #[c2rust::src_loc = "16:1"]
        pub fn program_free(program: *mut program_t);
        #[c2rust::src_loc = "49:1"]
        pub fn program_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:10"]
pub mod package_h {
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "13:16"]
        pub type package_t;
        #[c2rust::src_loc = "105:1"]
        pub fn package_free(package: *mut package_t);
        #[c2rust::src_loc = "198:1"]
        pub fn package_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/printbuf.h:6"]
pub mod printbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct printbuf_t {
        pub m: *mut libc::c_char,
        pub size: size_t,
        pub offset: size_t,
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn printbuf_new() -> *mut printbuf_t;
        #[c2rust::src_loc = "19:1"]
        pub fn printbuf_free(buf: *mut printbuf_t);
        #[c2rust::src_loc = "21:1"]
        pub fn printbuf(buf: *mut printbuf_t, fmt: *const libc::c_char, _: ...);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "68:14"]
        pub static mut __stdoutp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "156:1"]
        pub fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "24:1"]
        pub fn string_trace(ctx: *mut pony_ctx_t, string: *const libc::c_char);
        #[c2rust::src_loc = "27:1"]
        pub fn string_deserialise_offset(
            ctx: *mut pony_ctx_t,
            offset: uintptr_t,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:7"]
pub mod literal_h {
    use super::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn operatorliteral_serialise_data(ast: *mut ast_t, dst: *mut ast_t);
        #[c2rust::src_loc = "12:1"]
        pub fn operatorliteral_deserialise_data(ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:11"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> size_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:11"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:13"]
pub mod ponyassert_h {
    use super::_size_t_h::size_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t, __darwin_va_list, __int64_t};
pub use self::_uintptr_t_h::uintptr_t;
pub use self::_va_list_h::va_list;
pub use self::ast_h::{
    ast_ptr_t, astlist_cmp_fn, astlist_free_fn, astlist_map_fn, C2RustUnnamed_0,
    AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
pub use self::error_h::{errorframe_t, errorframev, errormsg_t, errors_t, errorv, errorv_continue};
pub use self::fun_h::{cmp_fn, free_fn, map_fn};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::lexint_h::lexint_t;
pub use self::list_h::{
    list_t, ponyint_list_append, ponyint_list_data, ponyint_list_equals, ponyint_list_find,
    ponyint_list_findindex, ponyint_list_free, ponyint_list_index, ponyint_list_length,
    ponyint_list_map, ponyint_list_next, ponyint_list_pop, ponyint_list_push, ponyint_list_reverse,
    ponyint_list_subset,
};
use self::literal_h::{operatorliteral_deserialise_data, operatorliteral_serialise_data};
use self::package_h::{package_free, package_pony_type, package_t};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, C2RustUnnamed, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE,
    PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free, ponyint_pool_free_size};
pub use self::printbuf_h::{printbuf, printbuf_free, printbuf_new, printbuf_t};
use self::program_h::{program_create, program_free, program_pony_type, program_t};
use self::serialise_h::{pony_deserialise_offset, pony_serialise_offset};
pub use self::source_h::{pony_type_t, source_close, source_pony_type, source_t};
use self::stdio_h::{__stdoutp, fprintf, fputc};
use self::string_h::{memset, strlen};
use self::stringtab_h::{string_deserialise_offset, string_trace, stringtab};
pub use self::symtab_h::{
    sym_status_t, symbol_t, symtab_add, symtab_can_merge_public, symtab_dup, symtab_find,
    symtab_find_case, symtab_free, symtab_inherit_branch, symtab_inherit_status,
    symtab_merge_public, symtab_new, symtab_next, symtab_pony_type, symtab_set_status, symtab_t,
    SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL, SYM_NOCASE,
    SYM_NONE, SYM_UNDEFINED,
};
pub use self::sys__types_h::__darwin_off_t;
pub use self::token_h::{
    token_docstring_signature_pony_type, token_dup, token_dup_new_id, token_float, token_free,
    token_freeze, token_get_id, token_id, token_int, token_line_number, token_line_position,
    token_new, token_pony_type, token_print, token_print_escaped, token_set_float, token_set_id,
    token_set_int, token_set_pos, token_set_string, token_signature_pony_type, token_signature_t,
    token_source, token_string, token_string_len, token_t, TK_ACTOR, TK_ADDRESS, TK_ALIASED,
    TK_AND, TK_ANNOTATION, TK_ARRAY, TK_ARROW, TK_AS, TK_ASSIGN, TK_AT, TK_AT_LBRACE, TK_BACKSLASH,
    TK_BARELAMBDA, TK_BARELAMBDATYPE, TK_BE, TK_BEAPP, TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK,
    TK_CALL, TK_CAP_ALIAS, TK_CAP_ANY, TK_CAP_READ, TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES,
    TK_CHAIN, TK_CLASS, TK_COLON, TK_COMMA, TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT,
    TK_CONSUME, TK_CONTINUE, TK_DBLARROW, TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE,
    TK_DIVIDE_TILDE, TK_DO, TK_DONTCARE, TK_DONTCAREREF, TK_DONTCARETYPE, TK_DOT, TK_ELLIPSIS,
    TK_ELSE, TK_ELSEIF, TK_EMBED, TK_EMBEDREF, TK_END, TK_EOF, TK_EPHEMERAL, TK_EQ, TK_EQ_TILDE,
    TK_ERROR, TK_ERRORTYPE, TK_FALSE, TK_FFICALL, TK_FFIDECL, TK_FLATTEN, TK_FLET, TK_FLETREF,
    TK_FLOAT, TK_FOR, TK_FUN, TK_FUNAPP, TK_FUNCHAIN, TK_FUNREF, TK_FUNTYPE, TK_FVAR, TK_FVARREF,
    TK_GE, TK_GE_TILDE, TK_GT, TK_GT_TILDE, TK_ID, TK_IF, TK_IFDEF, TK_IFDEFAND, TK_IFDEFFLAG,
    TK_IFDEFNOT, TK_IFDEFOR, TK_IFTYPE, TK_IFTYPE_SET, TK_IN, TK_INFERTYPE, TK_INT, TK_INTERFACE,
    TK_IS, TK_ISECTTYPE, TK_ISNT, TK_ISO, TK_LAMBDA, TK_LAMBDACAPTURE, TK_LAMBDACAPTURES,
    TK_LAMBDATYPE, TK_LBRACE, TK_LE, TK_LET, TK_LETREF, TK_LEX_ERROR, TK_LE_TILDE, TK_LITERAL,
    TK_LITERALBRANCH, TK_LOCATION, TK_LPAREN, TK_LPAREN_NEW, TK_LSHIFT, TK_LSHIFT_TILDE,
    TK_LSQUARE, TK_LSQUARE_NEW, TK_LT, TK_LT_TILDE, TK_MATCH, TK_MATCH_CAPTURE, TK_MATCH_DONTCARE,
    TK_MEMBERS, TK_MINUS, TK_MINUS_NEW, TK_MINUS_TILDE, TK_MINUS_TILDE_NEW, TK_MOD, TK_MODULE,
    TK_MOD_TILDE, TK_MULTIPLY, TK_MULTIPLY_TILDE, TK_NAMEDARG, TK_NAMEDARGS, TK_NE, TK_NEW,
    TK_NEWAPP, TK_NEWBEREF, TK_NEWLINE, TK_NEWREF, TK_NE_TILDE, TK_NOMINAL, TK_NONE, TK_NOT,
    TK_OBJECT, TK_OPERATORLITERAL, TK_OR, TK_PACKAGE, TK_PACKAGEREF, TK_PARAM, TK_PARAMREF,
    TK_PARAMS, TK_PIPE, TK_PLUS, TK_PLUS_TILDE, TK_POSITIONALARGS, TK_PRIMITIVE, TK_PROGRAM,
    TK_PROVIDES, TK_QUALIFY, TK_QUESTION, TK_RBRACE, TK_RECOVER, TK_REF, TK_REFERENCE, TK_REM,
    TK_REM_TILDE, TK_REPEAT, TK_RETURN, TK_RPAREN, TK_RSHIFT, TK_RSHIFT_TILDE, TK_RSQUARE, TK_SEMI,
    TK_SEQ, TK_STRING, TK_STRUCT, TK_SUBTYPE, TK_TAG, TK_TEST_ALIASED, TK_TEST_EXTRA,
    TK_TEST_NO_SEQ, TK_TEST_SEQ_SCOPE, TK_TEST_TRY_NO_CHECK, TK_TEST_UPDATEARG, TK_THEN, TK_THIS,
    TK_THISTYPE, TK_TILDE, TK_TRAIT, TK_TRN, TK_TRUE, TK_TRY, TK_TRY_NO_CHECK, TK_TUPLE,
    TK_TUPLEELEMREF, TK_TUPLETYPE, TK_TYPE, TK_TYPEARGS, TK_TYPEPARAM, TK_TYPEPARAMREF,
    TK_TYPEPARAMS, TK_TYPEREF, TK_UNARY_MINUS, TK_UNARY_MINUS_TILDE, TK_UNIONTYPE, TK_UNTIL,
    TK_UPDATEARG, TK_USE, TK_VAL, TK_VALUEFORMALARG, TK_VALUEFORMALPARAM, TK_VAR, TK_VARREF,
    TK_WHERE, TK_WHILE, TK_WITH, TK_XOR,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "69:8"]
pub struct ast_t {
    pub t: *mut token_t,
    pub symtab: *mut symtab_t,
    pub data: *mut libc::c_void,
    pub parent: *mut ast_t,
    pub child: *mut ast_t,
    pub sibling: *mut ast_t,
    pub annotation_type: *mut ast_t,
    pub flags: u32,
    pub frozen: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "98:22"]
pub struct astlist_t {
    pub contents: list_t,
}
#[c2rust::src_loc = "58:3"]
pub const AST_ORPHAN: C2RustUnnamed_1 = 32;
#[c2rust::src_loc = "61:3"]
pub const AST_ALL_FLAGS: C2RustUnnamed_1 = 67108863;
#[c2rust::src_loc = "59:3"]
pub const AST_INHERIT_FLAGS: C2RustUnnamed_1 = 147904;
#[c2rust::src_loc = "103:1"]
pub type print_special = libc::c_uint;
#[c2rust::src_loc = "107:3"]
pub const SPECIAL_ANNOTATION: print_special = 2;
#[c2rust::src_loc = "106:3"]
pub const SPECIAL_TYPE: print_special = 1;
#[c2rust::src_loc = "105:3"]
pub const NOT_SPECIAL: print_special = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "85:16"]
pub struct ast_signature_t {
    pub t: *mut token_signature_t,
    pub child: *mut ast_signature_t,
    pub sibling: *mut ast_signature_t,
    pub annotation_type: *mut ast_signature_t,
}
#[c2rust::src_loc = "56:1"]
pub type C2RustUnnamed_1 = libc::c_uint;
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn ast_cmp(mut a: *mut ast_t, mut b: *mut ast_t) -> bool {
    a == b
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn astlist_length(mut list: *mut astlist_t) -> size_t {
    ponyint_list_length(list as *mut list_t)
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn astlist_subset(mut a: *mut astlist_t, mut b: *mut astlist_t) -> bool {
    let mut cmp: astlist_cmp_fn =
        Some(ast_cmp as unsafe extern "C" fn(*mut ast_t, *mut ast_t) -> bool);
    ponyint_list_subset(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<astlist_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn astlist_free(mut list: *mut astlist_t) {
    let mut free: astlist_free_fn = None;
    ponyint_list_free(
        list as *mut list_t,
        ::core::mem::transmute::<astlist_free_fn, free_fn>(free),
    );
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn astlist_findindex(
    mut list: *mut astlist_t,
    mut data: *mut ast_t,
) -> ssize_t {
    let mut cmp: astlist_cmp_fn =
        Some(ast_cmp as unsafe extern "C" fn(*mut ast_t, *mut ast_t) -> bool);
    ponyint_list_findindex(
        list as *mut list_t,
        ::core::mem::transmute::<astlist_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    )
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn astlist_equals(mut a: *mut astlist_t, mut b: *mut astlist_t) -> bool {
    let mut cmp: astlist_cmp_fn =
        Some(ast_cmp as unsafe extern "C" fn(*mut ast_t, *mut ast_t) -> bool);
    ponyint_list_equals(
        a as *mut list_t,
        b as *mut list_t,
        ::core::mem::transmute::<astlist_cmp_fn, cmp_fn>(cmp),
    )
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_push(
    mut list: *mut astlist_t,
    mut data: *mut ast_t,
) -> *mut astlist_t {
    ponyint_list_push(list as *mut list_t, data as *mut libc::c_void) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_pop(
    mut list: *mut astlist_t,
    mut data: *mut *mut ast_t,
) -> *mut astlist_t {
    ponyint_list_pop(list as *mut list_t, data as *mut *mut libc::c_void) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_append(
    mut list: *mut astlist_t,
    mut data: *mut ast_t,
) -> *mut astlist_t {
    ponyint_list_append(list as *mut list_t, data as *mut libc::c_void) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_next(mut list: *mut astlist_t) -> *mut astlist_t {
    ponyint_list_next(list as *mut list_t) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_map(
    mut list: *mut astlist_t,
    mut f: astlist_map_fn,
    mut arg: *mut libc::c_void,
) -> *mut astlist_t {
    ponyint_list_map(
        list as *mut list_t,
        ::core::mem::transmute::<astlist_map_fn, map_fn>(f),
        arg,
    ) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_reverse(mut list: *mut astlist_t) -> *mut astlist_t {
    ponyint_list_reverse(list as *mut list_t) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:22"]
pub unsafe extern "C" fn astlist_index(
    mut list: *mut astlist_t,
    mut index: ssize_t,
) -> *mut astlist_t {
    ponyint_list_index(list as *mut list_t, index) as *mut astlist_t
}
#[no_mangle]
#[c2rust::src_loc = "98:33"]
pub unsafe extern "C" fn astlist_data(mut list: *mut astlist_t) -> *mut ast_t {
    ponyint_list_data(list as *mut list_t) as *mut ast_t
}
#[no_mangle]
#[c2rust::src_loc = "98:33"]
pub unsafe extern "C" fn astlist_find(
    mut list: *mut astlist_t,
    mut data: *mut ast_t,
) -> *mut ast_t {
    let mut cmp: astlist_cmp_fn =
        Some(ast_cmp as unsafe extern "C" fn(*mut ast_t, *mut ast_t) -> bool);
    ponyint_list_find(
        list as *mut list_t,
        ::core::mem::transmute::<astlist_cmp_fn, cmp_fn>(cmp),
        data as *mut libc::c_void,
    ) as *mut ast_t
}
#[c2rust::src_loc = "100:19"]
static mut in_0: [libc::c_char; 3] =
    unsafe { *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"  \0") };
#[c2rust::src_loc = "101:21"]
static mut in_len: size_t = 2 as libc::c_int as size_t;
#[c2rust::src_loc = "110:19"]
static mut special_char: [libc::c_char; 6] = [
    '(' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
];
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn print_token(mut fp: *mut FILE, mut token: *mut token_t) {
    match token_get_id(token) as libc::c_uint {
        5 => {
            let mut escaped: *mut libc::c_char = token_print_escaped(token);
            fprintf(fp, b"\"%s\"\0" as *const u8 as *const libc::c_char, escaped);
            ponyint_pool_free_size(strlen(escaped), escaped as *mut libc::c_void);
        }
        8 => {
            fprintf(
                fp,
                b"(id %s)\0" as *const u8 as *const libc::c_char,
                token_print(token),
            );
        }
        _ => {
            fprintf(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                token_print(token),
            );
        }
    };
}
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn length(
    mut ast: *mut ast_t,
    mut indent: size_t,
    mut kind: print_special,
) -> size_t {
    let mut len: size_t = indent
        .wrapping_mul(in_len)
        .wrapping_add(strlen(token_print((*ast).t)));
    let mut child: *mut ast_t = (*ast).child;
    if kind as libc::c_uint != NOT_SPECIAL as libc::c_int as libc::c_uint
        || !child.is_null()
        || !(ast_type(ast)).is_null()
        || !(ast_annotation(ast)).is_null()
    {
        len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
    }
    match token_get_id((*ast).t) as libc::c_uint {
        5 => {
            len = (len as libc::c_ulong).wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        8 => {
            len = (len as libc::c_ulong).wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        _ => {}
    }
    if !((*ast).symtab).is_null() {
        len = (len as libc::c_ulong).wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
    }
    while !child.is_null() {
        len = (len as libc::c_ulong).wrapping_add(
            (1 as libc::c_int as libc::c_ulong).wrapping_add(length(
                child,
                0 as libc::c_int as size_t,
                NOT_SPECIAL,
            )),
        ) as size_t as size_t;
        child = (*child).sibling;
    }
    if !(ast_type(ast)).is_null() {
        len = (len as libc::c_ulong).wrapping_add((1 as libc::c_int as libc::c_ulong).wrapping_add(
            length(ast_type(ast), 0 as libc::c_int as size_t, SPECIAL_TYPE),
        )) as size_t as size_t;
    }
    if !(ast_annotation(ast)).is_null() {
        len = (len as libc::c_ulong).wrapping_add((1 as libc::c_int as libc::c_ulong).wrapping_add(
            length(
                ast_annotation(ast),
                0 as libc::c_int as size_t,
                SPECIAL_ANNOTATION,
            ),
        )) as size_t as size_t;
    }
    len
}
#[c2rust::src_loc = "172:1"]
unsafe extern "C" fn print_compact(
    mut fp: *mut FILE,
    mut ast: *mut ast_t,
    mut indent: size_t,
    mut kind: print_special,
) {
    let mut i: size_t = 0;
    while i < indent {
        fprintf(fp, in_0.as_ptr());
        i = i.wrapping_add(1);
    }
    let mut child: *mut ast_t = (*ast).child;
    let mut parens: bool = kind as libc::c_uint != NOT_SPECIAL as libc::c_int as libc::c_uint
        || !child.is_null()
        || !(ast_type(ast)).is_null()
        || !(ast_annotation(ast)).is_null();
    if parens {
        fputc(
            special_char
                [(kind as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
            fp,
        );
    }
    print_token(fp, (*ast).t);
    if !((*ast).symtab).is_null() {
        fprintf(fp, b":scope\0" as *const u8 as *const libc::c_char);
    }
    if !(ast_annotation(ast)).is_null() {
        fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
        print_compact(
            fp,
            ast_annotation(ast),
            0 as libc::c_int as size_t,
            SPECIAL_ANNOTATION,
        );
    }
    while !child.is_null() {
        fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
        print_compact(fp, child, 0 as libc::c_int as size_t, NOT_SPECIAL);
        child = (*child).sibling;
    }
    if !(ast_type(ast)).is_null() {
        fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
        print_compact(fp, ast_type(ast), 0 as libc::c_int as size_t, SPECIAL_TYPE);
    }
    if parens {
        fputc(
            special_char[(kind as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
            fp,
        );
    }
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn print_extended(
    mut fp: *mut FILE,
    mut ast: *mut ast_t,
    mut indent: size_t,
    mut kind: print_special,
    mut width: size_t,
) {
    let mut i: size_t = 0;
    while i < indent {
        fprintf(fp, in_0.as_ptr());
        i = i.wrapping_add(1);
    }
    let mut child: *mut ast_t = (*ast).child;
    let mut parens: bool = kind as libc::c_uint != NOT_SPECIAL as libc::c_int as libc::c_uint
        || !child.is_null()
        || !(ast_type(ast)).is_null()
        || !(ast_annotation(ast)).is_null();
    if parens {
        fputc(
            special_char
                [(kind as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
            fp,
        );
    }
    print_token(fp, (*ast).t);
    if !((*ast).symtab).is_null() {
        fprintf(fp, b":scope\0" as *const u8 as *const libc::c_char);
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
    if !(ast_annotation(ast)).is_null() {
        print(
            fp,
            ast_annotation(ast),
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            SPECIAL_ANNOTATION,
            width,
        );
    }
    while !child.is_null() {
        print(
            fp,
            child,
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            NOT_SPECIAL,
            width,
        );
        child = (*child).sibling;
    }
    if !(ast_type(ast)).is_null() {
        print(
            fp,
            ast_type(ast),
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            SPECIAL_TYPE,
            width,
        );
    }
    if parens {
        let mut i_0: size_t = 0;
        while i_0 < indent {
            fprintf(fp, in_0.as_ptr());
            i_0 = i_0.wrapping_add(1);
        }
        fputc(
            special_char[(kind as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
            fp,
        );
    }
}
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn print_verbose(
    mut fp: *mut FILE,
    mut ast: *mut ast_t,
    mut indent: size_t,
    mut kind: print_special,
) {
    let mut i: size_t = 0;
    while i < indent {
        fprintf(fp, in_0.as_ptr());
        i = i.wrapping_add(1);
    }
    let mut child: *mut ast_t = (*ast).child;
    let mut parens: bool = kind as libc::c_uint != NOT_SPECIAL as libc::c_int as libc::c_uint
        || !child.is_null()
        || !(ast_type(ast)).is_null()
        || !(ast_annotation(ast)).is_null();
    if parens {
        fputc(
            special_char
                [(kind as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
            fp,
        );
    }
    print_token(fp, (*ast).t);
    fprintf(
        fp,
        b":%p,%0x\0" as *const u8 as *const libc::c_char,
        ast,
        (*ast).flags,
    );
    if !((*ast).data).is_null() {
        fprintf(
            fp,
            b":data=%p\0" as *const u8 as *const libc::c_char,
            (*ast).data,
        );
    }
    if !((*ast).symtab).is_null() {
        fprintf(fp, b":scope {\n\0" as *const u8 as *const libc::c_char);
        let mut i_0: size_t = -(1 as libc::c_int) as size_t;
        let mut sym: *mut symbol_t = 0 as *mut symbol_t;
        loop {
            sym = symtab_next((*ast).symtab, &mut i_0);
            if sym.is_null() {
                break;
            }
            fprintf(
                fp,
                b"  %s (%d): %p\n\0" as *const u8 as *const libc::c_char,
                (*sym).name,
                (*sym).status as libc::c_uint,
                (*sym).def,
            );
        }
        fprintf(fp, b"}\0" as *const u8 as *const libc::c_char);
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
    if !(ast_annotation(ast)).is_null() {
        print_verbose(
            fp,
            ast_annotation(ast),
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            SPECIAL_ANNOTATION,
        );
    }
    while !child.is_null() {
        print_verbose(
            fp,
            child,
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            NOT_SPECIAL,
        );
        child = (*child).sibling;
    }
    if !(ast_type(ast)).is_null() {
        print_verbose(
            fp,
            ast_type(ast),
            indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
            SPECIAL_TYPE,
        );
    }
    if parens {
        let mut i_1: size_t = 0;
        while i_1 < indent {
            fprintf(fp, in_0.as_ptr());
            i_1 = i_1.wrapping_add(1);
        }
        fprintf(
            fp,
            b"%c\n\0" as *const u8 as *const libc::c_char,
            special_char[(kind as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "309:1"]
unsafe extern "C" fn print(
    mut fp: *mut FILE,
    mut ast: *mut ast_t,
    mut indent: size_t,
    mut kind: print_special,
    mut width: size_t,
) {
    let mut len: size_t = length(ast, indent, kind);
    if len < width {
        print_compact(fp, ast, indent, kind);
    } else {
        print_extended(fp, ast, indent, kind, width);
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "323:1"]
unsafe extern "C" fn hasparent(mut ast: *mut ast_t) -> bool {
    return ast_checkflag(ast, AST_ORPHAN as libc::c_int as u32) == 0
        && !((*ast).parent).is_null();
}
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn set_scope_and_parent(mut ast: *mut ast_t, mut parent: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            331 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"set_scope_and_parent\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            332 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"set_scope_and_parent\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh0 = (*ast).parent;
    *fresh0 = parent;
    ast_clearflag(ast, AST_ORPHAN as libc::c_int as u32);
}
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn set_scope_no_parent(mut ast: *mut ast_t, mut scope: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            341 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"set_scope_no_parent\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            342 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"set_scope_no_parent\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh1 = (*ast).parent;
    *fresh1 = scope;
    ast_setflag(ast, AST_ORPHAN as libc::c_int as u32);
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn make_orphan_leave_scope(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            351 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"make_orphan_leave_scope\0",
            ))
            .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            352 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"make_orphan_leave_scope\0",
            ))
            .as_ptr(),
        );
    };
    ast_setflag(ast, AST_ORPHAN as libc::c_int as u32);
}
#[c2rust::src_loc = "357:1"]
unsafe extern "C" fn duplicate(mut parent: *mut ast_t, mut ast: *mut ast_t) -> *mut ast_t {
    if ast.is_null() {
        return 0 as *mut ast_t;
    }
    if ast_id(ast) as libc::c_uint != TK_PROGRAM as libc::c_int as libc::c_uint
        && ast_id(ast) as libc::c_uint != TK_PACKAGE as libc::c_int as libc::c_uint
        && ast_id(ast) as libc::c_uint != TK_MODULE as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) != TK_PROGRAM && ast_id(ast) != TK_PACKAGE && ast_id(ast) != TK_MODULE\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            363 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"duplicate\0")).as_ptr(),
        );
    };
    let mut n: *mut ast_t = ast_token(token_dup((*ast).t));
    let ref mut fresh2 = (*n).data;
    *fresh2 = (*ast).data;
    (*n).flags = (*ast).flags & AST_ALL_FLAGS as libc::c_int as libc::c_uint;
    if parent.is_null() {
        set_scope_no_parent(n, (*ast).parent);
    } else {
        set_scope_and_parent(n, parent);
    }
    let ref mut fresh3 = (*n).child;
    *fresh3 = duplicate(n, (*ast).child);
    ast_setannotation(n, duplicate(0 as *mut ast_t, ast_annotation(ast)));
    ast_settype(n, duplicate(0 as *mut ast_t, ast_type(ast)));
    if !((*ast).symtab).is_null() {
        let ref mut fresh4 = (*n).symtab;
        *fresh4 = symtab_dup((*ast).symtab);
    }
    if !parent.is_null() {
        let ref mut fresh5 = (*n).sibling;
        *fresh5 = duplicate(parent, (*ast).sibling);
    }
    n
}
#[no_mangle]
#[c2rust::src_loc = "390:1"]
pub unsafe extern "C" fn ast_new(mut t: *mut token_t, mut id: token_id) -> *mut ast_t {
    return ast_token(token_dup_new_id(t, id));
}
#[no_mangle]
#[c2rust::src_loc = "395:1"]
pub unsafe extern "C" fn ast_blank(mut id: token_id) -> *mut ast_t {
    return ast_token(token_new(id));
}
#[no_mangle]
#[c2rust::src_loc = "400:1"]
pub unsafe extern "C" fn ast_token(mut t: *mut token_t) -> *mut ast_t {
    let mut ast: *mut ast_t = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut ast_t;
    memset(
        ast as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ast_t>() as libc::c_ulong,
    );
    let ref mut fresh6 = (*ast).t;
    *fresh6 = t;
    match token_get_id(t) as libc::c_uint {
        136 => {
            let ref mut fresh7 = (*ast).data;
            *fresh7 = program_create() as *mut libc::c_void;
        }
        _ => {}
    }
    return ast;
}
#[no_mangle]
#[c2rust::src_loc = "419:1"]
pub unsafe extern "C" fn ast_from(mut ast: *mut ast_t, mut id: token_id) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            421 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_from\0")).as_ptr(),
        );
    };
    let mut new_ast: *mut ast_t = ast_token(token_dup_new_id((*ast).t, id));
    set_scope_no_parent(new_ast, (*ast).parent);
    new_ast
}
#[no_mangle]
#[c2rust::src_loc = "427:1"]
pub unsafe extern "C" fn ast_from_string(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
) -> *mut ast_t {
    if name.is_null() {
        return ast_from(ast, TK_NONE);
    }
    let mut t: *mut token_t = token_dup((*ast).t);
    token_set_id(t, TK_ID);
    token_set_string(t, name, 0 as libc::c_int as size_t);
    let mut new_ast: *mut ast_t = ast_token(t);
    set_scope_no_parent(new_ast, (*ast).parent);
    new_ast
}
#[no_mangle]
#[c2rust::src_loc = "441:1"]
pub unsafe extern "C" fn ast_from_int(mut ast: *mut ast_t, mut value: u64) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            443 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_from_int\0")).as_ptr(),
        );
    };
    let mut t: *mut token_t = token_dup((*ast).t);
    token_set_id(t, TK_INT);
    let mut lexint: lexint_t = {
        let mut init = lexint_t {
            low: value,
            high: 0 as libc::c_int as u64,
        };
        init
    };
    token_set_int(t, &mut lexint);
    let mut new_ast: *mut ast_t = ast_token(t);
    set_scope_no_parent(new_ast, (*ast).parent);
    new_ast
}
#[no_mangle]
#[c2rust::src_loc = "455:1"]
pub unsafe extern "C" fn ast_from_float(
    mut ast: *mut ast_t,
    mut value: libc::c_double,
) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            457 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"ast_from_float\0"))
                .as_ptr(),
        );
    };
    let mut t: *mut token_t = token_dup((*ast).t);
    token_set_id(t, TK_FLOAT);
    token_set_float(t, value);
    let mut new_ast: *mut ast_t = ast_token(t);
    set_scope_no_parent(new_ast, (*ast).parent);
    new_ast
}
#[no_mangle]
#[c2rust::src_loc = "467:1"]
pub unsafe extern "C" fn ast_dup(mut ast: *mut ast_t) -> *mut ast_t {
    return duplicate(0 as *mut ast_t, ast);
}
#[no_mangle]
#[c2rust::src_loc = "472:1"]
pub unsafe extern "C" fn ast_dup_partial(
    mut ast: *mut ast_t,
    mut dup_child: *mut bool,
    mut dup_type: bool,
    mut dup_annotation: bool,
    mut dup_symtab: bool,
) -> *mut ast_t {
    if ast.is_null() {
        return 0 as *mut ast_t;
    }
    if ast_id(ast) as libc::c_uint != TK_PROGRAM as libc::c_int as libc::c_uint
        && ast_id(ast) as libc::c_uint != TK_PACKAGE as libc::c_int as libc::c_uint
        && ast_id(ast) as libc::c_uint != TK_MODULE as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) != TK_PROGRAM && ast_id(ast) != TK_PACKAGE && ast_id(ast) != TK_MODULE\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            479 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_dup_partial\0"))
                .as_ptr(),
        );
    };
    let mut n: *mut ast_t = ast_token(token_dup((*ast).t));
    let ref mut fresh8 = (*n).data;
    *fresh8 = (*ast).data;
    (*n).flags = (*ast).flags & AST_ALL_FLAGS as libc::c_int as libc::c_uint;
    set_scope_no_parent(n, (*ast).parent);
    if dup_annotation {
        ast_setannotation(n, duplicate(0 as *mut ast_t, ast_annotation(ast)));
    }
    if dup_type {
        ast_settype(n, duplicate(0 as *mut ast_t, ast_type(ast)));
    }
    if dup_symtab as libc::c_int != 0 && !((*ast).symtab).is_null() {
        let ref mut fresh9 = (*n).symtab;
        *fresh9 = symtab_dup((*ast).symtab);
    }
    let mut src_child: *mut ast_t = (*ast).child;
    if !src_child.is_null() {
        let mut dst_child: *mut ast_t = 0 as *mut ast_t;
        if *dup_child.offset(0 as libc::c_int as isize) {
            dst_child = duplicate(0 as *mut ast_t, src_child);
        } else {
            dst_child = ast_blank(TK_NONE);
        }
        let ref mut fresh10 = (*n).child;
        *fresh10 = dst_child;
        set_scope_and_parent(dst_child, n);
        src_child = ast_sibling(src_child);
        let mut idx: size_t = 1 as libc::c_int as size_t;
        while !src_child.is_null() {
            let mut dst_sibling: *mut ast_t = 0 as *mut ast_t;
            if *dup_child.offset(idx as isize) {
                dst_sibling = duplicate(0 as *mut ast_t, src_child);
            } else {
                dst_sibling = ast_blank(TK_NONE);
            }
            let ref mut fresh11 = (*dst_child).sibling;
            *fresh11 = dst_sibling;
            dst_child = dst_sibling;
            set_scope_and_parent(dst_sibling, n);
            src_child = ast_sibling(src_child);
            idx = idx.wrapping_add(1);
        }
    }
    return n;
}
#[no_mangle]
#[c2rust::src_loc = "530:1"]
pub unsafe extern "C" fn ast_scope(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            532 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_scope\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            533 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_scope\0")).as_ptr(),
        );
    };
    let ref mut fresh12 = (*ast).symtab;
    *fresh12 = symtab_new();
}
#[no_mangle]
#[c2rust::src_loc = "537:1"]
pub unsafe extern "C" fn ast_has_scope(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            539 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_has_scope\0"))
                .as_ptr(),
        );
    };
    return !((*ast).symtab).is_null();
}
#[no_mangle]
#[c2rust::src_loc = "543:1"]
pub unsafe extern "C" fn ast_set_scope(mut ast: *mut ast_t, mut scope: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            545 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_set_scope\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            546 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_set_scope\0"))
                .as_ptr(),
        );
    };
    if !hasparent(ast) {
    } else {
        ponyint_assert_fail(
            b"!hasparent(ast)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            547 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_set_scope\0"))
                .as_ptr(),
        );
    };
    set_scope_no_parent(ast, scope);
}
#[no_mangle]
#[c2rust::src_loc = "551:1"]
pub unsafe extern "C" fn ast_get_symtab(mut ast: *mut ast_t) -> *mut symtab_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            553 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"ast_get_symtab\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            556 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"ast_get_symtab\0"))
                .as_ptr(),
        );
    };
    return (*ast).symtab;
}
#[no_mangle]
#[c2rust::src_loc = "560:1"]
pub unsafe extern "C" fn ast_setid(mut ast: *mut ast_t, mut id: token_id) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            562 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_setid\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            563 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_setid\0")).as_ptr(),
        );
    };
    token_set_id((*ast).t, id);
    return ast;
}
#[no_mangle]
#[c2rust::src_loc = "568:1"]
pub unsafe extern "C" fn ast_setpos(
    mut ast: *mut ast_t,
    mut source: *mut source_t,
    mut line: size_t,
    mut pos: size_t,
) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            570 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_setpos\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            571 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_setpos\0")).as_ptr(),
        );
    };
    token_set_pos((*ast).t, source, line, pos);
}
#[no_mangle]
#[c2rust::src_loc = "575:1"]
pub unsafe extern "C" fn ast_id(mut ast: *mut ast_t) -> token_id {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            577 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"ast_id\0")).as_ptr(),
        );
    };
    return token_get_id((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "581:1"]
pub unsafe extern "C" fn ast_line(mut ast: *mut ast_t) -> size_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            583 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_line\0")).as_ptr(),
        );
    };
    return token_line_number((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "587:1"]
pub unsafe extern "C" fn ast_pos(mut ast: *mut ast_t) -> size_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            589 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_pos\0")).as_ptr(),
        );
    };
    return token_line_position((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "593:1"]
pub unsafe extern "C" fn ast_source(mut ast: *mut ast_t) -> *mut source_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            595 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_source\0")).as_ptr(),
        );
    };
    return token_source((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "599:1"]
pub unsafe extern "C" fn ast_data(mut ast: *mut ast_t) -> *mut libc::c_void {
    if ast.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*ast).data;
}
#[no_mangle]
#[c2rust::src_loc = "607:1"]
pub unsafe extern "C" fn ast_setdata(
    mut ast: *mut ast_t,
    mut data: *mut libc::c_void,
) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            609 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setdata\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            610 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setdata\0")).as_ptr(),
        );
    };
    let ref mut fresh13 = (*ast).data;
    *fresh13 = data;
    return ast;
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn ast_canerror(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            617 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_canerror\0")).as_ptr(),
        );
    };
    return ast_checkflag(ast, AST_FLAG_CAN_ERROR as libc::c_int as u32) != 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "621:1"]
pub unsafe extern "C" fn ast_seterror(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            623 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_seterror\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            624 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_seterror\0")).as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_CAN_ERROR as libc::c_int as u32);
}
#[no_mangle]
#[c2rust::src_loc = "628:1"]
pub unsafe extern "C" fn ast_cansend(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            630 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_cansend\0")).as_ptr(),
        );
    };
    return ast_checkflag(ast, AST_FLAG_CAN_SEND as libc::c_int as u32) != 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "634:1"]
pub unsafe extern "C" fn ast_setsend(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            636 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setsend\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            637 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setsend\0")).as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_CAN_SEND as libc::c_int as u32);
}
#[no_mangle]
#[c2rust::src_loc = "641:1"]
pub unsafe extern "C" fn ast_mightsend(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            643 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_mightsend\0"))
                .as_ptr(),
        );
    };
    return ast_checkflag(ast, AST_FLAG_MIGHT_SEND as libc::c_int as u32) != 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "647:1"]
pub unsafe extern "C" fn ast_setmightsend(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            649 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_setmightsend\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            650 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_setmightsend\0"))
                .as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_MIGHT_SEND as libc::c_int as u32);
}
#[no_mangle]
#[c2rust::src_loc = "654:1"]
pub unsafe extern "C" fn ast_clearmightsend(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            656 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"ast_clearmightsend\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            657 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"ast_clearmightsend\0"))
                .as_ptr(),
        );
    };
    ast_clearflag(ast, AST_FLAG_MIGHT_SEND as libc::c_int as u32);
}
#[no_mangle]
#[c2rust::src_loc = "661:1"]
pub unsafe extern "C" fn ast_inheritflags(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            663 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_inheritflags\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            664 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_inheritflags\0"))
                .as_ptr(),
        );
    };
    let mut child: *mut ast_t = (*ast).child;
    while !child.is_null() {
        ast_setflag(
            ast,
            (*child).flags & AST_INHERIT_FLAGS as libc::c_int as libc::c_uint,
        );
        child = ast_sibling(child);
    }
}
#[no_mangle]
#[c2rust::src_loc = "670:1"]
pub unsafe extern "C" fn ast_checkflag(mut ast: *mut ast_t, mut flag: u32) -> libc::c_int {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            672 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_checkflag\0"))
                .as_ptr(),
        );
    };
    if flag & AST_ALL_FLAGS as libc::c_int as libc::c_uint == flag {
    } else {
        ponyint_assert_fail(
            b"(flag & AST_ALL_FLAGS) == flag\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            673 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_checkflag\0"))
                .as_ptr(),
        );
    };
    return ((*ast).flags & flag) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "678:1"]
pub unsafe extern "C" fn ast_setflag(mut ast: *mut ast_t, mut flag: u32) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            680 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setflag\0")).as_ptr(),
        );
    };
    if flag & AST_ALL_FLAGS as libc::c_int as libc::c_uint == flag {
    } else {
        ponyint_assert_fail(
            b"(flag & AST_ALL_FLAGS) == flag\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            681 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setflag\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            682 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_setflag\0")).as_ptr(),
        );
    };
    let ref mut fresh14 = (*ast).flags;
    *fresh14 |= flag;
}
#[no_mangle]
#[c2rust::src_loc = "687:1"]
pub unsafe extern "C" fn ast_clearflag(mut ast: *mut ast_t, mut flag: u32) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            689 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_clearflag\0"))
                .as_ptr(),
        );
    };
    if flag & AST_ALL_FLAGS as libc::c_int as libc::c_uint == flag {
    } else {
        ponyint_assert_fail(
            b"(flag & AST_ALL_FLAGS) == flag\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            690 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_clearflag\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            691 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_clearflag\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh15 = (*ast).flags;
    *fresh15 &= !flag;
}
#[no_mangle]
#[c2rust::src_loc = "696:1"]
pub unsafe extern "C" fn ast_resetpass(mut ast: *mut ast_t, mut flag: u32) {
    if flag & AST_FLAG_PASS_MASK as libc::c_int as libc::c_uint == flag {
    } else {
        ponyint_assert_fail(
            b"(flag & AST_FLAG_PASS_MASK) == flag\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            698 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_resetpass\0"))
                .as_ptr(),
        );
    };
    if ast.is_null() {
        return;
    }
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            703 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_resetpass\0"))
                .as_ptr(),
        );
    };
    if ast_checkflag(ast, AST_FLAG_PRESERVE as libc::c_int as u32) != 0 {
        return;
    }
    ast_clearflag(ast, AST_FLAG_PASS_MASK as libc::c_int as u32);
    ast_setflag(ast, flag);
    ast_resetpass(ast_type(ast), flag);
    let mut p: *mut ast_t = ast_child(ast);
    while !p.is_null() {
        ast_resetpass(p, flag);
        p = ast_sibling(p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "716:1"]
pub unsafe extern "C" fn ast_get_print(mut ast: *mut ast_t) -> *const libc::c_char {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            718 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_get_print\0"))
                .as_ptr(),
        );
    };
    return token_print((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "722:1"]
pub unsafe extern "C" fn ast_name(mut ast: *mut ast_t) -> *const libc::c_char {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            724 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_name\0")).as_ptr(),
        );
    };
    return token_string((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "729:1"]
pub unsafe extern "C" fn ast_nice_name(mut ast: *mut ast_t) -> *const libc::c_char {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            731 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_nice_name\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            732 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_nice_name\0"))
                .as_ptr(),
        );
    };
    if !((*ast).data).is_null() {
        return (*ast).data as *const libc::c_char;
    }
    return ast_name(ast);
}
#[no_mangle]
#[c2rust::src_loc = "741:1"]
pub unsafe extern "C" fn ast_name_len(mut ast: *mut ast_t) -> size_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            743 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_name_len\0")).as_ptr(),
        );
    };
    return token_string_len((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "747:1"]
pub unsafe extern "C" fn ast_set_name(mut ast: *mut ast_t, mut name: *const libc::c_char) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            749 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_set_name\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            750 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_set_name\0")).as_ptr(),
        );
    };
    token_set_string((*ast).t, name, 0 as libc::c_int as size_t);
}
#[no_mangle]
#[c2rust::src_loc = "754:1"]
pub unsafe extern "C" fn ast_float(mut ast: *mut ast_t) -> libc::c_double {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            756 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_float\0")).as_ptr(),
        );
    };
    return token_float((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "760:1"]
pub unsafe extern "C" fn ast_int(mut ast: *mut ast_t) -> *mut lexint_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            762 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_int\0")).as_ptr(),
        );
    };
    return token_int((*ast).t);
}
#[no_mangle]
#[c2rust::src_loc = "766:1"]
pub unsafe extern "C" fn ast_type(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            768 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_type\0")).as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
        return 0 as *mut ast_t;
    }
    let mut type_0: *mut ast_t = (*ast).annotation_type;
    if !type_0.is_null()
        && ast_id(type_0) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint
    {
        type_0 = (*type_0).annotation_type;
    }
    return type_0;
}
#[c2rust::src_loc = "783:1"]
unsafe extern "C" fn settype(mut ast: *mut ast_t, mut type_0: *mut ast_t, mut allow_free: bool) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            785 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"settype\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            786 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"settype\0")).as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
        if type_0.is_null() {
        } else {
            ponyint_assert_fail(
                b"type == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                790 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"settype\0")).as_ptr(),
            );
        };
    }
    if !type_0.is_null() {
        if ast_id(type_0) as libc::c_uint != TK_ANNOTATION as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(type) != TK_ANNOTATION\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                794 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"settype\0")).as_ptr(),
            );
        };
    }
    let mut prev_type: *mut ast_t = ast_type(ast);
    if prev_type == type_0 {
        return;
    }
    if !type_0.is_null() {
        if hasparent(type_0) {
            type_0 = duplicate(ast, type_0);
        }
        set_scope_and_parent(type_0, ast);
    }
    if !((*ast).annotation_type).is_null()
        && ast_id((*ast).annotation_type) as libc::c_uint
            == TK_ANNOTATION as libc::c_int as libc::c_uint
    {
        let ref mut fresh16 = (*(*ast).annotation_type).annotation_type;
        *fresh16 = type_0;
    } else {
        let ref mut fresh17 = (*ast).annotation_type;
        *fresh17 = type_0;
    }
    if allow_free {
        ast_free(prev_type);
    }
}
#[no_mangle]
#[c2rust::src_loc = "818:1"]
pub unsafe extern "C" fn ast_settype(mut ast: *mut ast_t, mut type_0: *mut ast_t) {
    settype(ast, type_0, 1 as libc::c_int != 0);
}
#[no_mangle]
#[c2rust::src_loc = "823:1"]
pub unsafe extern "C" fn ast_annotation(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            825 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"ast_annotation\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
        return 0 as *mut ast_t;
    }
    let mut annotation: *mut ast_t = (*ast).annotation_type;
    if !annotation.is_null()
        && ast_id(annotation) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint
    {
        return annotation;
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "839:1"]
pub unsafe extern "C" fn setannotation(
    mut ast: *mut ast_t,
    mut annotation: *mut ast_t,
    mut allow_free: bool,
) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            841 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            842 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
        if annotation.is_null() {
        } else {
            ponyint_assert_fail(
                b"annotation == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                846 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                    .as_ptr(),
            );
        };
    }
    if !annotation.is_null() {
        if ast_id(annotation) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(annotation) == TK_ANNOTATION\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                850 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                    .as_ptr(),
            );
        };
    }
    let mut prev_annotation: *mut ast_t = ast_annotation(ast);
    if prev_annotation == annotation {
        return;
    }
    if !annotation.is_null() {
        if ((*annotation).annotation_type).is_null() {
        } else {
            ponyint_assert_fail(
                b"annotation->annotation_type == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                858 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                    .as_ptr(),
            );
        };
        if hasparent(annotation) {
            annotation = duplicate(ast, annotation);
        }
        if !prev_annotation.is_null() {
            let ref mut fresh18 = (*annotation).annotation_type;
            *fresh18 = (*prev_annotation).annotation_type;
            let ref mut fresh19 = (*prev_annotation).annotation_type;
            *fresh19 = 0 as *mut ast_t;
        } else {
            let ref mut fresh20 = (*annotation).annotation_type;
            *fresh20 = (*ast).annotation_type;
        }
        let ref mut fresh21 = (*ast).annotation_type;
        *fresh21 = annotation;
        set_scope_and_parent(annotation, ast);
    } else {
        if !prev_annotation.is_null() {
        } else {
            ponyint_assert_fail(
                b"prev_annotation != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                875 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"setannotation\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh22 = (*ast).annotation_type;
        *fresh22 = (*prev_annotation).annotation_type;
        let ref mut fresh23 = (*prev_annotation).annotation_type;
        *fresh23 = 0 as *mut ast_t;
    }
    if allow_free {
        ast_free(prev_annotation);
    }
}
#[no_mangle]
#[c2rust::src_loc = "885:1"]
pub unsafe extern "C" fn ast_setannotation(mut ast: *mut ast_t, mut annotation: *mut ast_t) {
    setannotation(ast, annotation, 1 as libc::c_int != 0);
}
#[no_mangle]
#[c2rust::src_loc = "890:1"]
pub unsafe extern "C" fn ast_consumeannotation(mut ast: *mut ast_t) -> *mut ast_t {
    let mut prev_annotation: *mut ast_t = ast_annotation(ast);
    setannotation(ast, 0 as *mut ast_t, 0 as libc::c_int != 0);
    return prev_annotation;
}
#[no_mangle]
#[c2rust::src_loc = "899:1"]
pub unsafe extern "C" fn ast_has_annotation(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            901 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"ast_has_annotation\0"))
                .as_ptr(),
        );
    };
    let mut annotation: *mut ast_t = ast_annotation(ast);
    if !annotation.is_null()
        && ast_id(annotation) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint
    {
        let mut strtab_name: *const libc::c_char = stringtab(name);
        let mut elem: *mut ast_t = ast_child(annotation);
        while !elem.is_null() {
            if ast_name(elem) == strtab_name {
                return 1 as libc::c_int != 0;
            }
            elem = ast_sibling(elem);
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "920:1"]
pub unsafe extern "C" fn ast_erase(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            922 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_erase\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            923 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_erase\0")).as_ptr(),
        );
    };
    let mut child: *mut ast_t = (*ast).child;
    while !child.is_null() {
        let mut next: *mut ast_t = (*child).sibling;
        ast_free(child);
        child = next;
    }
    let ref mut fresh24 = (*ast).child;
    *fresh24 = 0 as *mut ast_t;
    token_set_id((*ast).t, TK_NONE);
}
#[no_mangle]
#[c2rust::src_loc = "938:1"]
pub unsafe extern "C" fn ast_nearest(mut ast: *mut ast_t, mut id: token_id) -> *mut ast_t {
    while !ast.is_null() && token_get_id((*ast).t) as libc::c_uint != id as libc::c_uint {
        ast = (*ast).parent;
    }
    return ast;
}
#[no_mangle]
#[c2rust::src_loc = "946:1"]
pub unsafe extern "C" fn ast_error_handling_clause(
    mut ast: *mut ast_t,
    mut clause: *mut size_t,
) -> *mut ast_t {
    let mut last: *mut ast_t = 0 as *mut ast_t;
    while !ast.is_null() {
        match token_get_id((*ast).t) as libc::c_uint {
            124 | 125 | 206 => {
                *clause = ast_index(last);
                return ast;
            }
            88 | 89 | 90 => return 0 as *mut ast_t,
            _ => {}
        }
        last = ast;
        ast = ast_parent(ast);
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "978:1"]
pub unsafe extern "C" fn ast_parent(mut ast: *mut ast_t) -> *mut ast_t {
    if ast_checkflag(ast, AST_ORPHAN as libc::c_int as u32) != 0 {
        return 0 as *mut ast_t;
    }
    return (*ast).parent;
}
#[no_mangle]
#[c2rust::src_loc = "986:1"]
pub unsafe extern "C" fn ast_child(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            988 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_child\0")).as_ptr(),
        );
    };
    return (*ast).child;
}
#[no_mangle]
#[c2rust::src_loc = "992:1"]
pub unsafe extern "C" fn ast_childidx(mut ast: *mut ast_t, mut idx: size_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            994 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_childidx\0")).as_ptr(),
        );
    };
    let mut child: *mut ast_t = (*ast).child;
    while !child.is_null() && idx > 0 as libc::c_int as libc::c_ulong {
        child = (*child).sibling;
        idx = idx.wrapping_sub(1);
    }
    return child;
}
#[no_mangle]
#[c2rust::src_loc = "1006:1"]
pub unsafe extern "C" fn ast_childlast(mut ast: *mut ast_t) -> *mut ast_t {
    let mut child: *mut ast_t = (*ast).child;
    if child.is_null() {
        return 0 as *mut ast_t;
    }
    while !((*child).sibling).is_null() {
        child = (*child).sibling;
    }
    return child;
}
#[no_mangle]
#[c2rust::src_loc = "1019:1"]
pub unsafe extern "C" fn ast_childcount(mut ast: *mut ast_t) -> size_t {
    let mut child: *mut ast_t = (*ast).child;
    let mut count: size_t = 0;
    while !child.is_null() {
        count = count.wrapping_add(1);
        child = (*child).sibling;
    }
    return count;
}
#[no_mangle]
#[c2rust::src_loc = "1034:1"]
pub unsafe extern "C" fn ast_sibling(mut ast: *mut ast_t) -> *mut ast_t {
    return (*ast).sibling;
}
#[no_mangle]
#[c2rust::src_loc = "1039:1"]
pub unsafe extern "C" fn ast_previous(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1041 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_previous\0")).as_ptr(),
        );
    };
    if hasparent(ast) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"hasparent(ast)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1042 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_previous\0")).as_ptr(),
        );
    };
    let mut last: *mut ast_t = 0 as *mut ast_t;
    let mut cur: *mut ast_t = (*(*ast).parent).child;
    while cur != ast {
        last = cur;
        cur = (*cur).sibling;
    }
    return last;
}
#[no_mangle]
#[c2rust::src_loc = "1056:1"]
pub unsafe extern "C" fn ast_index(mut ast: *mut ast_t) -> size_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1058 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_index\0")).as_ptr(),
        );
    };
    if hasparent(ast) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"hasparent(ast)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1059 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_index\0")).as_ptr(),
        );
    };
    let mut child: *mut ast_t = (*(*ast).parent).child;
    let mut idx: size_t = 0;
    while child != ast {
        child = (*child).sibling;
        idx = idx.wrapping_add(1);
    }
    return idx;
}
#[no_mangle]
#[c2rust::src_loc = "1073:1"]
pub unsafe extern "C" fn ast_get(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut status: *mut sym_status_t,
) -> *mut ast_t {
    if !status.is_null() {
        *status = SYM_NONE;
    }
    loop {
        if !((*ast).symtab).is_null() {
            let mut status2: sym_status_t = SYM_NONE;
            let mut value: *mut ast_t =
                symtab_find((*ast).symtab, name, &mut status2) as *mut ast_t;
            if !status.is_null()
                && *status as libc::c_uint == SYM_NONE as libc::c_int as libc::c_uint
            {
                *status = status2;
            }
            if !value.is_null() {
                return value;
            }
        }
        ast = (*ast).parent;
        if !(!ast.is_null()
            && token_get_id((*ast).t) as libc::c_uint != TK_PROGRAM as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn ast_get_case(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut status: *mut sym_status_t,
) -> *mut ast_t {
    if !status.is_null() {
        *status = SYM_NONE;
    }
    loop {
        if !((*ast).symtab).is_null() {
            let mut status2: sym_status_t = SYM_NONE;
            let mut value: *mut ast_t =
                symtab_find_case((*ast).symtab, name, &mut status2) as *mut ast_t;
            if !status.is_null()
                && *status as libc::c_uint == SYM_NONE as libc::c_int as libc::c_uint
            {
                *status = status2;
            }
            if !value.is_null() {
                return value;
            }
        }
        ast = (*ast).parent;
        if !(!ast.is_null()
            && token_get_id((*ast).t) as libc::c_uint != TK_PROGRAM as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "1128:1"]
pub unsafe extern "C" fn ast_set(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut value: *mut ast_t,
    mut status: sym_status_t,
    mut allow_shadowing: bool,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1131 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_set\0")).as_ptr(),
        );
    };
    while ((*ast).symtab).is_null() {
        ast = (*ast).parent;
    }
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1136 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_set\0")).as_ptr(),
        );
    };
    let mut find: *mut ast_t = 0 as *mut ast_t;
    if allow_shadowing {
        find = symtab_find_case((*ast).symtab, name, 0 as *mut sym_status_t);
    } else {
        find = ast_get_case(ast, name, 0 as *mut sym_status_t);
    }
    if find == value {
        return 1 as libc::c_int != 0;
    }
    if !find.is_null() {
        return 0 as libc::c_int != 0;
    }
    return symtab_add((*ast).symtab, name, value, status);
}
#[no_mangle]
#[c2rust::src_loc = "1158:1"]
pub unsafe extern "C" fn ast_setstatus(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut status: sym_status_t,
) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1160 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_setstatus\0"))
                .as_ptr(),
        );
    };
    while ((*ast).symtab).is_null() {
        ast = (*ast).parent;
    }
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1165 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_setstatus\0"))
                .as_ptr(),
        );
    };
    symtab_set_status((*ast).symtab, name, status);
}
#[no_mangle]
#[c2rust::src_loc = "1169:1"]
pub unsafe extern "C" fn ast_inheritstatus(mut dst: *mut ast_t, mut src: *mut ast_t) {
    if !src.is_null() {
    } else {
        ponyint_assert_fail(
            b"src != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1171 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritstatus\0"))
                .as_ptr(),
        );
    };
    if dst.is_null() {
        return;
    }
    while ((*dst).symtab).is_null() {
        dst = (*dst).parent;
    }
    if !(*dst).frozen {
    } else {
        ponyint_assert_fail(
            b"!dst->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1179 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritstatus\0"))
                .as_ptr(),
        );
    };
    symtab_inherit_status((*dst).symtab, (*src).symtab);
}
#[no_mangle]
#[c2rust::src_loc = "1183:1"]
pub unsafe extern "C" fn ast_inheritbranch(mut dst: *mut ast_t, mut src: *mut ast_t) {
    if !dst.is_null() {
    } else {
        ponyint_assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1185 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritbranch\0"))
                .as_ptr(),
        );
    };
    if !src.is_null() {
    } else {
        ponyint_assert_fail(
            b"src != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1186 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritbranch\0"))
                .as_ptr(),
        );
    };
    if !((*dst).symtab).is_null() {
    } else {
        ponyint_assert_fail(
            b"dst->symtab != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1187 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritbranch\0"))
                .as_ptr(),
        );
    };
    if !((*src).symtab).is_null() {
    } else {
        ponyint_assert_fail(
            b"src->symtab != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1188 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"ast_inheritbranch\0"))
                .as_ptr(),
        );
    };
    symtab_inherit_branch((*dst).symtab, (*src).symtab);
}
#[no_mangle]
#[c2rust::src_loc = "1193:1"]
pub unsafe extern "C" fn ast_consolidate_branches(mut ast: *mut ast_t, mut count: size_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1195 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"ast_consolidate_branches\0",
            ))
            .as_ptr(),
        );
    };
    if hasparent(ast) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"hasparent(ast)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1196 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"ast_consolidate_branches\0",
            ))
            .as_ptr(),
        );
    };
    if !((*ast).symtab).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast->symtab != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1197 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"ast_consolidate_branches\0",
            ))
            .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1198 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"ast_consolidate_branches\0",
            ))
            .as_ptr(),
        );
    };
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut sym: *mut symbol_t = 0 as *mut symbol_t;
    loop {
        sym = symtab_next((*ast).symtab, &mut i);
        if sym.is_null() {
            break;
        }
        let mut status: sym_status_t = SYM_NONE;
        ast_get((*ast).parent, (*sym).name, &mut status);
        if (*sym).status as libc::c_uint == SYM_UNDEFINED as libc::c_int as libc::c_uint {
            if (*sym).branch_count <= count {
            } else {
                ponyint_assert_fail(
                    b"sym->branch_count <= count\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0"
                        as *const u8 as *const libc::c_char,
                    1210 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"ast_consolidate_branches\0",
                    ))
                    .as_ptr(),
                );
            };
            if status as libc::c_uint == SYM_DEFINED as libc::c_int as libc::c_uint
                || (*sym).branch_count == count
            {
                (*sym).status = SYM_DEFINED;
            } else {
                (*sym).branch_count = 0 as libc::c_int as size_t;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1225:1"]
pub unsafe extern "C" fn ast_canmerge(mut dst: *mut ast_t, mut src: *mut ast_t) -> bool {
    if !dst.is_null() {
    } else {
        ponyint_assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1227 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_canmerge\0")).as_ptr(),
        );
    };
    if !src.is_null() {
    } else {
        ponyint_assert_fail(
            b"src != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1228 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ast_canmerge\0")).as_ptr(),
        );
    };
    while ((*dst).symtab).is_null() {
        dst = (*dst).parent;
    }
    return symtab_can_merge_public((*dst).symtab, (*src).symtab);
}
#[no_mangle]
#[c2rust::src_loc = "1236:1"]
pub unsafe extern "C" fn ast_merge(mut dst: *mut ast_t, mut src: *mut ast_t) -> bool {
    if !dst.is_null() {
    } else {
        ponyint_assert_fail(
            b"dst != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1238 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_merge\0")).as_ptr(),
        );
    };
    if !src.is_null() {
    } else {
        ponyint_assert_fail(
            b"src != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1239 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_merge\0")).as_ptr(),
        );
    };
    while ((*dst).symtab).is_null() {
        dst = (*dst).parent;
    }
    if !(*dst).frozen {
    } else {
        ponyint_assert_fail(
            b"!dst->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1244 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_merge\0")).as_ptr(),
        );
    };
    return symtab_merge_public((*dst).symtab, (*src).symtab);
}
#[no_mangle]
#[c2rust::src_loc = "1248:1"]
pub unsafe extern "C" fn ast_within_scope(
    mut outer: *mut ast_t,
    mut inner: *mut ast_t,
    mut name: *const libc::c_char,
) -> bool {
    if !outer.is_null() {
    } else {
        ponyint_assert_fail(
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1250 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_within_scope\0"))
                .as_ptr(),
        );
    };
    if !inner.is_null() {
    } else {
        ponyint_assert_fail(
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1251 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_within_scope\0"))
                .as_ptr(),
        );
    };
    loop {
        if !((*inner).symtab).is_null() {
            let mut status2: sym_status_t = SYM_NONE;
            let mut value: *mut ast_t =
                symtab_find((*inner).symtab, name, &mut status2) as *mut ast_t;
            if !value.is_null() {
                return 1 as libc::c_int != 0;
            }
        }
        if inner == outer {
            break;
        }
        inner = (*inner).parent;
        if !(!inner.is_null()
            && token_get_id((*inner).t) as libc::c_uint
                != TK_PROGRAM as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1273:1"]
pub unsafe extern "C" fn ast_all_consumes_in_scope(
    mut outer: *mut ast_t,
    mut inner: *mut ast_t,
    mut errorf: *mut errorframe_t,
) -> bool {
    if !outer.is_null() {
    } else {
        ponyint_assert_fail(
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1275 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"ast_all_consumes_in_scope\0",
            ))
            .as_ptr(),
        );
    };
    if !inner.is_null() {
    } else {
        ponyint_assert_fail(
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1276 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"ast_all_consumes_in_scope\0",
            ))
            .as_ptr(),
        );
    };
    let mut from: *mut ast_t = inner;
    let mut ok: bool = 1 as libc::c_int != 0;
    loop {
        if !((*inner).symtab).is_null() {
            let mut i: size_t = -(1 as libc::c_int) as size_t;
            let mut sym: *mut symbol_t = 0 as *mut symbol_t;
            loop {
                sym = symtab_next((*inner).symtab, &mut i);
                if sym.is_null() {
                    break;
                }
                if ((*sym).status as libc::c_uint == SYM_CONSUMED as libc::c_int as libc::c_uint
                    || (*sym).status as libc::c_uint
                        == SYM_CONSUMED_SAME_EXPR as libc::c_int as libc::c_uint)
                    && ((*sym).def).is_null()
                {
                    if !ast_within_scope(outer, inner, (*sym).name) {
                        let mut def: *mut ast_t =
                            ast_get(inner, (*sym).name, 0 as *mut sym_status_t);
                        if !errorf.is_null() {
                            ast_error_frame(
                                errorf,
                                from,
                                b"identifier '%s' is consumed when the loop exits\0" as *const u8
                                    as *const libc::c_char,
                                (*sym).name,
                            );
                            ast_error_frame(
                                errorf,
                                def,
                                b"consumed identifier is defined here\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        ok = 0 as libc::c_int != 0;
                    }
                }
            }
        }
        if inner == outer {
            break;
        }
        inner = (*inner).parent;
        if !(!inner.is_null()
            && token_get_id((*inner).t) as libc::c_uint
                != TK_PROGRAM as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "1320:1"]
pub unsafe extern "C" fn ast_clear(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1322 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_clear\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1323 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_clear\0")).as_ptr(),
        );
    };
    if !((*ast).symtab).is_null() {
        symtab_free((*ast).symtab);
        ast_scope(ast);
    }
    let mut child: *mut ast_t = (*ast).child;
    while !child.is_null() {
        ast_clear(child);
        child = (*child).sibling;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1340:1"]
pub unsafe extern "C" fn ast_clear_local(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1342 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_clear_local\0"))
                .as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1343 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_clear_local\0"))
                .as_ptr(),
        );
    };
    if !((*ast).symtab).is_null() {
        symtab_free((*ast).symtab);
        ast_scope(ast);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1352:1"]
pub unsafe extern "C" fn ast_add(mut parent: *mut ast_t, mut child: *mut ast_t) -> *mut ast_t {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1354 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_add\0")).as_ptr(),
        );
    };
    if parent != child {
    } else {
        ponyint_assert_fail(
            b"parent != child\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1355 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_add\0")).as_ptr(),
        );
    };
    if (*parent).child != child {
    } else {
        ponyint_assert_fail(
            b"parent->child != child\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1356 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_add\0")).as_ptr(),
        );
    };
    if !(*parent).frozen {
    } else {
        ponyint_assert_fail(
            b"!parent->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1357 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_add\0")).as_ptr(),
        );
    };
    if hasparent(child) {
        child = ast_dup(child);
    } else {
        if !(*child).frozen {
        } else {
            ponyint_assert_fail(
                b"!child->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1362 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_add\0")).as_ptr(),
            );
        };
    }
    set_scope_and_parent(child, parent);
    let ref mut fresh25 = (*child).sibling;
    *fresh25 = (*parent).child;
    let ref mut fresh26 = (*parent).child;
    *fresh26 = child;
    return child;
}
#[no_mangle]
#[c2rust::src_loc = "1370:1"]
pub unsafe extern "C" fn ast_add_sibling(
    mut older_sibling: *mut ast_t,
    mut new_sibling: *mut ast_t,
) -> *mut ast_t {
    if !older_sibling.is_null() {
    } else {
        ponyint_assert_fail(
            b"older_sibling != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1372 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    if !new_sibling.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_sibling != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1373 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    if older_sibling != new_sibling {
    } else {
        ponyint_assert_fail(
            b"older_sibling != new_sibling\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1374 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    if hasparent(older_sibling) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"hasparent(older_sibling)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1375 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    if !(*(*older_sibling).parent).frozen {
    } else {
        ponyint_assert_fail(
            b"!older_sibling->parent->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1376 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    if hasparent(new_sibling) {
        new_sibling = ast_dup(new_sibling);
    } else {
        if !(*new_sibling).frozen {
        } else {
            ponyint_assert_fail(
                b"!new_sibling->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1381 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                    .as_ptr(),
            );
        };
    }
    if ((*new_sibling).sibling).is_null() {
    } else {
        ponyint_assert_fail(
            b"new_sibling->sibling == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1383 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_add_sibling\0"))
                .as_ptr(),
        );
    };
    set_scope_and_parent(new_sibling, (*older_sibling).parent);
    let ref mut fresh27 = (*new_sibling).sibling;
    *fresh27 = (*older_sibling).sibling;
    let ref mut fresh28 = (*older_sibling).sibling;
    *fresh28 = new_sibling;
    return new_sibling;
}
#[no_mangle]
#[c2rust::src_loc = "1391:1"]
pub unsafe extern "C" fn ast_pop(mut parent: *mut ast_t) -> *mut ast_t {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1393 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_pop\0")).as_ptr(),
        );
    };
    if !(*parent).frozen {
    } else {
        ponyint_assert_fail(
            b"!parent->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1394 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ast_pop\0")).as_ptr(),
        );
    };
    let mut child: *mut ast_t = (*parent).child;
    if !child.is_null() {
        let ref mut fresh29 = (*parent).child;
        *fresh29 = (*child).sibling;
        let ref mut fresh30 = (*child).sibling;
        *fresh30 = 0 as *mut ast_t;
        make_orphan_leave_scope(child);
    }
    return child;
}
#[no_mangle]
#[c2rust::src_loc = "1408:1"]
pub unsafe extern "C" fn ast_append(mut parent: *mut ast_t, mut child: *mut ast_t) -> *mut ast_t {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1410 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_append\0")).as_ptr(),
        );
    };
    if !child.is_null() {
    } else {
        ponyint_assert_fail(
            b"child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1411 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_append\0")).as_ptr(),
        );
    };
    if parent != child {
    } else {
        ponyint_assert_fail(
            b"parent != child\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1412 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_append\0")).as_ptr(),
        );
    };
    if !(*parent).frozen {
    } else {
        ponyint_assert_fail(
            b"!parent->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1413 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_append\0")).as_ptr(),
        );
    };
    if hasparent(child) {
        child = ast_dup(child);
    } else {
        if !(*child).frozen {
        } else {
            ponyint_assert_fail(
                b"!child->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1418 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_append\0"))
                    .as_ptr(),
            );
        };
    }
    set_scope_and_parent(child, parent);
    if ((*parent).child).is_null() {
        let ref mut fresh31 = (*parent).child;
        *fresh31 = child;
        return child;
    }
    let mut ast: *mut ast_t = (*parent).child;
    while !((*ast).sibling).is_null() {
        ast = (*ast).sibling;
    }
    let ref mut fresh32 = (*ast).sibling;
    *fresh32 = child;
    return child;
}
#[no_mangle]
#[c2rust::src_loc = "1437:1"]
pub unsafe extern "C" fn ast_list_append(
    mut parent: *mut ast_t,
    mut last_pointer: *mut *mut ast_t,
    mut new_child: *mut ast_t,
) -> *mut ast_t {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1439 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                .as_ptr(),
        );
    };
    if !last_pointer.is_null() {
    } else {
        ponyint_assert_fail(
            b"last_pointer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1440 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                .as_ptr(),
        );
    };
    if !new_child.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1441 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                .as_ptr(),
        );
    };
    if (*last_pointer).is_null() {
        if ((*parent).child).is_null() {
        } else {
            ponyint_assert_fail(
                b"parent->child == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1446 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                    .as_ptr(),
            );
        };
        *last_pointer = ast_add(parent, new_child);
    } else {
        if !((*parent).child).is_null() {
        } else {
            ponyint_assert_fail(
                b"parent->child != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1452 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                    .as_ptr(),
            );
        };
        if ((**last_pointer).sibling).is_null() {
        } else {
            ponyint_assert_fail(
                b"(*last_pointer)->sibling == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1453 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_list_append\0"))
                    .as_ptr(),
            );
        };
        *last_pointer = ast_add_sibling(*last_pointer, new_child);
    }
    return *last_pointer;
}
#[no_mangle]
#[c2rust::src_loc = "1460:1"]
pub unsafe extern "C" fn ast_remove(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1462 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_remove\0")).as_ptr(),
        );
    };
    if hasparent(ast) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"hasparent(ast)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1463 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_remove\0")).as_ptr(),
        );
    };
    if !(*ast).frozen {
    } else {
        ponyint_assert_fail(
            b"!ast->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1464 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_remove\0")).as_ptr(),
        );
    };
    let mut last: *mut ast_t = ast_previous(ast);
    if !last.is_null() {
        if !(*last).frozen {
        } else {
            ponyint_assert_fail(
                b"!last->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1470 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_remove\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh33 = (*last).sibling;
        *fresh33 = (*ast).sibling;
    } else {
        if !(*(*ast).parent).frozen {
        } else {
            ponyint_assert_fail(
                b"!ast->parent->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1473 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_remove\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh34 = (*(*ast).parent).child;
        *fresh34 = (*ast).sibling;
    }
    let ref mut fresh35 = (*ast).parent;
    *fresh35 = 0 as *mut ast_t;
    let ref mut fresh36 = (*ast).sibling;
    *fresh36 = 0 as *mut ast_t;
    ast_free(ast);
}
#[no_mangle]
#[c2rust::src_loc = "1482:1"]
pub unsafe extern "C" fn ast_swap(mut prev: *mut ast_t, mut next: *mut ast_t) {
    if !prev.is_null() {
    } else {
        ponyint_assert_fail(
            b"prev != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1484 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
        );
    };
    if prev != next {
    } else {
        ponyint_assert_fail(
            b"prev != next\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1485 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
        );
    };
    if !(*prev).frozen {
    } else {
        ponyint_assert_fail(
            b"!prev->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1486 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
        );
    };
    let mut parent: *mut ast_t = ast_parent(prev);
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1489 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
        );
    };
    if !(*parent).frozen {
    } else {
        ponyint_assert_fail(
            b"!parent->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1490 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
        );
    };
    if hasparent(next) {
        next = ast_dup(next);
    } else {
        if !(*next).frozen {
        } else {
            ponyint_assert_fail(
                b"!next->frozen\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1495 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ast_swap\0")).as_ptr(),
            );
        };
    }
    if ast_type(parent) == prev {
        settype(parent, next, 0 as libc::c_int != 0);
    } else {
        let mut last: *mut ast_t = ast_previous(prev);
        if !last.is_null() {
            let ref mut fresh37 = (*last).sibling;
            *fresh37 = next;
        } else {
            let ref mut fresh38 = (*parent).child;
            *fresh38 = next;
        }
        let ref mut fresh39 = (*next).sibling;
        *fresh39 = (*prev).sibling;
    }
    set_scope_and_parent(next, parent);
    let ref mut fresh40 = (*prev).sibling;
    *fresh40 = 0 as *mut ast_t;
    make_orphan_leave_scope(prev);
}
#[no_mangle]
#[c2rust::src_loc = "1517:1"]
pub unsafe extern "C" fn ast_replace(mut prev: *mut *mut ast_t, mut next: *mut ast_t) {
    if !(**prev).frozen {
    } else {
        ponyint_assert_fail(
            b"!(*prev)->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1519 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"ast_replace\0")).as_ptr(),
        );
    };
    if *prev == next {
        return;
    }
    if hasparent(next) {
        next = ast_dup(next);
    }
    if hasparent(*prev) {
        ast_swap(*prev, next);
    }
    ast_free(*prev);
    *prev = next;
}
#[no_mangle]
#[c2rust::src_loc = "1534:1"]
pub unsafe extern "C" fn ast_reorder_children(
    mut ast: *mut ast_t,
    mut new_order: *const size_t,
    mut shuffle_space: *mut *mut ast_t,
) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1537 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_reorder_children\0"))
                .as_ptr(),
        );
    };
    if !new_order.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_order != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1538 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_reorder_children\0"))
                .as_ptr(),
        );
    };
    if !shuffle_space.is_null() {
    } else {
        ponyint_assert_fail(
            b"shuffle_space != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1539 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_reorder_children\0"))
                .as_ptr(),
        );
    };
    let mut count: size_t = ast_childcount(ast);
    let mut i: size_t = 0;
    while i < count {
        let ref mut fresh41 = *shuffle_space.offset(i as isize);
        *fresh41 = ast_pop(ast);
        i = i.wrapping_add(1);
    }
    let mut i_0: size_t = 0;
    while i_0 < count {
        let mut index: size_t = *new_order.offset(i_0 as isize);
        if index < count {
        } else {
            ponyint_assert_fail(
                b"index < count\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1549 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"ast_reorder_children\0",
                ))
                .as_ptr(),
            );
        };
        let mut t: *mut ast_t = *shuffle_space.offset(index as isize);
        if !t.is_null() {
        } else {
            ponyint_assert_fail(
                b"t != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1551 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"ast_reorder_children\0",
                ))
                .as_ptr(),
            );
        };
        ast_append(ast, t);
        let ref mut fresh42 = *shuffle_space.offset(index as isize);
        *fresh42 = 0 as *mut ast_t;
        i_0 = i_0.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1557:1"]
pub unsafe extern "C" fn ast_free(mut ast: *mut ast_t) {
    if ast.is_null() {
        return;
    }
    (*ast).frozen = 0 as libc::c_int != 0;
    let mut child: *mut ast_t = (*ast).child;
    let mut next: *mut ast_t = 0 as *mut ast_t;
    while !child.is_null() {
        next = (*child).sibling;
        ast_free(child);
        child = next;
    }
    ast_settype(ast, 0 as *mut ast_t);
    ast_setannotation(ast, 0 as *mut ast_t);
    match token_get_id((*ast).t) as libc::c_uint {
        136 => {
            program_free((*ast).data as *mut program_t);
        }
        137 => {
            package_free((*ast).data as *mut package_t);
        }
        138 => {
            source_close((*ast).data as *mut source_t);
        }
        _ => {}
    }
    token_free((*ast).t);
    symtab_free((*ast).symtab);
    ponyint_pool_free(1 as libc::c_int as size_t, ast as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1602:1"]
pub unsafe extern "C" fn ast_free_unattached(mut ast: *mut ast_t) {
    if !ast.is_null() && !hasparent(ast) {
        ast_free(ast);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1608:1"]
pub unsafe extern "C" fn ast_is_frozen(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1610 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ast_is_frozen\0"))
                .as_ptr(),
        );
    };
    return (*ast).frozen;
}
#[no_mangle]
#[c2rust::src_loc = "1620:1"]
pub unsafe extern "C" fn ast_freeze(mut ast: *mut ast_t) {
    if ast.is_null() || (*ast).frozen as libc::c_int != 0 {
        return;
    }
    (*ast).frozen = 1 as libc::c_int != 0;
    token_freeze((*ast).t);
    let mut child: *mut ast_t = (*ast).child;
    while !child.is_null() {
        ast_freeze(child);
        child = ast_sibling(child);
    }
    ast_freeze((*ast).annotation_type);
}
#[no_mangle]
#[c2rust::src_loc = "1637:1"]
pub unsafe extern "C" fn ast_print(mut ast: *mut ast_t, mut width: size_t) {
    ast_fprint(__stdoutp, ast, width);
}
#[no_mangle]
#[c2rust::src_loc = "1642:1"]
pub unsafe extern "C" fn ast_printverbose(mut ast: *mut ast_t) {
    ast_fprintverbose(__stdoutp, ast);
}
#[no_mangle]
#[c2rust::src_loc = "1647:1"]
pub unsafe extern "C" fn ast_fprint(mut fp: *mut FILE, mut ast: *mut ast_t, mut width: size_t) {
    if ast.is_null() {
        return;
    }
    print(fp, ast, 0 as libc::c_int as size_t, NOT_SPECIAL, width);
}
#[no_mangle]
#[c2rust::src_loc = "1655:1"]
pub unsafe extern "C" fn ast_fprintverbose(mut fp: *mut FILE, mut ast: *mut ast_t) {
    if ast.is_null() {
        return;
    }
    print_verbose(fp, ast, 0 as libc::c_int as size_t, NOT_SPECIAL);
}
#[c2rust::src_loc = "1665:1"]
unsafe extern "C" fn print_typeexpr(
    mut buffer: *mut printbuf_t,
    mut type_0: *mut ast_t,
    mut sep: *const libc::c_char,
    mut square: bool,
    mut print_cap: bool,
) {
    if square {
        printbuf(buffer, b"[\0" as *const u8 as *const libc::c_char);
    } else {
        printbuf(buffer, b"(\0" as *const u8 as *const libc::c_char);
    }
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        let mut next: *mut ast_t = ast_sibling(child);
        print_type(buffer, child, print_cap);
        if !next.is_null() {
            printbuf(buffer, b"%s\0" as *const u8 as *const libc::c_char, sep);
        }
        child = next;
    }
    if square {
        printbuf(buffer, b"]\0" as *const u8 as *const libc::c_char);
    } else {
        printbuf(buffer, b")\0" as *const u8 as *const libc::c_char);
    };
}
#[c2rust::src_loc = "1692:1"]
unsafe extern "C" fn print_type(
    mut buffer: *mut printbuf_t,
    mut type_0: *mut ast_t,
    mut print_cap: bool,
) {
    match ast_id(type_0) as libc::c_uint {
        151 => {
            let mut package: ast_ptr_t = 0 as *mut ast_t;
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
            let mut cap: ast_ptr_t = 0 as *mut ast_t;
            let mut ephemeral: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 6] = [
                &mut package,
                &mut id,
                &mut typeargs,
                &mut cap,
                &mut ephemeral,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut origpkg: *mut ast_t = ast_sibling(ephemeral);
            if !origpkg.is_null()
                && ast_id(origpkg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
            {
                printbuf(
                    buffer,
                    b"%s.\0" as *const u8 as *const libc::c_char,
                    ast_name(origpkg),
                );
            }
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            if !def.is_null() {
                id = ast_child(def);
            }
            printbuf(
                buffer,
                b"%s\0" as *const u8 as *const libc::c_char,
                ast_nice_name(id),
            );
            if ast_id(typeargs) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                print_typeexpr(
                    buffer,
                    typeargs,
                    b", \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int != 0,
                    1 as libc::c_int != 0,
                );
            }
            if print_cap {
                if ast_id(cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    printbuf(
                        buffer,
                        b" %s\0" as *const u8 as *const libc::c_char,
                        token_print((*cap).t),
                    );
                }
                if ast_id(ephemeral) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    printbuf(
                        buffer,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        token_print((*ephemeral).t),
                    );
                }
            }
        }
        149 => {
            print_typeexpr(
                buffer,
                type_0,
                b" | \0" as *const u8 as *const libc::c_char,
                0 as libc::c_int != 0,
                print_cap,
            );
        }
        56 => {
            print_typeexpr(
                buffer,
                type_0,
                b" & \0" as *const u8 as *const libc::c_char,
                0 as libc::c_int != 0,
                print_cap,
            );
        }
        150 => {
            print_typeexpr(
                buffer,
                type_0,
                b", \0" as *const u8 as *const libc::c_char,
                0 as libc::c_int != 0,
                print_cap,
            );
        }
        187 => {
            let mut id_0: ast_ptr_t = 0 as *mut ast_t;
            let mut cap_0: ast_ptr_t = 0 as *mut ast_t;
            let mut ephemeral_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 4] = [
                &mut id_0,
                &mut cap_0,
                &mut ephemeral_0,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            printbuf(
                buffer,
                b"%s\0" as *const u8 as *const libc::c_char,
                ast_nice_name(id_0),
            );
            if print_cap {
                if ast_id(cap_0) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    printbuf(
                        buffer,
                        b" %s\0" as *const u8 as *const libc::c_char,
                        token_print((*cap_0).t),
                    );
                }
                if ast_id(ephemeral_0) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    printbuf(
                        buffer,
                        b" %s\0" as *const u8 as *const libc::c_char,
                        token_print((*ephemeral_0).t),
                    );
                }
            }
        }
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children_1: [*mut *mut ast_t; 3] =
                [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_1.as_mut_ptr(),
            );
            print_type(buffer, left, print_cap);
            printbuf(buffer, b"->\0" as *const u8 as *const libc::c_char);
            print_type(buffer, right, print_cap);
        }
        152 => {
            printbuf(buffer, b"this\0" as *const u8 as *const libc::c_char);
        }
        156 => {
            printbuf(buffer, b"_\0" as *const u8 as *const libc::c_char);
        }
        153 => {
            printbuf(buffer, b"function\0" as *const u8 as *const libc::c_char);
        }
        157 => {
            printbuf(buffer, b"to_infer\0" as *const u8 as *const libc::c_char);
        }
        158 => {
            printbuf(
                buffer,
                b"<type error>\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {}
        _ => {
            printbuf(
                buffer,
                b"%s\0" as *const u8 as *const libc::c_char,
                token_print((*type_0).t),
            );
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1791:1"]
pub unsafe extern "C" fn ast_print_type(mut type_0: *mut ast_t) -> *const libc::c_char {
    let mut buffer: *mut printbuf_t = printbuf_new();
    print_type(buffer, type_0, 1 as libc::c_int != 0);
    let mut s: *const libc::c_char = stringtab((*buffer).m);
    printbuf_free(buffer);
    return s;
}
#[no_mangle]
#[c2rust::src_loc = "1802:1"]
pub unsafe extern "C" fn ast_print_type_no_cap(mut type_0: *mut ast_t) -> *const libc::c_char {
    let mut buffer: *mut printbuf_t = printbuf_new();
    print_type(buffer, type_0, 0 as libc::c_int != 0);
    let mut s: *const libc::c_char = stringtab((*buffer).m);
    printbuf_free(buffer);
    return s;
}
#[no_mangle]
#[c2rust::src_loc = "1813:1"]
pub unsafe extern "C" fn ast_error(
    mut errors: *mut errors_t,
    mut ast: *mut ast_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv(
        errors,
        token_source((*ast).t),
        token_line_number((*ast).t),
        token_line_position((*ast).t),
        fmt,
        ap.as_va_list(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1822:1"]
pub unsafe extern "C" fn ast_error_continue(
    mut errors: *mut errors_t,
    mut ast: *mut ast_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv_continue(
        errors,
        token_source((*ast).t),
        token_line_number((*ast).t),
        token_line_position((*ast).t),
        fmt,
        ap.as_va_list(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1831:1"]
pub unsafe extern "C" fn ast_error_frame(
    mut frame: *mut errorframe_t,
    mut ast: *mut ast_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if !frame.is_null() {
    } else {
        ponyint_assert_fail(
            b"frame != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1833 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_error_frame\0"))
                .as_ptr(),
        );
    };
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1834 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_error_frame\0"))
                .as_ptr(),
        );
    };
    if !fmt.is_null() {
    } else {
        ponyint_assert_fail(
            b"fmt != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1835 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_error_frame\0"))
                .as_ptr(),
        );
    };
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorframev(
        frame,
        token_source((*ast).t),
        token_line_number((*ast).t),
        token_line_position((*ast).t),
        fmt,
        ap.as_va_list(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1844:1"]
pub unsafe extern "C" fn ast_get_children(
    mut parent: *mut ast_t,
    mut child_count: size_t,
    mut out_children: *mut *mut *mut ast_t,
) {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1847 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_get_children\0"))
                .as_ptr(),
        );
    };
    if child_count > 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"child_count > 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1848 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_get_children\0"))
                .as_ptr(),
        );
    };
    if !out_children.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_children != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1849 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_get_children\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut ast_t = (*parent).child;
    let mut i: size_t = 0;
    while i < child_count {
        if !p.is_null() {
        } else {
            ponyint_assert_fail(
                b"p != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1855 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"ast_get_children\0"))
                    .as_ptr(),
            );
        };
        if !(*out_children.offset(i as isize)).is_null() {
            let ref mut fresh43 = **out_children.offset(i as isize);
            *fresh43 = p;
        }
        p = (*p).sibling;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1864:1"]
pub unsafe extern "C" fn ast_extract_children(
    mut parent: *mut ast_t,
    mut child_count: size_t,
    mut out_children: *mut *mut *mut ast_t,
) {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1867 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_extract_children\0"))
                .as_ptr(),
        );
    };
    if child_count > 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"child_count > 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1868 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_extract_children\0"))
                .as_ptr(),
        );
    };
    if !out_children.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_children != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            1869 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"ast_extract_children\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut ast_t = (*parent).child;
    let mut last_remaining_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut i: size_t = 0;
    while i < child_count {
        if !p.is_null() {
        } else {
            ponyint_assert_fail(
                b"p != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1876 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"ast_extract_children\0",
                ))
                .as_ptr(),
            );
        };
        let mut next: *mut ast_t = (*p).sibling;
        if !(*out_children.offset(i as isize)).is_null() {
            if !last_remaining_sibling.is_null() {
                let ref mut fresh44 = (*last_remaining_sibling).sibling;
                *fresh44 = (*p).sibling;
            } else {
                let ref mut fresh45 = (*parent).child;
                *fresh45 = (*p).sibling;
            }
            make_orphan_leave_scope(p);
            let ref mut fresh46 = (*p).sibling;
            *fresh46 = 0 as *mut ast_t;
            let ref mut fresh47 = **out_children.offset(i as isize);
            *fresh47 = p;
        } else {
            last_remaining_sibling = p;
        }
        p = next;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1899:1"]
pub unsafe extern "C" fn ast_get_provided_symbol_definition(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut status: *mut sym_status_t,
) -> *mut ast_t {
    let mut def: *mut ast_t = 0 as *mut ast_t;
    let mut found: bool = 0 as libc::c_int != 0;
    while !ast.is_null() && !found {
        if ast_id(ast) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
            || ast_id(ast) as libc::c_uint == TK_BE as libc::c_int as libc::c_uint
        {
            let mut body_donor: *mut ast_t = ast_data(ast) as *mut ast_t;
            if !body_donor.is_null() {
                def = ast_get(body_donor, name, status);
            }
            found = 1 as libc::c_int != 0;
        }
        ast = ast_parent(ast);
    }
    return def;
}
#[c2rust::src_loc = "1926:1"]
unsafe extern "C" fn ast_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    let mut id: token_id = ast_id(ast);
    let mut docstring: bool = 0 as libc::c_int != 0;
    if id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        match ast_id(ast_parent(ast)) as libc::c_uint {
            138 | 88 | 89 | 90 | 71 | 72 | 73 | 74 | 75 | 76 | 77 => {
                docstring = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    pony_traceknown(
        ctx,
        (*ast).t as *mut libc::c_void,
        if docstring as libc::c_int != 0 {
            token_docstring_signature_pony_type()
        } else {
            token_signature_pony_type()
        },
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    if id as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
        if !((*ast).child).is_null() {
        } else {
            ponyint_assert_fail(
                b"ast->child != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                    as *const libc::c_char,
                1965 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                    b"ast_signature_serialise_trace\0",
                ))
                .as_ptr(),
            );
        };
        pony_traceknown(
            ctx,
            (*ast).child as *mut libc::c_void,
            if ast_id((*ast).child) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
                ast_nominal_pkg_id_signature_pony_type()
            } else {
                ast_signature_pony_type()
            },
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    } else if !((*ast).child).is_null() {
        pony_traceknown(
            ctx,
            (*ast).child as *mut libc::c_void,
            ast_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if id as libc::c_uint != TK_PACKAGE as libc::c_int as libc::c_uint
        && !((*ast).sibling).is_null()
    {
        pony_traceknown(
            ctx,
            (*ast).sibling as *mut libc::c_void,
            ast_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    let mut annotation: *mut ast_t = ast_annotation(ast);
    if !annotation.is_null() {
        pony_traceknown(
            ctx,
            annotation as *mut libc::c_void,
            ast_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "1988:1"]
unsafe extern "C" fn ast_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut _mutability: libc::c_int,
) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    let mut dst: *mut ast_signature_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut ast_signature_t;
    let ref mut fresh48 = (*dst).t;
    *fresh48 = pony_serialise_offset(ctx, (*ast).t as *mut libc::c_void) as *mut token_signature_t;
    let ref mut fresh49 = (*dst).child;
    *fresh49 =
        pony_serialise_offset(ctx, (*ast).child as *mut libc::c_void) as *mut ast_signature_t;
    if ast_id(ast) as libc::c_uint != TK_PACKAGE as libc::c_int as libc::c_uint {
        let ref mut fresh50 = (*dst).sibling;
        *fresh50 =
            pony_serialise_offset(ctx, (*ast).sibling as *mut libc::c_void) as *mut ast_signature_t;
    } else {
        let ref mut fresh51 = (*dst).sibling;
        *fresh51 = 0 as *mut ast_signature_t;
    }
    let mut annotation: *mut ast_t = ast_annotation(ast);
    let ref mut fresh52 = (*dst).annotation_type;
    *fresh52 = pony_serialise_offset(ctx, annotation as *mut libc::c_void) as *mut ast_signature_t;
}
#[c2rust::src_loc = "2010:20"]
static mut ast_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<ast_signature_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                ast_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                ast_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as u32,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "2031:1"]
pub unsafe extern "C" fn ast_signature_pony_type() -> *const pony_type_t {
    return &ast_signature_pony;
}
#[c2rust::src_loc = "2037:1"]
unsafe extern "C" fn ast_nominal_pkg_id_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    if ast_id(ast) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2042 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"ast_nominal_pkg_id_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    let mut parent: *mut ast_t = (*ast).parent;
    if !parent.is_null()
        && ast_id(parent) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint
        && ast == (*parent).child
    {
    } else {
        ponyint_assert_fail(
            b"(parent != NULL) && (ast_id(parent) == TK_NOMINAL) && (ast == parent->child)\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2046 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"ast_nominal_pkg_id_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    if ((*ast).child).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast->child == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2051 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"ast_nominal_pkg_id_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    if !((*ast).sibling).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast->sibling != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2052 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"ast_nominal_pkg_id_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
    pony_traceknown(
        ctx,
        (*ast).sibling as *mut libc::c_void,
        ast_signature_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    let mut annotation: *mut ast_t = ast_annotation(ast);
    if !annotation.is_null() {
        pony_traceknown(
            ctx,
            annotation as *mut libc::c_void,
            ast_signature_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "2064:1"]
unsafe extern "C" fn ast_nominal_pkg_id_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut _mutability: libc::c_int,
) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    let mut dst: *mut ast_signature_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut ast_signature_t;
    let mut def: *mut ast_t = ast_data(ast_parent(ast)) as *mut ast_t;
    if !def.is_null() {
    } else {
        ponyint_assert_fail(
            b"def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2073 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"ast_nominal_pkg_id_signature_serialise\0",
            ))
            .as_ptr(),
        );
    };
    let mut pkg_ast: *mut ast_t = ast_nearest(def, TK_PACKAGE);
    let mut pkg: *mut package_t = ast_data(pkg_ast) as *mut package_t;
    if !pkg.is_null() {
    } else {
        ponyint_assert_fail(
            b"pkg != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.c\0" as *const u8
                as *const libc::c_char,
            2076 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"ast_nominal_pkg_id_signature_serialise\0",
            ))
            .as_ptr(),
        );
    };
    let ref mut fresh53 = (*dst).t;
    *fresh53 = pony_serialise_offset(ctx, pkg as *mut libc::c_void) as *mut token_signature_t;
    let ref mut fresh54 = (*dst).child;
    *fresh54 =
        pony_serialise_offset(ctx, (*ast).child as *mut libc::c_void) as *mut ast_signature_t;
    let ref mut fresh55 = (*dst).sibling;
    *fresh55 =
        pony_serialise_offset(ctx, (*ast).sibling as *mut libc::c_void) as *mut ast_signature_t;
    let mut annotation: *mut ast_t = ast_annotation(ast);
    let ref mut fresh56 = (*dst).annotation_type;
    *fresh56 = pony_serialise_offset(ctx, annotation as *mut libc::c_void) as *mut ast_signature_t;
}
#[c2rust::src_loc = "2088:20"]
static mut ast_nominal_pkg_id_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<ast_signature_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                ast_nominal_pkg_id_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                ast_nominal_pkg_id_signature_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: None,
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as u32,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "2109:1"]
pub unsafe extern "C" fn ast_nominal_pkg_id_signature_pony_type() -> *const pony_type_t {
    return &ast_nominal_pkg_id_signature_pony;
}
#[c2rust::src_loc = "2114:1"]
unsafe extern "C" fn ast_serialise_trace_data(mut ctx: *mut pony_ctx_t, mut ast: *mut ast_t) {
    if ((*ast).data).is_null() {
        return;
    }
    match ast_id(ast) as libc::c_uint {
        70 | 151 | 88 | 90 | 89 | 163 | 187 | 184 | 185 | 186 | 198 | 196 | 197 | 192 | 193
        | 194 | 188 | 189 | 190 | 191 | 200 | 201 | 202 | 203 | 204 | 19 | 143 | 160 => {
            pony_traceknown(
                ctx,
                (*ast).data,
                ast_pony_type(),
                PONY_TRACE_MUTABLE as libc::c_int,
            );
        }
        8 | 78 => {
            string_trace(ctx, (*ast).data as *const libc::c_char);
        }
        136 => {
            pony_traceknown(
                ctx,
                (*ast).data,
                program_pony_type(),
                PONY_TRACE_MUTABLE as libc::c_int,
            );
        }
        137 => {
            pony_traceknown(
                ctx,
                (*ast).data,
                package_pony_type(),
                PONY_TRACE_MUTABLE as libc::c_int,
            );
        }
        138 => {
            pony_traceknown(
                ctx,
                (*ast).data,
                source_pony_type(),
                PONY_TRACE_MUTABLE as libc::c_int,
            );
        }
        _ => {}
    };
}
#[c2rust::src_loc = "2173:1"]
unsafe extern "C" fn ast_serialise_data(
    mut ctx: *mut pony_ctx_t,
    mut ast: *mut ast_t,
    mut dst: *mut ast_t,
) {
    match ast_id(ast) as libc::c_uint {
        70 | 151 | 88 | 90 | 89 | 163 | 187 | 184 | 185 | 186 | 198 | 196 | 197 | 192 | 193
        | 194 | 188 | 189 | 190 | 191 | 200 | 201 | 202 | 203 | 204 | 19 | 143 | 160 | 8 | 78
        | 136 | 137 | 138 => {
            let ref mut fresh57 = (*dst).data;
            *fresh57 = pony_serialise_offset(ctx, (*ast).data) as *mut libc::c_void;
        }
        161 => {
            operatorliteral_serialise_data(ast, dst);
        }
        _ => {
            let ref mut fresh58 = (*dst).data;
            *fresh58 = 0 as *mut libc::c_void;
        }
    };
}
#[c2rust::src_loc = "2223:1"]
unsafe extern "C" fn ast_deserialise_data(mut ctx: *mut pony_ctx_t, mut ast: *mut ast_t) {
    match ast_id(ast) as libc::c_uint {
        70 | 151 | 88 | 90 | 89 | 163 | 187 | 184 | 185 | 186 | 198 | 196 | 197 | 192 | 193
        | 194 | 188 | 189 | 190 | 191 | 200 | 201 | 202 | 203 | 204 | 19 | 143 | 160 => {
            let ref mut fresh59 = (*ast).data;
            *fresh59 = pony_deserialise_offset(ctx, ast_pony_type(), (*ast).data as uintptr_t);
        }
        8 | 78 => {
            let ref mut fresh60 = (*ast).data;
            *fresh60 =
                string_deserialise_offset(ctx, (*ast).data as uintptr_t) as *mut libc::c_void;
        }
        136 => {
            let ref mut fresh61 = (*ast).data;
            *fresh61 = pony_deserialise_offset(ctx, program_pony_type(), (*ast).data as uintptr_t);
        }
        137 => {
            let ref mut fresh62 = (*ast).data;
            *fresh62 = pony_deserialise_offset(ctx, package_pony_type(), (*ast).data as uintptr_t);
        }
        138 => {
            let ref mut fresh63 = (*ast).data;
            *fresh63 = pony_deserialise_offset(ctx, source_pony_type(), (*ast).data as uintptr_t);
        }
        161 => {
            operatorliteral_deserialise_data(ast);
        }
        _ => {}
    };
}
#[c2rust::src_loc = "2287:1"]
unsafe extern "C" fn ast_serialise_trace(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    pony_traceknown(
        ctx,
        (*ast).t as *mut libc::c_void,
        token_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    ast_serialise_trace_data(ctx, ast);
    if !((*ast).symtab).is_null() {
        pony_traceknown(
            ctx,
            (*ast).symtab as *mut libc::c_void,
            symtab_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*ast).parent).is_null() {
        pony_traceknown(
            ctx,
            (*ast).parent as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*ast).child).is_null() {
        pony_traceknown(
            ctx,
            (*ast).child as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*ast).sibling).is_null() {
        pony_traceknown(
            ctx,
            (*ast).sibling as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*ast).annotation_type).is_null() {
        pony_traceknown(
            ctx,
            (*ast).annotation_type as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "2310:1"]
unsafe extern "C" fn ast_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut _mutability: libc::c_int,
) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    let mut dst: *mut ast_t = (buf as uintptr_t).wrapping_add(offset) as *mut ast_t;
    let ref mut fresh64 = (*dst).t;
    *fresh64 = pony_serialise_offset(ctx, (*ast).t as *mut libc::c_void) as *mut token_t;
    ast_serialise_data(ctx, ast, dst);
    let ref mut fresh65 = (*dst).symtab;
    *fresh65 = pony_serialise_offset(ctx, (*ast).symtab as *mut libc::c_void) as *mut symtab_t;
    let ref mut fresh66 = (*dst).parent;
    *fresh66 = pony_serialise_offset(ctx, (*ast).parent as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh67 = (*dst).child;
    *fresh67 = pony_serialise_offset(ctx, (*ast).child as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh68 = (*dst).sibling;
    *fresh68 = pony_serialise_offset(ctx, (*ast).sibling as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh69 = (*dst).annotation_type;
    *fresh69 =
        pony_serialise_offset(ctx, (*ast).annotation_type as *mut libc::c_void) as *mut ast_t;
    (*dst).flags = (*ast).flags;
    (*dst).frozen = (*ast).frozen;
}
#[c2rust::src_loc = "2331:1"]
unsafe extern "C" fn ast_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut ast: *mut ast_t = object as *mut ast_t;
    let ref mut fresh70 = (*ast).t;
    *fresh70 =
        pony_deserialise_offset(ctx, token_pony_type(), (*ast).t as uintptr_t) as *mut token_t;
    ast_deserialise_data(ctx, ast);
    let ref mut fresh71 = (*ast).symtab;
    *fresh71 = pony_deserialise_offset(ctx, symtab_pony_type(), (*ast).symtab as uintptr_t)
        as *mut symtab_t;
    let ref mut fresh72 = (*ast).parent;
    *fresh72 =
        pony_deserialise_offset(ctx, ast_pony_type(), (*ast).parent as uintptr_t) as *mut ast_t;
    let ref mut fresh73 = (*ast).child;
    *fresh73 =
        pony_deserialise_offset(ctx, ast_pony_type(), (*ast).child as uintptr_t) as *mut ast_t;
    let ref mut fresh74 = (*ast).sibling;
    *fresh74 =
        pony_deserialise_offset(ctx, ast_pony_type(), (*ast).sibling as uintptr_t) as *mut ast_t;
    let ref mut fresh75 = (*ast).annotation_type;
    *fresh75 = pony_deserialise_offset(ctx, ast_pony_type(), (*ast).annotation_type as uintptr_t)
        as *mut ast_t;
}
#[c2rust::src_loc = "2350:20"]
static mut ast_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<ast_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                ast_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                ast_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                ast_deserialise as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as u32,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "2371:1"]
pub unsafe extern "C" fn ast_pony_type() -> *const pony_type_t {
    return &ast_pony;
}
