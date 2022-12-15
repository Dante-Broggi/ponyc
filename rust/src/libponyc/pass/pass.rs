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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "53:1"]
        pub fn errors_alloc() -> *mut errors_t;
        #[c2rust::src_loc = "57:1"]
        pub fn errors_free(errors: *mut errors_t);
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
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "21:9"]
    pub type ast_result_t = libc::c_uint;
    #[c2rust::src_loc = "26:3"]
    pub const AST_FATAL: ast_result_t = 3;
    #[c2rust::src_loc = "25:3"]
    pub const AST_ERROR: ast_result_t = 2;
    #[c2rust::src_loc = "24:3"]
    pub const AST_IGNORE: ast_result_t = 1;
    #[c2rust::src_loc = "23:3"]
    pub const AST_OK: ast_result_t = 0;
    #[c2rust::src_loc = "29:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "51:3"]
    pub const AST_FLAG_FCNSM_REASGN: C2RustUnnamed = 33554432;
    #[c2rust::src_loc = "50:3"]
    pub const AST_FLAG_CNSM_REASGN: C2RustUnnamed = 16777216;
    #[c2rust::src_loc = "49:3"]
    pub const AST_FLAG_MAY_BREAK: C2RustUnnamed = 8388608;
    #[c2rust::src_loc = "48:3"]
    pub const AST_FLAG_IMPORT: C2RustUnnamed = 4194304;
    #[c2rust::src_loc = "47:3"]
    pub const AST_FLAG_INCOMPLETE: C2RustUnnamed = 2097152;
    #[c2rust::src_loc = "46:3"]
    pub const AST_FLAG_JUMPS_AWAY: C2RustUnnamed = 1048576;
    #[c2rust::src_loc = "45:3"]
    pub const AST_FLAG_ERROR_2: C2RustUnnamed = 524288;
    #[c2rust::src_loc = "44:3"]
    pub const AST_FLAG_DONE_2: C2RustUnnamed = 262144;
    #[c2rust::src_loc = "43:3"]
    pub const AST_FLAG_RECURSE_2: C2RustUnnamed = 131072;
    #[c2rust::src_loc = "42:3"]
    pub const AST_FLAG_ERROR_1: C2RustUnnamed = 65536;
    #[c2rust::src_loc = "41:3"]
    pub const AST_FLAG_DONE_1: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "40:3"]
    pub const AST_FLAG_RECURSE_1: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "39:3"]
    pub const AST_FLAG_PRESERVE: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "38:3"]
    pub const AST_FLAG_MISSING_SEMI: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "37:3"]
    pub const AST_FLAG_BAD_SEMI: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "36:3"]
    pub const AST_FLAG_AMBIGUOUS: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "35:3"]
    pub const AST_FLAG_IN_PARENS: C2RustUnnamed = 512;
    #[c2rust::src_loc = "34:3"]
    pub const AST_FLAG_MIGHT_SEND: C2RustUnnamed = 256;
    #[c2rust::src_loc = "33:3"]
    pub const AST_FLAG_CAN_SEND: C2RustUnnamed = 128;
    #[c2rust::src_loc = "32:3"]
    pub const AST_FLAG_CAN_ERROR: C2RustUnnamed = 64;
    #[c2rust::src_loc = "31:3"]
    pub const AST_FLAG_PASS_MASK: C2RustUnnamed = 31;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: u32) -> libc::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: u32);
        #[c2rust::src_loc = "90:1"]
        pub fn ast_clearflag(ast: *mut ast_t, flag: u32);
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "150:1"]
        pub fn ast_freeze(ast: *mut ast_t);
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

    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn frame_push(t: *mut typecheck_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "58:1"]
        pub fn frame_pop(t: *mut typecheck_t);
    }
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
    #[c2rust::src_loc = "402:1"]
    pub type ast_visit_t =
        Option<unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t>;

    use super::ast_h::ast_result_t;
    use super::frame_h::typecheck_t;
    use super::stringtab_h::strlist_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.h:2"]
pub mod syntax_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_syntax(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.h:3"]
pub mod sugar_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn pass_sugar(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/scope.h:4"]
pub mod scope_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn pass_scope(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/import.h:5"]
pub mod import_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn pass_import(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/names.h:6"]
pub mod names_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn pass_names(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/flatten.h:7"]
pub mod flatten_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn pass_flatten(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.h:8"]
pub mod traits_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn pass_traits(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.h:9"]
pub mod refer_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn pass_pre_refer(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
        #[c2rust::src_loc = "15:1"]
        pub fn pass_refer(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:10"]
pub mod expr_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn pass_pre_expr(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
        #[c2rust::src_loc = "22:1"]
        pub fn pass_expr(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/completeness.h:11"]
pub mod completeness_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_completeness(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/verify.h:12"]
pub mod verify_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_verify(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/finalisers.h:13"]
pub mod finalisers_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_finalisers(program: *mut ast_t, options: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/serialisers.h:14"]
pub mod serialisers_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_serialisers(program: *mut ast_t, options: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/docgen.h:15"]
pub mod docgen_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn generate_docs(program: *mut ast_t, options: *mut pass_opt_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.h:17"]
pub mod parser_h {
    use super::error_h::errors_t;
    use super::source_h::source_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "7:1"]
        pub fn pass_parse(
            package: *mut ast_t,
            source: *mut source_t,
            errors: *mut errors_t,
            allow_test_symbols: bool,
            trace: bool,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/treecheck.h:18"]
pub mod treecheck_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn check_tree(tree: *mut ast_t, opt: *mut pass_opt_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/codegen.h:19"]
pub mod codegen_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "239:1"]
        pub fn codegen(program: *mut ast_t, opt: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.h:20"]
pub mod program_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn program_signature(program: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "47:1"]
        pub fn program_dump(program: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/plugin/plugin.h:21"]
pub mod plugin_h {
    use super::pass_h::{pass_id, pass_opt_t};
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn plugin_visit_ast(ast: *const ast_t, opt: *mut pass_opt_t, pass: pass_id);
        #[c2rust::src_loc = "82:1"]
        pub fn plugin_unload(opt: *mut pass_opt_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:23"]
pub mod ponyassert_h {

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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:25"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __int64_t};
pub use self::ast_h::{
    ast_checkflag, ast_child, ast_clearflag, ast_freeze, ast_id, ast_nearest, ast_parent,
    ast_result_t, ast_setflag, ast_sibling, C2RustUnnamed, AST_ERROR, AST_FATAL,
    AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2, AST_IGNORE,
    AST_OK,
};
use self::codegen_h::codegen;
use self::completeness_h::pass_completeness;
use self::docgen_h::generate_docs;
use self::error_h::{errors_alloc, errors_free, errors_t};
use self::expr_h::{pass_expr, pass_pre_expr};
use self::finalisers_h::pass_finalisers;
use self::flatten_h::pass_flatten;
pub use self::frame_h::{frame_pop, frame_push, typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::import_h::pass_import;
use self::names_h::pass_names;
use self::parser_h::pass_parse;
pub use self::pass_h::{
    ast_visit_t, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL,
    PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::plugin_h::{plugin_unload, plugin_visit_ast};
use self::ponyassert_h::ponyint_assert_fail;
use self::program_h::{program_dump, program_signature};
use self::refer_h::{pass_pre_refer, pass_refer};
use self::scope_h::pass_scope;
use self::serialisers_h::pass_serialisers;
pub use self::source_h::source_t;
use self::stdio_h::{__stderrp, fprintf};
use self::string_h::{memset, strcmp};

use self::sugar_h::pass_sugar;
use self::symtab_h::ast_t;
use self::syntax_h::pass_syntax;
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
use self::traits_h::pass_traits;
use self::treecheck_h::check_tree;
use self::verify_h::pass_verify;
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn limit_passes(
    mut opt: *mut pass_opt_t,
    mut pass: *const libc::c_char,
) -> bool {
    let mut i: pass_id = PASS_PARSE;
    loop {
        if strcmp(pass, pass_name(i)) == 0 as libc::c_int {
            (*opt).limit = i;
            return 1 as libc::c_int != 0;
        }
        if i as libc::c_uint == PASS_ALL as libc::c_int as libc::c_uint {
            return 0 as libc::c_int != 0;
        }
        i = pass_next(i);
    }
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn pass_name(mut pass: pass_id) -> *const libc::c_char {
    match pass as libc::c_uint {
        0 => b"parse\0" as *const u8 as *const libc::c_char,
        1 => b"syntax\0" as *const u8 as *const libc::c_char,
        2 => b"sugar\0" as *const u8 as *const libc::c_char,
        3 => b"scope\0" as *const u8 as *const libc::c_char,
        4 => b"import\0" as *const u8 as *const libc::c_char,
        5 => b"name\0" as *const u8 as *const libc::c_char,
        6 => b"flatten\0" as *const u8 as *const libc::c_char,
        7 => b"traits\0" as *const u8 as *const libc::c_char,
        8 => b"docs\0" as *const u8 as *const libc::c_char,
        9 => b"refer\0" as *const u8 as *const libc::c_char,
        10 => b"expr\0" as *const u8 as *const libc::c_char,
        11 => b"completeness\0" as *const u8 as *const libc::c_char,
        12 => b"verify\0" as *const u8 as *const libc::c_char,
        13 => b"final\0" as *const u8 as *const libc::c_char,
        14 => b"serialise\0" as *const u8 as *const libc::c_char,
        15 => b"reach\0" as *const u8 as *const libc::c_char,
        16 => b"paint\0" as *const u8 as *const libc::c_char,
        17 => b"ir\0" as *const u8 as *const libc::c_char,
        18 => b"bitcode\0" as *const u8 as *const libc::c_char,
        19 => b"asm\0" as *const u8 as *const libc::c_char,
        20 => b"obj\0" as *const u8 as *const libc::c_char,
        21 => b"all\0" as *const u8 as *const libc::c_char,
        _ => b"error\0" as *const u8 as *const libc::c_char,
    }
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn pass_next(mut pass: pass_id) -> pass_id {
    if pass as libc::c_uint == PASS_ALL as libc::c_int as libc::c_uint {
        return PASS_ALL;
    }
    return (pass as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as pass_id;
}
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn pass_prev(mut pass: pass_id) -> pass_id {
    if pass as libc::c_uint == PASS_PARSE as libc::c_int as libc::c_uint {
        return PASS_PARSE;
    }
    return (pass as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint) as pass_id;
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn pass_opt_init(mut options: *mut pass_opt_t) {
    memset(
        options as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<pass_opt_t>().try_into().unwrap(),
    );
    (*options).limit = PASS_ALL;
    (*options).verbosity = VERBOSITY_INFO;
    let ref mut fresh0 = (*options).check.errors;
    *fresh0 = errors_alloc();
    (*options).ast_print_width = 80 as libc::c_int as usize;
    frame_push(&mut (*options).check, 0 as *mut ast_t);
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn pass_opt_done(mut options: *mut pass_opt_t) {
    plugin_unload(options);
    errors_free((*options).check.errors);
    let ref mut fresh1 = (*options).check.errors;
    *fresh1 = 0 as *mut errors_t;
    frame_pop(&mut (*options).check);
    if ((*options).check.frame).is_null() {
    } else {
        ponyint_assert_fail(
            b"options->check.frame == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            120 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"pass_opt_done\0"))
                .as_ptr(),
        );
    };
    if (*options).print_stats {
        fprintf(
            __stderrp,
            b"\nStats:\n  Names: %zu\n  Default caps: %zu\n  Heap alloc: %zu\n  Stack alloc: %zu\n\0"
                as *const u8 as *const libc::c_char,
            (*options).check.stats.names_count,
            (*options).check.stats.default_caps_count,
            (*options).check.stats.heap_alloc,
            (*options).check.stats.stack_alloc,
        );
    }
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn check_limit(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
    mut pass: pass_id,
    mut last_pass: pass_id,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            147 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"check_limit\0")).as_ptr(),
        );
    };
    if !(*astp).is_null() {
    } else {
        ponyint_assert_fail(
            b"*astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            148 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"check_limit\0")).as_ptr(),
        );
    };
    if !options.is_null() {
    } else {
        ponyint_assert_fail(
            b"options != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"check_limit\0")).as_ptr(),
        );
    };
    if (last_pass as libc::c_uint) < pass as libc::c_uint
        || ((*options).limit as libc::c_uint) < pass as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    if ast_id(*astp) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint {
        (*options).program_pass = pass;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn visit_pass(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
    mut last_pass: pass_id,
    mut out_r: *mut bool,
    mut pass: pass_id,
    mut pre_fn: ast_visit_t,
    mut post_fn: ast_visit_t,
) -> bool {
    if !out_r.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_r != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            167 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"visit_pass\0")).as_ptr(),
        );
    };
    if !check_limit(astp, options, pass, last_pass) {
        *out_r = 1 as libc::c_int != 0;
        return 0 as libc::c_int != 0;
    }
    if ast_visit(astp, pre_fn, post_fn, options, pass) as libc::c_uint
        != AST_OK as libc::c_int as libc::c_uint
    {
        *out_r = 0 as libc::c_int != 0;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "188:1"]
pub unsafe extern "C" fn module_passes(
    mut package: *mut ast_t,
    mut options: *mut pass_opt_t,
    mut source: *mut source_t,
) -> bool {
    if !pass_parse(
        package,
        source,
        (*options).check.errors,
        (*options).allow_test_symbols,
        (*options).parse_trace,
    ) {
        return 0 as libc::c_int != 0;
    }
    if ((*options).limit as libc::c_uint) < PASS_SYNTAX as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut module: *mut ast_t = ast_child(package);
    if ast_visit(
        &mut module,
        Some(pass_syntax as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
        options,
        PASS_SYNTAX,
    ) as libc::c_uint
        != AST_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    if (*options).check_tree {
        check_tree(module, options);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn ast_passes(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
    mut last: pass_id,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            211 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"ast_passes\0")).as_ptr(),
        );
    };
    let mut r: bool = false;
    let mut is_program: bool =
        ast_id(*astp) as libc::c_uint == TK_PROGRAM as libc::c_int as libc::c_uint;
    if is_program {
        plugin_visit_ast(*astp, options, PASS_SYNTAX);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_SUGAR,
        Some(pass_sugar as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_SUGAR);
    }
    if (*options).check_tree {
        check_tree(*astp, options);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_SCOPE,
        Some(pass_scope as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_SCOPE);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_IMPORT,
        Some(pass_import as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_IMPORT);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_NAME_RESOLUTION,
        None,
        Some(pass_names as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_NAME_RESOLUTION);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_FLATTEN,
        None,
        Some(
            pass_flatten as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
        ),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_FLATTEN);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_TRAITS,
        Some(pass_traits as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_TRAITS);
    }
    if !check_limit(astp, options, PASS_DOCS, last) {
        return 1 as libc::c_int != 0;
    }
    if is_program as libc::c_int != 0 && (*options).docs as libc::c_int != 0 {
        generate_docs(*astp, options);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_REFER,
        Some(
            pass_pre_refer
                as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
        ),
        Some(pass_refer as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_REFER);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_EXPR,
        Some(
            pass_pre_expr as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
        ),
        Some(pass_expr as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_EXPR);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_COMPLETENESS,
        None,
        Some(
            pass_completeness
                as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
        ),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_COMPLETENESS);
    }
    if !visit_pass(
        astp,
        options,
        last,
        &mut r,
        PASS_VERIFY,
        None,
        Some(pass_verify as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
    ) {
        return r;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_VERIFY);
    }
    if !check_limit(astp, options, PASS_FINALISER, last) {
        return 1 as libc::c_int != 0;
    }
    if !pass_finalisers(*astp, options) {
        return 0 as libc::c_int != 0;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_FINALISER);
    }
    if !check_limit(astp, options, PASS_SERIALISER, last) {
        return 1 as libc::c_int != 0;
    }
    if !pass_serialisers(*astp, options) {
        return 0 as libc::c_int != 0;
    }
    if is_program {
        plugin_visit_ast(*astp, options, PASS_SERIALISER);
    }
    if (*options).check_tree {
        check_tree(*astp, options);
    }
    ast_freeze(*astp);
    if is_program {
        program_signature(*astp);
        if (*options).verbosity as libc::c_uint
            >= VERBOSITY_TOOL_INFO as libc::c_int as libc::c_uint
        {
            program_dump(*astp);
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "325:1"]
pub unsafe extern "C" fn ast_passes_program(
    mut ast: *mut ast_t,
    mut options: *mut pass_opt_t,
) -> bool {
    ast_passes(&mut ast, options, PASS_ALL)
}
#[no_mangle]
#[c2rust::src_loc = "331:1"]
pub unsafe extern "C" fn ast_passes_type(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
    mut last_pass: pass_id,
) -> bool {
    let mut ast: *mut ast_t = *astp;
    if ast_id(ast) as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_TRAIT as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_INTERFACE as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ACTOR || ast_id(ast) == TK_CLASS || ast_id(ast) == TK_STRUCT || ast_id(ast) == TK_PRIMITIVE || ast_id(ast) == TK_TRAIT || ast_id(ast) == TK_INTERFACE\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0"
                as *const u8 as *const libc::c_char,
            337 as libc::c_int as usize,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"ast_passes_type\0"))
                .as_ptr(),
        );
    };
    let mut module: *mut ast_t = ast_parent(ast);
    let mut package: *mut ast_t = ast_parent(module);
    frame_push(&mut (*options).check, 0 as *mut ast_t);
    frame_push(&mut (*options).check, package);
    frame_push(&mut (*options).check, module);
    let mut ok: bool = ast_passes(astp, options, last_pass);
    frame_pop(&mut (*options).check);
    frame_pop(&mut (*options).check);
    frame_pop(&mut (*options).check);
    ok
}
#[no_mangle]
#[c2rust::src_loc = "358:1"]
pub unsafe extern "C" fn ast_passes_subtree(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
    mut last_pass: pass_id,
) -> bool {
    ast_passes(astp, options, last_pass)
}
#[no_mangle]
#[c2rust::src_loc = "364:1"]
pub unsafe extern "C" fn generate_passes(
    mut program: *mut ast_t,
    mut options: *mut pass_opt_t,
) -> bool {
    if ((*options).limit as libc::c_uint) < PASS_REACH as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    codegen(program, options)
}
#[no_mangle]
#[c2rust::src_loc = "373:1"]
pub unsafe extern "C" fn ast_pass_record(mut ast: *mut ast_t, mut pass: pass_id) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            375 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_pass_record\0"))
                .as_ptr(),
        );
    };
    if pass as libc::c_uint == PASS_ALL as libc::c_int as libc::c_uint {
        return;
    }
    ast_clearflag(ast, AST_FLAG_PASS_MASK as libc::c_int as u32);
    ast_setflag(ast, pass as libc::c_int as u32);
}
#[no_mangle]
#[c2rust::src_loc = "385:1"]
pub unsafe extern "C" fn ast_visit(
    mut ast: *mut *mut ast_t,
    mut pre: ast_visit_t,
    mut post: ast_visit_t,
    mut options: *mut pass_opt_t,
    mut pass: pass_id,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            388 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_visit\0")).as_ptr(),
        );
    };
    if !(*ast).is_null() {
    } else {
        ponyint_assert_fail(
            b"*ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            389 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"ast_visit\0")).as_ptr(),
        );
    };
    let mut ast_pass: pass_id =
        ast_checkflag(*ast, AST_FLAG_PASS_MASK as libc::c_int as u32) as pass_id;
    if ast_pass as libc::c_uint >= pass as libc::c_uint {
        return AST_OK;
    }
    if pass as libc::c_uint > PASS_SYNTAX as libc::c_int as libc::c_uint
        && ast_checkflag(*ast, AST_FLAG_PRESERVE as libc::c_int as u32) != 0
    {
        return AST_OK;
    }
    let mut t: *mut typecheck_t = &mut (*options).check;
    let mut pop: bool = frame_push(t, *ast);
    let mut ret: ast_result_t = AST_OK;
    let mut ignore: bool = 0 as libc::c_int != 0;
    if pre.is_some() {
        match pre.expect("non-null function pointer")(ast, options) as libc::c_uint {
            1 => {
                ignore = 1 as libc::c_int != 0;
            }
            2 => {
                ret = AST_ERROR;
            }
            3 => {
                ast_pass_record(*ast, pass);
                if pop {
                    frame_pop(t);
                }
                return AST_FATAL;
            }
            0 | _ => {}
        }
    }
    if !ignore && (pre.is_some() || post.is_some()) {
        let mut child: *mut ast_t = ast_child(*ast);
        while !child.is_null() {
            match ast_visit(&mut child, pre, post, options, pass) as libc::c_uint {
                1 => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0"
                                as *const u8 as *const libc::c_char,
                            444 as libc::c_int as usize,
                            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
                                b"ast_visit\0",
                            ))
                            .as_ptr(),
                        );
                    };
                }
                2 => {
                    ret = AST_ERROR;
                }
                3 => {
                    ast_pass_record(*ast, pass);
                    if pop {
                        frame_pop(t);
                    }
                    return AST_FATAL;
                }
                0 | _ => {}
            }
            child = ast_sibling(child);
        }
    }
    if !ignore && post.is_some() {
        match post.expect("non-null function pointer")(ast, options) as libc::c_uint {
            2 => {
                ret = AST_ERROR;
            }
            3 => {
                ast_pass_record(*ast, pass);
                if pop {
                    frame_pop(t);
                }
                return AST_FATAL;
            }
            0 | 1 | _ => {}
        }
    }
    if pop {
        frame_pop(t);
    }
    ast_pass_record(*ast, pass);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "494:1"]
pub unsafe extern "C" fn ast_visit_scope(
    mut ast: *mut *mut ast_t,
    mut pre: ast_visit_t,
    mut post: ast_visit_t,
    mut options: *mut pass_opt_t,
    mut pass: pass_id,
) -> ast_result_t {
    let mut t: *mut typecheck_t = &mut (*options).check;
    let mut module: *mut ast_t = ast_nearest(*ast, TK_MODULE);
    let mut package: *mut ast_t = ast_parent(module);
    if !module.is_null() {
    } else {
        ponyint_assert_fail(
            b"module != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            500 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_visit_scope\0"))
                .as_ptr(),
        );
    };
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/pass.c\0" as *const u8
                as *const libc::c_char,
            501 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ast_visit_scope\0"))
                .as_ptr(),
        );
    };
    frame_push(t, 0 as *mut ast_t);
    frame_push(t, package);
    frame_push(t, module);
    let mut ret: ast_result_t = ast_visit(ast, pre, post, options, pass);
    frame_pop(t);
    frame_pop(t);
    frame_pop(t);
    return ret;
}
