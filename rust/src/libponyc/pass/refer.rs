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
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "20:1"]
        pub fn stringtab_consume(
            string: *const libc::c_char,
            buf_size: usize,
        ) -> *const libc::c_char;
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
    use super::error_h::{errorframe_t, errors_t};
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "67:1"]
        pub fn ast_has_scope(ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ast_setdata(ast: *mut ast_t, data: *mut libc::c_void) -> *mut ast_t;
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: u32) -> libc::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: u32);
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
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "121:1"]
        pub fn ast_get_case(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "124:1"]
        pub fn ast_setstatus(ast: *mut ast_t, name: *const libc::c_char, status: sym_status_t);
        #[c2rust::src_loc = "125:1"]
        pub fn ast_inheritstatus(dst: *mut ast_t, src: *mut ast_t);
        #[c2rust::src_loc = "126:1"]
        pub fn ast_inheritbranch(dst: *mut ast_t, src: *mut ast_t);
        #[c2rust::src_loc = "127:1"]
        pub fn ast_consolidate_branches(ast: *mut ast_t, count: usize);
        #[c2rust::src_loc = "130:1"]
        pub fn ast_within_scope(
            outer: *mut ast_t,
            inner: *mut ast_t,
            name: *const libc::c_char,
        ) -> bool;
        #[c2rust::src_loc = "131:1"]
        pub fn ast_all_consumes_in_scope(
            outer: *mut ast_t,
            inner: *mut ast_t,
            errorf: *mut errorframe_t,
        ) -> bool;
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "143:1"]
        pub fn ast_replace(prev: *mut *mut ast_t, next: *mut ast_t);
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
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
            child_count: usize,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "216:1"]
        pub fn ast_get_provided_symbol_definition(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:2"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn is_name_private(name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "51:1"]
        pub fn is_name_dontcare(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:3"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn is_result_needed(ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:5"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:6"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::ast_h::{
    ast_add, ast_all_consumes_in_scope, ast_append, ast_checkflag, ast_child, ast_childcount,
    ast_childidx, ast_childlast, ast_consolidate_branches, ast_data, ast_error, ast_error_continue,
    ast_error_frame, ast_free, ast_from, ast_get, ast_get_case, ast_get_children,
    ast_get_provided_symbol_definition, ast_has_scope, ast_id, ast_inheritbranch,
    ast_inheritstatus, ast_line, ast_name, ast_parent, ast_pop, ast_pos, ast_print_type, ast_ptr_t,
    ast_replace, ast_result_t, ast_setdata, ast_setflag, ast_setid, ast_setstatus, ast_sibling,
    ast_type, ast_within_scope, C2RustUnnamed, AST_ERROR, AST_FATAL, AST_FLAG_AMBIGUOUS,
    AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND, AST_FLAG_CNSM_REASGN,
    AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2, AST_FLAG_FCNSM_REASGN,
    AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS, AST_FLAG_JUMPS_AWAY,
    AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI, AST_FLAG_PASS_MASK,
    AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2, AST_IGNORE, AST_OK,
};
pub use self::error_h::{errorframe_report, errorframe_t, errormsg_t, errors_get_count, errors_t};
use self::expr_h::is_result_needed;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::{is_name_dontcare, is_name_private};
pub use self::pass_h::{
    ast_passes_subtree, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL,
    PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::ponyint_pool_alloc_size;
use self::string_h::{memcpy, strlen};
use self::stringtab_h::{stringtab, stringtab_consume};
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
#[c2rust::src_loc = "10:3"]
pub const LVALUE: lvalue_t = 1;
#[c2rust::src_loc = "11:3"]
pub const ERR_LVALUE: lvalue_t = 2;
#[c2rust::src_loc = "9:3"]
pub const NOT_LVALUE: lvalue_t = 0;
#[c2rust::src_loc = "8:1"]
pub type lvalue_t = libc::c_uint;
#[no_mangle]
#[c2rust::src_loc = "19:1"]
pub unsafe extern "C" fn def_before_use(
    mut opt: *mut pass_opt_t,
    mut def: *mut ast_t,
    mut use_0: *mut ast_t,
    mut name: *const libc::c_char,
) -> bool {
    if ast_line(def) > ast_line(use_0)
        || ast_line(def) == ast_line(use_0) && ast_pos(def) > ast_pos(use_0)
    {
        ast_error(
            (*opt).check.errors,
            use_0,
            b"declaration of '%s' appears after use\0" as *const u8 as *const libc::c_char,
            name,
        );
        ast_error_continue(
            (*opt).check.errors,
            def,
            b"declaration of '%s' appears here\0" as *const u8 as *const libc::c_char,
            name,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn is_this_incomplete(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ((*(*opt).check.frame).method).is_null() {
        return 1 as libc::c_int != 0;
    }
    if ast_id((*(*opt).check.frame).method) as libc::c_uint != TK_NEW as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    let mut members: *mut ast_t =
        ast_childidx((*(*opt).check.frame).type_0, 4 as libc::c_int as usize);
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            141 | 140 | 86 => {
                let mut status: sym_status_t = SYM_NONE;
                let mut id: *mut ast_t = ast_child(member);
                ast_get(ast, ast_name(id), &mut status);
                if status as libc::c_uint != SYM_DEFINED as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        member = ast_sibling(member);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn generate_multi_dot_name(
    mut ast: *mut ast_t,
    mut def_found: *mut *mut ast_t,
) -> *const libc::c_char {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            87 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"generate_multi_dot_name\0",
            ))
            .as_ptr(),
        );
    };
    let mut def: *mut ast_t = 0 as *mut ast_t;
    let mut len: usize = 0;
    let mut temp_ast: *mut ast_t = ast;
    while !(ast_id(temp_ast) as libc::c_uint != TK_DOT as libc::c_int as libc::c_uint) {
        let mut left: ast_ptr_t = 0 as *mut ast_t;
        let mut right: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
        ast_get_children(
            temp_ast,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                .wrapping_sub(1).try_into().unwrap(),
            children.as_mut_ptr(),
        );
        def = ast_data(left) as *mut ast_t;
        temp_ast = left;
        len = (len as libc::c_ulong).wrapping_add((libc::strlen(ast_name(right))).wrapping_add(1).try_into().unwrap())
            as usize as usize;
        if !def.is_null() {
            break;
        }
    }
    match ast_id(temp_ast) as libc::c_uint {
        19 => {
            let mut left_0: ast_ptr_t = 0 as *mut ast_t;
            let mut right_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut left_0, &mut right_0, 0 as *mut *mut ast_t];
            ast_get_children(
                temp_ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                    .wrapping_sub(1).try_into().unwrap(),
                children_0.as_mut_ptr(),
            );
            if ast_id(left_0) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
                temp_ast = right_0;
                len = (len as libc::c_ulong).wrapping_add(libc::strlen(ast_name(temp_ast)).try_into().unwrap()) as usize
                    as usize;
            } else {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                            as *const u8 as *const libc::c_char,
                        116 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                            b"generate_multi_dot_name\0",
                        ))
                        .as_ptr(),
                    );
                };
            }
        }
        197 | 196 | 184 | 198 => {
            temp_ast = ast_child(temp_ast);
            len = (len as libc::c_ulong).wrapping_add(libc::strlen(ast_name(temp_ast)).try_into().unwrap()) as usize
                as usize;
        }
        102 => {
            temp_ast = ast_sibling(temp_ast);
            len = (len as libc::c_ulong).wrapping_sub(1) as usize as usize;
        }
        _ => {
            if def.is_null() {
                return stringtab(b"\0" as *const u8 as *const libc::c_char);
            }
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                        as *const u8 as *const libc::c_char,
                    144 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"generate_multi_dot_name\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    }
    if !def_found.is_null() {
        *def_found = def;
        if def.is_null() {
            return stringtab(b"\0" as *const u8 as *const libc::c_char);
        }
    }
    len = len.wrapping_add(1);
    let mut buf: *mut libc::c_char = ponyint_pool_alloc_size(len) as *mut libc::c_char;
    let mut offset: usize = 0;
    let mut name: *const libc::c_char = ast_name(temp_ast);
    let mut slen: usize = libc::strlen(name);
    memcpy(
        buf.offset(offset as isize) as *mut libc::c_void,
        name as *const libc::c_void,
        slen.try_into().unwrap(),
    );
    offset = (offset as libc::c_ulong).wrapping_add(slen.try_into().unwrap()) as usize as usize;
    temp_ast = ast_parent(temp_ast);
    while temp_ast != ast {
        *buf.offset(offset as isize) = '.' as i32 as libc::c_char;
        offset = (offset as libc::c_ulong).wrapping_add(1) as usize as usize;
        temp_ast = ast_sibling(temp_ast);
        name = ast_name(temp_ast);
        slen = libc::strlen(name);
        memcpy(
            buf.offset(offset as isize) as *mut libc::c_void,
            name as *const libc::c_void,
            slen.try_into().unwrap(),
        );
        offset = (offset as libc::c_ulong).wrapping_add(slen.try_into().unwrap()) as usize as usize;
        temp_ast = ast_parent(temp_ast);
    }
    if offset.wrapping_add(1) == len {
    } else {
        ponyint_assert_fail(
            b"(offset + 1) == len\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            178 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"generate_multi_dot_name\0",
            ))
            .as_ptr(),
        );
    };
    *buf.offset(offset as isize) = '\0' as i32 as libc::c_char;
    return stringtab_consume(buf, len);
}
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn is_matching_assign_lhs(mut a: *mut ast_t, mut b: *mut ast_t) -> bool {
    if a == b {
        return 1 as libc::c_int != 0;
    }
    if ast_id(a) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint
        && ast_id(b) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint
    {
        let mut a_name: *const libc::c_char = generate_multi_dot_name(a, 0 as *mut *mut ast_t);
        let mut b_name: *const libc::c_char = generate_multi_dot_name(b, 0 as *mut *mut ast_t);
        if a_name == b_name {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "204:1"]
unsafe extern "C" fn is_assigned_to(mut ast: *mut ast_t, mut check_result_needed: bool) -> bool {
    loop {
        let mut parent: *mut ast_t = ast_parent(ast);
        match ast_id(parent) as libc::c_uint {
            24 => {
                if !is_matching_assign_lhs(ast_child(parent), ast) {
                    return 0 as libc::c_int != 0;
                }
                if !check_result_needed {
                    return 1 as libc::c_int != 0;
                }
                return !is_result_needed(parent);
            }
            175 => {
                if ast_childcount(parent) > (1 as libc::c_int as libc::c_ulong).try_into().unwrap() {
                    return 0 as libc::c_int != 0;
                }
            }
            178 => {}
            _ => return 0 as libc::c_int != 0,
        }
        ast = parent;
    }
}
#[c2rust::src_loc = "251:1"]
unsafe extern "C" fn is_constructed_from(mut ast: *mut ast_t) -> bool {
    let mut parent: *mut ast_t = ast_parent(ast);
    if ast_id(parent) as libc::c_uint != TK_DOT as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        parent,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if left != ast {
        return 0 as libc::c_int != 0;
    }
    let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
    if def.is_null() {
        if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                    as *const u8 as *const libc::c_char,
                267 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"is_constructed_from\0",
                ))
                .as_ptr(),
            );
        };
        return 0 as libc::c_int != 0;
    }
    if ast_id(def) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
        def = ast_parent(def);
    }
    match ast_id(def) as libc::c_uint {
        84 | 85 | 140 | 141 | 86 => {
            let mut typeref: *mut ast_t = ast_childidx(def, 1 as libc::c_int as usize);
            if typeref.is_null() || (ast_data(typeref)).is_null() {
                return 0 as libc::c_int != 0;
            }
            let mut typedefn: *mut ast_t = ast_data(typeref) as *mut ast_t;
            let mut find: *mut ast_t = ast_get(typedefn, ast_name(right), 0 as *mut sym_status_t);
            return !find.is_null()
                && ast_id(find) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint;
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn valid_reference(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut status: sym_status_t,
) -> bool {
    if is_constructed_from(ast) {
        return 1 as libc::c_int != 0;
    }
    match status as libc::c_uint {
        2 => return 1 as libc::c_int != 0,
        4 | 5 => {
            if is_assigned_to(ast, 1 as libc::c_int != 0) {
                return 1 as libc::c_int != 0;
            }
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't use a consumed local or field in an expression\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        3 => {
            if is_assigned_to(ast, 1 as libc::c_int != 0) {
                return 1 as libc::c_int != 0;
            }
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't use an undefined variable in an expression\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        0 => {
            if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                        as *const u8 as *const libc::c_char,
                    328 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"valid_reference\0",
                    ))
                    .as_ptr(),
                );
            };
            return 1 as libc::c_int != 0;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            334 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"valid_reference\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "338:1"]
unsafe extern "C" fn suggest_alt_name(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            340 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"suggest_alt_name\0"))
                .as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            341 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"suggest_alt_name\0"))
                .as_ptr(),
        );
    };
    let mut name_len: usize = libc::strlen(name);
    if is_name_private(name) {
        let mut try_name: *const libc::c_char = stringtab(name.offset(1 as libc::c_int as isize));
        if !(ast_get(ast, try_name, 0 as *mut sym_status_t)).is_null() {
            return try_name;
        }
    } else {
        let mut buf: *mut libc::c_char =
            ponyint_pool_alloc_size(name_len.wrapping_add((2 as libc::c_int as libc::c_ulong).try_into().unwrap()))
                as *mut libc::c_char;
        *buf.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
        memcpy(
            buf.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            name as *const libc::c_void,
            name_len.wrapping_add(1).try_into().unwrap(),
        );
        let mut try_name_0: *const libc::c_char = stringtab_consume(
            buf,
            name_len.wrapping_add((2 as libc::c_int as libc::c_ulong).try_into().unwrap()),
        );
        if !(ast_get(ast, try_name_0, 0 as *mut sym_status_t)).is_null() {
            return try_name_0;
        }
    }
    let mut case_ast: *mut ast_t = ast_get_case(ast, name, 0 as *mut sym_status_t);
    if !case_ast.is_null() {
        let mut id: *mut ast_t = case_ast;
        let mut tk: libc::c_int = ast_id(id) as libc::c_int;
        if tk != TK_ID as libc::c_int {
            let mut first: ast_ptr_t = 0 as *mut ast_t;
            let mut second: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] =
                [&mut first, &mut second, 0 as *mut *mut ast_t];
            ast_get_children(
                case_ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                    .wrapping_sub(1).try_into().unwrap(),
                children.as_mut_ptr(),
            );
            tk = ast_id(first) as libc::c_int;
            if tk == TK_ID as libc::c_int {
                id = first;
            } else {
                tk = ast_id(second) as libc::c_int;
                if tk == TK_ID as libc::c_int {
                    id = second;
                }
            }
        }
        if tk == TK_ID as libc::c_int {
            let mut try_name_1: *const libc::c_char = ast_name(id);
            if !(ast_get(ast, try_name_1, 0 as *mut sym_status_t)).is_null() {
                return try_name_1;
            }
        }
    }
    return 0 as *const libc::c_char;
}
#[c2rust::src_loc = "401:1"]
unsafe extern "C" fn refer_this(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_THIS\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            403 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"refer_this\0")).as_ptr(),
        );
    };
    let mut status: sym_status_t = SYM_NONE;
    ast_get(
        ast,
        stringtab(b"this\0" as *const u8 as *const libc::c_char),
        &mut status,
    );
    if status as libc::c_uint == SYM_CONSUMED as libc::c_int as libc::c_uint
        || status as libc::c_uint == SYM_CONSUMED_SAME_EXPR as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can't use a consumed 'this' in an expression\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if status as libc::c_uint == SYM_NONE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"status == SYM_NONE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            415 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"refer_this\0")).as_ptr(),
        );
    };
    if is_this_incomplete(opt, ast) {
        ast_setflag(ast, AST_FLAG_INCOMPLETE as libc::c_int as u32);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn refer_reference(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> bool {
    let mut ast: *mut ast_t = *astp;
    let mut name: *const libc::c_char = ast_name(ast_child(ast));
    if is_name_dontcare(name) {
        ast_setid(ast, TK_DONTCAREREF);
        return 1 as libc::c_int != 0;
    }
    let mut status: sym_status_t = SYM_NONE;
    let mut def: *mut ast_t = ast_get(ast, name, &mut status);
    if def.is_null() {
        def = ast_get_provided_symbol_definition(ast, name, &mut status);
    }
    if def.is_null() {
        let mut alt_name: *const libc::c_char = suggest_alt_name(ast, name);
        if alt_name.is_null() {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't find declaration of '%s'\0" as *const u8 as *const libc::c_char,
                name,
            );
        } else {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't find declaration of '%s', did you mean '%s'?\0" as *const u8
                    as *const libc::c_char,
                name,
                alt_name,
            );
        }
        return 0 as libc::c_int != 0;
    }
    ast_setdata(ast, def as *mut libc::c_void);
    match ast_id(def) as libc::c_uint {
        137 => {
            if ast_id(ast_parent(ast)) as libc::c_uint != TK_DOT as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"a package can only appear as a prefix to a type\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            ast_setid(ast, TK_PACKAGEREF);
            return 1 as libc::c_int != 0;
        }
        72 | 73 | 71 | 163 | 74 | 75 | 76 | 77 => {
            ast_setid(ast, TK_TYPEREF);
            ast_add(ast, ast_from(ast, TK_NONE));
            ast_append(ast, ast_from(ast, TK_NONE));
            return 1 as libc::c_int != 0;
        }
        140 | 141 | 86 | 88 | 90 | 89 => {
            let mut dot: *mut ast_t = ast_from(ast, TK_DOT);
            ast_add(dot, ast_child(ast));
            let mut self_0: *mut ast_t = ast_from(ast, TK_THIS);
            ast_add(dot, self_0);
            ast_replace(astp, dot);
            ast = *astp;
            return refer_this(opt, self_0) as libc::c_int != 0
                && refer_dot(opt, ast) as libc::c_int != 0;
        }
        165 => {
            if !((*(*opt).check.frame).def_arg).is_null() {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't reference a parameter in a default argument\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            if !def_before_use(opt, def, ast, name) {
                return 0 as libc::c_int != 0;
            }
            if !valid_reference(opt, ast, status) {
                return 0 as libc::c_int != 0;
            }
            ast_setid(ast, TK_PARAMREF);
            return 1 as libc::c_int != 0;
        }
        84 | 85 | 182 => {
            if !def_before_use(opt, def, ast, name) {
                return 0 as libc::c_int != 0;
            }
            if !valid_reference(opt, ast, status) {
                return 0 as libc::c_int != 0;
            }
            if ast_id(def) as libc::c_uint == TK_VAR as libc::c_int as libc::c_uint {
                ast_setid(ast, TK_VARREF);
            } else {
                ast_setid(ast, TK_LETREF);
            }
            return 1 as libc::c_int != 0;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            552 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"refer_reference\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn refer_packageref_dot(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            558 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"refer_packageref_dot\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if ast_id(left) as libc::c_uint == TK_PACKAGEREF as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(left) == TK_PACKAGEREF\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            560 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"refer_packageref_dot\0"))
                .as_ptr(),
        );
    };
    if ast_id(right) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(right) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            561 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"refer_packageref_dot\0"))
                .as_ptr(),
        );
    };
    let mut package_name: *const libc::c_char = ast_name(ast_child(left));
    let mut package: *mut ast_t = ast_get(left, package_name, 0 as *mut sym_status_t);
    if package.is_null() {
        ast_error(
            (*opt).check.errors,
            right,
            b"can't access package '%s'\0" as *const u8 as *const libc::c_char,
            package_name,
        );
        return 0 as libc::c_int != 0;
    }
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            574 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"refer_packageref_dot\0"))
                .as_ptr(),
        );
    };
    let mut type_name: *const libc::c_char = ast_name(right);
    let mut def: *mut ast_t = ast_get(package, type_name, 0 as *mut sym_status_t);
    if def.is_null() {
        ast_error(
            (*opt).check.errors,
            right,
            b"can't find type '%s' in package '%s'\0" as *const u8 as *const libc::c_char,
            type_name,
            package_name,
        );
        return 0 as libc::c_int != 0;
    }
    ast_setdata(ast, def as *mut libc::c_void);
    ast_setid(ast, TK_TYPEREF);
    ast_append(ast, ast_from(ast, TK_NONE));
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "593:1"]
unsafe extern "C" fn refer_this_dot(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            595 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_this_dot\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if ast_id(left) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(left) == TK_THIS\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            597 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_this_dot\0"))
                .as_ptr(),
        );
    };
    if ast_id(right) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(right) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            598 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_this_dot\0"))
                .as_ptr(),
        );
    };
    let mut name: *const libc::c_char = ast_name(right);
    let mut status: sym_status_t = SYM_NONE;
    let mut def: *mut ast_t = ast_get(ast, name, &mut status);
    ast_setdata(ast, def as *mut libc::c_void);
    if def.is_null() {
        let mut alt_name: *const libc::c_char = suggest_alt_name(ast, name);
        if alt_name.is_null() {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't find declaration of '%s'\0" as *const u8 as *const libc::c_char,
                name,
            );
        } else {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't find declaration of '%s', did you mean '%s'?\0" as *const u8
                    as *const libc::c_char,
                name,
                alt_name,
            );
        }
        return 0 as libc::c_int != 0;
    }
    match ast_id(def) as libc::c_uint {
        140 | 141 | 86 => {
            if !valid_reference(opt, ast, status) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "635:1"]
unsafe extern "C" fn refer_multi_dot(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            637 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"refer_multi_dot\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    let mut name: *const libc::c_char = generate_multi_dot_name(ast, 0 as *mut *mut ast_t);
    let mut status: sym_status_t = SYM_NONE;
    ast_get(ast, name, &mut status);
    if !valid_reference(opt, ast, status) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "654:1"]
pub unsafe extern "C" fn refer_dot(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            656 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"refer_dot\0")).as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    match ast_id(left) as libc::c_uint {
        185 => return refer_packageref_dot(opt, ast),
        102 => return refer_this_dot(opt, ast),
        198 | 196 | 197 | 19 => {
            if ast_checkflag(ast, AST_FLAG_FCNSM_REASGN as libc::c_int as u32) != 0
                && ast_id(ast_parent(ast)) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint
                && ast_id(ast_parent(ast)) as libc::c_uint
                    != TK_QUALIFY as libc::c_int as libc::c_uint
            {
                return refer_multi_dot(opt, ast);
            }
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "681:1"]
unsafe extern "C" fn qualify_typeref(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    let mut typeref: *mut ast_t = ast_child(ast);
    if ast_id(ast_childidx(typeref, 2 as libc::c_int as usize)) as libc::c_uint
        == TK_TYPEARGS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    let mut def: *mut ast_t = ast_data(typeref) as *mut ast_t;
    if !def.is_null() {
    } else {
        ponyint_assert_fail(
            b"def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            692 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"qualify_typeref\0"))
                .as_ptr(),
        );
    };
    let mut typeparams: *mut ast_t = ast_childidx(def, 1 as libc::c_int as usize);
    if ast_id(typeparams) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    ast_setdata(ast, def as *mut libc::c_void);
    ast_setid(ast, TK_TYPEREF);
    let mut first_child: *mut ast_t = ast_pop(ast);
    if typeref == first_child {
    } else {
        ponyint_assert_fail(
            b"typeref == first_child\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            707 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"qualify_typeref\0"))
                .as_ptr(),
        );
    };
    let mut package: *mut ast_t = ast_pop(typeref);
    let mut type_name: *mut ast_t = ast_pop(typeref);
    ast_free(typeref);
    ast_add(ast, type_name);
    ast_add(ast, package);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "718:1"]
pub unsafe extern "C" fn refer_qualify(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_QUALIFY as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_QUALIFY\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            720 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"refer_qualify\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast_child(ast)) as libc::c_uint == TK_TYPEREF as libc::c_int as libc::c_uint {
        return qualify_typeref(opt, ast);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "728:1"]
unsafe extern "C" fn error_check_used_decl(mut frame: *mut errorframe_t, mut ast: *mut ast_t) {
    let mut parent: *mut ast_t = ast_parent(ast);
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            732 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"error_check_used_decl\0"))
                .as_ptr(),
        );
    };
    let mut parent_id: token_id = ast_id(parent);
    if parent_id as libc::c_uint == TK_VAR as libc::c_int as libc::c_uint
        || parent_id as libc::c_uint == TK_LET as libc::c_int as libc::c_uint
    {
        ast_error_frame(
            frame,
            parent,
            b"the previous value of '%s' is used because you are trying to use the resulting value of this %s declaration\0"
                as *const u8 as *const libc::c_char,
            ast_print_type(ast),
            ast_print_type(parent),
        );
    }
}
#[c2rust::src_loc = "740:1"]
unsafe extern "C" fn error_consumed_but_used(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) {
    let mut frame: errorframe_t = 0 as errorframe_t;
    ast_error_frame(
        &mut frame as *mut errorframe_t,
        ast,
        b"the left side is consumed but its value is used\0" as *const u8 as *const libc::c_char,
    );
    error_check_used_decl(&mut frame, ast);
    errorframe_report(&mut frame, (*opt).check.errors);
}
#[c2rust::src_loc = "752:1"]
unsafe extern "C" fn error_undefined_but_used(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) {
    let mut frame: errorframe_t = 0 as errorframe_t;
    ast_error_frame(
        &mut frame as *mut errorframe_t,
        ast,
        b"the left side is undefined but its value is used\0" as *const u8 as *const libc::c_char,
    );
    error_check_used_decl(&mut frame, ast);
    errorframe_report(&mut frame, (*opt).check.errors);
}
#[c2rust::src_loc = "764:1"]
unsafe extern "C" fn assign_multi_dot(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut need_value: bool,
) -> lvalue_t {
    if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            766 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"assign_multi_dot\0"))
                .as_ptr(),
        );
    };
    let mut name: *const libc::c_char = generate_multi_dot_name(ast, 0 as *mut *mut ast_t);
    let mut status: sym_status_t = SYM_NONE;
    ast_get(ast, name, &mut status);
    match status as libc::c_uint {
        3 => {
            if need_value {
                error_undefined_but_used(opt, ast);
                return ERR_LVALUE;
            }
            ast_setstatus(ast, name, SYM_DEFINED);
            return LVALUE;
        }
        2 => return LVALUE,
        4 | 5 => {
            let mut ok: lvalue_t = LVALUE;
            if need_value {
                error_consumed_but_used(opt, ast);
                ok = ERR_LVALUE;
            }
            if !((*(*opt).check.frame).try_expr).is_null() {
                if status as libc::c_uint == SYM_CONSUMED as libc::c_int as libc::c_uint {
                    ast_error(
                        (*opt).check.errors,
                        ast,
                        b"can't reassign to a consumed identifier in a try expression unless it is reassigned in the same expression\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ok = (if ok as libc::c_uint == ERR_LVALUE as libc::c_int as libc::c_uint {
                        ERR_LVALUE as libc::c_int
                    } else {
                        NOT_LVALUE as libc::c_int
                    }) as lvalue_t;
                }
            }
            if ok as libc::c_uint == LVALUE as libc::c_int as libc::c_uint {
                ast_setstatus(ast, name, SYM_DEFINED);
            }
            return ok;
        }
        0 => {
            if ast_id(ast) as libc::c_uint == TK_DOT as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(ast) == TK_DOT\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                        as *const u8 as *const libc::c_char,
                    820 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"assign_multi_dot\0",
                    ))
                    .as_ptr(),
                );
            };
            return LVALUE;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            826 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"assign_multi_dot\0"))
                .as_ptr(),
        );
    };
    return NOT_LVALUE;
}
#[c2rust::src_loc = "830:1"]
unsafe extern "C" fn assign_id(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut let_0: bool,
    mut need_value: bool,
) -> lvalue_t {
    if ast_id(ast) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            832 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"assign_id\0")).as_ptr(),
        );
    };
    let mut name: *const libc::c_char = ast_name(ast);
    let mut status: sym_status_t = SYM_NONE;
    ast_get(ast, name, &mut status);
    match status as libc::c_uint {
        3 => {
            if need_value {
                error_undefined_but_used(opt, ast);
                return ERR_LVALUE;
            }
            ast_setstatus(ast, name, SYM_DEFINED);
            return LVALUE;
        }
        2 => {
            if let_0 {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't assign to a let or embed definition more than once\0" as *const u8
                        as *const libc::c_char,
                );
                return NOT_LVALUE;
            }
            return LVALUE;
        }
        4 | 5 => {
            let mut ok: lvalue_t = LVALUE;
            if need_value {
                error_consumed_but_used(opt, ast);
                ok = ERR_LVALUE;
            }
            if let_0 {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can't assign to a let or embed definition more than once\0" as *const u8
                        as *const libc::c_char,
                );
                ok = (if ok as libc::c_uint == ERR_LVALUE as libc::c_int as libc::c_uint {
                    ERR_LVALUE as libc::c_int
                } else {
                    NOT_LVALUE as libc::c_int
                }) as lvalue_t;
            }
            if !((*(*opt).check.frame).try_expr).is_null() {
                if status as libc::c_uint == SYM_CONSUMED as libc::c_int as libc::c_uint {
                    ast_error(
                        (*opt).check.errors,
                        ast,
                        b"can't reassign to a consumed identifier in a try expression unless it is reassigned in the same expression\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ok = (if ok as libc::c_uint == ERR_LVALUE as libc::c_int as libc::c_uint {
                        ERR_LVALUE as libc::c_int
                    } else {
                        NOT_LVALUE as libc::c_int
                    }) as lvalue_t;
                }
            }
            if ok as libc::c_uint == LVALUE as libc::c_int as libc::c_uint {
                ast_setstatus(ast, name, SYM_DEFINED);
            }
            return ok;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            900 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"assign_id\0")).as_ptr(),
        );
    };
    return NOT_LVALUE;
}
#[c2rust::src_loc = "904:1"]
unsafe extern "C" fn is_lvalue(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut need_value: bool,
) -> lvalue_t {
    match ast_id(ast) as libc::c_uint {
        87 => return LVALUE,
        199 => {
            return (if need_value as libc::c_int != 0 {
                NOT_LVALUE as libc::c_int
            } else {
                LVALUE as libc::c_int
            }) as lvalue_t;
        }
        84 | 85 => {
            return assign_id(
                opt,
                ast_child(ast),
                ast_id(ast) as libc::c_uint == TK_LET as libc::c_int as libc::c_uint,
                need_value,
            );
        }
        196 => {
            let mut id: *mut ast_t = ast_child(ast);
            return assign_id(opt, id, 0 as libc::c_int != 0, need_value);
        }
        197 => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't reassign to a let local\0" as *const u8 as *const libc::c_char,
            );
            return NOT_LVALUE;
        }
        19 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                    .wrapping_sub(1).try_into().unwrap(),
                children.as_mut_ptr(),
            );
            match ast_id(left) as libc::c_uint {
                102 => {
                    let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
                    if def.is_null() {
                        return NOT_LVALUE;
                    }
                    match ast_id(def) as libc::c_uint {
                        140 => {
                            return (if assign_id(opt, right, 0 as libc::c_int != 0, need_value)
                                as libc::c_uint
                                != 0
                            {
                                LVALUE as libc::c_int
                            } else {
                                NOT_LVALUE as libc::c_int
                            }) as lvalue_t;
                        }
                        141 | 86 => {
                            return (if assign_id(opt, right, 1 as libc::c_int != 0, need_value)
                                as libc::c_uint
                                != 0
                            {
                                LVALUE as libc::c_int
                            } else {
                                NOT_LVALUE as libc::c_int
                            }) as lvalue_t;
                        }
                        _ => return NOT_LVALUE,
                    }
                }
                196 | 197 | 19 => return assign_multi_dot(opt, ast, need_value),
                _ => {}
            }
            return LVALUE;
        }
        178 => {
            let mut child: *mut ast_t = ast_child(ast);
            while !child.is_null() {
                match is_lvalue(opt, child, need_value) as libc::c_uint {
                    2 => return ERR_LVALUE,
                    0 => return NOT_LVALUE,
                    1 | _ => {}
                }
                child = ast_sibling(child);
            }
            return LVALUE;
        }
        175 => {
            let mut child_0: *mut ast_t = ast_child(ast);
            if !(ast_sibling(child_0)).is_null() {
                return NOT_LVALUE;
            }
            return is_lvalue(opt, child_0, need_value);
        }
        _ => {}
    }
    return NOT_LVALUE;
}
#[c2rust::src_loc = "1001:1"]
unsafe extern "C" fn refer_pre_call(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_CALL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CALL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1003 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_pre_call\0"))
                .as_ptr(),
        );
    };
    let mut lhs: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut lhs,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if !ast_passes_subtree(&mut positional, opt, PASS_REFER)
        || !ast_passes_subtree(&mut named, opt, PASS_REFER)
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1015:1"]
unsafe extern "C" fn refer_pre_assign(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_ASSIGN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ASSIGN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1017 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"refer_pre_assign\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if !ast_passes_subtree(&mut right, opt, PASS_REFER) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1028:1"]
unsafe extern "C" fn refer_assign(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_ASSIGN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ASSIGN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1030 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"refer_assign\0")).as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    match is_lvalue(opt, left, is_result_needed(ast)) as libc::c_uint {
        0 => {
            if ast_id(left) as libc::c_uint == TK_DONTCAREREF as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    left,
                    b"can't read from '_'\0" as *const u8 as *const libc::c_char,
                );
            } else {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"left side must be something that can be assigned to\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int != 0;
        }
        2 => return 0 as libc::c_int != 0,
        1 | _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1052:1"]
unsafe extern "C" fn ast_get_child(mut ast: *mut ast_t, mut name: *const libc::c_char) -> bool {
    let mut assign_name: *const libc::c_char = 0 as *const libc::c_char;
    match ast_id(ast) as libc::c_uint {
        8 => {
            assign_name = ast_name(ast);
        }
        19 => {
            assign_name = generate_multi_dot_name(ast, 0 as *mut *mut ast_t);
        }
        _ => {}
    }
    if assign_name == name {
        return 1 as libc::c_int != 0;
    }
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        if ast_get_child(child, name) {
            return 1 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1091:1"]
unsafe extern "C" fn check_assigned_same_expression(
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
    mut ret_assign_ast: *mut *mut ast_t,
) -> bool {
    let mut assign_ast: *mut ast_t = ast;
    while !assign_ast.is_null()
        && ast_id(assign_ast) as libc::c_uint != TK_ASSIGN as libc::c_int as libc::c_uint
    {
        assign_ast = ast_parent(assign_ast);
    }
    *ret_assign_ast = assign_ast;
    if assign_ast.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut assign_left: *mut ast_t = ast_child(assign_ast);
    return ast_get_child(assign_left, name);
}
#[c2rust::src_loc = "1107:1"]
unsafe extern "C" fn set_flag_recursive(mut outer: *mut ast_t, mut flag: u32) {
    if !outer.is_null() {
    } else {
        ponyint_assert_fail(
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1109 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"set_flag_recursive\0"))
                .as_ptr(),
        );
    };
    ast_setflag(outer, flag);
    let mut child: *mut ast_t = ast_child(outer);
    while !child.is_null() {
        set_flag_recursive(child, flag);
        child = ast_sibling(child);
    }
}
#[c2rust::src_loc = "1123:1"]
unsafe extern "C" fn refer_consume(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_CONSUME as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CONSUME\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1125 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"refer_consume\0"))
                .as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut term: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut cap, &mut term, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut consumed_same_expr: bool = 0 as libc::c_int != 0;
    match ast_id(term) as libc::c_uint {
        196 | 197 | 198 => {
            let mut id: *mut ast_t = ast_child(term);
            name = ast_name(id);
            let mut assign_ast: *mut ast_t = 0 as *mut ast_t;
            if check_assigned_same_expression(id, name, &mut assign_ast) {
                consumed_same_expr = 1 as libc::c_int != 0;
                ast_setflag(assign_ast, AST_FLAG_CNSM_REASGN as libc::c_int as u32);
            }
        }
        102 => {
            name = stringtab(b"this\0" as *const u8 as *const libc::c_char);
        }
        19 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                term,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                    .wrapping_sub(1).try_into().unwrap(),
                children_0.as_mut_ptr(),
            );
            let mut def: *mut ast_t = 0 as *mut ast_t;
            if ast_id(left) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
                def = ast_data(term) as *mut ast_t;
                name = ast_name(right);
                if ast_id(def) as libc::c_uint == TK_FLET as libc::c_int as libc::c_uint
                    || ast_id(def) as libc::c_uint == TK_EMBED as libc::c_int as libc::c_uint
                {
                    ast_error(
                        (*opt).check.errors,
                        ast,
                        b"can't consume a let or embed field\0" as *const u8 as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            } else {
                name = generate_multi_dot_name(term, &mut def);
            }
            if def.is_null() {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"cannot consume an unknown field type\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            let mut assign_ast_0: *mut ast_t = 0 as *mut ast_t;
            if !check_assigned_same_expression(ast, name, &mut assign_ast_0) {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"consuming a field is only allowed if it is reassigned in the same expression\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            consumed_same_expr = 1 as libc::c_int != 0;
            set_flag_recursive(assign_ast_0, AST_FLAG_FCNSM_REASGN as libc::c_int as u32);
        }
        _ => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"consume must take 'this', a local, or a parameter\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    if !((*(*opt).check.frame).loop_cond).is_null()
        && !ast_within_scope((*(*opt).check.frame).loop_cond, ast, name)
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can't consume from an outer scope in a loop condition\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if consumed_same_expr {
        ast_setstatus(ast, name, SYM_CONSUMED_SAME_EXPR);
    } else {
        ast_setstatus(ast, name, SYM_CONSUMED);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1233:1"]
unsafe extern "C" fn refer_pre_new(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_NEW\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1236 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"refer_pre_new\0"))
                .as_ptr(),
        );
    };
    let mut members: *mut ast_t = ast_parent(ast);
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            140 | 141 | 86 => {
                let mut id: ast_ptr_t = 0 as *mut ast_t;
                let mut type_0: ast_ptr_t = 0 as *mut ast_t;
                let mut expr: ast_ptr_t = 0 as *mut ast_t;
                let mut children: [*mut *mut ast_t; 4] =
                    [&mut id, &mut type_0, &mut expr, 0 as *mut *mut ast_t];
                ast_get_children(
                    member,
                    ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                        .wrapping_sub(1),
                    children.as_mut_ptr(),
                );
                ast_setstatus(ast, ast_name(id), SYM_UNDEFINED);
            }
            _ => {}
        }
        member = ast_sibling(member);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1264:1"]
unsafe extern "C" fn refer_local(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1266 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_local\0")).as_ptr(),
        );
    };
    if !(ast_type(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_type(ast) != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1267 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_local\0")).as_ptr(),
        );
    };
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut id, &mut type_0, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1270 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_local\0")).as_ptr(),
        );
    };
    let mut is_dontcare: bool = is_name_dontcare(ast_name(id));
    if ast_id(type_0) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        if !is_dontcare && !is_assigned_to(ast, 0 as libc::c_int != 0) {
            ast_error(
                (*opt).check.errors,
                ast,
                b"locals must specify a type or be assigned a value\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    } else if ast_id(ast) as libc::c_uint == TK_LET as libc::c_int as libc::c_uint {
        if !is_assigned_to(ast, 0 as libc::c_int != 0) {
            ast_error(
                (*opt).check.errors,
                ast,
                b"can't declare a let local without assigning to it\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    }
    if is_dontcare {
        ast_setid(ast, TK_DONTCARE);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1301:1"]
unsafe extern "C" fn refer_seq(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1304 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"refer_seq\0")).as_ptr(),
        );
    };
    if ast_checkflag(
        ast_childlast(ast),
        AST_FLAG_JUMPS_AWAY as libc::c_int as u32,
    ) != 0
    {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    if ast_has_scope(ast) {
        let mut parent: *mut ast_t = ast_parent(ast);
        match ast_id(parent) as libc::c_uint {
            124 | 125 => {
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
                    parent,
                    ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                        .wrapping_sub(1),
                    children.as_mut_ptr(),
                );
                if body == ast {
                    ast_inheritbranch(else_clause, body);
                    ast_consolidate_branches(else_clause, 2 as libc::c_int as usize);
                } else if else_clause == ast {
                    ast_inheritbranch(then_clause, else_clause);
                    ast_consolidate_branches(then_clause, 2 as libc::c_int as usize);
                }
            }
            206 => {
                let mut body_0: ast_ptr_t = 0 as *mut ast_t;
                let mut dispose_clause: ast_ptr_t = 0 as *mut ast_t;
                let mut children_0: [*mut *mut ast_t; 3] =
                    [&mut body_0, &mut dispose_clause, 0 as *mut *mut ast_t];
                ast_get_children(
                    parent,
                    (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
                        .wrapping_sub(1).try_into().unwrap(),
                    children_0.as_mut_ptr(),
                );
                if body_0 == ast {
                    ast_inheritbranch(dispose_clause, body_0);
                    ast_consolidate_branches(dispose_clause, 2 as libc::c_int as usize);
                }
            }
            118 => {
                let mut body_1: ast_ptr_t = 0 as *mut ast_t;
                let mut cond: ast_ptr_t = 0 as *mut ast_t;
                let mut else_clause_0: ast_ptr_t = 0 as *mut ast_t;
                let mut children_1: [*mut *mut ast_t; 4] = [
                    &mut body_1,
                    &mut cond,
                    &mut else_clause_0,
                    0 as *mut *mut ast_t,
                ];
                ast_get_children(
                    parent,
                    ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                        .wrapping_sub(1),
                    children_1.as_mut_ptr(),
                );
                if body_1 == ast {
                    ast_inheritstatus(cond, body_1);
                } else if cond == ast {
                    ast_inheritbranch(else_clause_0, cond);
                    ast_consolidate_branches(else_clause_0, 2 as libc::c_int as usize);
                }
            }
            _ => {}
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1374:1"]
unsafe extern "C" fn refer_if(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_IF as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_IFDEF as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_IF) || (ast_id(ast) == TK_IFDEF)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1377 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"refer_if\0")).as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut cond, &mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut branch_count: usize = 0;
    if ast_checkflag(left, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, left);
    }
    if ast_checkflag(right, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, right);
    }
    ast_consolidate_branches(ast, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), ast);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1406:1"]
unsafe extern "C" fn refer_iftype(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_IFTYPE_SET as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_IFTYPE_SET\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1409 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"refer_iftype\0")).as_ptr(),
        );
    };
    let mut left_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left_clause, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    let mut sub: ast_ptr_t = 0 as *mut ast_t;
    let mut super_0: ast_ptr_t = 0 as *mut ast_t;
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 4] =
        [&mut sub, &mut super_0, &mut left, 0 as *mut *mut ast_t];
    ast_get_children(
        left_clause,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut branch_count: usize = 0;
    if ast_checkflag(left, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, left);
    }
    if ast_checkflag(right, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, right);
    }
    ast_consolidate_branches(ast, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), ast);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1440:1"]
unsafe extern "C" fn refer_while(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_WHILE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_WHILE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1442 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_while\0")).as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut cond, &mut body, &mut else_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut errorf: errorframe_t = 0 as errorframe_t;
    if !ast_all_consumes_in_scope(body, body, &mut errorf) {
        errorframe_report(&mut errorf, (*opt).check.errors);
        return 0 as libc::c_int != 0;
    }
    let mut branch_count: usize = 0;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, else_clause);
    }
    ast_consolidate_branches(ast, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), ast);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1478:1"]
unsafe extern "C" fn refer_repeat(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_REPEAT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_REPEAT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1480 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"refer_repeat\0")).as_ptr(),
        );
    };
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut body, &mut cond, &mut else_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut errorf: errorframe_t = 0 as errorframe_t;
    if !ast_all_consumes_in_scope(body, body, &mut errorf) {
        errorframe_report(&mut errorf, (*opt).check.errors);
        return 0 as libc::c_int != 0;
    }
    let mut branch_count: usize = 0;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        if ast_checkflag(body, AST_FLAG_MAY_BREAK as libc::c_int as u32) != 0 {
            branch_count = branch_count.wrapping_add(1);
            ast_inheritbranch(ast, else_clause);
        }
    }
    ast_consolidate_branches(ast, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), ast);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1530:1"]
unsafe extern "C" fn refer_match(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_MATCH as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_MATCH\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1533 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_match\0")).as_ptr(),
        );
    };
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut cases: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [
        &mut expr,
        &mut cases,
        &mut else_clause,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut branch_count: usize = 0;
    if ast_checkflag(cases, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, cases);
    }
    if ast_id(else_clause) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        branch_count = branch_count.wrapping_add(1);
    } else if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(ast, else_clause);
    }
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_consolidate_branches(ast, branch_count);
    ast_inheritstatus(ast_parent(ast), ast);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1566:1"]
unsafe extern "C" fn refer_cases(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_CASES as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CASES\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1568 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_cases\0")).as_ptr(),
        );
    };
    let mut the_case: *mut ast_t = ast_child(ast);
    if the_case.is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"match must have at least one case\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut branch_count: usize = 0;
    while !the_case.is_null() {
        let mut pattern: ast_ptr_t = 0 as *mut ast_t;
        let mut guard: ast_ptr_t = 0 as *mut ast_t;
        let mut body: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 4] =
            [&mut pattern, &mut guard, &mut body, 0 as *mut *mut ast_t];
        ast_get_children(
            the_case,
            ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                .wrapping_sub(1),
            children.as_mut_ptr(),
        );
        if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
            branch_count = branch_count.wrapping_add(1);
            ast_inheritbranch(ast, the_case);
        }
        the_case = ast_sibling(the_case);
    }
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_consolidate_branches(ast, branch_count);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1600:1"]
unsafe extern "C" fn refer_try(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_TRY as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_TRY_NO_CHECK as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_TRY) || (ast_id(ast) == TK_TRY_NO_CHECK)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1602 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"refer_try\0")).as_ptr(),
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
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut branch_count: usize = 0;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(then_clause, body);
    }
    if ast_checkflag(else_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(then_clause, else_clause);
    }
    if ast_checkflag(then_clause, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) != 0 {
        ast_error(
            (*opt).check.errors,
            then_clause,
            b"then clause always terminates the function\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    ast_consolidate_branches(then_clause, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), then_clause);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1638:1"]
unsafe extern "C" fn refer_disposing_block(mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_DISPOSING_BLOCK as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_DISPOSING_BLOCK\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1640 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"refer_disposing_block\0"))
                .as_ptr(),
        );
    };
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut dispose_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut body, &mut dispose_clause, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    let mut branch_count: usize = 0;
    if ast_checkflag(body, AST_FLAG_JUMPS_AWAY as libc::c_int as u32) == 0 {
        branch_count = branch_count.wrapping_add(1);
        ast_inheritbranch(dispose_clause, body);
    }
    ast_consolidate_branches(dispose_clause, branch_count);
    if branch_count == 0 {
        ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    }
    ast_inheritstatus(ast_parent(ast), dispose_clause);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1663:1"]
unsafe extern "C" fn refer_recover(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_RECOVER as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_RECOVER\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1666 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"refer_recover\0"))
                .as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut cap, &mut expr, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>().try_into().unwrap())
            .wrapping_sub(1).try_into().unwrap(),
        children.as_mut_ptr(),
    );
    ast_inheritstatus(ast_parent(ast), expr);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1675:1"]
unsafe extern "C" fn refer_break(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_BREAK as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_BREAK\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1677 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_break\0")).as_ptr(),
        );
    };
    if ((*(*opt).check.frame).loop_body).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"must be in a loop\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    ast_setflag(
        (*(*opt).check.frame).loop_body,
        AST_FLAG_MAY_BREAK as libc::c_int as u32,
    );
    let mut errorf: errorframe_t = 0 as errorframe_t;
    if !ast_all_consumes_in_scope((*(*opt).check.frame).loop_body, ast, &mut errorf) {
        errorframe_report(&mut errorf, (*opt).check.errors);
        return 0 as libc::c_int != 0;
    }
    if (ast_sibling(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_sibling(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1695 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_break\0")).as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1702:1"]
unsafe extern "C" fn refer_continue(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_CONTINUE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_CONTINUE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1704 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_continue\0"))
                .as_ptr(),
        );
    };
    if ((*(*opt).check.frame).loop_body).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"must be in a loop\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    let mut errorf: errorframe_t = 0 as errorframe_t;
    if !ast_all_consumes_in_scope((*(*opt).check.frame).loop_body, ast, &mut errorf) {
        errorframe_report(&mut errorf, (*opt).check.errors);
        return 0 as libc::c_int != 0;
    }
    if (ast_sibling(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_sibling(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1720 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"refer_continue\0"))
                .as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1727:1"]
unsafe extern "C" fn refer_return(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if ast_id(ast) as libc::c_uint == TK_RETURN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_RETURN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1729 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"refer_return\0")).as_ptr(),
        );
    };
    if (ast_sibling(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_sibling(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1732 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"refer_return\0")).as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    if ast_id((*(*opt).check.frame).method) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
        && is_this_incomplete(opt, ast) as libc::c_int != 0
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"all fields must be defined before constructor returns\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1747:1"]
unsafe extern "C" fn refer_error(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if (ast_sibling(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_sibling(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1751 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"refer_error\0")).as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1757:1"]
unsafe extern "C" fn refer_compile_error(mut _opt: *mut pass_opt_t, mut ast: *mut ast_t) -> bool {
    if (ast_sibling(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_sibling(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0" as *const u8
                as *const libc::c_char,
            1761 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"refer_compile_error\0"))
                .as_ptr(),
        );
    };
    ast_setflag(ast, AST_FLAG_JUMPS_AWAY as libc::c_int as u32);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1767:1"]
pub unsafe extern "C" fn pass_pre_refer(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    let mut r: bool = 1 as libc::c_int != 0;
    match ast_id(ast) as libc::c_uint {
        88 => {
            r = refer_pre_new(options, ast);
        }
        177 => {
            r = refer_pre_call(options, ast);
        }
        24 => {
            r = refer_pre_assign(options, ast);
        }
        _ => {}
    }
    if !r {
        if errors_get_count((*options).check.errors) > 0 {
        } else {
            ponyint_assert_fail(
                b"errors_get_count(options->check.errors) > 0\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                    as *const u8 as *const libc::c_char,
                1783 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"pass_pre_refer\0"))
                    .as_ptr(),
            );
        };
        return AST_ERROR;
    }
    return AST_OK;
}
#[no_mangle]
#[c2rust::src_loc = "1790:1"]
pub unsafe extern "C" fn pass_refer(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    let mut r: bool = 1 as libc::c_int != 0;
    match ast_id(ast) as libc::c_uint {
        184 => {
            r = refer_reference(options, astp);
        }
        19 => {
            r = refer_dot(options, ast);
        }
        176 => {
            r = refer_qualify(options, ast);
        }
        24 => {
            r = refer_assign(options, ast);
        }
        106 => {
            r = refer_consume(options, ast);
        }
        102 => {
            r = refer_this(options, ast);
        }
        84 | 85 => {
            r = refer_local(options, ast);
        }
        175 => {
            r = refer_seq(options, ast);
        }
        109 | 108 => {
            r = refer_if(options, ast);
        }
        111 => {
            r = refer_iftype(options, ast);
        }
        116 => {
            r = refer_while(options, ast);
        }
        118 => {
            r = refer_repeat(options, ast);
        }
        122 => {
            r = refer_match(options, ast);
        }
        180 => {
            r = refer_cases(options, ast);
        }
        125 | 124 => {
            r = refer_try(options, ast);
        }
        206 => {
            r = refer_disposing_block(ast);
        }
        107 => {
            r = refer_recover(options, ast);
        }
        104 => {
            r = refer_break(options, ast);
        }
        105 => {
            r = refer_continue(options, ast);
        }
        103 => {
            r = refer_return(options, ast);
        }
        127 => {
            r = refer_error(options, ast);
        }
        128 => {
            r = refer_compile_error(options, ast);
        }
        _ => {}
    }
    if !r {
        if errors_get_count((*options).check.errors) > 0 {
        } else {
            ponyint_assert_fail(
                b"errors_get_count(options->check.errors) > 0\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/refer.c\0"
                    as *const u8 as *const libc::c_char,
                1833 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pass_refer\0"))
                    .as_ptr(),
            );
        };
        return AST_ERROR;
    }
    return AST_OK;
}
