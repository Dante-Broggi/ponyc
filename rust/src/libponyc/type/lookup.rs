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
        #[c2rust::src_loc = "115:1"]
        pub fn errorframe_append(first: *mut errorframe_t, second: *mut errorframe_t);
        #[c2rust::src_loc = "122:1"]
        pub fn errorframe_report(frame: *mut errorframe_t, errors: *mut errors_t);
        #[c2rust::src_loc = "126:1"]
        pub fn errorframe_discard(frame: *mut errorframe_t);
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::error_h::{errorframe_t, errors_t};
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
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
        pub fn ast_childidx(ast: *mut ast_t, idx: usize) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "156:1"]
        pub fn ast_print_type(type_0: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
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
        #[c2rust::src_loc = "412:1"]
        pub fn ast_visit_scope(
            ast: *mut *mut ast_t,
            pre: ast_visit_t,
            post: ast_visit_t,
            options: *mut pass_opt_t,
            pass: pass_id,
        ) -> ast_result_t;
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn deferred_reify_new(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            thistype: *mut ast_t,
        ) -> *mut deferred_reification_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:2"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:3"]
pub mod cap_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn cap_single(type_0: *mut ast_t) -> token_id;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.h:5"]
pub mod viewpoint_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn viewpoint_type(l_type: *mut ast_t, r_type: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:6"]
pub mod subtype_h {
    use super::error_h::errorframe_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn is_subtype_fun(
            sub: *mut ast_t,
            super_0: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:8"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn is_name_private(name: *const libc::c_char) -> bool;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/expr/literal.h:11"]
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
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_child, ast_childidx, ast_data, ast_error, ast_error_frame, ast_free, ast_free_unattached,
    ast_from, ast_get, ast_get_children, ast_id, ast_name, ast_nearest, ast_parent, ast_print_type,
    ast_ptr_t, ast_result_t, ast_settype, ast_sibling, ast_type, AST_ERROR, AST_FATAL, AST_IGNORE,
    AST_OK,
};
use self::cap_h::cap_single;
pub use self::error_h::{
    errorframe_append, errorframe_discard, errorframe_report, errorframe_t, errormsg_t, errors_t,
};
use self::expr_h::{pass_expr, pass_pre_expr};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::is_name_private;
use self::literal_h::coerce_literals;
pub use self::pass_h::{
    ast_visit_scope, ast_visit_t, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level,
    PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER,
    PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT,
    PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX,
    PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET,
    VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::reify_h::{
    deferred_reification_t, deferred_reify_free, deferred_reify_method_def, deferred_reify_new,
};
use self::stringtab_h::stringtab;
use self::subtype_h::is_subtype_fun;
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
use self::viewpoint_h::viewpoint_type;
#[c2rust::src_loc = "24:1"]
unsafe extern "C" fn downcast_iso_trn_receiver_to_ref(mut receiver: *mut ast_t) -> *mut ast_t {
    match ast_id(receiver) as libc::c_uint {
        151 | 187 => match cap_single(receiver) as libc::c_uint {
            92 | 91 => return set_cap_and_ephemeral(receiver, TK_REF, TK_NONE),
            _ => return receiver,
        },
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                receiver,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children.as_mut_ptr(),
            );
            let mut downcasted_right: *mut ast_t = downcast_iso_trn_receiver_to_ref(right);
            if right != downcasted_right {
                return viewpoint_type(left, downcasted_right);
            } else {
                return receiver;
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.c\0"
                        as *const u8 as *const libc::c_char,
                    51 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"downcast_iso_trn_receiver_to_ref\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn lookup_nominal(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut orig: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut errors: bool,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    if ast_id(type_0) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.c\0" as *const u8
                as *const libc::c_char,
            59 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"lookup_nominal\0"))
                .as_ptr(),
        );
    };
    let mut t: *mut typecheck_t = &mut (*opt).check;
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    let mut type_id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut type_id, &mut typeparams, 0 as *mut *mut ast_t];
    ast_get_children(
        def,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut type_name: *const libc::c_char = ast_name(type_id);
    if is_name_private(type_name) as libc::c_int != 0
        && !from.is_null()
        && !opt.is_null()
        && !allow_private
    {
        if ast_nearest(def, TK_PACKAGE) != (*(*t).frame).package {
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"can't lookup fields or methods on private types from other packages\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return 0 as *mut deferred_reification_t;
        }
    }
    let mut find: *mut ast_t = ast_get(def, name, 0 as *mut sym_status_t);
    if !find.is_null() {
        match ast_id(find) as libc::c_uint {
            140 | 141 | 86 => {}
            88 | 90 | 89 => {
                if !opt.is_null() {
                    let mut cap: ast_ptr_t = 0 as *mut ast_t;
                    let mut id: ast_ptr_t = 0 as *mut ast_t;
                    let mut typeparams_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut params: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_0: [*mut *mut ast_t; 5] = [
                        &mut cap,
                        &mut id,
                        &mut typeparams_0,
                        &mut params,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        find,
                        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1),
                        children_0.as_mut_ptr(),
                    );
                    let mut param: *mut ast_t = ast_child(params);
                    while !param.is_null() {
                        let mut name_0: ast_ptr_t = 0 as *mut ast_t;
                        let mut type_1: ast_ptr_t = 0 as *mut ast_t;
                        let mut def_arg: ast_ptr_t = 0 as *mut ast_t;
                        let mut children_1: [*mut *mut ast_t; 4] =
                            [&mut name_0, &mut type_1, &mut def_arg, 0 as *mut *mut ast_t];
                        ast_get_children(
                            param,
                            (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong
                                )
                                .wrapping_sub(1),
                            children_1.as_mut_ptr(),
                        );
                        if ast_id(def_arg) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
                            && (ast_type(def_arg)).is_null()
                        {
                            let mut child: *mut ast_t = ast_child(def_arg);
                            if ast_id(child) as libc::c_uint
                                == TK_CALL as libc::c_int as libc::c_uint
                            {
                                ast_settype(child, ast_from(child, TK_INFERTYPE));
                            }
                            if ast_visit_scope(
                                &mut param,
                                Some(
                                    pass_pre_expr
                                        as unsafe extern "C" fn(
                                            *mut *mut ast_t,
                                            *mut pass_opt_t,
                                        )
                                            -> ast_result_t,
                                ),
                                Some(
                                    pass_expr
                                        as unsafe extern "C" fn(
                                            *mut *mut ast_t,
                                            *mut pass_opt_t,
                                        )
                                            -> ast_result_t,
                                ),
                                opt,
                                PASS_EXPR,
                            ) as libc::c_uint
                                != AST_OK as libc::c_int as libc::c_uint
                            {
                                return 0 as *mut deferred_reification_t;
                            }
                            def_arg = ast_childidx(param, 2 as libc::c_int as usize);
                            if !coerce_literals(&mut def_arg, type_1, opt) {
                                return 0 as *mut deferred_reification_t;
                            }
                        }
                        param = ast_sibling(param);
                    }
                }
            }
            _ => {
                find = 0 as *mut ast_t;
            }
        }
    }
    if find.is_null() {
        if errors {
            ast_error(
                (*opt).check.errors,
                from,
                b"couldn't find '%s' in '%s'\0" as *const u8 as *const libc::c_char,
                name,
                type_name,
            );
        }
        return 0 as *mut deferred_reification_t;
    }
    if is_name_private(name) as libc::c_int != 0
        && !from.is_null()
        && !opt.is_null()
        && !allow_private
    {
        match ast_id(find) as libc::c_uint {
            140 | 141 | 86 => {
                if (*(*t).frame).type_0 != def {
                    if errors {
                        ast_error(
                            (*opt).check.errors,
                            from,
                            b"can't lookup private fields from outside the type\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    return 0 as *mut deferred_reification_t;
                }
            }
            88 | 90 | 89 => {
                if ast_nearest(def, TK_PACKAGE) != (*(*t).frame).package {
                    if errors {
                        ast_error(
                            (*opt).check.errors,
                            from,
                            b"can't lookup private methods from outside the package\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    return 0 as *mut deferred_reification_t;
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.c\0"
                            as *const u8 as *const libc::c_char,
                        180 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"lookup_nominal\0",
                        ))
                        .as_ptr(),
                    );
                };
                return 0 as *mut deferred_reification_t;
            }
        }
        if name == stringtab(b"_final\0" as *const u8 as *const libc::c_char) {
            match ast_id(find) as libc::c_uint {
                88 | 90 | 89 => {
                    if errors {
                        ast_error(
                            (*opt).check.errors,
                            from,
                            b"can't lookup a _final function\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    return 0 as *mut deferred_reification_t;
                }
                _ => {}
            }
        } else if name == stringtab(b"_init\0" as *const u8 as *const libc::c_char)
            && ast_id(def) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint
        {
            match ast_id(find) as libc::c_uint {
                88 | 90 | 89 => {}
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.c\0"
                                as *const u8 as *const libc::c_char,
                            208 as libc::c_int as usize,
                            (*::core::mem::transmute::<
                                &[u8; 15],
                                &[libc::c_char; 15],
                            >(b"lookup_nominal\0"))
                                .as_ptr(),
                        );
                    };
                }
            }
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"can't lookup an _init function on a primitive\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as *mut deferred_reification_t;
        }
    }
    let mut typeargs: *mut ast_t = ast_childidx(type_0, 2 as libc::c_int as usize);
    let mut orig_initial: *mut ast_t = orig;
    if ast_id(find) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
        && ast_id(ast_child(find)) as libc::c_uint == TK_BOX as libc::c_int as libc::c_uint
    {
        orig = downcast_iso_trn_receiver_to_ref(orig);
    }
    let mut reified: *mut deferred_reification_t =
        deferred_reify_new(find, typeparams, typeargs, orig);
    if orig != orig_initial {
        ast_free(orig);
    }
    reified
}
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn lookup_typeparam(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut orig: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut errors: bool,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    let mut constraint: *mut ast_t = ast_childidx(def, 1 as libc::c_int as usize);
    let mut constraint_def: *mut ast_t = ast_data(constraint) as *mut ast_t;
    if def == constraint_def {
        if errors {
            let mut type_id: *mut ast_t = ast_child(def);
            let mut type_name: *const libc::c_char = ast_name(type_id);
            ast_error(
                (*opt).check.errors,
                from,
                b"couldn't find '%s' in '%s'\0" as *const u8 as *const libc::c_char,
                name,
                type_name,
            );
        }
        return 0 as *mut deferred_reification_t;
    }
    return lookup_base(opt, from, orig, constraint, name, errors, allow_private);
}
#[c2rust::src_loc = "259:1"]
unsafe extern "C" fn param_names_match(
    mut from: *mut ast_t,
    mut prev_fun: *mut ast_t,
    mut cur_fun: *mut ast_t,
    mut name: *const libc::c_char,
    mut pframe: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut parent: *mut ast_t = if !from.is_null() {
        ast_parent(from)
    } else {
        0 as *mut ast_t
    };
    if !parent.is_null() && ast_id(parent) as libc::c_uint == TK_CALL as libc::c_int as libc::c_uint
    {
        let mut receiver: ast_ptr_t = 0 as *mut ast_t;
        let mut positional: ast_ptr_t = 0 as *mut ast_t;
        let mut namedargs: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 4] = [
            &mut receiver,
            &mut positional,
            &mut namedargs,
            0 as *mut *mut ast_t,
        ];
        ast_get_children(
            parent,
            (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1),
            children.as_mut_ptr(),
        );
        if !namedargs.is_null()
            && ast_id(namedargs) as libc::c_uint == TK_NAMEDARGS as libc::c_int as libc::c_uint
        {
            let mut prev_cap: ast_ptr_t = 0 as *mut ast_t;
            let mut prev_name: ast_ptr_t = 0 as *mut ast_t;
            let mut prev_tparams: ast_ptr_t = 0 as *mut ast_t;
            let mut prev_params: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 5] = [
                &mut prev_cap,
                &mut prev_name,
                &mut prev_tparams,
                &mut prev_params,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                prev_fun,
                (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children_0.as_mut_ptr(),
            );
            let mut cur_cap: ast_ptr_t = 0 as *mut ast_t;
            let mut cur_name: ast_ptr_t = 0 as *mut ast_t;
            let mut cur_tparams: ast_ptr_t = 0 as *mut ast_t;
            let mut cur_params: ast_ptr_t = 0 as *mut ast_t;
            let mut children_1: [*mut *mut ast_t; 5] = [
                &mut cur_cap,
                &mut cur_name,
                &mut cur_tparams,
                &mut cur_params,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                cur_fun,
                (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children_1.as_mut_ptr(),
            );
            let mut prev_param: *mut ast_t = ast_child(prev_params);
            let mut cur_param: *mut ast_t = ast_child(cur_params);
            while !prev_param.is_null() && !cur_param.is_null() {
                let mut prev_id: ast_ptr_t = 0 as *mut ast_t;
                let mut children_2: [*mut *mut ast_t; 2] = [&mut prev_id, 0 as *mut *mut ast_t];
                ast_get_children(
                    prev_param,
                    (::core::mem::size_of::<[*mut *mut ast_t; 2]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                        .wrapping_sub(1),
                    children_2.as_mut_ptr(),
                );
                let mut cur_id: ast_ptr_t = 0 as *mut ast_t;
                let mut children_3: [*mut *mut ast_t; 2] = [&mut cur_id, 0 as *mut *mut ast_t];
                ast_get_children(
                    cur_param,
                    (::core::mem::size_of::<[*mut *mut ast_t; 2]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                        .wrapping_sub(1),
                    children_3.as_mut_ptr(),
                );
                if ast_name(prev_id) != ast_name(cur_id) {
                    if !pframe.is_null() {
                        let mut frame: errorframe_t = 0 as errorframe_t;
                        ast_error_frame(
                            &mut frame as *mut errorframe_t,
                            from,
                            b"the '%s' methods of this union type have different parameter names; this prevents their use as named arguments\0"
                                as *const u8 as *const libc::c_char,
                            name,
                        );
                        errorframe_append(&mut frame, pframe);
                        errorframe_report(&mut frame, (*opt).check.errors);
                    }
                    return 0 as libc::c_int != 0;
                }
                prev_param = ast_sibling(prev_param);
                cur_param = ast_sibling(cur_param);
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "303:1"]
unsafe extern "C" fn lookup_union(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut errors: bool,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    let mut child: *mut ast_t = ast_child(type_0);
    let mut result: *mut deferred_reification_t = 0 as *mut deferred_reification_t;
    let mut reified_result: *mut ast_t = 0 as *mut ast_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    while !child.is_null() {
        let mut r: *mut deferred_reification_t =
            lookup_base(opt, from, child, child, name, errors, allow_private);
        if r.is_null() {
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"couldn't find %s in %s\0" as *const u8 as *const libc::c_char,
                    name,
                    ast_print_type(child),
                );
            }
            ok = 0 as libc::c_int != 0;
        } else {
            match ast_id((*r).ast) as libc::c_uint {
                140 | 141 | 86 => {
                    if errors {
                        ast_error(
                            (*opt).check.errors,
                            from,
                            b"can't lookup field %s in %s in a union type\0" as *const u8
                                as *const libc::c_char,
                            name,
                            ast_print_type(child),
                        );
                    }
                    deferred_reify_free(r);
                    ok = 0 as libc::c_int != 0;
                }
                _ => {
                    let mut frame: errorframe_t = 0 as errorframe_t;
                    let mut pframe: *mut errorframe_t = if errors as libc::c_int != 0 {
                        &mut frame
                    } else {
                        0 as *mut errorframe_t
                    };
                    if result.is_null() {
                        result = r;
                    } else {
                        if reified_result.is_null() {
                            reified_result = deferred_reify_method_def(result, (*result).ast, opt);
                        }
                        let mut reified_r: *mut ast_t = deferred_reify_method_def(r, (*r).ast, opt);
                        if !is_subtype_fun(reified_r, reified_result, pframe, opt) {
                            if is_subtype_fun(reified_result, reified_r, pframe, opt) {
                                if !param_names_match(
                                    from,
                                    reified_result,
                                    reified_r,
                                    name,
                                    pframe,
                                    opt,
                                ) {
                                    ast_free_unattached(reified_r);
                                    deferred_reify_free(r);
                                    ok = 0 as libc::c_int != 0;
                                } else {
                                    if errors {
                                        errorframe_discard(pframe);
                                    }
                                    ast_free_unattached(reified_result);
                                    deferred_reify_free(result);
                                    reified_result = reified_r;
                                    result = r;
                                }
                            } else {
                                if errors {
                                    let mut frame_0: errorframe_t = 0 as errorframe_t;
                                    ast_error_frame(
                                        &mut frame_0 as *mut errorframe_t,
                                        from,
                                        b"a member of the union type has an incompatible method signature\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    errorframe_append(&mut frame_0, pframe);
                                    errorframe_report(&mut frame_0, (*opt).check.errors);
                                }
                                ast_free_unattached(reified_r);
                                deferred_reify_free(r);
                                ok = 0 as libc::c_int != 0;
                            }
                        } else {
                            if !param_names_match(
                                from,
                                reified_result,
                                reified_r,
                                name,
                                pframe,
                                opt,
                            ) {
                                ok = 0 as libc::c_int != 0;
                            }
                            ast_free_unattached(reified_r);
                            deferred_reify_free(r);
                        }
                    }
                }
            }
        }
        child = ast_sibling(child);
    }
    if !reified_result.is_null() {
        ast_free_unattached(reified_result);
    }
    if !ok {
        deferred_reify_free(result);
        result = 0 as *mut deferred_reification_t;
    }
    result
}
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn lookup_isect(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut errors: bool,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    let mut child: *mut ast_t = ast_child(type_0);
    let mut result: *mut deferred_reification_t = 0 as *mut deferred_reification_t;
    let mut reified_result: *mut ast_t = 0 as *mut ast_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    while !child.is_null() {
        let mut r: *mut deferred_reification_t = lookup_base(
            opt,
            from,
            child,
            child,
            name,
            0 as libc::c_int != 0,
            allow_private,
        );
        if !r.is_null() {
            match ast_id((*r).ast) as libc::c_uint {
                140 | 141 | 86 => {
                    deferred_reify_free(r);
                }
                _ => {
                    if result.is_null() {
                        result = r;
                    } else {
                        if reified_result.is_null() {
                            reified_result = deferred_reify_method_def(result, (*result).ast, opt);
                        }
                        let mut reified_r: *mut ast_t = deferred_reify_method_def(r, (*r).ast, opt);
                        let mut changed: bool = 0 as libc::c_int != 0;
                        if !is_subtype_fun(reified_result, reified_r, 0 as *mut errorframe_t, opt) {
                            if is_subtype_fun(
                                reified_r,
                                reified_result,
                                0 as *mut errorframe_t,
                                opt,
                            ) {
                                ast_free_unattached(reified_result);
                                deferred_reify_free(result);
                                reified_result = reified_r;
                                result = r;
                                changed = 1 as libc::c_int != 0;
                            }
                        }
                        if !changed {
                            ast_free_unattached(reified_r);
                            deferred_reify_free(r);
                        }
                    }
                }
            }
        }
        child = ast_sibling(child);
    }
    if errors as libc::c_int != 0 && result.is_null() {
        ast_error(
            (*opt).check.errors,
            from,
            b"couldn't find '%s'\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    if !reified_result.is_null() {
        ast_free_unattached(reified_result);
    }
    if !ok {
        deferred_reify_free(result);
        result = 0 as *mut deferred_reification_t;
    }
    result
}
#[c2rust::src_loc = "507:1"]
unsafe extern "C" fn lookup_base(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut orig: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut errors: bool,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    match ast_id(type_0) as libc::c_uint {
        149 => return lookup_union(opt, from, type_0, name, errors, allow_private),
        56 => return lookup_isect(opt, from, type_0, name, errors, allow_private),
        150 => {
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"can't lookup by name on a tuple\0" as *const u8 as *const libc::c_char,
                );
            }
            return 0 as *mut deferred_reification_t;
        }
        156 => {
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"can't lookup by name on '_'\0" as *const u8 as *const libc::c_char,
                );
            }
            return 0 as *mut deferred_reification_t;
        }
        151 => {
            return lookup_nominal(opt, from, orig, type_0, name, errors, allow_private);
        }
        17 => {
            return lookup_base(
                opt,
                from,
                orig,
                ast_childidx(type_0, 1 as libc::c_int as usize),
                name,
                errors,
                allow_private,
            );
        }
        187 => {
            return lookup_typeparam(opt, from, orig, type_0, name, errors, allow_private);
        }
        153 => {
            if errors {
                ast_error(
                    (*opt).check.errors,
                    from,
                    b"can't lookup by name on a function type\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as *mut deferred_reification_t;
        }
        157 | 158 => return 0 as *mut deferred_reification_t,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.c\0" as *const u8
                as *const libc::c_char,
            556 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"lookup_base\0")).as_ptr(),
        );
    };
    return 0 as *mut deferred_reification_t;
}
#[no_mangle]
#[c2rust::src_loc = "560:1"]
pub unsafe extern "C" fn lookup(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
) -> *mut deferred_reification_t {
    return lookup_base(
        opt,
        from,
        type_0,
        type_0,
        name,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
}
#[no_mangle]
#[c2rust::src_loc = "566:1"]
pub unsafe extern "C" fn lookup_try(
    mut opt: *mut pass_opt_t,
    mut from: *mut ast_t,
    mut type_0: *mut ast_t,
    mut name: *const libc::c_char,
    mut allow_private: bool,
) -> *mut deferred_reification_t {
    return lookup_base(
        opt,
        from,
        type_0,
        type_0,
        name,
        0 as libc::c_int != 0,
        allow_private,
    );
}
