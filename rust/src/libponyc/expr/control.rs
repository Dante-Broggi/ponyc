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
    use super::token_h::{token_id, TK_EOF};
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
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
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "114:1"]
        pub fn ast_childlast(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "141:1"]
        pub fn ast_remove(ast: *mut ast_t);
        #[c2rust::src_loc = "143:1"]
        pub fn ast_replace(prev: *mut *mut ast_t, next: *mut ast_t);
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
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:3"]
pub mod literal_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn make_literal_type(ast: *mut ast_t);
        #[c2rust::src_loc = "20:1"]
        pub fn is_type_literal(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "25:1"]
        pub fn coerce_literals(
            astp: *mut *mut ast_t,
            target_type: *mut ast_t,
            options: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "44:1"]
        pub fn literal_unify_control(ast: *mut ast_t, options: *mut pass_opt_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:8"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn is_typecheck_error(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/ifdef.h:9"]
pub mod ifdef_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn ifdef_cond_eval(ast: *mut ast_t, opt: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:10"]
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
        #[c2rust::src_loc = "44:1"]
        pub fn control_type_add_branch(
            opt: *mut pass_opt_t,
            control_type: *mut ast_t,
            branch: *mut ast_t,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:11"]
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
        #[c2rust::src_loc = "36:1"]
        pub fn is_none(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "42:1"]
        pub fn is_bool(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:12"]
pub mod alias_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:14"]
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
use self::alias_h::{consume_type, recover_type};
use self::assemble_h::{control_type_add_branch, type_builtin};
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_checkflag, ast_child, ast_childidx, ast_childlast, ast_error,
    ast_error_continue, ast_error_frame, ast_free_unattached, ast_from, ast_get_children, ast_id,
    ast_inheritflags, ast_parent, ast_print_type, ast_ptr_t, ast_remove, ast_replace, ast_setid,
    ast_settype, ast_sibling, ast_type, C2RustUnnamed, AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI,
    AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND, AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2,
    AST_FLAG_ERROR_1, AST_FLAG_ERROR_2, AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT,
    AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS, AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK,
    AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI, AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE,
    AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
pub use self::error_h::{errorframe_append, errorframe_report, errorframe_t, errormsg_t, errors_t};
use self::expr_h::is_typecheck_error;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::ifdef_h::ifdef_cond_eval;
use self::literal_h::{coerce_literals, is_type_literal, literal_unify_control, make_literal_type};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::stringtab_h::strlist_t;
use self::subtype_h::{is_bool, is_none, is_subtype};
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
#[no_mangle]
#[c2rust::src_loc = "16:1"]
pub unsafe extern "C" fn expr_seq(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            18 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"expr_seq\0")).as_ptr(),
        );
    };
    let mut parent: *mut ast_t = ast_parent(ast);
    if ast_id(parent) as libc::c_uint == TK_ARRAY as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut p: *mut ast_t = ast_child(ast);
    while !(ast_sibling(p)).is_null() {
        let mut p_type: *mut ast_t = ast_type(p);
        if is_typecheck_error(p_type) {
            ok = 0 as libc::c_int != 0;
        } else if is_type_literal(p_type) {
            ast_error(
                (*opt).check.errors,
                p,
                b"Cannot infer type of unused literal\0" as *const u8 as *const libc::c_char,
            );
            ok = 0 as libc::c_int != 0;
        }
        p = ast_sibling(p);
    }
    if !ok {
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(ast);
    let mut last: *mut ast_t = ast_childlast(ast);
    if !type_0.is_null() && !coerce_literals(&mut last, type_0, opt) {
        return 0 as libc::c_int != 0;
    }
    type_0 = control_type_add_branch(opt, type_0, last);
    ast_settype(ast, type_0);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn resolve_ifdef(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            62 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"resolve_ifdef\0"))
                .as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut then_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut else_cond: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut cond,
        &mut then_clause,
        &mut else_clause,
        &mut else_cond,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut then_value: bool = ifdef_cond_eval(cond, opt);
    ast_setid(ast, TK_IF);
    let mut basis_ast: *mut ast_t = cond;
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
    node_0 = ast_from(
        basis_ast,
        (if then_value as libc::c_int != 0 {
            TK_TRUE as libc::c_int
        } else {
            TK_FALSE as libc::c_int
        }) as token_id,
    );
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
    ast_inheritflags(parent_0);
    ast_replace(&mut cond, parent);
    ast_remove(else_cond);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn expr_if(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_IF as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_IFDEF as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_IF) || (ast_id(ast) == TK_IFDEF)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            84 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"expr_if\0")).as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut cond, &mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(ast) as libc::c_uint == TK_IF as libc::c_int as libc::c_uint {
        let mut cond_type: *mut ast_t = ast_type(cond);
        if is_typecheck_error(cond_type) {
            return 0 as libc::c_int != 0;
        }
        if !is_bool(cond_type) {
            ast_error(
                (*opt).check.errors,
                cond,
                b"condition must be a Bool\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    let mut type_0: *mut ast_t = 0 as *mut ast_t;
    if ast_checkflag(left, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(left)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, left);
    }
    if ast_checkflag(right, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(right)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, right);
    }
    if type_0.is_null() {
        if ast_id(ast_parent(ast)) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
            && !(ast_sibling(ast)).is_null()
        {
            ast_error(
                (*opt).check.errors,
                ast_sibling(ast),
                b"unreachable code\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    if ast_id(ast) as libc::c_uint == TK_IFDEF as libc::c_int as libc::c_uint {
        return resolve_ifdef(opt, ast);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn expr_iftype(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_IFTYPE_SET as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_IFTYPE_SET\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            139 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_iftype\0")).as_ptr(),
        );
    };
    let mut left_control: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left_control, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut sub: ast_ptr_t = 0 as *mut ast_t;
    let mut super_0: ast_ptr_t = 0 as *mut ast_t;
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 4] =
        [&mut sub, &mut super_0, &mut left, 0 as *mut *mut ast_t];
    ast_get_children(
        left_control,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children_0.as_mut_ptr(),
    );
    let mut type_0: *mut ast_t = 0 as *mut ast_t;
    if ast_checkflag(left, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(left)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, left);
    }
    if ast_checkflag(right, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(right)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, right);
    }
    if type_0.is_null() {
        if ast_id(ast_parent(ast)) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
            && !(ast_sibling(ast)).is_null()
        {
            ast_error(
                (*opt).check.errors,
                ast_sibling(ast),
                b"unreachable code\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "176:1"]
pub unsafe extern "C" fn expr_while(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_WHILE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_WHILE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            178 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"expr_while\0")).as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut cond, &mut body, &mut else_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut cond_type: *mut ast_t = ast_type(cond);
    if is_typecheck_error(cond_type) {
        return 0 as libc::c_int != 0;
    }
    if !is_bool(cond_type) {
        ast_error(
            (*opt).check.errors,
            cond,
            b"condition must be a Bool\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(ast);
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(body)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(else_clause)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, else_clause);
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "217:1"]
pub unsafe extern "C" fn expr_repeat(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_REPEAT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_REPEAT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            219 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_repeat\0")).as_ptr(),
        );
    };
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut body, &mut cond, &mut else_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut cond_type: *mut ast_t = ast_type(cond);
    if is_typecheck_error(cond_type) {
        return 0 as libc::c_int != 0;
    }
    if !is_bool(cond_type) {
        ast_error(
            (*opt).check.errors,
            cond,
            b"condition must be a Bool\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t = ast_type(ast);
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(body)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(else_clause)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, else_clause);
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "258:1"]
pub unsafe extern "C" fn expr_try(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_TRY as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_TRY_NO_CHECK as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_TRY) || (ast_id(ast) == TK_TRY_NO_CHECK)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            260 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"expr_try\0")).as_ptr(),
        );
    };
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut then_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [
        &mut body,
        &mut else_clause,
        &mut then_clause,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut type_0: *mut ast_t = 0 as *mut ast_t;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(body)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(else_clause)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, else_clause);
    }
    if type_0.is_null() && !(ast_sibling(ast)).is_null() {
        ast_error(
            (*opt).check.errors,
            ast_sibling(ast),
            b"unreachable code\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut then_type: *mut ast_t = ast_type(then_clause);
    if is_typecheck_error(then_type) {
        return 0 as libc::c_int != 0;
    }
    if is_type_literal(then_type) {
        ast_error(
            (*opt).check.errors,
            then_clause,
            b"Cannot infer type of unused literal\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "306:1"]
pub unsafe extern "C" fn expr_disposing_block(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DISPOSING_BLOCK as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DISPOSING_BLOCK\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"expr_disposing_block\0"))
                .as_ptr(),
        );
    };
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut dispose_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut body, &mut dispose_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut type_0: *mut ast_t = 0 as *mut ast_t;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) == 0 {
        if is_typecheck_error(ast_type(body)) {
            return 0 as libc::c_int != 0;
        }
        type_0 = control_type_add_branch(opt, type_0, body);
    }
    if type_0.is_null() && !(ast_sibling(ast)).is_null() {
        ast_error(
            (*opt).check.errors,
            ast_sibling(ast),
            b"unreachable code\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut dispose_type: *mut ast_t = ast_type(dispose_clause);
    if is_typecheck_error(dispose_type) {
        return 0 as libc::c_int != 0;
    }
    if is_type_literal(dispose_type) {
        ast_error(
            (*opt).check.errors,
            dispose_clause,
            b"Cannot infer type of unused literal\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, type_0);
    literal_unify_control(ast, opt);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "346:1"]
pub unsafe extern "C" fn expr_recover(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_RECOVER as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_RECOVER\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            348 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"expr_recover\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut cap, &mut expr, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut type_0: *mut ast_t = ast_type(expr);
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    if is_type_literal(type_0) {
        make_literal_type(ast);
        return 1 as libc::c_int != 0;
    }
    let mut r_type: *mut ast_t = recover_type(type_0, ast_id(cap));
    if r_type.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can't recover to this capability\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            expr,
            b"expression type is %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(type_0),
        );
        return 0 as libc::c_int != 0;
    }
    ast_settype(ast, r_type);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "376:1"]
pub unsafe extern "C" fn expr_break(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_BREAK as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_BREAK\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            378 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"expr_break\0")).as_ptr(),
        );
    };
    let mut body: *mut ast_t = ast_child(ast);
    if ast_id(body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) != 0 {
            ast_error(
                (*opt).check.errors,
                body,
                b"break value cannot be an expression that jumps away with no value\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        let mut loop_type: *mut ast_t = ast_type((*(*opt).check.frame).loop_0);
        loop_type = control_type_add_branch(opt, loop_type, body);
        ast_settype((*(*opt).check.frame).loop_0, loop_type);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "403:1"]
pub unsafe extern "C" fn is_local_or_param(mut ast: *mut ast_t) -> bool {
    while ast_id(ast) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        ast = ast_child(ast);
        if !(ast_sibling(ast)).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    match ast_id(ast) as libc::c_uint {
        198 | 197 | 196 => return 1 as libc::c_int != 0,
        184 => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0"
                        as *const u8 as *const libc::c_char,
                    419 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"is_local_or_param\0",
                    ))
                    .as_ptr(),
                );
            };
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "426:1"]
pub unsafe extern "C" fn expr_return(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_RETURN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_RETURN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0" as *const u8
                as *const libc::c_char,
            428 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_return\0")).as_ptr(),
        );
    };
    let mut parent: *mut ast_t = ast_parent(ast);
    let mut current: *mut ast_t = ast;
    while ast_id(parent) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        if ast_childlast(parent) == current {
        } else {
            ponyint_assert_fail(
                b"ast_childlast(parent) == current\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0"
                    as *const u8 as *const libc::c_char,
                435 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_return\0"))
                    .as_ptr(),
            );
        };
        current = parent;
        parent = ast_parent(parent);
    }
    if current == (*(*opt).check.frame).method_body {
        ast_error(
            (*opt).check.errors,
            ast,
            b"use return only to exit early from a method, not at the end\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut type_0: *mut ast_t =
        ast_childidx((*(*opt).check.frame).method, 4 as libc::c_int as size_t);
    let mut body: *mut ast_t = ast_child(ast);
    if !coerce_literals(&mut body, type_0, opt) {
        return 0 as libc::c_int != 0;
    }
    let mut body_type: *mut ast_t = ast_type(body);
    if is_typecheck_error(body_type) {
        return 0 as libc::c_int != 0;
    }
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as uint32_t) != 0 {
        ast_error(
            (*opt).check.errors,
            body,
            b"return value cannot be a control statement\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    match ast_id((*(*opt).check.frame).method) as libc::c_uint {
        88 => {
            if ast_id(ast_child(ast)) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(ast_child(ast)) == TK_THIS\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0"
                        as *const u8 as *const libc::c_char,
                    470 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_return\0"))
                        .as_ptr(),
                );
            };
        }
        90 => {
            if is_none(body_type) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"is_none(body_type)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.c\0"
                        as *const u8 as *const libc::c_char,
                    474 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_return\0"))
                        .as_ptr(),
                );
            };
        }
        _ => {
            let mut info: errorframe_t = 0 as errorframe_t;
            let mut r_type: *mut ast_t = 0 as *mut ast_t;
            if is_local_or_param(body) {
                r_type = consume_type(body_type, TK_NONE, 0 as libc::c_int != 0);
                if !r_type.is_null() {
                    body_type = r_type;
                }
            }
            if !is_subtype(body_type, type_0, &mut info, opt) {
                let mut frame: errorframe_t = 0 as errorframe_t;
                let mut last: *mut ast_t = ast_childlast(body);
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    last,
                    b"returned value isn't the return type\0" as *const u8 as *const libc::c_char,
                );
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    type_0,
                    b"function return type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(type_0),
                );
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    body_type,
                    b"returned value type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(body_type),
                );
                errorframe_append(&mut frame, &mut info);
                errorframe_report(&mut frame, (*opt).check.errors);
                ok = 0 as libc::c_int != 0;
            }
            if !r_type.is_null() {
                ast_free_unattached(r_type);
            }
        }
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "518:1"]
pub unsafe extern "C" fn expr_location(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    ast_settype(
        ast,
        type_builtin(opt, ast, b"SourceLoc\0" as *const u8 as *const libc::c_char),
    );
    return 1 as libc::c_int != 0;
}
