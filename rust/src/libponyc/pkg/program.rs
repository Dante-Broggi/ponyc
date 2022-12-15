use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = i64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
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
        pub traits: *mut *mut libc::uintptr_t,
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
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> usize>;
    #[c2rust::src_loc = "95:1"]
    pub type pony_custom_serialise_space_fn =
        Option<unsafe extern "C" fn(*mut libc::c_void) -> usize>;
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
    use super::pony_h::_pony_type_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    use super::_size_t_h::size_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_append(list: *mut strlist_t, data: *const libc::c_char) -> *mut strlist_t;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_next(list: *mut strlist_t) -> *mut strlist_t;
        #[c2rust::src_loc = "9:34"]
        pub fn strlist_data(list: *mut strlist_t) -> *const libc::c_char;
        #[c2rust::src_loc = "9:34"]
        pub fn strlist_find(list: *mut strlist_t, data: *const libc::c_char)
            -> *const libc::c_char;
        #[c2rust::src_loc = "9:1"]
        pub fn strlist_free(list: *mut strlist_t);
        #[c2rust::src_loc = "20:1"]
        pub fn stringtab_consume(
            string: *const libc::c_char,
            buf_size: usize,
        ) -> *const libc::c_char;
        #[c2rust::src_loc = "29:1"]
        pub fn strlist_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/frame.h:1"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct typecheck_frame_t {
        pub package: *mut ast_t,
        pub module: *mut ast_t,
        pub type_0: *mut ast_t,
        pub constraint: *mut ast_t,
        pub provides: *mut ast_t,
        pub method: *mut ast_t,
        pub def_arg: *mut ast_t,
        pub method_body: *mut ast_t,
        pub method_params: *mut ast_t,
        pub method_type: *mut ast_t,
        pub ffi_type: *mut ast_t,
        pub local_type: *mut ast_t,
        pub as_type: *mut ast_t,
        pub the_case: *mut ast_t,
        pub pattern: *mut ast_t,
        pub loop_0: *mut ast_t,
        pub loop_cond: *mut ast_t,
        pub loop_body: *mut ast_t,
        pub loop_else: *mut ast_t,
        pub try_expr: *mut ast_t,
        pub recover: *mut ast_t,
        pub ifdef_cond: *mut ast_t,
        pub ifdef_clause: *mut ast_t,
        pub iftype_constraint: *mut ast_t,
        pub iftype_body: *mut ast_t,
        pub prev: *mut typecheck_frame_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct typecheck_stats_t {
        pub names_count: usize,
        pub default_caps_count: usize,
        pub heap_alloc: usize,
        pub stack_alloc: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct typecheck_t {
        pub frame: *mut typecheck_frame_t,
        pub stats: typecheck_stats_t,
        pub errors: *mut errors_t,
    }
    use super::_size_t_h::size_t;
    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.h:1"]
pub mod pass_h {
    #[c2rust::src_loc = "205:9"]
    pub type verbosity_level = libc::c_uint;
    #[c2rust::src_loc = "211:3"]
    pub const VERBOSITY_ALL: verbosity_level = 4;
    #[c2rust::src_loc = "210:3"]
    pub const VERBOSITY_TOOL_INFO: verbosity_level = 3;
    #[c2rust::src_loc = "209:3"]
    pub const VERBOSITY_INFO: verbosity_level = 2;
    #[c2rust::src_loc = "208:3"]
    pub const VERBOSITY_MINIMAL: verbosity_level = 1;
    #[c2rust::src_loc = "207:3"]
    pub const VERBOSITY_QUIET: verbosity_level = 0;
    #[c2rust::src_loc = "216:9"]
    pub type pass_id = libc::c_uint;
    #[c2rust::src_loc = "239:3"]
    pub const PASS_ALL: pass_id = 21;
    #[c2rust::src_loc = "238:3"]
    pub const PASS_OBJ: pass_id = 20;
    #[c2rust::src_loc = "237:3"]
    pub const PASS_ASM: pass_id = 19;
    #[c2rust::src_loc = "236:3"]
    pub const PASS_BITCODE: pass_id = 18;
    #[c2rust::src_loc = "235:3"]
    pub const PASS_LLVM_IR: pass_id = 17;
    #[c2rust::src_loc = "234:3"]
    pub const PASS_PAINT: pass_id = 16;
    #[c2rust::src_loc = "233:3"]
    pub const PASS_REACH: pass_id = 15;
    #[c2rust::src_loc = "232:3"]
    pub const PASS_SERIALISER: pass_id = 14;
    #[c2rust::src_loc = "231:3"]
    pub const PASS_FINALISER: pass_id = 13;
    #[c2rust::src_loc = "230:3"]
    pub const PASS_VERIFY: pass_id = 12;
    #[c2rust::src_loc = "229:3"]
    pub const PASS_COMPLETENESS: pass_id = 11;
    #[c2rust::src_loc = "228:3"]
    pub const PASS_EXPR: pass_id = 10;
    #[c2rust::src_loc = "227:3"]
    pub const PASS_REFER: pass_id = 9;
    #[c2rust::src_loc = "226:3"]
    pub const PASS_DOCS: pass_id = 8;
    #[c2rust::src_loc = "225:3"]
    pub const PASS_TRAITS: pass_id = 7;
    #[c2rust::src_loc = "224:3"]
    pub const PASS_FLATTEN: pass_id = 6;
    #[c2rust::src_loc = "223:3"]
    pub const PASS_NAME_RESOLUTION: pass_id = 5;
    #[c2rust::src_loc = "222:3"]
    pub const PASS_IMPORT: pass_id = 4;
    #[c2rust::src_loc = "221:3"]
    pub const PASS_SCOPE: pass_id = 3;
    #[c2rust::src_loc = "220:3"]
    pub const PASS_SUGAR: pass_id = 2;
    #[c2rust::src_loc = "219:3"]
    pub const PASS_SYNTAX: pass_id = 1;
    #[c2rust::src_loc = "218:3"]
    pub const PASS_PARSE: pass_id = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "274:16"]
    pub struct pass_opt_t {
        pub limit: pass_id,
        pub program_pass: pass_id,
        pub release: bool,
        pub library: bool,
        pub runtimebc: bool,
        pub staticbin: bool,
        pub pic: bool,
        pub print_stats: bool,
        pub verify: bool,
        pub extfun: bool,
        pub strip_debug: bool,
        pub print_filenames: bool,
        pub check_tree: bool,
        pub lint_llvm: bool,
        pub docs: bool,
        pub docs_private: bool,
        pub verbosity: verbosity_level,
        pub ast_print_width: usize,
        pub allow_test_symbols: bool,
        pub parse_trace: bool,
        pub package_search_paths: *mut strlist_t,
        pub safe_packages: *mut strlist_t,
        pub magic_packages: *mut magic_package_t,
        pub argv0: *const libc::c_char,
        pub all_args: *const libc::c_char,
        pub output: *const libc::c_char,
        pub bin_name: *const libc::c_char,
        pub link_arch: *mut libc::c_char,
        pub linker: *mut libc::c_char,
        pub link_ldcmd: *mut libc::c_char,
        pub llvm_args: *const libc::c_char,
        pub triple: *mut libc::c_char,
        pub abi: *mut libc::c_char,
        pub cpu: *mut libc::c_char,
        pub features: *mut libc::c_char,
        pub check: typecheck_t,
        pub plugins: *mut plugins_t,
        pub data: *mut libc::c_void,
    }
    use super::_size_t_h::size_t;
    use super::frame_h::typecheck_t;
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:2"]
pub mod package_h {
    use super::source_h::pony_type_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "20:44"]
        pub type package_group_list_t;
        #[c2rust::src_loc = "14:16"]
        pub type package_group_t;
        #[c2rust::src_loc = "20:1"]
        pub fn package_group_list_free(list: *mut package_group_list_t);
        #[c2rust::src_loc = "20:1"]
        pub fn package_group_list_pony_type() -> *const pony_type_t;
        #[c2rust::src_loc = "20:44"]
        pub fn package_group_list_next(
            list: *mut package_group_list_t,
        ) -> *mut package_group_list_t;
        #[c2rust::src_loc = "21:3"]
        pub fn package_group_list_data(list: *mut package_group_list_t) -> *mut package_group_t;
        #[c2rust::src_loc = "30:1"]
        pub fn path_cat(
            part1: *const libc::c_char,
            part2: *const libc::c_char,
            result: *mut libc::c_char,
        );
        #[c2rust::src_loc = "121:1"]
        pub fn package_path(package: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "179:1"]
        pub fn package_dependency_groups(first_package: *mut ast_t) -> *mut package_group_list_t;
        #[c2rust::src_loc = "181:1"]
        pub fn package_group_signature(group: *mut package_group_t) -> *const libc::c_char;
        #[c2rust::src_loc = "183:1"]
        pub fn package_group_dump(group: *mut package_group_t);
        #[c2rust::src_loc = "202:1"]
        pub fn is_path_absolute(path: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/lib/blake2/blake2.h:7"]
pub mod blake2_h {
    #[c2rust::src_loc = "40:3"]
    pub type blake2b_state = blake2b_state__;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:18"]
    pub struct blake2b_state__ {
        pub h: [u64; 8],
        pub t: [u64; 2],
        pub f: [u64; 2],
        pub buf: [u8; 128],
        pub buflen: usize,
        pub outlen: usize,
        pub last_node: u8,
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "75:3"]
        pub fn blake2b_init(S: *mut blake2b_state, outlen: usize) -> libc::c_int;
        #[c2rust::src_loc = "78:3"]
        pub fn blake2b_update(
            S: *mut blake2b_state,
            in_0: *const libc::c_void,
            inlen: usize,
        ) -> libc::c_int;
        #[c2rust::src_loc = "79:3"]
        pub fn blake2b_final(
            S: *mut blake2b_state,
            out: *mut libc::c_void,
            outlen: usize,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "68:14"]
        pub static mut __stdoutp: *mut FILE;
        #[c2rust::src_loc = "157:1"]
        pub fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "177:1"]
        pub fn putchar(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "178:1"]
        pub fn puts(_: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:4"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn pony_deserialise_block(
            ctx: *mut pony_ctx_t,
            offset: libc::uintptr_t,
            size: usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: libc::uintptr_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
        #[c2rust::src_loc = "37:1"]
        pub fn pony_serialise_reserve(ctx: *mut pony_ctx_t, p: *mut libc::c_void, size: usize);
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:8"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "75:7"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "86:7"]
        pub fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __int64_t};
pub use self::_uintptr_t_h::uintptr_t;
use self::ast_h::{ast_child, ast_data, ast_error, ast_id, ast_nearest};
pub use self::blake2_h::{
    blake2b_final, blake2b_init, blake2b_state, blake2b_state__, blake2b_update,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::package_h::{
    is_path_absolute, package_dependency_groups, package_group_dump, package_group_list_data,
    package_group_list_free, package_group_list_next, package_group_list_pony_type,
    package_group_list_t, package_group_signature, package_group_t, package_path, path_cat,
};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
pub use self::pony_h::{
    _pony_type_t, pony_actor_t, pony_ctx_t, pony_custom_deserialise_fn,
    pony_custom_serialise_space_fn, pony_dispatch_fn, pony_final_fn, pony_msg_t, pony_serialise_fn,
    pony_trace_fn, pony_traceknown, C2RustUnnamed, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE,
    PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
use self::serialise_h::{
    pony_deserialise_block, pony_deserialise_offset, pony_serialise_offset, pony_serialise_reserve,
};
pub use self::source_h::pony_type_t;
use self::stdio_h::{__stdoutp, fputs, printf, putchar, puts};
use self::string_h::{memcpy, strcat, strlen, strpbrk};
use self::stringtab_h::{
    stringtab_consume, strlist_append, strlist_data, strlist_find, strlist_free, strlist_next,
    strlist_pony_type, strlist_t,
};
use self::symtab_h::ast_t;
pub use self::sys__types_h::__darwin_off_t;
pub use self::token_h::{
    token_id, TK_ACTOR, TK_ADDRESS, TK_ALIASED, TK_AND, TK_ANNOTATION, TK_ARRAY, TK_ARROW, TK_AS,
    TK_ASSIGN, TK_AT, TK_AT_LBRACE, TK_BACKSLASH, TK_BARELAMBDA, TK_BARELAMBDATYPE, TK_BE,
    TK_BEAPP, TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK, TK_CALL, TK_CAP_ALIAS, TK_CAP_ANY,
    TK_CAP_READ, TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES, TK_CHAIN, TK_CLASS, TK_COLON,
    TK_COMMA, TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT, TK_CONSUME, TK_CONTINUE,
    TK_DBLARROW, TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE, TK_DIVIDE_TILDE, TK_DO, TK_DONTCARE,
    TK_DONTCAREREF, TK_DONTCARETYPE, TK_DOT, TK_ELLIPSIS, TK_ELSE, TK_ELSEIF, TK_EMBED,
    TK_EMBEDREF, TK_END, TK_EOF, TK_EPHEMERAL, TK_EQ, TK_EQ_TILDE, TK_ERROR, TK_ERRORTYPE,
    TK_FALSE, TK_FFICALL, TK_FFIDECL, TK_FLATTEN, TK_FLET, TK_FLETREF, TK_FLOAT, TK_FOR, TK_FUN,
    TK_FUNAPP, TK_FUNCHAIN, TK_FUNREF, TK_FUNTYPE, TK_FVAR, TK_FVARREF, TK_GE, TK_GE_TILDE, TK_GT,
    TK_GT_TILDE, TK_ID, TK_IF, TK_IFDEF, TK_IFDEFAND, TK_IFDEFFLAG, TK_IFDEFNOT, TK_IFDEFOR,
    TK_IFTYPE, TK_IFTYPE_SET, TK_IN, TK_INFERTYPE, TK_INT, TK_INTERFACE, TK_IS, TK_ISECTTYPE,
    TK_ISNT, TK_ISO, TK_LAMBDA, TK_LAMBDACAPTURE, TK_LAMBDACAPTURES, TK_LAMBDATYPE, TK_LBRACE,
    TK_LE, TK_LET, TK_LETREF, TK_LEX_ERROR, TK_LE_TILDE, TK_LITERAL, TK_LITERALBRANCH, TK_LOCATION,
    TK_LPAREN, TK_LPAREN_NEW, TK_LSHIFT, TK_LSHIFT_TILDE, TK_LSQUARE, TK_LSQUARE_NEW, TK_LT,
    TK_LT_TILDE, TK_MATCH, TK_MATCH_CAPTURE, TK_MATCH_DONTCARE, TK_MEMBERS, TK_MINUS, TK_MINUS_NEW,
    TK_MINUS_TILDE, TK_MINUS_TILDE_NEW, TK_MOD, TK_MODULE, TK_MOD_TILDE, TK_MULTIPLY,
    TK_MULTIPLY_TILDE, TK_NAMEDARG, TK_NAMEDARGS, TK_NE, TK_NEW, TK_NEWAPP, TK_NEWBEREF,
    TK_NEWLINE, TK_NEWREF, TK_NE_TILDE, TK_NOMINAL, TK_NONE, TK_NOT, TK_OBJECT, TK_OPERATORLITERAL,
    TK_OR, TK_PACKAGE, TK_PACKAGEREF, TK_PARAM, TK_PARAMREF, TK_PARAMS, TK_PIPE, TK_PLUS,
    TK_PLUS_TILDE, TK_POSITIONALARGS, TK_PRIMITIVE, TK_PROGRAM, TK_PROVIDES, TK_QUALIFY,
    TK_QUESTION, TK_RBRACE, TK_RECOVER, TK_REF, TK_REFERENCE, TK_REM, TK_REM_TILDE, TK_REPEAT,
    TK_RETURN, TK_RPAREN, TK_RSHIFT, TK_RSHIFT_TILDE, TK_RSQUARE, TK_SEMI, TK_SEQ, TK_STRING,
    TK_STRUCT, TK_SUBTYPE, TK_TAG, TK_TEST_ALIASED, TK_TEST_EXTRA, TK_TEST_NO_SEQ,
    TK_TEST_SEQ_SCOPE, TK_TEST_TRY_NO_CHECK, TK_TEST_UPDATEARG, TK_THEN, TK_THIS, TK_THISTYPE,
    TK_TILDE, TK_TRAIT, TK_TRN, TK_TRUE, TK_TRY, TK_TRY_NO_CHECK, TK_TUPLE, TK_TUPLEELEMREF,
    TK_TUPLETYPE, TK_TYPE, TK_TYPEARGS, TK_TYPEPARAM, TK_TYPEPARAMREF, TK_TYPEPARAMS, TK_TYPEREF,
    TK_UNARY_MINUS, TK_UNARY_MINUS_TILDE, TK_UNIONTYPE, TK_UNTIL, TK_UPDATEARG, TK_USE, TK_VAL,
    TK_VALUEFORMALARG, TK_VALUEFORMALPARAM, TK_VAR, TK_VARREF, TK_WHERE, TK_WHILE, TK_WITH, TK_XOR,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "12:16"]
pub struct program_t {
    pub package_groups: *mut package_group_list_t,
    pub signature: *mut libc::c_char,
    pub next_package_id: u32,
    pub libpaths: *mut strlist_t,
    pub libs: *mut strlist_t,
    pub lib_args_size: usize,
    pub lib_args_alloced: usize,
    pub lib_args: *mut libc::c_char,
}
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn append_to_args(mut program: *mut program_t, mut text: *const libc::c_char) {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            28 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"append_to_args\0"))
                .as_ptr(),
        );
    };
    if !text.is_null() {
    } else {
        ponyint_assert_fail(
            b"text != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            29 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"append_to_args\0"))
                .as_ptr(),
        );
    };
    let mut text_len: usize = libc::strlen(text);
    let mut new_len: usize = ((*program).lib_args_size)
        .wrapping_add(text_len)
        .wrapping_add(1);
    if new_len > (*program).lib_args_alloced {
        let mut new_alloc: usize = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(new_len.try_into().unwrap())
            .try_into()
            .unwrap();
        let mut new_args: *mut libc::c_char =
            ponyint_pool_alloc_size(new_alloc) as *mut libc::c_char;
        memcpy(
            new_args as *mut libc::c_void,
            (*program).lib_args as *const libc::c_void,
            ((*program).lib_args_size)
                .wrapping_add(1)
                .try_into()
                .unwrap(),
        );
        ponyint_pool_free_size(
            (*program).lib_args_alloced,
            (*program).lib_args as *mut libc::c_void,
        );
        let ref mut fresh0 = (*program).lib_args;
        *fresh0 = new_args;
        (*program).lib_args_alloced = new_alloc;
    }
    strcat((*program).lib_args, text);
    (*program).lib_args_size = new_len.wrapping_sub(1);
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn program_create() -> *mut program_t {
    let mut p: *mut program_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut program_t;
    let ref mut fresh1 = (*p).package_groups;
    *fresh1 = 0 as *mut package_group_list_t;
    let ref mut fresh2 = (*p).signature;
    *fresh2 = 0 as *mut libc::c_char;
    (*p).next_package_id = 0 as libc::c_int as u32;
    let ref mut fresh3 = (*p).libpaths;
    *fresh3 = 0 as *mut strlist_t;
    let ref mut fresh4 = (*p).libs;
    *fresh4 = 0 as *mut strlist_t;
    (*p).lib_args_size = -(1 as libc::c_int) as usize;
    let ref mut fresh5 = (*p).lib_args;
    *fresh5 = 0 as *mut libc::c_char;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn program_free(mut program: *mut program_t) {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"program_free\0")).as_ptr(),
        );
    };
    package_group_list_free((*program).package_groups);
    if !((*program).signature).is_null() {
        ponyint_pool_free_size(
            64 as libc::c_int as usize,
            (*program).signature as *mut libc::c_void,
        );
    }
    strlist_free((*program).libpaths);
    strlist_free((*program).libs);
    if !((*program).lib_args).is_null() {
        ponyint_pool_free_size(
            (*program).lib_args_alloced,
            (*program).lib_args as *mut libc::c_void,
        );
    }
    ponyint_pool_free(1 as libc::c_int as usize, program as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn program_assign_pkg_id(mut ast: *mut ast_t) -> u32 {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            85 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"program_assign_pkg_id\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_PROGRAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"program_assign_pkg_id\0"))
                .as_ptr(),
        );
    };
    let mut data: *mut program_t = ast_data(ast) as *mut program_t;
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            89 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"program_assign_pkg_id\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh6 = (*data).next_package_id;
    let fresh7 = *fresh6;
    *fresh6 = (*fresh6).wrapping_add(1);
    return fresh7;
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn quoted_locator(
    mut opt: *mut pass_opt_t,
    mut use_0: *mut ast_t,
    mut locator: *const libc::c_char,
) -> *const libc::c_char {
    if !locator.is_null() {
    } else {
        ponyint_assert_fail(
            b"locator != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            103 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"quoted_locator\0"))
                .as_ptr(),
        );
    };
    if !(strpbrk(
        locator,
        b"\t\r\n\"'`;$|&<>%*?[]{}()\0" as *const u8 as *const libc::c_char,
    ))
    .is_null()
    {
        if !use_0.is_null() {
            ast_error(
                (*opt).check.errors,
                use_0,
                b"use URI contains invalid characters\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as *const libc::c_char;
    }
    let mut len: usize = libc::strlen(locator);
    let mut quoted: *mut libc::c_char = ponyint_pool_alloc_size(
        len.wrapping_add((3 as libc::c_int as libc::c_ulong).try_into().unwrap()),
    ) as *mut libc::c_char;
    *quoted.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_char;
    memcpy(
        quoted.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        locator as *const libc::c_void,
        len.try_into().unwrap(),
    );
    *quoted.offset(len.wrapping_add(1) as isize) = '"' as i32 as libc::c_char;
    *quoted.offset(
        len.wrapping_add((2 as libc::c_int as libc::c_ulong).try_into().unwrap()) as isize,
    ) = '\0' as i32 as libc::c_char;
    return stringtab_consume(
        quoted,
        len.wrapping_add((3 as libc::c_int as libc::c_ulong).try_into().unwrap()),
    );
}
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub unsafe extern "C" fn use_library(
    mut use_0: *mut ast_t,
    mut locator: *const libc::c_char,
    mut _name: *mut ast_t,
    mut options: *mut pass_opt_t,
) -> bool {
    let mut libname: *const libc::c_char = quoted_locator(options, use_0, locator);
    if libname.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut p: *mut ast_t = ast_nearest(use_0, TK_PROGRAM);
    let mut prog: *mut program_t = ast_data(p) as *mut program_t;
    if ((*prog).lib_args).is_null() {
    } else {
        ponyint_assert_fail(
            b"prog->lib_args == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            136 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"use_library\0")).as_ptr(),
        );
    };
    if !(strlist_find((*prog).libs, libname)).is_null() {
        return 1 as libc::c_int != 0;
    }
    let ref mut fresh8 = (*prog).libs;
    *fresh8 = strlist_append((*prog).libs, libname);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn use_path(
    mut use_0: *mut ast_t,
    mut locator: *const libc::c_char,
    mut _name: *mut ast_t,
    mut options: *mut pass_opt_t,
) -> bool {
    let mut absolute: [libc::c_char; 1024] = [0; 1024];
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    if !is_path_absolute(locator) {
        let mut pkg_ast: *mut ast_t = ast_nearest(use_0, TK_PACKAGE);
        prefix = package_path(pkg_ast);
    }
    path_cat(prefix, locator, absolute.as_mut_ptr());
    let mut libpath: *const libc::c_char = quoted_locator(options, use_0, absolute.as_mut_ptr());
    if libpath.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut prog_ast: *mut ast_t = ast_nearest(use_0, TK_PROGRAM);
    let mut prog: *mut program_t = ast_data(prog_ast) as *mut program_t;
    if ((*prog).lib_args).is_null() {
    } else {
        ponyint_assert_fail(
            b"prog->lib_args == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            167 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"use_path\0")).as_ptr(),
        );
    };
    if !(strlist_find((*prog).libpaths, libpath)).is_null() {
        return 1 as libc::c_int != 0;
    }
    let ref mut fresh9 = (*prog).libpaths;
    *fresh9 = strlist_append((*prog).libpaths, libpath);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn program_lib_build_args(
    mut program: *mut ast_t,
    mut opt: *mut pass_opt_t,
    mut path_preamble: *const libc::c_char,
    mut rpath_preamble: *const libc::c_char,
    mut global_preamble: *const libc::c_char,
    mut global_postamble: *const libc::c_char,
    mut lib_premable: *const libc::c_char,
    mut lib_postamble: *const libc::c_char,
) {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            182 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if ast_id(program) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(program) == TK_PROGRAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if !global_preamble.is_null() {
    } else {
        ponyint_assert_fail(
            b"global_preamble != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            184 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if !global_postamble.is_null() {
    } else {
        ponyint_assert_fail(
            b"global_postamble != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            185 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if !lib_premable.is_null() {
    } else {
        ponyint_assert_fail(
            b"lib_premable != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if !lib_postamble.is_null() {
    } else {
        ponyint_assert_fail(
            b"lib_postamble != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            187 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    let mut data: *mut program_t = ast_data(program) as *mut program_t;
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            190 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    if ((*data).lib_args).is_null() {
    } else {
        ponyint_assert_fail(
            b"data->lib_args == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            191 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"program_lib_build_args\0",
            ))
            .as_ptr(),
        );
    };
    (*data).lib_args_alloced = 256 as libc::c_int as usize;
    let ref mut fresh10 = (*data).lib_args;
    *fresh10 = ponyint_pool_alloc_size((*data).lib_args_alloced) as *mut libc::c_char;
    *((*data).lib_args).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*data).lib_args_size = 0 as libc::c_int as usize;
    let mut p: *mut strlist_t = (*data).libpaths;
    while !p.is_null() {
        let mut libpath: *const libc::c_char = strlist_data(p);
        append_to_args(data, path_preamble);
        append_to_args(data, libpath);
        append_to_args(data, b" \0" as *const u8 as *const libc::c_char);
        if !rpath_preamble.is_null() {
            append_to_args(data, rpath_preamble);
            append_to_args(data, libpath);
            append_to_args(data, b" \0" as *const u8 as *const libc::c_char);
        }
        p = strlist_next(p);
    }
    let mut p_0: *mut strlist_t = (*opt).package_search_paths;
    while !p_0.is_null() {
        let mut libpath_0: *const libc::c_char =
            quoted_locator(opt, 0 as *mut ast_t, strlist_data(p_0));
        if !libpath_0.is_null() {
            append_to_args(data, path_preamble);
            append_to_args(data, libpath_0);
            append_to_args(data, b" \0" as *const u8 as *const libc::c_char);
            if !rpath_preamble.is_null() {
                append_to_args(data, rpath_preamble);
                append_to_args(data, libpath_0);
                append_to_args(data, b" \0" as *const u8 as *const libc::c_char);
            }
        }
        p_0 = strlist_next(p_0);
    }
    append_to_args(data, global_preamble);
    let mut p_1: *mut strlist_t = (*data).libs;
    while !p_1.is_null() {
        let mut lib: *const libc::c_char = strlist_data(p_1);
        let mut amble: bool = !is_path_absolute(&*lib.offset(1 as libc::c_int as isize));
        if amble {
            append_to_args(data, lib_premable);
        }
        append_to_args(data, lib);
        if amble {
            append_to_args(data, lib_postamble);
        }
        append_to_args(data, b" \0" as *const u8 as *const libc::c_char);
        p_1 = strlist_next(p_1);
    }
    append_to_args(data, global_postamble);
}
#[no_mangle]
#[c2rust::src_loc = "258:1"]
pub unsafe extern "C" fn program_lib_args(mut program: *mut ast_t) -> *const libc::c_char {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            260 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"program_lib_args\0"))
                .as_ptr(),
        );
    };
    if ast_id(program) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(program) == TK_PROGRAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            261 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"program_lib_args\0"))
                .as_ptr(),
        );
    };
    let mut data: *mut program_t = ast_data(program) as *mut program_t;
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            264 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"program_lib_args\0"))
                .as_ptr(),
        );
    };
    if !((*data).lib_args).is_null() {
    } else {
        ponyint_assert_fail(
            b"data->lib_args != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"program_lib_args\0"))
                .as_ptr(),
        );
    };
    return (*data).lib_args;
}
#[no_mangle]
#[c2rust::src_loc = "271:1"]
pub unsafe extern "C" fn program_signature(mut program: *mut ast_t) -> *const libc::c_char {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            273 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                .as_ptr(),
        );
    };
    if ast_id(program) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(program) == TK_PROGRAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            274 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                .as_ptr(),
        );
    };
    let mut data: *mut program_t = ast_data(program) as *mut program_t;
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            277 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                .as_ptr(),
        );
    };
    if ((*data).signature).is_null() {
        let mut first_package: *mut ast_t = ast_child(program);
        if !first_package.is_null() {
        } else {
            ponyint_assert_fail(
                b"first_package != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0"
                    as *const u8 as *const libc::c_char,
                282 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                    .as_ptr(),
            );
        };
        if ((*data).package_groups).is_null() {
        } else {
            ponyint_assert_fail(
                b"data->package_groups == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0"
                    as *const u8 as *const libc::c_char,
                284 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh11 = (*data).package_groups;
        *fresh11 = package_dependency_groups(first_package);
        let mut hash_state: blake2b_state = blake2b_state {
            h: [0; 8],
            t: [0; 2],
            f: [0; 2],
            buf: [0; 128],
            buflen: 0,
            outlen: 0,
            last_node: 0,
        };
        let mut status: libc::c_int = blake2b_init(&mut hash_state, 64 as libc::c_int as usize);
        if status == 0 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"status == 0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0"
                    as *const u8 as *const libc::c_char,
                290 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                    .as_ptr(),
            );
        };
        let mut iter: *mut package_group_list_t = (*data).package_groups;
        while !iter.is_null() {
            let mut group: *mut package_group_t = package_group_list_data(iter);
            let mut group_sig: *const libc::c_char = package_group_signature(group);
            blake2b_update(
                &mut hash_state,
                group_sig as *const libc::c_void,
                64 as libc::c_int as usize,
            );
            iter = package_group_list_next(iter);
        }
        let ref mut fresh12 = (*data).signature;
        *fresh12 = ponyint_pool_alloc_size(64 as libc::c_int as usize) as *mut libc::c_char;
        status = blake2b_final(
            &mut hash_state,
            (*data).signature as *mut libc::c_void,
            64 as libc::c_int as usize,
        );
        if status == 0 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"status == 0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0"
                    as *const u8 as *const libc::c_char,
                304 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"program_signature\0"))
                    .as_ptr(),
            );
        };
    }
    return (*data).signature;
}
#[c2rust::src_loc = "311:1"]
unsafe extern "C" fn print_signature(mut sig: *const libc::c_char) {
    let mut i: usize = 0;
    while i < (64 as libc::c_int as libc::c_ulong).try_into().unwrap() {
        printf(
            b"%02hhX\0" as *const u8 as *const libc::c_char,
            *sig.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "318:1"]
pub unsafe extern "C" fn program_dump(mut program: *mut ast_t) {
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            320 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"program_dump\0")).as_ptr(),
        );
    };
    if ast_id(program) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(program) == TK_PROGRAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            321 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"program_dump\0")).as_ptr(),
        );
    };
    let mut data: *mut program_t = ast_data(program) as *mut program_t;
    if !data.is_null() {
    } else {
        ponyint_assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.c\0" as *const u8
                as *const libc::c_char,
            324 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"program_dump\0")).as_ptr(),
        );
    };
    let mut signature: *const libc::c_char = program_signature(program);
    fputs(
        b"Program signature: \0" as *const u8 as *const libc::c_char,
        __stdoutp,
    );
    print_signature(signature);
    puts(b"\n\0" as *const u8 as *const libc::c_char);
    let mut i: usize = 0;
    let mut iter: *mut package_group_list_t = (*data).package_groups;
    while !iter.is_null() {
        printf(b"Group %zu\n\0" as *const u8 as *const libc::c_char, i);
        let mut group: *mut package_group_t = package_group_list_data(iter);
        package_group_dump(group);
        putchar('\n' as i32);
        iter = package_group_list_next(iter);
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn program_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut program: *mut program_t = object as *mut program_t;
    if !((*program).package_groups).is_null() {
        pony_traceknown(
            ctx,
            (*program).package_groups as *mut libc::c_void,
            package_group_list_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*program).signature).is_null() {
        pony_serialise_reserve(
            ctx,
            (*program).signature as *mut libc::c_void,
            64 as libc::c_int as usize,
        );
    }
    if !((*program).libpaths).is_null() {
        pony_traceknown(
            ctx,
            (*program).libpaths as *mut libc::c_void,
            strlist_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*program).libs).is_null() {
        pony_traceknown(
            ctx,
            (*program).libs as *mut libc::c_void,
            strlist_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*program).lib_args).is_null() {
        pony_serialise_reserve(
            ctx,
            (*program).lib_args as *mut libc::c_void,
            ((*program).lib_args_size).wrapping_add(1),
        );
    }
}
#[c2rust::src_loc = "369:1"]
unsafe extern "C" fn program_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut program: *mut program_t = object as *mut program_t;
    let mut dst: *mut program_t = (buf as libc::uintptr_t).wrapping_add(offset) as *mut program_t;
    let ref mut fresh13 = (*dst).package_groups;
    *fresh13 = pony_serialise_offset(ctx, (*program).package_groups as *mut libc::c_void)
        as *mut package_group_list_t;
    let mut ptr_offset: libc::uintptr_t =
        pony_serialise_offset(ctx, (*program).signature as *mut libc::c_void);
    let ref mut fresh14 = (*dst).signature;
    *fresh14 = ptr_offset as *mut libc::c_char;
    if !((*program).signature).is_null() {
        let mut dst_sig: *mut libc::c_char =
            (buf as libc::uintptr_t).wrapping_add(ptr_offset) as *mut libc::c_char;
        memcpy(
            dst_sig as *mut libc::c_void,
            (*program).signature as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
    }
    (*dst).next_package_id = (*program).next_package_id;
    let ref mut fresh15 = (*dst).libpaths;
    *fresh15 =
        pony_serialise_offset(ctx, (*program).libpaths as *mut libc::c_void) as *mut strlist_t;
    let ref mut fresh16 = (*dst).libs;
    *fresh16 = pony_serialise_offset(ctx, (*program).libs as *mut libc::c_void) as *mut strlist_t;
    (*dst).lib_args_size = (*program).lib_args_size;
    (*dst).lib_args_alloced = ((*program).lib_args_size).wrapping_add(1);
    ptr_offset = pony_serialise_offset(ctx, (*program).lib_args as *mut libc::c_void);
    let ref mut fresh17 = (*dst).lib_args;
    *fresh17 = ptr_offset as *mut libc::c_char;
    if !((*dst).lib_args).is_null() {
        let mut dst_lib: *mut libc::c_char =
            (buf as libc::uintptr_t).wrapping_add(ptr_offset) as *mut libc::c_char;
        memcpy(
            dst_lib as *mut libc::c_void,
            (*program).lib_args as *const libc::c_void,
            ((*program).lib_args_size)
                .wrapping_add(1)
                .try_into()
                .unwrap(),
        );
    }
}
#[c2rust::src_loc = "405:1"]
unsafe extern "C" fn program_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut program: *mut program_t = object as *mut program_t;
    let ref mut fresh18 = (*program).package_groups;
    *fresh18 = pony_deserialise_offset(
        ctx,
        package_group_list_pony_type(),
        (*program).package_groups as libc::uintptr_t,
    ) as *mut package_group_list_t;
    let ref mut fresh19 = (*program).signature;
    *fresh19 = pony_deserialise_block(
        ctx,
        (*program).signature as libc::uintptr_t,
        64 as libc::c_int as usize,
    ) as *mut libc::c_char;
    let ref mut fresh20 = (*program).libpaths;
    *fresh20 = pony_deserialise_offset(
        ctx,
        strlist_pony_type(),
        (*program).libpaths as libc::uintptr_t,
    ) as *mut strlist_t;
    let ref mut fresh21 = (*program).libs;
    *fresh21 = pony_deserialise_offset(ctx, strlist_pony_type(), (*program).libs as libc::uintptr_t)
        as *mut strlist_t;
    let ref mut fresh22 = (*program).lib_args;
    *fresh22 = pony_deserialise_block(
        ctx,
        (*program).lib_args as libc::uintptr_t,
        ((*program).lib_args_size)
            .wrapping_add((1 as libc::c_int as libc::c_ulong).try_into().unwrap()),
    ) as *mut libc::c_char;
}
#[c2rust::src_loc = "422:20"]
static mut program_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<program_t>() as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                program_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                program_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                program_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as u32,
            traits: 0 as *const *mut libc::uintptr_t as *mut *mut libc::uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "444:1"]
pub unsafe extern "C" fn program_pony_type() -> *const pony_type_t {
    &program_pony
}
