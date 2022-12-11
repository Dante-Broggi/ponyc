use ::libc;
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
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexint.h:1"]
pub mod lexint_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct lexint_t {
        pub low: uint64_t,
        pub high: uint64_t,
    }
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:22"]
    pub struct _pony_type_t {
        pub id: uint32_t,
        pub size: uint32_t,
        pub field_count: uint32_t,
        pub field_offset: uint32_t,
        pub instance: *mut libc::c_void,
        pub trace: pony_trace_fn,
        pub serialise_trace: pony_trace_fn,
        pub serialise: pony_serialise_fn,
        pub deserialise: pony_trace_fn,
        pub custom_serialise_space: pony_custom_serialise_space_fn,
        pub custom_deserialise: pony_custom_deserialise_fn,
        pub dispatch: pony_dispatch_fn,
        pub final_0: pony_final_fn,
        pub event_notify: uint32_t,
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
        pub index: uint32_t,
        pub id: uint32_t,
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
    use super::_uint32_t_h::uint32_t;
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
        #[c2rust::src_loc = "37:1"]
        pub fn source_pony_type() -> *const pony_type_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:2"]
pub mod lexer_h {
    use super::token_h::{token_id};
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn lexer_print(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:3"]
pub mod stringtab_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn string_trace_len(ctx: *mut pony_ctx_t, string: *const libc::c_char, len: size_t);
        #[c2rust::src_loc = "15:1"]
        pub fn stringtab_len(string: *const libc::c_char, len: size_t) -> *const libc::c_char;
        #[c2rust::src_loc = "27:1"]
        pub fn string_deserialise_offset(
            ctx: *mut pony_ctx_t,
            offset: uintptr_t,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:4"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> size_t;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: uintptr_t,
        ) -> *mut libc::c_void;
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
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:7"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:9"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "80:9"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uintptr_t_h::uintptr_t;
use self::lexer_h::lexer_print;
pub use self::lexint_h::lexint_t;
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
use self::serialise_h::{pony_deserialise_offset, pony_serialise_offset};
pub use self::source_h::{pony_type_t, source_pony_type, source_t};
use self::stdio_h::snprintf;
use self::string_h::{memcpy, memset, strcspn, strlen};
use self::stringtab_h::{string_deserialise_offset, string_trace_len, stringtab_len};
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
#[c2rust::src_loc = "11:8"]
pub struct token_t {
    pub id: token_id,
    pub source: *mut source_t,
    pub line: size_t,
    pub pos: size_t,
    pub printed: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub frozen: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "19:3"]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub real: libc::c_double,
    pub integer: lexint_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "21:5"]
pub struct C2RustUnnamed_1 {
    pub string: *const libc::c_char,
    pub str_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:8"]
pub struct token_signature_t {
    pub id: token_id,
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "42:3"]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub real: libc::c_double,
    pub integer: lexint_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "44:5"]
pub struct C2RustUnnamed_3 {
    pub string: *const libc::c_char,
    pub str_length: size_t,
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn token_new(mut id: token_id) -> *mut token_t {
    let mut t: *mut token_t = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut token_t;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<token_t>() as libc::c_ulong,
    );
    (*t).id = id;
    return t;
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn token_dup(mut token: *mut token_t) -> *mut token_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"token_dup\0")).as_ptr(),
        );
    };
    let mut t: *mut token_t = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut token_t;
    memcpy(
        t as *mut libc::c_void,
        token as *const libc::c_void,
        ::core::mem::size_of::<token_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*t).printed;
    *fresh0 = 0 as *mut libc::c_char;
    (*t).frozen = 0 as libc::c_int != 0;
    return t;
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn token_dup_new_id(
    mut token: *mut token_t,
    mut id: token_id,
) -> *mut token_t {
    let mut new_token: *mut token_t = token_dup(token);
    (*new_token).id = id;
    return new_token;
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn token_free(mut token: *mut token_t) {
    if token.is_null() {
        return;
    }
    if !((*token).printed).is_null() {
        ponyint_pool_free_size(
            64 as libc::c_int as size_t,
            (*token).printed as *mut libc::c_void,
        );
    }
    ponyint_pool_free(1 as libc::c_int as size_t, token as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn token_freeze(mut token: *mut token_t) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            101 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_freeze\0")).as_ptr(),
        );
    };
    (*token).frozen = 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn token_get_id(mut token: *mut token_t) -> token_id {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            111 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_get_id\0")).as_ptr(),
        );
    };
    return (*token).id;
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn token_string(mut token: *mut token_t) -> *const libc::c_char {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            118 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_string\0")).as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_STRING || token->id == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_string\0")).as_ptr(),
        );
    };
    return (*token).c2rust_unnamed.c2rust_unnamed.string;
}
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub unsafe extern "C" fn token_string_len(mut token: *mut token_t) -> size_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_string_len\0"))
                .as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_STRING || token->id == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            127 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_string_len\0"))
                .as_ptr(),
        );
    };
    return (*token).c2rust_unnamed.c2rust_unnamed.str_length;
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn token_float(mut token: *mut token_t) -> libc::c_double {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            134 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"token_float\0")).as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_FLOAT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_FLOAT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            135 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"token_float\0")).as_ptr(),
        );
    };
    return (*token).c2rust_unnamed.real;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn token_int(mut token: *mut token_t) -> *mut lexint_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            142 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"token_int\0")).as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_INT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_INT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            143 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"token_int\0")).as_ptr(),
        );
    };
    return &mut (*token).c2rust_unnamed.integer;
}
#[no_mangle]
#[c2rust::src_loc = "148:1"]
pub unsafe extern "C" fn token_print(mut token: *mut token_t) -> *const libc::c_char {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            150 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"token_print\0")).as_ptr(),
        );
    };
    match (*token).id as libc::c_uint {
        0 => return b"EOF\0" as *const u8 as *const libc::c_char,
        8 | 5 => return (*token).c2rust_unnamed.c2rust_unnamed.string,
        6 => {
            if ((*token).printed).is_null() {
                let ref mut fresh1 = (*token).printed;
                *fresh1 = ponyint_pool_alloc_size(64 as libc::c_int as size_t) as *mut libc::c_char;
            }
            snprintf(
                (*token).printed,
                64 as libc::c_int as libc::c_ulong,
                b"%llu\0" as *const u8 as *const libc::c_char,
                (*token).c2rust_unnamed.integer.low,
            );
            return (*token).printed;
        }
        7 => {
            if ((*token).printed).is_null() {
                let ref mut fresh2 = (*token).printed;
                *fresh2 = ponyint_pool_alloc_size(64 as libc::c_int as size_t) as *mut libc::c_char;
            }
            let mut r: libc::c_int = snprintf(
                (*token).printed,
                64 as libc::c_int as libc::c_ulong,
                b"%g\0" as *const u8 as *const libc::c_char,
                (*token).c2rust_unnamed.real,
            );
            if strcspn(
                (*token).printed,
                b".e\0" as *const u8 as *const libc::c_char,
            ) == r as size_t
            {
                snprintf(
                    ((*token).printed).offset(r as isize),
                    (64 as libc::c_int - r) as libc::c_ulong,
                    b".0\0" as *const u8 as *const libc::c_char,
                );
            }
            return (*token).printed;
        }
        1 => return b"LEX_ERROR\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    let mut p: *const libc::c_char = lexer_print((*token).id);
    if !p.is_null() {
        return p;
    }
    if ((*token).printed).is_null() {
        let ref mut fresh3 = (*token).printed;
        *fresh3 = ponyint_pool_alloc_size(64 as libc::c_int as size_t) as *mut libc::c_char;
    }
    snprintf(
        (*token).printed,
        64 as libc::c_int as libc::c_ulong,
        b"Unknown_token_%d\0" as *const u8 as *const libc::c_char,
        (*token).id as libc::c_uint,
    );
    return (*token).printed;
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn token_print_escaped(mut token: *mut token_t) -> *mut libc::c_char {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"token_print_escaped\0"))
                .as_ptr(),
        );
    };
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut str_len: size_t = 0 as libc::c_int as size_t;
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        str = (*token).c2rust_unnamed.c2rust_unnamed.string;
        str_len = (*token).c2rust_unnamed.c2rust_unnamed.str_length;
    } else {
        str = token_print(token);
        str_len = strlen(str);
    }
    let mut escapes: size_t = 0 as libc::c_int as size_t;
    let mut idx: size_t = 0 as libc::c_int as size_t;
    while idx < str_len {
        let mut c: libc::c_char = *str.offset(idx as isize);
        if c as libc::c_int == '"' as i32
            || c as libc::c_int == '\\' as i32
            || c as libc::c_int == 0 as libc::c_int
        {
            escapes = escapes.wrapping_add(1);
        }
        idx = idx.wrapping_add(1);
    }
    if escapes == 0 as libc::c_int as libc::c_ulong {
        let mut copy: *mut libc::c_char =
            ponyint_pool_alloc_size(str_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
        memcpy(
            copy as *mut libc::c_void,
            str as *const libc::c_void,
            str_len,
        );
        *copy.offset(str_len as isize) = 0 as libc::c_int as libc::c_char;
        return copy;
    }
    let mut escaped_len: size_t = str_len.wrapping_add(escapes);
    let mut escaped: *mut libc::c_char =
        ponyint_pool_alloc_size(escaped_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    let mut escaped_idx: size_t = 0 as libc::c_int as size_t;
    let mut idx_0: size_t = 0 as libc::c_int as size_t;
    while idx_0 < str_len {
        let mut c_0: libc::c_char = *str.offset(idx_0 as isize);
        if c_0 as libc::c_int == '"' as i32 || c_0 as libc::c_int == '\\' as i32 {
            let fresh4 = escaped_idx;
            escaped_idx = escaped_idx.wrapping_add(1);
            *escaped.offset(fresh4 as isize) = '\\' as i32 as libc::c_char;
            let fresh5 = escaped_idx;
            escaped_idx = escaped_idx.wrapping_add(1);
            *escaped.offset(fresh5 as isize) = c_0;
        } else if c_0 as libc::c_int == 0 as libc::c_int {
            let fresh6 = escaped_idx;
            escaped_idx = escaped_idx.wrapping_add(1);
            *escaped.offset(fresh6 as isize) = '\\' as i32 as libc::c_char;
            let fresh7 = escaped_idx;
            escaped_idx = escaped_idx.wrapping_add(1);
            *escaped.offset(fresh7 as isize) = '0' as i32 as libc::c_char;
        } else {
            let fresh8 = escaped_idx;
            escaped_idx = escaped_idx.wrapping_add(1);
            *escaped.offset(fresh8 as isize) = c_0;
        }
        idx_0 = idx_0.wrapping_add(1);
    }
    let fresh9 = escaped_idx;
    escaped_idx = escaped_idx.wrapping_add(1);
    *escaped.offset(fresh9 as isize) = 0 as libc::c_int as libc::c_char;
    return escaped;
}
#[no_mangle]
#[c2rust::src_loc = "260:1"]
pub unsafe extern "C" fn token_id_desc(mut id: token_id) -> *const libc::c_char {
    match id as libc::c_uint {
        0 => return b"EOF\0" as *const u8 as *const libc::c_char,
        8 => return b"id\0" as *const u8 as *const libc::c_char,
        5 => return b"string literal\0" as *const u8 as *const libc::c_char,
        6 => return b"int literal\0" as *const u8 as *const libc::c_char,
        7 => return b"float literal\0" as *const u8 as *const libc::c_char,
        3 => return b"true literal\0" as *const u8 as *const libc::c_char,
        4 => return b"false literal\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    let mut p: *const libc::c_char = lexer_print(id);
    if !p.is_null() {
        return p;
    }
    return b"UNKOWN\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "282:1"]
pub unsafe extern "C" fn token_source(mut token: *mut token_t) -> *mut source_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            284 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_source\0")).as_ptr(),
        );
    };
    return (*token).source;
}
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn token_line_number(mut token: *mut token_t) -> size_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            291 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"token_line_number\0"))
                .as_ptr(),
        );
    };
    return (*token).line;
}
#[no_mangle]
#[c2rust::src_loc = "296:1"]
pub unsafe extern "C" fn token_line_position(mut token: *mut token_t) -> size_t {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            298 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"token_line_position\0"))
                .as_ptr(),
        );
    };
    return (*token).pos;
}
#[no_mangle]
#[c2rust::src_loc = "305:1"]
pub unsafe extern "C" fn token_set_id(mut token: *mut token_t, mut id: token_id) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            307 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_set_id\0")).as_ptr(),
        );
    };
    if !(*token).frozen {
    } else {
        ponyint_assert_fail(
            b"!token->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"token_set_id\0")).as_ptr(),
        );
    };
    (*token).id = id;
}
#[no_mangle]
#[c2rust::src_loc = "313:1"]
pub unsafe extern "C" fn token_set_string(
    mut token: *mut token_t,
    mut value: *const libc::c_char,
    mut length: size_t,
) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            315 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_set_string\0"))
                .as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_STRING || token->id == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            316 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_set_string\0"))
                .as_ptr(),
        );
    };
    if !(*token).frozen {
    } else {
        ponyint_assert_fail(
            b"!token->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            317 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_set_string\0"))
                .as_ptr(),
        );
    };
    if !value.is_null() {
    } else {
        ponyint_assert_fail(
            b"value != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            318 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"token_set_string\0"))
                .as_ptr(),
        );
    };
    if length == 0 as libc::c_int as libc::c_ulong {
        length = strlen(value);
    }
    let ref mut fresh10 = (*token).c2rust_unnamed.c2rust_unnamed.string;
    *fresh10 = stringtab_len(value, length);
    (*token).c2rust_unnamed.c2rust_unnamed.str_length = length;
}
#[no_mangle]
#[c2rust::src_loc = "328:1"]
pub unsafe extern "C" fn token_set_float(mut token: *mut token_t, mut value: libc::c_double) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            330 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"token_set_float\0"))
                .as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_FLOAT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_FLOAT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            331 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"token_set_float\0"))
                .as_ptr(),
        );
    };
    if !(*token).frozen {
    } else {
        ponyint_assert_fail(
            b"!token->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            332 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"token_set_float\0"))
                .as_ptr(),
        );
    };
    (*token).c2rust_unnamed.real = value;
}
#[no_mangle]
#[c2rust::src_loc = "337:1"]
pub unsafe extern "C" fn token_set_int(mut token: *mut token_t, mut value: *mut lexint_t) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            339 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"token_set_int\0"))
                .as_ptr(),
        );
    };
    if (*token).id as libc::c_uint == TK_INT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_INT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            340 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"token_set_int\0"))
                .as_ptr(),
        );
    };
    if !(*token).frozen {
    } else {
        ponyint_assert_fail(
            b"!token->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            341 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"token_set_int\0"))
                .as_ptr(),
        );
    };
    (*token).c2rust_unnamed.integer = *value;
}
#[no_mangle]
#[c2rust::src_loc = "346:1"]
pub unsafe extern "C" fn token_set_pos(
    mut token: *mut token_t,
    mut source: *mut source_t,
    mut line: size_t,
    mut pos: size_t,
) {
    if !token.is_null() {
    } else {
        ponyint_assert_fail(
            b"token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            348 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"token_set_pos\0"))
                .as_ptr(),
        );
    };
    if !(*token).frozen {
    } else {
        ponyint_assert_fail(
            b"!token->frozen\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            349 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"token_set_pos\0"))
                .as_ptr(),
        );
    };
    if !source.is_null() {
        let ref mut fresh11 = (*token).source;
        *fresh11 = source;
    }
    (*token).line = line;
    (*token).pos = pos;
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn token_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut token: *mut token_t = object as *mut token_t;
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
        string_trace_len(
            ctx,
            (*token).c2rust_unnamed.c2rust_unnamed.string,
            (*token).c2rust_unnamed.c2rust_unnamed.str_length,
        );
    }
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn token_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut mutability: libc::c_int,
) {
    let mut token: *mut token_t = object as *mut token_t;
    let mut dst: *mut token_signature_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut token_signature_t;
    memset(
        dst as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<token_signature_t>() as libc::c_ulong,
    );
    (*dst).id = (*token).id;
    match (*token).id as libc::c_uint {
        5 | 8 => {
            (*dst).c2rust_unnamed.c2rust_unnamed.str_length =
                (*token).c2rust_unnamed.c2rust_unnamed.str_length;
            let ref mut fresh12 = (*dst).c2rust_unnamed.c2rust_unnamed.string;
            *fresh12 = pony_serialise_offset(
                ctx,
                (*token).c2rust_unnamed.c2rust_unnamed.string as *mut libc::c_char
                    as *mut libc::c_void,
            ) as *const libc::c_char;
        }
        7 => {
            (*dst).c2rust_unnamed.real = (*token).c2rust_unnamed.real;
        }
        6 => {
            (*dst).c2rust_unnamed.integer = (*token).c2rust_unnamed.integer;
        }
        _ => {}
    };
}
#[c2rust::src_loc = "401:20"]
static mut token_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as uint32_t,
            size: ::core::mem::size_of::<token_signature_t>() as libc::c_ulong as uint32_t,
            field_count: 0 as libc::c_int as uint32_t,
            field_offset: 0 as libc::c_int as uint32_t,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                token_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                token_signature_serialise
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
            event_notify: 0 as libc::c_int as uint32_t,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "422:1"]
pub unsafe extern "C" fn token_signature_pony_type() -> *const pony_type_t {
    return &token_signature_pony;
}
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn token_docstring_signature_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut token: *mut token_t = object as *mut token_t;
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"token->id == TK_STRING\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.c\0" as *const u8
                as *const libc::c_char,
            437 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"token_docstring_signature_serialise_trace\0",
            ))
            .as_ptr(),
        );
    };
}
#[c2rust::src_loc = "440:1"]
unsafe extern "C" fn token_docstring_signature_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut mutability: libc::c_int,
) {
    let mut dst: *mut token_signature_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut token_signature_t;
    memset(
        dst as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<token_signature_t>() as libc::c_ulong,
    );
    (*dst).id = TK_NONE;
}
#[c2rust::src_loc = "454:20"]
static mut token_docstring_signature_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as uint32_t,
            size: ::core::mem::size_of::<token_signature_t>() as libc::c_ulong as uint32_t,
            field_count: 0 as libc::c_int as uint32_t,
            field_offset: 0 as libc::c_int as uint32_t,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                token_docstring_signature_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                token_docstring_signature_serialise
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
            event_notify: 0 as libc::c_int as uint32_t,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "475:1"]
pub unsafe extern "C" fn token_docstring_signature_pony_type() -> *const pony_type_t {
    return &token_docstring_signature_pony;
}
#[c2rust::src_loc = "480:1"]
unsafe extern "C" fn token_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut token: *mut token_t = object as *mut token_t;
    if !((*token).source).is_null() {
        pony_traceknown(
            ctx,
            (*token).source as *mut libc::c_void,
            source_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
        string_trace_len(
            ctx,
            (*token).c2rust_unnamed.c2rust_unnamed.string,
            (*token).c2rust_unnamed.c2rust_unnamed.str_length,
        );
    }
}
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn token_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: size_t,
    mut mutability: libc::c_int,
) {
    let mut token: *mut token_t = object as *mut token_t;
    let mut dst: *mut token_t = (buf as uintptr_t).wrapping_add(offset) as *mut token_t;
    (*dst).id = (*token).id;
    let ref mut fresh13 = (*dst).source;
    *fresh13 = pony_serialise_offset(ctx, (*token).source as *mut libc::c_void) as *mut source_t;
    (*dst).line = (*token).line;
    (*dst).pos = (*token).pos;
    let ref mut fresh14 = (*dst).printed;
    *fresh14 = 0 as *mut libc::c_char;
    (*dst).frozen = (*token).frozen;
    match (*token).id as libc::c_uint {
        5 | 8 => {
            (*dst).c2rust_unnamed.c2rust_unnamed.str_length =
                (*token).c2rust_unnamed.c2rust_unnamed.str_length;
            let ref mut fresh15 = (*dst).c2rust_unnamed.c2rust_unnamed.string;
            *fresh15 = pony_serialise_offset(
                ctx,
                (*token).c2rust_unnamed.c2rust_unnamed.string as *mut libc::c_char
                    as *mut libc::c_void,
            ) as *const libc::c_char;
        }
        7 => {
            (*dst).c2rust_unnamed.real = (*token).c2rust_unnamed.real;
        }
        6 => {
            (*dst).c2rust_unnamed.integer = (*token).c2rust_unnamed.integer;
        }
        _ => {}
    };
}
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn token_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut token: *mut token_t = object as *mut token_t;
    let ref mut fresh16 = (*token).source;
    *fresh16 = pony_deserialise_offset(ctx, source_pony_type(), (*token).source as uintptr_t)
        as *mut source_t;
    if (*token).id as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
        || (*token).id as libc::c_uint == TK_ID as libc::c_int as libc::c_uint
    {
        let ref mut fresh17 = (*token).c2rust_unnamed.c2rust_unnamed.string;
        *fresh17 = string_deserialise_offset(
            ctx,
            (*token).c2rust_unnamed.c2rust_unnamed.string as uintptr_t,
        );
    }
}
#[c2rust::src_loc = "541:20"]
static mut token_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as uint32_t,
            size: ::core::mem::size_of::<token_t>() as libc::c_ulong as uint32_t,
            field_count: 0 as libc::c_int as uint32_t,
            field_offset: 0 as libc::c_int as uint32_t,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                token_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                token_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                token_deserialise as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            custom_serialise_space: None,
            custom_deserialise: None,
            dispatch: None,
            final_0: None,
            event_notify: 0 as libc::c_int as uint32_t,
            traits: 0 as *const *mut uintptr_t as *mut *mut uintptr_t,
            fields: 0 as *const libc::c_void as *mut libc::c_void,
            vtable: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
#[c2rust::src_loc = "562:1"]
pub unsafe extern "C" fn token_pony_type() -> *const pony_type_t {
    return &token_pony;
}
