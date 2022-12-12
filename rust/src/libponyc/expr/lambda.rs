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
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::error_h::{errorframe_t, errors_t};
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "54:23"]
        pub type astlist_t;
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "54:23"]
        pub fn astlist_push(list: *mut astlist_t, data: *mut ast_t) -> *mut astlist_t;
        #[c2rust::src_loc = "54:23"]
        pub fn astlist_next(list: *mut astlist_t) -> *mut astlist_t;
        #[c2rust::src_loc = "54:34"]
        pub fn astlist_data(list: *mut astlist_t) -> *mut ast_t;
        #[c2rust::src_loc = "54:1"]
        pub fn astlist_length(list: *mut astlist_t) -> size_t;
        #[c2rust::src_loc = "54:1"]
        pub fn astlist_free(list: *mut astlist_t);
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
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
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: uint32_t) -> libc::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "90:1"]
        pub fn ast_clearflag(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "91:1"]
        pub fn ast_resetpass(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "103:1"]
        pub fn ast_setannotation(ast: *mut ast_t, annotation: *mut ast_t);
        #[c2rust::src_loc = "104:1"]
        pub fn ast_consumeannotation(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "105:1"]
        pub fn ast_has_annotation(ast: *mut ast_t, name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "133:1"]
        pub fn ast_clear(ast: *mut ast_t);
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "140:1"]
        pub fn ast_list_append(
            parent: *mut ast_t,
            last_pointer: *mut *mut ast_t,
            new_child: *mut ast_t,
        ) -> *mut ast_t;
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
    #[c2rust::src_loc = "402:1"]
    pub type ast_visit_t =
        Option<unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t>;
    use super::_size_t_h::size_t;
    use super::ast_h::ast_result_t;
    use super::frame_h::typecheck_t;
    use super::stringtab_h::strlist_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "409:1"]
        pub fn ast_visit(
            ast: *mut *mut ast_t,
            pre: ast_visit_t,
            post: ast_visit_t,
            options: *mut pass_opt_t,
            pass: pass_id,
        ) -> ast_result_t;
        #[c2rust::src_loc = "399:1"]
        pub fn ast_pass_record(ast: *mut ast_t, pass: pass_id);
        #[c2rust::src_loc = "390:1"]
        pub fn ast_passes_subtree(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
        #[c2rust::src_loc = "377:1"]
        pub fn ast_passes_type(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:5"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn is_name_dontcare(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:7"]
pub mod expr_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn is_typecheck_error(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn find_antecedent_type(
            opt: *mut pass_opt_t,
            ast: *mut ast_t,
            is_recovered: *mut bool,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.h:9"]
pub mod refer_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn def_before_use(
            opt: *mut pass_opt_t,
            def: *mut ast_t,
            use_0: *mut ast_t,
            name: *const libc::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.h:10"]
pub mod syntax_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn pass_syntax(astp: *mut *mut ast_t, options: *mut pass_opt_t) -> ast_result_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:11"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:12"]
pub mod assemble_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "60:1"]
        pub fn type_for_class(
            opt: *mut pass_opt_t,
            def: *mut ast_t,
            ast: *mut ast_t,
            cap: token_id,
            ephemeral: token_id,
            expr: bool,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:13"]
pub mod cap_h {
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn is_cap_sub_cap(
            sub: token_id,
            subalias: token_id,
            super_0: token_id,
            supalias: token_id,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:14"]
pub mod reify_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn reify_method_def(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/sanitise.h:15"]
pub mod sanitise_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn collect_type_params(
            ast: *mut ast_t,
            out_params: *mut *mut ast_t,
            out_args: *mut *mut ast_t,
        );
        #[c2rust::src_loc = "39:1"]
        pub fn sanitise_type(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:16"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:17"]
pub mod package_h {
    use super::frame_h::typecheck_t;
    extern "C" {
        #[c2rust::src_loc = "143:1"]
        pub fn package_hygienic_id(t: *mut typecheck_t) -> *const libc::c_char;
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
use self::alias_h::{alias, consume_type};
use self::assemble_h::type_for_class;
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_checkflag, ast_child, ast_childcount, ast_childidx,
    ast_clear, ast_clearflag, ast_consumeannotation, ast_data, ast_error, ast_error_continue,
    ast_error_frame, ast_free, ast_free_unattached, ast_from, ast_from_string, ast_get,
    ast_get_children, ast_has_annotation, ast_id, ast_inheritflags, ast_list_append, ast_name,
    ast_nearest, ast_print_type, ast_ptr_t, ast_replace, ast_resetpass, ast_result_t, ast_scope,
    ast_setannotation, ast_setdata, ast_setflag, ast_setid, ast_sibling, ast_type, astlist_data,
    astlist_free, astlist_length, astlist_next, astlist_push, astlist_t, C2RustUnnamed, AST_ERROR,
    AST_FATAL, AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2, AST_IGNORE,
    AST_OK,
};
use self::cap_h::is_cap_sub_cap;
pub use self::error_h::{errorframe_append, errorframe_report, errorframe_t, errormsg_t, errors_t};
use self::expr_h::{find_antecedent_type, is_typecheck_error};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::is_name_dontcare;
use self::literal_h::coerce_literals;
use self::package_h::package_hygienic_id;
pub use self::pass_h::{
    ast_pass_record, ast_passes_subtree, ast_passes_type, ast_visit, ast_visit_t, magic_package_t,
    pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE,
    PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT,
    PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER,
    PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL,
    VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::printbuf_h::{printbuf, printbuf_free, printbuf_new, printbuf_t};
use self::refer_h::def_before_use;
use self::reify_h::reify_method_def;
use self::sanitise_h::{collect_type_params, sanitise_type};
use self::stringtab_h::stringtab;
use self::subtype_h::is_subtype;
pub use self::symtab_h::{
    ast_t, sym_status_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL,
    SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};
use self::syntax_h::pass_syntax;
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
#[c2rust::src_loc = "25:1"]
unsafe extern "C" fn make_capture_field(
    mut opt: *mut pass_opt_t,
    mut capture: *mut ast_t,
    mut out_field: *mut *mut ast_t,
) -> bool {
    if !capture.is_null() {
    } else {
        ponyint_assert_fail(
            b"capture != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0" as *const u8
                as *const libc::c_char,
            28 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"make_capture_field\0"))
                .as_ptr(),
        );
    };
    if !out_field.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_field != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0" as *const u8
                as *const libc::c_char,
            29 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"make_capture_field\0"))
                .as_ptr(),
        );
    };
    let mut id_node: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut value: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut id_node, &mut type_0, &mut value, 0 as *mut *mut ast_t];
    ast_get_children(
        capture,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut name: *const libc::c_char = ast_name(id_node);
    let mut is_dontcare: bool = is_name_dontcare(name);
    if ast_id(value) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        if ast_id(type_0) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(type) == TK_NONE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0"
                    as *const u8 as *const libc::c_char,
                44 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"make_capture_field\0",
                ))
                .as_ptr(),
            );
        };
        if is_dontcare {
            *out_field = 0 as *mut ast_t;
            return 1 as libc::c_int != 0;
        }
        let mut def: *mut ast_t = ast_get(capture, name, 0 as *mut sym_status_t);
        if def.is_null() {
            ast_error(
                (*opt).check.errors,
                id_node,
                b"cannot capture \"%s\", variable not defined\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            return 0 as libc::c_int != 0;
        }
        if !def_before_use(opt, def, capture, name) {
            return 0 as libc::c_int != 0;
        }
        match ast_id(def) as libc::c_uint {
            84 | 85 | 165 | 182 | 140 | 141 | 86 => {}
            _ => {
                ast_error(
                    (*opt).check.errors,
                    id_node,
                    b"cannot capture \"%s\", can only capture fields, parameters and local variables\0"
                        as *const u8 as *const libc::c_char,
                    name,
                );
                return 0 as libc::c_int != 0;
            }
        }
        let mut capture_rhs: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = id_node;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_REFERENCE);
        if parent.is_null() {
            parent = node;
        } else if last_sibling.is_null() {
            last_sibling = ast_add(parent, node);
        } else {
            last_sibling = ast_add_sibling(last_sibling, node);
        }
        let mut parent_0: *mut ast_t = node;
        let mut last_sibling_0: *mut ast_t = 0 as *mut ast_t;
        let mut _node_0: *mut ast_t = 0 as *mut ast_t;
        if parent_0.is_null() {
            parent_0 = ast_from_string(basis_ast, name);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from_string(basis_ast, name));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from_string(basis_ast, name));
        }
        ast_inheritflags(parent_0);
        capture_rhs = parent;
        type_0 = alias(ast_type(def));
        value = capture_rhs;
    } else if ast_id(type_0) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        if (ast_type(value)).is_null() {
            return 0 as libc::c_int != 0;
        }
        type_0 = alias(ast_type(value));
    } else {
        if !coerce_literals(&mut value, type_0, opt) {
            return 0 as libc::c_int != 0;
        }
        if is_dontcare {
            let mut p_type: *mut ast_t = consume_type(type_0, TK_NONE, 0 as libc::c_int != 0);
            let mut v_type: *mut ast_t = ast_type(value);
            let mut info: errorframe_t = 0 as errorframe_t;
            let mut frame: errorframe_t = 0 as errorframe_t;
            if p_type.is_null() {
                ast_error_frame(
                    &mut frame as *mut errorframe_t,
                    type_0,
                    b"invalid parameter type: %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(type_0),
                );
                errorframe_append(&mut frame, &mut info);
                errorframe_report(&mut frame, (*opt).check.errors);
                ast_free_unattached(p_type);
                return 0 as libc::c_int != 0;
            } else {
                if !is_subtype(v_type, p_type, &mut info, opt) {
                    ast_error_frame(
                        &mut frame as *mut errorframe_t,
                        value,
                        b"argument not assignable to parameter\0" as *const u8
                            as *const libc::c_char,
                    );
                    ast_error_frame(
                        &mut frame as *mut errorframe_t,
                        value,
                        b"argument type is %s\0" as *const u8 as *const libc::c_char,
                        ast_print_type(v_type),
                    );
                    ast_error_frame(
                        &mut frame as *mut errorframe_t,
                        id_node,
                        b"parameter type requires %s\0" as *const u8 as *const libc::c_char,
                        ast_print_type(p_type),
                    );
                    errorframe_append(&mut frame, &mut info);
                    errorframe_report(&mut frame, (*opt).check.errors);
                    ast_free_unattached(p_type);
                    return 0 as libc::c_int != 0;
                }
            }
            ast_free_unattached(p_type);
        }
    }
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    if is_dontcare {
        *out_field = 0 as *mut ast_t;
        return 1 as libc::c_int != 0;
    }
    type_0 = sanitise_type(type_0);
    let mut field: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_0: *mut ast_t = id_node;
    let mut parent_1: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut node_1: *mut ast_t = 0 as *mut ast_t;
    node_1 = ast_from(basis_ast_0, TK_FVAR);
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
        parent_2 = id_node;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, id_node);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, id_node);
    }
    if parent_2.is_null() {
        parent_2 = type_0;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, type_0);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, type_0);
    }
    if parent_2.is_null() {
        parent_2 = value;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, value);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, value);
    }
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
    }
    ast_inheritflags(parent_2);
    field = parent_1;
    *out_field = field;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "157:1"]
unsafe extern "C" fn find_possible_fun_defs(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut fun_defs: *mut *mut astlist_t,
    mut obj_caps: *mut *mut astlist_t,
) {
    match ast_id(ast) as libc::c_uint {
        151 => {
            let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
            if ast_id(def) as libc::c_uint != TK_INTERFACE as libc::c_int as libc::c_uint {
                return;
            }
            let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as size_t);
            if ast_id(members) as libc::c_uint == TK_MEMBERS as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(members) == TK_MEMBERS\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0"
                        as *const u8 as *const libc::c_char,
                    171 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"find_possible_fun_defs\0",
                    ))
                    .as_ptr(),
                );
            };
            if ast_childcount(members) != 1 as libc::c_int as libc::c_ulong {
                return;
            }
            let mut fun_def: *mut ast_t = ast_child(members);
            let mut typeargs: *mut ast_t = ast_childidx(ast, 2 as libc::c_int as size_t);
            let mut typeparams: *mut ast_t = ast_childidx(def, 1 as libc::c_int as size_t);
            if ast_id(typeargs) as libc::c_uint == TK_TYPEARGS as libc::c_int as libc::c_uint
                && ast_id(typeparams) as libc::c_uint
                    == TK_TYPEPARAMS as libc::c_int as libc::c_uint
            {
                fun_def = reify_method_def(fun_def, typeparams, typeargs, opt);
            }
            *obj_caps = astlist_push(*obj_caps, ast_childidx(ast, 3 as libc::c_int as size_t));
            *fun_defs = astlist_push(*fun_defs, fun_def);
        }
        17 => {
            find_possible_fun_defs(
                opt,
                ast_childidx(ast, 1 as libc::c_int as size_t),
                fun_defs,
                obj_caps,
            );
        }
        187 => {
            let mut def_0: *mut ast_t = ast_data(ast) as *mut ast_t;
            if ast_id(def_0) as libc::c_uint == TK_TYPEPARAM as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(def) == TK_TYPEPARAM\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0"
                        as *const u8 as *const libc::c_char,
                    199 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"find_possible_fun_defs\0",
                    ))
                    .as_ptr(),
                );
            };
            find_possible_fun_defs(
                opt,
                ast_childidx(def_0, 1 as libc::c_int as size_t),
                fun_defs,
                obj_caps,
            );
        }
        149 | 56 => {
            let mut c: *mut ast_t = ast_child(ast);
            while !c.is_null() {
                find_possible_fun_defs(opt, c, fun_defs, obj_caps);
                c = ast_sibling(c);
            }
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "218:1"]
pub unsafe extern "C" fn expr_lambda(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) -> bool {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0" as *const u8
                as *const libc::c_char,
            220 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_lambda\0")).as_ptr(),
        );
    };
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0" as *const u8
                as *const libc::c_char,
            222 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_lambda\0")).as_ptr(),
        );
    };
    let mut receiver_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut name: ast_ptr_t = 0 as *mut ast_t;
    let mut t_params: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut captures: ast_ptr_t = 0 as *mut ast_t;
    let mut ret_type: ast_ptr_t = 0 as *mut ast_t;
    let mut raises: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut obj_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 10] = [
        &mut receiver_cap,
        &mut name,
        &mut t_params,
        &mut params,
        &mut captures,
        &mut ret_type,
        &mut raises,
        &mut body,
        &mut obj_cap,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut annotation: *mut ast_t = ast_consumeannotation(ast);
    let mut antecedent_type: *mut ast_t = find_antecedent_type(opt, ast, 0 as *mut bool);
    let mut possible_fun_defs: *mut astlist_t = 0 as *mut astlist_t;
    let mut possible_obj_caps: *mut astlist_t = 0 as *mut astlist_t;
    if !is_typecheck_error(antecedent_type) {
        find_possible_fun_defs(
            opt,
            antecedent_type,
            &mut possible_fun_defs,
            &mut possible_obj_caps,
        );
    }
    if astlist_length(possible_fun_defs) > 1 as libc::c_int as libc::c_ulong {
        let mut new_fun_defs: *mut astlist_t = 0 as *mut astlist_t;
        let mut new_obj_caps: *mut astlist_t = 0 as *mut astlist_t;
        let mut fun_def_cursor: *mut astlist_t = possible_fun_defs;
        let mut obj_cap_cursor: *mut astlist_t = possible_obj_caps;
        while !fun_def_cursor.is_null() && !obj_cap_cursor.is_null() {
            let mut fun_def: *mut ast_t = astlist_data(fun_def_cursor);
            let mut def_obj_cap: *mut ast_t = astlist_data(obj_cap_cursor);
            if !is_typecheck_error(fun_def) {
                let mut def_receiver_cap: ast_ptr_t = 0 as *mut ast_t;
                let mut def_name: ast_ptr_t = 0 as *mut ast_t;
                let mut def_t_params: ast_ptr_t = 0 as *mut ast_t;
                let mut def_params: ast_ptr_t = 0 as *mut ast_t;
                let mut def_ret_type: ast_ptr_t = 0 as *mut ast_t;
                let mut def_raises: ast_ptr_t = 0 as *mut ast_t;
                let mut children_0: [*mut *mut ast_t; 7] = [
                    &mut def_receiver_cap,
                    &mut def_name,
                    &mut def_t_params,
                    &mut def_params,
                    &mut def_ret_type,
                    &mut def_raises,
                    0 as *mut *mut ast_t,
                ];
                ast_get_children(
                    fun_def,
                    (::core::mem::size_of::<[*mut *mut ast_t; 7]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    children_0.as_mut_ptr(),
                );
                if !(ast_childcount(params) != ast_childcount(def_params)) {
                    if !(ast_id(receiver_cap) as libc::c_uint
                        != TK_NONE as libc::c_int as libc::c_uint
                        && ast_id(receiver_cap) as libc::c_uint
                            != TK_AT as libc::c_int as libc::c_uint
                        && !is_cap_sub_cap(
                            ast_id(def_receiver_cap),
                            TK_NONE,
                            ast_id(receiver_cap),
                            TK_NONE,
                        ))
                    {
                        if !(ast_id(obj_cap) as libc::c_uint
                            != TK_NONE as libc::c_int as libc::c_uint
                            && !is_cap_sub_cap(
                                ast_id(obj_cap),
                                TK_NONE,
                                ast_id(def_obj_cap),
                                TK_NONE,
                            ))
                        {
                            new_fun_defs = astlist_push(new_fun_defs, fun_def);
                            new_obj_caps = astlist_push(new_obj_caps, def_obj_cap);
                        }
                    }
                }
            }
            fun_def_cursor = astlist_next(fun_def_cursor);
            obj_cap_cursor = astlist_next(obj_cap_cursor);
        }
        astlist_free(possible_fun_defs);
        astlist_free(possible_obj_caps);
        possible_fun_defs = new_fun_defs;
        possible_obj_caps = new_obj_caps;
    }
    if astlist_length(possible_fun_defs) == 1 as libc::c_int as libc::c_ulong {
        let mut fun_def_0: *mut ast_t = astlist_data(possible_fun_defs);
        let mut def_obj_cap_0: *mut ast_t = astlist_data(possible_obj_caps);
        if !is_typecheck_error(fun_def_0) {
            if ast_id(obj_cap) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                ast_replace(&mut obj_cap, def_obj_cap_0);
            }
            if ast_id(receiver_cap) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                ast_replace(&mut receiver_cap, ast_child(fun_def_0));
            }
            if ast_id(ret_type) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                ast_replace(
                    &mut ret_type,
                    ast_childidx(fun_def_0, 4 as libc::c_int as size_t),
                );
            }
            let mut param: *mut ast_t = ast_child(params);
            let mut def_param: *mut ast_t =
                ast_child(ast_childidx(fun_def_0, 3 as libc::c_int as size_t));
            while !param.is_null() && !def_param.is_null() {
                let mut param_id: *mut ast_t = ast_child(param);
                let mut param_type: *mut ast_t = ast_sibling(param_id);
                if is_name_dontcare(ast_name(param_id)) {
                    ast_replace(&mut param_id, ast_child(def_param));
                    ast_replace(
                        &mut param_type,
                        ast_childidx(def_param, 1 as libc::c_int as size_t),
                    );
                } else if ast_id(param_type) as libc::c_uint
                    == TK_NONE as libc::c_int as libc::c_uint
                {
                    ast_replace(
                        &mut param_type,
                        ast_childidx(def_param, 1 as libc::c_int as size_t),
                    );
                }
                param = ast_sibling(param);
                def_param = ast_sibling(def_param);
            }
        }
        ast_free_unattached(fun_def_0);
    }
    astlist_free(possible_obj_caps);
    let mut param_0: *mut ast_t = ast_child(params);
    while !param_0.is_null() {
        if ast_id(ast_childidx(param_0, 1 as libc::c_int as size_t)) as libc::c_uint
            == TK_NONE as libc::c_int as libc::c_uint
        {
            ast_error(
                (*opt).check.errors,
                param_0,
                b"a lambda parameter must specify a type or be inferable from context\0"
                    as *const u8 as *const libc::c_char,
            );
            if astlist_length(possible_fun_defs) > 1 as libc::c_int as libc::c_ulong {
                let mut fun_def_cursor_0: *mut astlist_t = possible_fun_defs;
                while !fun_def_cursor_0.is_null() {
                    ast_error_continue(
                        (*opt).check.errors,
                        astlist_data(fun_def_cursor_0),
                        b"this lambda interface is inferred, but it's not the only one\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fun_def_cursor_0 = astlist_next(fun_def_cursor_0);
                }
            }
            astlist_free(possible_fun_defs);
            return 0 as libc::c_int != 0;
        }
        param_0 = ast_sibling(param_0);
    }
    astlist_free(possible_fun_defs);
    let mut bare: bool =
        ast_id(ast) as libc::c_uint == TK_BARELAMBDA as libc::c_int as libc::c_uint;
    let mut members: *mut ast_t = ast_from(ast, TK_MEMBERS);
    let mut last_member: *mut ast_t = 0 as *mut ast_t;
    let mut failed: bool = 0 as libc::c_int != 0;
    if bare {
        if ast_id(captures) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(captures) == TK_NONE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0"
                    as *const u8 as *const libc::c_char,
                369 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_lambda\0"))
                    .as_ptr(),
            );
        };
    }
    let mut p: *mut ast_t = ast_child(captures);
    while !p.is_null() {
        let mut field: *mut ast_t = 0 as *mut ast_t;
        let mut ok: bool = make_capture_field(opt, p, &mut field);
        if !field.is_null() {
            ast_list_append(members, &mut last_member, field);
        } else if !ok {
            failed = 1 as libc::c_int != 0;
        }
        p = ast_sibling(p);
    }
    if failed {
        ast_free(members);
        return 0 as libc::c_int != 0;
    }
    ast_clearflag(t_params, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    ast_clearflag(params, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    ast_clearflag(ret_type, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    ast_clearflag(body, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    let mut fn_name: *const libc::c_char = b"apply\0" as *const u8 as *const libc::c_char;
    if ast_id(name) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
        fn_name = ast_name(name);
    }
    let mut apply: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_FUN);
    if parent.is_null() {
        parent = node;
    } else if last_sibling.is_null() {
        last_sibling = ast_add(parent, node);
    } else {
        last_sibling = ast_add_sibling(last_sibling, node);
    }
    let mut parent_0: *mut ast_t = node;
    let mut last_sibling_0: *mut ast_t = 0 as *mut ast_t;
    let mut _node_0: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_0);
    ast_setannotation(parent_0, annotation);
    if parent_0.is_null() {
        parent_0 = receiver_cap;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, receiver_cap);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, receiver_cap);
    }
    if parent_0.is_null() {
        parent_0 = ast_from_string(basis_ast, fn_name);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from_string(basis_ast, fn_name));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from_string(basis_ast, fn_name));
    }
    if parent_0.is_null() {
        parent_0 = t_params;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, t_params);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, t_params);
    }
    if parent_0.is_null() {
        parent_0 = params;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, params);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, params);
    }
    if parent_0.is_null() {
        parent_0 = ret_type;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ret_type);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ret_type);
    }
    if parent_0.is_null() {
        parent_0 = raises;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, raises);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, raises);
    }
    if parent_0.is_null() {
        parent_0 = body;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, body);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, body);
    }
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    apply = parent;
    ast_list_append(members, &mut last_member, apply);
    ast_setflag(members, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    let mut buf: *mut printbuf_t = printbuf_new();
    printbuf(
        buf,
        if bare as libc::c_int != 0 {
            b"@{(\0" as *const u8 as *const libc::c_char
        } else {
            b"{(\0" as *const u8 as *const libc::c_char
        },
    );
    let mut first: bool = 1 as libc::c_int != 0;
    let mut p_0: *mut ast_t = ast_child(params);
    while !p_0.is_null() {
        if first {
            first = 0 as libc::c_int != 0;
        } else {
            printbuf(buf, b", \0" as *const u8 as *const libc::c_char);
        }
        printbuf(
            buf,
            b"%s\0" as *const u8 as *const libc::c_char,
            ast_print_type(ast_childidx(p_0, 1 as libc::c_int as size_t)),
        );
        p_0 = ast_sibling(p_0);
    }
    printbuf(buf, b")\0" as *const u8 as *const libc::c_char);
    if ast_id(ret_type) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        printbuf(
            buf,
            b": %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(ret_type),
        );
    }
    if ast_id(raises) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        printbuf(buf, b" ?\0" as *const u8 as *const libc::c_char);
    }
    printbuf(buf, b"}\0" as *const u8 as *const libc::c_char);
    let mut basis_ast_0: *mut ast_t = *astp;
    let mut parent_1: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut node_1: *mut ast_t = 0 as *mut ast_t;
    node_1 = ast_from(basis_ast_0, TK_OBJECT);
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
    ast_setdata(parent_2, stringtab((*buf).m) as *mut libc::c_void);
    if parent_2.is_null() {
        parent_2 = obj_cap;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, obj_cap);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, obj_cap);
    }
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
    }
    if parent_2.is_null() {
        parent_2 = members;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, members);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, members);
    }
    ast_inheritflags(parent_2);
    ast_replace(astp, parent_1);
    printbuf_free(buf);
    if bare {
        let mut bare_annotation: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_1: *mut ast_t = *astp;
        let mut parent_3: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
        let mut node_3: *mut ast_t = 0 as *mut ast_t;
        node_3 = ast_from(basis_ast_1, TK_ANNOTATION);
        if parent_3.is_null() {
            parent_3 = node_3;
        } else if last_sibling_3.is_null() {
            last_sibling_3 = ast_add(parent_3, node_3);
        } else {
            last_sibling_3 = ast_add_sibling(last_sibling_3, node_3);
        }
        let mut parent_4: *mut ast_t = node_3;
        let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
        let mut _node_4: *mut ast_t = 0 as *mut ast_t;
        if parent_4.is_null() {
            parent_4 = ast_from_string(
                basis_ast_1,
                b"ponyint_bare\0" as *const u8 as *const libc::c_char,
            );
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(
                parent_4,
                ast_from_string(
                    basis_ast_1,
                    b"ponyint_bare\0" as *const u8 as *const libc::c_char,
                ),
            );
        } else {
            last_sibling_4 = ast_add_sibling(
                last_sibling_4,
                ast_from_string(
                    basis_ast_1,
                    b"ponyint_bare\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        ast_inheritflags(parent_4);
        bare_annotation = parent_3;
        ast_pass_record(bare_annotation, PASS_SYNTAX);
        ast_setannotation(*astp, bare_annotation);
    }
    if ast_visit(
        astp,
        Some(pass_syntax as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
        opt,
        PASS_SYNTAX,
    ) as libc::c_uint
        != AST_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    ast_passes_subtree(astp, opt, PASS_EXPR)
}
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn capture_from_reference(
    mut opt: *mut pass_opt_t,
    mut ctx: *mut ast_t,
    mut ast: *mut ast_t,
    mut captures: *mut ast_t,
    mut last_capture: *mut *mut ast_t,
) -> bool {
    let mut name: *const libc::c_char = ast_name(ast_child(ast));
    if is_name_dontcare(name) {
        return 1 as libc::c_int != 0;
    }
    let mut refdef: *mut ast_t = ast_get(ast, name, 0 as *mut sym_status_t);
    if !refdef.is_null() {
        return 1 as libc::c_int != 0;
    }
    refdef = ast_get(ctx, name, 0 as *mut sym_status_t);
    if refdef.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"cannot capture \"%s\", variable not defined\0" as *const u8 as *const libc::c_char,
            name,
        );
        return 0 as libc::c_int != 0;
    }
    if !def_before_use(opt, refdef, ctx, name) {
        return 0 as libc::c_int != 0;
    }
    match ast_id(refdef) as libc::c_uint {
        84 | 85 | 165 | 182 | 140 | 141 | 86 => {}
        _ => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"cannot capture \"%s\", can only capture fields, parameters and local variables\0"
                    as *const u8 as *const libc::c_char,
                name,
            );
            return !(0 as *mut libc::c_void).is_null();
        }
    }
    let mut p: *mut ast_t = ast_child(captures);
    while !p.is_null() {
        let mut c_name: ast_ptr_t = 0 as *mut ast_t;
        let mut c_type: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut c_name, &mut c_type, 0 as *mut *mut ast_t];
        ast_get_children(
            p,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children.as_mut_ptr(),
        );
        if name == ast_name(c_name) {
            return 1 as libc::c_int != 0;
        }
        p = ast_sibling(p);
    }
    let mut type_0: *mut ast_t = alias(ast_type(refdef));
    if is_typecheck_error(type_0) {
        return 0 as libc::c_int != 0;
    }
    type_0 = sanitise_type(type_0);
    let mut field: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_FVAR);
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
    if parent_0.is_null() {
        parent_0 = ast_from_string(basis_ast, name);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from_string(basis_ast, name));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from_string(basis_ast, name));
    }
    if parent_0.is_null() {
        parent_0 = type_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, type_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, type_0);
    }
    node_0 = ast_from(basis_ast, TK_REFERENCE);
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
        parent_1 = ast_from_string(basis_ast, name);
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, ast_from_string(basis_ast, name));
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, ast_from_string(basis_ast, name));
    }
    ast_inheritflags(parent_1);
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    field = parent;
    ast_list_append(captures, last_capture, field);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "540:1"]
unsafe extern "C" fn capture_from_expr(
    mut opt: *mut pass_opt_t,
    mut ctx: *mut ast_t,
    mut ast: *mut ast_t,
    mut capture: *mut ast_t,
    mut last_capture: *mut *mut ast_t,
) -> bool {
    if ast_checkflag(ast, AST_FLAG_PRESERVE as libc::c_int as uint32_t) != 0 {
        return 1 as libc::c_int != 0;
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    if ast_id(ast) as libc::c_uint == TK_REFERENCE as libc::c_int as libc::c_uint {
        if !capture_from_reference(opt, ctx, ast, capture, last_capture) {
            ok = 0 as libc::c_int != 0;
        }
    } else {
        let mut p: *mut ast_t = ast_child(ast);
        while !p.is_null() {
            if !capture_from_expr(opt, ctx, p, capture, last_capture) {
                ok = 0 as libc::c_int != 0;
            }
            p = ast_sibling(p);
        }
    }
    ok
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn capture_from_type(
    mut opt: *mut pass_opt_t,
    mut ctx: *mut ast_t,
    mut def: *mut *mut ast_t,
    mut capture: *mut ast_t,
    mut last_capture: *mut *mut ast_t,
) -> bool {
    if !ast_passes_type(def, opt, PASS_SCOPE) {
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut members: *mut ast_t = ast_childidx(*def, 4 as libc::c_int as size_t);
    let mut p: *mut ast_t = ast_child(members);
    while !p.is_null() {
        match ast_id(p) as libc::c_uint {
            89 | 90 => {
                if ast_id(ast_child(p)) as libc::c_uint != TK_AT as libc::c_int as libc::c_uint {
                    let mut body: *mut ast_t = ast_childidx(p, 6 as libc::c_int as size_t);
                    if !capture_from_expr(opt, ctx, body, capture, last_capture) {
                        ok = 0 as libc::c_int != 0;
                    }
                }
            }
            _ => {}
        }
        p = ast_sibling(p);
    }
    ast_clear(*def);
    ok
}
#[c2rust::src_loc = "605:1"]
unsafe extern "C" fn add_field_to_object(
    mut opt: *mut pass_opt_t,
    mut field: *mut ast_t,
    mut class_members: *mut ast_t,
    mut create_params: *mut ast_t,
    mut create_body: *mut ast_t,
    mut call_args: *mut ast_t,
) {
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut init: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut id, &mut type_0, &mut init, 0 as *mut *mut ast_t];
    ast_get_children(
        field,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut p_id: *mut ast_t = ast_from_string(id, package_hygienic_id(&mut (*opt).check));
    let mut param: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = field;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_PARAM);
    if parent.is_null() {
        parent = node;
    } else if last_sibling.is_null() {
        last_sibling = ast_add(parent, node);
    } else {
        last_sibling = ast_add_sibling(last_sibling, node);
    }
    let mut parent_0: *mut ast_t = node;
    let mut last_sibling_0: *mut ast_t = 0 as *mut ast_t;
    let mut _node_0: *mut ast_t = 0 as *mut ast_t;
    if parent_0.is_null() {
        parent_0 = p_id;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, p_id);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, p_id);
    }
    if parent_0.is_null() {
        parent_0 = type_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, type_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, type_0);
    }
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    param = parent;
    let mut arg: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_0: *mut ast_t = init;
    let mut parent_1: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
    let mut node_1: *mut ast_t = 0 as *mut ast_t;
    node_1 = ast_from(basis_ast_0, TK_SEQ);
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
        parent_2 = init;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, init);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, init);
    }
    ast_inheritflags(parent_2);
    arg = parent_1;
    let mut assign: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_1: *mut ast_t = init;
    let mut parent_3: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
    let mut node_3: *mut ast_t = 0 as *mut ast_t;
    node_3 = ast_from(basis_ast_1, TK_ASSIGN);
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
    node_4 = ast_from(basis_ast_1, TK_REFERENCE);
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
        parent_5 = id;
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(parent_5, id);
    } else {
        last_sibling_5 = ast_add_sibling(last_sibling_5, id);
    }
    ast_inheritflags(parent_5);
    node_4 = ast_from(basis_ast_1, TK_CONSUME);
    if parent_4.is_null() {
        parent_4 = node_4;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, node_4);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, node_4);
    }
    let mut parent_6: *mut ast_t = node_4;
    let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
    let mut node_6: *mut ast_t = 0 as *mut ast_t;
    node_6 = ast_from(basis_ast_1, TK_NONE);
    if parent_6.is_null() {
        parent_6 = node_6;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, node_6);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
    }
    let mut parent_7: *mut ast_t = node_6;
    let mut _last_sibling_7: *mut ast_t = 0 as *mut ast_t;
    let mut _node_7: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_7);
    node_6 = ast_from(basis_ast_1, TK_REFERENCE);
    if parent_6.is_null() {
        parent_6 = node_6;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, node_6);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
    }
    let mut parent_8: *mut ast_t = node_6;
    let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
    let mut _node_8: *mut ast_t = 0 as *mut ast_t;
    if parent_8.is_null() {
        parent_8 = p_id;
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, p_id);
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, p_id);
    }
    ast_inheritflags(parent_8);
    ast_inheritflags(parent_6);
    ast_inheritflags(parent_4);
    assign = parent_3;
    ast_replace(&mut init, ast_from(init, TK_NONE));
    ast_add(class_members, field);
    ast_append(create_params, param);
    ast_append(create_body, assign);
    ast_append(call_args, arg);
}
#[c2rust::src_loc = "640:1"]
unsafe extern "C" fn catch_up_provides(mut opt: *mut pass_opt_t, mut provides: *mut ast_t) -> bool {
    let mut child: *mut ast_t = ast_child(provides);
    while !child.is_null() {
        if !ast_passes_subtree(&mut child, opt, PASS_EXPR) {
            return 0 as libc::c_int != 0;
        }
        let mut def: *mut ast_t = ast_data(child) as *mut ast_t;
        if !def.is_null() && !ast_passes_type(&mut def, opt, PASS_EXPR) {
            return 0 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "665:1"]
pub unsafe extern "C" fn expr_object(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) -> bool {
    let mut ast: *mut ast_t = *astp;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut provides: ast_ptr_t = 0 as *mut ast_t;
    let mut members: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut cap, &mut provides, &mut members, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    ast_clearflag(cap, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    ast_clearflag(provides, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    ast_clearflag(members, AST_FLAG_PRESERVE as libc::c_int as uint32_t);
    let mut annotation: *mut ast_t = ast_consumeannotation(ast);
    let mut c_id: *const libc::c_char = package_hygienic_id(&mut (*opt).check);
    let mut t_params: *mut ast_t = 0 as *mut ast_t;
    let mut t_args: *mut ast_t = 0 as *mut ast_t;
    collect_type_params(ast, &mut t_params, &mut t_args);
    let mut nice_id: *const libc::c_char = ast_data(ast) as *const libc::c_char;
    if nice_id.is_null() {
        nice_id = b"object literal\0" as *const u8 as *const libc::c_char;
    }
    let mut def: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_CLASS);
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
    ast_setannotation(parent_0, annotation);
    if parent_0.is_null() {
        parent_0 = ast_setdata(
            ast_from_string(basis_ast, c_id),
            stringtab(nice_id) as *mut libc::c_void,
        );
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(
            parent_0,
            ast_setdata(
                ast_from_string(basis_ast, c_id),
                stringtab(nice_id) as *mut libc::c_void,
            ),
        );
    } else {
        last_sibling_0 = ast_add_sibling(
            last_sibling_0,
            ast_setdata(
                ast_from_string(basis_ast, c_id),
                stringtab(nice_id) as *mut libc::c_void,
            ),
        );
    }
    if parent_0.is_null() {
        parent_0 = t_params;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, t_params);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, t_params);
    }
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    if parent_0.is_null() {
        parent_0 = provides;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, provides);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, provides);
    }
    node_0 = ast_from(basis_ast, TK_MEMBERS);
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
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    def = parent;
    let mut create: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_0: *mut ast_t = members;
    let mut parent_2: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    node_2 = ast_from(basis_ast_0, TK_NEW);
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
    ast_scope(parent_3);
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast_0, TK_NONE));
    }
    if parent_3.is_null() {
        parent_3 = ast_from_string(basis_ast_0, b"create\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(
            parent_3,
            ast_from_string(basis_ast_0, b"create\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_3 = ast_add_sibling(
            last_sibling_3,
            ast_from_string(basis_ast_0, b"create\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast_0, TK_NONE));
    }
    node_3 = ast_from(basis_ast_0, TK_PARAMS);
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
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast_0, TK_NONE));
    }
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast_0, TK_NONE));
    }
    node_3 = ast_from(basis_ast_0, TK_SEQ);
    if parent_3.is_null() {
        parent_3 = node_3;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, node_3);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, node_3);
    }
    let mut parent_5: *mut ast_t = node_3;
    let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
    let mut node_5: *mut ast_t = 0 as *mut ast_t;
    node_5 = ast_from(basis_ast_0, TK_TRUE);
    if parent_5.is_null() {
        parent_5 = node_5;
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(parent_5, node_5);
    } else {
        last_sibling_5 = ast_add_sibling(last_sibling_5, node_5);
    }
    let mut parent_6: *mut ast_t = node_5;
    let mut _last_sibling_6: *mut ast_t = 0 as *mut ast_t;
    let mut _node_6: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_6);
    ast_inheritflags(parent_5);
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast_0, TK_NONE));
    }
    ast_inheritflags(parent_3);
    create = parent_2;
    let mut type_ref: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_1: *mut ast_t = ast;
    let mut parent_7: *mut ast_t = 0 as *mut ast_t;
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
        parent_8 = ast_from_string(basis_ast_1, c_id);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, ast_from_string(basis_ast_1, c_id));
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, ast_from_string(basis_ast_1, c_id));
    }
    ast_inheritflags(parent_8);
    type_ref = parent_7;
    if ast_id(t_args) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        let mut t: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_2: *mut ast_t = ast;
        let mut parent_9: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_9: *mut ast_t = 0 as *mut ast_t;
        let mut node_9: *mut ast_t = 0 as *mut ast_t;
        node_9 = ast_from(basis_ast_2, TK_QUALIFY);
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
            parent_10 = type_ref;
        } else if last_sibling_10.is_null() {
            last_sibling_10 = ast_add(parent_10, type_ref);
        } else {
            last_sibling_10 = ast_add_sibling(last_sibling_10, type_ref);
        }
        if parent_10.is_null() {
            parent_10 = t_args;
        } else if last_sibling_10.is_null() {
            last_sibling_10 = ast_add(parent_10, t_args);
        } else {
            last_sibling_10 = ast_add_sibling(last_sibling_10, t_args);
        }
        ast_inheritflags(parent_10);
        t = parent_9;
        type_ref = t;
    }
    ast_free_unattached(t_args);
    let mut call: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_3: *mut ast_t = ast;
    let mut parent_11: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_11: *mut ast_t = 0 as *mut ast_t;
    let mut node_11: *mut ast_t = 0 as *mut ast_t;
    node_11 = ast_from(basis_ast_3, TK_CALL);
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
    node_12 = ast_from(basis_ast_3, TK_DOT);
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
        parent_13 = type_ref;
    } else if last_sibling_13.is_null() {
        last_sibling_13 = ast_add(parent_13, type_ref);
    } else {
        last_sibling_13 = ast_add_sibling(last_sibling_13, type_ref);
    }
    if parent_13.is_null() {
        parent_13 = ast_from_string(basis_ast_3, b"create\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_13.is_null() {
        last_sibling_13 = ast_add(
            parent_13,
            ast_from_string(basis_ast_3, b"create\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_13 = ast_add_sibling(
            last_sibling_13,
            ast_from_string(basis_ast_3, b"create\0" as *const u8 as *const libc::c_char),
        );
    }
    ast_inheritflags(parent_13);
    node_12 = ast_from(basis_ast_3, TK_POSITIONALARGS);
    if parent_12.is_null() {
        parent_12 = node_12;
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, node_12);
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, node_12);
    }
    let mut parent_14: *mut ast_t = node_12;
    let mut _last_sibling_14: *mut ast_t = 0 as *mut ast_t;
    let mut _node_14: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_14);
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast_3, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast_3, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast_3, TK_NONE));
    }
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast_3, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast_3, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast_3, TK_NONE));
    }
    ast_inheritflags(parent_12);
    call = parent_11;
    let mut create_params: *mut ast_t = ast_childidx(create, 3 as libc::c_int as size_t);
    let mut create_body: *mut ast_t = ast_childidx(create, 6 as libc::c_int as size_t);
    let mut call_args: *mut ast_t = ast_childidx(call, 1 as libc::c_int as size_t);
    let mut class_members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as size_t);
    let mut member: *mut ast_t = ast_child(members);
    let mut has_fields: bool = 0 as libc::c_int != 0;
    let mut has_behaviours: bool = 0 as libc::c_int != 0;
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            140 | 141 | 86 => {
                add_field_to_object(
                    opt,
                    member,
                    class_members,
                    create_params,
                    create_body,
                    call_args,
                );
                has_fields = 1 as libc::c_int != 0;
            }
            90 => {
                ast_append(class_members, member);
                has_behaviours = 1 as libc::c_int != 0;
            }
            _ => {
                ast_append(class_members, member);
            }
        }
        member = ast_sibling(member);
    }
    ast_append(class_members, create);
    let mut module: *mut ast_t = ast_nearest(ast, TK_MODULE);
    ast_append(module, def);
    let mut captures: *mut ast_t = ast_from(ast, TK_MEMBERS);
    let mut last_capture: *mut ast_t = 0 as *mut ast_t;
    if !capture_from_type(opt, *astp, &mut def, captures, &mut last_capture) {
        ok = 0 as libc::c_int != 0;
    }
    let mut p: *mut ast_t = ast_child(captures);
    while !p.is_null() {
        add_field_to_object(opt, p, class_members, create_params, create_body, call_args);
        has_fields = 1 as libc::c_int != 0;
        p = ast_sibling(p);
    }
    ast_free_unattached(captures);
    ast_resetpass(def, PASS_SUGAR as libc::c_int as uint32_t);
    let mut cap_id: token_id = ast_id(cap);
    if has_behaviours {
        ast_setid(def, TK_ACTOR);
        if cap_id as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
            && cap_id as libc::c_uint != TK_TAG as libc::c_int as libc::c_uint
        {
            ast_error(
                (*opt).check.errors,
                cap,
                b"object literals with behaviours are actors and so must have tag capability\0"
                    as *const u8 as *const libc::c_char,
            );
            ok = 0 as libc::c_int != 0;
        }
        cap_id = TK_TAG;
    } else if !has_fields
        && (cap_id as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
            || cap_id as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint
            || cap_id as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
            || cap_id as libc::c_uint == TK_VAL as libc::c_int as libc::c_uint)
    {
        ast_setid(def, TK_PRIMITIVE);
        cap_id = TK_VAL;
    }
    if ast_id(def) as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
        if !ast_has_annotation(def, b"ponyint_bare\0" as *const u8 as *const libc::c_char) {
        } else {
            ponyint_assert_fail(
                b"!ast_has_annotation(def, \"ponyint_bare\")\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.c\0"
                    as *const u8 as *const libc::c_char,
                823 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"expr_object\0"))
                    .as_ptr(),
            );
        };
    }
    ast_setid(ast_child(create), cap_id);
    let mut result: *mut ast_t = ast_childidx(create, 4 as libc::c_int as size_t);
    ast_replace(
        &mut result,
        type_for_class(
            opt,
            def,
            result,
            cap_id,
            TK_EPHEMERAL,
            0 as libc::c_int != 0,
        ),
    );
    if !catch_up_provides(opt, provides) {
        return 0 as libc::c_int != 0;
    }
    if !ast_passes_type(&mut def, opt, PASS_EXPR) {
        return 0 as libc::c_int != 0;
    }
    ast_replace(astp, call);
    if ast_visit(
        astp,
        Some(pass_syntax as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
        opt,
        PASS_SYNTAX,
    ) as libc::c_uint
        != AST_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    if !ast_passes_subtree(astp, opt, PASS_EXPR) {
        return 0 as libc::c_int != 0;
    }
    ok
}
