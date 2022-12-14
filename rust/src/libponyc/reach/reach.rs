use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
    use super::_size_t_h::size_t;
    use super::pony_h::_pony_type_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct errormsg_t {
        pub file: *const libc::c_char,
        pub line: usize,
        pub pos: usize,
        pub msg: *const libc::c_char,
        pub source: *const libc::c_char,
        pub frame: *mut errormsg_t,
        pub next: *mut errormsg_t,
    }
    #[c2rust::src_loc = "49:1"]
    pub type errorframe_t = *mut errormsg_t;
    use super::_size_t_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_hash_str(str: *const libc::c_char) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = usize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: usize,
        pub size: usize,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    use super::pony_h::pony_ctx_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: usize);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: *mut usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: usize,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
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
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut usize,
            count: usize,
            item_bitmap: *mut bitmap_t,
            size: usize,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "124:1"]
        pub fn ponyint_hashmap_serialise_trace(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
        #[c2rust::src_loc = "127:1"]
        pub fn ponyint_hashmap_serialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            buf: *mut libc::c_void,
            offset: usize,
        );
        #[c2rust::src_loc = "130:1"]
        pub fn ponyint_hashmap_deserialise(
            ctx: *mut pony_ctx_t,
            object: *mut libc::c_void,
            elem_type: *const pony_type_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
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
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::source_h::{pony_type_t, source_t};
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn ast_dup(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "68:1"]
        pub fn ast_set_scope(ast: *mut ast_t, scope: *mut ast_t);
        #[c2rust::src_loc = "71:1"]
        pub fn ast_setpos(ast: *mut ast_t, source: *mut source_t, line: usize, pos: usize);
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: u32) -> libc::c_int;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: usize) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: usize,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "223:1"]
        pub fn ast_pony_type() -> *const pony_type_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:1"]
pub mod reify_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct deferred_reification_t {
        pub ast: *mut ast_t,
        pub type_typeparams: *mut ast_t,
        pub type_typeargs: *mut ast_t,
        pub method_typeparams: *mut ast_t,
        pub method_typeargs: *mut ast_t,
        pub thistype: *mut ast_t,
    }
    use super::pass_h::pass_opt_t;
    use super::source_h::pony_type_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "23:1"]
        pub fn reify(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
            duplicate: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "32:1"]
        pub fn deferred_reify_add_method_typeparams(
            deferred: *mut deferred_reification_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
        );
        #[c2rust::src_loc = "35:1"]
        pub fn deferred_reify(
            deferred: *mut deferred_reification_t,
            ast: *mut ast_t,
            opt: *mut pass_opt_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "41:1"]
        pub fn deferred_reify_dup(
            deferred: *mut deferred_reification_t,
        ) -> *mut deferred_reification_t;
        #[c2rust::src_loc = "43:1"]
        pub fn deferred_reify_free(deferred: *mut deferred_reification_t);
        #[c2rust::src_loc = "48:1"]
        pub fn deferred_reification_pony_type() -> *const pony_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/stack.h:1"]
pub mod stack_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct Stack {
        pub index: libc::c_int,
        pub data: [*mut libc::c_void; 62],
        pub prev: *mut Stack,
    }
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn ponyint_stack_pop(stack: *mut Stack, data: *mut *mut libc::c_void) -> *mut Stack;
        #[c2rust::src_loc = "19:1"]
        pub fn ponyint_stack_push(list: *mut Stack, data: *mut libc::c_void) -> *mut Stack;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.h:1"]
pub mod reach_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct reach_method_t {
        pub name: *const libc::c_char,
        pub mangled_name: *const libc::c_char,
        pub full_name: *const libc::c_char,
        pub cap: token_id,
        pub fun: *mut deferred_reification_t,
        pub typeargs: *mut ast_t,
        pub vtable_index: u32,
        pub intrinsic: bool,
        pub internal: bool,
        pub forwarding: bool,
        pub subordinate: *mut reach_method_t,
        pub param_count: usize,
        pub params: *mut reach_param_t,
        pub result: *mut reach_type_t,
        pub c_method: *mut compile_opaque_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct compile_opaque_t {
        pub free_fn: compile_opaque_free_fn,
    }
    #[c2rust::src_loc = "26:1"]
    pub type compile_opaque_free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:8"]
    pub struct reach_type_t {
        pub name: *const libc::c_char,
        pub mangle: *const libc::c_char,
        pub ast: *mut ast_t,
        pub ast_cap: *mut ast_t,
        pub underlying: token_id,
        pub methods: reach_method_names_t,
        pub bare_method: *mut reach_method_t,
        pub subtypes: reach_type_cache_t,
        pub type_id: u32,
        pub vtable_size: u32,
        pub can_be_boxed: bool,
        pub is_trait: bool,
        pub field_count: u32,
        pub fields: *mut reach_field_t,
        pub c_type: *mut compile_opaque_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:8"]
    pub struct reach_field_t {
        pub ast: *mut ast_t,
        pub type_0: *mut reach_type_t,
        pub embed: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:45"]
    pub struct reach_type_cache_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:47"]
    pub struct reach_method_names_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:8"]
    pub struct reach_param_t {
        pub name: *const libc::c_char,
        pub ast: *mut ast_t,
        pub type_0: *mut reach_type_t,
        pub cap: token_id,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:8"]
    pub struct reach_method_name_t {
        pub id: token_id,
        pub cap: token_id,
        pub name: *const libc::c_char,
        pub r_methods: reach_methods_t,
        pub r_mangled: reach_mangled_t,
        pub internal: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:42"]
    pub struct reach_mangled_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:42"]
    pub struct reach_methods_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:40"]
    pub struct reach_types_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:16"]
    pub struct reach_t {
        pub types: reach_types_t,
        pub method_stack: *mut reach_method_stack_t,
        pub object_type_count: u32,
        pub numeric_type_count: u32,
        pub tuple_type_count: u32,
        pub total_type_count: u32,
        pub trait_type_count: u32,
    }
    use super::_size_t_h::size_t;
    use super::hash_h::hashmap_t;
    use super::reach_method_stack_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/printbuf.h:4"]
pub mod printbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct printbuf_t {
        pub m: *mut libc::c_char,
        pub size: usize,
        pub offset: usize,
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
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:4"]
pub mod genname_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn genname_type(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "33:1"]
        pub fn genname_fun(
            cap: token_id,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:5"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn is_result_needed(ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:6"]
pub mod assemble_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn set_cap_and_ephemeral(
            type_0: *mut ast_t,
            cap: token_id,
            ephemeral: token_id,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:7"]
pub mod cap_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.h:8"]
pub mod lookup_h {
    use super::pass_h::pass_opt_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn lookup(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            type_0: *mut ast_t,
            name: *const libc::c_char,
        ) -> *mut deferred_reification_t;
        #[c2rust::src_loc = "14:1"]
        pub fn lookup_try(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            type_0: *mut ast_t,
            name: *const libc::c_char,
            allow_private: bool,
        ) -> *mut deferred_reification_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:9"]
pub mod subtype_h {
    use super::error_h::errorframe_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn is_subtype(
            sub: *mut ast_t,
            super_0: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn is_subtype_constraint(
            sub: *mut ast_t,
            super_0: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "27:1"]
        pub fn is_eqtype(
            a: *mut ast_t,
            b: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "48:1"]
        pub fn is_machine_word(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "58:1"]
        pub fn is_bare(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "64:1"]
        pub fn contains_dontcare(ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:10"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/gc/serialise.h:10"]
pub mod serialise_h {
    use super::_size_t_h::size_t;
    use super::_uintptr_t_h::uintptr_t;
    use super::pony_h::pony_ctx_t;
    use super::source_h::pony_type_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn pony_serialise_offset(ctx: *mut pony_ctx_t, p: *mut libc::c_void) -> usize;
        #[c2rust::src_loc = "37:1"]
        pub fn pony_serialise_reserve(ctx: *mut pony_ctx_t, p: *mut libc::c_void, size: usize);
        #[c2rust::src_loc = "42:1"]
        pub fn pony_deserialise_block(
            ctx: *mut pony_ctx_t,
            offset: uintptr_t,
            size: usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "44:1"]
        pub fn pony_deserialise_offset(
            ctx: *mut pony_ctx_t,
            t: *const pony_type_t,
            offset: uintptr_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:12"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:14"]
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
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uintptr_t_h::uintptr_t;
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_add, ast_checkflag, ast_child, ast_childcount, ast_childidx, ast_data, ast_dup, ast_free,
    ast_free_unattached, ast_get, ast_get_children, ast_id, ast_line, ast_name, ast_pony_type,
    ast_pop, ast_pos, ast_ptr_t, ast_set_scope, ast_setpos, ast_sibling, ast_type, C2RustUnnamed_0,
    AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
use self::cap_h::cap_fetch;
pub use self::error_h::{errorframe_t, errormsg_t, errors_t};
use self::expr_h::is_result_needed;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_str};
use self::genname_h::{genname_fun, genname_type};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_deserialise, ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio,
    ponyint_hashmap_get, ponyint_hashmap_init, ponyint_hashmap_mem_size, ponyint_hashmap_next,
    ponyint_hashmap_optimize, ponyint_hashmap_put, ponyint_hashmap_putindex,
    ponyint_hashmap_remove, ponyint_hashmap_removeindex, ponyint_hashmap_serialise,
    ponyint_hashmap_serialise_trace, ponyint_hashmap_size,
};
use self::lookup_h::{lookup, lookup_try};
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
pub use self::printbuf_h::{printbuf, printbuf_free, printbuf_new, printbuf_t};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_mangled_t, reach_method_name_t,
    reach_method_names_t, reach_method_t, reach_methods_t, reach_param_t, reach_t,
    reach_type_cache_t, reach_type_t, reach_types_t,
};
pub use self::reify_h::{
    deferred_reification_pony_type, deferred_reification_t, deferred_reify,
    deferred_reify_add_method_typeparams, deferred_reify_dup, deferred_reify_free, reify,
};
use self::serialise_h::{
    pony_deserialise_block, pony_deserialise_offset, pony_serialise_offset, pony_serialise_reserve,
};
pub use self::source_h::{pony_type_t, source_t};
pub use self::stack_h::{ponyint_stack_pop, ponyint_stack_push, Stack};
use self::stdio_h::printf;
use self::string_h::{memcpy, memset, strcmp};
use self::stringtab_h::{string_deserialise_offset, string_trace, stringtab};
use self::subtype_h::{
    contains_dontcare, is_bare, is_eqtype, is_machine_word, is_subtype, is_subtype_constraint,
};
pub use self::symtab_h::{
    ast_t, sym_status_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL,
    SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};
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
#[c2rust::src_loc = "16:34"]
pub struct reach_method_stack_t {}
#[c2rust::src_loc = "40:1"]
pub type reach_methods_free_fn = Option<unsafe extern "C" fn(*mut reach_method_t) -> ()>;
#[c2rust::src_loc = "40:1"]
pub type reach_methods_cmp_fn =
    Option<unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool>;
#[c2rust::src_loc = "70:1"]
pub type reach_mangled_free_fn = Option<unsafe extern "C" fn(*mut reach_method_t) -> ()>;
#[c2rust::src_loc = "70:1"]
pub type reach_mangled_cmp_fn =
    Option<unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool>;
#[c2rust::src_loc = "92:1"]
pub type reach_method_names_free_fn = Option<unsafe extern "C" fn(*mut reach_method_name_t) -> ()>;
#[c2rust::src_loc = "92:1"]
pub type reach_method_names_cmp_fn =
    Option<unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool>;
#[c2rust::src_loc = "129:1"]
pub type reach_types_free_fn = Option<unsafe extern "C" fn(*mut reach_type_t) -> ()>;
#[c2rust::src_loc = "132:1"]
pub type reach_type_cache_free_fn = Option<unsafe extern "C" fn(*mut reach_type_t) -> ()>;
#[c2rust::src_loc = "129:1"]
pub type reach_types_cmp_fn =
    Option<unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool>;
#[c2rust::src_loc = "132:1"]
pub type reach_type_cache_cmp_fn =
    Option<unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool>;
#[no_mangle]
#[c2rust::src_loc = "16:34"]
pub unsafe extern "C" fn reach_method_stack_push(
    mut stack: *mut reach_method_stack_t,
    mut data: *mut reach_method_t,
) -> *mut reach_method_stack_t {
    ponyint_stack_push(stack as *mut Stack, data as *mut libc::c_void) as *mut reach_method_stack_t
}
#[no_mangle]
#[c2rust::src_loc = "16:34"]
pub unsafe extern "C" fn reach_method_stack_pop(
    mut stack: *mut reach_method_stack_t,
    mut data: *mut *mut reach_method_t,
) -> *mut reach_method_stack_t {
    ponyint_stack_pop(stack as *mut Stack, data as *mut *mut libc::c_void)
        as *mut reach_method_stack_t
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn reach_method_hash(mut m: *mut reach_method_t) -> usize {
    ponyint_hash_str((*m).name)
}
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn reach_method_cmp(
    mut a: *mut reach_method_t,
    mut b: *mut reach_method_t,
) -> bool {
    (*a).name == (*b).name
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_optimize(mut map: *mut reach_methods_t) {
    let mut cmpf: reach_methods_cmp_fn = Some(
        reach_method_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_methods_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_destroy(mut map: *mut reach_methods_t) {
    let mut freef: reach_methods_free_fn = None;
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_methods_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_init(mut map: *mut reach_methods_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_putindex(
    mut map: *mut reach_methods_t,
    mut entry: *mut reach_method_t,
    mut index: usize,
) {
    let mut cmpf: reach_methods_cmp_fn = Some(
        reach_method_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_hash(entry),
        ::core::mem::transmute::<reach_methods_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[c2rust::src_loc = "40:1"]
static mut reach_methods_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_methods_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_methods_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_methods_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_methods_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_mem_size(mut map: *mut reach_methods_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, reach_method_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_alloc_size(mut map: *mut reach_methods_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_pony_type() -> *const pony_type_t {
    &reach_methods_pony
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_clearindex(
    mut map: *mut reach_methods_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_removeindex(
    mut map: *mut reach_methods_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_size(mut map: *mut reach_methods_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn reach_methods_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, reach_method_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "40:58"]
pub unsafe extern "C" fn reach_methods_remove(
    mut map: *mut reach_methods_t,
    mut entry: *mut reach_method_t,
) -> *mut reach_method_t {
    let mut cmpf: reach_methods_cmp_fn = Some(
        reach_method_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_hash(entry),
        ::core::mem::transmute::<reach_methods_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "40:58"]
pub unsafe extern "C" fn reach_methods_get(
    mut map: *mut reach_methods_t,
    mut key: *mut reach_method_t,
    mut index: *mut usize,
) -> *mut reach_method_t {
    let mut cmpf: reach_methods_cmp_fn = Some(
        reach_method_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        reach_method_hash(key),
        ::core::mem::transmute::<reach_methods_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "40:58"]
pub unsafe extern "C" fn reach_methods_next(
    mut map: *mut reach_methods_t,
    mut i: *mut usize,
) -> *mut reach_method_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "40:58"]
pub unsafe extern "C" fn reach_methods_put(
    mut map: *mut reach_methods_t,
    mut entry: *mut reach_method_t,
) -> *mut reach_method_t {
    let mut cmpf: reach_methods_cmp_fn = Some(
        reach_method_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_hash(entry),
        ::core::mem::transmute::<reach_methods_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_t
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn reach_mangled_hash(mut m: *mut reach_method_t) -> usize {
    ponyint_hash_str((*m).mangled_name)
}
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn reach_mangled_cmp(
    mut a: *mut reach_method_t,
    mut b: *mut reach_method_t,
) -> bool {
    (*a).mangled_name == (*b).mangled_name
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn reach_mangled_free(mut m: *mut reach_method_t) {
    if (*m).param_count > 0 as libc::c_int as libc::c_ulong {
        ponyint_pool_free_size(
            ((*m).param_count)
                .wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
            (*m).params as *mut libc::c_void,
        );
    }
    if !((*m).c_method).is_null() {
        ((*(*m).c_method).free_fn).expect("non-null function pointer")(
            (*m).c_method as *mut libc::c_void,
        );
    }
    if ((*m).fun).is_null() {
        ast_free_unattached((*m).typeargs);
    }
    deferred_reify_free((*m).fun);
    ponyint_pool_free(2 as libc::c_int as usize, m as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_init(mut map: *mut reach_mangled_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, reach_method_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_alloc_size(mut map: *mut reach_mangled_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_removeindex(
    mut map: *mut reach_mangled_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[c2rust::src_loc = "70:1"]
static mut reach_mangled_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_mangled_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_mangled_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_mangled_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_mangled_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_clearindex(
    mut map: *mut reach_mangled_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_pony_type() -> *const pony_type_t {
    &reach_mangled_pony
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_mem_size(mut map: *mut reach_mangled_t) -> usize {
    return ponyint_hashmap_mem_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_destroy(mut map: *mut reach_mangled_t) {
    let mut freef: reach_mangled_free_fn =
        Some(reach_mangled_free as unsafe extern "C" fn(*mut reach_method_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_mangled_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_putindex(
    mut map: *mut reach_mangled_t,
    mut entry: *mut reach_method_t,
    mut index: usize,
) {
    let mut cmpf: reach_mangled_cmp_fn = Some(
        reach_mangled_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_mangled_hash(entry),
        ::core::mem::transmute::<reach_mangled_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, reach_method_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_optimize(mut map: *mut reach_mangled_t) {
    let mut cmpf: reach_mangled_cmp_fn = Some(
        reach_mangled_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_mangled_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn reach_mangled_size(mut map: *mut reach_mangled_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "70:58"]
pub unsafe extern "C" fn reach_mangled_get(
    mut map: *mut reach_mangled_t,
    mut key: *mut reach_method_t,
    mut index: *mut usize,
) -> *mut reach_method_t {
    let mut cmpf: reach_mangled_cmp_fn = Some(
        reach_mangled_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        reach_mangled_hash(key),
        ::core::mem::transmute::<reach_mangled_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "70:58"]
pub unsafe extern "C" fn reach_mangled_remove(
    mut map: *mut reach_mangled_t,
    mut entry: *mut reach_method_t,
) -> *mut reach_method_t {
    let mut cmpf: reach_mangled_cmp_fn = Some(
        reach_mangled_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_mangled_hash(entry),
        ::core::mem::transmute::<reach_mangled_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "70:58"]
pub unsafe extern "C" fn reach_mangled_put(
    mut map: *mut reach_mangled_t,
    mut entry: *mut reach_method_t,
) -> *mut reach_method_t {
    let mut cmpf: reach_mangled_cmp_fn = Some(
        reach_mangled_cmp as unsafe extern "C" fn(*mut reach_method_t, *mut reach_method_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_mangled_hash(entry),
        ::core::mem::transmute::<reach_mangled_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_t
}
#[no_mangle]
#[c2rust::src_loc = "70:58"]
pub unsafe extern "C" fn reach_mangled_next(
    mut map: *mut reach_mangled_t,
    mut i: *mut usize,
) -> *mut reach_method_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut reach_method_t
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn reach_method_name_hash(mut n: *mut reach_method_name_t) -> usize {
    ponyint_hash_str((*n).name)
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn reach_method_name_cmp(
    mut a: *mut reach_method_name_t,
    mut b: *mut reach_method_name_t,
) -> bool {
    (*a).name == (*b).name
}
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn reach_method_name_free(mut n: *mut reach_method_name_t) {
    reach_methods_destroy(&mut (*n).r_methods);
    reach_mangled_destroy(&mut (*n).r_mangled);
    ponyint_pool_free(2 as libc::c_int as usize, n as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_mem_size(mut map: *mut reach_method_names_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_destroy(mut map: *mut reach_method_names_t) {
    let mut freef: reach_method_names_free_fn =
        Some(reach_method_name_free as unsafe extern "C" fn(*mut reach_method_name_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_method_names_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_optimize(mut map: *mut reach_method_names_t) {
    let mut cmpf: reach_method_names_cmp_fn = Some(
        reach_method_name_cmp
            as unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_method_names_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_putindex(
    mut map: *mut reach_method_names_t,
    mut entry: *mut reach_method_name_t,
    mut index: usize,
) {
    let mut cmpf: reach_method_names_cmp_fn = Some(
        reach_method_name_cmp
            as unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_name_hash(entry),
        ::core::mem::transmute::<reach_method_names_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_removeindex(
    mut map: *mut reach_method_names_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_clearindex(
    mut map: *mut reach_method_names_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_size(mut map: *mut reach_method_names_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, reach_method_name_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, reach_method_name_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_alloc_size(
    mut map: *mut reach_method_names_t,
) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_pony_type() -> *const pony_type_t {
    &reach_method_names_pony
}
#[c2rust::src_loc = "92:1"]
static mut reach_method_names_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_method_names_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_method_names_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_method_names_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_method_names_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn reach_method_names_init(
    mut map: *mut reach_method_names_t,
    mut size: usize,
) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "93:3"]
pub unsafe extern "C" fn reach_method_names_next(
    mut map: *mut reach_method_names_t,
    mut i: *mut usize,
) -> *mut reach_method_name_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut reach_method_name_t
}
#[no_mangle]
#[c2rust::src_loc = "93:3"]
pub unsafe extern "C" fn reach_method_names_get(
    mut map: *mut reach_method_names_t,
    mut key: *mut reach_method_name_t,
    mut index: *mut usize,
) -> *mut reach_method_name_t {
    let mut cmpf: reach_method_names_cmp_fn = Some(
        reach_method_name_cmp
            as unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        reach_method_name_hash(key),
        ::core::mem::transmute::<reach_method_names_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut reach_method_name_t
}
#[no_mangle]
#[c2rust::src_loc = "93:3"]
pub unsafe extern "C" fn reach_method_names_put(
    mut map: *mut reach_method_names_t,
    mut entry: *mut reach_method_name_t,
) -> *mut reach_method_name_t {
    let mut cmpf: reach_method_names_cmp_fn = Some(
        reach_method_name_cmp
            as unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_name_hash(entry),
        ::core::mem::transmute::<reach_method_names_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_name_t
}
#[no_mangle]
#[c2rust::src_loc = "93:3"]
pub unsafe extern "C" fn reach_method_names_remove(
    mut map: *mut reach_method_names_t,
    mut entry: *mut reach_method_name_t,
) -> *mut reach_method_name_t {
    let mut cmpf: reach_method_names_cmp_fn = Some(
        reach_method_name_cmp
            as unsafe extern "C" fn(*mut reach_method_name_t, *mut reach_method_name_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_method_name_hash(entry),
        ::core::mem::transmute::<reach_method_names_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_method_name_t
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn reach_type_hash(mut t: *mut reach_type_t) -> usize {
    ponyint_hash_str((*t).name)
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn reach_type_cmp(mut a: *mut reach_type_t, mut b: *mut reach_type_t) -> bool {
    (*a).name == (*b).name
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn reach_type_free(mut t: *mut reach_type_t) {
    ast_free((*t).ast);
    ast_free((*t).ast_cap);
    reach_method_names_destroy(&mut (*t).methods);
    reach_type_cache_destroy(&mut (*t).subtypes);
    if (*t).field_count > 0 as libc::c_int as libc::c_uint {
        let mut i: u32 = 0 as libc::c_int as u32;
        while i < (*t).field_count {
            ast_free((*((*t).fields).offset(i as isize)).ast);
            i = i.wrapping_add(1);
        }
        ponyint_pool_free_size(
            ((*t).field_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<reach_field_t>() as libc::c_ulong),
            (*t).fields as *mut libc::c_void,
        );
        (*t).field_count = 0 as libc::c_int as u32;
        let ref mut fresh0 = (*t).fields;
        *fresh0 = 0 as *mut reach_field_t;
    }
    if !((*t).c_type).is_null() {
        ((*(*t).c_type).free_fn).expect("non-null function pointer")(
            (*t).c_type as *mut libc::c_void,
        );
    }
    ponyint_pool_free(3 as libc::c_int as usize, t as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_mem_size(mut map: *mut reach_types_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_destroy(mut map: *mut reach_types_t) {
    let mut freef: reach_types_free_fn =
        Some(reach_type_free as unsafe extern "C" fn(*mut reach_type_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_types_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_init(mut map: *mut reach_types_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, reach_type_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_pony_type() -> *const pony_type_t {
    &reach_types_pony
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_putindex(
    mut map: *mut reach_types_t,
    mut entry: *mut reach_type_t,
    mut index: usize,
) {
    let mut cmpf: reach_types_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_types_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_clearindex(mut map: *mut reach_types_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_alloc_size(mut map: *mut reach_types_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_optimize(mut map: *mut reach_types_t) {
    let mut cmpf: reach_types_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_types_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_removeindex(mut map: *mut reach_types_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_size(mut map: *mut reach_types_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn reach_types_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, reach_type_pony_type());
}
#[c2rust::src_loc = "129:1"]
static mut reach_types_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_types_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_types_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_types_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_types_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "129:54"]
pub unsafe extern "C" fn reach_types_remove(
    mut map: *mut reach_types_t,
    mut entry: *mut reach_type_t,
) -> *mut reach_type_t {
    let mut cmpf: reach_types_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_types_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "129:54"]
pub unsafe extern "C" fn reach_types_get(
    mut map: *mut reach_types_t,
    mut key: *mut reach_type_t,
    mut index: *mut usize,
) -> *mut reach_type_t {
    let mut cmpf: reach_types_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        reach_type_hash(key),
        ::core::mem::transmute::<reach_types_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "129:54"]
pub unsafe extern "C" fn reach_types_next(
    mut map: *mut reach_types_t,
    mut i: *mut usize,
) -> *mut reach_type_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "129:54"]
pub unsafe extern "C" fn reach_types_put(
    mut map: *mut reach_types_t,
    mut entry: *mut reach_type_t,
) -> *mut reach_type_t {
    let mut cmpf: reach_types_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_types_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_init(mut map: *mut reach_type_cache_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    ponyint_hashmap_serialise(ctx, object, buf, offset);
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_serialise_trace(ctx, object, reach_type_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    ponyint_hashmap_deserialise(ctx, object, reach_type_pony_type());
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_destroy(mut map: *mut reach_type_cache_t) {
    let mut freef: reach_type_cache_free_fn = None;
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_type_cache_free_fn, free_fn>(freef),
    );
}
#[c2rust::src_loc = "132:1"]
static mut reach_type_cache_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_type_cache_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_type_cache_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_type_cache_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_type_cache_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_pony_type() -> *const pony_type_t {
    &reach_type_cache_pony
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_alloc_size(mut map: *mut reach_type_cache_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_mem_size(mut map: *mut reach_type_cache_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_size(mut map: *mut reach_type_cache_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_clearindex(
    mut map: *mut reach_type_cache_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_removeindex(
    mut map: *mut reach_type_cache_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_optimize(mut map: *mut reach_type_cache_t) {
    let mut cmpf: reach_type_cache_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<reach_type_cache_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn reach_type_cache_putindex(
    mut map: *mut reach_type_cache_t,
    mut entry: *mut reach_type_t,
    mut index: usize,
) {
    let mut cmpf: reach_type_cache_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_type_cache_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "132:64"]
pub unsafe extern "C" fn reach_type_cache_get(
    mut map: *mut reach_type_cache_t,
    mut key: *mut reach_type_t,
    mut index: *mut usize,
) -> *mut reach_type_t {
    let mut cmpf: reach_type_cache_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        reach_type_hash(key),
        ::core::mem::transmute::<reach_type_cache_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "132:64"]
pub unsafe extern "C" fn reach_type_cache_remove(
    mut map: *mut reach_type_cache_t,
    mut entry: *mut reach_type_t,
) -> *mut reach_type_t {
    let mut cmpf: reach_type_cache_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_type_cache_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "132:64"]
pub unsafe extern "C" fn reach_type_cache_next(
    mut map: *mut reach_type_cache_t,
    mut i: *mut usize,
) -> *mut reach_type_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut reach_type_t
}
#[no_mangle]
#[c2rust::src_loc = "132:64"]
pub unsafe extern "C" fn reach_type_cache_put(
    mut map: *mut reach_type_cache_t,
    mut entry: *mut reach_type_t,
) -> *mut reach_type_t {
    let mut cmpf: reach_type_cache_cmp_fn =
        Some(reach_type_cmp as unsafe extern "C" fn(*mut reach_type_t, *mut reach_type_t) -> bool);
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        reach_type_hash(entry),
        ::core::mem::transmute::<reach_type_cache_cmp_fn, cmp_fn>(cmpf),
    ) as *mut reach_type_t
}
#[c2rust::src_loc = "135:1"]
unsafe extern "C" fn reach_rmethod(
    mut n: *mut reach_method_name_t,
    mut name: *const libc::c_char,
) -> *mut reach_method_t {
    let mut k: reach_method_t = reach_method_t {
        name: 0 as *const libc::c_char,
        mangled_name: 0 as *const libc::c_char,
        full_name: 0 as *const libc::c_char,
        cap: TK_EOF,
        fun: 0 as *mut deferred_reification_t,
        typeargs: 0 as *mut ast_t,
        vtable_index: 0,
        intrinsic: false,
        internal: false,
        forwarding: false,
        subordinate: 0 as *mut reach_method_t,
        param_count: 0,
        params: 0 as *mut reach_param_t,
        result: 0 as *mut reach_type_t,
        c_method: 0 as *mut compile_opaque_t,
    };
    k.name = name;
    let mut index: usize = -(1 as libc::c_int) as usize;
    return reach_methods_get(&mut (*n).r_methods, &mut k, &mut index);
}
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn add_method_name(
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
    mut internal: bool,
) -> *mut reach_method_name_t {
    let mut n: *mut reach_method_name_t = reach_method_name(t, name);
    if n.is_null() {
        n = ponyint_pool_alloc(2 as libc::c_int as usize) as *mut reach_method_name_t;
        let ref mut fresh1 = (*n).name;
        *fresh1 = name;
        reach_methods_init(&mut (*n).r_methods, 0 as libc::c_int as usize);
        reach_mangled_init(&mut (*n).r_mangled, 0 as libc::c_int as usize);
        reach_method_names_put(&mut (*t).methods, n);
        if internal {
            (*n).id = TK_FUN;
            (*n).cap = TK_BOX;
            (*n).internal = 1 as libc::c_int != 0;
        } else {
            let mut fun: *mut deferred_reification_t =
                lookup(0 as *mut pass_opt_t, 0 as *mut ast_t, (*t).ast, name);
            let mut fun_ast: *mut ast_t = (*fun).ast;
            (*n).id = ast_id(fun_ast);
            (*n).cap = ast_id(ast_child(fun_ast));
            (*n).internal = 0 as libc::c_int != 0;
            deferred_reify_free(fun);
        }
    }
    n
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn set_method_types(
    mut r: *mut reach_t,
    mut m: *mut reach_method_t,
    mut opt: *mut pass_opt_t,
) {
    let mut params: *mut ast_t = ast_childidx((*(*m).fun).ast, 3 as libc::c_int as usize);
    let mut result: *mut ast_t = ast_sibling(params);
    (*m).param_count = ast_childcount(params);
    if (*m).param_count > 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh2 = (*m).params;
        *fresh2 = ponyint_pool_alloc_size(
            ((*m).param_count)
                .wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
        ) as *mut reach_param_t;
        let mut param: *mut ast_t = ast_child(params);
        let mut i: usize = 0;
        while !param.is_null() {
            let mut p_type: *mut ast_t = deferred_reify((*m).fun, ast_type(param), opt);
            ast_set_scope(p_type, 0 as *mut ast_t);
            let ref mut fresh3 = (*((*m).params).offset(i as isize)).name;
            *fresh3 = ast_name(ast_child(param));
            let ref mut fresh4 = (*((*m).params).offset(i as isize)).ast;
            *fresh4 = p_type;
            let ref mut fresh5 = (*((*m).params).offset(i as isize)).type_0;
            *fresh5 = add_type(r, p_type, opt);
            if ast_id(p_type) as libc::c_uint != TK_NOMINAL as libc::c_int as libc::c_uint
                && ast_id(p_type) as libc::c_uint != TK_TYPEPARAMREF as libc::c_int as libc::c_uint
            {
                (*((*m).params).offset(i as isize)).cap = TK_REF;
            } else {
                (*((*m).params).offset(i as isize)).cap = ast_id(cap_fetch(p_type));
            }
            i = i.wrapping_add(1);
            param = ast_sibling(param);
        }
    }
    let mut r_result: *mut ast_t = deferred_reify((*m).fun, result, opt);
    let ref mut fresh6 = (*m).result;
    *fresh6 = add_type(r, r_result, opt);
    ast_free_unattached(r_result);
}
#[c2rust::src_loc = "214:1"]
unsafe extern "C" fn make_mangled_name(mut m: *mut reach_method_t) -> *const libc::c_char {
    let mut buf: *mut printbuf_t = printbuf_new();
    printbuf(buf, b"%s_\0" as *const u8 as *const libc::c_char, (*m).name);
    let mut i: usize = 0;
    while i < (*m).param_count {
        printbuf(
            buf,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*(*((*m).params).offset(i as isize)).type_0).mangle,
        );
        i = i.wrapping_add(1);
    }
    if !(*m).internal {
        printbuf(
            buf,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*(*m).result).mangle,
        );
    }
    let mut name: *const libc::c_char = stringtab((*buf).m);
    printbuf_free(buf);
    name
}
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn make_full_name(
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> *const libc::c_char {
    let mut buf: *mut printbuf_t = printbuf_new();
    printbuf(
        buf,
        b"%s_%s\0" as *const u8 as *const libc::c_char,
        (*t).name,
        (*m).mangled_name,
    );
    let mut name: *const libc::c_char = stringtab((*buf).m);
    printbuf_free(buf);
    name
}
#[c2rust::src_loc = "242:1"]
unsafe extern "C" fn add_rmethod_to_subtype(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
    mut opt: *mut pass_opt_t,
) {
    let mut n2: *mut reach_method_name_t = add_method_name(t, (*n).name, 0 as libc::c_int != 0);
    add_rmethod(
        r,
        t,
        n2,
        (*m).cap,
        (*m).typeargs,
        opt,
        0 as libc::c_int != 0,
    );
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut mangled: *mut reach_method_t = reach_mangled_get(&mut (*n2).r_mangled, m, &mut index);
    if !mangled.is_null() {
        return;
    }
    mangled = ponyint_pool_alloc(2 as libc::c_int as usize) as *mut reach_method_t;
    memset(
        mangled as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<reach_method_t>() as libc::c_ulong,
    );
    let ref mut fresh7 = (*mangled).name;
    *fresh7 = (*m).name;
    let ref mut fresh8 = (*mangled).mangled_name;
    *fresh8 = (*m).mangled_name;
    let ref mut fresh9 = (*mangled).full_name;
    *fresh9 = make_full_name(t, mangled);
    (*mangled).cap = (*m).cap;
    let ref mut fresh10 = (*mangled).fun;
    *fresh10 = deferred_reify_dup((*m).fun);
    if !((*mangled).fun).is_null() {
        let ref mut fresh11 = (*mangled).typeargs;
        *fresh11 = (*(*mangled).fun).method_typeargs;
    } else {
        let ref mut fresh12 = (*mangled).typeargs;
        *fresh12 = ast_dup((*m).typeargs);
    }
    (*mangled).forwarding = 1 as libc::c_int != 0;
    (*mangled).param_count = (*m).param_count;
    let ref mut fresh13 = (*mangled).params;
    *fresh13 = ponyint_pool_alloc_size(
        ((*mangled).param_count)
            .wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
    ) as *mut reach_param_t;
    memcpy(
        (*mangled).params as *mut libc::c_void,
        (*m).params as *const libc::c_void,
        ((*m).param_count).wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
    );
    let ref mut fresh14 = (*mangled).result;
    *fresh14 = (*m).result;
    reach_mangled_putindex(&mut (*n2).r_mangled, mangled, index);
}
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn add_rmethod_to_subtypes(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
    mut opt: *mut pass_opt_t,
) {
    if !(*m).internal {
    } else {
        ponyint_assert_fail(
            b"!m->internal\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"add_rmethod_to_subtypes\0",
            ))
            .as_ptr(),
        );
    };
    match (*t).underlying as libc::c_uint {
        149 | 72 | 73 => {
            let mut i: usize = -(1 as libc::c_int) as usize;
            let mut t2: *mut reach_type_t = 0 as *mut reach_type_t;
            loop {
                t2 = reach_type_cache_next(&mut (*t).subtypes, &mut i);
                if t2.is_null() {
                    break;
                }
                add_rmethod_to_subtype(r, t2, n, m, opt);
            }
        }
        56 => {
            let mut child: *mut ast_t = ast_child((*t).ast_cap);
            let mut t2_0: *mut reach_type_t = 0 as *mut reach_type_t;
            while !child.is_null() {
                let mut find: *mut deferred_reification_t = lookup_try(
                    0 as *mut pass_opt_t,
                    0 as *mut ast_t,
                    child,
                    (*n).name,
                    0 as libc::c_int != 0,
                );
                if !find.is_null() {
                    deferred_reify_free(find);
                    t2_0 = add_type(r, child, opt);
                    add_rmethod_to_subtype(r, t2_0, n, m, opt);
                }
                child = ast_sibling(child);
            }
        }
        _ => {}
    };
}
#[c2rust::src_loc = "331:1"]
unsafe extern "C" fn add_rmethod(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut cap: token_id,
    mut typeargs: *mut ast_t,
    mut opt: *mut pass_opt_t,
    mut internal: bool,
) -> *mut reach_method_t {
    let mut name: *const libc::c_char = genname_fun(cap, (*n).name, typeargs);
    let mut m: *mut reach_method_t = reach_rmethod(n, name);
    if !m.is_null() {
        return m;
    }
    m = ponyint_pool_alloc(2 as libc::c_int as usize) as *mut reach_method_t;
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<reach_method_t>() as libc::c_ulong,
    );
    let ref mut fresh15 = (*m).name;
    *fresh15 = name;
    (*m).cap = cap;
    (*m).vtable_index = -(1 as libc::c_int) as u32;
    (*m).internal = internal;
    (*m).intrinsic = internal;
    if !internal {
        let mut r_ast: *mut ast_t = set_cap_and_ephemeral((*t).ast, cap, TK_NONE);
        let mut fun: *mut deferred_reification_t =
            lookup(0 as *mut pass_opt_t, 0 as *mut ast_t, r_ast, (*n).name);
        if !fun.is_null() {
        } else {
            ponyint_assert_fail(
                b"fun != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                    as *const u8 as *const libc::c_char,
                353 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"add_rmethod\0"))
                    .as_ptr(),
            );
        };
        if !((*fun).type_typeargs).is_null() {
            ast_set_scope((*fun).type_typeargs, (*t).ast);
        }
        ast_set_scope((*fun).thistype, (*t).ast);
        ast_free_unattached(r_ast);
        if !typeargs.is_null() {
            let mut typeparams: *mut ast_t = ast_childidx((*fun).ast, 2 as libc::c_int as usize);
            deferred_reify_add_method_typeparams(fun, typeparams, typeargs, opt);
            let ref mut fresh16 = (*m).typeargs;
            *fresh16 = (*fun).method_typeargs;
        }
        let ref mut fresh17 = (*m).fun;
        *fresh17 = fun;
        set_method_types(r, m, opt);
    }
    let ref mut fresh18 = (*m).mangled_name;
    *fresh18 = make_mangled_name(m);
    let ref mut fresh19 = (*m).full_name;
    *fresh19 = make_full_name(t, m);
    reach_methods_put(&mut (*n).r_methods, m);
    reach_mangled_put(&mut (*n).r_mangled, m);
    if !internal {
        let ref mut fresh20 = (*r).method_stack;
        *fresh20 = reach_method_stack_push((*r).method_stack, m);
        add_rmethod_to_subtypes(r, t, n, m, opt);
    }
    m
}
#[c2rust::src_loc = "394:1"]
unsafe extern "C" fn add_methods_to_type(
    mut r: *mut reach_t,
    mut from: *mut reach_type_t,
    mut to: *mut reach_type_t,
    mut opt: *mut pass_opt_t,
) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
    loop {
        n = reach_method_names_next(&mut (*from).methods, &mut i);
        if n.is_null() {
            break;
        }
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            if !(*m).internal {
                add_rmethod_to_subtype(r, to, n, m, opt);
            }
        }
    }
}
#[c2rust::src_loc = "413:1"]
unsafe extern "C" fn add_internal(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) {
    name = stringtab(name);
    let mut n: *mut reach_method_name_t = add_method_name(t, name, 1 as libc::c_int != 0);
    add_rmethod(r, t, n, TK_BOX, 0 as *mut ast_t, opt, 1 as libc::c_int != 0);
}
#[c2rust::src_loc = "421:1"]
unsafe extern "C" fn add_types_to_trait(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut opt: *mut pass_opt_t,
) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut t2: *mut reach_type_t = 0 as *mut reach_type_t;
    let mut interface: bool = 0 as libc::c_int != 0;
    match ast_id((*t).ast) as libc::c_uint {
        151 => {
            let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
            interface = ast_id(def) as libc::c_uint == TK_INTERFACE as libc::c_int as libc::c_uint;
        }
        149 | 56 => {
            interface = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    loop {
        t2 = reach_types_next(&mut (*r).types, &mut i);
        if t2.is_null() {
            break;
        }
        match ast_id((*t2).ast) as libc::c_uint {
            151 => {
                let mut def2: *mut ast_t = ast_data((*t2).ast) as *mut ast_t;
                match ast_id(def2) as libc::c_uint {
                    72 => {
                        if interface as libc::c_int != 0
                            && is_eqtype((*t).ast, (*t2).ast, 0 as *mut errorframe_t, opt)
                                as libc::c_int
                                != 0
                        {
                            (*t).type_id = (*t2).type_id;
                        }
                    }
                    74 | 76 | 77 => {
                        if is_subtype((*t2).ast, (*t).ast, 0 as *mut errorframe_t, opt) {
                            reach_type_cache_put(&mut (*t).subtypes, t2);
                            reach_type_cache_put(&mut (*t2).subtypes, t);
                            if ast_id((*t).ast) as libc::c_uint
                                == TK_NOMINAL as libc::c_int as libc::c_uint
                            {
                                add_methods_to_type(r, t, t2, opt);
                            }
                            if (*t2).can_be_boxed {
                                add_internal(
                                    r,
                                    t,
                                    b"__digestof\0" as *const u8 as *const libc::c_char,
                                    opt,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            149 | 56 => {
                if interface as libc::c_int != 0
                    && is_eqtype((*t).ast, (*t2).ast, 0 as *mut errorframe_t, opt) as libc::c_int
                        != 0
                {
                    (*t).type_id = (*t2).type_id;
                }
            }
            150 => {
                if is_subtype((*t2).ast, (*t).ast, 0 as *mut errorframe_t, opt) {
                    reach_type_cache_put(&mut (*t).subtypes, t2);
                    reach_type_cache_put(&mut (*t2).subtypes, t);
                    add_internal(r, t, b"__is\0" as *const u8 as *const libc::c_char, opt);
                    add_internal(
                        r,
                        t,
                        b"__digestof\0" as *const u8 as *const libc::c_char,
                        opt,
                    );
                }
            }
            _ => {}
        }
    }
}
#[c2rust::src_loc = "506:1"]
unsafe extern "C" fn add_traits_to_type(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut opt: *mut pass_opt_t,
) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut t2: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t2 = reach_types_next(&mut (*r).types, &mut i);
        if t2.is_null() {
            break;
        }
        if ast_id((*t2).ast) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
            let mut def: *mut ast_t = ast_data((*t2).ast) as *mut ast_t;
            match ast_id(def) as libc::c_uint {
                72 | 73 => {
                    if is_subtype((*t).ast, (*t2).ast, 0 as *mut errorframe_t, opt) {
                        reach_type_cache_put(&mut (*t).subtypes, t2);
                        reach_type_cache_put(&mut (*t2).subtypes, t);
                        add_methods_to_type(r, t2, t, opt);
                        if (*t).underlying as libc::c_uint
                            == TK_TUPLETYPE as libc::c_int as libc::c_uint
                        {
                            add_internal(r, t2, b"__is\0" as *const u8 as *const libc::c_char, opt);
                        }
                        if (*t).can_be_boxed {
                            add_internal(
                                r,
                                t2,
                                b"__digestof\0" as *const u8 as *const libc::c_char,
                                opt,
                            );
                        }
                    }
                }
                _ => {}
            }
        } else {
            match ast_id((*t2).ast) as libc::c_uint {
                149 | 56 => {
                    if is_subtype((*t).ast, (*t2).ast, 0 as *mut errorframe_t, opt) {
                        reach_type_cache_put(&mut (*t).subtypes, t2);
                        reach_type_cache_put(&mut (*t2).subtypes, t);
                        add_methods_to_type(r, t2, t, opt);
                        if (*t).underlying as libc::c_uint
                            == TK_TUPLETYPE as libc::c_int as libc::c_uint
                        {
                            add_internal(r, t2, b"__is\0" as *const u8 as *const libc::c_char, opt);
                        }
                        if (*t).can_be_boxed {
                            add_internal(
                                r,
                                t2,
                                b"__digestof\0" as *const u8 as *const libc::c_char,
                                opt,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
#[c2rust::src_loc = "563:1"]
unsafe extern "C" fn add_special(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut type_0: *mut ast_t,
    mut special: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) {
    special = stringtab(special);
    let mut find: *mut deferred_reification_t = lookup_try(
        0 as *mut pass_opt_t,
        0 as *mut ast_t,
        type_0,
        special,
        0 as libc::c_int != 0,
    );
    if !find.is_null() {
        match ast_id((*find).ast) as libc::c_uint {
            88 | 89 | 90 => {
                reachable_method(
                    r,
                    0 as *mut deferred_reification_t,
                    (*t).ast,
                    special,
                    0 as *mut ast_t,
                    opt,
                );
            }
            _ => {}
        }
        deferred_reify_free(find);
    }
}
#[c2rust::src_loc = "586:1"]
unsafe extern "C" fn embed_has_finaliser(
    mut ast: *mut ast_t,
    mut str_final: *const libc::c_char,
) -> bool {
    match ast_id(ast) as libc::c_uint {
        151 => {}
        _ => return 0 as libc::c_int != 0,
    }
    let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
    if !(ast_get(def, str_final, 0 as *mut sym_status_t)).is_null() {
        return 1 as libc::c_int != 0;
    }
    let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as usize);
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        if ast_id(member) as libc::c_uint == TK_EMBED as libc::c_int as libc::c_uint
            && embed_has_finaliser(ast_type(member), str_final) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        member = ast_sibling(member);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "616:1"]
unsafe extern "C" fn add_fields(
    mut r: *mut reach_t,
    mut t: *mut reach_type_t,
    mut opt: *mut pass_opt_t,
) {
    let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as usize);
    let mut typeparams: *mut ast_t = ast_childidx(def, 1 as libc::c_int as usize);
    let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as usize);
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            140 | 141 | 86 => {
                let ref mut fresh21 = (*t).field_count;
                *fresh21 = (*fresh21).wrapping_add(1);
            }
            _ => {}
        }
        member = ast_sibling(member);
    }
    if (*t).field_count == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let ref mut fresh22 = (*t).fields;
    *fresh22 = ponyint_pool_alloc_size(
        ((*t).field_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<reach_field_t>() as libc::c_ulong),
    ) as *mut reach_field_t;
    member = ast_child(members);
    let mut index: usize = 0;
    let mut str_final: *const libc::c_char =
        stringtab(b"_final\0" as *const u8 as *const libc::c_char);
    let mut has_finaliser: bool = !(ast_get(def, str_final, 0 as *mut sym_status_t)).is_null();
    let mut needs_finaliser: bool = 0 as libc::c_int != 0;
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            140 | 141 | 86 => {
                let mut member_lookup: *mut deferred_reification_t = lookup(
                    0 as *mut pass_opt_t,
                    0 as *mut ast_t,
                    (*t).ast,
                    ast_name(ast_child(member)),
                );
                if !member_lookup.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"member_lookup != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                            as *const u8 as *const libc::c_char,
                        664 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(
                            b"add_fields\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut r_member: *mut ast_t =
                    deferred_reify(member_lookup, (*member_lookup).ast, opt);
                deferred_reify_free(member_lookup);
                let mut name: *mut ast_t = ast_pop(r_member);
                let mut type_0: *mut ast_t = ast_pop(r_member);
                ast_add(r_member, name);
                ast_set_scope(type_0, member);
                let ref mut fresh23 = (*((*t).fields).offset(index as isize)).embed;
                *fresh23 =
                    ast_id(member) as libc::c_uint == TK_EMBED as libc::c_int as libc::c_uint;
                let mut embed: bool = *fresh23;
                let ref mut fresh24 = (*((*t).fields).offset(index as isize)).ast;
                *fresh24 = reify(
                    ast_type(member),
                    typeparams,
                    typeargs,
                    opt,
                    1 as libc::c_int != 0,
                );
                ast_setpos(
                    (*((*t).fields).offset(index as isize)).ast,
                    0 as *mut source_t,
                    ast_line(name),
                    ast_pos(name),
                );
                let ref mut fresh25 = (*((*t).fields).offset(index as isize)).type_0;
                *fresh25 = add_type(r, type_0, opt);
                if embed as libc::c_int != 0 && !has_finaliser && !needs_finaliser {
                    needs_finaliser = embed_has_finaliser(type_0, str_final);
                }
                ast_free_unattached(r_member);
                index = index.wrapping_add(1);
            }
            _ => {}
        }
        member = ast_sibling(member);
    }
    if !has_finaliser && needs_finaliser as libc::c_int != 0 {
        let mut n: *mut reach_method_name_t = add_method_name(
            t,
            stringtab(b"_final\0" as *const u8 as *const libc::c_char),
            1 as libc::c_int != 0,
        );
        add_rmethod(r, t, n, TK_BOX, 0 as *mut ast_t, opt, 1 as libc::c_int != 0);
    }
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn get_new_trait_id(mut r: *mut reach_t) -> u32 {
    let ref mut fresh26 = (*r).trait_type_count;
    let fresh27 = *fresh26;
    *fresh26 = (*fresh26).wrapping_add(1);
    fresh27
}
#[c2rust::src_loc = "715:1"]
unsafe extern "C" fn get_new_object_id(mut r: *mut reach_t) -> u32 {
    let ref mut fresh28 = (*r).total_type_count;
    *fresh28 = (*fresh28).wrapping_add(1);
    let ref mut fresh29 = (*r).object_type_count;
    let fresh30 = *fresh29;
    *fresh29 = (*fresh29).wrapping_add(1);
    return fresh30
        .wrapping_mul(2 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[c2rust::src_loc = "721:1"]
unsafe extern "C" fn get_new_numeric_id(mut r: *mut reach_t) -> u32 {
    let ref mut fresh31 = (*r).total_type_count;
    *fresh31 = (*fresh31).wrapping_add(1);
    let ref mut fresh32 = (*r).numeric_type_count;
    let fresh33 = *fresh32;
    *fresh32 = (*fresh32).wrapping_add(1);
    return fresh33.wrapping_mul(4 as libc::c_int as libc::c_uint);
}
#[c2rust::src_loc = "727:1"]
unsafe extern "C" fn get_new_tuple_id(mut r: *mut reach_t) -> u32 {
    let ref mut fresh34 = (*r).total_type_count;
    *fresh34 = (*fresh34).wrapping_add(1);
    let ref mut fresh35 = (*r).tuple_type_count;
    let fresh36 = *fresh35;
    *fresh35 = (*fresh35).wrapping_add(1);
    return fresh36
        .wrapping_mul(4 as libc::c_int as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint);
}
#[c2rust::src_loc = "733:1"]
unsafe extern "C" fn add_reach_type(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
) -> *mut reach_type_t {
    let mut t: *mut reach_type_t =
        ponyint_pool_alloc(3 as libc::c_int as usize) as *mut reach_type_t;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<reach_type_t>() as libc::c_ulong,
    );
    let ref mut fresh37 = (*t).name;
    *fresh37 = genname_type(type_0);
    let ref mut fresh38 = (*t).mangle;
    *fresh38 = b"o\0" as *const u8 as *const libc::c_char;
    let ref mut fresh39 = (*t).ast;
    *fresh39 = set_cap_and_ephemeral(type_0, TK_REF, TK_NONE);
    let ref mut fresh40 = (*t).ast_cap;
    *fresh40 = ast_dup(type_0);
    (*t).type_id = -(1 as libc::c_int) as u32;
    ast_set_scope((*t).ast, 0 as *mut ast_t);
    ast_set_scope((*t).ast_cap, 0 as *mut ast_t);
    reach_method_names_init(&mut (*t).methods, 0 as libc::c_int as usize);
    reach_type_cache_init(&mut (*t).subtypes, 0 as libc::c_int as usize);
    reach_types_put(&mut (*r).types, t);
    return t;
}
#[c2rust::src_loc = "754:1"]
unsafe extern "C" fn add_isect_or_union(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> *mut reach_type_t {
    let mut t: *mut reach_type_t = reach_type(r, type_0);
    if !t.is_null() {
        return t;
    }
    t = add_reach_type(r, type_0);
    (*t).underlying = ast_id((*t).ast);
    (*t).is_trait = 1 as libc::c_int != 0;
    add_types_to_trait(r, t, opt);
    if (*t).type_id == -(1 as libc::c_int) as u32 {
        (*t).type_id = get_new_trait_id(r);
    }
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        add_type(r, child, opt);
        child = ast_sibling(child);
    }
    t
}
#[c2rust::src_loc = "782:1"]
unsafe extern "C" fn add_tuple(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> *mut reach_type_t {
    if contains_dontcare(type_0) {
        return 0 as *mut reach_type_t;
    }
    let mut t: *mut reach_type_t = reach_type(r, type_0);
    if !t.is_null() {
        return t;
    }
    t = add_reach_type(r, type_0);
    (*t).underlying = TK_TUPLETYPE;
    (*t).type_id = get_new_tuple_id(r);
    (*t).can_be_boxed = 1 as libc::c_int != 0;
    (*t).field_count = ast_childcount((*t).ast) as u32;
    let ref mut fresh41 = (*t).fields;
    *fresh41 = ponyint_pool_alloc_size(
        ((*t).field_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<reach_field_t>() as libc::c_ulong),
    ) as *mut reach_field_t;
    add_traits_to_type(r, t, opt);
    add_internal(r, t, b"__is\0" as *const u8 as *const libc::c_char, opt);
    add_internal(
        r,
        t,
        b"__digestof\0" as *const u8 as *const libc::c_char,
        opt,
    );
    let mut mangle: *mut printbuf_t = printbuf_new();
    printbuf(
        mangle,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*t).field_count,
    );
    let mut child: *mut ast_t = ast_child((*t).ast_cap);
    let mut index: usize = 0;
    while !child.is_null() {
        let ref mut fresh42 = (*((*t).fields).offset(index as isize)).ast;
        *fresh42 = ast_dup(child);
        let ref mut fresh43 = (*((*t).fields).offset(index as isize)).type_0;
        *fresh43 = add_type(r, child, opt);
        (*((*t).fields).offset(index as isize)).embed = 0 as libc::c_int != 0;
        printbuf(
            mangle,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*(*((*t).fields).offset(index as isize)).type_0).mangle,
        );
        index = index.wrapping_add(1);
        child = ast_sibling(child);
    }
    let ref mut fresh44 = (*t).mangle;
    *fresh44 = stringtab((*mangle).m);
    printbuf_free(mangle);
    return t;
}
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn add_nominal(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> *mut reach_type_t {
    let mut t: *mut reach_type_t = reach_type(r, type_0);
    if !t.is_null() {
        return t;
    }
    t = add_reach_type(r, type_0);
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    (*t).underlying = ast_id(def);
    let mut pkg: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut pkg, &mut id, &mut typeargs, 0 as *mut *mut ast_t];
    ast_get_children(
        type_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut typearg: *mut ast_t = ast_child(typeargs);
    while !typearg.is_null() {
        add_type(r, typearg, opt);
        typearg = ast_sibling(typearg);
    }
    match (*t).underlying as libc::c_uint {
        72 | 73 => {
            add_types_to_trait(r, t, opt);
            (*t).is_trait = 1 as libc::c_int != 0;
        }
        74 => {
            if is_machine_word(type_0) {
                (*t).can_be_boxed = 1 as libc::c_int != 0;
                add_internal(
                    r,
                    t,
                    b"__digestof\0" as *const u8 as *const libc::c_char,
                    opt,
                );
            }
            add_traits_to_type(r, t, opt);
            add_special(
                r,
                t,
                type_0,
                b"_init\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_special(
                r,
                t,
                type_0,
                b"_final\0" as *const u8 as *const libc::c_char,
                opt,
            );
        }
        75 | 76 => {
            add_traits_to_type(r, t, opt);
            add_special(
                r,
                t,
                type_0,
                b"_final\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_special(
                r,
                t,
                type_0,
                b"_serialise_space\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_special(
                r,
                t,
                type_0,
                b"_serialise\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_special(
                r,
                t,
                type_0,
                b"_deserialise\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_fields(r, t, opt);
        }
        77 => {
            add_traits_to_type(r, t, opt);
            add_special(
                r,
                t,
                type_0,
                b"_event_notify\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_special(
                r,
                t,
                type_0,
                b"_final\0" as *const u8 as *const libc::c_char,
                opt,
            );
            add_fields(r, t, opt);
        }
        _ => {}
    }
    let mut bare: bool = 0 as libc::c_int != 0;
    if is_bare(type_0) {
        bare = 1 as libc::c_int != 0;
        let mut bare_method: *mut ast_t = 0 as *mut ast_t;
        let mut member: *mut ast_t = ast_child(ast_childidx(def, 4 as libc::c_int as usize));
        while !member.is_null() {
            if ast_id(member) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
                && ast_id(ast_child(member)) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint
            {
                if bare_method.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"bare_method == NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                            as *const u8 as *const libc::c_char,
                        901 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                            b"add_nominal\0",
                        ))
                        .as_ptr(),
                    );
                };
                bare_method = member;
            }
            member = ast_sibling(member);
        }
        if !bare_method.is_null() {
        } else {
            ponyint_assert_fail(
                b"bare_method != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                    as *const u8 as *const libc::c_char,
                911 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"add_nominal\0"))
                    .as_ptr(),
            );
        };
        let mut cap: ast_ptr_t = 0 as *mut ast_t;
        let mut name: ast_ptr_t = 0 as *mut ast_t;
        let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
        let mut children_0: [*mut *mut ast_t; 4] =
            [&mut cap, &mut name, &mut typeparams, 0 as *mut *mut ast_t];
        ast_get_children(
            bare_method,
            (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children_0.as_mut_ptr(),
        );
        if ast_id(typeparams) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(typeparams) == TK_NONE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                    as *const u8 as *const libc::c_char,
                913 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"add_nominal\0"))
                    .as_ptr(),
            );
        };
        let mut n: *mut reach_method_name_t =
            add_method_name(t, ast_name(name), 0 as libc::c_int != 0);
        let ref mut fresh45 = (*t).bare_method;
        *fresh45 = add_rmethod(r, t, n, TK_AT, 0 as *mut ast_t, opt, 0 as libc::c_int != 0);
    }
    if (*t).type_id == -(1 as libc::c_int) as u32 {
        if (*t).is_trait as libc::c_int != 0 && !bare {
            (*t).type_id = get_new_trait_id(r);
        } else if (*t).can_be_boxed {
            (*t).type_id = get_new_numeric_id(r);
        } else if (*t).underlying as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
            (*t).type_id = get_new_object_id(r);
        }
    }
    if ast_id(def) as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
        return t;
    }
    if strcmp(ast_name(pkg), b"$0\0" as *const u8 as *const libc::c_char) != 0 {
        return t;
    }
    let mut name_0: *const libc::c_char = ast_name(id);
    if *name_0.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32 {
        if strcmp(name_0, b"I8\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh46 = (*t).mangle;
            *fresh46 = b"c\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"I16\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh47 = (*t).mangle;
            *fresh47 = b"s\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"I32\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh48 = (*t).mangle;
            *fresh48 = b"i\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"I64\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh49 = (*t).mangle;
            *fresh49 = b"w\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"I128\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh50 = (*t).mangle;
            *fresh50 = b"q\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"ILong\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh51 = (*t).mangle;
            *fresh51 = b"l\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"ISize\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh52 = (*t).mangle;
            *fresh52 = b"z\0" as *const u8 as *const libc::c_char;
        }
    } else if *name_0.offset(0 as libc::c_int as isize) as libc::c_int == 'U' as i32 {
        if strcmp(name_0, b"U8\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh53 = (*t).mangle;
            *fresh53 = b"C\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"U16\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh54 = (*t).mangle;
            *fresh54 = b"S\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"U32\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh55 = (*t).mangle;
            *fresh55 = b"I\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"U64\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh56 = (*t).mangle;
            *fresh56 = b"W\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"U128\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh57 = (*t).mangle;
            *fresh57 = b"Q\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"ULong\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh58 = (*t).mangle;
            *fresh58 = b"L\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"USize\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh59 = (*t).mangle;
            *fresh59 = b"Z\0" as *const u8 as *const libc::c_char;
        }
    } else if *name_0.offset(0 as libc::c_int as isize) as libc::c_int == 'F' as i32 {
        if strcmp(name_0, b"F32\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh60 = (*t).mangle;
            *fresh60 = b"f\0" as *const u8 as *const libc::c_char;
        } else if strcmp(name_0, b"F64\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh61 = (*t).mangle;
            *fresh61 = b"d\0" as *const u8 as *const libc::c_char;
        }
    } else if strcmp(name_0, b"Bool\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh62 = (*t).mangle;
        *fresh62 = b"b\0" as *const u8 as *const libc::c_char;
    }
    return t;
}
#[c2rust::src_loc = "980:1"]
unsafe extern "C" fn add_type(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> *mut reach_type_t {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 => return add_isect_or_union(r, type_0, opt),
        150 => return add_tuple(r, type_0, opt),
        151 => return add_nominal(r, type_0, opt),
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                        as *const u8 as *const libc::c_char,
                    995 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_type\0"))
                        .as_ptr(),
                );
            };
        }
    }
    return 0 as *mut reach_type_t;
}
#[c2rust::src_loc = "1001:1"]
unsafe extern "C" fn reachable_pattern(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    match ast_id(ast) as libc::c_uint {
        2 => {}
        182 | 183 => {
            let mut idseq: ast_ptr_t = 0 as *mut ast_t;
            let mut type_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] =
                [&mut idseq, &mut type_0, 0 as *mut *mut ast_t];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut r_type: *mut ast_t = deferred_reify(reify_0, type_0, opt);
            add_type(r, r_type, opt);
            ast_free_unattached(r_type);
        }
        178 | 175 => {
            let mut child: *mut ast_t = ast_child(ast);
            while !child.is_null() {
                reachable_pattern(r, reify_0, child, opt);
                child = ast_sibling(child);
            }
        }
        _ => {
            let mut type_1: *mut ast_t = ast_type(ast);
            if ast_id(type_1) as libc::c_uint != TK_DONTCARETYPE as libc::c_int as libc::c_uint {
                reachable_method(
                    r,
                    reify_0,
                    type_1,
                    stringtab(b"eq\0" as *const u8 as *const libc::c_char),
                    0 as *mut ast_t,
                    opt,
                );
                reachable_expr(r, reify_0, ast, opt);
            }
        }
    };
}
#[c2rust::src_loc = "1047:1"]
unsafe extern "C" fn reachable_fun(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut typeargs: *mut ast_t = 0 as *mut ast_t;
    match ast_id(receiver) as libc::c_uint {
        188 | 189 | 190 | 191 | 203 | 204 => {
            typeargs = deferred_reify(reify_0, method, opt);
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut receiver, &mut method, 0 as *mut *mut ast_t];
            ast_get_children(
                receiver,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
        }
        _ => {}
    }
    let mut type_0: *mut ast_t = ast_type(receiver);
    let mut method_name: *const libc::c_char = ast_name(method);
    reachable_method(r, reify_0, type_0, method_name, typeargs, opt);
    ast_free_unattached(typeargs);
}
#[c2rust::src_loc = "1078:1"]
unsafe extern "C" fn reachable_addressof(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    let mut expr: *mut ast_t = ast_child(ast);
    match ast_id(expr) as libc::c_uint {
        191 | 190 => {
            reachable_fun(r, reify_0, expr, opt);
        }
        _ => {}
    };
}
#[c2rust::src_loc = "1094:1"]
unsafe extern "C" fn reachable_call(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    let mut postfix: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut postfix,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    reachable_fun(r, reify_0, postfix, opt);
}
#[c2rust::src_loc = "1101:1"]
unsafe extern "C" fn reachable_ffi(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    let mut decl: *mut ast_t = ast_data(ast) as *mut ast_t;
    if !decl.is_null() {
    } else {
        ponyint_assert_fail(
            b"decl != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0" as *const u8
                as *const libc::c_char,
            1105 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"reachable_ffi\0"))
                .as_ptr(),
        );
    };
    let mut decl_name: ast_ptr_t = 0 as *mut ast_t;
    let mut decl_ret_typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut named_params: ast_ptr_t = 0 as *mut ast_t;
    let mut decl_error: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut decl_name,
        &mut decl_ret_typeargs,
        &mut params,
        &mut named_params,
        &mut decl_error,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        decl,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut reified: bool = false;
    let mut return_typeargs: *mut ast_t = ast_childidx(ast, 1 as libc::c_int as usize);
    if (ast_child(return_typeargs)).is_null() {
        return_typeargs = decl_ret_typeargs;
        reified = 0 as libc::c_int != 0;
    } else {
        return_typeargs = deferred_reify(reify_0, return_typeargs, opt);
        reified = 1 as libc::c_int != 0;
    }
    let mut arg: *mut ast_t = ast_child(params);
    let mut return_type: *mut ast_t = ast_child(return_typeargs);
    add_type(r, return_type, opt);
    while !arg.is_null() {
        if ast_id(arg) as libc::c_uint != TK_ELLIPSIS as libc::c_int as libc::c_uint {
            let mut type_0: *mut ast_t = ast_type(arg);
            if type_0.is_null() {
                type_0 = ast_childidx(arg, 1 as libc::c_int as usize);
            }
            add_type(r, type_0, opt);
        }
        arg = ast_sibling(arg);
    }
    if reified {
        ast_free_unattached(return_typeargs);
    }
}
#[c2rust::src_loc = "1147:1"]
unsafe extern "C" fn reachable_expr(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    match ast_id(ast) as libc::c_uint {
        3 | 4 | 6 | 7 | 5 => {
            let mut type_0: *mut ast_t = ast_type(ast);
            if !type_0.is_null() {
                reachable_method(
                    r,
                    reify_0,
                    type_0,
                    stringtab(b"create\0" as *const u8 as *const libc::c_char),
                    0 as *mut ast_t,
                    opt,
                );
            }
        }
        85 | 84 | 193 | 192 | 194 | 178 => {
            let mut type_1: *mut ast_t = deferred_reify(reify_0, ast_type(ast), opt);
            add_type(r, type_1, opt);
            ast_free_unattached(type_1);
        }
        181 => {
            let mut pattern: ast_ptr_t = 0 as *mut ast_t;
            let mut guard: ast_ptr_t = 0 as *mut ast_t;
            let mut body: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 4] =
                [&mut pattern, &mut guard, &mut body, 0 as *mut *mut ast_t];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            reachable_pattern(r, reify_0, pattern, opt);
            reachable_expr(r, reify_0, guard, opt);
            reachable_expr(r, reify_0, body, opt);
        }
        177 => {
            reachable_call(r, reify_0, ast, opt);
        }
        143 => {
            reachable_ffi(r, reify_0, ast, opt);
        }
        134 => {
            reachable_addressof(r, reify_0, ast, opt);
        }
        108 => {
            let mut cond: ast_ptr_t = 0 as *mut ast_t;
            let mut then_clause: ast_ptr_t = 0 as *mut ast_t;
            let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 4] = [
                &mut cond,
                &mut then_clause,
                &mut else_clause,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            if ast_id(cond) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(cond) == TK_SEQ\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                        as *const u8 as *const libc::c_char,
                    1205 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"reachable_expr\0",
                    ))
                    .as_ptr(),
                );
            };
            cond = ast_child(cond);
            if is_result_needed(ast) as libc::c_int != 0
                && ast_checkflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0
            {
                let mut type_2: *mut ast_t = deferred_reify(reify_0, ast_type(ast), opt);
                add_type(r, type_2, opt);
                ast_free_unattached(type_2);
            }
            if (ast_sibling(cond)).is_null() {
                if ast_id(cond) as libc::c_uint == TK_TRUE as libc::c_int as libc::c_uint {
                    reachable_expr(r, reify_0, then_clause, opt);
                    return;
                } else {
                    if ast_id(cond) as libc::c_uint == TK_FALSE as libc::c_int as libc::c_uint {
                        reachable_expr(r, reify_0, else_clause, opt);
                        return;
                    }
                }
            }
        }
        111 => {
            let mut left_clause: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children_1: [*mut *mut ast_t; 3] =
                [&mut left_clause, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_1.as_mut_ptr(),
            );
            let mut sub: ast_ptr_t = 0 as *mut ast_t;
            let mut super_0: ast_ptr_t = 0 as *mut ast_t;
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut children_2: [*mut *mut ast_t; 4] =
                [&mut sub, &mut super_0, &mut left, 0 as *mut *mut ast_t];
            ast_get_children(
                left_clause,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_2.as_mut_ptr(),
            );
            if is_result_needed(ast) as libc::c_int != 0
                && ast_checkflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0
            {
                let mut type_3: *mut ast_t = deferred_reify(reify_0, ast_type(ast), opt);
                add_type(r, type_3, opt);
                ast_free_unattached(type_3);
            }
            let mut r_sub: *mut ast_t = deferred_reify(reify_0, sub, opt);
            let mut r_super: *mut ast_t = deferred_reify(reify_0, super_0, opt);
            if is_subtype_constraint(r_sub, r_super, 0 as *mut errorframe_t, opt) {
                reachable_expr(r, reify_0, left, opt);
            } else {
                reachable_expr(r, reify_0, right, opt);
            }
            ast_free_unattached(r_sub);
            ast_free_unattached(r_super);
            return;
        }
        122 | 116 | 118 | 124 | 206 | 107 => {
            if is_result_needed(ast) as libc::c_int != 0
                && ast_checkflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0
            {
                let mut type_4: *mut ast_t = deferred_reify(reify_0, ast_type(ast), opt);
                add_type(r, type_4, opt);
                ast_free_unattached(type_4);
            }
        }
        82 | 83 => {
            let mut left_0: ast_ptr_t = 0 as *mut ast_t;
            let mut right_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_3: [*mut *mut ast_t; 3] =
                [&mut left_0, &mut right_0, 0 as *mut *mut ast_t];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_3.as_mut_ptr(),
            );
            let mut type_5: *mut ast_t = deferred_reify(reify_0, ast_type(left_0), opt);
            add_type(r, type_5, opt);
            ast_free_unattached(type_5);
            type_5 = deferred_reify(reify_0, ast_type(right_0), opt);
            add_type(r, type_5, opt);
            ast_free_unattached(type_5);
        }
        133 => {
            let mut expr: *mut ast_t = ast_child(ast);
            let mut type_6: *mut ast_t = deferred_reify(reify_0, ast_type(expr), opt);
            add_type(r, type_6, opt);
            ast_free_unattached(type_6);
        }
        _ => {}
    }
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        reachable_expr(r, reify_0, child, opt);
        child = ast_sibling(child);
    }
}
#[c2rust::src_loc = "1312:1"]
unsafe extern "C" fn reachable_method(
    mut r: *mut reach_t,
    mut reify_0: *mut deferred_reification_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut typeargs: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    if !reify_0.is_null() {
        let mut r_type: *mut ast_t = deferred_reify(reify_0, type_0, opt);
        t = add_type(r, r_type, opt);
        ast_free_unattached(r_type);
    } else {
        t = add_type(r, type_0, opt);
    }
    let mut n: *mut reach_method_name_t = add_method_name(t, name, 0 as libc::c_int != 0);
    let mut m: *mut reach_method_t =
        add_rmethod(r, t, n, (*n).cap, typeargs, opt, 0 as libc::c_int != 0);
    if (*n).id as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
        && ((*n).cap as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
            || (*n).cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint)
    {
        if name == stringtab(b"_final\0" as *const u8 as *const libc::c_char) {
            if (*n).cap as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"n->cap == TK_BOX\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.c\0"
                        as *const u8 as *const libc::c_char,
                    1335 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"reachable_method\0",
                    ))
                    .as_ptr(),
                );
            };
            return;
        }
        let mut subordinate: bool =
            (*n).cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint;
        let mut m2: *mut reach_method_t = 0 as *mut reach_method_t;
        if (*t).underlying as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
            m2 = add_rmethod(r, t, n, TK_REF, typeargs, opt, 0 as libc::c_int != 0);
            if subordinate {
                (*m2).intrinsic = 1 as libc::c_int != 0;
                let ref mut fresh63 = (*m).subordinate;
                *fresh63 = m2;
                m = m2;
            }
        }
        m2 = add_rmethod(r, t, n, TK_VAL, typeargs, opt, 0 as libc::c_int != 0);
        if subordinate {
            (*m2).intrinsic = 1 as libc::c_int != 0;
            let ref mut fresh64 = (*m).subordinate;
            *fresh64 = m2;
            m = m2;
        }
        if (*n).cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint {
            m2 = add_rmethod(r, t, n, TK_BOX, typeargs, opt, 0 as libc::c_int != 0);
            (*m2).intrinsic = 1 as libc::c_int != 0;
            let ref mut fresh65 = (*m).subordinate;
            *fresh65 = m2;
            m = m2;
        }
    }
}
#[c2rust::src_loc = "1375:1"]
unsafe extern "C" fn handle_method_stack(mut r: *mut reach_t, mut opt: *mut pass_opt_t) {
    while !((*r).method_stack).is_null() {
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        let ref mut fresh66 = (*r).method_stack;
        *fresh66 = reach_method_stack_pop((*r).method_stack, &mut m);
        let mut body: *mut ast_t = ast_childidx((*(*m).fun).ast, 6 as libc::c_int as usize);
        reachable_expr(r, (*m).fun, body, opt);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1387:1"]
pub unsafe extern "C" fn reach_new() -> *mut reach_t {
    let mut r: *mut reach_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut reach_t;
    let ref mut fresh67 = (*r).method_stack;
    *fresh67 = 0 as *mut reach_method_stack_t;
    (*r).object_type_count = 0 as libc::c_int as u32;
    (*r).numeric_type_count = 0 as libc::c_int as u32;
    (*r).tuple_type_count = 0 as libc::c_int as u32;
    (*r).total_type_count = 0 as libc::c_int as u32;
    (*r).trait_type_count = 0 as libc::c_int as u32;
    reach_types_init(&mut (*r).types, 64 as libc::c_int as usize);
    return r;
}
#[no_mangle]
#[c2rust::src_loc = "1400:1"]
pub unsafe extern "C" fn reach_free(mut r: *mut reach_t) {
    if r.is_null() {
        return;
    }
    reach_types_destroy(&mut (*r).types);
    ponyint_pool_free(1 as libc::c_int as usize, r as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1409:1"]
pub unsafe extern "C" fn reach(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut typeargs: *mut ast_t,
    mut opt: *mut pass_opt_t,
) {
    reachable_method(
        r,
        0 as *mut deferred_reification_t,
        type_0,
        name,
        typeargs,
        opt,
    );
    handle_method_stack(r, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1416:1"]
pub unsafe extern "C" fn reach_type(
    mut r: *mut reach_t,
    mut type_0: *mut ast_t,
) -> *mut reach_type_t {
    let mut k: reach_type_t = reach_type_t {
        name: 0 as *const libc::c_char,
        mangle: 0 as *const libc::c_char,
        ast: 0 as *mut ast_t,
        ast_cap: 0 as *mut ast_t,
        underlying: TK_EOF,
        methods: reach_method_names_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        bare_method: 0 as *mut reach_method_t,
        subtypes: reach_type_cache_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        type_id: 0,
        vtable_size: 0,
        can_be_boxed: false,
        is_trait: false,
        field_count: 0,
        fields: 0 as *mut reach_field_t,
        c_type: 0 as *mut compile_opaque_t,
    };
    k.name = genname_type(type_0);
    let mut index: usize = -(1 as libc::c_int) as usize;
    return reach_types_get(&mut (*r).types, &mut k, &mut index);
}
#[no_mangle]
#[c2rust::src_loc = "1424:1"]
pub unsafe extern "C" fn reach_type_name(
    mut r: *mut reach_t,
    mut name: *const libc::c_char,
) -> *mut reach_type_t {
    let mut k: reach_type_t = reach_type_t {
        name: 0 as *const libc::c_char,
        mangle: 0 as *const libc::c_char,
        ast: 0 as *mut ast_t,
        ast_cap: 0 as *mut ast_t,
        underlying: TK_EOF,
        methods: reach_method_names_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        bare_method: 0 as *mut reach_method_t,
        subtypes: reach_type_cache_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        type_id: 0,
        vtable_size: 0,
        can_be_boxed: false,
        is_trait: false,
        field_count: 0,
        fields: 0 as *mut reach_field_t,
        c_type: 0 as *mut compile_opaque_t,
    };
    k.name = stringtab(name);
    let mut index: usize = -(1 as libc::c_int) as usize;
    return reach_types_get(&mut (*r).types, &mut k, &mut index);
}
#[no_mangle]
#[c2rust::src_loc = "1432:1"]
pub unsafe extern "C" fn reach_method(
    mut t: *mut reach_type_t,
    mut cap: token_id,
    mut name: *const libc::c_char,
    mut typeargs: *mut ast_t,
) -> *mut reach_method_t {
    let mut n: *mut reach_method_name_t = reach_method_name(t, name);
    if n.is_null() {
        return 0 as *mut reach_method_t;
    }
    if (*n).id as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
        && ((*n).cap as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
            || (*n).cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint)
    {
        match cap as libc::c_uint {
            91 | 92 => {
                cap = TK_REF;
            }
            93 | 94 | 95 => {}
            _ => {
                cap = (*n).cap;
            }
        }
    } else {
        cap = (*n).cap;
    }
    name = genname_fun(cap, (*n).name, typeargs);
    return reach_rmethod(n, name);
}
#[no_mangle]
#[c2rust::src_loc = "1465:1"]
pub unsafe extern "C" fn reach_method_name(
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
) -> *mut reach_method_name_t {
    let mut k: reach_method_name_t = reach_method_name_t {
        id: TK_EOF,
        cap: TK_EOF,
        name: 0 as *const libc::c_char,
        r_methods: reach_methods_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        r_mangled: reach_mangled_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        internal: false,
    };
    k.name = name;
    let mut index: usize = -(1 as libc::c_int) as usize;
    return reach_method_names_get(&mut (*t).methods, &mut k, &mut index);
}
#[no_mangle]
#[c2rust::src_loc = "1473:1"]
pub unsafe extern "C" fn reach_vtable_index(
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
) -> u32 {
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, name, 0 as *mut ast_t);
    if m.is_null() {
        return -(1 as libc::c_int) as u32;
    }
    return (*m).vtable_index;
}
#[no_mangle]
#[c2rust::src_loc = "1483:1"]
pub unsafe extern "C" fn reach_max_type_id(mut r: *mut reach_t) -> u32 {
    let mut object_id_max: u32 = ((*r).object_type_count)
        .wrapping_mul(2 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut numeric_id_max: u32 =
        ((*r).numeric_type_count).wrapping_mul(4 as libc::c_int as libc::c_uint);
    let mut tuple_id_max: u32 = ((*r).tuple_type_count)
        .wrapping_mul(4 as libc::c_int as libc::c_uint)
        .wrapping_add(2 as libc::c_int as libc::c_uint);
    let mut len: u32 = object_id_max;
    if len < numeric_id_max {
        len = numeric_id_max;
    }
    if len < tuple_id_max {
        len = tuple_id_max;
    }
    return len;
}
#[no_mangle]
#[c2rust::src_loc = "1500:1"]
pub unsafe extern "C" fn reach_dump(mut r: *mut reach_t) {
    printf(b"REACH\n\0" as *const u8 as *const libc::c_char);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(&mut (*r).types, &mut i);
        if t.is_null() {
            break;
        }
        printf(
            b"  %d: %s, %s\n\0" as *const u8 as *const libc::c_char,
            (*t).type_id,
            (*t).name,
            (*t).mangle,
        );
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
        printf(
            b"    vtable: %d\n\0" as *const u8 as *const libc::c_char,
            (*t).vtable_size,
        );
        loop {
            n = reach_method_names_next(&mut (*t).methods, &mut j);
            if n.is_null() {
                break;
            }
            let mut k: usize = -(1 as libc::c_int) as usize;
            let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
            loop {
                m = reach_mangled_next(&mut (*n).r_mangled, &mut k);
                if m.is_null() {
                    break;
                }
                printf(
                    b"      %d: %s\n\0" as *const u8 as *const libc::c_char,
                    (*m).vtable_index,
                    (*m).mangled_name,
                );
            }
        }
        j = -(1 as libc::c_int) as usize;
        let mut t2: *mut reach_type_t = 0 as *mut reach_type_t;
        loop {
            t2 = reach_type_cache_next(&mut (*t).subtypes, &mut j);
            if t2.is_null() {
                break;
            }
            printf(
                b"    %s\n\0" as *const u8 as *const libc::c_char,
                (*t2).name,
            );
        }
    }
}
#[c2rust::src_loc = "1534:1"]
unsafe extern "C" fn reach_param_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut p: *mut reach_param_t = object as *mut reach_param_t;
    string_trace(ctx, (*p).name);
    pony_traceknown(
        ctx,
        (*p).ast as *mut libc::c_void,
        ast_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    pony_traceknown(
        ctx,
        (*p).type_0 as *mut libc::c_void,
        reach_type_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1543:1"]
unsafe extern "C" fn reach_param_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut p: *mut reach_param_t = object as *mut reach_param_t;
    let mut dst: *mut reach_param_t = (buf as uintptr_t).wrapping_add(offset) as *mut reach_param_t;
    let ref mut fresh68 = (*dst).name;
    *fresh68 = pony_serialise_offset(ctx, (*p).name as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh69 = (*dst).ast;
    *fresh69 = pony_serialise_offset(ctx, (*p).ast as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh70 = (*dst).type_0;
    *fresh70 = pony_serialise_offset(ctx, (*p).type_0 as *mut libc::c_void) as *mut reach_type_t;
    (*dst).cap = (*p).cap;
}
#[c2rust::src_loc = "1557:1"]
unsafe extern "C" fn reach_param_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut p: *mut reach_param_t = object as *mut reach_param_t;
    let ref mut fresh71 = (*p).name;
    *fresh71 = string_deserialise_offset(ctx, (*p).name as uintptr_t);
    let ref mut fresh72 = (*p).type_0;
    *fresh72 = pony_deserialise_offset(ctx, reach_type_pony_type(), (*p).type_0 as uintptr_t)
        as *mut reach_type_t;
    let ref mut fresh73 = (*p).ast;
    *fresh73 = pony_deserialise_offset(ctx, ast_pony_type(), (*p).ast as uintptr_t) as *mut ast_t;
}
#[c2rust::src_loc = "1568:20"]
static mut reach_param_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_param_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_param_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_param_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_param_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "1589:1"]
pub unsafe extern "C" fn reach_param_pony_type() -> *const pony_type_t {
    return &reach_param_pony;
}
#[c2rust::src_loc = "1594:1"]
unsafe extern "C" fn reach_method_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut m: *mut reach_method_t = object as *mut reach_method_t;
    string_trace(ctx, (*m).name);
    string_trace(ctx, (*m).mangled_name);
    string_trace(ctx, (*m).full_name);
    if !((*m).fun).is_null() {
        pony_traceknown(
            ctx,
            (*m).fun as *mut libc::c_void,
            deferred_reification_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*m).typeargs).is_null() {
        pony_traceknown(
            ctx,
            (*m).typeargs as *mut libc::c_void,
            ast_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*m).subordinate).is_null() {
        pony_traceknown(
            ctx,
            (*m).subordinate as *mut libc::c_void,
            reach_method_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    if !((*m).params).is_null() {
        pony_serialise_reserve(
            ctx,
            (*m).params as *mut libc::c_void,
            ((*m).param_count)
                .wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
        );
        let mut i: usize = 0;
        while i < (*m).param_count {
            reach_param_serialise_trace(
                ctx,
                &mut *((*m).params).offset(i as isize) as *mut reach_param_t as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
    }
    if !((*m).result).is_null() {
        pony_traceknown(
            ctx,
            (*m).result as *mut libc::c_void,
            reach_type_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
}
#[c2rust::src_loc = "1626:1"]
unsafe extern "C" fn reach_method_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut m: *mut reach_method_t = object as *mut reach_method_t;
    let mut dst: *mut reach_method_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut reach_method_t;
    let ref mut fresh74 = (*dst).name;
    *fresh74 = pony_serialise_offset(ctx, (*m).name as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh75 = (*dst).mangled_name;
    *fresh75 = pony_serialise_offset(
        ctx,
        (*m).mangled_name as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    let ref mut fresh76 = (*dst).full_name;
    *fresh76 = pony_serialise_offset(
        ctx,
        (*m).full_name as *mut libc::c_char as *mut libc::c_void,
    ) as *const libc::c_char;
    (*dst).cap = (*m).cap;
    let ref mut fresh77 = (*dst).fun;
    *fresh77 =
        pony_serialise_offset(ctx, (*m).fun as *mut libc::c_void) as *mut deferred_reification_t;
    let ref mut fresh78 = (*dst).typeargs;
    *fresh78 = pony_serialise_offset(ctx, (*m).typeargs as *mut libc::c_void) as *mut ast_t;
    (*dst).vtable_index = (*m).vtable_index;
    (*dst).intrinsic = (*m).intrinsic;
    (*dst).internal = (*m).internal;
    (*dst).forwarding = (*m).forwarding;
    let ref mut fresh79 = (*dst).subordinate;
    *fresh79 =
        pony_serialise_offset(ctx, (*m).subordinate as *mut libc::c_void) as *mut reach_method_t;
    (*dst).param_count = (*m).param_count;
    let ref mut fresh80 = (*dst).params;
    *fresh80 = pony_serialise_offset(ctx, (*m).params as *mut libc::c_void) as *mut reach_param_t;
    if !((*m).params).is_null() {
        let mut param_offset: usize = (*dst).params as usize;
        let mut i: usize = 0;
        while i < (*m).param_count {
            reach_param_serialise(
                ctx,
                &mut *((*m).params).offset(i as isize) as *mut reach_param_t as *mut libc::c_void,
                buf,
                param_offset,
                PONY_TRACE_MUTABLE as libc::c_int,
            );
            param_offset = (param_offset as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<reach_param_t>() as libc::c_ulong)
                as usize as usize;
            i = i.wrapping_add(1);
        }
    }
    let ref mut fresh81 = (*dst).result;
    *fresh81 = pony_serialise_offset(ctx, (*m).result as *mut libc::c_void) as *mut reach_type_t;
    let ref mut fresh82 = (*dst).c_method;
    *fresh82 = 0 as *mut compile_opaque_t;
}
#[c2rust::src_loc = "1670:1"]
unsafe extern "C" fn reach_method_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut m: *mut reach_method_t = object as *mut reach_method_t;
    let ref mut fresh83 = (*m).name;
    *fresh83 = string_deserialise_offset(ctx, (*m).name as uintptr_t);
    let ref mut fresh84 = (*m).mangled_name;
    *fresh84 = string_deserialise_offset(ctx, (*m).mangled_name as uintptr_t);
    let ref mut fresh85 = (*m).full_name;
    *fresh85 = string_deserialise_offset(ctx, (*m).full_name as uintptr_t);
    let ref mut fresh86 = (*m).fun;
    *fresh86 = pony_deserialise_offset(ctx, deferred_reification_pony_type(), (*m).fun as uintptr_t)
        as *mut deferred_reification_t;
    let ref mut fresh87 = (*m).typeargs;
    *fresh87 =
        pony_deserialise_offset(ctx, ast_pony_type(), (*m).typeargs as uintptr_t) as *mut ast_t;
    let ref mut fresh88 = (*m).subordinate;
    *fresh88 = pony_deserialise_offset(ctx, reach_method_pony_type(), (*m).subordinate as uintptr_t)
        as *mut reach_method_t;
    if (*m).param_count > 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh89 = (*m).params;
        *fresh89 = pony_deserialise_block(
            ctx,
            (*m).params as uintptr_t,
            ((*m).param_count)
                .wrapping_mul(::core::mem::size_of::<reach_param_t>() as libc::c_ulong),
        ) as *mut reach_param_t;
        let mut i: usize = 0;
        while i < (*m).param_count {
            reach_param_deserialise(
                ctx,
                &mut *((*m).params).offset(i as isize) as *mut reach_param_t as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
    } else {
        let ref mut fresh90 = (*m).params;
        *fresh90 = 0 as *mut reach_param_t;
    }
    let ref mut fresh91 = (*m).result;
    *fresh91 = pony_deserialise_offset(ctx, reach_type_pony_type(), (*m).result as uintptr_t)
        as *mut reach_type_t;
}
#[c2rust::src_loc = "1702:20"]
static mut reach_method_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_method_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_method_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_method_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_method_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "1723:1"]
pub unsafe extern "C" fn reach_method_pony_type() -> *const pony_type_t {
    return &reach_method_pony;
}
#[c2rust::src_loc = "1728:1"]
unsafe extern "C" fn reach_method_name_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut n: *mut reach_method_name_t = object as *mut reach_method_name_t;
    string_trace(ctx, (*n).name);
    reach_methods_serialise_trace(
        ctx,
        &mut (*n).r_methods as *mut reach_methods_t as *mut libc::c_void,
    );
    reach_mangled_serialise_trace(
        ctx,
        &mut (*n).r_mangled as *mut reach_mangled_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1737:1"]
unsafe extern "C" fn reach_method_name_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut n: *mut reach_method_name_t = object as *mut reach_method_name_t;
    let mut dst: *mut reach_method_name_t =
        (buf as uintptr_t).wrapping_add(offset) as *mut reach_method_name_t;
    (*dst).id = (*n).id;
    (*dst).cap = (*n).cap;
    let ref mut fresh92 = (*dst).name;
    *fresh92 = pony_serialise_offset(ctx, (*n).name as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    (*dst).internal = (*n).internal;
    reach_methods_serialise(
        ctx,
        &mut (*n).r_methods as *mut reach_methods_t as *mut libc::c_void,
        buf,
        offset.wrapping_add(16 as libc::c_ulong),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    reach_mangled_serialise(
        ctx,
        &mut (*n).r_mangled as *mut reach_mangled_t as *mut libc::c_void,
        buf,
        offset.wrapping_add(48 as libc::c_ulong),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1755:1"]
unsafe extern "C" fn reach_method_name_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut n: *mut reach_method_name_t = object as *mut reach_method_name_t;
    let ref mut fresh93 = (*n).name;
    *fresh93 = string_deserialise_offset(ctx, (*n).name as uintptr_t);
    reach_methods_deserialise(
        ctx,
        &mut (*n).r_methods as *mut reach_methods_t as *mut libc::c_void,
    );
    reach_mangled_deserialise(
        ctx,
        &mut (*n).r_mangled as *mut reach_mangled_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1764:20"]
static mut reach_method_name_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_method_name_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_method_name_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_method_name_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_method_name_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "1785:1"]
pub unsafe extern "C" fn reach_method_name_pony_type() -> *const pony_type_t {
    return &reach_method_name_pony;
}
#[c2rust::src_loc = "1790:1"]
unsafe extern "C" fn reach_field_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut f: *mut reach_field_t = object as *mut reach_field_t;
    pony_traceknown(
        ctx,
        (*f).ast as *mut libc::c_void,
        ast_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    pony_traceknown(
        ctx,
        (*f).type_0 as *mut libc::c_void,
        reach_type_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
}
#[c2rust::src_loc = "1798:1"]
unsafe extern "C" fn reach_field_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut f: *mut reach_field_t = object as *mut reach_field_t;
    let mut dst: *mut reach_field_t = (buf as uintptr_t).wrapping_add(offset) as *mut reach_field_t;
    let ref mut fresh94 = (*dst).ast;
    *fresh94 = pony_serialise_offset(ctx, (*f).ast as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh95 = (*dst).type_0;
    *fresh95 = pony_serialise_offset(ctx, (*f).type_0 as *mut libc::c_void) as *mut reach_type_t;
    (*dst).embed = (*f).embed;
}
#[c2rust::src_loc = "1811:1"]
unsafe extern "C" fn reach_field_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut f: *mut reach_field_t = object as *mut reach_field_t;
    let ref mut fresh96 = (*f).ast;
    *fresh96 = pony_deserialise_offset(ctx, ast_pony_type(), (*f).ast as uintptr_t) as *mut ast_t;
    let ref mut fresh97 = (*f).type_0;
    *fresh97 = pony_deserialise_offset(ctx, reach_type_pony_type(), (*f).type_0 as uintptr_t)
        as *mut reach_type_t;
}
#[c2rust::src_loc = "1821:20"]
static mut reach_field_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_field_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_field_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_field_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_field_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "1842:1"]
pub unsafe extern "C" fn reach_field_pony_type() -> *const pony_type_t {
    return &reach_field_pony;
}
#[c2rust::src_loc = "1847:1"]
unsafe extern "C" fn reach_type_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut t: *mut reach_type_t = object as *mut reach_type_t;
    string_trace(ctx, (*t).name);
    string_trace(ctx, (*t).mangle);
    pony_traceknown(
        ctx,
        (*t).ast as *mut libc::c_void,
        ast_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    pony_traceknown(
        ctx,
        (*t).ast_cap as *mut libc::c_void,
        ast_pony_type(),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    reach_method_names_serialise_trace(
        ctx,
        &mut (*t).methods as *mut reach_method_names_t as *mut libc::c_void,
    );
    if !((*t).bare_method).is_null() {
        pony_traceknown(
            ctx,
            (*t).bare_method as *mut libc::c_void,
            reach_method_pony_type(),
            PONY_TRACE_MUTABLE as libc::c_int,
        );
    }
    reach_type_cache_serialise_trace(
        ctx,
        &mut (*t).subtypes as *mut reach_type_cache_t as *mut libc::c_void,
    );
    if !((*t).fields).is_null() {
        pony_serialise_reserve(
            ctx,
            (*t).fields as *mut libc::c_void,
            ((*t).field_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<reach_field_t>() as libc::c_ulong),
        );
        let mut i: usize = 0;
        while i < (*t).field_count as libc::c_ulong {
            reach_field_serialise_trace(
                ctx,
                &mut *((*t).fields).offset(i as isize) as *mut reach_field_t as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
    }
}
#[c2rust::src_loc = "1874:1"]
unsafe extern "C" fn reach_type_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut t: *mut reach_type_t = object as *mut reach_type_t;
    let mut dst: *mut reach_type_t = (buf as uintptr_t).wrapping_add(offset) as *mut reach_type_t;
    let ref mut fresh98 = (*dst).name;
    *fresh98 = pony_serialise_offset(ctx, (*t).name as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh99 = (*dst).mangle;
    *fresh99 = pony_serialise_offset(ctx, (*t).mangle as *mut libc::c_char as *mut libc::c_void)
        as *const libc::c_char;
    let ref mut fresh100 = (*dst).ast;
    *fresh100 = pony_serialise_offset(ctx, (*t).ast as *mut libc::c_void) as *mut ast_t;
    let ref mut fresh101 = (*dst).ast_cap;
    *fresh101 = pony_serialise_offset(ctx, (*t).ast_cap as *mut libc::c_void) as *mut ast_t;
    (*dst).underlying = (*t).underlying;
    reach_method_names_serialise(
        ctx,
        &mut (*t).methods as *mut reach_method_names_t as *mut libc::c_void,
        buf,
        offset.wrapping_add(40 as libc::c_ulong),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    let ref mut fresh102 = (*dst).bare_method;
    *fresh102 =
        pony_serialise_offset(ctx, (*t).bare_method as *mut libc::c_void) as *mut reach_method_t;
    reach_type_cache_serialise(
        ctx,
        &mut (*t).subtypes as *mut reach_type_cache_t as *mut libc::c_void,
        buf,
        offset.wrapping_add(80 as libc::c_ulong),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    (*dst).type_id = (*t).type_id;
    (*dst).vtable_size = (*t).vtable_size;
    (*dst).can_be_boxed = (*t).can_be_boxed;
    (*dst).is_trait = (*t).is_trait;
    (*dst).field_count = (*t).field_count;
    let ref mut fresh103 = (*dst).fields;
    *fresh103 = pony_serialise_offset(ctx, (*t).fields as *mut libc::c_void) as *mut reach_field_t;
    if !((*t).fields).is_null() {
        let mut field_offset: usize = (*dst).fields as usize;
        let mut i: usize = 0;
        while i < (*t).field_count as libc::c_ulong {
            reach_field_serialise(
                ctx,
                &mut *((*t).fields).offset(i as isize) as *mut reach_field_t as *mut libc::c_void,
                buf,
                field_offset,
                PONY_TRACE_MUTABLE as libc::c_int,
            );
            field_offset = (field_offset as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<reach_field_t>() as libc::c_ulong)
                as usize as usize;
            i = i.wrapping_add(1);
        }
    }
    let ref mut fresh104 = (*dst).c_type;
    *fresh104 = 0 as *mut compile_opaque_t;
}
#[c2rust::src_loc = "1916:1"]
unsafe extern "C" fn reach_type_deserialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut t: *mut reach_type_t = object as *mut reach_type_t;
    let ref mut fresh105 = (*t).name;
    *fresh105 = string_deserialise_offset(ctx, (*t).name as uintptr_t);
    let ref mut fresh106 = (*t).mangle;
    *fresh106 = string_deserialise_offset(ctx, (*t).mangle as uintptr_t);
    let ref mut fresh107 = (*t).ast;
    *fresh107 = pony_deserialise_offset(ctx, ast_pony_type(), (*t).ast as uintptr_t) as *mut ast_t;
    let ref mut fresh108 = (*t).ast_cap;
    *fresh108 =
        pony_deserialise_offset(ctx, ast_pony_type(), (*t).ast_cap as uintptr_t) as *mut ast_t;
    reach_method_names_deserialise(
        ctx,
        &mut (*t).methods as *mut reach_method_names_t as *mut libc::c_void,
    );
    let ref mut fresh109 = (*t).bare_method;
    *fresh109 =
        pony_deserialise_offset(ctx, reach_method_pony_type(), (*t).bare_method as uintptr_t)
            as *mut reach_method_t;
    reach_type_cache_deserialise(
        ctx,
        &mut (*t).subtypes as *mut reach_type_cache_t as *mut libc::c_void,
    );
    if (*t).field_count > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh110 = (*t).fields;
        *fresh110 = pony_deserialise_block(
            ctx,
            (*t).fields as uintptr_t,
            ((*t).field_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<reach_field_t>() as libc::c_ulong),
        ) as *mut reach_field_t;
        let mut i: usize = 0;
        while i < (*t).field_count as libc::c_ulong {
            reach_field_deserialise(
                ctx,
                &mut *((*t).fields).offset(i as isize) as *mut reach_field_t as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
    } else {
        let ref mut fresh111 = (*t).fields;
        *fresh111 = 0 as *mut reach_field_t;
    };
}
#[c2rust::src_loc = "1944:20"]
static mut reach_type_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_type_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_type_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_type_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_type_deserialise
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "1965:1"]
pub unsafe extern "C" fn reach_type_pony_type() -> *const pony_type_t {
    return &reach_type_pony;
}
#[c2rust::src_loc = "1970:1"]
unsafe extern "C" fn reach_serialise_trace(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
) {
    let mut r: *mut reach_t = object as *mut reach_t;
    reach_types_serialise_trace(
        ctx,
        &mut (*r).types as *mut reach_types_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1977:1"]
unsafe extern "C" fn reach_serialise(
    mut ctx: *mut pony_ctx_t,
    mut object: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut offset: usize,
    mut _mutability: libc::c_int,
) {
    let mut r: *mut reach_t = object as *mut reach_t;
    let mut dst: *mut reach_t = (buf as uintptr_t).wrapping_add(offset) as *mut reach_t;
    reach_types_serialise(
        ctx,
        &mut (*r).types as *mut reach_types_t as *mut libc::c_void,
        buf,
        offset.wrapping_add(0 as libc::c_ulong),
        PONY_TRACE_MUTABLE as libc::c_int,
    );
    let ref mut fresh112 = (*dst).method_stack;
    *fresh112 = 0 as *mut reach_method_stack_t;
    (*dst).object_type_count = (*r).object_type_count;
    (*dst).numeric_type_count = (*r).numeric_type_count;
    (*dst).tuple_type_count = (*r).tuple_type_count;
    (*dst).total_type_count = (*r).total_type_count;
    (*dst).trait_type_count = (*r).trait_type_count;
}
#[c2rust::src_loc = "1995:1"]
unsafe extern "C" fn reach_deserialise(mut ctx: *mut pony_ctx_t, mut object: *mut libc::c_void) {
    let mut r: *mut reach_t = object as *mut reach_t;
    reach_types_deserialise(
        ctx,
        &mut (*r).types as *mut reach_types_t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "2002:20"]
static mut reach_pony: pony_type_t = unsafe {
    {
        let mut init = _pony_type_t {
            id: 0 as libc::c_int as u32,
            size: ::core::mem::size_of::<reach_t>() as libc::c_ulong as u32,
            field_count: 0 as libc::c_int as u32,
            field_offset: 0 as libc::c_int as u32,
            instance: 0 as *const libc::c_void as *mut libc::c_void,
            trace: None,
            serialise_trace: Some(
                reach_serialise_trace
                    as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
            ),
            serialise: Some(
                reach_serialise
                    as unsafe extern "C" fn(
                        *mut pony_ctx_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        usize,
                        libc::c_int,
                    ) -> (),
            ),
            deserialise: Some(
                reach_deserialise as unsafe extern "C" fn(*mut pony_ctx_t, *mut libc::c_void) -> (),
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
#[c2rust::src_loc = "2023:1"]
pub unsafe extern "C" fn reach_pony_type() -> *const pony_type_t {
    return &reach_pony;
}
