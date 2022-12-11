use ::libc;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
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
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
    }
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
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn lexint_cmp(a: *const lexint_t, b: *const lexint_t) -> libc::c_int;
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::error_h::errors_t;
    use super::lexint_h::lexint_t;
    use super::symtab_h::ast_t;
    use super::token_h::{token_id};
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "63:1"]
        pub fn ast_dup(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ast_setdata(ast: *mut ast_t, data: *mut libc::c_void) -> *mut ast_t;
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
        #[c2rust::src_loc = "93:1"]
        pub fn ast_get_print(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "99:1"]
        pub fn ast_int(ast: *mut ast_t) -> *mut lexint_t;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "101:1"]
        pub fn ast_settype(ast: *mut ast_t, type_0: *mut ast_t);
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "114:1"]
        pub fn ast_childlast(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
            out_children: *mut *mut *mut ast_t,
        );
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
        pub names_count: size_t,
        pub default_caps_count: size_t,
        pub heap_alloc: size_t,
        pub stack_alloc: size_t,
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
        pub ast_print_width: size_t,
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:4"]
pub mod expr_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn is_typecheck_error(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "22:1"]
        pub fn pass_expr(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:5"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:6"]
pub mod assemble_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn type_builtin(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            name: *const libc::c_char,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "20:1"]
        pub fn type_builtin_args(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "50:1"]
        pub fn type_union(
            opt: *mut pass_opt_t,
            l_type: *mut ast_t,
            r_type: *mut ast_t,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:8"]
pub mod reify_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn check_constraints(
            orig: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            report_errors: bool,
            opt: *mut pass_opt_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:11"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:12"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint64_t_h::uint64_t;
use self::assemble_h::{type_builtin, type_builtin_args, type_union};
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_child, ast_childcount, ast_childidx, ast_childlast, ast_data,
    ast_dup, ast_error, ast_free, ast_free_unattached, ast_from, ast_from_string, ast_get_children,
    ast_get_print, ast_id, ast_inheritflags, ast_int, ast_name, ast_parent, ast_ptr_t,
    ast_result_t, ast_setdata, ast_setid, ast_settype, ast_sibling, ast_type, AST_ERROR, AST_FATAL,
    AST_IGNORE, AST_OK,
};
pub use self::error_h::{errorframe_t, errormsg_t, errors_t};
use self::expr_h::{is_typecheck_error, pass_expr};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::lexint_h::{lexint_cmp, lexint_t};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::reify_h::check_constraints;
use self::string_h::strcmp;

use self::subtype_h::is_subtype;
use self::symtab_h::ast_t;
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
#[c2rust::src_loc = "48:16"]
pub struct lit_op_info_t {
    pub name: *const libc::c_char,
    pub arg_count: size_t,
    pub can_propogate_literal: bool,
    pub neg_plus_one: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "149:16"]
pub struct lit_chain_t {
    pub cardinality: size_t,
    pub index: size_t,
    pub formal: *mut ast_t,
    pub cached_type: *mut ast_t,
    pub name: *const libc::c_char,
    pub cached_uif_index: libc::c_int,
    pub valid_for_float: bool,
    pub next: *mut lit_chain_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "22:8"]
pub struct C2RustUnnamed {
    pub name: *const libc::c_char,
    pub limit: lexint_t,
    pub neg_plus_one: bool,
}
#[c2rust::src_loc = "27:9"]
static mut _str_uif_types: [C2RustUnnamed; 16] = [
    {
        let mut init = C2RustUnnamed {
            name: b"U8\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x100 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"U16\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x10000 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"U32\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x100000000 as libc::c_longlong as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"U64\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 1 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"U128\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"ULong\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 1 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"USize\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 1 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"I8\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x80 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"I16\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x8000 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"I32\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x80000000 as libc::c_uint as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"I64\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x8000000000000000 as libc::c_ulonglong,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"I128\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 0x8000000000000000 as libc::c_ulonglong,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"ILong\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x8000000000000000 as libc::c_ulonglong,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"ISize\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0x8000000000000000 as libc::c_ulonglong,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"F32\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"F64\0" as *const u8 as *const libc::c_char,
            limit: {
                let mut init = lexint_t {
                    low: 0 as libc::c_int as uint64_t,
                    high: 0 as libc::c_int as uint64_t,
                };
                init
            },
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
];
#[c2rust::src_loc = "56:28"]
static mut _operator_fns: [lit_op_info_t; 19] = [
    {
        let mut init = lit_op_info_t {
            name: b"add\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"sub\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"mul\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"div\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"rem\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"neg\0" as *const u8 as *const libc::c_char,
            arg_count: 0 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 1 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"shl\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"shr\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"op_and\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"op_or\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"op_xor\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"op_not\0" as *const u8 as *const libc::c_char,
            arg_count: 0 as libc::c_int as size_t,
            can_propogate_literal: 1 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"eq\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"ne\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"lt\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"le\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"gt\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: b"ge\0" as *const u8 as *const libc::c_char,
            arg_count: 1 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = lit_op_info_t {
            name: 0 as *const libc::c_char,
            arg_count: 0 as libc::c_int as size_t,
            can_propogate_literal: 0 as libc::c_int != 0,
            neg_plus_one: 0 as libc::c_int != 0,
        };
        init
    },
];
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn lookup_literal_op(mut name: *const libc::c_char) -> *const lit_op_info_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(_operator_fns[i as usize].name).is_null() {
        if strcmp(name, _operator_fns[i as usize].name) == 0 as libc::c_int {
            return &*_operator_fns.as_ptr().offset(i as isize) as *const lit_op_info_t;
        }
        i += 1;
    }
    return 0 as *const lit_op_info_t;
}
#[no_mangle]
#[c2rust::src_loc = "90:1"]
pub unsafe extern "C" fn operatorliteral_serialise_data(mut ast: *mut ast_t, mut dst: *mut ast_t) {
    let mut data: *const lit_op_info_t = ast_data(ast) as *mut lit_op_info_t;
    if !data.is_null() {
        let mut index: size_t = data.offset_from(_operator_fns.as_ptr()) as libc::c_long as size_t;
        ast_setdata(dst, index as *mut libc::c_void);
    } else {
        ast_setdata(dst, !(0 as libc::c_int) as size_t as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn operatorliteral_deserialise_data(mut ast: *mut ast_t) {
    let mut index: size_t = ast_data(ast) as size_t;
    if index > 17 as libc::c_int as libc::c_ulong {
        ast_setdata(ast, 0 as *mut libc::c_void);
    } else {
        ast_setdata(
            ast,
            &*_operator_fns.as_ptr().offset(index as isize) as *const lit_op_info_t
                as *mut libc::c_void,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn expr_literal(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
) -> bool {
    let mut type_0: *mut ast_t = type_builtin(opt, ast, name);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, type_0);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "126:1"]
pub unsafe extern "C" fn make_literal_type(mut ast: *mut ast_t) {
    let mut type_0: *mut ast_t = ast_from(ast, TK_LITERAL);
    ast_settype(ast, type_0);
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn is_type_literal(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    return ast_id(type_0) as libc::c_uint == TK_LITERAL as libc::c_int as libc::c_uint;
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn chain_init_head(mut head: *mut lit_chain_t) {
    if !head.is_null() {
    } else {
        ponyint_assert_fail(
            b"head != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            172 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"chain_init_head\0"))
                .as_ptr(),
        );
    };
    (*head).cardinality = 0 as libc::c_int as size_t;
    let ref mut fresh0 = (*head).formal;
    *fresh0 = 0 as *mut ast_t;
    let ref mut fresh1 = (*head).cached_type;
    *fresh1 = 0 as *mut ast_t;
    let ref mut fresh2 = (*head).name;
    *fresh2 = 0 as *const libc::c_char;
    (*head).cached_uif_index = -(1 as libc::c_int);
    (*head).valid_for_float = 0 as libc::c_int != 0;
    let ref mut fresh3 = (*head).next;
    *fresh3 = head;
}
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn chain_clear_cache(mut chain: *mut lit_chain_t) {
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"chain_clear_cache\0"))
                .as_ptr(),
        );
    };
    while (*chain).cardinality != 0 as libc::c_int as libc::c_ulong {
        chain = (*chain).next;
    }
    let ref mut fresh4 = (*chain).formal;
    *fresh4 = 0 as *mut ast_t;
    let ref mut fresh5 = (*chain).cached_type;
    *fresh5 = 0 as *mut ast_t;
    let ref mut fresh6 = (*chain).name;
    *fresh6 = 0 as *const libc::c_char;
    (*chain).cached_uif_index = -(1 as libc::c_int);
    (*chain).valid_for_float = 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn chain_add(
    mut chain: *mut lit_chain_t,
    mut new_link: *mut lit_chain_t,
    mut cardinality: size_t,
) {
    if !new_link.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_link != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            202 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"chain_add\0")).as_ptr(),
        );
    };
    if cardinality != 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"cardinality != CHAIN_CARD_BASE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"chain_add\0")).as_ptr(),
        );
    };
    (*new_link).cardinality = cardinality;
    (*new_link).index = 0 as libc::c_int as size_t;
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            208 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"chain_add\0")).as_ptr(),
        );
    };
    if !((*chain).next).is_null() {
    } else {
        ponyint_assert_fail(
            b"chain->next != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            209 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"chain_add\0")).as_ptr(),
        );
    };
    let ref mut fresh7 = (*new_link).next;
    *fresh7 = (*chain).next;
    let ref mut fresh8 = (*chain).next;
    *fresh8 = new_link;
    chain_clear_cache(new_link);
}
#[c2rust::src_loc = "217:1"]
unsafe extern "C" fn chain_remove(mut old_tail: *mut lit_chain_t) {
    if !old_tail.is_null() {
    } else {
        ponyint_assert_fail(
            b"old_tail != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            219 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"chain_remove\0")).as_ptr(),
        );
    };
    if !((*old_tail).next).is_null() {
    } else {
        ponyint_assert_fail(
            b"old_tail->next != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            220 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"chain_remove\0")).as_ptr(),
        );
    };
    if !((*(*old_tail).next).next).is_null() {
    } else {
        ponyint_assert_fail(
            b"old_tail->next->next != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            221 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"chain_remove\0")).as_ptr(),
        );
    };
    if (*(*(*old_tail).next).next).cardinality == 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"old_tail->next->next->cardinality == CHAIN_CARD_BASE\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            222 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"chain_remove\0")).as_ptr(),
        );
    };
    let ref mut fresh9 = (*old_tail).next;
    *fresh9 = (*(*old_tail).next).next;
    chain_clear_cache(old_tail);
}
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn uifset_simple_type(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
) -> libc::c_int {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            234 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"uifset_simple_type\0"))
                .as_ptr(),
        );
    };
    let mut set: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut uif: *mut ast_t = type_builtin(opt, type_0, _str_uif_types[i as usize].name);
        ast_setid(ast_childidx(uif, 3 as libc::c_int as size_t), TK_VAL);
        ast_setid(ast_childidx(uif, 4 as libc::c_int as size_t), TK_EPHEMERAL);
        if is_subtype(uif, type_0, 0 as *mut errorframe_t, opt) {
            set |= (1 as libc::c_int) << i;
        }
        ast_free(uif);
        i += 1;
    }
    return set;
}
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn uifset_formal_param(
    mut opt: *mut pass_opt_t,
    mut type_param_ref: *mut ast_t,
    mut chain: *mut lit_chain_t,
) -> libc::c_int {
    if !type_param_ref.is_null() {
    } else {
        ponyint_assert_fail(
            b"type_param_ref != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            258 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    if ast_id(type_param_ref) as libc::c_uint == TK_TYPEPARAMREF as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type_param_ref) == TK_TYPEPARAMREF\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            259 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    let mut type_param: *mut ast_t = ast_data(type_param_ref) as *mut ast_t;
    if !type_param.is_null() {
    } else {
        ponyint_assert_fail(
            b"type_param != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    if ast_id(type_param) as libc::c_uint == TK_TYPEPARAM as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type_param) == TK_TYPEPARAM\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            264 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    let mut constraint: *mut ast_t = ast_childidx(type_param, 1 as libc::c_int as size_t);
    if !constraint.is_null() {
    } else {
        ponyint_assert_fail(
            b"constraint != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            268 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uifset_formal_param\0"))
                .as_ptr(),
        );
    };
    let mut typeargs: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = type_param;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_TYPEARGS);
    if parent.is_null() {
        parent = node;
    } else if last_sibling.is_null() {
        last_sibling = ast_add(parent, node);
    } else {
        last_sibling = ast_add_sibling(last_sibling, node);
    }
    let mut parent_0: *mut ast_t = node;
    let mut last_sibling_0: *mut ast_t = 0 as *mut ast_t;
    let mut node_0: *mut ast_t = 0 as *mut ast_t;
    node_0 = ast_from(basis_ast, TK_TYPEPARAMREF);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_1: *mut ast_t = node_0;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut node_1: *mut ast_t = 0 as *mut ast_t;
    ast_setdata(parent_1, type_param as *mut libc::c_void);
    if parent_1.is_null() {
        parent_1 = ast_from_string(basis_ast, ast_name(ast_child(type_param)));
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(
            parent_1,
            ast_from_string(basis_ast, ast_name(ast_child(type_param))),
        );
    } else {
        last_sibling_1 = ast_add_sibling(
            last_sibling_1,
            ast_from_string(basis_ast, ast_name(ast_child(type_param))),
        );
    }
    node_1 = ast_from(basis_ast, TK_VAL);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_2: *mut ast_t = node_1;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_2);
    if parent_1.is_null() {
        parent_1 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_1);
    ast_inheritflags(parent_0);
    typeargs = parent;
    let mut number: *mut ast_t = type_builtin(
        opt,
        type_param,
        b"Number\0" as *const u8 as *const libc::c_char,
    );
    let mut real: *mut ast_t = type_builtin_args(
        opt,
        type_param,
        b"Real\0" as *const u8 as *const libc::c_char,
        typeargs,
    );
    ast_setid(ast_childidx(real, 3 as libc::c_int as size_t), TK_BOX);
    let mut is_real: bool = is_subtype(constraint, real, 0 as *mut errorframe_t, opt);
    let mut is_number: bool = is_subtype(constraint, number, 0 as *mut errorframe_t, opt);
    ast_free(number);
    ast_free(real);
    if !is_real || !is_number {
        return 0 as libc::c_int;
    }
    let mut uif_set: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut uif: *mut ast_t = type_builtin(opt, type_param, _str_uif_types[i as usize].name);
        let mut params: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_0: *mut ast_t = type_param;
        let mut parent_3: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
        let mut node_3: *mut ast_t = 0 as *mut ast_t;
        node_3 = ast_from(basis_ast_0, TK_TYPEPARAMS);
        if parent_3.is_null() {
            parent_3 = node_3;
        } else if last_sibling_3.is_null() {
            last_sibling_3 = ast_add(parent_3, node_3);
        } else {
            last_sibling_3 = ast_add_sibling(last_sibling_3, node_3);
        }
        let mut parent_4: *mut ast_t = node_3;
        let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
        let mut node_4: *mut ast_t = 0 as *mut ast_t;
        if parent_4.is_null() {
            parent_4 = ast_dup(type_param);
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, ast_dup(type_param));
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, ast_dup(type_param));
        }
        ast_inheritflags(parent_4);
        params = parent_3;
        let mut args: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_1: *mut ast_t = type_param;
        let mut parent_5: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
        let mut node_5: *mut ast_t = 0 as *mut ast_t;
        node_5 = ast_from(basis_ast_1, TK_TYPEARGS);
        if parent_5.is_null() {
            parent_5 = node_5;
        } else if last_sibling_5.is_null() {
            last_sibling_5 = ast_add(parent_5, node_5);
        } else {
            last_sibling_5 = ast_add_sibling(last_sibling_5, node_5);
        }
        let mut parent_6: *mut ast_t = node_5;
        let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
        let mut node_6: *mut ast_t = 0 as *mut ast_t;
        if parent_6.is_null() {
            parent_6 = uif;
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, uif);
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, uif);
        }
        ast_inheritflags(parent_6);
        args = parent_5;
        if check_constraints(0 as *mut ast_t, params, args, 0 as libc::c_int != 0, opt) {
            uif_set |= (1 as libc::c_int) << i;
        }
        ast_free(args);
        ast_free(params);
        i += 1;
    }
    if uif_set == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !((*chain).formal).is_null() && (*chain).formal != type_param {
        ast_error(
            (*opt).check.errors,
            type_param_ref,
            b"Cannot infer a literal type with multiple formal parameters\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let ref mut fresh10 = (*chain).formal;
    *fresh10 = type_param;
    let ref mut fresh11 = (*chain).name;
    *fresh11 = ast_name(ast_child(type_param));
    return uif_set | 0x10000 as libc::c_int;
}
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn uifset_union(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut chain: *mut lit_chain_t,
) -> libc::c_int {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            326 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"uifset_union\0")).as_ptr(),
        );
    };
    if ast_id(type_0) as libc::c_uint == TK_UNIONTYPE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_UNIONTYPE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            327 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"uifset_union\0")).as_ptr(),
        );
    };
    let mut uif_set: libc::c_int = 0 as libc::c_int;
    let mut p: *mut ast_t = ast_child(type_0);
    while !p.is_null() {
        let mut r: libc::c_int = uifset(opt, p, chain);
        if r == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        let mut child_valid: bool = r != 0 as libc::c_int;
        let mut child_formal: bool = r & 0x10000 as libc::c_int != 0 as libc::c_int;
        let mut others_valid: bool = uif_set & 0xffff as libc::c_int != 0 as libc::c_int;
        let mut others_formal: bool = uif_set & 0x10000 as libc::c_int != 0 as libc::c_int;
        if child_valid as libc::c_int != 0
            && others_valid as libc::c_int != 0
            && child_formal as libc::c_int != others_formal as libc::c_int
        {
            ast_error(
                (*opt).check.errors,
                type_0,
                b"could not infer literal type, ambiguous union\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        uif_set |= r;
        p = ast_sibling(p);
    }
    return uif_set;
}
#[c2rust::src_loc = "359:1"]
unsafe extern "C" fn uifset_intersect(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut chain: *mut lit_chain_t,
) -> libc::c_int {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            361 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"uifset_intersect\0"))
                .as_ptr(),
        );
    };
    if ast_id(type_0) as libc::c_uint == TK_ISECTTYPE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_ISECTTYPE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            362 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"uifset_intersect\0"))
                .as_ptr(),
        );
    };
    let mut uif_set: libc::c_int = 0xffff as libc::c_int;
    let mut constraint: libc::c_int = 0 as libc::c_int;
    let mut p: *mut ast_t = ast_child(type_0);
    while !p.is_null() {
        let mut r: libc::c_int = uifset(opt, p, chain);
        if r == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if r & 0x10000 as libc::c_int != 0 as libc::c_int {
            constraint = r;
        } else {
            uif_set |= r;
        }
        p = ast_sibling(p);
    }
    if constraint != 0 as libc::c_int {
        let mut constraint_set: libc::c_int = constraint & 0xffff as libc::c_int;
        if constraint_set & uif_set != constraint_set {
            return 0 as libc::c_int;
        }
        return constraint;
    }
    return uif_set;
}
#[c2rust::src_loc = "402:1"]
unsafe extern "C" fn uifset(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut chain: *mut lit_chain_t,
) -> libc::c_int {
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            404 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"uifset\0")).as_ptr(),
        );
    };
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int;
    }
    match ast_id(type_0) as libc::c_uint {
        149 => return uifset_union(opt, type_0, chain),
        56 => return uifset_intersect(opt, type_0, chain),
        17 => {
            let mut rhs: *mut ast_t = ast_childidx(type_0, 1 as libc::c_int as size_t);
            match ast_id(rhs) as libc::c_uint {
                151 | 187 => return uifset(opt, rhs, chain),
                _ => {
                    ast_error(
                        (*opt).check.errors,
                        rhs,
                        b"Internal error: uif type, node %d\0" as *const u8 as *const libc::c_char,
                        ast_id(rhs) as libc::c_uint,
                    );
                    if 0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                                as *const u8 as *const libc::c_char,
                            430 as libc::c_int as size_t,
                            (*::core::mem::transmute::<
                                &[u8; 7],
                                &[libc::c_char; 7],
                            >(b"uifset\0"))
                                .as_ptr(),
                        );
                    };
                    return -(1 as libc::c_int);
                }
            }
        }
        187 => {
            if (*chain).cardinality != 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            return uifset_formal_param(opt, type_0, chain);
        }
        150 => {
            if (*chain).cardinality != ast_childcount(type_0) {
                return 0 as libc::c_int;
            }
            return uifset(opt, ast_childidx(type_0, (*chain).index), (*chain).next);
        }
        151 => {
            if strcmp(
                ast_name(ast_childidx(type_0, 1 as libc::c_int as size_t)),
                b"Array\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if (*chain).cardinality != 1 as libc::c_int as libc::c_ulong {
                    return 0 as libc::c_int;
                }
                let mut type_args: *mut ast_t = ast_childidx(type_0, 2 as libc::c_int as size_t);
                if ast_childcount(type_args) == 1 as libc::c_int as libc::c_ulong {
                } else {
                    ponyint_assert_fail(
                        b"ast_childcount(type_args) == 1\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                            as *const u8 as *const libc::c_char,
                        454 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"uifset\0"))
                            .as_ptr(),
                    );
                };
                return uifset(opt, ast_child(type_args), (*chain).next);
            }
            if (*chain).cardinality != 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            return uifset_simple_type(opt, type_0);
        }
        156 | 153 => return 0 as libc::c_int,
        _ => {
            ast_error(
                (*opt).check.errors,
                type_0,
                b"Internal error: uif type, node %d\0" as *const u8 as *const libc::c_char,
                ast_id(type_0) as libc::c_uint,
            );
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                        as *const u8 as *const libc::c_char,
                    469 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"uifset\0")).as_ptr(),
                );
            };
            return -(1 as libc::c_int);
        }
    };
}
#[c2rust::src_loc = "476:1"]
unsafe extern "C" fn uif_type(
    mut opt: *mut pass_opt_t,
    mut literal: *mut ast_t,
    mut type_0: *mut ast_t,
    mut chain_head: *mut lit_chain_t,
    mut report_errors: bool,
) -> bool {
    if !chain_head.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain_head != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            479 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
        );
    };
    if (*chain_head).cardinality == 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"chain_head->cardinality == CHAIN_CARD_BASE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            480 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
        );
    };
    let ref mut fresh12 = (*chain_head).formal;
    *fresh12 = 0 as *mut ast_t;
    let mut r: libc::c_int = uifset(opt, type_0, (*chain_head).next);
    if r == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    if r == 0 as libc::c_int {
        if report_errors {
            ast_error(
                (*opt).check.errors,
                literal,
                b"could not infer literal type, no valid types found\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            497 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
        );
    };
    if r & 0x10000 as libc::c_int != 0 as libc::c_int {
        if !((*chain_head).formal).is_null() {
        } else {
            ponyint_assert_fail(
                b"chain_head->formal != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                    as *const u8 as *const libc::c_char,
                502 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
            );
        };
        if !((*chain_head).name).is_null() {
        } else {
            ponyint_assert_fail(
                b"chain_head->name != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                    as *const u8 as *const libc::c_char,
                503 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
            );
        };
        if (*chain_head).cached_uif_index < 0 as libc::c_int {
        } else {
            ponyint_assert_fail(
                b"chain_head->cached_uif_index < 0\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                    as *const u8 as *const libc::c_char,
                504 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"uif_type\0")).as_ptr(),
            );
        };
        let mut uif_type_0: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = type_0;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_TYPEPARAMREF);
        if parent.is_null() {
            parent = node;
        } else if last_sibling.is_null() {
            last_sibling = ast_add(parent, node);
        } else {
            last_sibling = ast_add_sibling(last_sibling, node);
        }
        let mut parent_0: *mut ast_t = node;
        let mut last_sibling_0: *mut ast_t = 0 as *mut ast_t;
        let mut node_0: *mut ast_t = 0 as *mut ast_t;
        ast_setdata(parent_0, (*chain_head).formal as *mut libc::c_void);
        if parent_0.is_null() {
            parent_0 = ast_from_string(basis_ast, (*chain_head).name);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from_string(basis_ast, (*chain_head).name));
        } else {
            last_sibling_0 = ast_add_sibling(
                last_sibling_0,
                ast_from_string(basis_ast, (*chain_head).name),
            );
        }
        node_0 = ast_from(basis_ast, TK_VAL);
        if parent_0.is_null() {
            parent_0 = node_0;
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, node_0);
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
        }
        let mut parent_1: *mut ast_t = node_0;
        let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
        let mut node_1: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_1);
        if parent_0.is_null() {
            parent_0 = ast_from(basis_ast, TK_NONE);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
        }
        ast_inheritflags(parent_0);
        uif_type_0 = parent;
        let ref mut fresh13 = (*chain_head).cached_type;
        *fresh13 = uif_type_0;
        (*chain_head).valid_for_float = r & 0x3fff as libc::c_int == 0 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if r == (1 as libc::c_int) << i {
            (*chain_head).valid_for_float =
                (1 as libc::c_int) << i & 0x3fff as libc::c_int == 0 as libc::c_int;
            let ref mut fresh14 = (*chain_head).cached_type;
            *fresh14 = type_builtin(opt, type_0, _str_uif_types[i as usize].name);
            let ref mut fresh15 = (*chain_head).name;
            *fresh15 = _str_uif_types[i as usize].name;
            (*chain_head).cached_uif_index = i;
            return 1 as libc::c_int != 0;
        }
        i += 1;
    }
    ast_error(
        (*opt).check.errors,
        literal,
        b"Multiple possible types for literal\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "536:1"]
unsafe extern "C" fn uif_type_from_chain(
    mut opt: *mut pass_opt_t,
    mut literal: *mut ast_t,
    mut target_type: *mut ast_t,
    mut chain: *mut lit_chain_t,
    mut require_float: bool,
    mut report_errors: bool,
) -> bool {
    if !literal.is_null() {
    } else {
        ponyint_assert_fail(
            b"literal != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            540 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uif_type_from_chain\0"))
                .as_ptr(),
        );
    };
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            541 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"uif_type_from_chain\0"))
                .as_ptr(),
        );
    };
    let mut chain_head: *mut lit_chain_t = chain;
    while (*chain_head).cardinality != 0 as libc::c_int as libc::c_ulong {
        chain_head = (*chain_head).next;
    }
    if ((*chain_head).cached_type).is_null() {
        if !uif_type(opt, literal, target_type, chain_head, report_errors) {
            return 0 as libc::c_int != 0;
        }
    }
    if require_float as libc::c_int != 0 && !(*chain_head).valid_for_float {
        if report_errors {
            ast_error(
                (*opt).check.errors,
                literal,
                b"Inferred possibly integer type %s for float literal\0" as *const u8
                    as *const libc::c_char,
                (*chain_head).name,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if ast_id(literal) as libc::c_uint == TK_INT as libc::c_int as libc::c_uint
        && (*chain_head).cached_uif_index >= 0 as libc::c_int
    {
        let mut i: libc::c_int = (*chain_head).cached_uif_index;
        if _str_uif_types[i as usize].limit.low != 0 as libc::c_int as libc::c_ulonglong
            || _str_uif_types[i as usize].limit.high != 0 as libc::c_int as libc::c_ulonglong
        {
            let mut neg_plus_one: bool = 0 as libc::c_int != 0;
            if _str_uif_types[i as usize].neg_plus_one {
                let mut parent: *mut ast_t = ast_parent(literal);
                if !parent.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"parent != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                            as *const u8 as *const libc::c_char,
                        586 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                            b"uif_type_from_chain\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut parent_type: *mut ast_t = ast_type(parent);
                if !parent_type.is_null()
                    && ast_id(parent_type) as libc::c_uint
                        == TK_OPERATORLITERAL as libc::c_int as libc::c_uint
                    && ast_child(parent) == literal
                    && (*(ast_data(parent_type) as *const lit_op_info_t)).neg_plus_one
                        as libc::c_int
                        != 0
                {
                    neg_plus_one = 1 as libc::c_int != 0;
                }
            }
            let mut actual: *mut lexint_t = ast_int(literal);
            let mut test: libc::c_int =
                lexint_cmp(actual, &(*_str_uif_types.as_ptr().offset(i as isize)).limit);
            if test > 0 as libc::c_int || !neg_plus_one && test == 0 as libc::c_int {
                ast_error(
                    (*opt).check.errors,
                    literal,
                    b"Literal value is out of range for type (%s)\0" as *const u8
                        as *const libc::c_char,
                    (*chain_head).name,
                );
                return 0 as libc::c_int != 0;
            }
        }
    }
    ast_settype(literal, (*chain_head).cached_type);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn coerce_group(
    mut astp: *mut *mut ast_t,
    mut target_type: *mut ast_t,
    mut chain: *mut lit_chain_t,
    mut cardinality: size_t,
    mut options: *mut pass_opt_t,
    mut report_errors: bool,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            617 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"coerce_group\0")).as_ptr(),
        );
    };
    let mut literal_expr: *mut ast_t = *astp;
    if !literal_expr.is_null() {
    } else {
        ponyint_assert_fail(
            b"literal_expr != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            619 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"coerce_group\0")).as_ptr(),
        );
    };
    if ast_id(literal_expr) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint
        || ast_id(literal_expr) as libc::c_uint == TK_ARRAY as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(literal_expr) == TK_TUPLE || ast_id(literal_expr) == TK_ARRAY\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            620 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"coerce_group\0")).as_ptr(),
        );
    };
    if !chain.is_null() {
    } else {
        ponyint_assert_fail(
            b"chain != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            621 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"coerce_group\0")).as_ptr(),
        );
    };
    if cardinality != 0 as libc::c_int as libc::c_ulong {
    } else {
        ponyint_assert_fail(
            b"cardinality != CHAIN_CARD_BASE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            622 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"coerce_group\0")).as_ptr(),
        );
    };
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut link: lit_chain_t = lit_chain_t {
        cardinality: 0,
        index: 0,
        formal: 0 as *mut ast_t,
        cached_type: 0 as *mut ast_t,
        name: 0 as *const libc::c_char,
        cached_uif_index: 0,
        valid_for_float: false,
        next: 0 as *mut lit_chain_t,
    };
    chain_add(chain, &mut link, cardinality);
    if ast_id(literal_expr) as libc::c_uint == TK_ARRAY as libc::c_int as libc::c_uint {
        literal_expr = ast_childidx(literal_expr, 1 as libc::c_int as size_t);
    }
    let mut p: *mut ast_t = ast_child(literal_expr);
    while !p.is_null() {
        let mut p_type: *mut ast_t = ast_type(p);
        if is_typecheck_error(p_type) {
            return 0 as libc::c_int != 0;
        }
        if is_type_literal(p_type) {
            if cardinality != 1 as libc::c_int as libc::c_ulong {
                chain_clear_cache(&mut link);
                link.index = i;
            }
            if !coerce_literal_to_type(&mut p, target_type, &mut link, options, report_errors) {
                return 0 as libc::c_int != 0;
            }
        }
        i = i.wrapping_add(1);
        p = ast_sibling(p);
    }
    chain_remove(chain);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "667:1"]
unsafe extern "C" fn coerce_control_block(
    mut astp: *mut *mut ast_t,
    mut target_type: *mut ast_t,
    mut chain: *mut lit_chain_t,
    mut opt: *mut pass_opt_t,
    mut report_errors: bool,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            670 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"coerce_control_block\0"))
                .as_ptr(),
        );
    };
    let mut literal_expr: *mut ast_t = *astp;
    if !literal_expr.is_null() {
    } else {
        ponyint_assert_fail(
            b"literal_expr != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            672 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"coerce_control_block\0"))
                .as_ptr(),
        );
    };
    let mut lit_type: *mut ast_t = ast_type(literal_expr);
    if !lit_type.is_null() {
    } else {
        ponyint_assert_fail(
            b"lit_type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            675 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"coerce_control_block\0"))
                .as_ptr(),
        );
    };
    if ast_id(lit_type) as libc::c_uint == TK_LITERAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(lit_type) == TK_LITERAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            676 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"coerce_control_block\0"))
                .as_ptr(),
        );
    };
    let mut block_type: *mut ast_t = ast_type(lit_type);
    let mut p: *mut ast_t = ast_child(lit_type);
    while !p.is_null() {
        if ast_id(p) as libc::c_uint == TK_LITERALBRANCH as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(p) == TK_LITERALBRANCH\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                    as *const u8 as *const libc::c_char,
                681 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"coerce_control_block\0",
                ))
                .as_ptr(),
            );
        };
        let mut branch: *mut ast_t = ast_data(p) as *mut ast_t;
        if !branch.is_null() {
        } else {
            ponyint_assert_fail(
                b"branch != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                    as *const u8 as *const libc::c_char,
                683 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"coerce_control_block\0",
                ))
                .as_ptr(),
            );
        };
        if !coerce_literal_to_type(&mut branch, target_type, chain, opt, report_errors) {
            ast_free_unattached(block_type);
            return 0 as libc::c_int != 0;
        }
        block_type = type_union(opt, block_type, ast_type(branch));
        p = ast_sibling(p);
    }
    if is_typecheck_error(block_type) {
        return 0 as libc::c_int != 0;
    }
    if !(ast_parent(block_type)).is_null() {
        block_type = ast_dup(block_type);
    }
    ast_settype(literal_expr, block_type);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "711:1"]
unsafe extern "C" fn coerce_literal_to_type(
    mut astp: *mut *mut ast_t,
    mut target_type: *mut ast_t,
    mut chain: *mut lit_chain_t,
    mut opt: *mut pass_opt_t,
    mut report_errors: bool,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            714 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"coerce_literal_to_type\0",
            ))
            .as_ptr(),
        );
    };
    let mut literal_expr: *mut ast_t = *astp;
    if !literal_expr.is_null() {
    } else {
        ponyint_assert_fail(
            b"literal_expr != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            716 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"coerce_literal_to_type\0",
            ))
            .as_ptr(),
        );
    };
    let mut lit_type: *mut ast_t = ast_type(literal_expr);
    if lit_type.is_null()
        || ast_id(lit_type) as libc::c_uint != TK_LITERAL as libc::c_int as libc::c_uint
            && ast_id(lit_type) as libc::c_uint != TK_OPERATORLITERAL as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if !(ast_child(lit_type)).is_null() {
        return coerce_control_block(astp, target_type, chain, opt, report_errors);
    }
    match ast_id(literal_expr) as libc::c_uint {
        178 => {
            let mut cardinality: size_t = ast_childcount(literal_expr);
            if !coerce_group(astp, target_type, chain, cardinality, opt, report_errors) {
                return 0 as libc::c_int != 0;
            }
        }
        6 => {
            return uif_type_from_chain(
                opt,
                literal_expr,
                target_type,
                chain,
                0 as libc::c_int != 0,
                report_errors,
            );
        }
        7 => {
            return uif_type_from_chain(
                opt,
                literal_expr,
                target_type,
                chain,
                1 as libc::c_int != 0,
                report_errors,
            );
        }
        179 => {
            if !coerce_group(
                astp,
                target_type,
                chain,
                1 as libc::c_int as size_t,
                opt,
                report_errors,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
        175 => {
            let mut last: *mut ast_t = ast_childlast(literal_expr);
            if !coerce_literal_to_type(&mut last, target_type, chain, opt, report_errors) {
                return 0 as libc::c_int != 0;
            }
            ast_settype(literal_expr, ast_type(last));
            return 1 as libc::c_int != 0;
        }
        177 => {
            let mut receiver: ast_ptr_t = 0 as *mut ast_t;
            let mut positional: ast_ptr_t = 0 as *mut ast_t;
            let mut named: ast_ptr_t = 0 as *mut ast_t;
            let mut question: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 5] = [
                &mut receiver,
                &mut positional,
                &mut named,
                &mut question,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                literal_expr,
                (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut arg: *mut ast_t = ast_child(positional);
            if !coerce_literal_to_type(&mut receiver, target_type, chain, opt, report_errors) {
                return 0 as libc::c_int != 0;
            }
            if !arg.is_null()
                && !coerce_literal_to_type(&mut arg, target_type, chain, opt, report_errors)
            {
                return 0 as libc::c_int != 0;
            }
            ast_settype(literal_expr, ast_type(ast_child(receiver)));
            return 1 as libc::c_int != 0;
        }
        20 | 19 => {
            let mut receiver_0: *mut ast_t = ast_child(literal_expr);
            if !coerce_literal_to_type(&mut receiver_0, target_type, chain, opt, report_errors) {
                return 0 as libc::c_int != 0;
            }
        }
        107 => {
            let mut expr: *mut ast_t = ast_childidx(literal_expr, 1 as libc::c_int as size_t);
            if !coerce_literal_to_type(&mut expr, target_type, chain, opt, report_errors) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {
            ast_error(
                (*opt).check.errors,
                literal_expr,
                b"Internal error, coerce_literal_to_type node %s\0" as *const u8
                    as *const libc::c_char,
                ast_get_print(literal_expr),
            );
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0"
                        as *const u8 as *const libc::c_char,
                    817 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"coerce_literal_to_type\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    ast_settype(literal_expr, 0 as *mut ast_t);
    return pass_expr(astp, opt) as libc::c_uint == AST_OK as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "828:1"]
pub unsafe extern "C" fn coerce_literals(
    mut astp: *mut *mut ast_t,
    mut target_type: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            830 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"coerce_literals\0"))
                .as_ptr(),
        );
    };
    let mut literal_expr: *mut ast_t = *astp;
    if !literal_expr.is_null() {
    } else {
        ponyint_assert_fail(
            b"literal_expr != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            832 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"coerce_literals\0"))
                .as_ptr(),
        );
    };
    if ast_id(literal_expr) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut lit_type: *mut ast_t = ast_type(literal_expr);
    if !lit_type.is_null()
        && ast_id(lit_type) as libc::c_uint != TK_LITERAL as libc::c_int as libc::c_uint
        && ast_id(lit_type) as libc::c_uint != TK_OPERATORLITERAL as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if target_type.is_null() && !unify(literal_expr, opt, 1 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut chain: lit_chain_t = lit_chain_t {
        cardinality: 0,
        index: 0,
        formal: 0 as *mut ast_t,
        cached_type: 0 as *mut ast_t,
        name: 0 as *const libc::c_char,
        cached_uif_index: 0,
        valid_for_float: false,
        next: 0 as *mut lit_chain_t,
    };
    chain_init_head(&mut chain);
    return coerce_literal_to_type(astp, target_type, &mut chain, opt, 1 as libc::c_int != 0);
}
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn unify(
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
    mut report_errors: bool,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            855 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"unify\0")).as_ptr(),
        );
    };
    let mut type_0: *mut ast_t = ast_type(ast);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    if !is_type_literal(type_0) {
        return 1 as libc::c_int != 0;
    }
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            864 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"unify\0")).as_ptr(),
        );
    };
    let mut non_literal: *mut ast_t = ast_type(type_0);
    if !non_literal.is_null() {
        let mut chain: lit_chain_t = lit_chain_t {
            cardinality: 0,
            index: 0,
            formal: 0 as *mut ast_t,
            cached_type: 0 as *mut ast_t,
            name: 0 as *const libc::c_char,
            cached_uif_index: 0,
            valid_for_float: false,
            next: 0 as *mut lit_chain_t,
        };
        chain_init_head(&mut chain);
        return coerce_literal_to_type(&mut ast, non_literal, &mut chain, opt, report_errors);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "882:1"]
pub unsafe extern "C" fn literal_member_access(
    mut ast: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            884 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"literal_member_access\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_TILDE as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_CHAIN as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT || ast_id(ast) == TK_TILDE || ast_id(ast) == TK_CHAIN\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            886 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"literal_member_access\0"))
                .as_ptr(),
        );
    };
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut name_node: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut receiver, &mut name_node, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if !unify(receiver, opt, 1 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut recv_type: *mut ast_t = ast_type(receiver);
    if is_typecheck_error(recv_type) {
        return 0 as libc::c_int != 0;
    }
    if ast_id(recv_type) as libc::c_uint != TK_LITERAL as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if ast_id(name_node) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(name_node) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            903 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"literal_member_access\0"))
                .as_ptr(),
        );
    };
    let mut name: *const libc::c_char = ast_name(name_node);
    let mut op: *const lit_op_info_t = lookup_literal_op(name);
    if op.is_null()
        || ast_id(ast_parent(ast)) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Cannot look up member %s on a literal\0" as *const u8 as *const libc::c_char,
            name,
        );
        return 0 as libc::c_int != 0;
    }
    let mut op_type: *mut ast_t = ast_from(ast, TK_OPERATORLITERAL);
    ast_setdata(op_type, op as *mut libc::c_void);
    ast_settype(ast, op_type);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "921:1"]
pub unsafe extern "C" fn literal_call(mut ast: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            923 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"literal_call\0")).as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_CALL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CALL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            924 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"literal_call\0")).as_ptr(),
        );
    };
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut positional_args: ast_ptr_t = 0 as *mut ast_t;
    let mut named_args: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut receiver,
        &mut positional_args,
        &mut named_args,
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
    let mut recv_type: *mut ast_t = ast_type(receiver);
    if is_typecheck_error(recv_type) {
        return 0 as libc::c_int != 0;
    }
    if ast_id(recv_type) as libc::c_uint == TK_LITERAL as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Cannot call a literal\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if ast_id(recv_type) as libc::c_uint != TK_OPERATORLITERAL as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut op: *const lit_op_info_t = ast_data(recv_type) as *const lit_op_info_t;
    if !op.is_null() {
    } else {
        ponyint_assert_fail(
            b"op != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            943 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"literal_call\0")).as_ptr(),
        );
    };
    if ast_childcount(named_args) != 0 as libc::c_int as libc::c_ulong {
        ast_error(
            (*opt).check.errors,
            named_args,
            b"Cannot use named arguments with literal operator\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut arg: *mut ast_t = ast_child(positional_args);
    if !arg.is_null() {
        if !unify(arg, opt, 1 as libc::c_int != 0) {
            return 0 as libc::c_int != 0;
        }
        let mut arg_type: *mut ast_t = ast_type(arg);
        if is_typecheck_error(arg_type) {
            return 0 as libc::c_int != 0;
        }
        if ast_id(arg_type) as libc::c_uint != TK_LITERAL as libc::c_int as libc::c_uint {
            return coerce_literals(&mut receiver, arg_type, opt);
        }
        if !(*op).can_propogate_literal {
            ast_error(
                (*opt).check.errors,
                ast,
                b"Cannot infer operand type\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    let mut arg_count: size_t = ast_childcount(positional_args);
    if (*op).arg_count != arg_count {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Invalid number of arguments to literal operator\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    make_literal_type(ast);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "986:1"]
pub unsafe extern "C" fn literal_is(mut ast: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            988 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"literal_is\0")).as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_IS as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_ISNT as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_IS || ast_id(ast) == TK_ISNT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            989 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"literal_is\0")).as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut l_type: *mut ast_t = ast_type(left);
    let mut r_type: *mut ast_t = ast_type(right);
    if is_typecheck_error(l_type) as libc::c_int != 0
        || is_typecheck_error(r_type) as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    if !is_type_literal(l_type) && !is_type_literal(r_type) {
        return 1 as libc::c_int != 0;
    }
    if is_type_literal(l_type) as libc::c_int != 0 && !is_type_literal(r_type) {
        return coerce_literals(&mut left, r_type, opt);
    }
    if !is_type_literal(l_type) && is_type_literal(r_type) as libc::c_int != 0 {
        return coerce_literals(&mut right, l_type, opt);
    }
    if is_type_literal(l_type) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"is_type_literal(l_type)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            1016 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"literal_is\0")).as_ptr(),
        );
    };
    if is_type_literal(r_type) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"is_type_literal(r_type)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.c\0" as *const u8
                as *const libc::c_char,
            1017 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"literal_is\0")).as_ptr(),
        );
    };
    ast_error(
        (*opt).check.errors,
        ast,
        b"Cannot infer type of operands\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1023:1"]
pub unsafe extern "C" fn literal_unify_control(mut ast: *mut ast_t, mut opt: *mut pass_opt_t) {
    unify(ast, opt, 0 as libc::c_int != 0);
}
