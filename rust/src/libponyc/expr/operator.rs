use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
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
        #[c2rust::src_loc = "63:1"]
        pub fn errors_get_count(errors: *mut errors_t) -> size_t;
        #[c2rust::src_loc = "115:1"]
        pub fn errorframe_append(first: *mut errorframe_t, second: *mut errorframe_t);
        #[c2rust::src_loc = "122:1"]
        pub fn errorframe_report(frame: *mut errorframe_t, errors: *mut errors_t);
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::error_h::{errorframe_t, errors_t};
    use super::symtab_h::ast_t;
    use super::token_h::{token_id};
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: uint32_t) -> libc::c_int;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "101:1"]
        pub fn ast_settype(ast: *mut ast_t, type_0: *mut ast_t);
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
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
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "143:1"]
        pub fn ast_replace(prev: *mut *mut ast_t, next: *mut ast_t);
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "156:1"]
        pub fn ast_print_type(type_0: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "161:1"]
        pub fn ast_error_continue(
            errors: *mut errors_t,
            ast: *mut ast_t,
            fmt: *const libc::c_char,
            _: ...
        );
        #[c2rust::src_loc = "163:1"]
        pub fn ast_error_frame(
            frame: *mut errorframe_t,
            ast: *mut ast_t,
            fmt: *const libc::c_char,
            _: ...
        );
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "390:1"]
        pub fn ast_passes_subtree(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:16"]
pub mod cap_h {
    #[c2rust::src_loc = "76:9"]
    pub type direction = libc::c_uint;
    #[c2rust::src_loc = "78:3"]
    pub const EXTRACT: direction = 1;
    #[c2rust::src_loc = "77:3"]
    pub const WRITE: direction = 0;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/matchtype.h:15"]
pub mod matchtype_h {
    #[c2rust::src_loc = "16:3"]
    pub const MATCHTYPE_DENY_NODESC: matchtype_t = 3;
    #[c2rust::src_loc = "11:9"]
    pub type matchtype_t = libc::c_uint;
    #[c2rust::src_loc = "15:3"]
    pub const MATCHTYPE_DENY_CAP: matchtype_t = 2;
    #[c2rust::src_loc = "14:3"]
    pub const MATCHTYPE_REJECT: matchtype_t = 1;
    #[c2rust::src_loc = "13:3"]
    pub const MATCHTYPE_ACCEPT: matchtype_t = 0;
    use super::error_h::errorframe_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn is_matchtype(
            operand: *mut ast_t,
            pattern: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> matchtype_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:2"]
pub mod literal_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn coerce_literals(
            astp: *mut *mut ast_t,
            target_type: *mut ast_t,
            options: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "40:1"]
        pub fn literal_is(ast: *mut ast_t, options: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.h:6"]
pub mod call_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn check_auto_recover_newref(dest_type: *mut ast_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:9"]
pub mod lexer_h {
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn lexer_print(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:10"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn is_typecheck_error(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:12"]
pub mod package_h {
    use super::frame_h::typecheck_t;
    extern "C" {
        #[c2rust::src_loc = "143:1"]
        pub fn package_hygienic_id(t: *mut typecheck_t) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:13"]
pub mod alias_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn alias(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "13:1"]
        pub fn consume_type(
            type_0: *mut ast_t,
            cap: token_id,
            keep_double_ephemeral: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "15:1"]
        pub fn recover_type(type_0: *mut ast_t, cap: token_id) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:14"]
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
        #[c2rust::src_loc = "50:1"]
        pub fn type_union(
            opt: *mut pass_opt_t,
            l_type: *mut ast_t,
            r_type: *mut ast_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "55:1"]
        pub fn type_isect(
            opt: *mut pass_opt_t,
            l_type: *mut ast_t,
            r_type: *mut ast_t,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/safeto.h:16"]
pub mod safeto_h {
    use super::cap_h::{direction};
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn safe_to_move(ast: *mut ast_t, type_0: *mut ast_t, direction: direction) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:17"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:18"]
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
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
use self::alias_h::{alias, consume_type, recover_type};
use self::assemble_h::{type_builtin, type_isect, type_union};
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_checkflag, ast_child, ast_childcount, ast_childidx,
    ast_childlast, ast_error, ast_error_continue, ast_error_frame, ast_free, ast_free_unattached,
    ast_from, ast_from_string, ast_get_children, ast_id, ast_inheritflags, ast_nearest, ast_pop,
    ast_print_type, ast_ptr_t, ast_replace, ast_scope, ast_settype, ast_sibling, ast_type,
    C2RustUnnamed, AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
use self::call_h::check_auto_recover_newref;
pub use self::cap_h::{cap_fetch, direction, EXTRACT, WRITE};
pub use self::error_h::{
    errorframe_append, errorframe_report, errorframe_t, errormsg_t, errors_get_count, errors_t,
};
use self::expr_h::is_typecheck_error;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::lexer_h::lexer_print;
use self::literal_h::{coerce_literals, literal_is};
pub use self::matchtype_h::{
    is_matchtype, matchtype_t, MATCHTYPE_ACCEPT, MATCHTYPE_DENY_CAP, MATCHTYPE_DENY_NODESC,
    MATCHTYPE_REJECT,
};
use self::package_h::package_hygienic_id;
pub use self::pass_h::{
    ast_passes_subtree, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL,
    PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::safeto_h::safe_to_move;

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
#[c2rust::src_loc = "26:16"]
pub struct infer_path_t {
    pub index: size_t,
    pub next: *mut infer_path_t,
    pub root: *mut infer_path_t,
}
#[c2rust::src_loc = "37:3"]
pub const INFER_ERROR: infer_ret_t = 2;
#[c2rust::src_loc = "33:9"]
pub type infer_ret_t = libc::c_uint;
#[c2rust::src_loc = "36:3"]
pub const INFER_NOP: infer_ret_t = 1;
#[c2rust::src_loc = "35:3"]
pub const INFER_OK: infer_ret_t = 0;
#[no_mangle]
#[c2rust::src_loc = "20:1"]
pub unsafe extern "C" fn expr_identity(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    ast_settype(
        ast,
        type_builtin(opt, ast, b"Bool\0" as *const u8 as *const libc::c_char),
    );
    literal_is(ast, opt)
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn find_infer_type(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut path: *mut infer_path_t,
) -> *mut ast_t {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            42 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"find_infer_type\0"))
                .as_ptr(),
        );
    };
    match ast_id(type_0) as libc::c_uint {
        150 => {
            if path.is_null() {
                return type_0;
            }
            if (*path).index >= ast_childcount(type_0) {
                return 0 as *mut ast_t;
            }
            find_infer_type(opt, ast_childidx(type_0, (*path).index), (*path).next)
        }
        149 => {
            let mut u_type: *mut ast_t = 0 as *mut ast_t;
            let mut p: *mut ast_t = ast_child(type_0);
            while !p.is_null() {
                let mut t: *mut ast_t = find_infer_type(opt, p, path);
                if t.is_null() {
                    ast_free_unattached(u_type);
                    return 0 as *mut ast_t;
                }
                u_type = type_union(opt, u_type, t);
                p = ast_sibling(p);
            }
            u_type
        }
        56 => {
            let mut i_type: *mut ast_t = 0 as *mut ast_t;
            let mut p_0: *mut ast_t = ast_child(type_0);
            while !p_0.is_null() {
                let mut t_0: *mut ast_t = find_infer_type(opt, p_0, path);
                if t_0.is_null() {
                    ast_free_unattached(i_type);
                    return 0 as *mut ast_t;
                }
                i_type = type_isect(opt, i_type, t_0);
                p_0 = ast_sibling(p_0);
            }
            i_type
        }
        _ => {
            if !path.is_null() {
                return 0 as *mut ast_t;
            }
            type_0
        }
    }
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn infer_local_inner(
    mut opt: *mut pass_opt_t,
    mut left: *mut ast_t,
    mut r_type: *mut ast_t,
    mut path: *mut infer_path_t,
) -> infer_ret_t {
    if !left.is_null() {
    } else {
        ponyint_assert_fail(
            b"left != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            111 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"infer_local_inner\0"))
                .as_ptr(),
        );
    };
    if !r_type.is_null() {
    } else {
        ponyint_assert_fail(
            b"r_type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            112 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"infer_local_inner\0"))
                .as_ptr(),
        );
    };
    if !path.is_null() {
    } else {
        ponyint_assert_fail(
            b"path != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            113 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"infer_local_inner\0"))
                .as_ptr(),
        );
    };
    if !((*path).root).is_null() {
    } else {
        ponyint_assert_fail(
            b"path->root != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            114 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"infer_local_inner\0"))
                .as_ptr(),
        );
    };
    let mut ret_val: infer_ret_t = INFER_NOP;
    match ast_id(left) as libc::c_uint {
        175 => {
            if ast_childcount(left) == 1 as libc::c_int as libc::c_ulong {
            } else {
                ponyint_assert_fail(
                    b"ast_childcount(left) == 1\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                        as *const u8 as *const libc::c_char,
                    122 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"infer_local_inner\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut r: infer_ret_t = infer_local_inner(opt, ast_child(left), r_type, path);
            if r as libc::c_uint == INFER_OK as libc::c_int as libc::c_uint {
                ast_settype(left, ast_type(ast_child(left)));
            }
            r
        }
        178 => {
            let mut path_node: infer_path_t = {
                let mut init = infer_path_t {
                    index: 0 as libc::c_int as size_t,
                    next: 0 as *mut infer_path_t,
                    root: (*path).root,
                };
                init
            };
            let ref mut fresh0 = (*path).next;
            *fresh0 = &mut path_node;
            let mut p: *mut ast_t = ast_child(left);
            while !p.is_null() {
                let mut r_0: infer_ret_t = infer_local_inner(opt, p, r_type, &mut path_node);
                if r_0 as libc::c_uint == INFER_ERROR as libc::c_int as libc::c_uint {
                    return INFER_ERROR;
                }
                if r_0 as libc::c_uint == INFER_OK as libc::c_int as libc::c_uint {
                    let mut old_ele: *mut ast_t = ast_childidx(ast_type(left), path_node.index);
                    ast_replace(&mut old_ele, ast_type(p));
                    ret_val = INFER_OK;
                }
                path_node.index = (path_node.index).wrapping_add(1);
                p = ast_sibling(p);
            }
            let ref mut fresh1 = (*path).next;
            *fresh1 = 0 as *mut infer_path_t;
            return ret_val;
        }
        84 | 85 => {
            let mut var_type: *mut ast_t = ast_type(left);
            if !var_type.is_null() {
            } else {
                ponyint_assert_fail(
                    b"var_type != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                        as *const u8 as *const libc::c_char,
                    164 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"infer_local_inner\0",
                    ))
                    .as_ptr(),
                );
            };
            if ast_id(var_type) as libc::c_uint != TK_INFERTYPE as libc::c_int as libc::c_uint {
                return INFER_NOP;
            }
            let mut infer_type: *mut ast_t = find_infer_type(opt, r_type, (*(*path).root).next);
            if infer_type.is_null() {
                ast_error(
                    (*opt).check.errors,
                    left,
                    b"could not infer type of local\0" as *const u8 as *const libc::c_char,
                );
                ast_settype(left, ast_from(left, TK_ERRORTYPE));
                return INFER_ERROR;
            }
            let mut a_type: *mut ast_t = alias(infer_type);
            ast_settype(left, a_type);
            ast_settype(ast_child(left), a_type);
            let mut speced_type: *mut ast_t = ast_childidx(left, 1 as libc::c_int as size_t);
            if ast_id(speced_type) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(speced_type) == TK_NONE\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                        as *const u8 as *const libc::c_char,
                    186 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"infer_local_inner\0",
                    ))
                    .as_ptr(),
                );
            };
            ast_replace(&mut speced_type, a_type);
            ast_free_unattached(infer_type);
            INFER_OK
        }
        _ => INFER_NOP,
    }
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn infer_locals(
    mut opt: *mut pass_opt_t,
    mut left: *mut ast_t,
    mut r_type: *mut ast_t,
) -> bool {
    let mut path_root: infer_path_t = {
        let mut init = infer_path_t {
            index: 0 as libc::c_int as size_t,
            next: 0 as *mut infer_path_t,
            root: 0 as *mut infer_path_t,
        };
        init
    };
    path_root.root = &mut path_root;
    if infer_local_inner(opt, left, r_type, &mut path_root) as libc::c_uint
        == INFER_ERROR as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    if (path_root.next).is_null() {
    } else {
        ponyint_assert_fail(
            b"path_root.next == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            207 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"infer_locals\0")).as_ptr(),
        );
    };
    path_root.root = &mut path_root as *mut infer_path_t;
    if !(path_root.root).is_null() {
    } else {
        ponyint_assert_fail(
            b"path_root.root = &path_root\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            208 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"infer_locals\0")).as_ptr(),
        );
    };
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn is_expr_constructor(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            214 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"is_expr_constructor\0"))
                .as_ptr(),
        );
    };
    match ast_id(ast) as libc::c_uint {
        177 => {
            return ast_id(ast_child(ast)) as libc::c_uint
                == TK_NEWREF as libc::c_int as libc::c_uint;
        }
        175 => return is_expr_constructor(ast_childlast(ast)),
        108 | 116 | 118 => {
            let mut body: *mut ast_t = ast_childidx(ast, 1 as libc::c_int as size_t);
            let mut else_expr: *mut ast_t = ast_childidx(ast, 2 as libc::c_int as size_t);
            return is_expr_constructor(body) as libc::c_int != 0
                && is_expr_constructor(else_expr) as libc::c_int != 0;
        }
        124 => {
            let mut body_0: *mut ast_t = ast_childidx(ast, 0 as libc::c_int as size_t);
            let mut else_expr_0: *mut ast_t = ast_childidx(ast, 1 as libc::c_int as size_t);
            return is_expr_constructor(body_0) as libc::c_int != 0
                && is_expr_constructor(else_expr_0) as libc::c_int != 0;
        }
        206 => {
            let mut body_1: *mut ast_t = ast_childidx(ast, 0 as libc::c_int as size_t);
            return is_expr_constructor(body_1);
        }
        122 => {
            let mut cases: *mut ast_t = ast_childidx(ast, 1 as libc::c_int as size_t);
            let mut else_expr_1: *mut ast_t = ast_childidx(ast, 2 as libc::c_int as size_t);
            let mut the_case: *mut ast_t = ast_child(cases);
            while !the_case.is_null() {
                if !is_expr_constructor(ast_childidx(the_case, 2 as libc::c_int as size_t)) {
                    return 0 as libc::c_int != 0;
                }
                the_case = ast_sibling(the_case);
            }
            return is_expr_constructor(else_expr_1);
        }
        107 => return is_expr_constructor(ast_childidx(ast, 1 as libc::c_int as size_t)),
        _ => return 0 as libc::c_int != 0,
    };
}
#[c2rust::src_loc = "261:1"]
unsafe extern "C" fn tuple_contains_embed(mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_TUPLE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"tuple_contains_embed\0"))
                .as_ptr(),
        );
    };
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        if ast_id(child) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(child) == TK_SEQ\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                    as *const u8 as *const libc::c_char,
                267 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"tuple_contains_embed\0",
                ))
                .as_ptr(),
            );
        };
        let mut member: *mut ast_t = ast_childlast(child);
        if ast_id(member) as libc::c_uint == TK_EMBEDREF as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        } else {
            if ast_id(member) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
                if tuple_contains_embed(member) {
                    return 1 as libc::c_int != 0;
                }
            }
        }
        child = ast_sibling(child);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "281:1"]
unsafe extern "C" fn check_embed_construction(
    mut opt: *mut pass_opt_t,
    mut left: *mut ast_t,
    mut right: *mut ast_t,
) -> bool {
    let mut result: bool = 1 as libc::c_int != 0;
    if ast_id(left) as libc::c_uint == TK_EMBEDREF as libc::c_int as libc::c_uint {
        if !is_expr_constructor(right) {
            ast_error(
                (*opt).check.errors,
                left,
                b"an embedded field must be assigned using a constructor\0" as *const u8
                    as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                right,
                b"the assigned expression is here\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    } else if ast_id(left) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
        if ast_id(right) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
            let mut l_child: *mut ast_t = ast_child(left);
            let mut r_child: *mut ast_t = ast_child(right);
            while !l_child.is_null() {
                if ast_id(l_child) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
                    && ast_id(r_child) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
                {
                } else {
                    ponyint_assert_fail(
                        b"(ast_id(l_child) == TK_SEQ) && (ast_id(r_child) == TK_SEQ)\0" as *const u8
                            as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                            as *const u8 as *const libc::c_char,
                        301 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                            b"check_embed_construction\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut l_member: *mut ast_t = ast_childlast(l_child);
                let mut r_member: *mut ast_t = ast_childlast(r_child);
                if !check_embed_construction(opt, l_member, r_member) {
                    result = 0 as libc::c_int != 0;
                }
                l_child = ast_sibling(l_child);
                r_child = ast_sibling(r_child);
            }
            if r_child.is_null() {
            } else {
                ponyint_assert_fail(
                    b"r_child == NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                        as *const u8 as *const libc::c_char,
                    310 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"check_embed_construction\0",
                    ))
                    .as_ptr(),
                );
            };
        } else if tuple_contains_embed(left) {
            ast_error(
                (*opt).check.errors,
                left,
                b"an embedded field must be assigned using a constructor\0" as *const u8
                    as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                right,
                b"the assigned expression isn't a tuple literal\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    result
}
#[no_mangle]
#[c2rust::src_loc = "321:1"]
pub unsafe extern "C" fn expr_assign(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_ASSIGN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ASSIGN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            325 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_assign\0")).as_ptr(),
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
    if l_type.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"left side must be something that can be assigned to\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if !coerce_literals(&mut right, l_type, opt) {
        return 0 as libc::c_int != 0;
    }
    let mut r_type: *mut ast_t = ast_type(right);
    if is_typecheck_error(r_type) {
        if errors_get_count((*opt).check.errors) == 0 as libc::c_int as libc::c_ulong {
            ast_error(
                (*opt).check.errors,
                right,
                b"right side must be something that can be assigned\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if ast_checkflag(right, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) != 0 {
        ast_error(
            (*opt).check.errors,
            ast,
            b"the right hand side jumps away without a value\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if !infer_locals(opt, left, r_type) {
        return 0 as libc::c_int != 0;
    }
    l_type = ast_type(left);
    let mut fl_type: *mut ast_t = l_type;
    match ast_id(left) as libc::c_uint {
        192 | 193 => {
            let mut origin: ast_ptr_t = 0 as *mut ast_t;
            let mut field: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut origin, &mut field, 0 as *mut *mut ast_t];
            ast_get_children(
                left,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            fl_type = ast_type(field);
        }
        _ => {}
    }
    let mut wl_type: *mut ast_t = consume_type(fl_type, TK_NONE, 0 as libc::c_int != 0);
    if check_auto_recover_newref(fl_type, right) {
        let mut left_cap: token_id = ast_id(cap_fetch(wl_type));
        let mut recovered_left_type: *mut ast_t = recover_type(r_type, left_cap);
        if !recovered_left_type.is_null() {
            r_type = recovered_left_type;
        }
    }
    let mut info: errorframe_t = 0 as errorframe_t;
    let mut frame: errorframe_t = 0 as errorframe_t;
    if wl_type.is_null() {
        ast_error_frame(
            &mut frame as *mut errorframe_t,
            ast,
            b"Invalid type for field of assignment: %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(fl_type),
        );
        if ast_checkflag(
            ast_type(right),
            AST_FLAG_INCOMPLETE as libc::c_int as uint32_t,
        ) != 0
        {
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                right,
                b"this might be possible if all fields were already defined\0" as *const u8
                    as *const libc::c_char,
            );
        }
        errorframe_append(&mut frame, &mut info);
        errorframe_report(&mut frame, (*opt).check.errors);
        return 0 as libc::c_int != 0;
    } else {
        if !is_subtype(r_type, wl_type, &mut info, opt) {
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                ast,
                b"right side must be a subtype of left side\0" as *const u8 as *const libc::c_char,
            );
            if ast_checkflag(
                ast_type(right),
                AST_FLAG_INCOMPLETE as libc::c_int as uint32_t,
            ) != 0
            {
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    right,
                    b"this might be possible if all fields were already defined\0" as *const u8
                        as *const libc::c_char,
                );
            }
            errorframe_append(&mut frame, &mut info);
            errorframe_report(&mut frame, (*opt).check.errors);
            ast_free_unattached(wl_type);
            return 0 as libc::c_int != 0;
        }
    }
    if ast_id(left) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint
        && ast_id(r_type) as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint
    {
        match ast_id(r_type) as libc::c_uint {
            149 => {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't destructure a union using assignment, use pattern matching instead\0"
                        as *const u8 as *const libc::c_char,
                );
                ast_error_continue(
                    (*opt).check.errors,
                    right,
                    b"inferred type of expression: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(r_type),
                );
            }
            56 => {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't destructure an intersection using assignment, use pattern matching instead\0"
                        as *const u8 as *const libc::c_char,
                );
                ast_error_continue(
                    (*opt).check.errors,
                    right,
                    b"inferred type of expression: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(r_type),
                );
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                            as *const u8 as *const libc::c_char,
                        438 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                            b"expr_assign\0",
                        ))
                        .as_ptr(),
                    );
                };
            }
        }
        ast_free_unattached(wl_type);
        return 0 as libc::c_int != 0;
    }
    let mut ok_safe: bool = safe_to_move(left, r_type, WRITE);
    if !ok_safe {
        if ast_id(left) as libc::c_uint == TK_FVARREF as libc::c_int as libc::c_uint
            && !(ast_child(left)).is_null()
            && ast_id(ast_child(left)) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint
        {
            let mut fn_0: *mut ast_t = ast_nearest(left, TK_FUN);
            if !fn_0.is_null() {
                let mut iso: *mut ast_t = ast_child(fn_0);
                if !iso.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"iso != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0"
                            as *const u8 as *const libc::c_char,
                        459 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                            b"expr_assign\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut iso_id: token_id = ast_id(iso);
                if iso_id as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
                    || iso_id as libc::c_uint == TK_VAL as libc::c_int as libc::c_uint
                    || iso_id as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint
                {
                    ast_error(
                        (*opt).check.errors,
                        ast,
                        b"cannot write to a field in a %s function. If you are trying to change state in a function use fun ref\0"
                            as *const u8 as *const libc::c_char,
                        lexer_print(iso_id),
                    );
                    ast_free_unattached(wl_type);
                    return 0 as libc::c_int != 0;
                }
            }
        }
        ast_error(
            (*opt).check.errors,
            ast,
            b"not safe to write right side to left side\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            wl_type,
            b"right side type: %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(wl_type),
        );
        ast_free_unattached(wl_type);
        return 0 as libc::c_int != 0;
    }
    ast_free_unattached(wl_type);
    if !check_embed_construction(opt, left, right) {
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, consume_type(l_type, TK_NONE, 0 as libc::c_int != 0));
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn add_as_type(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut expr: *mut ast_t,
    mut type_0: *mut ast_t,
    mut pattern: *mut ast_t,
    mut body: *mut ast_t,
) -> bool {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            494 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"add_as_type\0")).as_ptr(),
        );
    };
    match ast_id(type_0) as libc::c_uint {
        150 => {
            let mut tuple_pattern: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast: *mut ast_t = pattern;
            let mut parent: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
            let mut node: *mut ast_t = 0 as *mut ast_t;
            node = ast_from(basis_ast, TK_SEQ);
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
            node_0 = ast_from(basis_ast, TK_TUPLE);
            if parent_0.is_null() {
                parent_0 = node_0;
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(parent_0, node_0);
            } else {
                last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
            }
            let mut parent_1: *mut ast_t = node_0;
            let mut _last_sibling_1: *mut ast_t = 0 as *mut ast_t;
            let mut _node_1: *mut ast_t = 0 as *mut ast_t;
            ast_inheritflags(parent_1);
            ast_inheritflags(parent_0);
            tuple_pattern = parent;
            ast_append(pattern, tuple_pattern);
            let mut pattern_child: *mut ast_t = ast_child(tuple_pattern);
            let mut tuple_body: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_0: *mut ast_t = body;
            let mut parent_2: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
            let mut node_2: *mut ast_t = 0 as *mut ast_t;
            node_2 = ast_from(basis_ast_0, TK_SEQ);
            if parent_2.is_null() {
                parent_2 = node_2;
            } else if last_sibling_2.is_null() {
                last_sibling_2 = ast_add(parent_2, node_2);
            } else {
                last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
            }
            let mut parent_3: *mut ast_t = node_2;
            let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
            let mut node_3: *mut ast_t = 0 as *mut ast_t;
            node_3 = ast_from(basis_ast_0, TK_TUPLE);
            if parent_3.is_null() {
                parent_3 = node_3;
            } else if last_sibling_3.is_null() {
                last_sibling_3 = ast_add(parent_3, node_3);
            } else {
                last_sibling_3 = ast_add_sibling(last_sibling_3, node_3);
            }
            let mut parent_4: *mut ast_t = node_3;
            let mut _last_sibling_4: *mut ast_t = 0 as *mut ast_t;
            let mut _node_4: *mut ast_t = 0 as *mut ast_t;
            ast_inheritflags(parent_4);
            ast_inheritflags(parent_3);
            tuple_body = parent_2;
            let mut body_child: *mut ast_t = ast_child(tuple_body);
            let mut p: *mut ast_t = ast_child(type_0);
            while !p.is_null() {
                if !add_as_type(opt, ast, expr, p, pattern_child, body_child) {
                    return 0 as libc::c_int != 0;
                }
                p = ast_sibling(p);
            }
            if ast_childcount(body_child) == 1 as libc::c_int as libc::c_ulong {
                let mut t: *mut ast_t = ast_pop(body_child);
                ast_free(tuple_body);
                tuple_body = t;
            }
            ast_append(body, tuple_body);
        }
        156 => {
            let mut dontcare: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_1: *mut ast_t = pattern;
            let mut parent_5: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
            let mut node_5: *mut ast_t = 0 as *mut ast_t;
            node_5 = ast_from(basis_ast_1, TK_SEQ);
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
            node_6 = ast_from(basis_ast_1, TK_REFERENCE);
            if parent_6.is_null() {
                parent_6 = node_6;
            } else if last_sibling_6.is_null() {
                last_sibling_6 = ast_add(parent_6, node_6);
            } else {
                last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
            }
            let mut parent_7: *mut ast_t = node_6;
            let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
            let mut _node_7: *mut ast_t = 0 as *mut ast_t;
            if parent_7.is_null() {
                parent_7 = ast_from_string(basis_ast_1, b"_\0" as *const u8 as *const libc::c_char);
            } else if last_sibling_7.is_null() {
                last_sibling_7 = ast_add(
                    parent_7,
                    ast_from_string(basis_ast_1, b"_\0" as *const u8 as *const libc::c_char),
                );
            } else {
                last_sibling_7 = ast_add_sibling(
                    last_sibling_7,
                    ast_from_string(basis_ast_1, b"_\0" as *const u8 as *const libc::c_char),
                );
            }
            ast_inheritflags(parent_7);
            ast_inheritflags(parent_6);
            dontcare = parent_5;
            ast_append(pattern, dontcare);
        }
        _ => {
            let mut name: *const libc::c_char = package_hygienic_id(&mut (*opt).check);
            let mut expr_type: *mut ast_t = ast_type(expr);
            let mut info: errorframe_t = 0 as errorframe_t;
            let mut is_match: matchtype_t = is_matchtype(expr_type, type_0, &mut info, opt);
            if is_match as libc::c_uint == MATCHTYPE_DENY_CAP as libc::c_int as libc::c_uint {
                let mut frame: errorframe_t = 0 as errorframe_t;
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    ast,
                    b"this capture violates capabilities\0" as *const u8 as *const libc::c_char,
                );
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    type_0,
                    b"match type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(type_0),
                );
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    expr,
                    b"pattern type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(expr_type),
                );
                errorframe_append(&mut frame, &mut info);
                errorframe_report(&mut frame, (*opt).check.errors);
                return 0 as libc::c_int != 0;
            } else {
                if is_match as libc::c_uint == MATCHTYPE_DENY_NODESC as libc::c_int as libc::c_uint
                {
                    let mut frame_0: errorframe_t = 0 as errorframe_t;
                    ast_error_frame(
                        &mut frame_0 as *mut errorframe_t,
                        ast,
                        b"matching variable of type %s with %s is not possible, since a struct lacks a type descriptor\0"
                            as *const u8 as *const libc::c_char,
                        ast_print_type(expr_type),
                        ast_print_type(type_0),
                    );
                    ast_error_frame(
                        &mut frame_0 as *mut errorframe_t,
                        type_0,
                        b"match type: %s\0" as *const u8 as *const libc::c_char,
                        ast_print_type(type_0),
                    );
                    ast_error_frame(
                        &mut frame_0 as *mut errorframe_t,
                        expr,
                        b"a struct cannot be part of a union type. pattern type: %s\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(expr_type),
                    );
                    errorframe_append(&mut frame_0, &mut info);
                    errorframe_report(&mut frame_0, (*opt).check.errors);
                    return 0 as libc::c_int != 0;
                }
            }
            let mut a_type: *mut ast_t = alias(type_0);
            let mut pattern_elem: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_2: *mut ast_t = pattern;
            let mut parent_8: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
            let mut node_8: *mut ast_t = 0 as *mut ast_t;
            node_8 = ast_from(basis_ast_2, TK_SEQ);
            if parent_8.is_null() {
                parent_8 = node_8;
            } else if last_sibling_8.is_null() {
                last_sibling_8 = ast_add(parent_8, node_8);
            } else {
                last_sibling_8 = ast_add_sibling(last_sibling_8, node_8);
            }
            let mut parent_9: *mut ast_t = node_8;
            let mut last_sibling_9: *mut ast_t = 0 as *mut ast_t;
            let mut node_9: *mut ast_t = 0 as *mut ast_t;
            node_9 = ast_from(basis_ast_2, TK_LET);
            if parent_9.is_null() {
                parent_9 = node_9;
            } else if last_sibling_9.is_null() {
                last_sibling_9 = ast_add(parent_9, node_9);
            } else {
                last_sibling_9 = ast_add_sibling(last_sibling_9, node_9);
            }
            let mut parent_10: *mut ast_t = node_9;
            let mut last_sibling_10: *mut ast_t = 0 as *mut ast_t;
            let mut _node_10: *mut ast_t = 0 as *mut ast_t;
            if parent_10.is_null() {
                parent_10 = ast_from_string(basis_ast_2, name);
            } else if last_sibling_10.is_null() {
                last_sibling_10 = ast_add(parent_10, ast_from_string(basis_ast_2, name));
            } else {
                last_sibling_10 =
                    ast_add_sibling(last_sibling_10, ast_from_string(basis_ast_2, name));
            }
            if parent_10.is_null() {
                parent_10 = a_type;
            } else if last_sibling_10.is_null() {
                last_sibling_10 = ast_add(parent_10, a_type);
            } else {
                last_sibling_10 = ast_add_sibling(last_sibling_10, a_type);
            }
            ast_inheritflags(parent_10);
            ast_inheritflags(parent_9);
            pattern_elem = parent_8;
            let mut body_elem: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_3: *mut ast_t = body;
            let mut parent_11: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_11: *mut ast_t = 0 as *mut ast_t;
            let mut node_11: *mut ast_t = 0 as *mut ast_t;
            node_11 = ast_from(basis_ast_3, TK_SEQ);
            if parent_11.is_null() {
                parent_11 = node_11;
            } else if last_sibling_11.is_null() {
                last_sibling_11 = ast_add(parent_11, node_11);
            } else {
                last_sibling_11 = ast_add_sibling(last_sibling_11, node_11);
            }
            let mut parent_12: *mut ast_t = node_11;
            let mut last_sibling_12: *mut ast_t = 0 as *mut ast_t;
            let mut node_12: *mut ast_t = 0 as *mut ast_t;
            node_12 = ast_from(basis_ast_3, TK_CONSUME);
            if parent_12.is_null() {
                parent_12 = node_12;
            } else if last_sibling_12.is_null() {
                last_sibling_12 = ast_add(parent_12, node_12);
            } else {
                last_sibling_12 = ast_add_sibling(last_sibling_12, node_12);
            }
            let mut parent_13: *mut ast_t = node_12;
            let mut last_sibling_13: *mut ast_t = 0 as *mut ast_t;
            let mut node_13: *mut ast_t = 0 as *mut ast_t;
            node_13 = ast_from(basis_ast_3, TK_ALIASED);
            if parent_13.is_null() {
                parent_13 = node_13;
            } else if last_sibling_13.is_null() {
                last_sibling_13 = ast_add(parent_13, node_13);
            } else {
                last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
            }
            let mut parent_14: *mut ast_t = node_13;
            let mut _last_sibling_14: *mut ast_t = 0 as *mut ast_t;
            let mut _node_14: *mut ast_t = 0 as *mut ast_t;
            ast_inheritflags(parent_14);
            node_13 = ast_from(basis_ast_3, TK_REFERENCE);
            if parent_13.is_null() {
                parent_13 = node_13;
            } else if last_sibling_13.is_null() {
                last_sibling_13 = ast_add(parent_13, node_13);
            } else {
                last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
            }
            let mut parent_15: *mut ast_t = node_13;
            let mut last_sibling_15: *mut ast_t = 0 as *mut ast_t;
            let mut _node_15: *mut ast_t = 0 as *mut ast_t;
            if parent_15.is_null() {
                parent_15 = ast_from_string(basis_ast_3, name);
            } else if last_sibling_15.is_null() {
                last_sibling_15 = ast_add(parent_15, ast_from_string(basis_ast_3, name));
            } else {
                last_sibling_15 =
                    ast_add_sibling(last_sibling_15, ast_from_string(basis_ast_3, name));
            }
            ast_inheritflags(parent_15);
            ast_inheritflags(parent_13);
            ast_inheritflags(parent_12);
            body_elem = parent_11;
            ast_append(pattern, pattern_elem);
            ast_append(body, body_elem);
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "590:1"]
pub unsafe extern "C" fn expr_as(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            592 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"expr_as\0")).as_ptr(),
        );
    };
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            594 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"expr_as\0")).as_ptr(),
        );
    };
    if ast_id(ast) as libc::c_uint == TK_AS as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_AS\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            596 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"expr_as\0")).as_ptr(),
        );
    };
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut expr, &mut type_0, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut expr_type: *mut ast_t = ast_type(expr);
    if is_typecheck_error(expr_type) {
        return 0 as libc::c_int != 0;
    }
    if ast_id(expr_type) as libc::c_uint == TK_LITERAL as libc::c_int as libc::c_uint
        || ast_id(expr_type) as libc::c_uint == TK_OPERATORLITERAL as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            expr,
            b"Cannot cast uninferred literal\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if is_subtype(expr_type, type_0, 0 as *mut errorframe_t, opt) {
        if is_subtype(type_0, expr_type, 0 as *mut errorframe_t, opt) {
            ast_error(
                (*opt).check.errors,
                ast,
                b"Cannot cast to same type\0" as *const u8 as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                expr,
                b"Expression is already of type %s\0" as *const u8 as *const libc::c_char,
                ast_print_type(type_0),
            );
        } else {
            ast_error(
                (*opt).check.errors,
                ast,
                b"Cannot cast to subtype\0" as *const u8 as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                expr,
                b"%s is a subtype of this Expression. 'as' is not needed here.\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(type_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut pattern_root: *mut ast_t = ast_from(type_0, TK_LEX_ERROR);
    let mut body_root: *mut ast_t = ast_from(type_0, TK_LEX_ERROR);
    if !add_as_type(opt, ast, expr, type_0, pattern_root, body_root) {
        return 0 as libc::c_int != 0;
    }
    let mut body: *mut ast_t = ast_pop(body_root);
    ast_free(body_root);
    if body.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Cannot treat value as \"don't care\"\0" as *const u8 as *const libc::c_char,
        );
        ast_free(pattern_root);
        return 0 as libc::c_int != 0;
    }
    if ast_id(ast_child(pattern_root)) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast_child(pattern_root)) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            643 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"expr_as\0")).as_ptr(),
        );
    };
    let mut pattern: *mut ast_t = ast_pop(ast_child(pattern_root));
    ast_free(pattern_root);
    let mut basis_ast: *mut ast_t = *astp;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_MATCH);
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
    ast_scope(parent_0);
    node_0 = ast_from(basis_ast, TK_SEQ);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_1: *mut ast_t = node_0;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut _node_1: *mut ast_t = 0 as *mut ast_t;
    if parent_1.is_null() {
        parent_1 = expr;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, expr);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, expr);
    }
    ast_inheritflags(parent_1);
    node_0 = ast_from(basis_ast, TK_CASES);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_2: *mut ast_t = node_0;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_2);
    node_2 = ast_from(basis_ast, TK_CASE);
    if parent_2.is_null() {
        parent_2 = node_2;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, node_2);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
    }
    let mut parent_3: *mut ast_t = node_2;
    let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
    let mut _node_3: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_3);
    if parent_3.is_null() {
        parent_3 = pattern;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, pattern);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, pattern);
    }
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast, TK_NONE));
    }
    if parent_3.is_null() {
        parent_3 = body;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, body);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, body);
    }
    ast_inheritflags(parent_3);
    ast_inheritflags(parent_2);
    node_0 = ast_from(basis_ast, TK_SEQ);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_4: *mut ast_t = node_0;
    let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
    let mut node_4: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_4);
    node_4 = ast_from(basis_ast, TK_ERROR);
    if parent_4.is_null() {
        parent_4 = node_4;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, node_4);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, node_4);
    }
    let mut parent_5: *mut ast_t = node_4;
    let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
    let mut _node_5: *mut ast_t = 0 as *mut ast_t;
    if parent_5.is_null() {
        parent_5 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(parent_5, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_5 = ast_add_sibling(last_sibling_5, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_5);
    ast_inheritflags(parent_4);
    ast_inheritflags(parent_0);
    ast_replace(astp, parent);
    if !ast_passes_subtree(astp, opt, PASS_EXPR) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn expr_consume(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_CONSUME as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CONSUME\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.c\0" as *const u8
                as *const libc::c_char,
            665 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"expr_consume\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut term: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut cap, &mut term, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(term) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_FVARREF as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_LETREF as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_VARREF as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_THIS as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_DOT as libc::c_int as libc::c_uint
        && ast_id(term) as libc::c_uint != TK_PARAMREF as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can't consume let or embed fields\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(term);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut tcap: token_id = ast_id(cap);
    let mut c_type: *mut ast_t = consume_type(type_0, tcap, 0 as libc::c_int != 0);
    if c_type.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can't consume to this capability\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            term,
            b"expression type is %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(type_0),
        );
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, c_type);
    return 1 as libc::c_int != 0;
}
