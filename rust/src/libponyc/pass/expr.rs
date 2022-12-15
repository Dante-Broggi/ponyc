use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = usize;
    use super::_types_h::__darwin_size_t;
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
        #[c2rust::src_loc = "63:1"]
        pub fn errors_get_count(errors: *mut errors_t) -> usize;
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:35"]
    pub struct symtab_t {
        pub contents: hashmap_t,
    }
    use super::error_h::errors_t;
    use super::hash_h::hashmap_t;
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
        #[c2rust::src_loc = "63:1"]
        pub fn symtab_check_all_defined(symtab: *mut symtab_t, errors: *mut errors_t) -> bool;
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::symtab_h::{ast_t, symtab_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn ast_get_symtab(ast: *mut ast_t) -> *mut symtab_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: usize) -> *mut ast_t;
        #[c2rust::src_loc = "114:1"]
        pub fn ast_childlast(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "141:1"]
        pub fn ast_remove(ast: *mut ast_t);
        #[c2rust::src_loc = "143:1"]
        pub fn ast_replace(prev: *mut *mut ast_t, next: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: usize,
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:13"]
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
        #[c2rust::src_loc = "38:1"]
        pub fn deferred_reify_method_def(
            deferred: *mut deferred_reification_t,
            ast: *mut ast_t,
            opt: *mut pass_opt_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "43:1"]
        pub fn deferred_reify_free(deferred: *mut deferred_reification_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:2"]
pub mod literal_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn expr_literal(
            opt: *mut pass_opt_t,
            ast: *mut ast_t,
            name: *const libc::c_char,
        ) -> bool;
        #[c2rust::src_loc = "17:1"]
        pub fn make_literal_type(ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/reference.h:3"]
pub mod reference_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn expr_provides(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_param(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn expr_field(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "15:1"]
        pub fn expr_typeref(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn expr_dontcareref(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "17:1"]
        pub fn expr_local(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn expr_localref(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "19:1"]
        pub fn expr_paramref(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "20:1"]
        pub fn expr_addressof(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "21:1"]
        pub fn expr_digestof(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "22:1"]
        pub fn expr_this(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn expr_tuple(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn expr_nominal(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "25:1"]
        pub fn expr_fun(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "26:1"]
        pub fn expr_compile_intrinsic(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/operator.h:4"]
pub mod operator_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_identity(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "11:1"]
        pub fn expr_assign(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_as(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn expr_consume(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/postfix.h:5"]
pub mod postfix_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_qualify(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "11:1"]
        pub fn expr_dot(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_tilde(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn expr_chain(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/call.h:6"]
pub mod call_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn expr_call(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/control.h:7"]
pub mod control_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_seq(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "11:1"]
        pub fn expr_if(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_iftype(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn expr_while(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "14:1"]
        pub fn expr_repeat(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "15:1"]
        pub fn expr_try(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn expr_disposing_block(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "17:1"]
        pub fn expr_recover(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn expr_break(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "19:1"]
        pub fn expr_return(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "22:1"]
        pub fn expr_location(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/match.h:8"]
pub mod match_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_match(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "11:1"]
        pub fn expr_cases(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_case(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn expr_match_capture(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/array.h:9"]
pub mod array_h {
    use super::ast_h::ast_result_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_pre_array(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> ast_result_t;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_array(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/ffi.h:10"]
pub mod ffi_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_ffi(opt: *mut pass_opt_t, ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/lambda.h:11"]
pub mod lambda_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn expr_lambda(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn expr_object(opt: *mut pass_opt_t, astp: *mut *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:12"]
pub mod assemble_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "72:1"]
        pub fn type_for_fun(ast: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.h:13"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:14"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:15"]
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
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::array_h::{expr_array, expr_pre_array};
use self::assemble_h::type_for_fun;
pub use self::ast_h::{
    ast_child, ast_childcount, ast_childidx, ast_childlast, ast_free_unattached, ast_get_children,
    ast_get_symtab, ast_id, ast_name, ast_parent, ast_ptr_t, ast_remove, ast_replace, ast_result_t,
    ast_sibling, ast_type, AST_ERROR, AST_FATAL, AST_IGNORE, AST_OK,
};
use self::call_h::expr_call;
use self::control_h::{
    expr_break, expr_disposing_block, expr_if, expr_iftype, expr_location, expr_recover,
    expr_repeat, expr_return, expr_seq, expr_try, expr_while,
};
pub use self::error_h::{errorframe_t, errormsg_t, errors_get_count, errors_t};
use self::ffi_h::expr_ffi;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
use self::lambda_h::{expr_lambda, expr_object};
use self::literal_h::{expr_literal, make_literal_type};
use self::lookup_h::lookup;
use self::match_h::{expr_case, expr_cases, expr_match, expr_match_capture};
use self::operator_h::{expr_as, expr_assign, expr_consume, expr_identity};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::postfix_h::{expr_chain, expr_dot, expr_qualify, expr_tilde};
use self::reference_h::{
    expr_addressof, expr_compile_intrinsic, expr_digestof, expr_dontcareref, expr_field, expr_fun,
    expr_local, expr_localref, expr_nominal, expr_param, expr_paramref, expr_provides, expr_this,
    expr_tuple, expr_typeref,
};
pub use self::reify_h::{deferred_reification_t, deferred_reify_free, deferred_reify_method_def};
use self::stringtab_h::stringtab;
use self::subtype_h::is_subtype;
pub use self::symtab_h::{ast_t, symtab_check_all_defined, symtab_t};
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
#[c2rust::src_loc = "17:1"]
unsafe extern "C" fn is_numeric_primitive(mut name: *const libc::c_char) -> bool {
    if name == stringtab(b"U8\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"I8\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"U16\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"I16\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"U32\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"I32\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"U64\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"I64\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"U128\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"I128\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"ULong\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"ILong\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"USize\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"ISize\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"F32\0" as *const u8 as *const libc::c_char)
        || name == stringtab(b"F64\0" as *const u8 as *const libc::c_char)
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn is_result_needed(mut ast: *mut ast_t) -> bool {
    let mut parent: *mut ast_t = ast_parent(ast);
    match ast_id(parent) as libc::c_uint {
        175 => {
            if !(ast_sibling(ast)).is_null() {
                return 0 as libc::c_int != 0;
            }
            return is_result_needed(parent);
        }
        108 | 109 | 116 | 122 => {
            if ast_child(parent) == ast {
                return 1 as libc::c_int != 0;
            }
            return is_result_needed(parent);
        }
        110 => {
            if ast_child(parent) == ast || ast_childidx(parent, 1 as libc::c_int as usize) == ast {
                return 0 as libc::c_int != 0;
            }
            return is_result_needed(parent);
        }
        118 => {
            if ast_childidx(parent, 1 as libc::c_int as usize) == ast {
                return 1 as libc::c_int != 0;
            }
            return is_result_needed(parent);
        }
        181 => {
            if ast_childidx(parent, 2 as libc::c_int as usize) != ast {
                return 1 as libc::c_int != 0;
            }
            return is_result_needed(parent);
        }
        180 | 111 | 124 | 125 | 107 | 206 => return is_result_needed(parent),
        88 => {
            let mut type_0: *mut ast_t = ast_childidx(parent, 4 as libc::c_int as usize);
            if ast_id(type_0) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    96 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"is_result_needed\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut pkg_name: *const libc::c_char = ast_name(ast_child(type_0));
            let mut type_name: *const libc::c_char =
                ast_name(ast_childidx(type_0, 1 as libc::c_int as usize));
            if pkg_name == stringtab(b"$0\0" as *const u8 as *const libc::c_char) {
                return is_numeric_primitive(type_name);
            }
            return 0 as libc::c_int != 0;
        }
        90 => return 0 as libc::c_int != 0,
        203 | 204 => {
            if ast_childidx(parent, 0 as libc::c_int as usize) == ast {
                return is_result_needed(parent);
            }
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn is_method_result(mut t: *mut typecheck_t, mut ast: *mut ast_t) -> bool {
    if ast == (*(*t).frame).method_body {
        return 1 as libc::c_int != 0;
    }
    let mut parent: *mut ast_t = ast_parent(ast);
    let mut current_block_14: u64;
    match ast_id(parent) as libc::c_uint {
        175 => {
            if !(ast_sibling(ast)).is_null() {
                return 0 as libc::c_int != 0;
            }
            current_block_14 = 2668756484064249700;
        }
        108 | 116 | 122 | 109 => {
            if ast_child(parent) == ast {
                return 0 as libc::c_int != 0;
            }
            current_block_14 = 2668756484064249700;
        }
        110 => {
            current_block_14 = 10275258781883576179;
        }
        120 | 181 => {
            current_block_14 = 10275258781883576179;
        }
        118 => {
            if ast_childidx(parent, 1 as libc::c_int as usize) == ast {
                return 0 as libc::c_int != 0;
            }
            current_block_14 = 2668756484064249700;
        }
        180 | 111 | 107 | 178 => {
            current_block_14 = 2668756484064249700;
        }
        124 | 125 => {
            if ast_childidx(parent, 2 as libc::c_int as usize) == ast {
                return 0 as libc::c_int != 0;
            }
            current_block_14 = 2668756484064249700;
        }
        206 => {
            if ast_childidx(parent, 1 as libc::c_int as usize) == ast {
                return 0 as libc::c_int != 0;
            }
            current_block_14 = 2668756484064249700;
        }
        _ => return 0 as libc::c_int != 0,
    }
    match current_block_14 {
        10275258781883576179 => {
            if ast_child(parent) == ast || ast_childidx(parent, 1 as libc::c_int as usize) == ast {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return is_method_result(t, parent);
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn is_method_return(mut t: *mut typecheck_t, mut ast: *mut ast_t) -> bool {
    let mut parent: *mut ast_t = ast_parent(ast);
    if ast_id(parent) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        parent = ast_parent(parent);
        if ast_id(parent) as libc::c_uint == TK_RETURN as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        }
    }
    return is_method_result(t, ast);
}
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn is_typecheck_error(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 1 as libc::c_int != 0;
    }
    if ast_id(type_0) as libc::c_uint == TK_INFERTYPE as libc::c_int as libc::c_uint
        || ast_id(type_0) as libc::c_uint == TK_ERRORTYPE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn find_tuple_type(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut child_count: usize,
) -> *mut ast_t {
    if ast_id(ast) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint
        && ast_childcount(ast) == child_count
    {
        return ast;
    }
    match ast_id(ast) as libc::c_uint {
        149 | 56 => {
            let mut member_type: *mut ast_t = ast_child(ast);
            while !member_type.is_null() {
                let mut member_tuple_type: *mut ast_t =
                    find_tuple_type(opt, member_type, child_count);
                if !member_tuple_type.is_null() {
                    return member_tuple_type;
                }
                member_type = ast_sibling(member_type);
            }
        }
        17 => return find_tuple_type(opt, ast_childlast(ast), child_count),
        187 | _ => {}
    }
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn find_antecedent_type(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut is_recovered: *mut bool,
) -> *mut ast_t {
    let mut parent: *mut ast_t = ast_parent(ast);
    match ast_id(parent) as libc::c_uint {
        24 => {
            let mut lhs: ast_ptr_t = 0 as *mut ast_t;
            let mut rhs: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut lhs, &mut rhs, 0 as *mut *mut ast_t];
            ast_get_children(
                parent,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                    .wrapping_sub(1),
                children.as_mut_ptr(),
            );
            if rhs != ast {
                return 0 as *mut ast_t;
            }
            return ast_type(lhs);
        }
        165 | 174 => {
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut type_0: ast_ptr_t = 0 as *mut ast_t;
            let mut deflt: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 4] =
                [&mut id, &mut type_0, &mut deflt, 0 as *mut *mut ast_t];
            ast_get_children(
                parent,
                ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                    .wrapping_sub(1),
                children_0.as_mut_ptr(),
            );
            if ast == deflt {
            } else {
                ponyint_assert_fail(
                    b"ast == deflt\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    283 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"find_antecedent_type\0",
                    ))
                    .as_ptr(),
                );
            };
            return type_0;
        }
        179 => {
            let mut type_1: ast_ptr_t = 0 as *mut ast_t;
            let mut seq: ast_ptr_t = 0 as *mut ast_t;
            let mut children_1: [*mut *mut ast_t; 3] =
                [&mut type_1, &mut seq, 0 as *mut *mut ast_t];
            ast_get_children(
                parent,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                    .wrapping_sub(1),
                children_1.as_mut_ptr(),
            );
            if ast == seq {
            } else {
                ponyint_assert_fail(
                    b"ast == seq\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    291 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"find_antecedent_type\0",
                    ))
                    .as_ptr(),
                );
            };
            if ast_id(type_1) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                return 0 as *mut ast_t;
            }
            return type_1;
        }
        169 => {
            let mut receiver: *mut ast_t = ast_child(ast_parent(parent));
            let mut funtype: *mut ast_t = ast_type(receiver);
            if is_typecheck_error(funtype) {
                return funtype;
            }
            if ast_id(funtype) as libc::c_uint != TK_FUNTYPE as libc::c_int as libc::c_uint {
                let mut fun: *mut deferred_reification_t = lookup(
                    opt,
                    receiver,
                    funtype,
                    stringtab(b"apply\0" as *const u8 as *const libc::c_char),
                );
                if fun.is_null() {
                    return 0 as *mut ast_t;
                }
                if ast_id((*fun).ast) as libc::c_uint != TK_BE as libc::c_int as libc::c_uint
                    && ast_id((*fun).ast) as libc::c_uint != TK_FUN as libc::c_int as libc::c_uint
                {
                    deferred_reify_free(fun);
                    return 0 as *mut ast_t;
                }
                let mut r_fun: *mut ast_t = deferred_reify_method_def(fun, (*fun).ast, opt);
                funtype = type_for_fun(r_fun);
                ast_free_unattached(r_fun);
                deferred_reify_free(fun);
            }
            let mut cap: ast_ptr_t = 0 as *mut ast_t;
            let mut t_params: ast_ptr_t = 0 as *mut ast_t;
            let mut params: ast_ptr_t = 0 as *mut ast_t;
            let mut ret_type: ast_ptr_t = 0 as *mut ast_t;
            let mut children_2: [*mut *mut ast_t; 5] = [
                &mut cap,
                &mut t_params,
                &mut params,
                &mut ret_type,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                funtype,
                (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                    .wrapping_sub(1),
                children_2.as_mut_ptr(),
            );
            let mut arg: *mut ast_t = ast_child(parent);
            let mut param: *mut ast_t = ast_child(params);
            while !arg.is_null() && !param.is_null() {
                if arg == ast {
                    return ast_childidx(param, 1 as libc::c_int as usize);
                }
                arg = ast_sibling(arg);
                param = ast_sibling(param);
            }
            return 0 as *mut ast_t;
        }
        171 | 172 => {
            let mut receiver_0: *mut ast_t = ast_child(ast_parent(ast_parent(parent)));
            let mut funtype_0: *mut ast_t = ast_type(receiver_0);
            if is_typecheck_error(funtype_0) {
                return funtype_0;
            }
            if ast_id(funtype_0) as libc::c_uint == TK_FUNTYPE as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(funtype) == TK_FUNTYPE\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    358 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"find_antecedent_type\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut cap_0: ast_ptr_t = 0 as *mut ast_t;
            let mut t_params_0: ast_ptr_t = 0 as *mut ast_t;
            let mut params_0: ast_ptr_t = 0 as *mut ast_t;
            let mut ret_type_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_3: [*mut *mut ast_t; 5] = [
                &mut cap_0,
                &mut t_params_0,
                &mut params_0,
                &mut ret_type_0,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                funtype_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                    .wrapping_sub(1),
                children_3.as_mut_ptr(),
            );
            let mut name: *const libc::c_char = ast_name(ast_child(parent));
            let mut param_0: *mut ast_t = ast_child(params_0);
            while !param_0.is_null() {
                if ast_name(ast_child(param_0)) == name {
                    return ast_childidx(param_0, 1 as libc::c_int as usize);
                }
                param_0 = ast_sibling(param_0);
            }
            return 0 as *mut ast_t;
        }
        89 => {
            let mut body: *mut ast_t = ast_childidx(parent, 6 as libc::c_int as usize);
            if ast == body {
            } else {
                ponyint_assert_fail(
                    b"ast == body\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    381 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"find_antecedent_type\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut ret_type_1: *mut ast_t = ast_childidx(parent, 4 as libc::c_int as usize);
            if ast_id(ret_type_1) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                return 0 as *mut ast_t;
            }
            return ret_type_1;
        }
        175 => {
            if ast_childlast(parent) == ast {
                return find_antecedent_type(opt, parent, is_recovered);
            }
            if ast_id(ast_parent(parent)) as libc::c_uint == TK_ARRAY as libc::c_int as libc::c_uint
            {
                return find_antecedent_type(opt, parent, is_recovered);
            }
            return 0 as *mut ast_t;
        }
        178 => {
            let mut antecedent: *mut ast_t = find_antecedent_type(opt, parent, is_recovered);
            if antecedent.is_null() {
                return 0 as *mut ast_t;
            }
            antecedent = find_tuple_type(opt, antecedent, ast_childcount(parent));
            if antecedent.is_null() {
                return 0 as *mut ast_t;
            }
            if ast_id(antecedent) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(antecedent) == TK_TUPLETYPE\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0"
                        as *const u8 as *const libc::c_char,
                    415 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"find_antecedent_type\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut elem: *mut ast_t = ast_child(parent);
            let mut type_elem: *mut ast_t = ast_child(antecedent);
            while !elem.is_null() && !type_elem.is_null() {
                if elem == ast {
                    return type_elem;
                }
                elem = ast_sibling(elem);
                type_elem = ast_sibling(type_elem);
            }
        }
        103 => {
            let mut body_0: *mut ast_t = (*(*opt).check.frame).method_body;
            if body_0.is_null() {
                return 0 as *mut ast_t;
            }
            return find_antecedent_type(opt, body_0, is_recovered);
        }
        104 => {
            let mut body_1: *mut ast_t = (*(*opt).check.frame).loop_body;
            if body_1.is_null() {
                return 0 as *mut ast_t;
            }
            return find_antecedent_type(opt, body_1, is_recovered);
        }
        107 => {
            if !is_recovered.is_null() {
                *is_recovered = 1 as libc::c_int != 0;
            }
            return find_antecedent_type(opt, parent, is_recovered);
        }
        108 | 109 | 110 | 111 | 112 | 113 | 116 | 118 | 122 | 180 | 181 | 124 | 125 | 206 | 177 => {
            return find_antecedent_type(opt, parent, is_recovered)
        }
        _ => {}
    }
    return 0 as *mut ast_t;
}
#[c2rust::src_loc = "485:1"]
unsafe extern "C" fn fold_union(mut opt: *mut pass_opt_t, mut astp: *mut *mut ast_t) {
    let mut ast: *mut ast_t = *astp;
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        let mut next: *mut ast_t = ast_sibling(child);
        let mut remove: bool = 0 as libc::c_int != 0;
        while !next.is_null() {
            if is_subtype(next, child, 0 as *mut errorframe_t, opt) {
                let mut tmp: *mut ast_t = next;
                next = ast_sibling(next);
                ast_remove(tmp);
            } else if is_subtype(child, next, 0 as *mut errorframe_t, opt) {
                remove = 1 as libc::c_int != 0;
                break;
            } else {
                next = ast_sibling(next);
            }
        }
        if remove {
            let mut tmp_0: *mut ast_t = child;
            child = ast_sibling(child);
            ast_remove(tmp_0);
        } else {
            child = ast_sibling(child);
        }
    }
    child = ast_child(ast);
    if (ast_sibling(child)).is_null() {
        ast_replace(astp, child);
    }
}
#[no_mangle]
#[c2rust::src_loc = "527:1"]
pub unsafe extern "C" fn pass_pre_expr(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    match ast_id(ast) as libc::c_uint {
        179 => return expr_pre_array(options, astp),
        146 | 144 | 145 | 147 => {
            if !(ast_parent(ast)).is_null()
                && ast_id(ast_parent(ast)) as libc::c_uint == TK_USE as libc::c_int as libc::c_uint
            {
                return AST_IGNORE;
            }
        }
        _ => {}
    }
    AST_OK
}
#[no_mangle]
#[c2rust::src_loc = "548:1"]
pub unsafe extern "C" fn pass_expr(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    let mut r: bool = 1 as libc::c_int != 0;
    match ast_id(ast) as libc::c_uint {
        74 | 75 | 76 | 77 | 73 | 72 => {
            r = expr_provides(options, ast);
        }
        151 => {
            r = expr_nominal(options, astp);
        }
        140 | 141 | 86 => {
            r = expr_field(options, ast);
        }
        165 => {
            r = expr_param(options, ast);
        }
        88 | 90 | 89 => {
            r = expr_fun(options, ast);
        }
        175 => {
            r = expr_seq(options, ast);
        }
        84 | 85 => {
            r = expr_local(options, ast);
        }
        104 => {
            r = expr_break(options, ast);
        }
        103 => {
            r = expr_return(options, ast);
        }
        82 | 83 => {
            r = expr_identity(options, ast);
        }
        24 => {
            r = expr_assign(options, ast);
        }
        106 => {
            r = expr_consume(options, ast);
        }
        107 => {
            r = expr_recover(options, ast);
        }
        19 => {
            r = expr_dot(options, astp);
        }
        20 => {
            r = expr_tilde(options, astp);
        }
        21 => {
            r = expr_chain(options, astp);
        }
        176 => {
            r = expr_qualify(options, astp);
        }
        177 => {
            r = expr_call(options, astp);
        }
        109 | 108 => {
            r = expr_if(options, ast);
        }
        111 => {
            r = expr_iftype(options, ast);
        }
        116 => {
            r = expr_while(options, ast);
        }
        118 => {
            r = expr_repeat(options, ast);
        }
        125 | 124 => {
            r = expr_try(options, ast);
        }
        206 => {
            r = expr_disposing_block(options, ast);
        }
        122 => {
            r = expr_match(options, ast);
        }
        180 => {
            r = expr_cases(options, ast);
        }
        181 => {
            r = expr_case(options, ast);
        }
        182 => {
            r = expr_match_capture(options, ast);
        }
        178 => {
            r = expr_tuple(options, ast);
        }
        179 => {
            r = expr_array(options, astp);
        }
        199 => {
            r = expr_dontcareref(options, ast);
        }
        186 => {
            r = expr_typeref(options, astp);
        }
        196 | 197 => {
            r = expr_localref(options, ast);
        }
        198 => {
            r = expr_paramref(options, ast);
        }
        102 => {
            r = expr_this(options, ast);
        }
        3 | 4 => {
            r = expr_literal(options, ast, b"Bool\0" as *const u8 as *const libc::c_char);
        }
        69 => {
            r = expr_compile_intrinsic(options, ast);
        }
        135 => {
            r = expr_location(options, ast);
        }
        134 => {
            r = expr_addressof(options, ast);
        }
        133 => {
            r = expr_digestof(options, ast);
        }
        81 => {
            if !expr_as(options, astp) {
                return AST_FATAL;
            }
        }
        78 => {
            if !expr_object(options, astp) {
                return AST_FATAL;
            }
        }
        79 | 80 => {
            if !expr_lambda(options, astp) {
                return AST_FATAL;
            }
        }
        149 => {
            fold_union(options, astp);
        }
        6 => {
            make_literal_type(ast);
        }
        7 => {
            make_literal_type(ast);
        }
        5 => {
            if ast_id(ast_parent(ast)) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint
            {
                return AST_OK;
            }
            r = expr_literal(
                options,
                ast,
                b"String\0" as *const u8 as *const libc::c_char,
            );
        }
        143 => {
            r = expr_ffi(options, ast);
        }
        _ => {}
    }
    if !r {
        if errors_get_count((*options).check.errors) > 0 {
        } else {
            ponyint_assert_fail(
                b"errors_get_count(options->check.errors) > 0\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.c\0" as *const u8
                    as *const libc::c_char,
                661 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"pass_expr\0"))
                    .as_ptr(),
            );
        };
        return AST_ERROR;
    }
    let mut type_0: *mut ast_t = ast_type(*astp);
    if !type_0.is_null()
        && ast_id(type_0) as libc::c_uint == TK_UNIONTYPE as libc::c_int as libc::c_uint
    {
        fold_union(options, &mut type_0);
    }
    let mut symtab: *mut symtab_t = ast_get_symtab(*astp);
    if !symtab.is_null() && !symtab_check_all_defined(symtab, (*options).check.errors) {
        return AST_ERROR;
    }
    AST_OK
}
