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
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: uint32_t) -> libc::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "101:1"]
        pub fn ast_settype(ast: *mut ast_t, type_0: *mut ast_t);
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
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
        #[c2rust::src_loc = "142:1"]
        pub fn ast_swap(prev: *mut ast_t, next: *mut ast_t);
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:13"]
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
        #[c2rust::src_loc = "105:1"]
        pub fn unisolated(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:15"]
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn reify_defaults(
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            errors: bool,
            opt: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn reify(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
            duplicate: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "43:1"]
        pub fn deferred_reify_free(deferred: *mut deferred_reification_t);
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/ffi.h:2"]
pub mod ffi_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn void_star_param(param_type: *mut ast_t, arg_type: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/postfix.h:3"]
pub mod postfix_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn expr_dot(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:5"]
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
        #[c2rust::src_loc = "35:1"]
        pub fn literal_call(ast: *mut ast_t, options: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:8"]
pub mod lexer_h {
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn lexer_print(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:9"]
pub mod package_h {
    use super::frame_h::typecheck_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn package_id(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "143:1"]
        pub fn package_hygienic_id(t: *mut typecheck_t) -> *const libc::c_char;
        #[c2rust::src_loc = "160:1"]
        pub fn package_alias_from_id(
            module: *mut ast_t,
            id: *const libc::c_char,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:10"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn is_result_needed(ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn is_typecheck_error(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.h:11"]
pub mod sugar_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn expand_location(location: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:12"]
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
        #[c2rust::src_loc = "17:1"]
        pub fn chain_type(
            type_0: *mut ast_t,
            fun_cap: token_id,
            recovered_call: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "19:1"]
        pub fn sendable(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:14"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.h:15"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/safeto.h:17"]
pub mod safeto_h {
    use super::cap_h::{direction};
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn safe_to_autorecover(
            receiver: *mut ast_t,
            type_0: *mut ast_t,
            direction: direction,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/sanitise.h:18"]
pub mod sanitise_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn sanitise_type(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:19"]
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
        #[c2rust::src_loc = "56:1"]
        pub fn is_known(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "58:1"]
        pub fn is_bare(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.h:20"]
pub mod viewpoint_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn viewpoint_type(l_type: *mut ast_t, r_type: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "17:1"]
        pub fn viewpoint_upper(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:21"]
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
use self::alias_h::{alias, chain_type, consume_type, recover_type, sendable};
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_checkflag, ast_child, ast_childcount, ast_childidx,
    ast_data, ast_error, ast_error_continue, ast_error_frame, ast_free, ast_free_unattached,
    ast_from, ast_from_string, ast_get_children, ast_id, ast_inheritflags, ast_name, ast_nearest,
    ast_parent, ast_pop, ast_print_type, ast_ptr_t, ast_replace, ast_setflag, ast_setid,
    ast_settype, ast_sibling, ast_swap, ast_type, C2RustUnnamed, AST_FLAG_AMBIGUOUS,
    AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND, AST_FLAG_CNSM_REASGN,
    AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2, AST_FLAG_FCNSM_REASGN,
    AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS, AST_FLAG_JUMPS_AWAY,
    AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI, AST_FLAG_PASS_MASK,
    AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
pub use self::cap_h::{cap_fetch, direction, unisolated, EXTRACT, WRITE};
pub use self::error_h::{errorframe_append, errorframe_report, errorframe_t, errormsg_t, errors_t};
use self::expr_h::{is_result_needed, is_typecheck_error};
use self::ffi_h::void_star_param;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::lexer_h::lexer_print;
use self::literal_h::{coerce_literals, literal_call};
use self::lookup_h::lookup;
use self::package_h::{package_alias_from_id, package_hygienic_id, package_id};
pub use self::pass_h::{
    ast_passes_subtree, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL,
    PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::postfix_h::expr_dot;
pub use self::reify_h::{
    check_constraints, deferred_reification_t, deferred_reify_free, reify, reify_defaults,
};
use self::safeto_h::safe_to_autorecover;
use self::sanitise_h::sanitise_type;

use self::subtype_h::{is_bare, is_known, is_subtype};
use self::sugar_h::expand_location;
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
use self::viewpoint_h::{viewpoint_type, viewpoint_upper};
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn insert_apply(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) -> bool {
    let mut ast: *mut ast_t = *astp;
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    let mut dot: *mut ast_t = ast_from(ast, TK_DOT);
    ast_add(
        dot,
        ast_from_string(ast, b"apply\0" as *const u8 as *const libc::c_char),
    );
    ast_swap(lhs, dot);
    ast_add(dot, lhs);
    if !expr_dot(opt, &mut dot) {
        return 0 as libc::c_int != 0;
    }
    return expr_call(opt, astp);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn method_check_type_params(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> bool {
    let mut lhs: *mut ast_t = *astp;
    let mut type_0: *mut ast_t = ast_type(lhs);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut typeparams: *mut ast_t = ast_childidx(type_0, 1 as libc::c_int as size_t);
    if ast_id(type_0) as libc::c_uint == TK_FUNTYPE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_FUNTYPE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                as *const libc::c_char,
            49 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"method_check_type_params\0",
            ))
            .as_ptr(),
        );
    };
    if ast_id(typeparams) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut typeargs: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast_parent(lhs);
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
    let mut _last_sibling_0: *mut ast_t = 0 as *mut ast_t;
    let mut _node_0: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_0);
    typeargs = parent;
    if !reify_defaults(typeparams, typeargs, 1 as libc::c_int != 0, opt) {
        ast_free_unattached(typeargs);
        return 0 as libc::c_int != 0;
    }
    if !check_constraints(lhs, typeparams, typeargs, 1 as libc::c_int != 0, opt) {
        ast_free_unattached(typeargs);
        return 0 as libc::c_int != 0;
    }
    type_0 = reify(type_0, typeparams, typeargs, opt, 1 as libc::c_int != 0);
    typeparams = ast_childidx(type_0, 1 as libc::c_int as size_t);
    ast_replace(&mut typeparams, ast_from(typeparams, TK_NONE));
    let mut basis_ast_0: *mut ast_t = *astp;
    let mut parent_1: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut node_1: *mut ast_t = 0 as *mut ast_t;
    node_1 = ast_from(basis_ast_0, ast_id(lhs));
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_2: *mut ast_t = node_1;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut _node_2: *mut ast_t = 0 as *mut ast_t;
    if parent_2.is_null() {
        parent_2 = lhs;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, lhs);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, lhs);
    }
    if parent_2.is_null() {
        parent_2 = typeargs;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, typeargs);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, typeargs);
    }
    ast_inheritflags(parent_2);
    ast_replace(astp, parent_1);
    ast_settype(*astp, type_0);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn extend_positional_args(
    mut opt: *mut pass_opt_t,
    mut params: *mut ast_t,
    mut positional: *mut ast_t,
) -> bool {
    let mut param_len: size_t = ast_childcount(params);
    let mut arg_len: size_t = ast_childcount(positional);
    if arg_len > param_len {
        ast_error(
            (*opt).check.errors,
            positional,
            b"too many arguments\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            params,
            b"definition is here\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    while arg_len < param_len {
        ast_setid(positional, TK_POSITIONALARGS);
        ast_append(positional, ast_from(positional, TK_NONE));
        arg_len = arg_len.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn apply_named_args(
    mut opt: *mut pass_opt_t,
    mut params: *mut ast_t,
    mut positional: *mut ast_t,
    mut namedargs: *mut ast_t,
) -> bool {
    let mut namedarg: *mut ast_t = ast_pop(namedargs);
    while !namedarg.is_null() {
        let mut arg_id: ast_ptr_t = 0 as *mut ast_t;
        let mut arg: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut arg_id, &mut arg, 0 as *mut *mut ast_t];
        ast_get_children(
            namedarg,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children.as_mut_ptr(),
        );
        let mut param: *mut ast_t = ast_child(params);
        let mut param_index: size_t = 0;
        while !param.is_null() {
            let mut param_id: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 2] = [&mut param_id, 0 as *mut *mut ast_t];
            ast_get_children(
                param,
                (::core::mem::size_of::<[*mut *mut ast_t; 2]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            if ast_name(arg_id) == ast_name(param_id) {
                break;
            }
            param = ast_sibling(param);
            param_index = param_index.wrapping_add(1);
        }
        if param.is_null() {
            if ast_id(namedarg) as libc::c_uint == TK_UPDATEARG as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    arg_id,
                    b"cannot use sugar, update() has no parameter named \"value\"\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            ast_error(
                (*opt).check.errors,
                arg_id,
                b"not a parameter name\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        let mut arg_replace: *mut ast_t = ast_childidx(positional, param_index);
        if ast_id(arg_replace) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                arg_id,
                b"named argument is already supplied\0" as *const u8 as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                arg_replace,
                b"supplied argument is here\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        ast_free(ast_pop(namedarg));
        arg = ast_pop(namedarg);
        ast_replace(&mut arg_replace, arg);
        namedarg = ast_pop(namedargs);
    }
    ast_setid(namedargs, TK_NONE);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn apply_default_arg(
    mut opt: *mut pass_opt_t,
    mut param: *mut ast_t,
    mut argp: *mut *mut ast_t,
) -> bool {
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut def_arg: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut id, &mut type_0, &mut def_arg, 0 as *mut *mut ast_t];
    ast_get_children(
        param,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(def_arg) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            *argp,
            b"not enough arguments\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            param,
            b"definition is here\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if ast_id(def_arg) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(def_arg) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                as *const libc::c_char,
            173 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"apply_default_arg\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast_child(def_arg)) as libc::c_uint == TK_LOCATION as libc::c_int as libc::c_uint {
        let mut arg: *mut ast_t = *argp;
        let mut location: *mut ast_t = expand_location(arg);
        ast_add(arg, location);
        ast_setid(arg, TK_SEQ);
        if !ast_passes_subtree(&mut location, opt, PASS_EXPR) {
            return 0 as libc::c_int != 0;
        }
    } else {
        ast_replace(argp, def_arg);
    }
    if !ast_passes_subtree(argp, opt, PASS_EXPR) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn check_auto_recover_newref(
    mut dest_type: *mut ast_t,
    mut ast: *mut ast_t,
) -> bool {
    if ast_id(dest_type) as libc::c_uint != TK_NOMINAL as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    while !ast.is_null() && ast_id(ast) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint {
        ast = ast_child(ast);
    }
    if ast.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut newref: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [
        &mut newref,
        &mut positional,
        &mut named,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(newref) as libc::c_uint != TK_NEWREF as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    let mut child: *mut ast_t = ast_child(newref);
    if !child.is_null() && ast_id(child) as libc::c_uint == TK_NEWREF as libc::c_int as libc::c_uint
    {
        newref = child;
    }
    let mut arg: *mut ast_t = ast_child(positional);
    while !arg.is_null() {
        let mut arg_type: *mut ast_t = ast_type(arg);
        if is_typecheck_error(arg_type) {
            return 0 as libc::c_int != 0;
        }
        let mut arg_type_aliased: *mut ast_t = alias(arg_type);
        let mut ok: bool = safe_to_autorecover(dest_type, arg_type_aliased, WRITE);
        ast_free_unattached(arg_type_aliased);
        if !ok {
            return 0 as libc::c_int != 0;
        }
        arg = ast_sibling(arg);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn check_arg_types(
    mut opt: *mut pass_opt_t,
    mut params: *mut ast_t,
    mut positional: *mut ast_t,
    mut partial: bool,
    mut is_bare_0: bool,
) -> bool {
    let mut param: *mut ast_t = ast_child(params);
    let mut arg: *mut ast_t = ast_child(positional);
    while !arg.is_null() {
        if ast_id(arg) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
            if partial {
                arg = ast_sibling(arg);
                param = ast_sibling(param);
                continue;
            } else if !apply_default_arg(opt, param, &mut arg) {
                return 0 as libc::c_int != 0;
            }
        }
        let mut p_type: *mut ast_t = ast_childidx(param, 1 as libc::c_int as size_t);
        if !coerce_literals(&mut arg, p_type, opt) {
            return 0 as libc::c_int != 0;
        }
        let mut arg_type: *mut ast_t = ast_type(arg);
        if is_typecheck_error(arg_type) {
            return 0 as libc::c_int != 0;
        }
        if ast_checkflag(arg, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) != 0 {
            ast_error(
                (*opt).check.errors,
                arg,
                b"can't use a control expression in an argument\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        let mut info: errorframe_t = 0 as errorframe_t;
        let mut wp_type: *mut ast_t = consume_type(p_type, TK_NONE, 0 as libc::c_int != 0);
        if check_auto_recover_newref(wp_type, arg) {
            let mut arg_cap: token_id = ast_id(cap_fetch(wp_type));
            let mut recovered_arg_type: *mut ast_t = recover_type(arg_type, arg_cap);
            if !recovered_arg_type.is_null() {
                arg_type = recovered_arg_type;
            }
        }
        if wp_type.is_null() {
            let mut frame: errorframe_t = 0 as errorframe_t;
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                arg,
                b"argument not assignable to parameter\0" as *const u8 as *const libc::c_char,
            );
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                param,
                b"parameter type is illegal: %s\0" as *const u8 as *const libc::c_char,
                ast_print_type(p_type),
            );
            errorframe_append(&mut frame, &mut info);
            errorframe_report(&mut frame, (*opt).check.errors);
            return 0 as libc::c_int != 0;
        } else {
            if !is_subtype(arg_type, wp_type, &mut info, opt)
                && (!is_bare_0 || !void_star_param(wp_type, arg_type))
            {
                let mut frame_0: errorframe_t = 0 as errorframe_t;
                ast_error_frame(
                    &mut frame_0 as *mut errorframe_t,
                    arg,
                    b"argument not assignable to parameter\0" as *const u8 as *const libc::c_char,
                );
                ast_error_frame(
                    &mut frame_0 as *mut errorframe_t,
                    arg,
                    b"argument type is %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(arg_type),
                );
                ast_error_frame(
                    &mut frame_0 as *mut errorframe_t,
                    param,
                    b"parameter type requires %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(wp_type),
                );
                if ast_childcount(arg) > 1 as libc::c_int as libc::c_ulong {
                    ast_error_frame(
                        &mut frame_0 as *mut errorframe_t,
                        arg,
                        b"note that arguments must be separated by a comma\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if ast_checkflag(
                    ast_type(arg),
                    AST_FLAG_INCOMPLETE as libc::c_int as uint32_t,
                ) != 0
                {
                    ast_error_frame(
                        &mut frame_0 as *mut errorframe_t,
                        arg,
                        b"this might be possible if all fields were already defined\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                errorframe_append(&mut frame_0, &mut info);
                errorframe_report(&mut frame_0, (*opt).check.errors);
                ast_free_unattached(wp_type);
                return 0 as libc::c_int != 0;
            }
        }
        ast_free_unattached(wp_type);
        arg = ast_sibling(arg);
        param = ast_sibling(param);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "334:1"]
unsafe extern "C" fn auto_recover_call(
    mut ast: *mut ast_t,
    mut receiver_type: *mut ast_t,
    mut positional: *mut ast_t,
    mut result: *mut ast_t,
) -> bool {
    match ast_id(ast) as libc::c_uint {
        191 | 202 | 204 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                        as *const u8 as *const libc::c_char,
                    345 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"auto_recover_call\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    }
    let mut call: *mut ast_t = ast_parent(ast);
    if is_result_needed(call) as libc::c_int != 0
        && !safe_to_autorecover(receiver_type, result, EXTRACT)
    {
        return 0 as libc::c_int != 0;
    }
    let mut arg: *mut ast_t = ast_child(positional);
    while !arg.is_null() {
        if ast_id(arg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            let mut arg_type: *mut ast_t = ast_type(arg);
            if is_typecheck_error(arg_type) {
                return 0 as libc::c_int != 0;
            }
            let mut a_type: *mut ast_t = alias(arg_type);
            let mut ok: bool = safe_to_autorecover(receiver_type, a_type, WRITE);
            ast_free_unattached(a_type);
            if !ok {
                return 0 as libc::c_int != 0;
            }
        }
        arg = ast_sibling(arg);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "381:1"]
unsafe extern "C" fn method_receiver(mut method: *mut ast_t) -> *mut ast_t {
    let mut receiver: *mut ast_t = ast_child(method);
    if ast_id(receiver) as libc::c_uint == TK_FUNREF as libc::c_int as libc::c_uint
        || ast_id(receiver) as libc::c_uint == TK_FUNAPP as libc::c_int as libc::c_uint
        || ast_id(receiver) as libc::c_uint == TK_FUNCHAIN as libc::c_int as libc::c_uint
    {
        receiver = ast_child(receiver);
    }
    return receiver;
}
#[c2rust::src_loc = "393:1"]
unsafe extern "C" fn method_receiver_type(mut method: *mut ast_t) -> *mut ast_t {
    let mut receiver: *mut ast_t = ast_child(method);
    if ast_id(receiver) as libc::c_uint == TK_FUNREF as libc::c_int as libc::c_uint
        || ast_id(receiver) as libc::c_uint == TK_FUNAPP as libc::c_int as libc::c_uint
        || ast_id(receiver) as libc::c_uint == TK_FUNCHAIN as libc::c_int as libc::c_uint
    {
        receiver = ast_child(receiver);
    }
    let mut r_type: *mut ast_t = ast_type(receiver);
    return r_type;
}
#[c2rust::src_loc = "407:1"]
unsafe extern "C" fn check_receiver_cap(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut recovered: *mut bool,
) -> bool {
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    let mut type_0: *mut ast_t = ast_type(lhs);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 5] = [
        &mut cap,
        &mut typeparams,
        &mut params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        type_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_0.as_mut_ptr(),
    );
    let mut r_type: *mut ast_t = method_receiver_type(lhs);
    if is_typecheck_error(r_type) {
        return 0 as libc::c_int != 0;
    }
    let mut t_type: *mut ast_t = set_cap_and_ephemeral(r_type, ast_id(cap), TK_EPHEMERAL);
    let mut a_type: *mut ast_t = 0 as *mut ast_t;
    let mut can_recover: bool = auto_recover_call(lhs, r_type, positional, result);
    let mut cap_recover: bool = 0 as libc::c_int != 0;
    match ast_id(cap) as libc::c_uint {
        91 | 92 | 94 | 96 => {}
        93 | 95 => {
            cap_recover = 1 as libc::c_int != 0;
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                        as *const u8 as *const libc::c_char,
                    445 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"check_receiver_cap\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    }
    if can_recover as libc::c_int != 0 && cap_recover as libc::c_int != 0 {
        a_type = unisolated(r_type);
        if !recovered.is_null() {
            *recovered = 1 as libc::c_int != 0;
        }
    } else {
        a_type = r_type;
        if !recovered.is_null() {
            *recovered = 0 as libc::c_int != 0;
        }
    }
    let mut info: errorframe_t = 0 as errorframe_t;
    let mut ok: bool = is_subtype(a_type, t_type, &mut info, opt);
    if !ok {
        let mut frame: errorframe_t = 0 as errorframe_t;
        ast_error_frame(
            &mut frame as *mut errorframe_t,
            ast,
            b"receiver type is not a subtype of target type\0" as *const u8 as *const libc::c_char,
        );
        match ast_id(a_type) as libc::c_uint {
            17 => {
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    ast_child(lhs),
                    b"receiver type: %s (which becomes '%s' in this context)\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(a_type),
                    ast_print_type(viewpoint_upper(a_type)),
                );
            }
            _ => {
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    ast_child(lhs),
                    b"receiver type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(a_type),
                );
            }
        }
        ast_error_frame(
            &mut frame as *mut errorframe_t,
            cap,
            b"target type: %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(t_type),
        );
        errorframe_append(&mut frame, &mut info);
        if ast_checkflag(
            ast_type(method_receiver(lhs)),
            AST_FLAG_INCOMPLETE as libc::c_int as uint32_t,
        ) != 0
        {
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                method_receiver(lhs),
                b"this might be possible if all fields were already defined\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !can_recover
            && cap_recover as libc::c_int != 0
            && is_subtype(r_type, t_type, 0 as *mut errorframe_t, opt) as libc::c_int != 0
        {
            ast_error_frame(
                &mut frame as *mut errorframe_t,
                ast,
                b"this would be possible if the arguments and return value were all sendable\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        let mut fn_0: *mut ast_t = ast_nearest(lhs, TK_FUN);
        if !fn_0.is_null()
            && ast_id(a_type) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint
        {
            let mut iso: *mut ast_t = ast_child(fn_0);
            if !iso.is_null() {
            } else {
                ponyint_assert_fail(
                    b"iso != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                        as *const u8 as *const libc::c_char,
                    501 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"check_receiver_cap\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut iso_id: token_id = ast_id(iso);
            let mut t_cap: *mut ast_t = cap_fetch(t_type);
            if !t_cap.is_null() {
            } else {
                ponyint_assert_fail(
                    b"t_cap != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                        as *const u8 as *const libc::c_char,
                    505 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"check_receiver_cap\0",
                    ))
                    .as_ptr(),
                );
            };
            if ast_id(t_cap) as libc::c_uint == TK_REF as libc::c_int as libc::c_uint
                && (iso_id as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
                    || iso_id as libc::c_uint == TK_VAL as libc::c_int as libc::c_uint
                    || iso_id as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint)
            {
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    iso,
                    b"you are trying to change state in a %s function; this would be possible in a ref function\0"
                        as *const u8 as *const libc::c_char,
                    lexer_print(iso_id),
                );
            }
        }
        errorframe_report(&mut frame, (*opt).check.errors);
    }
    if a_type != r_type {
        ast_free_unattached(a_type);
    }
    ast_free_unattached(r_type);
    ast_free_unattached(t_type);
    return ok;
}
#[c2rust::src_loc = "524:1"]
unsafe extern "C" fn is_receiver_safe(mut t: *mut typecheck_t, mut ast: *mut ast_t) -> bool {
    match ast_id(ast) as libc::c_uint {
        102 | 193 | 192 | 194 | 198 | 195 => {
            let mut type_0: *mut ast_t = ast_type(ast);
            return sendable(type_0);
        }
        197 | 196 => {
            let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
            if !def.is_null() {
            } else {
                ponyint_assert_fail(
                    b"def != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                        as *const u8 as *const libc::c_char,
                    543 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"is_receiver_safe\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut def_recover: *mut ast_t = ast_nearest(def, TK_RECOVER);
            if (*(*t).frame).recover == def_recover {
                return 1 as libc::c_int != 0;
            }
            let mut type_1: *mut ast_t = ast_type(ast);
            return sendable(type_1);
        }
        _ => return 1 as libc::c_int != 0,
    };
}
#[c2rust::src_loc = "557:1"]
unsafe extern "C" fn check_nonsendable_recover(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> bool {
    if !((*(*opt).check.frame).recover).is_null() {
        let mut lhs: ast_ptr_t = 0 as *mut ast_t;
        let mut positional: ast_ptr_t = 0 as *mut ast_t;
        let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
        let mut question: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 5] = [
            &mut lhs,
            &mut positional,
            &mut namedargs,
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
        let mut type_0: *mut ast_t = ast_type(lhs);
        let mut cap: ast_ptr_t = 0 as *mut ast_t;
        let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
        let mut params: ast_ptr_t = 0 as *mut ast_t;
        let mut result: ast_ptr_t = 0 as *mut ast_t;
        let mut children_0: [*mut *mut ast_t; 5] = [
            &mut cap,
            &mut typeparams,
            &mut params,
            &mut result,
            0 as *mut *mut ast_t,
        ];
        ast_get_children(
            type_0,
            (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children_0.as_mut_ptr(),
        );
        if ast_id(cap) as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        }
        let mut receiver: *mut ast_t = ast_child(lhs);
        if ast_id(receiver) as libc::c_uint == TK_FUNREF as libc::c_int as libc::c_uint
            || ast_id(receiver) as libc::c_uint == TK_FUNAPP as libc::c_int as libc::c_uint
            || ast_id(receiver) as libc::c_uint == TK_FUNCHAIN as libc::c_int as libc::c_uint
        {
            receiver = ast_child(receiver);
        }
        if !is_receiver_safe(&mut (*opt).check, receiver) {
            let mut arg: *mut ast_t = ast_child(positional);
            let mut args_sendable: bool = 1 as libc::c_int != 0;
            while !arg.is_null() {
                if ast_id(arg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    let mut arg_type: *mut ast_t = ast_type(arg);
                    if !sendable(arg_type) {
                        args_sendable = 0 as libc::c_int != 0;
                        break;
                    }
                }
                arg = ast_sibling(arg);
            }
            if !args_sendable || !sendable(result) {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't call method on non-sendable object inside of a recover expression\0"
                        as *const u8 as *const libc::c_char,
                );
                ast_error_continue(
                    (*opt).check.errors,
                    ast,
                    b"this would be possible if the arguments and return value were all sendable\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "610:1"]
unsafe extern "C" fn method_application(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut partial: bool,
) -> bool {
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    if !method_check_type_params(opt, &mut lhs) {
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(lhs);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 5] = [
        &mut cap,
        &mut typeparams,
        &mut params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        type_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_0.as_mut_ptr(),
    );
    let mut bare: bool = ast_id(cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
    if !extend_positional_args(opt, params, positional) {
        return 0 as libc::c_int != 0;
    }
    if !apply_named_args(opt, params, positional, namedargs) {
        return 0 as libc::c_int != 0;
    }
    if !check_arg_types(opt, params, positional, partial, bare) {
        return 0 as libc::c_int != 0;
    }
    match ast_id(lhs) as libc::c_uint {
        191 | 202 => {
            if !bare {
                if !check_receiver_cap(opt, ast, 0 as *mut bool) {
                    return 0 as libc::c_int != 0;
                }
                if !check_nonsendable_recover(opt, ast) {
                    return 0 as libc::c_int != 0;
                }
            } else {
                let mut receiver: *mut ast_t = ast_child(lhs);
                if ast_id(receiver) as libc::c_uint == TK_FUNREF as libc::c_int as libc::c_uint
                    || ast_id(receiver) as libc::c_uint == TK_FUNAPP as libc::c_int as libc::c_uint
                    || ast_id(receiver) as libc::c_uint
                        == TK_FUNCHAIN as libc::c_int as libc::c_uint
                {
                    receiver = ast_child(receiver);
                }
                let mut recv_type: *mut ast_t = ast_type(receiver);
                if !is_known(recv_type)
                    && ast_id(receiver) as libc::c_uint == TK_TYPEREF as libc::c_int as libc::c_uint
                {
                    ast_error(
                        (*opt).check.errors,
                        lhs,
                        b"a bare method cannot be called on an abstract type reference\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "670:1"]
unsafe extern "C" fn method_call(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if !method_application(opt, ast, 0 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    let mut type_0: *mut ast_t = ast_type(lhs);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 5] = [
        &mut cap,
        &mut typeparams,
        &mut params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        type_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_0.as_mut_ptr(),
    );
    ast_settype(ast, result);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "687:1"]
unsafe extern "C" fn partial_application_cap(
    mut opt: *mut pass_opt_t,
    mut ftype: *mut ast_t,
    mut receiver: *mut ast_t,
    mut positional: *mut ast_t,
) -> token_id {
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut cap,
        &mut typeparams,
        &mut params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ftype,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut type_0: *mut ast_t = ast_type(receiver);
    let mut view_type: *mut ast_t = viewpoint_type(ast_from(type_0, TK_BOX), type_0);
    let mut need_type: *mut ast_t = set_cap_and_ephemeral(type_0, ast_id(cap), TK_NONE);
    let mut ok: bool = is_subtype(view_type, need_type, 0 as *mut errorframe_t, opt);
    ast_free_unattached(view_type);
    ast_free_unattached(need_type);
    if !ok {
        return TK_REF;
    }
    let mut param: *mut ast_t = ast_child(params);
    let mut arg: *mut ast_t = ast_child(positional);
    while !arg.is_null() {
        if ast_id(arg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            type_0 = ast_type(arg);
            view_type = viewpoint_type(ast_from(type_0, TK_BOX), type_0);
            need_type = ast_childidx(param, 1 as libc::c_int as size_t);
            ok = is_subtype(view_type, need_type, 0 as *mut errorframe_t, opt);
            ast_free_unattached(view_type);
            ast_free_unattached(need_type);
            if !ok {
                return TK_REF;
            }
        }
        arg = ast_sibling(arg);
        param = ast_sibling(param);
    }
    return TK_BOX;
}
#[c2rust::src_loc = "733:1"]
unsafe extern "C" fn partial_application(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> bool {
    let mut ast: *mut ast_t = *astp;
    let mut t: *mut typecheck_t = &mut (*opt).check;
    if !method_application(opt, ast, 1 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    if ast_id(lhs) as libc::c_uint == TK_FUNAPP as libc::c_int as libc::c_uint
        || ast_id(lhs) as libc::c_uint == TK_BEAPP as libc::c_int as libc::c_uint
        || ast_id(lhs) as libc::c_uint == TK_NEWAPP as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(lhs) == TK_FUNAPP || ast_id(lhs) == TK_BEAPP || ast_id(lhs) == TK_NEWAPP\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                as *const libc::c_char,
            761 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"partial_application\0"))
                .as_ptr(),
        );
    };
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        lhs,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_0.as_mut_ptr(),
    );
    let mut type_args: *mut ast_t = 0 as *mut ast_t;
    if ast_id(receiver) as libc::c_uint == ast_id(lhs) as libc::c_uint {
        type_args = method;
        let mut children_1: [*mut *mut ast_t; 3] =
            [&mut receiver, &mut method, 0 as *mut *mut ast_t];
        ast_get_children(
            receiver,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children_1.as_mut_ptr(),
        );
    }
    let mut method_def: *mut deferred_reification_t =
        lookup(opt, lhs, ast_type(receiver), ast_name(method));
    let mut method_ast: *mut ast_t = (*method_def).ast;
    deferred_reify_free(method_def);
    if ast_id(method_ast) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
        || ast_id(method_ast) as libc::c_uint == TK_BE as libc::c_int as libc::c_uint
        || ast_id(method_ast) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(method_ast) == TK_FUN || ast_id(method_ast) == TK_BE || ast_id(method_ast) == TK_NEW\0"
                as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0"
                as *const u8 as *const libc::c_char,
            781 as libc::c_int as size_t,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"partial_application\0"))
                .as_ptr(),
        );
    };
    let mut type_0: *mut ast_t = ast_type(lhs);
    if ast_id(type_0) as libc::c_uint == TK_FUNTYPE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_FUNTYPE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                as *const libc::c_char,
            785 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"partial_application\0"))
                .as_ptr(),
        );
    };
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut type_params: ast_ptr_t = 0 as *mut ast_t;
    let mut target_params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children_2: [*mut *mut ast_t; 5] = [
        &mut cap,
        &mut type_params,
        &mut target_params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        type_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_2.as_mut_ptr(),
    );
    let mut bare: bool = ast_id(cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
    let mut apply_cap: token_id = TK_AT;
    if !bare {
        apply_cap = partial_application_cap(opt, type_0, receiver, positional);
    }
    let mut can_error: token_id = ast_id(ast_childidx(method_ast, 5 as libc::c_int as size_t));
    let mut recv_name: *const libc::c_char = package_hygienic_id(t);
    let mut call_receiver: *mut ast_t = 0 as *mut ast_t;
    if bare {
        let mut arg: *mut ast_t = ast_child(positional);
        while !arg.is_null() {
            if ast_id(arg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    arg,
                    b"the partial application of a bare method cannot take arguments\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            arg = ast_sibling(arg);
        }
        let mut receiver_type: *mut ast_t = ast_type(receiver);
        if is_bare(receiver_type) {
            ast_replace(astp, receiver);
            return 1 as libc::c_int != 0;
        }
        let mut recv_type_package: ast_ptr_t = 0 as *mut ast_t;
        let mut recv_type_name: ast_ptr_t = 0 as *mut ast_t;
        let mut children_3: [*mut *mut ast_t; 3] = [
            &mut recv_type_package,
            &mut recv_type_name,
            0 as *mut *mut ast_t,
        ];
        ast_get_children(
            receiver_type,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children_3.as_mut_ptr(),
        );
        let mut recv_package_str: *const libc::c_char = ast_name(recv_type_package);
        let mut recv_name_str: *const libc::c_char = ast_name(recv_type_name);
        let mut module: *mut ast_t = ast_nearest(ast, TK_MODULE);
        let mut package: *mut ast_t = ast_parent(module);
        let mut pkg_id: *mut ast_t = package_id(package);
        let mut pkg_str: *const libc::c_char = ast_name(pkg_id);
        let mut pkg_alias: *const libc::c_char = 0 as *const libc::c_char;
        if recv_package_str != pkg_str {
            pkg_alias = package_alias_from_id(module, recv_package_str);
        }
        ast_free_unattached(pkg_id);
        if !pkg_alias.is_null() {
            let mut basis_ast: *mut ast_t = ast;
            let mut parent: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
            let mut node: *mut ast_t = 0 as *mut ast_t;
            node = ast_from(basis_ast, TK_DOT);
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
            node_0 = ast_from(basis_ast, TK_DOT);
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
            node_1 = ast_from(basis_ast, TK_REFERENCE);
            if parent_1.is_null() {
                parent_1 = node_1;
            } else if last_sibling_1.is_null() {
                last_sibling_1 = ast_add(parent_1, node_1);
            } else {
                last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
            }
            let mut parent_2: *mut ast_t = node_1;
            let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
            let mut _node_2: *mut ast_t = 0 as *mut ast_t;
            if parent_2.is_null() {
                parent_2 = ast_from_string(basis_ast, pkg_alias);
            } else if last_sibling_2.is_null() {
                last_sibling_2 = ast_add(parent_2, ast_from_string(basis_ast, pkg_alias));
            } else {
                last_sibling_2 =
                    ast_add_sibling(last_sibling_2, ast_from_string(basis_ast, pkg_alias));
            }
            ast_inheritflags(parent_2);
            if parent_1.is_null() {
                parent_1 = ast_from_string(basis_ast, recv_name_str);
            } else if last_sibling_1.is_null() {
                last_sibling_1 = ast_add(parent_1, ast_from_string(basis_ast, recv_name_str));
            } else {
                last_sibling_1 =
                    ast_add_sibling(last_sibling_1, ast_from_string(basis_ast, recv_name_str));
            }
            ast_inheritflags(parent_1);
            if parent_0.is_null() {
                parent_0 = method;
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(parent_0, method);
            } else {
                last_sibling_0 = ast_add_sibling(last_sibling_0, method);
            }
            ast_inheritflags(parent_0);
            call_receiver = parent;
        } else {
            let mut basis_ast_0: *mut ast_t = ast;
            let mut parent_3: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
            let mut node_3: *mut ast_t = 0 as *mut ast_t;
            node_3 = ast_from(basis_ast_0, TK_DOT);
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
            node_4 = ast_from(basis_ast_0, TK_REFERENCE);
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
                parent_5 = ast_from_string(basis_ast_0, recv_name_str);
            } else if last_sibling_5.is_null() {
                last_sibling_5 = ast_add(parent_5, ast_from_string(basis_ast_0, recv_name_str));
            } else {
                last_sibling_5 =
                    ast_add_sibling(last_sibling_5, ast_from_string(basis_ast_0, recv_name_str));
            }
            ast_inheritflags(parent_5);
            if parent_4.is_null() {
                parent_4 = method;
            } else if last_sibling_4.is_null() {
                last_sibling_4 = ast_add(parent_4, method);
            } else {
                last_sibling_4 = ast_add_sibling(last_sibling_4, method);
            }
            ast_inheritflags(parent_4);
            call_receiver = parent_3;
        }
    } else {
        let mut basis_ast_1: *mut ast_t = ast;
        let mut parent_6: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
        let mut node_6: *mut ast_t = 0 as *mut ast_t;
        node_6 = ast_from(basis_ast_1, TK_DOT);
        if parent_6.is_null() {
            parent_6 = node_6;
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, node_6);
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
        }
        let mut parent_7: *mut ast_t = node_6;
        let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
        let mut node_7: *mut ast_t = 0 as *mut ast_t;
        node_7 = ast_from(basis_ast_1, TK_REFERENCE);
        if parent_7.is_null() {
            parent_7 = node_7;
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, node_7);
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, node_7);
        }
        let mut parent_8: *mut ast_t = node_7;
        let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
        let mut _node_8: *mut ast_t = 0 as *mut ast_t;
        if parent_8.is_null() {
            parent_8 = ast_from_string(basis_ast_1, recv_name);
        } else if last_sibling_8.is_null() {
            last_sibling_8 = ast_add(parent_8, ast_from_string(basis_ast_1, recv_name));
        } else {
            last_sibling_8 =
                ast_add_sibling(last_sibling_8, ast_from_string(basis_ast_1, recv_name));
        }
        ast_inheritflags(parent_8);
        if parent_7.is_null() {
            parent_7 = method;
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, method);
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, method);
        }
        ast_inheritflags(parent_7);
        call_receiver = parent_6;
    }
    let mut captures: *mut ast_t = 0 as *mut ast_t;
    if bare {
        captures = ast_from(receiver, TK_NONE);
    } else {
        let mut basis_ast_2: *mut ast_t = receiver;
        let mut parent_9: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_9: *mut ast_t = 0 as *mut ast_t;
        let mut node_9: *mut ast_t = 0 as *mut ast_t;
        node_9 = ast_from(basis_ast_2, TK_LAMBDACAPTURES);
        if parent_9.is_null() {
            parent_9 = node_9;
        } else if last_sibling_9.is_null() {
            last_sibling_9 = ast_add(parent_9, node_9);
        } else {
            last_sibling_9 = ast_add_sibling(last_sibling_9, node_9);
        }
        let mut parent_10: *mut ast_t = node_9;
        let mut last_sibling_10: *mut ast_t = 0 as *mut ast_t;
        let mut node_10: *mut ast_t = 0 as *mut ast_t;
        node_10 = ast_from(basis_ast_2, TK_LAMBDACAPTURE);
        if parent_10.is_null() {
            parent_10 = node_10;
        } else if last_sibling_10.is_null() {
            last_sibling_10 = ast_add(parent_10, node_10);
        } else {
            last_sibling_10 = ast_add_sibling(last_sibling_10, node_10);
        }
        let mut parent_11: *mut ast_t = node_10;
        let mut last_sibling_11: *mut ast_t = 0 as *mut ast_t;
        let mut _node_11: *mut ast_t = 0 as *mut ast_t;
        if parent_11.is_null() {
            parent_11 = ast_from_string(basis_ast_2, recv_name);
        } else if last_sibling_11.is_null() {
            last_sibling_11 = ast_add(parent_11, ast_from_string(basis_ast_2, recv_name));
        } else {
            last_sibling_11 =
                ast_add_sibling(last_sibling_11, ast_from_string(basis_ast_2, recv_name));
        }
        if parent_11.is_null() {
            parent_11 = ast_from(basis_ast_2, TK_NONE);
        } else if last_sibling_11.is_null() {
            last_sibling_11 = ast_add(parent_11, ast_from(basis_ast_2, TK_NONE));
        } else {
            last_sibling_11 = ast_add_sibling(last_sibling_11, ast_from(basis_ast_2, TK_NONE));
        }
        if parent_11.is_null() {
            parent_11 = receiver;
        } else if last_sibling_11.is_null() {
            last_sibling_11 = ast_add(parent_11, receiver);
        } else {
            last_sibling_11 = ast_add_sibling(last_sibling_11, receiver);
        }
        ast_inheritflags(parent_11);
        ast_inheritflags(parent_10);
        captures = parent_9;
    }
    let mut target_param: *mut ast_t = ast_child(target_params);
    let mut lambda_params: *mut ast_t = ast_from(target_params, TK_NONE);
    let mut lambda_call_args: *mut ast_t = ast_from(positional, TK_NONE);
    let mut given_arg: *mut ast_t = ast_child(positional);
    while !given_arg.is_null() {
        if !target_param.is_null() {
        } else {
            ponyint_assert_fail(
                b"target_param != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                    as *const libc::c_char,
                890 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"partial_application\0",
                ))
                .as_ptr(),
            );
        };
        let mut target_p_name: *const libc::c_char = ast_name(ast_child(target_param));
        if ast_id(given_arg) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
            let mut p_id: ast_ptr_t = 0 as *mut ast_t;
            let mut p_type: ast_ptr_t = 0 as *mut ast_t;
            let mut p_default: ast_ptr_t = 0 as *mut ast_t;
            let mut children_4: [*mut *mut ast_t; 4] =
                [&mut p_id, &mut p_type, &mut p_default, 0 as *mut *mut ast_t];
            ast_get_children(
                target_param,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_4.as_mut_ptr(),
            );
            let mut lambda_param: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_3: *mut ast_t = target_param;
            let mut parent_12: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_12: *mut ast_t = 0 as *mut ast_t;
            let mut node_12: *mut ast_t = 0 as *mut ast_t;
            node_12 = ast_from(basis_ast_3, TK_PARAM);
            if parent_12.is_null() {
                parent_12 = node_12;
            } else if last_sibling_12.is_null() {
                last_sibling_12 = ast_add(parent_12, node_12);
            } else {
                last_sibling_12 = ast_add_sibling(last_sibling_12, node_12);
            }
            let mut parent_13: *mut ast_t = node_12;
            let mut last_sibling_13: *mut ast_t = 0 as *mut ast_t;
            let mut _node_13: *mut ast_t = 0 as *mut ast_t;
            if parent_13.is_null() {
                parent_13 = p_id;
            } else if last_sibling_13.is_null() {
                last_sibling_13 = ast_add(parent_13, p_id);
            } else {
                last_sibling_13 = ast_add_sibling(last_sibling_13, p_id);
            }
            if parent_13.is_null() {
                parent_13 = sanitise_type(p_type);
            } else if last_sibling_13.is_null() {
                last_sibling_13 = ast_add(parent_13, sanitise_type(p_type));
            } else {
                last_sibling_13 = ast_add_sibling(last_sibling_13, sanitise_type(p_type));
            }
            if parent_13.is_null() {
                parent_13 = p_default;
            } else if last_sibling_13.is_null() {
                last_sibling_13 = ast_add(parent_13, p_default);
            } else {
                last_sibling_13 = ast_add_sibling(last_sibling_13, p_default);
            }
            ast_inheritflags(parent_13);
            lambda_param = parent_12;
            ast_append(lambda_params, lambda_param);
            ast_setid(lambda_params, TK_PARAMS);
            let mut target_arg: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_4: *mut ast_t = lambda_param;
            let mut parent_14: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_14: *mut ast_t = 0 as *mut ast_t;
            let mut node_14: *mut ast_t = 0 as *mut ast_t;
            node_14 = ast_from(basis_ast_4, TK_SEQ);
            if parent_14.is_null() {
                parent_14 = node_14;
            } else if last_sibling_14.is_null() {
                last_sibling_14 = ast_add(parent_14, node_14);
            } else {
                last_sibling_14 = ast_add_sibling(last_sibling_14, node_14);
            }
            let mut parent_15: *mut ast_t = node_14;
            let mut last_sibling_15: *mut ast_t = 0 as *mut ast_t;
            let mut node_15: *mut ast_t = 0 as *mut ast_t;
            node_15 = ast_from(basis_ast_4, TK_CONSUME);
            if parent_15.is_null() {
                parent_15 = node_15;
            } else if last_sibling_15.is_null() {
                last_sibling_15 = ast_add(parent_15, node_15);
            } else {
                last_sibling_15 = ast_add_sibling(last_sibling_15, node_15);
            }
            let mut parent_16: *mut ast_t = node_15;
            let mut last_sibling_16: *mut ast_t = 0 as *mut ast_t;
            let mut node_16: *mut ast_t = 0 as *mut ast_t;
            if parent_16.is_null() {
                parent_16 = ast_from(basis_ast_4, TK_NONE);
            } else if last_sibling_16.is_null() {
                last_sibling_16 = ast_add(parent_16, ast_from(basis_ast_4, TK_NONE));
            } else {
                last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast_4, TK_NONE));
            }
            node_16 = ast_from(basis_ast_4, TK_REFERENCE);
            if parent_16.is_null() {
                parent_16 = node_16;
            } else if last_sibling_16.is_null() {
                last_sibling_16 = ast_add(parent_16, node_16);
            } else {
                last_sibling_16 = ast_add_sibling(last_sibling_16, node_16);
            }
            let mut parent_17: *mut ast_t = node_16;
            let mut last_sibling_17: *mut ast_t = 0 as *mut ast_t;
            let mut _node_17: *mut ast_t = 0 as *mut ast_t;
            if parent_17.is_null() {
                parent_17 = ast_from_string(basis_ast_4, target_p_name);
            } else if last_sibling_17.is_null() {
                last_sibling_17 = ast_add(parent_17, ast_from_string(basis_ast_4, target_p_name));
            } else {
                last_sibling_17 =
                    ast_add_sibling(last_sibling_17, ast_from_string(basis_ast_4, target_p_name));
            }
            ast_inheritflags(parent_17);
            ast_inheritflags(parent_16);
            ast_inheritflags(parent_15);
            target_arg = parent_14;
            ast_append(lambda_call_args, target_arg);
            ast_setid(lambda_call_args, TK_POSITIONALARGS);
        } else {
            let mut capture: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_5: *mut ast_t = given_arg;
            let mut parent_18: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_18: *mut ast_t = 0 as *mut ast_t;
            let mut node_18: *mut ast_t = 0 as *mut ast_t;
            node_18 = ast_from(basis_ast_5, TK_LAMBDACAPTURE);
            if parent_18.is_null() {
                parent_18 = node_18;
            } else if last_sibling_18.is_null() {
                last_sibling_18 = ast_add(parent_18, node_18);
            } else {
                last_sibling_18 = ast_add_sibling(last_sibling_18, node_18);
            }
            let mut parent_19: *mut ast_t = node_18;
            let mut last_sibling_19: *mut ast_t = 0 as *mut ast_t;
            let mut _node_19: *mut ast_t = 0 as *mut ast_t;
            if parent_19.is_null() {
                parent_19 = ast_from_string(basis_ast_5, target_p_name);
            } else if last_sibling_19.is_null() {
                last_sibling_19 = ast_add(parent_19, ast_from_string(basis_ast_5, target_p_name));
            } else {
                last_sibling_19 =
                    ast_add_sibling(last_sibling_19, ast_from_string(basis_ast_5, target_p_name));
            }
            if parent_19.is_null() {
                parent_19 = ast_from(basis_ast_5, TK_NONE);
            } else if last_sibling_19.is_null() {
                last_sibling_19 = ast_add(parent_19, ast_from(basis_ast_5, TK_NONE));
            } else {
                last_sibling_19 = ast_add_sibling(last_sibling_19, ast_from(basis_ast_5, TK_NONE));
            }
            if parent_19.is_null() {
                parent_19 = given_arg;
            } else if last_sibling_19.is_null() {
                last_sibling_19 = ast_add(parent_19, given_arg);
            } else {
                last_sibling_19 = ast_add_sibling(last_sibling_19, given_arg);
            }
            ast_inheritflags(parent_19);
            capture = parent_18;
            ast_append(captures, capture);
            let mut target_arg_0: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_6: *mut ast_t = given_arg;
            let mut parent_20: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_20: *mut ast_t = 0 as *mut ast_t;
            let mut node_20: *mut ast_t = 0 as *mut ast_t;
            node_20 = ast_from(basis_ast_6, TK_SEQ);
            if parent_20.is_null() {
                parent_20 = node_20;
            } else if last_sibling_20.is_null() {
                last_sibling_20 = ast_add(parent_20, node_20);
            } else {
                last_sibling_20 = ast_add_sibling(last_sibling_20, node_20);
            }
            let mut parent_21: *mut ast_t = node_20;
            let mut last_sibling_21: *mut ast_t = 0 as *mut ast_t;
            let mut node_21: *mut ast_t = 0 as *mut ast_t;
            node_21 = ast_from(basis_ast_6, TK_REFERENCE);
            if parent_21.is_null() {
                parent_21 = node_21;
            } else if last_sibling_21.is_null() {
                last_sibling_21 = ast_add(parent_21, node_21);
            } else {
                last_sibling_21 = ast_add_sibling(last_sibling_21, node_21);
            }
            let mut parent_22: *mut ast_t = node_21;
            let mut last_sibling_22: *mut ast_t = 0 as *mut ast_t;
            let mut _node_22: *mut ast_t = 0 as *mut ast_t;
            if parent_22.is_null() {
                parent_22 = ast_from_string(basis_ast_6, target_p_name);
            } else if last_sibling_22.is_null() {
                last_sibling_22 = ast_add(parent_22, ast_from_string(basis_ast_6, target_p_name));
            } else {
                last_sibling_22 =
                    ast_add_sibling(last_sibling_22, ast_from_string(basis_ast_6, target_p_name));
            }
            ast_inheritflags(parent_22);
            ast_inheritflags(parent_21);
            target_arg_0 = parent_20;
            ast_append(lambda_call_args, target_arg_0);
            ast_setid(lambda_call_args, TK_POSITIONALARGS);
        }
        given_arg = ast_sibling(given_arg);
        target_param = ast_sibling(target_param);
    }
    if target_param.is_null() {
    } else {
        ponyint_assert_fail(
            b"target_param == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.c\0" as *const u8
                as *const libc::c_char,
            948 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"partial_application\0"))
                .as_ptr(),
        );
    };
    if !type_args.is_null() {
        let mut qualified: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_7: *mut ast_t = type_args;
        let mut parent_23: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_23: *mut ast_t = 0 as *mut ast_t;
        let mut node_23: *mut ast_t = 0 as *mut ast_t;
        node_23 = ast_from(basis_ast_7, TK_QUALIFY);
        if parent_23.is_null() {
            parent_23 = node_23;
        } else if last_sibling_23.is_null() {
            last_sibling_23 = ast_add(parent_23, node_23);
        } else {
            last_sibling_23 = ast_add_sibling(last_sibling_23, node_23);
        }
        let mut parent_24: *mut ast_t = node_23;
        let mut last_sibling_24: *mut ast_t = 0 as *mut ast_t;
        let mut _node_24: *mut ast_t = 0 as *mut ast_t;
        if parent_24.is_null() {
            parent_24 = call_receiver;
        } else if last_sibling_24.is_null() {
            last_sibling_24 = ast_add(parent_24, call_receiver);
        } else {
            last_sibling_24 = ast_add_sibling(last_sibling_24, call_receiver);
        }
        if parent_24.is_null() {
            parent_24 = type_args;
        } else if last_sibling_24.is_null() {
            last_sibling_24 = ast_add(parent_24, type_args);
        } else {
            last_sibling_24 = ast_add_sibling(last_sibling_24, type_args);
        }
        ast_inheritflags(parent_24);
        qualified = parent_23;
        call_receiver = qualified;
    }
    let mut basis_ast_8: *mut ast_t = *astp;
    let mut parent_25: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_25: *mut ast_t = 0 as *mut ast_t;
    let mut node_25: *mut ast_t = 0 as *mut ast_t;
    node_25 = ast_from(
        basis_ast_8,
        (if bare as libc::c_int != 0 {
            TK_BARELAMBDA as libc::c_int
        } else {
            TK_LAMBDA as libc::c_int
        }) as token_id,
    );
    if parent_25.is_null() {
        parent_25 = node_25;
    } else if last_sibling_25.is_null() {
        last_sibling_25 = ast_add(parent_25, node_25);
    } else {
        last_sibling_25 = ast_add_sibling(last_sibling_25, node_25);
    }
    let mut parent_26: *mut ast_t = node_25;
    let mut last_sibling_26: *mut ast_t = 0 as *mut ast_t;
    let mut node_26: *mut ast_t = 0 as *mut ast_t;
    node_26 = ast_from(basis_ast_8, apply_cap);
    if parent_26.is_null() {
        parent_26 = node_26;
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, node_26);
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, node_26);
    }
    let mut parent_27: *mut ast_t = node_26;
    let mut _last_sibling_27: *mut ast_t = 0 as *mut ast_t;
    let mut _node_27: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_27);
    if parent_26.is_null() {
        parent_26 = ast_from(basis_ast_8, TK_NONE);
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, ast_from(basis_ast_8, TK_NONE));
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, ast_from(basis_ast_8, TK_NONE));
    }
    if parent_26.is_null() {
        parent_26 = ast_from(basis_ast_8, TK_NONE);
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, ast_from(basis_ast_8, TK_NONE));
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, ast_from(basis_ast_8, TK_NONE));
    }
    if parent_26.is_null() {
        parent_26 = lambda_params;
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, lambda_params);
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, lambda_params);
    }
    if parent_26.is_null() {
        parent_26 = captures;
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, captures);
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, captures);
    }
    if parent_26.is_null() {
        parent_26 = sanitise_type(result);
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, sanitise_type(result));
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, sanitise_type(result));
    }
    node_26 = ast_from(basis_ast_8, can_error);
    if parent_26.is_null() {
        parent_26 = node_26;
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, node_26);
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, node_26);
    }
    let mut parent_28: *mut ast_t = node_26;
    let mut _last_sibling_28: *mut ast_t = 0 as *mut ast_t;
    let mut _node_28: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_28);
    node_26 = ast_from(basis_ast_8, TK_SEQ);
    if parent_26.is_null() {
        parent_26 = node_26;
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, node_26);
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, node_26);
    }
    let mut parent_29: *mut ast_t = node_26;
    let mut last_sibling_29: *mut ast_t = 0 as *mut ast_t;
    let mut node_29: *mut ast_t = 0 as *mut ast_t;
    node_29 = ast_from(basis_ast_8, TK_CALL);
    if parent_29.is_null() {
        parent_29 = node_29;
    } else if last_sibling_29.is_null() {
        last_sibling_29 = ast_add(parent_29, node_29);
    } else {
        last_sibling_29 = ast_add_sibling(last_sibling_29, node_29);
    }
    let mut parent_30: *mut ast_t = node_29;
    let mut last_sibling_30: *mut ast_t = 0 as *mut ast_t;
    let mut node_30: *mut ast_t = 0 as *mut ast_t;
    if parent_30.is_null() {
        parent_30 = call_receiver;
    } else if last_sibling_30.is_null() {
        last_sibling_30 = ast_add(parent_30, call_receiver);
    } else {
        last_sibling_30 = ast_add_sibling(last_sibling_30, call_receiver);
    }
    if parent_30.is_null() {
        parent_30 = lambda_call_args;
    } else if last_sibling_30.is_null() {
        last_sibling_30 = ast_add(parent_30, lambda_call_args);
    } else {
        last_sibling_30 = ast_add_sibling(last_sibling_30, lambda_call_args);
    }
    if parent_30.is_null() {
        parent_30 = ast_from(basis_ast_8, TK_NONE);
    } else if last_sibling_30.is_null() {
        last_sibling_30 = ast_add(parent_30, ast_from(basis_ast_8, TK_NONE));
    } else {
        last_sibling_30 = ast_add_sibling(last_sibling_30, ast_from(basis_ast_8, TK_NONE));
    }
    node_30 = ast_from(basis_ast_8, can_error);
    if parent_30.is_null() {
        parent_30 = node_30;
    } else if last_sibling_30.is_null() {
        last_sibling_30 = ast_add(parent_30, node_30);
    } else {
        last_sibling_30 = ast_add_sibling(last_sibling_30, node_30);
    }
    let mut parent_31: *mut ast_t = node_30;
    let mut _last_sibling_31: *mut ast_t = 0 as *mut ast_t;
    let mut _node_31: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_31);
    ast_inheritflags(parent_30);
    ast_inheritflags(parent_29);
    if parent_26.is_null() {
        parent_26 = ast_from(basis_ast_8, TK_NONE);
    } else if last_sibling_26.is_null() {
        last_sibling_26 = ast_add(parent_26, ast_from(basis_ast_8, TK_NONE));
    } else {
        last_sibling_26 = ast_add_sibling(last_sibling_26, ast_from(basis_ast_8, TK_NONE));
    }
    ast_inheritflags(parent_26);
    ast_replace(astp, parent_25);
    ast_setflag(
        ast_childidx(*astp, 2 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(*astp, 3 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(*astp, 5 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(*astp, 7 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    return ast_passes_subtree(astp, opt, PASS_EXPR);
}
#[c2rust::src_loc = "988:1"]
unsafe extern "C" fn method_chain(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if !method_application(opt, ast, 0 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    let mut type_0: *mut ast_t = ast_type(lhs);
    if ast_id(ast_child(type_0)) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a bare method cannot be chained\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut r_type: *mut ast_t = method_receiver_type(lhs);
    if ast_id(lhs) as libc::c_uint == TK_FUNCHAIN as libc::c_int as libc::c_uint {
        let mut recovered: bool = false;
        if !check_receiver_cap(opt, ast, &mut recovered) {
            return 0 as libc::c_int != 0;
        }
        if !check_nonsendable_recover(opt, ast) {
            return 0 as libc::c_int != 0;
        }
        let mut f_type: *mut ast_t = ast_type(lhs);
        let mut f_cap: token_id = ast_id(ast_child(f_type));
        let mut c_type: *mut ast_t = chain_type(r_type, f_cap, recovered);
        ast_settype(ast, c_type);
    } else {
        ast_settype(ast, r_type);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1026:1"]
pub unsafe extern "C" fn expr_call(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) -> bool {
    let mut ast: *mut ast_t = *astp;
    if !literal_call(ast, opt) {
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(ast);
    if !type_0.is_null()
        && ast_id(type_0) as libc::c_uint != TK_INFERTYPE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut namedargs,
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
    match ast_id(lhs) as libc::c_uint {
        188 | 189 | 190 | 191 => return method_call(opt, ast),
        200 | 201 | 202 => return partial_application(opt, astp),
        203 | 204 => return method_chain(opt, ast),
        _ => {}
    }
    insert_apply(opt, astp)
}
