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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:3"]
pub mod lexer_h {
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn lexer_print(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:4"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "177:1"]
        pub fn putchar(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:6"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:7"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:9"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::lexer_h::lexer_print;
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::stdio_h::{printf, putchar};
use self::string_h::{memset, strcmp};
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
#[c2rust::src_loc = "175:16"]
pub struct bnf_t {
    pub id: bnf_id,
    pub name: *const libc::c_char,
    pub hack_count: libc::c_int,
    pub optional: bool,
    pub used: bool,
    pub inline_rule: bool,
    pub child: *mut bnf_t,
    pub last_child: *mut bnf_t,
    pub sibling: *mut bnf_t,
}
#[c2rust::src_loc = "160:9"]
pub type bnf_id = libc::c_uint;
#[c2rust::src_loc = "171:3"]
pub const BNF_NOP: bnf_id = 9;
#[c2rust::src_loc = "170:3"]
pub const BNF_NEVER: bnf_id = 8;
#[c2rust::src_loc = "169:3"]
pub const BNF_RULE: bnf_id = 7;
#[c2rust::src_loc = "168:3"]
pub const BNF_QUOTED_TOKEN: bnf_id = 6;
#[c2rust::src_loc = "167:3"]
pub const BNF_TOKEN: bnf_id = 5;
#[c2rust::src_loc = "166:3"]
pub const BNF_REPEAT: bnf_id = 4;
#[c2rust::src_loc = "165:3"]
pub const BNF_OR: bnf_id = 3;
#[c2rust::src_loc = "164:3"]
pub const BNF_SEQ: bnf_id = 2;
#[c2rust::src_loc = "163:3"]
pub const BNF_DEF: bnf_id = 1;
#[c2rust::src_loc = "162:3"]
pub const BNF_TREE: bnf_id = 0;
#[c2rust::src_loc = "35:26"]
static mut antlr_pre: *const libc::c_char = b"// ANTLR v3 grammar\ngrammar pony;\n\noptions\n{\n  output = AST;\n  k = 1;\n}\n\n// Parser\n\0"
    as *const u8 as *const libc::c_char;
#[c2rust::src_loc = "45:26"]
static mut antlr_post: *const libc::c_char = b"// Rules of the form antlr_* are only present to avoid a bug in the\n// interpreter\n\n/* Precedence\n\nValue:\n1. postfix\n2. unop\n3. binop\n4. =\n5. seq\n6. ,\n\nType:\n1. ->\n2. & |\n3. ,\n*/\n\n// Lexer\n\nID\n  : LETTER (LETTER | DIGIT | '_' | '\\'')*\n  | '_' (LETTER | DIGIT | '_' | '\\'')*\n  ;\n\nINT\n  : DIGIT (DIGIT | '_')*\n  | '0' 'x' (HEX | '_')+\n  | '0' 'b' (BINARY | '_')+\n  | '\\'' CHAR_CHAR* '\\''\n  ;\n\nFLOAT\n  : DIGIT (DIGIT | '_')* ('.' DIGIT (DIGIT | '_')*)? EXP?\n  ;\n\nSTRING\n  : '\"' STRING_CHAR* '\"'\n  | '\"\"\"' (('\"' | '\"\"') ? ~'\"')* '\"\"\"' '\"'*\n  ;\n\nLPAREN_NEW\n  : NEWLINE '('\n  ;\n\nLSQUARE_NEW\n  : NEWLINE '['\n  ;\n\nMINUS_NEW\n  : NEWLINE '-'\n  ;\n\nMINUS_TILDE_NEW\n  : NEWLINE '-~'\n  ;\n\nLINECOMMENT\n  : '//' ~('\\n')* {$channel = HIDDEN;}\n  ;\n\nNESTEDCOMMENT\n  : '/*' (NESTEDCOMMENT | '/' ~'*' | ~('*' | '/') | ('*'+ ~('*' | '/')))* '*'+ '/' {$channel = HIDDEN;}\n  ;\n\nWS\n  : (' ' | '\\t' | '\\r')+ {$channel = HIDDEN;}\n  ;\n\nNEWLINE\n  : '\\n' (' ' | '\\t' | '\\r')* {$channel = HIDDEN;}\n  ;\n\nfragment\nCHAR_CHAR\n  : '\\\\' '\\'' | CHAR_ESC\n  | ~('\\'' | '\\\\')\n  ;\n\nfragment\nSTRING_CHAR\n  : '\\\\' '\"' | ESC\n  | ~('\"' | '\\\\')\n  ;\n\nfragment\nEXP\n  : ('e' | 'E') ('+' | '-')? (DIGIT | '_')+\n  ;\n\nfragment\nLETTER\n  : 'a'..'z' | 'A'..'Z'\n  ;\n\nfragment\nBINARY\n  : '0'..'1'\n  ;\n\nfragment\nDIGIT\n  : '0'..'9'\n  ;\n\nfragment\nHEX\n  : DIGIT | 'a'..'f' | 'A'..'F'\n  ;\n\nfragment\nESC\n  : CHAR_ESC\n  | UNICODE_ESC\n  | UNICODE2_ESC\n  ;\n\nfragment\nCHAR_ESC\n  : '\\\\' ('a' | 'b' | 'e' | 'f' | 'n' | 'r' | 't' | 'v' | '\\\\' | '0')\n  | HEX_ESC\n  ;\n\nfragment\nHEX_ESC\n  : '\\\\' 'x' HEX HEX\n  ;\n\nfragment\nUNICODE_ESC\n  : '\\\\' 'u' HEX HEX HEX HEX\n  ;\n\nfragment\nUNICODE2_ESC\n  : '\\\\' 'U' HEX HEX HEX HEX HEX HEX\n  ;\n\0"
    as *const u8 as *const libc::c_char;
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn bnf_create(mut id: bnf_id) -> *mut bnf_t {
    let mut b: *mut bnf_t = ponyint_pool_alloc(1 as libc::c_int as usize) as *mut bnf_t;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<bnf_t>().try_into().unwrap(),
    );
    (*b).id = id;
    b
}
#[c2rust::src_loc = "210:1"]
unsafe extern "C" fn bnf_free(mut bnf: *mut bnf_t) {
    if bnf.is_null() {
        return;
    }
    bnf_free((*bnf).child);
    bnf_free((*bnf).sibling);
    ponyint_pool_free(1 as libc::c_int as usize, bnf as *mut libc::c_void);
}
#[c2rust::src_loc = "223:1"]
unsafe extern "C" fn bnf_copy(
    mut bnf: *mut bnf_t,
    mut out_last_sibling: *mut *mut bnf_t,
) -> *mut bnf_t {
    if bnf.is_null() {
        return 0 as *mut bnf_t;
    }
    let mut new_bnf: *mut bnf_t = bnf_create((*bnf).id);
    let ref mut fresh0 = (*new_bnf).name;
    *fresh0 = (*bnf).name;
    (*new_bnf).optional = (*bnf).optional;
    (*new_bnf).used = (*bnf).used;
    (*new_bnf).inline_rule = (*bnf).inline_rule;
    if !out_last_sibling.is_null() {
        *out_last_sibling = new_bnf;
    }
    let ref mut fresh1 = (*new_bnf).sibling;
    *fresh1 = bnf_copy((*bnf).sibling, out_last_sibling);
    let ref mut fresh2 = (*new_bnf).child;
    *fresh2 = bnf_copy((*bnf).child, &mut (*new_bnf).last_child);
    return new_bnf;
}
#[c2rust::src_loc = "246:1"]
unsafe extern "C" fn bnf_add(mut bnf: *mut bnf_t, mut parent: *mut bnf_t) -> *mut bnf_t {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            248 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"bnf_add\0")).as_ptr(),
        );
    };
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            249 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"bnf_add\0")).as_ptr(),
        );
    };
    if ((*parent).last_child).is_null() {
        let ref mut fresh3 = (*parent).child;
        *fresh3 = bnf;
    } else {
        let ref mut fresh4 = (*(*parent).last_child).sibling;
        *fresh4 = bnf;
    }
    let ref mut fresh5 = (*parent).last_child;
    *fresh5 = bnf;
    return bnf;
}
#[c2rust::src_loc = "262:1"]
unsafe extern "C" fn bnf_print_quoted_token(mut token: *const libc::c_char) {
    printf(b"'\0" as *const u8 as *const libc::c_char);
    let mut c: *const libc::c_char = token;
    while *c as libc::c_int != '\0' as i32 {
        match *c as libc::c_int {
            39 => {
                printf(b"\\'\0" as *const u8 as *const libc::c_char);
            }
            92 => {
                printf(b"\\\\\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                printf(b"\\n\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                printf(b"\\r\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                printf(b"\\t\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                printf(b"\\b\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                printf(b"\\f\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                putchar(*c as libc::c_int);
            }
        }
        c = c.offset(1);
    }
    printf(b"'\0" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn bnf_print(mut bnf: *mut bnf_t, mut top_format: bool) {
    if bnf.is_null() {
        return;
    }
    match (*bnf).id as libc::c_uint {
        0 => {
            bnf_print_children(
                bnf,
                b"\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
        }
        1 => {
            if (*bnf).used as libc::c_int != 0 || ((*bnf).name).is_null() {
                if ((*bnf).name).is_null() {
                    printf(
                        b"antlr_%d\n  : \0" as *const u8 as *const libc::c_char,
                        (*bnf).hack_count,
                    );
                } else {
                    printf(
                        b"%s\n  : \0" as *const u8 as *const libc::c_char,
                        (*bnf).name,
                    );
                }
                bnf_print((*bnf).child, 1 as libc::c_int != 0);
                printf(b"\n  ;\n\n\0" as *const u8 as *const libc::c_char);
            }
        }
        2 => {
            bnf_print_children(
                bnf,
                b" \0" as *const u8 as *const libc::c_char,
                top_format,
                0 as libc::c_int != 0,
            );
        }
        3 => {
            if top_format as libc::c_int != 0 && !(*bnf).optional {
                bnf_print_children(
                    bnf,
                    b"\n  | \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int != 0,
                    1 as libc::c_int != 0,
                );
            } else {
                bnf_print_children(
                    bnf,
                    b" | \0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
            }
            if (*bnf).optional {
                printf(b"?\0" as *const u8 as *const libc::c_char);
            }
        }
        4 => {
            bnf_print((*bnf).child, 0 as libc::c_int != 0);
            printf(b"*\0" as *const u8 as *const libc::c_char);
        }
        5 | 7 => {
            if ((*bnf).name).is_null() {
                printf(
                    b"antlr_%d\0" as *const u8 as *const libc::c_char,
                    (*bnf).hack_count,
                );
            } else {
                printf(b"%s\0" as *const u8 as *const libc::c_char, (*bnf).name);
            }
        }
        6 => {
            bnf_print_quoted_token((*bnf).name);
        }
        8 => {
            printf(b"NEVER\0" as *const u8 as *const libc::c_char);
        }
        9 => {
            printf(b"nop\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"false\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                        as *const u8 as *const libc::c_char,
                    351 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"bnf_print\0"))
                        .as_ptr(),
                );
            };
        }
    };
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn bnf_print_children(
    mut bnf: *mut bnf_t,
    mut separator: *const libc::c_char,
    mut top_format: bool,
    mut children_top_format: bool,
) {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            363 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"bnf_print_children\0"))
                .as_ptr(),
        );
    };
    if !separator.is_null() {
    } else {
        ponyint_assert_fail(
            b"separator != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            364 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"bnf_print_children\0"))
                .as_ptr(),
        );
    };
    let mut child: *mut bnf_t = (*bnf).child;
    if !child.is_null() {
    } else {
        ponyint_assert_fail(
            b"child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            367 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"bnf_print_children\0"))
                .as_ptr(),
        );
    };
    let mut parens: bool = !top_format && !((*child).sibling).is_null();
    if parens {
        printf(b"(\0" as *const u8 as *const libc::c_char);
    }
    bnf_print(child, children_top_format);
    let mut p: *mut bnf_t = (*child).sibling;
    while !p.is_null() {
        printf(b"%s\0" as *const u8 as *const libc::c_char, separator);
        bnf_print(p, children_top_format);
        p = (*p).sibling;
    }
    if parens {
        printf(b")\0" as *const u8 as *const libc::c_char);
    }
}
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn bnf_token_set(
    mut bnf: *mut bnf_t,
    mut tokens: *mut token_id,
    mut clean: bool,
) {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            390 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"bnf_token_set\0"))
                .as_ptr(),
        );
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while *tokens.offset(i as isize) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        let mut p: *mut bnf_t = bnf_add(bnf_create(BNF_TOKEN), bnf);
        if !p.is_null() {
        } else {
            ponyint_assert_fail(
                b"p != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                    as *const u8 as *const libc::c_char,
                395 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"bnf_token_set\0"))
                    .as_ptr(),
            );
        };
        match *tokens.offset(i as isize) as libc::c_uint {
            0 => {
                let ref mut fresh6 = (*p).name;
                *fresh6 = b"\0" as *const u8 as *const libc::c_char;
            }
            5 => {
                let ref mut fresh7 = (*p).name;
                *fresh7 = b"STRING\0" as *const u8 as *const libc::c_char;
            }
            6 => {
                let ref mut fresh8 = (*p).name;
                *fresh8 = b"INT\0" as *const u8 as *const libc::c_char;
            }
            7 => {
                let ref mut fresh9 = (*p).name;
                *fresh9 = b"FLOAT\0" as *const u8 as *const libc::c_char;
            }
            8 => {
                let ref mut fresh10 = (*p).name;
                *fresh10 = b"ID\0" as *const u8 as *const libc::c_char;
            }
            65 => {
                let ref mut fresh11 = (*p).name;
                *fresh11 = b"LPAREN_NEW\0" as *const u8 as *const libc::c_char;
            }
            66 => {
                let ref mut fresh12 = (*p).name;
                *fresh12 = b"LSQUARE_NEW\0" as *const u8 as *const libc::c_char;
            }
            67 => {
                let ref mut fresh13 = (*p).name;
                *fresh13 = b"MINUS_NEW\0" as *const u8 as *const libc::c_char;
            }
            68 => {
                let ref mut fresh14 = (*p).name;
                *fresh14 = b"MINUS_TILDE_NEW\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                let ref mut fresh15 = (*p).name;
                *fresh15 = lexer_print(*tokens.offset(i as isize));
                (*p).id = BNF_QUOTED_TOKEN;
                if !((*p).name).is_null() {
                } else {
                    ponyint_assert_fail(
                        b"p->name != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                            as *const u8 as *const libc::c_char,
                        417 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            b"bnf_token_set\0",
                        ))
                        .as_ptr(),
                    );
                };
                if clean as libc::c_int != 0
                    && *((*p).name).offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
                    || *tokens.offset(i as isize) as libc::c_uint
                        == TK_NEWLINE as libc::c_int as libc::c_uint
                {
                    (*p).id = BNF_NEVER;
                }
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "432:1"]
unsafe extern "C" fn bnf_rule_set(mut bnf: *mut bnf_t, mut rules: *mut *const libc::c_char) {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            434 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_rule_set\0")).as_ptr(),
        );
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*rules.offset(i as isize)).is_null() {
        let mut p: *mut bnf_t = bnf_add(bnf_create(BNF_RULE), bnf);
        if !p.is_null() {
        } else {
            ponyint_assert_fail(
                b"p != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                    as *const u8 as *const libc::c_char,
                439 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_rule_set\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh16 = (*p).name;
        *fresh16 = *rules.offset(i as isize);
        i += 1;
    }
}
#[c2rust::src_loc = "448:1"]
unsafe extern "C" fn bnf_use_child(mut bnf: *mut bnf_t) {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            450 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"bnf_use_child\0"))
                .as_ptr(),
        );
    };
    if !((*bnf).child).is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf->child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            451 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"bnf_use_child\0"))
                .as_ptr(),
        );
    };
    let mut child: *mut bnf_t = (*bnf).child;
    (*bnf).id = (*child).id;
    let ref mut fresh17 = (*bnf).name;
    *fresh17 = (*child).name;
    (*bnf).optional = (*child).optional;
    let ref mut fresh18 = (*bnf).child;
    *fresh18 = (*child).child;
    let ref mut fresh19 = (*bnf).last_child;
    *fresh19 = (*child).last_child;
    let ref mut fresh20 = (*child).child;
    *fresh20 = 0 as *mut bnf_t;
    bnf_free(child);
}
#[c2rust::src_loc = "467:1"]
unsafe extern "C" fn bnf_find_def(
    mut tree: *mut bnf_t,
    mut name: *const libc::c_char,
) -> *mut bnf_t {
    if !tree.is_null() {
    } else {
        ponyint_assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            469 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_find_def\0")).as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            470 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_find_def\0")).as_ptr(),
        );
    };
    let mut p: *mut bnf_t = (*tree).child;
    while !p.is_null() {
        if strcmp((*p).name, name) == 0 as libc::c_int {
            return p;
        }
        p = (*p).sibling;
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"false\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            479 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_find_def\0")).as_ptr(),
        );
    };
    return 0 as *mut bnf_t;
}
#[c2rust::src_loc = "488:1"]
unsafe extern "C" fn bnf_simplify_node(
    mut tree: *mut bnf_t,
    mut bnf: *mut bnf_t,
    mut out_changed: *mut bool,
) {
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            490 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"bnf_simplify_node\0"))
                .as_ptr(),
        );
    };
    if !out_changed.is_null() {
    } else {
        ponyint_assert_fail(
            b"out_changed != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            491 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"bnf_simplify_node\0"))
                .as_ptr(),
        );
    };
    match (*bnf).id as libc::c_uint {
        0 => {
            bnf_simplify_children(tree, bnf, 0 as *mut bool, 0 as *mut bool, out_changed);
        }
        1 => {
            bnf_simplify_node(tree, (*bnf).child, out_changed);
        }
        2 => {
            let mut any_never: bool = 0 as libc::c_int != 0;
            bnf_simplify_children(tree, bnf, &mut any_never, 0 as *mut bool, out_changed);
            if any_never {
                (*bnf).id = BNF_NEVER;
                *out_changed = 1 as libc::c_int != 0;
            } else if ((*bnf).child).is_null() {
                (*bnf).id = BNF_NOP;
                *out_changed = 1 as libc::c_int != 0;
            } else if ((*(*bnf).child).sibling).is_null() {
                bnf_use_child(bnf);
                *out_changed = 1 as libc::c_int != 0;
            }
        }
        3 => {
            let mut any_nop: bool = 0 as libc::c_int != 0;
            bnf_simplify_children(tree, bnf, 0 as *mut bool, &mut any_nop, out_changed);
            if any_nop {
                (*bnf).optional = 1 as libc::c_int != 0;
                *out_changed = 1 as libc::c_int != 0;
            }
            if ((*bnf).child).is_null() {
                (*bnf).id = (if (*bnf).optional as libc::c_int != 0 {
                    BNF_NOP as libc::c_int
                } else {
                    BNF_NEVER as libc::c_int
                }) as bnf_id;
                *out_changed = 1 as libc::c_int != 0;
            } else if ((*(*bnf).child).sibling).is_null() && !(*bnf).optional {
                bnf_use_child(bnf);
                *out_changed = 1 as libc::c_int != 0;
            }
        }
        4 => {
            bnf_simplify_children(tree, bnf, 0 as *mut bool, 0 as *mut bool, out_changed);
            if ((*bnf).child).is_null() {
                (*bnf).id = BNF_NOP;
                *out_changed = 1 as libc::c_int != 0;
            }
        }
        7 => {
            if !((*bnf).name).is_null() {
                let mut def: *mut bnf_t = bnf_find_def(tree, (*bnf).name);
                if !def.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"def != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                            as *const u8 as *const libc::c_char,
                        575 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                            b"bnf_simplify_node\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut rule: *mut bnf_t = (*def).child;
                if !rule.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"rule != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                            as *const u8 as *const libc::c_char,
                        578 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                            b"bnf_simplify_node\0",
                        ))
                        .as_ptr(),
                    );
                };
                if (*rule).id as libc::c_uint == BNF_NEVER as libc::c_int as libc::c_uint
                    || (*rule).id as libc::c_uint == BNF_NOP as libc::c_int as libc::c_uint
                    || (*rule).id as libc::c_uint == BNF_TOKEN as libc::c_int as libc::c_uint
                    || (*rule).id as libc::c_uint == BNF_QUOTED_TOKEN as libc::c_int as libc::c_uint
                    || (*rule).id as libc::c_uint == BNF_RULE as libc::c_int as libc::c_uint
                    || (*def).inline_rule as libc::c_int != 0
                {
                    (*bnf).id = (*rule).id;
                    let ref mut fresh21 = (*bnf).name;
                    *fresh21 = (*rule).name;
                    (*bnf).optional = (*rule).optional;
                    let ref mut fresh22 = (*bnf).last_child;
                    *fresh22 = 0 as *mut bnf_t;
                    let ref mut fresh23 = (*bnf).child;
                    *fresh23 = bnf_copy((*rule).child, &mut (*bnf).last_child);
                    if ((*rule).sibling).is_null() {
                    } else {
                        ponyint_assert_fail(
                            b"rule->sibling == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                                as *const u8 as *const libc::c_char,
                            595 as libc::c_int as usize,
                            (*::core::mem::transmute::<
                                &[u8; 18],
                                &[libc::c_char; 18],
                            >(b"bnf_simplify_node\0"))
                                .as_ptr(),
                        );
                    };
                    *out_changed = 1 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    };
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn bnf_simplify_children(
    mut tree: *mut bnf_t,
    mut parent: *mut bnf_t,
    mut out_never: *mut bool,
    mut out_nop: *mut bool,
    mut out_changed: *mut bool,
) {
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            617 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"bnf_simplify_children\0"))
                .as_ptr(),
        );
    };
    if !out_never.is_null() {
        *out_never = 0 as libc::c_int != 0;
    }
    if !out_nop.is_null() {
        *out_nop = 0 as libc::c_int != 0;
    }
    let mut prev: *mut bnf_t = 0 as *mut bnf_t;
    let mut p: *mut bnf_t = (*parent).child;
    while !p.is_null() {
        bnf_simplify_node(tree, p, out_changed);
        if (*p).id as libc::c_uint == BNF_NEVER as libc::c_int as libc::c_uint
            && !out_never.is_null()
        {
            *out_never = 1 as libc::c_int != 0;
        }
        if (*p).id as libc::c_uint == BNF_NOP as libc::c_int as libc::c_uint && !out_nop.is_null() {
            *out_nop = 1 as libc::c_int != 0;
        }
        if (*p).id as libc::c_uint == BNF_NEVER as libc::c_int as libc::c_uint
            || (*p).id as libc::c_uint == BNF_NOP as libc::c_int as libc::c_uint
        {
            if prev.is_null() {
                let ref mut fresh24 = (*parent).child;
                *fresh24 = (*p).sibling;
            } else {
                let ref mut fresh25 = (*prev).sibling;
                *fresh25 = (*p).sibling;
            }
            let mut next: *mut bnf_t = (*p).sibling;
            let ref mut fresh26 = (*p).sibling;
            *fresh26 = 0 as *mut bnf_t;
            bnf_free(p);
            p = next;
            *out_changed = 1 as libc::c_int != 0;
        } else {
            prev = p;
            p = (*p).sibling;
        }
    }
    let ref mut fresh27 = (*parent).last_child;
    *fresh27 = prev;
}
#[c2rust::src_loc = "663:1"]
unsafe extern "C" fn bnf_simplify(mut tree: *mut bnf_t) {
    if !tree.is_null() {
    } else {
        ponyint_assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            665 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"bnf_simplify\0")).as_ptr(),
        );
    };
    let mut changed: bool = 1 as libc::c_int != 0;
    while changed {
        changed = 0 as libc::c_int != 0;
        bnf_simplify_node(tree, tree, &mut changed);
    }
}
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn bnf_avoid_antlr_bug(mut tree: *mut bnf_t, mut bnf: *mut bnf_t) {
    if !tree.is_null() {
    } else {
        ponyint_assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            680 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"bnf_avoid_antlr_bug\0"))
                .as_ptr(),
        );
    };
    if bnf.is_null() {
        return;
    }
    let mut p: *mut bnf_t = (*bnf).child;
    while !p.is_null() {
        bnf_avoid_antlr_bug(tree, p);
        p = (*p).sibling;
    }
    if (*bnf).id as libc::c_uint != BNF_REPEAT as libc::c_int as libc::c_uint {
        return;
    }
    let mut or_node: *mut bnf_t = (*bnf).child;
    if !or_node.is_null() {
    } else {
        ponyint_assert_fail(
            b"or_node != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            695 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"bnf_avoid_antlr_bug\0"))
                .as_ptr(),
        );
    };
    if (*or_node).id as libc::c_uint != BNF_OR as libc::c_int as libc::c_uint {
        return;
    }
    if !((*or_node).child).is_null() {
    } else {
        ponyint_assert_fail(
            b"or_node->child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            700 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"bnf_avoid_antlr_bug\0"))
                .as_ptr(),
        );
    };
    let mut second_child: *mut bnf_t = (*(*or_node).child).sibling;
    if second_child.is_null()
        || (*second_child).id as libc::c_uint == BNF_TOKEN as libc::c_int as libc::c_uint
        || (*second_child).id as libc::c_uint == BNF_QUOTED_TOKEN as libc::c_int as libc::c_uint
    {
        return;
    }
    let ref mut fresh28 = (*tree).hack_count;
    let fresh29 = *fresh28;
    *fresh28 = *fresh28 + 1;
    let mut rule_no: libc::c_int = fresh29;
    if !((*tree).last_child).is_null() {
    } else {
        ponyint_assert_fail(
            b"tree->last_child != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            711 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"bnf_avoid_antlr_bug\0"))
                .as_ptr(),
        );
    };
    let mut new_rule: *mut bnf_t = bnf_create(BNF_DEF);
    (*new_rule).hack_count = rule_no;
    let ref mut fresh30 = (*new_rule).child;
    *fresh30 = or_node;
    let ref mut fresh31 = (*(*tree).last_child).sibling;
    *fresh31 = new_rule;
    let ref mut fresh32 = (*tree).last_child;
    *fresh32 = new_rule;
    let mut new_ref: *mut bnf_t = bnf_create(BNF_RULE);
    (*new_ref).hack_count = rule_no;
    let ref mut fresh33 = (*bnf).child;
    *fresh33 = new_ref;
    let ref mut fresh34 = (*bnf).last_child;
    *fresh34 = new_ref;
}
#[c2rust::src_loc = "727:1"]
unsafe extern "C" fn bnf_mark_refd_defs(mut tree: *mut bnf_t, mut bnf: *mut bnf_t) {
    if !tree.is_null() {
    } else {
        ponyint_assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            729 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"bnf_mark_refd_defs\0"))
                .as_ptr(),
        );
    };
    if !bnf.is_null() {
    } else {
        ponyint_assert_fail(
            b"bnf != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            730 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"bnf_mark_refd_defs\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut bnf_t = bnf;
    while !p.is_null() {
        if !((*p).child).is_null() {
            bnf_mark_refd_defs(tree, (*p).child);
        }
        if (*p).id as libc::c_uint == BNF_RULE as libc::c_int as libc::c_uint
            && !((*p).name).is_null()
        {
            let mut rule: *mut bnf_t = bnf_find_def(tree, (*p).name);
            if !rule.is_null() {
            } else {
                ponyint_assert_fail(
                    b"rule != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0"
                        as *const u8 as *const libc::c_char,
                    740 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"bnf_mark_refd_defs\0",
                    ))
                    .as_ptr(),
                );
            };
            (*rule).used = 1 as libc::c_int != 0;
        }
        p = (*p).sibling;
    }
}
#[c2rust::src_loc = "748:1"]
unsafe extern "C" fn bnf_mark_used_rules(mut tree: *mut bnf_t) {
    (*(*tree).child).used = 1 as libc::c_int != 0;
    bnf_mark_refd_defs(tree, (*tree).child);
}
#[c2rust::src_loc = "863:1"]
unsafe extern "C" fn bnf_def(mut clean: bool) -> *mut bnf_t {
    let mut parent: *mut bnf_t = bnf_create(BNF_TREE);
    let mut optional: bool = 0 as libc::c_int != 0;
    let mut rule: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh35 = (*rule).name;
    *fresh35 = b"provides\0" as *const u8 as *const libc::c_char;
    let ref mut fresh36 = (*rule).sibling;
    *fresh36 = (*parent).child;
    let ref mut fresh37 = (*parent).child;
    *fresh37 = rule;
    let mut parent_0: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule);
    (*rule).inline_rule = 1 as libc::c_int != 0;
    static mut rules: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_0);
    (*p).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p, rules.as_mut_ptr());
    let mut rule_0: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh38 = (*rule_0).name;
    *fresh38 = b"defaultarg\0" as *const u8 as *const libc::c_char;
    let ref mut fresh39 = (*rule_0).sibling;
    *fresh39 = (*parent).child;
    let ref mut fresh40 = (*parent).child;
    *fresh40 = rule_0;
    let mut parent_1: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_0);
    (*rule_0).inline_rule = 1 as libc::c_int != 0;
    static mut rules_0: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_0: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_1);
    (*p_0).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_0, rules_0.as_mut_ptr());
    let mut rule_1: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh41 = (*rule_1).name;
    *fresh41 = b"param\0" as *const u8 as *const libc::c_char;
    let ref mut fresh42 = (*rule_1).sibling;
    *fresh42 = (*parent).child;
    let ref mut fresh43 = (*parent).child;
    *fresh43 = rule_1;
    let mut parent_2: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_1);
    static mut tokens: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_1: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_2);
    (*p_1).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_1, tokens.as_mut_ptr(), clean);
    static mut tokens_0: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_2: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_2);
    (*p_2).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_2, tokens_0.as_mut_ptr(), clean);
    static mut rules_1: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_3: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_2);
    (*p_3).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_3, rules_1.as_mut_ptr());
    let mut p_4: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_2);
    (*p_4).optional = 1 as libc::c_int != 0;
    let mut parent_3: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_4);
    static mut tokens_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_5: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_3);
    (*p_5).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_5, tokens_1.as_mut_ptr(), clean);
    static mut rules_2: [*const libc::c_char; 2] = [
        b"defaultarg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_6: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_3);
    (*p_6).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_6, rules_2.as_mut_ptr());
    let mut rule_2: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh44 = (*rule_2).name;
    *fresh44 = b"ellipsis\0" as *const u8 as *const libc::c_char;
    let ref mut fresh45 = (*rule_2).sibling;
    *fresh45 = (*parent).child;
    let ref mut fresh46 = (*parent).child;
    *fresh46 = rule_2;
    let mut parent_4: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_2);
    static mut tokens_2: [token_id; 2] = [TK_ELLIPSIS, TK_NONE];
    let mut p_7: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_4);
    (*p_7).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_7, tokens_2.as_mut_ptr(), clean);
    let mut rule_3: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh47 = (*rule_3).name;
    *fresh47 = b"literal\0" as *const u8 as *const libc::c_char;
    let ref mut fresh48 = (*rule_3).sibling;
    *fresh48 = (*parent).child;
    let ref mut fresh49 = (*parent).child;
    *fresh49 = rule_3;
    let mut parent_5: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_3);
    static mut tokens_3: [token_id; 6] = [TK_TRUE, TK_FALSE, TK_INT, TK_FLOAT, TK_STRING, TK_NONE];
    let mut p_8: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_5);
    (*p_8).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_8, tokens_3.as_mut_ptr(), clean);
    let mut rule_4: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh50 = (*rule_4).name;
    *fresh50 = b"const_expr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh51 = (*rule_4).sibling;
    *fresh51 = (*parent).child;
    let ref mut fresh52 = (*parent).child;
    *fresh52 = rule_4;
    let mut parent_6: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_4);
    (*rule_4).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_4: [token_id; 2] = [TK_CONSTANT, TK_NONE];
    let mut p_9: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_6);
    (*p_9).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_9, tokens_4.as_mut_ptr(), clean);
    static mut rules_3: [*const libc::c_char; 2] = [
        b"postfix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_10: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_6);
    (*p_10).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_10, rules_3.as_mut_ptr());
    let mut rule_5: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh53 = (*rule_5).name;
    *fresh53 = b"typeargliteral\0" as *const u8 as *const libc::c_char;
    let ref mut fresh54 = (*rule_5).sibling;
    *fresh54 = (*parent).child;
    let ref mut fresh55 = (*parent).child;
    *fresh55 = rule_5;
    let mut parent_7: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_5);
    (*rule_5).inline_rule = 1 as libc::c_int != 0;
    static mut rules_4: [*const libc::c_char; 2] = [
        b"literal\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_11: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_7);
    (*p_11).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_11, rules_4.as_mut_ptr());
    let mut rule_6: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh56 = (*rule_6).name;
    *fresh56 = b"typeargconst\0" as *const u8 as *const libc::c_char;
    let ref mut fresh57 = (*rule_6).sibling;
    *fresh57 = (*parent).child;
    let ref mut fresh58 = (*parent).child;
    *fresh58 = rule_6;
    let mut parent_8: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_6);
    (*rule_6).inline_rule = 1 as libc::c_int != 0;
    static mut rules_5: [*const libc::c_char; 2] = [
        b"const_expr\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_12: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_8);
    (*p_12).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_12, rules_5.as_mut_ptr());
    let mut rule_7: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh59 = (*rule_7).name;
    *fresh59 = b"typearg\0" as *const u8 as *const libc::c_char;
    let ref mut fresh60 = (*rule_7).sibling;
    *fresh60 = (*parent).child;
    let ref mut fresh61 = (*parent).child;
    *fresh61 = rule_7;
    let mut parent_9: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_7);
    static mut rules_6: [*const libc::c_char; 4] = [
        b"type\0" as *const u8 as *const libc::c_char,
        b"typeargliteral\0" as *const u8 as *const libc::c_char,
        b"typeargconst\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_13: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_9);
    (*p_13).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_13, rules_6.as_mut_ptr());
    let mut rule_8: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh62 = (*rule_8).name;
    *fresh62 = b"typeparam\0" as *const u8 as *const libc::c_char;
    let ref mut fresh63 = (*rule_8).sibling;
    *fresh63 = (*parent).child;
    let ref mut fresh64 = (*parent).child;
    *fresh64 = rule_8;
    let mut parent_10: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_8);
    static mut tokens_5: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_14: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_10);
    (*p_14).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_14, tokens_5.as_mut_ptr(), clean);
    let mut p_15: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_10);
    (*p_15).optional = 1 as libc::c_int != 0;
    let mut parent_11: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_15);
    static mut tokens_6: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_16: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_11);
    (*p_16).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_16, tokens_6.as_mut_ptr(), clean);
    static mut rules_7: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_17: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_11);
    (*p_17).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_17, rules_7.as_mut_ptr());
    let mut p_18: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_10);
    (*p_18).optional = 1 as libc::c_int != 0;
    let mut parent_12: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_18);
    static mut tokens_7: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_19: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_12);
    (*p_19).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_19, tokens_7.as_mut_ptr(), clean);
    static mut rules_8: [*const libc::c_char; 2] = [
        b"typearg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_20: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_12);
    (*p_20).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_20, rules_8.as_mut_ptr());
    let mut rule_9: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh65 = (*rule_9).name;
    *fresh65 = b"params\0" as *const u8 as *const libc::c_char;
    let ref mut fresh66 = (*rule_9).sibling;
    *fresh66 = (*parent).child;
    let ref mut fresh67 = (*parent).child;
    *fresh67 = rule_9;
    let mut parent_13: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_9);
    static mut rules_9: [*const libc::c_char; 3] = [
        b"param\0" as *const u8 as *const libc::c_char,
        b"ellipsis\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_21: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_13);
    (*p_21).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_21, rules_9.as_mut_ptr());
    let mut p_22: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_13);
    let mut parent_14: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_22);
    static mut tokens_8: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_23: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_14);
    (*p_23).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_23, tokens_8.as_mut_ptr(), clean);
    static mut rules_10: [*const libc::c_char; 3] = [
        b"param\0" as *const u8 as *const libc::c_char,
        b"ellipsis\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_24: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_14);
    (*p_24).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_24, rules_10.as_mut_ptr());
    let mut rule_10: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh68 = (*rule_10).name;
    *fresh68 = b"typeparams\0" as *const u8 as *const libc::c_char;
    let ref mut fresh69 = (*rule_10).sibling;
    *fresh69 = (*parent).child;
    let ref mut fresh70 = (*parent).child;
    *fresh70 = rule_10;
    let mut parent_15: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_10);
    static mut tokens_9: [token_id; 3] = [TK_LSQUARE, TK_LSQUARE_NEW, TK_NONE];
    let mut p_25: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_15);
    (*p_25).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_25, tokens_9.as_mut_ptr(), clean);
    static mut rules_11: [*const libc::c_char; 2] = [
        b"typeparam\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_26: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_15);
    (*p_26).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_26, rules_11.as_mut_ptr());
    let mut p_27: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_15);
    let mut parent_16: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_27);
    static mut tokens_10: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_28: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_16);
    (*p_28).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_28, tokens_10.as_mut_ptr(), clean);
    static mut rules_12: [*const libc::c_char; 2] = [
        b"typeparam\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_29: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_16);
    (*p_29).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_29, rules_12.as_mut_ptr());
    static mut tokens_11: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut p_30: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_15);
    (*p_30).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_30, tokens_11.as_mut_ptr(), clean);
    let mut rule_11: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh71 = (*rule_11).name;
    *fresh71 = b"typeargs\0" as *const u8 as *const libc::c_char;
    let ref mut fresh72 = (*rule_11).sibling;
    *fresh72 = (*parent).child;
    let ref mut fresh73 = (*parent).child;
    *fresh73 = rule_11;
    let mut parent_17: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_11);
    static mut tokens_12: [token_id; 2] = [TK_LSQUARE, TK_NONE];
    let mut p_31: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_17);
    (*p_31).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_31, tokens_12.as_mut_ptr(), clean);
    static mut rules_13: [*const libc::c_char; 2] = [
        b"typearg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_32: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_17);
    (*p_32).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_32, rules_13.as_mut_ptr());
    let mut p_33: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_17);
    let mut parent_18: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_33);
    static mut tokens_13: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_34: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_18);
    (*p_34).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_34, tokens_13.as_mut_ptr(), clean);
    static mut rules_14: [*const libc::c_char; 2] = [
        b"typearg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_35: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_18);
    (*p_35).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_35, rules_14.as_mut_ptr());
    static mut tokens_14: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut p_36: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_17);
    (*p_36).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_36, tokens_14.as_mut_ptr(), clean);
    let mut rule_12: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh74 = (*rule_12).name;
    *fresh74 = b"cap\0" as *const u8 as *const libc::c_char;
    let ref mut fresh75 = (*rule_12).sibling;
    *fresh75 = (*parent).child;
    let ref mut fresh76 = (*parent).child;
    *fresh76 = rule_12;
    let mut parent_19: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_12);
    static mut tokens_15: [token_id; 7] = [TK_ISO, TK_TRN, TK_REF, TK_VAL, TK_BOX, TK_TAG, TK_NONE];
    let mut p_37: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_19);
    (*p_37).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_37, tokens_15.as_mut_ptr(), clean);
    let mut rule_13: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh77 = (*rule_13).name;
    *fresh77 = b"gencap\0" as *const u8 as *const libc::c_char;
    let ref mut fresh78 = (*rule_13).sibling;
    *fresh78 = (*parent).child;
    let ref mut fresh79 = (*parent).child;
    *fresh79 = rule_13;
    let mut parent_20: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_13);
    static mut tokens_16: [token_id; 6] = [
        TK_CAP_READ,
        TK_CAP_SEND,
        TK_CAP_SHARE,
        TK_CAP_ALIAS,
        TK_CAP_ANY,
        TK_NONE,
    ];
    let mut p_38: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_20);
    (*p_38).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_38, tokens_16.as_mut_ptr(), clean);
    let mut rule_14: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh80 = (*rule_14).name;
    *fresh80 = b"bare\0" as *const u8 as *const libc::c_char;
    let ref mut fresh81 = (*rule_14).sibling;
    *fresh81 = (*parent).child;
    let ref mut fresh82 = (*parent).child;
    *fresh82 = rule_14;
    let mut parent_21: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_14);
    static mut tokens_17: [token_id; 2] = [TK_AT, TK_NONE];
    let mut p_39: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_21);
    (*p_39).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_39, tokens_17.as_mut_ptr(), clean);
    let mut rule_15: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh83 = (*rule_15).name;
    *fresh83 = b"nominal\0" as *const u8 as *const libc::c_char;
    let ref mut fresh84 = (*rule_15).sibling;
    *fresh84 = (*parent).child;
    let ref mut fresh85 = (*parent).child;
    *fresh85 = rule_15;
    let mut parent_22: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_15);
    static mut tokens_18: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_40: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_22);
    (*p_40).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_40, tokens_18.as_mut_ptr(), clean);
    let mut p_41: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_22);
    let mut parent_23: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_41);
    static mut tokens_19: [token_id; 2] = [TK_DOT, TK_NONE];
    let mut p_42: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_23);
    (*p_42).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_42, tokens_19.as_mut_ptr(), clean);
    static mut tokens_20: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_43: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_23);
    (*p_43).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_43, tokens_20.as_mut_ptr(), clean);
    parent_23 = bnf_add(bnf_create(BNF_SEQ), p_41);
    optional = 1 as libc::c_int != 0;
    static mut rules_15: [*const libc::c_char; 2] = [
        b"typeargs\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_44: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_22);
    (*p_44).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_44, rules_15.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_16: [*const libc::c_char; 3] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        b"gencap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_45: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_22);
    (*p_45).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_45, rules_16.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_21: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut p_46: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_22);
    (*p_46).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_46, tokens_21.as_mut_ptr(), clean);
    let mut rule_16: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh86 = (*rule_16).name;
    *fresh86 = b"uniontype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh87 = (*rule_16).sibling;
    *fresh87 = (*parent).child;
    let ref mut fresh88 = (*parent).child;
    *fresh88 = rule_16;
    let mut parent_24: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_16);
    static mut tokens_22: [token_id; 2] = [TK_PIPE, TK_NONE];
    let mut p_47: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_24);
    (*p_47).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_47, tokens_22.as_mut_ptr(), clean);
    static mut rules_17: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_48: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_24);
    (*p_48).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_48, rules_17.as_mut_ptr());
    let mut rule_17: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh89 = (*rule_17).name;
    *fresh89 = b"isecttype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh90 = (*rule_17).sibling;
    *fresh90 = (*parent).child;
    let ref mut fresh91 = (*parent).child;
    *fresh91 = rule_17;
    let mut parent_25: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_17);
    static mut tokens_23: [token_id; 2] = [TK_ISECTTYPE, TK_NONE];
    let mut p_49: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_25);
    (*p_49).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_49, tokens_23.as_mut_ptr(), clean);
    static mut rules_18: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_50: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_25);
    (*p_50).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_50, rules_18.as_mut_ptr());
    let mut rule_18: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh92 = (*rule_18).name;
    *fresh92 = b"infixtype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh93 = (*rule_18).sibling;
    *fresh93 = (*parent).child;
    let ref mut fresh94 = (*parent).child;
    *fresh94 = rule_18;
    let mut parent_26: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_18);
    static mut rules_19: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_51: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_26);
    (*p_51).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_51, rules_19.as_mut_ptr());
    static mut tokens_24: [*const libc::c_char; 3] = [
        b"uniontype\0" as *const u8 as *const libc::c_char,
        b"isecttype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_52: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_26);
    p_52 = bnf_add(bnf_create(BNF_OR), p_52);
    bnf_rule_set(p_52, tokens_24.as_mut_ptr());
    let mut rule_19: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh95 = (*rule_19).name;
    *fresh95 = b"tupletype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh96 = (*rule_19).sibling;
    *fresh96 = (*parent).child;
    let ref mut fresh97 = (*parent).child;
    *fresh97 = rule_19;
    let mut parent_27: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_19);
    static mut tokens_25: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_53: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_27);
    (*p_53).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_53, tokens_25.as_mut_ptr(), clean);
    static mut rules_20: [*const libc::c_char; 2] = [
        b"infixtype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_54: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_27);
    (*p_54).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_54, rules_20.as_mut_ptr());
    let mut p_55: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_27);
    let mut parent_28: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_55);
    static mut tokens_26: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_56: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_28);
    (*p_56).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_56, tokens_26.as_mut_ptr(), clean);
    static mut rules_21: [*const libc::c_char; 2] = [
        b"infixtype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_57: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_28);
    (*p_57).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_57, rules_21.as_mut_ptr());
    let mut rule_20: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh98 = (*rule_20).name;
    *fresh98 = b"groupedtype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh99 = (*rule_20).sibling;
    *fresh99 = (*parent).child;
    let ref mut fresh100 = (*parent).child;
    *fresh100 = rule_20;
    let mut parent_29: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_20);
    (*rule_20).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_27: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_58: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_29);
    (*p_58).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_58, tokens_27.as_mut_ptr(), clean);
    static mut rules_22: [*const libc::c_char; 2] = [
        b"infixtype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_59: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_29);
    (*p_59).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_59, rules_22.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_23: [*const libc::c_char; 2] = [
        b"tupletype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_60: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_29);
    (*p_60).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_60, rules_23.as_mut_ptr());
    static mut tokens_28: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_61: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_29);
    (*p_61).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_61, tokens_28.as_mut_ptr(), clean);
    let mut rule_21: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh101 = (*rule_21).name;
    *fresh101 = b"thistype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh102 = (*rule_21).sibling;
    *fresh102 = (*parent).child;
    let ref mut fresh103 = (*parent).child;
    *fresh103 = rule_21;
    let mut parent_30: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_21);
    (*rule_21).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_29: [token_id; 2] = [TK_THIS, TK_NONE];
    let mut p_62: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_30);
    (*p_62).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_62, tokens_29.as_mut_ptr(), clean);
    let mut rule_22: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh104 = (*rule_22).name;
    *fresh104 = b"typelist\0" as *const u8 as *const libc::c_char;
    let ref mut fresh105 = (*rule_22).sibling;
    *fresh105 = (*parent).child;
    let ref mut fresh106 = (*parent).child;
    *fresh106 = rule_22;
    let mut parent_31: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_22);
    (*rule_22).inline_rule = 1 as libc::c_int != 0;
    static mut rules_24: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_63: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_31);
    (*p_63).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_63, rules_24.as_mut_ptr());
    let mut p_64: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_31);
    let mut parent_32: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_64);
    static mut tokens_30: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_65: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_32);
    (*p_65).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_65, tokens_30.as_mut_ptr(), clean);
    static mut rules_25: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_66: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_32);
    (*p_66).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_66, rules_25.as_mut_ptr());
    let mut rule_23: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh107 = (*rule_23).name;
    *fresh107 = b"lambdatype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh108 = (*rule_23).sibling;
    *fresh108 = (*parent).child;
    let ref mut fresh109 = (*parent).child;
    *fresh109 = rule_23;
    let mut parent_33: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_23);
    static mut tokens_31: [token_id; 2] = [TK_LBRACE, TK_NONE];
    let mut p_67: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_67).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_67, tokens_31.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_26: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_68: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_68).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_68, rules_26.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_32: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_69: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_69).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_69, tokens_32.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_27: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_70: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_70).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_70, rules_27.as_mut_ptr());
    static mut tokens_33: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_71: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_71).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_71, tokens_33.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_28: [*const libc::c_char; 2] = [
        b"typelist\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_72: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_72).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_72, rules_28.as_mut_ptr());
    static mut tokens_34: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_73: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_73).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_73, tokens_34.as_mut_ptr(), clean);
    let mut p_74: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_74).optional = 1 as libc::c_int != 0;
    let mut parent_34: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_74);
    static mut tokens_35: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_75: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_34);
    (*p_75).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_75, tokens_35.as_mut_ptr(), clean);
    static mut rules_29: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_76: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_34);
    (*p_76).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_76, rules_29.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_36: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_77: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_77).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_77, tokens_36.as_mut_ptr(), clean);
    static mut tokens_37: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut p_78: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_78).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_78, tokens_37.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_30: [*const libc::c_char; 3] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        b"gencap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_79: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_79).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_79, rules_30.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_38: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut p_80: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_33);
    (*p_80).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_80, tokens_38.as_mut_ptr(), clean);
    let mut rule_24: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh110 = (*rule_24).name;
    *fresh110 = b"barelambdatype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh111 = (*rule_24).sibling;
    *fresh111 = (*parent).child;
    let ref mut fresh112 = (*parent).child;
    *fresh112 = rule_24;
    let mut parent_35: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_24);
    static mut tokens_39: [token_id; 2] = [TK_AT_LBRACE, TK_NONE];
    let mut p_81: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_81).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_81, tokens_39.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_31: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_82: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_82).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_82, rules_31.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_40: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_83: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_83).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_83, tokens_40.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_32: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_84: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_84).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_84, rules_32.as_mut_ptr());
    static mut tokens_41: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_85: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_85).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_85, tokens_41.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_33: [*const libc::c_char; 2] = [
        b"typelist\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_86: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_86).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_86, rules_33.as_mut_ptr());
    static mut tokens_42: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_87: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_87).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_87, tokens_42.as_mut_ptr(), clean);
    let mut p_88: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_88).optional = 1 as libc::c_int != 0;
    let mut parent_36: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_88);
    static mut tokens_43: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_89: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_36);
    (*p_89).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_89, tokens_43.as_mut_ptr(), clean);
    static mut rules_34: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_90: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_36);
    (*p_90).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_90, rules_34.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_44: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_91: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_91).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_91, tokens_44.as_mut_ptr(), clean);
    static mut tokens_45: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut p_92: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_92).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_92, tokens_45.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_35: [*const libc::c_char; 3] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        b"gencap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_93: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_93).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_93, rules_35.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_46: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut p_94: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_35);
    (*p_94).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_94, tokens_46.as_mut_ptr(), clean);
    let mut rule_25: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh113 = (*rule_25).name;
    *fresh113 = b"atomtype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh114 = (*rule_25).sibling;
    *fresh114 = (*parent).child;
    let ref mut fresh115 = (*parent).child;
    *fresh115 = rule_25;
    let mut parent_37: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_25);
    static mut rules_36: [*const libc::c_char; 7] = [
        b"thistype\0" as *const u8 as *const libc::c_char,
        b"cap\0" as *const u8 as *const libc::c_char,
        b"groupedtype\0" as *const u8 as *const libc::c_char,
        b"nominal\0" as *const u8 as *const libc::c_char,
        b"lambdatype\0" as *const u8 as *const libc::c_char,
        b"barelambdatype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_95: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_37);
    (*p_95).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_95, rules_36.as_mut_ptr());
    let mut rule_26: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh116 = (*rule_26).name;
    *fresh116 = b"viewpoint\0" as *const u8 as *const libc::c_char;
    let ref mut fresh117 = (*rule_26).sibling;
    *fresh117 = (*parent).child;
    let ref mut fresh118 = (*parent).child;
    *fresh118 = rule_26;
    let mut parent_38: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_26);
    (*rule_26).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_47: [token_id; 2] = [TK_ARROW, TK_NONE];
    let mut p_96: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_38);
    (*p_96).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_96, tokens_47.as_mut_ptr(), clean);
    static mut rules_37: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_97: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_38);
    (*p_97).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_97, rules_37.as_mut_ptr());
    let mut rule_27: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh119 = (*rule_27).name;
    *fresh119 = b"type\0" as *const u8 as *const libc::c_char;
    let ref mut fresh120 = (*rule_27).sibling;
    *fresh120 = (*parent).child;
    let ref mut fresh121 = (*parent).child;
    *fresh121 = rule_27;
    let mut parent_39: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_27);
    static mut rules_38: [*const libc::c_char; 2] = [
        b"atomtype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_98: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_39);
    (*p_98).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_98, rules_38.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_39: [*const libc::c_char; 2] = [
        b"viewpoint\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_99: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_39);
    (*p_99).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_99, rules_39.as_mut_ptr());
    let mut rule_28: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh122 = (*rule_28).name;
    *fresh122 = b"namedarg\0" as *const u8 as *const libc::c_char;
    let ref mut fresh123 = (*rule_28).sibling;
    *fresh123 = (*parent).child;
    let ref mut fresh124 = (*parent).child;
    *fresh124 = rule_28;
    let mut parent_40: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_28);
    static mut tokens_48: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_100: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_40);
    (*p_100).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_100, tokens_48.as_mut_ptr(), clean);
    let mut p_101: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_40);
    let mut parent_41: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_101);
    static mut tokens_49: [token_id; 2] = [TK_TEST_UPDATEARG, TK_NONE];
    let mut p_102: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_41);
    (*p_102).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_102, tokens_49.as_mut_ptr(), clean);
    parent_41 = bnf_add(bnf_create(BNF_SEQ), p_101);
    static mut tokens_50: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_103: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_40);
    (*p_103).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_103, tokens_50.as_mut_ptr(), clean);
    static mut rules_40: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_104: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_40);
    (*p_104).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_104, rules_40.as_mut_ptr());
    let mut rule_29: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh125 = (*rule_29).name;
    *fresh125 = b"named\0" as *const u8 as *const libc::c_char;
    let ref mut fresh126 = (*rule_29).sibling;
    *fresh126 = (*parent).child;
    let ref mut fresh127 = (*parent).child;
    *fresh127 = rule_29;
    let mut parent_42: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_29);
    static mut tokens_51: [token_id; 2] = [TK_WHERE, TK_NONE];
    let mut p_105: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_42);
    (*p_105).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_105, tokens_51.as_mut_ptr(), clean);
    static mut rules_41: [*const libc::c_char; 2] = [
        b"namedarg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_106: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_42);
    (*p_106).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_106, rules_41.as_mut_ptr());
    let mut p_107: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_42);
    let mut parent_43: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_107);
    static mut tokens_52: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_108: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_43);
    (*p_108).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_108, tokens_52.as_mut_ptr(), clean);
    static mut rules_42: [*const libc::c_char; 2] = [
        b"namedarg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_109: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_43);
    (*p_109).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_109, rules_42.as_mut_ptr());
    let mut rule_30: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh128 = (*rule_30).name;
    *fresh128 = b"positional\0" as *const u8 as *const libc::c_char;
    let ref mut fresh129 = (*rule_30).sibling;
    *fresh129 = (*parent).child;
    let ref mut fresh130 = (*parent).child;
    *fresh130 = rule_30;
    let mut parent_44: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_30);
    static mut rules_43: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_110: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_44);
    (*p_110).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_110, rules_43.as_mut_ptr());
    let mut p_111: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_44);
    let mut parent_45: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_111);
    static mut tokens_53: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_112: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_45);
    (*p_112).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_112, tokens_53.as_mut_ptr(), clean);
    static mut rules_44: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_113: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_45);
    (*p_113).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_113, rules_44.as_mut_ptr());
    let mut rule_31: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh131 = (*rule_31).name;
    *fresh131 = b"annotations\0" as *const u8 as *const libc::c_char;
    let ref mut fresh132 = (*rule_31).sibling;
    *fresh132 = (*parent).child;
    let ref mut fresh133 = (*parent).child;
    *fresh133 = rule_31;
    let mut parent_46: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_31);
    (*rule_31).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_54: [token_id; 2] = [TK_BACKSLASH, TK_NONE];
    let mut p_114: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_46);
    (*p_114).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_114, tokens_54.as_mut_ptr(), clean);
    static mut tokens_55: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_115: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_46);
    (*p_115).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_115, tokens_55.as_mut_ptr(), clean);
    let mut p_116: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_46);
    let mut parent_47: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_116);
    static mut tokens_56: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_117: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_47);
    (*p_117).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_117, tokens_56.as_mut_ptr(), clean);
    static mut tokens_57: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_118: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_47);
    (*p_118).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_118, tokens_57.as_mut_ptr(), clean);
    static mut tokens_58: [token_id; 2] = [TK_BACKSLASH, TK_NONE];
    let mut p_119: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_46);
    (*p_119).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_119, tokens_58.as_mut_ptr(), clean);
    let mut rule_32: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh134 = (*rule_32).name;
    *fresh134 = b"object\0" as *const u8 as *const libc::c_char;
    let ref mut fresh135 = (*rule_32).sibling;
    *fresh135 = (*parent).child;
    let ref mut fresh136 = (*parent).child;
    *fresh136 = rule_32;
    let mut parent_48: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_32);
    (*rule_32).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_59: [token_id; 2] = [TK_OBJECT, TK_NONE];
    let mut p_120: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_120).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_120, tokens_59.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_45: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_121: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_121).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_121, rules_45.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_46: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_122: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_122).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_122, rules_46.as_mut_ptr());
    let mut p_123: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_123).optional = 1 as libc::c_int != 0;
    let mut parent_49: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_123);
    static mut tokens_60: [token_id; 2] = [TK_IS, TK_NONE];
    let mut p_124: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_49);
    (*p_124).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_124, tokens_60.as_mut_ptr(), clean);
    static mut rules_47: [*const libc::c_char; 2] = [
        b"provides\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_125: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_49);
    (*p_125).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_125, rules_47.as_mut_ptr());
    static mut rules_48: [*const libc::c_char; 2] = [
        b"members\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_126: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_126).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_126, rules_48.as_mut_ptr());
    static mut tokens_61: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_127: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_48);
    (*p_127).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_127, tokens_61.as_mut_ptr(), clean);
    let mut rule_33: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh137 = (*rule_33).name;
    *fresh137 = b"lambdaparam\0" as *const u8 as *const libc::c_char;
    let ref mut fresh138 = (*rule_33).sibling;
    *fresh138 = (*parent).child;
    let ref mut fresh139 = (*parent).child;
    *fresh139 = rule_33;
    let mut parent_50: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_33);
    static mut tokens_62: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_128: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_50);
    (*p_128).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_128, tokens_62.as_mut_ptr(), clean);
    let mut p_129: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_50);
    (*p_129).optional = 1 as libc::c_int != 0;
    let mut parent_51: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_129);
    static mut tokens_63: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_130: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_51);
    (*p_130).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_130, tokens_63.as_mut_ptr(), clean);
    static mut rules_49: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_131: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_51);
    (*p_131).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_131, rules_49.as_mut_ptr());
    let mut p_132: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_50);
    (*p_132).optional = 1 as libc::c_int != 0;
    let mut parent_52: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_132);
    static mut tokens_64: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_133: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_52);
    (*p_133).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_133, tokens_64.as_mut_ptr(), clean);
    static mut rules_50: [*const libc::c_char; 2] = [
        b"defaultarg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_134: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_52);
    (*p_134).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_134, rules_50.as_mut_ptr());
    let mut rule_34: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh140 = (*rule_34).name;
    *fresh140 = b"lambdaparams\0" as *const u8 as *const libc::c_char;
    let ref mut fresh141 = (*rule_34).sibling;
    *fresh141 = (*parent).child;
    let ref mut fresh142 = (*parent).child;
    *fresh142 = rule_34;
    let mut parent_53: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_34);
    static mut rules_51: [*const libc::c_char; 2] = [
        b"lambdaparam\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_135: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_53);
    (*p_135).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_135, rules_51.as_mut_ptr());
    let mut p_136: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_53);
    let mut parent_54: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_136);
    static mut tokens_65: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_137: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_54);
    (*p_137).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_137, tokens_65.as_mut_ptr(), clean);
    static mut rules_52: [*const libc::c_char; 2] = [
        b"lambdaparam\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_138: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_54);
    (*p_138).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_138, rules_52.as_mut_ptr());
    let mut rule_35: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh143 = (*rule_35).name;
    *fresh143 = b"lambdacapture\0" as *const u8 as *const libc::c_char;
    let ref mut fresh144 = (*rule_35).sibling;
    *fresh144 = (*parent).child;
    let ref mut fresh145 = (*parent).child;
    *fresh145 = rule_35;
    let mut parent_55: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_35);
    static mut tokens_66: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_139: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_55);
    (*p_139).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_139, tokens_66.as_mut_ptr(), clean);
    let mut p_140: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_55);
    (*p_140).optional = 1 as libc::c_int != 0;
    let mut parent_56: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_140);
    static mut tokens_67: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_141: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_56);
    (*p_141).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_141, tokens_67.as_mut_ptr(), clean);
    static mut rules_53: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_142: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_56);
    (*p_142).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_142, rules_53.as_mut_ptr());
    let mut p_143: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_55);
    (*p_143).optional = 1 as libc::c_int != 0;
    let mut parent_57: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_143);
    static mut tokens_68: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_144: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_57);
    (*p_144).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_144, tokens_68.as_mut_ptr(), clean);
    static mut rules_54: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_145: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_57);
    (*p_145).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_145, rules_54.as_mut_ptr());
    let mut rule_36: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh146 = (*rule_36).name;
    *fresh146 = b"lambdacaptures\0" as *const u8 as *const libc::c_char;
    let ref mut fresh147 = (*rule_36).sibling;
    *fresh147 = (*parent).child;
    let ref mut fresh148 = (*parent).child;
    *fresh148 = rule_36;
    let mut parent_58: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_36);
    static mut tokens_69: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_146: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_58);
    (*p_146).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_146, tokens_69.as_mut_ptr(), clean);
    static mut rules_55: [*const libc::c_char; 3] = [
        b"lambdacapture\0" as *const u8 as *const libc::c_char,
        b"thisliteral\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_147: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_58);
    (*p_147).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_147, rules_55.as_mut_ptr());
    let mut p_148: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_58);
    let mut parent_59: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_148);
    static mut tokens_70: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_149: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_59);
    (*p_149).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_149, tokens_70.as_mut_ptr(), clean);
    static mut rules_56: [*const libc::c_char; 3] = [
        b"lambdacapture\0" as *const u8 as *const libc::c_char,
        b"thisliteral\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_150: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_59);
    (*p_150).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_150, rules_56.as_mut_ptr());
    static mut tokens_71: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_151: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_58);
    (*p_151).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_151, tokens_71.as_mut_ptr(), clean);
    let mut rule_37: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh149 = (*rule_37).name;
    *fresh149 = b"lambda\0" as *const u8 as *const libc::c_char;
    let ref mut fresh150 = (*rule_37).sibling;
    *fresh150 = (*parent).child;
    let ref mut fresh151 = (*parent).child;
    *fresh151 = rule_37;
    let mut parent_60: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_37);
    (*rule_37).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_72: [token_id; 2] = [TK_LBRACE, TK_NONE];
    let mut p_152: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_152).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_152, tokens_72.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_57: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_153: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_153).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_153, rules_57.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_58: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_154: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_154).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_154, rules_58.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_73: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_155: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_155).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_155, tokens_73.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_59: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_156: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_156).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_156, rules_59.as_mut_ptr());
    static mut tokens_74: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_157: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_157).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_157, tokens_74.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_60: [*const libc::c_char; 2] = [
        b"lambdaparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_158: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_158).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_158, rules_60.as_mut_ptr());
    static mut tokens_75: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_159: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_159).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_159, tokens_75.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_61: [*const libc::c_char; 2] = [
        b"lambdacaptures\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_160: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_160).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_160, rules_61.as_mut_ptr());
    let mut p_161: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_161).optional = 1 as libc::c_int != 0;
    let mut parent_61: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_161);
    static mut tokens_76: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_162: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_61);
    (*p_162).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_162, tokens_76.as_mut_ptr(), clean);
    static mut rules_62: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_163: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_61);
    (*p_163).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_163, rules_62.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_77: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_164: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_164).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_164, tokens_77.as_mut_ptr(), clean);
    static mut tokens_78: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut p_165: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_165).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_165, tokens_78.as_mut_ptr(), clean);
    static mut rules_63: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_166: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_166).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_166, rules_63.as_mut_ptr());
    static mut tokens_79: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut p_167: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_167).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_167, tokens_79.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_64: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_168: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_60);
    (*p_168).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_168, rules_64.as_mut_ptr());
    let mut rule_38: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh152 = (*rule_38).name;
    *fresh152 = b"barelambda\0" as *const u8 as *const libc::c_char;
    let ref mut fresh153 = (*rule_38).sibling;
    *fresh153 = (*parent).child;
    let ref mut fresh154 = (*parent).child;
    *fresh154 = rule_38;
    let mut parent_62: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_38);
    (*rule_38).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_80: [token_id; 2] = [TK_AT_LBRACE, TK_NONE];
    let mut p_169: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_169).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_169, tokens_80.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_65: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_170: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_170).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_170, rules_65.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_66: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_171: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_171).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_171, rules_66.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_81: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_172: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_172).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_172, tokens_81.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_67: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_173: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_173).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_173, rules_67.as_mut_ptr());
    static mut tokens_82: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_174: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_174).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_174, tokens_82.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_68: [*const libc::c_char; 2] = [
        b"lambdaparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_175: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_175).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_175, rules_68.as_mut_ptr());
    static mut tokens_83: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_176: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_176).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_176, tokens_83.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_69: [*const libc::c_char; 2] = [
        b"lambdacaptures\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_177: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_177).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_177, rules_69.as_mut_ptr());
    let mut p_178: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_178).optional = 1 as libc::c_int != 0;
    let mut parent_63: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_178);
    static mut tokens_84: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_179: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_63);
    (*p_179).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_179, tokens_84.as_mut_ptr(), clean);
    static mut rules_70: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_180: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_63);
    (*p_180).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_180, rules_70.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_85: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_181: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_181).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_181, tokens_85.as_mut_ptr(), clean);
    static mut tokens_86: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut p_182: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_182).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_182, tokens_86.as_mut_ptr(), clean);
    static mut rules_71: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_183: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_183).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_183, rules_71.as_mut_ptr());
    static mut tokens_87: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut p_184: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_184).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_184, tokens_87.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_72: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_185: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_62);
    (*p_185).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_185, rules_72.as_mut_ptr());
    let mut rule_39: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh155 = (*rule_39).name;
    *fresh155 = b"arraytype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh156 = (*rule_39).sibling;
    *fresh156 = (*parent).child;
    let ref mut fresh157 = (*parent).child;
    *fresh157 = rule_39;
    let mut parent_64: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_39);
    (*rule_39).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_88: [token_id; 2] = [TK_AS, TK_NONE];
    let mut p_186: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_64);
    (*p_186).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_186, tokens_88.as_mut_ptr(), clean);
    static mut rules_73: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_187: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_64);
    (*p_187).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_187, rules_73.as_mut_ptr());
    static mut tokens_89: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_188: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_64);
    (*p_188).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_188, tokens_89.as_mut_ptr(), clean);
    let mut rule_40: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh158 = (*rule_40).name;
    *fresh158 = b"array\0" as *const u8 as *const libc::c_char;
    let ref mut fresh159 = (*rule_40).sibling;
    *fresh159 = (*parent).child;
    let ref mut fresh160 = (*parent).child;
    *fresh160 = rule_40;
    let mut parent_65: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_40);
    (*rule_40).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_90: [token_id; 3] = [TK_LSQUARE, TK_LSQUARE_NEW, TK_NONE];
    let mut p_189: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_65);
    (*p_189).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_189, tokens_90.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_74: [*const libc::c_char; 2] = [
        b"arraytype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_190: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_65);
    (*p_190).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_190, rules_74.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_75: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_191: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_65);
    (*p_191).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_191, rules_75.as_mut_ptr());
    static mut tokens_91: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut p_192: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_65);
    (*p_192).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_192, tokens_91.as_mut_ptr(), clean);
    let mut rule_41: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh161 = (*rule_41).name;
    *fresh161 = b"nextarray\0" as *const u8 as *const libc::c_char;
    let ref mut fresh162 = (*rule_41).sibling;
    *fresh162 = (*parent).child;
    let ref mut fresh163 = (*parent).child;
    *fresh163 = rule_41;
    let mut parent_66: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_41);
    (*rule_41).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_92: [token_id; 2] = [TK_LSQUARE_NEW, TK_NONE];
    let mut p_193: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_66);
    (*p_193).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_193, tokens_92.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_76: [*const libc::c_char; 2] = [
        b"arraytype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_194: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_66);
    (*p_194).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_194, rules_76.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_77: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_195: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_66);
    (*p_195).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_195, rules_77.as_mut_ptr());
    static mut tokens_93: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut p_196: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_66);
    (*p_196).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_196, tokens_93.as_mut_ptr(), clean);
    let mut rule_42: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh164 = (*rule_42).name;
    *fresh164 = b"tuple\0" as *const u8 as *const libc::c_char;
    let ref mut fresh165 = (*rule_42).sibling;
    *fresh165 = (*parent).child;
    let ref mut fresh166 = (*parent).child;
    *fresh166 = rule_42;
    let mut parent_67: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_42);
    static mut tokens_94: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_197: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_67);
    (*p_197).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_197, tokens_94.as_mut_ptr(), clean);
    static mut rules_78: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_198: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_67);
    (*p_198).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_198, rules_78.as_mut_ptr());
    let mut p_199: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_67);
    let mut parent_68: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_199);
    static mut tokens_95: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_200: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_68);
    (*p_200).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_200, tokens_95.as_mut_ptr(), clean);
    static mut rules_79: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_201: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_68);
    (*p_201).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_201, rules_79.as_mut_ptr());
    let mut rule_43: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh167 = (*rule_43).name;
    *fresh167 = b"groupedexpr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh168 = (*rule_43).sibling;
    *fresh168 = (*parent).child;
    let ref mut fresh169 = (*parent).child;
    *fresh169 = rule_43;
    let mut parent_69: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_43);
    (*rule_43).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_96: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_202: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_69);
    (*p_202).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_202, tokens_96.as_mut_ptr(), clean);
    static mut rules_80: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_203: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_69);
    (*p_203).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_203, rules_80.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_81: [*const libc::c_char; 2] = [
        b"tuple\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_204: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_69);
    (*p_204).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_204, rules_81.as_mut_ptr());
    static mut tokens_97: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_205: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_69);
    (*p_205).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_205, tokens_97.as_mut_ptr(), clean);
    let mut rule_44: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh170 = (*rule_44).name;
    *fresh170 = b"nextgroupedexpr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh171 = (*rule_44).sibling;
    *fresh171 = (*parent).child;
    let ref mut fresh172 = (*parent).child;
    *fresh172 = rule_44;
    let mut parent_70: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_44);
    (*rule_44).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_98: [token_id; 2] = [TK_LPAREN_NEW, TK_NONE];
    let mut p_206: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_70);
    (*p_206).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_206, tokens_98.as_mut_ptr(), clean);
    static mut rules_82: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_207: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_70);
    (*p_207).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_207, rules_82.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_83: [*const libc::c_char; 2] = [
        b"tuple\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_208: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_70);
    (*p_208).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_208, rules_83.as_mut_ptr());
    static mut tokens_99: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_209: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_70);
    (*p_209).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_209, tokens_99.as_mut_ptr(), clean);
    let mut rule_45: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh173 = (*rule_45).name;
    *fresh173 = b"thisliteral\0" as *const u8 as *const libc::c_char;
    let ref mut fresh174 = (*rule_45).sibling;
    *fresh174 = (*parent).child;
    let ref mut fresh175 = (*parent).child;
    *fresh175 = rule_45;
    let mut parent_71: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_45);
    static mut tokens_100: [token_id; 2] = [TK_THIS, TK_NONE];
    let mut p_210: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_71);
    (*p_210).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_210, tokens_100.as_mut_ptr(), clean);
    let mut rule_46: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh176 = (*rule_46).name;
    *fresh176 = b"ref\0" as *const u8 as *const libc::c_char;
    let ref mut fresh177 = (*rule_46).sibling;
    *fresh177 = (*parent).child;
    let ref mut fresh178 = (*parent).child;
    *fresh178 = rule_46;
    let mut parent_72: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_46);
    (*rule_46).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_101: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_211: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_72);
    (*p_211).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_211, tokens_101.as_mut_ptr(), clean);
    let mut rule_47: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh179 = (*rule_47).name;
    *fresh179 = b"location\0" as *const u8 as *const libc::c_char;
    let ref mut fresh180 = (*rule_47).sibling;
    *fresh180 = (*parent).child;
    let ref mut fresh181 = (*parent).child;
    *fresh181 = rule_47;
    let mut parent_73: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_47);
    (*rule_47).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_102: [token_id; 2] = [TK_LOCATION, TK_NONE];
    let mut p_212: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_73);
    (*p_212).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_212, tokens_102.as_mut_ptr(), clean);
    let mut rule_48: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh182 = (*rule_48).name;
    *fresh182 = b"ffi\0" as *const u8 as *const libc::c_char;
    let ref mut fresh183 = (*rule_48).sibling;
    *fresh183 = (*parent).child;
    let ref mut fresh184 = (*parent).child;
    *fresh184 = rule_48;
    let mut parent_74: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_48);
    (*rule_48).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_103: [token_id; 2] = [TK_AT, TK_NONE];
    let mut p_213: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_213).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_213, tokens_103.as_mut_ptr(), clean);
    static mut tokens_104: [token_id; 3] = [TK_ID, TK_STRING, TK_NONE];
    let mut p_214: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_214).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_214, tokens_104.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_84: [*const libc::c_char; 2] = [
        b"typeargs\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_215: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_215).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_215, rules_84.as_mut_ptr());
    static mut tokens_105: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_216: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_216).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_216, tokens_105.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_85: [*const libc::c_char; 2] = [
        b"positional\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_217: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_217).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_217, rules_85.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_86: [*const libc::c_char; 2] = [
        b"named\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_218: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_218).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_218, rules_86.as_mut_ptr());
    static mut tokens_106: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_219: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_219).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_219, tokens_106.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut tokens_107: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_220: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_74);
    (*p_220).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_220, tokens_107.as_mut_ptr(), clean);
    let mut rule_49: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh185 = (*rule_49).name;
    *fresh185 = b"atom\0" as *const u8 as *const libc::c_char;
    let ref mut fresh186 = (*rule_49).sibling;
    *fresh186 = (*parent).child;
    let ref mut fresh187 = (*parent).child;
    *fresh187 = rule_49;
    let mut parent_75: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_49);
    static mut rules_87: [*const libc::c_char; 14] = [
        b"ref\0" as *const u8 as *const libc::c_char,
        b"thisliteral\0" as *const u8 as *const libc::c_char,
        b"literal\0" as *const u8 as *const libc::c_char,
        b"groupedexpr\0" as *const u8 as *const libc::c_char,
        b"array\0" as *const u8 as *const libc::c_char,
        b"object\0" as *const u8 as *const libc::c_char,
        b"lambda\0" as *const u8 as *const libc::c_char,
        b"barelambda\0" as *const u8 as *const libc::c_char,
        b"ffi\0" as *const u8 as *const libc::c_char,
        b"location\0" as *const u8 as *const libc::c_char,
        b"cond\0" as *const u8 as *const libc::c_char,
        b"whileloop\0" as *const u8 as *const libc::c_char,
        b"forloop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_221: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_75);
    (*p_221).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_221, rules_87.as_mut_ptr());
    let mut rule_50: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh188 = (*rule_50).name;
    *fresh188 = b"caseatom\0" as *const u8 as *const libc::c_char;
    let ref mut fresh189 = (*rule_50).sibling;
    *fresh189 = (*parent).child;
    let ref mut fresh190 = (*parent).child;
    *fresh190 = rule_50;
    let mut parent_76: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_50);
    static mut rules_88: [*const libc::c_char; 13] = [
        b"ref\0" as *const u8 as *const libc::c_char,
        b"thisliteral\0" as *const u8 as *const libc::c_char,
        b"literal\0" as *const u8 as *const libc::c_char,
        b"groupedexpr\0" as *const u8 as *const libc::c_char,
        b"array\0" as *const u8 as *const libc::c_char,
        b"object\0" as *const u8 as *const libc::c_char,
        b"lambda\0" as *const u8 as *const libc::c_char,
        b"barelambda\0" as *const u8 as *const libc::c_char,
        b"ffi\0" as *const u8 as *const libc::c_char,
        b"location\0" as *const u8 as *const libc::c_char,
        b"whileloop\0" as *const u8 as *const libc::c_char,
        b"forloop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_222: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_76);
    (*p_222).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_222, rules_88.as_mut_ptr());
    let mut rule_51: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh191 = (*rule_51).name;
    *fresh191 = b"nextatom\0" as *const u8 as *const libc::c_char;
    let ref mut fresh192 = (*rule_51).sibling;
    *fresh192 = (*parent).child;
    let ref mut fresh193 = (*parent).child;
    *fresh193 = rule_51;
    let mut parent_77: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_51);
    static mut rules_89: [*const libc::c_char; 14] = [
        b"ref\0" as *const u8 as *const libc::c_char,
        b"thisliteral\0" as *const u8 as *const libc::c_char,
        b"literal\0" as *const u8 as *const libc::c_char,
        b"nextgroupedexpr\0" as *const u8 as *const libc::c_char,
        b"nextarray\0" as *const u8 as *const libc::c_char,
        b"object\0" as *const u8 as *const libc::c_char,
        b"lambda\0" as *const u8 as *const libc::c_char,
        b"barelambda\0" as *const u8 as *const libc::c_char,
        b"ffi\0" as *const u8 as *const libc::c_char,
        b"location\0" as *const u8 as *const libc::c_char,
        b"cond\0" as *const u8 as *const libc::c_char,
        b"whileloop\0" as *const u8 as *const libc::c_char,
        b"forloop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_223: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_77);
    (*p_223).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_223, rules_89.as_mut_ptr());
    let mut rule_52: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh194 = (*rule_52).name;
    *fresh194 = b"dot\0" as *const u8 as *const libc::c_char;
    let ref mut fresh195 = (*rule_52).sibling;
    *fresh195 = (*parent).child;
    let ref mut fresh196 = (*parent).child;
    *fresh196 = rule_52;
    let mut parent_78: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_52);
    static mut tokens_108: [token_id; 2] = [TK_DOT, TK_NONE];
    let mut p_224: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_78);
    (*p_224).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_224, tokens_108.as_mut_ptr(), clean);
    static mut tokens_109: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_225: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_78);
    (*p_225).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_225, tokens_109.as_mut_ptr(), clean);
    let mut rule_53: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh197 = (*rule_53).name;
    *fresh197 = b"tilde\0" as *const u8 as *const libc::c_char;
    let ref mut fresh198 = (*rule_53).sibling;
    *fresh198 = (*parent).child;
    let ref mut fresh199 = (*parent).child;
    *fresh199 = rule_53;
    let mut parent_79: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_53);
    static mut tokens_110: [token_id; 2] = [TK_TILDE, TK_NONE];
    let mut p_226: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_79);
    (*p_226).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_226, tokens_110.as_mut_ptr(), clean);
    static mut tokens_111: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_227: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_79);
    (*p_227).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_227, tokens_111.as_mut_ptr(), clean);
    let mut rule_54: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh200 = (*rule_54).name;
    *fresh200 = b"chain\0" as *const u8 as *const libc::c_char;
    let ref mut fresh201 = (*rule_54).sibling;
    *fresh201 = (*parent).child;
    let ref mut fresh202 = (*parent).child;
    *fresh202 = rule_54;
    let mut parent_80: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_54);
    static mut tokens_112: [token_id; 2] = [TK_CHAIN, TK_NONE];
    let mut p_228: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_80);
    (*p_228).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_228, tokens_112.as_mut_ptr(), clean);
    static mut tokens_113: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_229: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_80);
    (*p_229).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_229, tokens_113.as_mut_ptr(), clean);
    let mut rule_55: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh203 = (*rule_55).name;
    *fresh203 = b"qualify\0" as *const u8 as *const libc::c_char;
    let ref mut fresh204 = (*rule_55).sibling;
    *fresh204 = (*parent).child;
    let ref mut fresh205 = (*parent).child;
    *fresh205 = rule_55;
    let mut parent_81: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_55);
    static mut rules_90: [*const libc::c_char; 2] = [
        b"typeargs\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_230: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_81);
    (*p_230).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_230, rules_90.as_mut_ptr());
    let mut rule_56: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh206 = (*rule_56).name;
    *fresh206 = b"call\0" as *const u8 as *const libc::c_char;
    let ref mut fresh207 = (*rule_56).sibling;
    *fresh207 = (*parent).child;
    let ref mut fresh208 = (*parent).child;
    *fresh208 = rule_56;
    let mut parent_82: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_56);
    static mut tokens_114: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut p_231: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_82);
    (*p_231).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_231, tokens_114.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_91: [*const libc::c_char; 2] = [
        b"positional\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_232: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_82);
    (*p_232).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_232, rules_91.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_92: [*const libc::c_char; 2] = [
        b"named\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_233: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_82);
    (*p_233).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_233, rules_92.as_mut_ptr());
    static mut tokens_115: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_234: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_82);
    (*p_234).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_234, tokens_115.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut tokens_116: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_235: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_82);
    (*p_235).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_235, tokens_116.as_mut_ptr(), clean);
    let mut rule_57: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh209 = (*rule_57).name;
    *fresh209 = b"postfix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh210 = (*rule_57).sibling;
    *fresh210 = (*parent).child;
    let ref mut fresh211 = (*parent).child;
    *fresh211 = rule_57;
    let mut parent_83: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_57);
    static mut rules_93: [*const libc::c_char; 2] = [
        b"atom\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_236: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_83);
    (*p_236).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_236, rules_93.as_mut_ptr());
    static mut tokens_117: [*const libc::c_char; 6] = [
        b"dot\0" as *const u8 as *const libc::c_char,
        b"tilde\0" as *const u8 as *const libc::c_char,
        b"chain\0" as *const u8 as *const libc::c_char,
        b"qualify\0" as *const u8 as *const libc::c_char,
        b"call\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_237: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_83);
    p_237 = bnf_add(bnf_create(BNF_OR), p_237);
    bnf_rule_set(p_237, tokens_117.as_mut_ptr());
    let mut rule_58: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh212 = (*rule_58).name;
    *fresh212 = b"casepostfix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh213 = (*rule_58).sibling;
    *fresh213 = (*parent).child;
    let ref mut fresh214 = (*parent).child;
    *fresh214 = rule_58;
    let mut parent_84: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_58);
    static mut rules_94: [*const libc::c_char; 2] = [
        b"caseatom\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_238: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_84);
    (*p_238).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_238, rules_94.as_mut_ptr());
    static mut tokens_118: [*const libc::c_char; 6] = [
        b"dot\0" as *const u8 as *const libc::c_char,
        b"tilde\0" as *const u8 as *const libc::c_char,
        b"chain\0" as *const u8 as *const libc::c_char,
        b"qualify\0" as *const u8 as *const libc::c_char,
        b"call\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_239: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_84);
    p_239 = bnf_add(bnf_create(BNF_OR), p_239);
    bnf_rule_set(p_239, tokens_118.as_mut_ptr());
    let mut rule_59: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh215 = (*rule_59).name;
    *fresh215 = b"nextpostfix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh216 = (*rule_59).sibling;
    *fresh216 = (*parent).child;
    let ref mut fresh217 = (*parent).child;
    *fresh217 = rule_59;
    let mut parent_85: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_59);
    static mut rules_95: [*const libc::c_char; 2] = [
        b"nextatom\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_240: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_85);
    (*p_240).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_240, rules_95.as_mut_ptr());
    static mut tokens_119: [*const libc::c_char; 6] = [
        b"dot\0" as *const u8 as *const libc::c_char,
        b"tilde\0" as *const u8 as *const libc::c_char,
        b"chain\0" as *const u8 as *const libc::c_char,
        b"qualify\0" as *const u8 as *const libc::c_char,
        b"call\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_241: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_85);
    p_241 = bnf_add(bnf_create(BNF_OR), p_241);
    bnf_rule_set(p_241, tokens_119.as_mut_ptr());
    let mut rule_60: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh218 = (*rule_60).name;
    *fresh218 = b"local\0" as *const u8 as *const libc::c_char;
    let ref mut fresh219 = (*rule_60).sibling;
    *fresh219 = (*parent).child;
    let ref mut fresh220 = (*parent).child;
    *fresh220 = rule_60;
    let mut parent_86: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_60);
    (*rule_60).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_120: [token_id; 5] = [TK_VAR, TK_LET, TK_EMBED, TK_MATCH_CAPTURE, TK_NONE];
    let mut p_242: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_86);
    (*p_242).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_242, tokens_120.as_mut_ptr(), clean);
    static mut tokens_121: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_243: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_86);
    (*p_243).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_243, tokens_121.as_mut_ptr(), clean);
    let mut p_244: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_86);
    (*p_244).optional = 1 as libc::c_int != 0;
    let mut parent_87: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_244);
    static mut tokens_122: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_245: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_87);
    (*p_245).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_245, tokens_122.as_mut_ptr(), clean);
    static mut rules_96: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_246: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_87);
    (*p_246).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_246, rules_96.as_mut_ptr());
    let mut rule_61: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh221 = (*rule_61).name;
    *fresh221 = b"prefix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh222 = (*rule_61).sibling;
    *fresh222 = (*parent).child;
    let ref mut fresh223 = (*parent).child;
    *fresh223 = rule_61;
    let mut parent_88: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_61);
    (*rule_61).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_123: [token_id; 8] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS,
        TK_MINUS_TILDE,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut p_247: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_88);
    (*p_247).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_247, tokens_123.as_mut_ptr(), clean);
    static mut rules_97: [*const libc::c_char; 2] = [
        b"parampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_248: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_88);
    (*p_248).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_248, rules_97.as_mut_ptr());
    let mut rule_62: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh224 = (*rule_62).name;
    *fresh224 = b"caseprefix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh225 = (*rule_62).sibling;
    *fresh225 = (*parent).child;
    let ref mut fresh226 = (*parent).child;
    *fresh226 = rule_62;
    let mut parent_89: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_62);
    (*rule_62).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_124: [token_id; 8] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS,
        TK_MINUS_TILDE,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut p_249: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_89);
    (*p_249).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_249, tokens_124.as_mut_ptr(), clean);
    static mut rules_98: [*const libc::c_char; 2] = [
        b"caseparampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_250: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_89);
    (*p_250).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_250, rules_98.as_mut_ptr());
    let mut rule_63: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh227 = (*rule_63).name;
    *fresh227 = b"nextprefix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh228 = (*rule_63).sibling;
    *fresh228 = (*parent).child;
    let ref mut fresh229 = (*parent).child;
    *fresh229 = rule_63;
    let mut parent_90: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_63);
    (*rule_63).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_125: [token_id; 6] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut p_251: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_90);
    (*p_251).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_251, tokens_125.as_mut_ptr(), clean);
    static mut rules_99: [*const libc::c_char; 2] = [
        b"parampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_252: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_90);
    (*p_252).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_252, rules_99.as_mut_ptr());
    let mut rule_64: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh230 = (*rule_64).name;
    *fresh230 = b"parampattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh231 = (*rule_64).sibling;
    *fresh231 = (*parent).child;
    let ref mut fresh232 = (*parent).child;
    *fresh232 = rule_64;
    let mut parent_91: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_64);
    static mut rules_100: [*const libc::c_char; 3] = [
        b"prefix\0" as *const u8 as *const libc::c_char,
        b"postfix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_253: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_91);
    (*p_253).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_253, rules_100.as_mut_ptr());
    let mut rule_65: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh233 = (*rule_65).name;
    *fresh233 = b"caseparampattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh234 = (*rule_65).sibling;
    *fresh234 = (*parent).child;
    let ref mut fresh235 = (*parent).child;
    *fresh235 = rule_65;
    let mut parent_92: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_65);
    static mut rules_101: [*const libc::c_char; 3] = [
        b"caseprefix\0" as *const u8 as *const libc::c_char,
        b"casepostfix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_254: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_92);
    (*p_254).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_254, rules_101.as_mut_ptr());
    let mut rule_66: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh236 = (*rule_66).name;
    *fresh236 = b"nextparampattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh237 = (*rule_66).sibling;
    *fresh237 = (*parent).child;
    let ref mut fresh238 = (*parent).child;
    *fresh238 = rule_66;
    let mut parent_93: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_66);
    static mut rules_102: [*const libc::c_char; 3] = [
        b"nextprefix\0" as *const u8 as *const libc::c_char,
        b"nextpostfix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_255: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_93);
    (*p_255).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_255, rules_102.as_mut_ptr());
    let mut rule_67: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh239 = (*rule_67).name;
    *fresh239 = b"pattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh240 = (*rule_67).sibling;
    *fresh240 = (*parent).child;
    let ref mut fresh241 = (*parent).child;
    *fresh241 = rule_67;
    let mut parent_94: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_67);
    static mut rules_103: [*const libc::c_char; 3] = [
        b"local\0" as *const u8 as *const libc::c_char,
        b"parampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_256: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_94);
    (*p_256).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_256, rules_103.as_mut_ptr());
    let mut rule_68: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh242 = (*rule_68).name;
    *fresh242 = b"casepattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh243 = (*rule_68).sibling;
    *fresh243 = (*parent).child;
    let ref mut fresh244 = (*parent).child;
    *fresh244 = rule_68;
    let mut parent_95: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_68);
    static mut rules_104: [*const libc::c_char; 3] = [
        b"local\0" as *const u8 as *const libc::c_char,
        b"caseparampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_257: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_95);
    (*p_257).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_257, rules_104.as_mut_ptr());
    let mut rule_69: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh245 = (*rule_69).name;
    *fresh245 = b"nextpattern\0" as *const u8 as *const libc::c_char;
    let ref mut fresh246 = (*rule_69).sibling;
    *fresh246 = (*parent).child;
    let ref mut fresh247 = (*parent).child;
    *fresh247 = rule_69;
    let mut parent_96: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_69);
    static mut rules_105: [*const libc::c_char; 3] = [
        b"local\0" as *const u8 as *const libc::c_char,
        b"nextparampattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_258: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_96);
    (*p_258).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_258, rules_105.as_mut_ptr());
    let mut rule_70: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh248 = (*rule_70).name;
    *fresh248 = b"idseqmulti\0" as *const u8 as *const libc::c_char;
    let ref mut fresh249 = (*rule_70).sibling;
    *fresh249 = (*parent).child;
    let ref mut fresh250 = (*parent).child;
    *fresh250 = rule_70;
    let mut parent_97: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_70);
    (*rule_70).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_126: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_259: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_97);
    (*p_259).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_259, tokens_126.as_mut_ptr(), clean);
    static mut rules_106: [*const libc::c_char; 2] = [
        b"idseq_in_seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_260: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_97);
    (*p_260).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_260, rules_106.as_mut_ptr());
    let mut p_261: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_97);
    let mut parent_98: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_261);
    static mut tokens_127: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_262: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_98);
    (*p_262).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_262, tokens_127.as_mut_ptr(), clean);
    static mut rules_107: [*const libc::c_char; 2] = [
        b"idseq_in_seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_263: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_98);
    (*p_263).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_263, rules_107.as_mut_ptr());
    static mut tokens_128: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_264: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_97);
    (*p_264).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_264, tokens_128.as_mut_ptr(), clean);
    let mut rule_71: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh251 = (*rule_71).name;
    *fresh251 = b"idseqsingle\0" as *const u8 as *const libc::c_char;
    let ref mut fresh252 = (*rule_71).sibling;
    *fresh252 = (*parent).child;
    let ref mut fresh253 = (*parent).child;
    *fresh253 = rule_71;
    let mut parent_99: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_71);
    (*rule_71).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_129: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_265: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_99);
    (*p_265).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_265, tokens_129.as_mut_ptr(), clean);
    let mut rule_72: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh254 = (*rule_72).name;
    *fresh254 = b"idseq_in_seq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh255 = (*rule_72).sibling;
    *fresh255 = (*parent).child;
    let ref mut fresh256 = (*parent).child;
    *fresh256 = rule_72;
    let mut parent_100: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_72);
    static mut rules_108: [*const libc::c_char; 3] = [
        b"idseqsingle\0" as *const u8 as *const libc::c_char,
        b"idseqmulti\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_266: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_100);
    (*p_266).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_266, rules_108.as_mut_ptr());
    let mut rule_73: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh257 = (*rule_73).name;
    *fresh257 = b"idseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh258 = (*rule_73).sibling;
    *fresh258 = (*parent).child;
    let ref mut fresh259 = (*parent).child;
    *fresh259 = rule_73;
    let mut parent_101: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_73);
    static mut rules_109: [*const libc::c_char; 3] = [
        b"idseqsingle\0" as *const u8 as *const libc::c_char,
        b"idseqmulti\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_267: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_101);
    (*p_267).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_267, rules_109.as_mut_ptr());
    let mut rule_74: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh260 = (*rule_74).name;
    *fresh260 = b"elseclause\0" as *const u8 as *const libc::c_char;
    let ref mut fresh261 = (*rule_74).sibling;
    *fresh261 = (*parent).child;
    let ref mut fresh262 = (*parent).child;
    *fresh262 = rule_74;
    let mut parent_102: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_74);
    (*rule_74).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_130: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_268: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_102);
    (*p_268).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_268, tokens_130.as_mut_ptr(), clean);
    static mut rules_110: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_269: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_102);
    (*p_269).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_269, rules_110.as_mut_ptr());
    let mut rule_75: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh263 = (*rule_75).name;
    *fresh263 = b"elseif\0" as *const u8 as *const libc::c_char;
    let ref mut fresh264 = (*rule_75).sibling;
    *fresh264 = (*parent).child;
    let ref mut fresh265 = (*parent).child;
    *fresh265 = rule_75;
    let mut parent_103: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_75);
    static mut tokens_131: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut p_270: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_270).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_270, tokens_131.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_111: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_271: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_271).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_271, rules_111.as_mut_ptr());
    static mut rules_112: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_272: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_272).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_272, rules_112.as_mut_ptr());
    static mut tokens_132: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_273: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_273).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_273, tokens_132.as_mut_ptr(), clean);
    static mut rules_113: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_274: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_274).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_274, rules_113.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_114: [*const libc::c_char; 3] = [
        b"elseif\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_275: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_103);
    (*p_275).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_275, rules_114.as_mut_ptr());
    let mut rule_76: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh266 = (*rule_76).name;
    *fresh266 = b"cond\0" as *const u8 as *const libc::c_char;
    let ref mut fresh267 = (*rule_76).sibling;
    *fresh267 = (*parent).child;
    let ref mut fresh268 = (*parent).child;
    *fresh268 = rule_76;
    let mut parent_104: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_76);
    (*rule_76).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_133: [token_id; 2] = [TK_IF, TK_NONE];
    let mut p_276: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_276).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_276, tokens_133.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_115: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_277: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_277).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_277, rules_115.as_mut_ptr());
    static mut rules_116: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_278: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_278).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_278, rules_116.as_mut_ptr());
    static mut tokens_134: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_279: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_279).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_279, tokens_134.as_mut_ptr(), clean);
    static mut rules_117: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_280: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_280).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_280, rules_117.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_118: [*const libc::c_char; 3] = [
        b"elseif\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_281: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_281).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_281, rules_118.as_mut_ptr());
    static mut tokens_135: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_282: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_104);
    (*p_282).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_282, tokens_135.as_mut_ptr(), clean);
    let mut rule_77: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh269 = (*rule_77).name;
    *fresh269 = b"elseifdef\0" as *const u8 as *const libc::c_char;
    let ref mut fresh270 = (*rule_77).sibling;
    *fresh270 = (*parent).child;
    let ref mut fresh271 = (*parent).child;
    *fresh271 = rule_77;
    let mut parent_105: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_77);
    static mut tokens_136: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut p_283: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_283).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_283, tokens_136.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_119: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_284: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_284).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_284, rules_119.as_mut_ptr());
    static mut rules_120: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_285: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_285).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_285, rules_120.as_mut_ptr());
    let mut p_286: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_286).optional = 1 as libc::c_int != 0;
    let mut parent_106: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_286);
    static mut tokens_137: [token_id; 2] = [TK_TEST_EXTRA, TK_NONE];
    let mut p_287: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_106);
    (*p_287).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_287, tokens_137.as_mut_ptr(), clean);
    static mut rules_121: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_288: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_106);
    (*p_288).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_288, rules_121.as_mut_ptr());
    static mut tokens_138: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_289: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_289).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_289, tokens_138.as_mut_ptr(), clean);
    static mut rules_122: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_290: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_290).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_290, rules_122.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_123: [*const libc::c_char; 3] = [
        b"elseifdef\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_291: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_105);
    (*p_291).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_291, rules_123.as_mut_ptr());
    let mut rule_78: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh272 = (*rule_78).name;
    *fresh272 = b"ifdef\0" as *const u8 as *const libc::c_char;
    let ref mut fresh273 = (*rule_78).sibling;
    *fresh273 = (*parent).child;
    let ref mut fresh274 = (*parent).child;
    *fresh274 = rule_78;
    let mut parent_107: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_78);
    (*rule_78).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_139: [token_id; 2] = [TK_IFDEF, TK_NONE];
    let mut p_292: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_292).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_292, tokens_139.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_124: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_293: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_293).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_293, rules_124.as_mut_ptr());
    static mut rules_125: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_294: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_294).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_294, rules_125.as_mut_ptr());
    let mut p_295: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_295).optional = 1 as libc::c_int != 0;
    let mut parent_108: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_295);
    static mut tokens_140: [token_id; 2] = [TK_TEST_EXTRA, TK_NONE];
    let mut p_296: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_108);
    (*p_296).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_296, tokens_140.as_mut_ptr(), clean);
    static mut rules_126: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_297: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_108);
    (*p_297).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_297, rules_126.as_mut_ptr());
    static mut tokens_141: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_298: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_298).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_298, tokens_141.as_mut_ptr(), clean);
    static mut rules_127: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_299: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_299).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_299, rules_127.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_128: [*const libc::c_char; 3] = [
        b"elseifdef\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_300: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_300).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_300, rules_128.as_mut_ptr());
    static mut tokens_142: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_301: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_107);
    (*p_301).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_301, tokens_142.as_mut_ptr(), clean);
    let mut rule_79: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh275 = (*rule_79).name;
    *fresh275 = b"iftype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh276 = (*rule_79).sibling;
    *fresh276 = (*parent).child;
    let ref mut fresh277 = (*parent).child;
    *fresh277 = rule_79;
    let mut parent_109: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_79);
    static mut rules_129: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_302: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_109);
    (*p_302).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_302, rules_129.as_mut_ptr());
    static mut tokens_143: [token_id; 2] = [TK_SUBTYPE, TK_NONE];
    let mut p_303: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_109);
    (*p_303).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_303, tokens_143.as_mut_ptr(), clean);
    static mut rules_130: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_304: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_109);
    (*p_304).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_304, rules_130.as_mut_ptr());
    static mut tokens_144: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_305: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_109);
    (*p_305).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_305, tokens_144.as_mut_ptr(), clean);
    static mut rules_131: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_306: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_109);
    (*p_306).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_306, rules_131.as_mut_ptr());
    let mut rule_80: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh278 = (*rule_80).name;
    *fresh278 = b"elseiftype\0" as *const u8 as *const libc::c_char;
    let ref mut fresh279 = (*rule_80).sibling;
    *fresh279 = (*parent).child;
    let ref mut fresh280 = (*parent).child;
    *fresh280 = rule_80;
    let mut parent_110: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_80);
    static mut tokens_145: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut p_307: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_110);
    (*p_307).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_307, tokens_145.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_132: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_308: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_110);
    (*p_308).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_308, rules_132.as_mut_ptr());
    static mut rules_133: [*const libc::c_char; 2] = [
        b"iftype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_309: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_110);
    (*p_309).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_309, rules_133.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_134: [*const libc::c_char; 3] = [
        b"elseiftype\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_310: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_110);
    (*p_310).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_310, rules_134.as_mut_ptr());
    let mut rule_81: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh281 = (*rule_81).name;
    *fresh281 = b"iftypeset\0" as *const u8 as *const libc::c_char;
    let ref mut fresh282 = (*rule_81).sibling;
    *fresh282 = (*parent).child;
    let ref mut fresh283 = (*parent).child;
    *fresh283 = rule_81;
    let mut parent_111: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_81);
    (*rule_81).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_146: [token_id; 2] = [TK_IFTYPE_SET, TK_NONE];
    let mut p_311: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_111);
    (*p_311).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_311, tokens_146.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_135: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_312: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_111);
    (*p_312).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_312, rules_135.as_mut_ptr());
    static mut rules_136: [*const libc::c_char; 2] = [
        b"iftype\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_313: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_111);
    (*p_313).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_313, rules_136.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_137: [*const libc::c_char; 3] = [
        b"elseiftype\0" as *const u8 as *const libc::c_char,
        b"elseclause\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_314: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_111);
    (*p_314).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_314, rules_137.as_mut_ptr());
    static mut tokens_147: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_315: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_111);
    (*p_315).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_315, tokens_147.as_mut_ptr(), clean);
    let mut rule_82: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh284 = (*rule_82).name;
    *fresh284 = b"caseexpr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh285 = (*rule_82).sibling;
    *fresh285 = (*parent).child;
    let ref mut fresh286 = (*parent).child;
    *fresh286 = rule_82;
    let mut parent_112: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_82);
    static mut tokens_148: [token_id; 2] = [TK_PIPE, TK_NONE];
    let mut p_316: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_112);
    (*p_316).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_316, tokens_148.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_138: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_317: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_112);
    (*p_317).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_317, rules_138.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_139: [*const libc::c_char; 2] = [
        b"casepattern\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_318: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_112);
    (*p_318).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_318, rules_139.as_mut_ptr());
    let mut p_319: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_112);
    (*p_319).optional = 1 as libc::c_int != 0;
    let mut parent_113: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_319);
    static mut tokens_149: [token_id; 2] = [TK_IF, TK_NONE];
    let mut p_320: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_113);
    (*p_320).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_320, tokens_149.as_mut_ptr(), clean);
    static mut rules_140: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_321: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_113);
    (*p_321).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_321, rules_140.as_mut_ptr());
    let mut p_322: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_112);
    (*p_322).optional = 1 as libc::c_int != 0;
    let mut parent_114: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_322);
    static mut tokens_150: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut p_323: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_114);
    (*p_323).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_323, tokens_150.as_mut_ptr(), clean);
    static mut rules_141: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_324: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_114);
    (*p_324).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_324, rules_141.as_mut_ptr());
    let mut rule_83: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh287 = (*rule_83).name;
    *fresh287 = b"cases\0" as *const u8 as *const libc::c_char;
    let ref mut fresh288 = (*rule_83).sibling;
    *fresh288 = (*parent).child;
    let ref mut fresh289 = (*parent).child;
    *fresh289 = rule_83;
    let mut parent_115: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_83);
    (*rule_83).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_151: [*const libc::c_char; 2] = [
        b"caseexpr\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_325: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_115);
    p_325 = bnf_add(bnf_create(BNF_OR), p_325);
    bnf_rule_set(p_325, tokens_151.as_mut_ptr());
    let mut rule_84: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh290 = (*rule_84).name;
    *fresh290 = b"match\0" as *const u8 as *const libc::c_char;
    let ref mut fresh291 = (*rule_84).sibling;
    *fresh291 = (*parent).child;
    let ref mut fresh292 = (*parent).child;
    *fresh292 = rule_84;
    let mut parent_116: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_84);
    (*rule_84).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_152: [token_id; 2] = [TK_MATCH, TK_NONE];
    let mut p_326: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_326).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_326, tokens_152.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_142: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_327: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_327).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_327, rules_142.as_mut_ptr());
    static mut rules_143: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_328: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_328).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_328, rules_143.as_mut_ptr());
    static mut rules_144: [*const libc::c_char; 2] = [
        b"cases\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_329: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_329).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_329, rules_144.as_mut_ptr());
    let mut p_330: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_330).optional = 1 as libc::c_int != 0;
    let mut parent_117: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_330);
    static mut tokens_153: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_331: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_117);
    (*p_331).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_331, tokens_153.as_mut_ptr(), clean);
    static mut rules_145: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_332: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_117);
    (*p_332).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_332, rules_145.as_mut_ptr());
    static mut tokens_154: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_333: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_116);
    (*p_333).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_333, tokens_154.as_mut_ptr(), clean);
    let mut rule_85: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh293 = (*rule_85).name;
    *fresh293 = b"whileloop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh294 = (*rule_85).sibling;
    *fresh294 = (*parent).child;
    let ref mut fresh295 = (*parent).child;
    *fresh295 = rule_85;
    let mut parent_118: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_85);
    (*rule_85).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_155: [token_id; 2] = [TK_WHILE, TK_NONE];
    let mut p_334: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_334).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_334, tokens_155.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_146: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_335: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_335).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_335, rules_146.as_mut_ptr());
    static mut rules_147: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_336: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_336).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_336, rules_147.as_mut_ptr());
    static mut tokens_156: [token_id; 2] = [TK_DO, TK_NONE];
    let mut p_337: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_337).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_337, tokens_156.as_mut_ptr(), clean);
    static mut rules_148: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_338: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_338).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_338, rules_148.as_mut_ptr());
    let mut p_339: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_339).optional = 1 as libc::c_int != 0;
    let mut parent_119: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_339);
    static mut tokens_157: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_340: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_119);
    (*p_340).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_340, tokens_157.as_mut_ptr(), clean);
    static mut rules_149: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_341: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_119);
    (*p_341).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_341, rules_149.as_mut_ptr());
    static mut tokens_158: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_342: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_118);
    (*p_342).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_342, tokens_158.as_mut_ptr(), clean);
    let mut rule_86: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh296 = (*rule_86).name;
    *fresh296 = b"repeat\0" as *const u8 as *const libc::c_char;
    let ref mut fresh297 = (*rule_86).sibling;
    *fresh297 = (*parent).child;
    let ref mut fresh298 = (*parent).child;
    *fresh298 = rule_86;
    let mut parent_120: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_86);
    (*rule_86).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_159: [token_id; 2] = [TK_REPEAT, TK_NONE];
    let mut p_343: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_343).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_343, tokens_159.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_150: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_344: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_344).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_344, rules_150.as_mut_ptr());
    static mut rules_151: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_345: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_345).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_345, rules_151.as_mut_ptr());
    static mut tokens_160: [token_id; 2] = [TK_UNTIL, TK_NONE];
    let mut p_346: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_346).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_346, tokens_160.as_mut_ptr(), clean);
    static mut rules_152: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_347: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_347).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_347, rules_152.as_mut_ptr());
    let mut p_348: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_348).optional = 1 as libc::c_int != 0;
    let mut parent_121: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_348);
    static mut tokens_161: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_349: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_121);
    (*p_349).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_349, tokens_161.as_mut_ptr(), clean);
    static mut rules_153: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_350: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_121);
    (*p_350).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_350, rules_153.as_mut_ptr());
    static mut tokens_162: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_351: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_120);
    (*p_351).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_351, tokens_162.as_mut_ptr(), clean);
    let mut rule_87: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh299 = (*rule_87).name;
    *fresh299 = b"forloop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh300 = (*rule_87).sibling;
    *fresh300 = (*parent).child;
    let ref mut fresh301 = (*parent).child;
    *fresh301 = rule_87;
    let mut parent_122: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_87);
    (*rule_87).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_163: [token_id; 2] = [TK_FOR, TK_NONE];
    let mut p_352: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_352).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_352, tokens_163.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_154: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_353: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_353).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_353, rules_154.as_mut_ptr());
    static mut rules_155: [*const libc::c_char; 2] = [
        b"idseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_354: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_354).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_354, rules_155.as_mut_ptr());
    static mut tokens_164: [token_id; 2] = [TK_IN, TK_NONE];
    let mut p_355: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_355).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_355, tokens_164.as_mut_ptr(), clean);
    static mut rules_156: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_356: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_356).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_356, rules_156.as_mut_ptr());
    static mut tokens_165: [token_id; 2] = [TK_DO, TK_NONE];
    let mut p_357: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_357).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_357, tokens_165.as_mut_ptr(), clean);
    static mut rules_157: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_358: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_358).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_358, rules_157.as_mut_ptr());
    let mut p_359: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_359).optional = 1 as libc::c_int != 0;
    let mut parent_123: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_359);
    static mut tokens_166: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_360: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_123);
    (*p_360).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_360, tokens_166.as_mut_ptr(), clean);
    static mut rules_158: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_361: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_123);
    (*p_361).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_361, rules_158.as_mut_ptr());
    static mut tokens_167: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_362: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_122);
    (*p_362).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_362, tokens_167.as_mut_ptr(), clean);
    let mut rule_88: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh302 = (*rule_88).name;
    *fresh302 = b"withelem\0" as *const u8 as *const libc::c_char;
    let ref mut fresh303 = (*rule_88).sibling;
    *fresh303 = (*parent).child;
    let ref mut fresh304 = (*parent).child;
    *fresh304 = rule_88;
    let mut parent_124: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_88);
    static mut rules_159: [*const libc::c_char; 2] = [
        b"idseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_363: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_124);
    (*p_363).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_363, rules_159.as_mut_ptr());
    static mut tokens_168: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_364: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_124);
    (*p_364).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_364, tokens_168.as_mut_ptr(), clean);
    static mut rules_160: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_365: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_124);
    (*p_365).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_365, rules_160.as_mut_ptr());
    let mut rule_89: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh305 = (*rule_89).name;
    *fresh305 = b"withexpr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh306 = (*rule_89).sibling;
    *fresh306 = (*parent).child;
    let ref mut fresh307 = (*parent).child;
    *fresh307 = rule_89;
    let mut parent_125: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_89);
    (*rule_89).inline_rule = 1 as libc::c_int != 0;
    static mut rules_161: [*const libc::c_char; 2] = [
        b"withelem\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_366: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_125);
    (*p_366).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_366, rules_161.as_mut_ptr());
    let mut p_367: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_125);
    let mut parent_126: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_367);
    static mut tokens_169: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut p_368: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_126);
    (*p_368).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_368, tokens_169.as_mut_ptr(), clean);
    static mut rules_162: [*const libc::c_char; 2] = [
        b"withelem\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_369: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_126);
    (*p_369).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_369, rules_162.as_mut_ptr());
    let mut rule_90: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh308 = (*rule_90).name;
    *fresh308 = b"with\0" as *const u8 as *const libc::c_char;
    let ref mut fresh309 = (*rule_90).sibling;
    *fresh309 = (*parent).child;
    let ref mut fresh310 = (*parent).child;
    *fresh310 = rule_90;
    let mut parent_127: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_90);
    (*rule_90).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_170: [token_id; 2] = [TK_WITH, TK_NONE];
    let mut p_370: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_370).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_370, tokens_170.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_163: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_371: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_371).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_371, rules_163.as_mut_ptr());
    static mut rules_164: [*const libc::c_char; 2] = [
        b"withexpr\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_372: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_372).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_372, rules_164.as_mut_ptr());
    static mut tokens_171: [token_id; 2] = [TK_DO, TK_NONE];
    let mut p_373: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_373).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_373, tokens_171.as_mut_ptr(), clean);
    static mut rules_165: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_374: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_374).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_374, rules_165.as_mut_ptr());
    static mut tokens_172: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_375: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_127);
    (*p_375).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_375, tokens_172.as_mut_ptr(), clean);
    let mut rule_91: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh311 = (*rule_91).name;
    *fresh311 = b"try_block\0" as *const u8 as *const libc::c_char;
    let ref mut fresh312 = (*rule_91).sibling;
    *fresh312 = (*parent).child;
    let ref mut fresh313 = (*parent).child;
    *fresh313 = rule_91;
    let mut parent_128: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_91);
    (*rule_91).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_173: [token_id; 2] = [TK_TRY, TK_NONE];
    let mut p_376: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_376).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_376, tokens_173.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_166: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_377: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_377).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_377, rules_166.as_mut_ptr());
    static mut rules_167: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_378: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_378).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_378, rules_167.as_mut_ptr());
    let mut p_379: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_379).optional = 1 as libc::c_int != 0;
    let mut parent_129: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_379);
    static mut tokens_174: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_380: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_129);
    (*p_380).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_380, tokens_174.as_mut_ptr(), clean);
    static mut rules_168: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_381: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_129);
    (*p_381).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_381, rules_168.as_mut_ptr());
    let mut p_382: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_382).optional = 1 as libc::c_int != 0;
    let mut parent_130: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_382);
    static mut tokens_175: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_383: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_130);
    (*p_383).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_383, tokens_175.as_mut_ptr(), clean);
    static mut rules_169: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_384: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_130);
    (*p_384).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_384, rules_169.as_mut_ptr());
    static mut tokens_176: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_385: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_128);
    (*p_385).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_385, tokens_176.as_mut_ptr(), clean);
    let mut rule_92: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh314 = (*rule_92).name;
    *fresh314 = b"test_try_block\0" as *const u8 as *const libc::c_char;
    let ref mut fresh315 = (*rule_92).sibling;
    *fresh315 = (*parent).child;
    let ref mut fresh316 = (*parent).child;
    *fresh316 = rule_92;
    let mut parent_131: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_92);
    (*rule_92).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_177: [token_id; 2] = [TK_TEST_TRY_NO_CHECK, TK_NONE];
    let mut p_386: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_386).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_386, tokens_177.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_170: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_387: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_387).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_387, rules_170.as_mut_ptr());
    static mut rules_171: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_388: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_388).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_388, rules_171.as_mut_ptr());
    let mut p_389: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_389).optional = 1 as libc::c_int != 0;
    let mut parent_132: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_389);
    static mut tokens_178: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut p_390: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_132);
    (*p_390).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_390, tokens_178.as_mut_ptr(), clean);
    static mut rules_172: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_391: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_132);
    (*p_391).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_391, rules_172.as_mut_ptr());
    let mut p_392: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_392).optional = 1 as libc::c_int != 0;
    let mut parent_133: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_392);
    static mut tokens_179: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut p_393: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_133);
    (*p_393).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_393, tokens_179.as_mut_ptr(), clean);
    static mut rules_173: [*const libc::c_char; 2] = [
        b"annotatedseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_394: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_133);
    (*p_394).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_394, rules_173.as_mut_ptr());
    static mut tokens_180: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_395: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_131);
    (*p_395).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_395, tokens_180.as_mut_ptr(), clean);
    let mut rule_93: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh317 = (*rule_93).name;
    *fresh317 = b"recover\0" as *const u8 as *const libc::c_char;
    let ref mut fresh318 = (*rule_93).sibling;
    *fresh318 = (*parent).child;
    let ref mut fresh319 = (*parent).child;
    *fresh319 = rule_93;
    let mut parent_134: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_93);
    (*rule_93).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_181: [token_id; 2] = [TK_RECOVER, TK_NONE];
    let mut p_396: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_134);
    (*p_396).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_396, tokens_181.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_174: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_397: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_134);
    (*p_397).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_397, rules_174.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_175: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_398: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_134);
    (*p_398).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_398, rules_175.as_mut_ptr());
    static mut rules_176: [*const libc::c_char; 2] = [
        b"seq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_399: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_134);
    (*p_399).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_399, rules_176.as_mut_ptr());
    static mut tokens_182: [token_id; 2] = [TK_END, TK_NONE];
    let mut p_400: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_134);
    (*p_400).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_400, tokens_182.as_mut_ptr(), clean);
    let mut rule_94: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh320 = (*rule_94).name;
    *fresh320 = b"test_aliased\0" as *const u8 as *const libc::c_char;
    let ref mut fresh321 = (*rule_94).sibling;
    *fresh321 = (*parent).child;
    let ref mut fresh322 = (*parent).child;
    *fresh322 = rule_94;
    let mut parent_135: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_94);
    (*rule_94).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_183: [token_id; 2] = [TK_TEST_ALIASED, TK_NONE];
    let mut p_401: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_135);
    (*p_401).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_401, tokens_183.as_mut_ptr(), clean);
    let mut rule_95: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh323 = (*rule_95).name;
    *fresh323 = b"consume\0" as *const u8 as *const libc::c_char;
    let ref mut fresh324 = (*rule_95).sibling;
    *fresh324 = (*parent).child;
    let ref mut fresh325 = (*parent).child;
    *fresh325 = rule_95;
    let mut parent_136: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_95);
    (*rule_95).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_184: [token_id; 2] = [TK_CONSUME, TK_NONE];
    let mut p_402: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_136);
    (*p_402).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_402, tokens_184.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_177: [*const libc::c_char; 3] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        b"test_aliased\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_403: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_136);
    (*p_403).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_403, rules_177.as_mut_ptr());
    static mut rules_178: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_404: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_136);
    (*p_404).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_404, rules_178.as_mut_ptr());
    let mut rule_96: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh326 = (*rule_96).name;
    *fresh326 = b"test_prefix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh327 = (*rule_96).sibling;
    *fresh327 = (*parent).child;
    let ref mut fresh328 = (*parent).child;
    *fresh328 = rule_96;
    let mut parent_137: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_96);
    (*rule_96).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_185: [token_id; 2] = [TK_IFDEFNOT, TK_NONE];
    let mut p_405: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_137);
    (*p_405).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_405, tokens_185.as_mut_ptr(), clean);
    static mut rules_179: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_406: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_137);
    (*p_406).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_406, rules_179.as_mut_ptr());
    let mut rule_97: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh329 = (*rule_97).name;
    *fresh329 = b"test_noseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh330 = (*rule_97).sibling;
    *fresh330 = (*parent).child;
    let ref mut fresh331 = (*parent).child;
    *fresh331 = rule_97;
    let mut parent_138: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_97);
    (*rule_97).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_186: [token_id; 2] = [TK_TEST_NO_SEQ, TK_NONE];
    let mut p_407: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_138);
    (*p_407).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_407, tokens_186.as_mut_ptr(), clean);
    static mut tokens_187: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut p_408: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_138);
    (*p_408).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_408, tokens_187.as_mut_ptr(), clean);
    static mut rules_180: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_409: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_138);
    (*p_409).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_409, rules_180.as_mut_ptr());
    static mut tokens_188: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_410: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_138);
    (*p_410).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_410, tokens_188.as_mut_ptr(), clean);
    let mut rule_98: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh332 = (*rule_98).name;
    *fresh332 = b"test_seq_scope\0" as *const u8 as *const libc::c_char;
    let ref mut fresh333 = (*rule_98).sibling;
    *fresh333 = (*parent).child;
    let ref mut fresh334 = (*parent).child;
    *fresh334 = rule_98;
    let mut parent_139: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_98);
    (*rule_98).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_189: [token_id; 2] = [TK_TEST_SEQ_SCOPE, TK_NONE];
    let mut p_411: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_139);
    (*p_411).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_411, tokens_189.as_mut_ptr(), clean);
    static mut tokens_190: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut p_412: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_139);
    (*p_412).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_412, tokens_190.as_mut_ptr(), clean);
    static mut rules_181: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_413: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_139);
    (*p_413).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_413, rules_181.as_mut_ptr());
    static mut tokens_191: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_414: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_139);
    (*p_414).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_414, tokens_191.as_mut_ptr(), clean);
    let mut rule_99: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh335 = (*rule_99).name;
    *fresh335 = b"test_ifdef_flag\0" as *const u8 as *const libc::c_char;
    let ref mut fresh336 = (*rule_99).sibling;
    *fresh336 = (*parent).child;
    let ref mut fresh337 = (*parent).child;
    *fresh337 = rule_99;
    let mut parent_140: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_99);
    (*rule_99).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_192: [token_id; 2] = [TK_IFDEFFLAG, TK_NONE];
    let mut p_415: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_140);
    (*p_415).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_415, tokens_192.as_mut_ptr(), clean);
    static mut tokens_193: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_416: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_140);
    (*p_416).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_416, tokens_193.as_mut_ptr(), clean);
    let mut rule_100: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh338 = (*rule_100).name;
    *fresh338 = b"term\0" as *const u8 as *const libc::c_char;
    let ref mut fresh339 = (*rule_100).sibling;
    *fresh339 = (*parent).child;
    let ref mut fresh340 = (*parent).child;
    *fresh340 = rule_100;
    let mut parent_141: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_100);
    static mut rules_182: [*const libc::c_char; 19] = [
        b"cond\0" as *const u8 as *const libc::c_char,
        b"ifdef\0" as *const u8 as *const libc::c_char,
        b"iftypeset\0" as *const u8 as *const libc::c_char,
        b"match\0" as *const u8 as *const libc::c_char,
        b"whileloop\0" as *const u8 as *const libc::c_char,
        b"repeat\0" as *const u8 as *const libc::c_char,
        b"forloop\0" as *const u8 as *const libc::c_char,
        b"with\0" as *const u8 as *const libc::c_char,
        b"try_block\0" as *const u8 as *const libc::c_char,
        b"recover\0" as *const u8 as *const libc::c_char,
        b"consume\0" as *const u8 as *const libc::c_char,
        b"pattern\0" as *const u8 as *const libc::c_char,
        b"const_expr\0" as *const u8 as *const libc::c_char,
        b"test_noseq\0" as *const u8 as *const libc::c_char,
        b"test_seq_scope\0" as *const u8 as *const libc::c_char,
        b"test_try_block\0" as *const u8 as *const libc::c_char,
        b"test_ifdef_flag\0" as *const u8 as *const libc::c_char,
        b"test_prefix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_417: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_141);
    (*p_417).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_417, rules_182.as_mut_ptr());
    let mut rule_101: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh341 = (*rule_101).name;
    *fresh341 = b"nextterm\0" as *const u8 as *const libc::c_char;
    let ref mut fresh342 = (*rule_101).sibling;
    *fresh342 = (*parent).child;
    let ref mut fresh343 = (*parent).child;
    *fresh343 = rule_101;
    let mut parent_142: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_101);
    static mut rules_183: [*const libc::c_char; 19] = [
        b"cond\0" as *const u8 as *const libc::c_char,
        b"ifdef\0" as *const u8 as *const libc::c_char,
        b"iftypeset\0" as *const u8 as *const libc::c_char,
        b"match\0" as *const u8 as *const libc::c_char,
        b"whileloop\0" as *const u8 as *const libc::c_char,
        b"repeat\0" as *const u8 as *const libc::c_char,
        b"forloop\0" as *const u8 as *const libc::c_char,
        b"with\0" as *const u8 as *const libc::c_char,
        b"try_block\0" as *const u8 as *const libc::c_char,
        b"recover\0" as *const u8 as *const libc::c_char,
        b"consume\0" as *const u8 as *const libc::c_char,
        b"nextpattern\0" as *const u8 as *const libc::c_char,
        b"const_expr\0" as *const u8 as *const libc::c_char,
        b"test_noseq\0" as *const u8 as *const libc::c_char,
        b"test_seq_scope\0" as *const u8 as *const libc::c_char,
        b"test_try_block\0" as *const u8 as *const libc::c_char,
        b"test_ifdef_flag\0" as *const u8 as *const libc::c_char,
        b"test_prefix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_418: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_142);
    (*p_418).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_418, rules_183.as_mut_ptr());
    let mut rule_102: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh344 = (*rule_102).name;
    *fresh344 = b"asop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh345 = (*rule_102).sibling;
    *fresh345 = (*parent).child;
    let ref mut fresh346 = (*parent).child;
    *fresh346 = rule_102;
    let mut parent_143: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_102);
    (*rule_102).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_194: [token_id; 2] = [TK_AS, TK_NONE];
    let mut p_419: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_143);
    (*p_419).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_419, tokens_194.as_mut_ptr(), clean);
    static mut rules_184: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_420: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_143);
    (*p_420).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_420, rules_184.as_mut_ptr());
    let mut rule_103: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh347 = (*rule_103).name;
    *fresh347 = b"binop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh348 = (*rule_103).sibling;
    *fresh348 = (*parent).child;
    let ref mut fresh349 = (*parent).child;
    *fresh349 = rule_103;
    let mut parent_144: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_103);
    static mut tokens_195: [token_id; 32] = [
        TK_AND,
        TK_OR,
        TK_XOR,
        TK_PLUS,
        TK_MINUS,
        TK_MULTIPLY,
        TK_DIVIDE,
        TK_REM,
        TK_MOD,
        TK_PLUS_TILDE,
        TK_MINUS_TILDE,
        TK_MULTIPLY_TILDE,
        TK_DIVIDE_TILDE,
        TK_REM_TILDE,
        TK_MOD_TILDE,
        TK_LSHIFT,
        TK_RSHIFT,
        TK_LSHIFT_TILDE,
        TK_RSHIFT_TILDE,
        TK_EQ,
        TK_NE,
        TK_LT,
        TK_LE,
        TK_GE,
        TK_GT,
        TK_EQ_TILDE,
        TK_NE_TILDE,
        TK_LT_TILDE,
        TK_LE_TILDE,
        TK_GE_TILDE,
        TK_GT_TILDE,
        TK_NONE,
    ];
    let mut p_421: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_144);
    (*p_421).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_421, tokens_195.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut tokens_196: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_422: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_144);
    (*p_422).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_422, tokens_196.as_mut_ptr(), clean);
    static mut rules_185: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_423: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_144);
    (*p_423).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_423, rules_185.as_mut_ptr());
    let mut rule_104: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh350 = (*rule_104).name;
    *fresh350 = b"isop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh351 = (*rule_104).sibling;
    *fresh351 = (*parent).child;
    let ref mut fresh352 = (*parent).child;
    *fresh352 = rule_104;
    let mut parent_145: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_104);
    static mut tokens_197: [token_id; 3] = [TK_IS, TK_ISNT, TK_NONE];
    let mut p_424: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_145);
    (*p_424).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_424, tokens_197.as_mut_ptr(), clean);
    static mut rules_186: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_425: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_145);
    (*p_425).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_425, rules_186.as_mut_ptr());
    let mut rule_105: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh353 = (*rule_105).name;
    *fresh353 = b"test_binop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh354 = (*rule_105).sibling;
    *fresh354 = (*parent).child;
    let ref mut fresh355 = (*parent).child;
    *fresh355 = rule_105;
    let mut parent_146: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_105);
    static mut tokens_198: [token_id; 3] = [TK_IFDEFAND, TK_IFDEFOR, TK_NONE];
    let mut p_426: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_146);
    (*p_426).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_426, tokens_198.as_mut_ptr(), clean);
    static mut rules_187: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_427: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_146);
    (*p_427).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_427, rules_187.as_mut_ptr());
    let mut rule_106: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh356 = (*rule_106).name;
    *fresh356 = b"infix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh357 = (*rule_106).sibling;
    *fresh357 = (*parent).child;
    let ref mut fresh358 = (*parent).child;
    *fresh358 = rule_106;
    let mut parent_147: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_106);
    static mut rules_188: [*const libc::c_char; 2] = [
        b"term\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_428: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_147);
    (*p_428).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_428, rules_188.as_mut_ptr());
    static mut tokens_199: [*const libc::c_char; 5] = [
        b"binop\0" as *const u8 as *const libc::c_char,
        b"isop\0" as *const u8 as *const libc::c_char,
        b"asop\0" as *const u8 as *const libc::c_char,
        b"test_binop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_429: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_147);
    p_429 = bnf_add(bnf_create(BNF_OR), p_429);
    bnf_rule_set(p_429, tokens_199.as_mut_ptr());
    let mut rule_107: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh359 = (*rule_107).name;
    *fresh359 = b"nextinfix\0" as *const u8 as *const libc::c_char;
    let ref mut fresh360 = (*rule_107).sibling;
    *fresh360 = (*parent).child;
    let ref mut fresh361 = (*parent).child;
    *fresh361 = rule_107;
    let mut parent_148: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_107);
    static mut rules_189: [*const libc::c_char; 2] = [
        b"nextterm\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_430: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_148);
    (*p_430).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_430, rules_189.as_mut_ptr());
    static mut tokens_200: [*const libc::c_char; 5] = [
        b"binop\0" as *const u8 as *const libc::c_char,
        b"isop\0" as *const u8 as *const libc::c_char,
        b"asop\0" as *const u8 as *const libc::c_char,
        b"test_binop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_431: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_148);
    p_431 = bnf_add(bnf_create(BNF_OR), p_431);
    bnf_rule_set(p_431, tokens_200.as_mut_ptr());
    let mut rule_108: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh362 = (*rule_108).name;
    *fresh362 = b"assignop\0" as *const u8 as *const libc::c_char;
    let ref mut fresh363 = (*rule_108).sibling;
    *fresh363 = (*parent).child;
    let ref mut fresh364 = (*parent).child;
    *fresh364 = rule_108;
    let mut parent_149: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_108);
    (*rule_108).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_201: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_432: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_149);
    (*p_432).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_432, tokens_201.as_mut_ptr(), clean);
    static mut rules_190: [*const libc::c_char; 2] = [
        b"assignment\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_433: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_149);
    (*p_433).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_433, rules_190.as_mut_ptr());
    let mut rule_109: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh365 = (*rule_109).name;
    *fresh365 = b"assignment\0" as *const u8 as *const libc::c_char;
    let ref mut fresh366 = (*rule_109).sibling;
    *fresh366 = (*parent).child;
    let ref mut fresh367 = (*parent).child;
    *fresh367 = rule_109;
    let mut parent_150: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_109);
    static mut rules_191: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_434: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_150);
    (*p_434).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_434, rules_191.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_192: [*const libc::c_char; 2] = [
        b"assignop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_435: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_150);
    (*p_435).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_435, rules_192.as_mut_ptr());
    let mut rule_110: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh368 = (*rule_110).name;
    *fresh368 = b"nextassignment\0" as *const u8 as *const libc::c_char;
    let ref mut fresh369 = (*rule_110).sibling;
    *fresh369 = (*parent).child;
    let ref mut fresh370 = (*parent).child;
    *fresh370 = rule_110;
    let mut parent_151: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_110);
    static mut rules_193: [*const libc::c_char; 2] = [
        b"nextinfix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_436: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_151);
    (*p_436).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_436, rules_193.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_194: [*const libc::c_char; 2] = [
        b"assignop\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_437: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_151);
    (*p_437).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_437, rules_194.as_mut_ptr());
    let mut rule_111: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh371 = (*rule_111).name;
    *fresh371 = b"jump\0" as *const u8 as *const libc::c_char;
    let ref mut fresh372 = (*rule_111).sibling;
    *fresh372 = (*parent).child;
    let ref mut fresh373 = (*parent).child;
    *fresh373 = rule_111;
    let mut parent_152: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_111);
    static mut tokens_202: [token_id; 7] = [
        TK_RETURN,
        TK_BREAK,
        TK_CONTINUE,
        TK_ERROR,
        TK_COMPILE_INTRINSIC,
        TK_COMPILE_ERROR,
        TK_NONE,
    ];
    let mut p_438: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_152);
    (*p_438).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_438, tokens_202.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_195: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_439: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_152);
    (*p_439).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_439, rules_195.as_mut_ptr());
    let mut rule_112: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh374 = (*rule_112).name;
    *fresh374 = b"semi\0" as *const u8 as *const libc::c_char;
    let ref mut fresh375 = (*rule_112).sibling;
    *fresh375 = (*parent).child;
    let ref mut fresh376 = (*parent).child;
    *fresh376 = rule_112;
    let mut parent_153: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_112);
    let mut p_440: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_153);
    let mut parent_154: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_440);
    static mut tokens_203: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    let mut p_441: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_154);
    (*p_441).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_441, tokens_203.as_mut_ptr(), clean);
    parent_154 = bnf_add(bnf_create(BNF_SEQ), p_440);
    static mut tokens_204: [token_id; 2] = [TK_SEMI, TK_NONE];
    let mut p_442: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_153);
    (*p_442).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_442, tokens_204.as_mut_ptr(), clean);
    let mut p_443: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_153);
    (*p_443).optional = 1 as libc::c_int != 0;
    let mut parent_155: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_443);
    static mut tokens_205: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    let mut p_444: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_155);
    (*p_444).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_444, tokens_205.as_mut_ptr(), clean);
    let mut rule_113: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh377 = (*rule_113).name;
    *fresh377 = b"semiexpr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh378 = (*rule_113).sibling;
    *fresh378 = (*parent).child;
    let ref mut fresh379 = (*parent).child;
    *fresh379 = rule_113;
    let mut parent_156: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_113);
    static mut rules_196: [*const libc::c_char; 2] = [
        b"semi\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_445: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_156);
    (*p_445).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_445, rules_196.as_mut_ptr());
    static mut rules_197: [*const libc::c_char; 3] = [
        b"exprseq\0" as *const u8 as *const libc::c_char,
        b"jump\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_446: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_156);
    (*p_446).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_446, rules_197.as_mut_ptr());
    let mut rule_114: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh380 = (*rule_114).name;
    *fresh380 = b"nosemi\0" as *const u8 as *const libc::c_char;
    let ref mut fresh381 = (*rule_114).sibling;
    *fresh381 = (*parent).child;
    let ref mut fresh382 = (*parent).child;
    *fresh382 = rule_114;
    let mut parent_157: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_114);
    let mut p_447: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_157);
    let mut parent_158: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_447);
    static mut tokens_206: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    let mut p_448: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_158);
    (*p_448).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_448, tokens_206.as_mut_ptr(), clean);
    parent_158 = bnf_add(bnf_create(BNF_SEQ), p_447);
    static mut rules_198: [*const libc::c_char; 3] = [
        b"nextexprseq\0" as *const u8 as *const libc::c_char,
        b"jump\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_449: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_157);
    (*p_449).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_449, rules_198.as_mut_ptr());
    let mut rule_115: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh383 = (*rule_115).name;
    *fresh383 = b"nextexprseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh384 = (*rule_115).sibling;
    *fresh384 = (*parent).child;
    let ref mut fresh385 = (*parent).child;
    *fresh385 = rule_115;
    let mut parent_159: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_115);
    static mut rules_199: [*const libc::c_char; 2] = [
        b"nextassignment\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_450: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_159);
    (*p_450).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_450, rules_199.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_200: [*const libc::c_char; 3] = [
        b"semiexpr\0" as *const u8 as *const libc::c_char,
        b"nosemi\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_451: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_159);
    (*p_451).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_451, rules_200.as_mut_ptr());
    let mut rule_116: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh386 = (*rule_116).name;
    *fresh386 = b"exprseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh387 = (*rule_116).sibling;
    *fresh387 = (*parent).child;
    let ref mut fresh388 = (*parent).child;
    *fresh388 = rule_116;
    let mut parent_160: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_116);
    static mut rules_201: [*const libc::c_char; 2] = [
        b"assignment\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_452: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_160);
    (*p_452).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_452, rules_201.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_202: [*const libc::c_char; 3] = [
        b"semiexpr\0" as *const u8 as *const libc::c_char,
        b"nosemi\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_453: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_160);
    (*p_453).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_453, rules_202.as_mut_ptr());
    let mut rule_117: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh389 = (*rule_117).name;
    *fresh389 = b"rawseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh390 = (*rule_117).sibling;
    *fresh390 = (*parent).child;
    let ref mut fresh391 = (*parent).child;
    *fresh391 = rule_117;
    let mut parent_161: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_117);
    static mut rules_203: [*const libc::c_char; 3] = [
        b"exprseq\0" as *const u8 as *const libc::c_char,
        b"jump\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_454: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_161);
    (*p_454).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_454, rules_203.as_mut_ptr());
    let mut rule_118: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh392 = (*rule_118).name;
    *fresh392 = b"seq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh393 = (*rule_118).sibling;
    *fresh393 = (*parent).child;
    let ref mut fresh394 = (*parent).child;
    *fresh394 = rule_118;
    let mut parent_162: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_118);
    static mut rules_204: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_455: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_162);
    (*p_455).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_455, rules_204.as_mut_ptr());
    let mut rule_119: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh395 = (*rule_119).name;
    *fresh395 = b"annotatedrawseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh396 = (*rule_119).sibling;
    *fresh396 = (*parent).child;
    let ref mut fresh397 = (*parent).child;
    *fresh397 = rule_119;
    let mut parent_163: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_119);
    optional = 1 as libc::c_int != 0;
    static mut rules_205: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_456: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_163);
    (*p_456).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_456, rules_205.as_mut_ptr());
    static mut rules_206: [*const libc::c_char; 3] = [
        b"exprseq\0" as *const u8 as *const libc::c_char,
        b"jump\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_457: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_163);
    (*p_457).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_457, rules_206.as_mut_ptr());
    let mut rule_120: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh398 = (*rule_120).name;
    *fresh398 = b"annotatedseq\0" as *const u8 as *const libc::c_char;
    let ref mut fresh399 = (*rule_120).sibling;
    *fresh399 = (*parent).child;
    let ref mut fresh400 = (*parent).child;
    *fresh400 = rule_120;
    let mut parent_164: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_120);
    static mut rules_207: [*const libc::c_char; 2] = [
        b"annotatedrawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_458: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_164);
    (*p_458).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_458, rules_207.as_mut_ptr());
    let mut rule_121: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh401 = (*rule_121).name;
    *fresh401 = b"method\0" as *const u8 as *const libc::c_char;
    let ref mut fresh402 = (*rule_121).sibling;
    *fresh402 = (*parent).child;
    let ref mut fresh403 = (*parent).child;
    *fresh403 = rule_121;
    let mut parent_165: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_121);
    static mut tokens_207: [token_id; 4] = [TK_FUN, TK_BE, TK_NEW, TK_NONE];
    let mut p_459: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_459).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_459, tokens_207.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_208: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_460: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_460).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_460, rules_208.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut rules_209: [*const libc::c_char; 3] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        b"bare\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_461: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_461).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_461, rules_209.as_mut_ptr());
    static mut tokens_208: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_462: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_462).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_462, tokens_208.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_210: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_463: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_463).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_463, rules_210.as_mut_ptr());
    static mut tokens_209: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_464: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_464).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_464, tokens_209.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_211: [*const libc::c_char; 2] = [
        b"params\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_465: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_465).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_465, rules_211.as_mut_ptr());
    static mut tokens_210: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_466: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_466).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_466, tokens_210.as_mut_ptr(), clean);
    let mut p_467: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_467).optional = 1 as libc::c_int != 0;
    let mut parent_166: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_467);
    static mut tokens_211: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_468: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_166);
    (*p_468).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_468, tokens_211.as_mut_ptr(), clean);
    static mut rules_212: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_469: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_166);
    (*p_469).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_469, rules_212.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_212: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_470: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_470).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_470, tokens_212.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut tokens_213: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut p_471: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_471).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_471, tokens_213.as_mut_ptr(), clean);
    let mut p_472: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_165);
    (*p_472).optional = 1 as libc::c_int != 0;
    let mut parent_167: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_472);
    static mut tokens_214: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut p_473: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_167);
    (*p_473).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_473, tokens_214.as_mut_ptr(), clean);
    static mut rules_213: [*const libc::c_char; 2] = [
        b"rawseq\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_474: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_167);
    (*p_474).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_474, rules_213.as_mut_ptr());
    let mut rule_122: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh404 = (*rule_122).name;
    *fresh404 = b"field\0" as *const u8 as *const libc::c_char;
    let ref mut fresh405 = (*rule_122).sibling;
    *fresh405 = (*parent).child;
    let ref mut fresh406 = (*parent).child;
    *fresh406 = rule_122;
    let mut parent_168: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_122);
    static mut tokens_215: [token_id; 4] = [TK_VAR, TK_LET, TK_EMBED, TK_NONE];
    let mut p_475: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_475).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_475, tokens_215.as_mut_ptr(), clean);
    static mut tokens_216: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_476: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_476).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_476, tokens_216.as_mut_ptr(), clean);
    static mut tokens_217: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut p_477: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_477).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_477, tokens_217.as_mut_ptr(), clean);
    static mut rules_214: [*const libc::c_char; 2] = [
        b"type\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_478: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_478).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_478, rules_214.as_mut_ptr());
    let mut p_479: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_479).optional = 1 as libc::c_int != 0;
    let mut parent_169: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_479);
    static mut tokens_218: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_480: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_169);
    (*p_480).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_480, tokens_218.as_mut_ptr(), clean);
    static mut rules_215: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_481: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_169);
    (*p_481).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_481, rules_215.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_219: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut p_482: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_168);
    (*p_482).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_482, tokens_219.as_mut_ptr(), clean);
    let mut rule_123: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh407 = (*rule_123).name;
    *fresh407 = b"members\0" as *const u8 as *const libc::c_char;
    let ref mut fresh408 = (*rule_123).sibling;
    *fresh408 = (*parent).child;
    let ref mut fresh409 = (*parent).child;
    *fresh409 = rule_123;
    let mut parent_170: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_123);
    static mut tokens_220: [*const libc::c_char; 2] = [
        b"field\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_483: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_170);
    p_483 = bnf_add(bnf_create(BNF_OR), p_483);
    bnf_rule_set(p_483, tokens_220.as_mut_ptr());
    static mut tokens_221: [*const libc::c_char; 2] = [
        b"method\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_484: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_170);
    p_484 = bnf_add(bnf_create(BNF_OR), p_484);
    bnf_rule_set(p_484, tokens_221.as_mut_ptr());
    let mut rule_124: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh410 = (*rule_124).name;
    *fresh410 = b"class_def\0" as *const u8 as *const libc::c_char;
    let ref mut fresh411 = (*rule_124).sibling;
    *fresh411 = (*parent).child;
    let ref mut fresh412 = (*parent).child;
    *fresh412 = rule_124;
    let mut parent_171: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_124);
    static mut tokens_222: [token_id; 8] = [
        TK_TYPE,
        TK_INTERFACE,
        TK_TRAIT,
        TK_PRIMITIVE,
        TK_STRUCT,
        TK_CLASS,
        TK_ACTOR,
        TK_NONE,
    ];
    let mut p_485: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_485).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_485, tokens_222.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_216: [*const libc::c_char; 2] = [
        b"annotations\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_486: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_486).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_486, rules_216.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_223: [token_id; 2] = [TK_AT, TK_NONE];
    let mut p_487: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_487).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_487, tokens_223.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_217: [*const libc::c_char; 2] = [
        b"cap\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_488: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_488).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_488, rules_217.as_mut_ptr());
    static mut tokens_224: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_489: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_489).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_489, tokens_224.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_218: [*const libc::c_char; 2] = [
        b"typeparams\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_490: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_490).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_490, rules_218.as_mut_ptr());
    let mut p_491: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_491).optional = 1 as libc::c_int != 0;
    let mut parent_172: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_491);
    static mut tokens_225: [token_id; 2] = [TK_IS, TK_NONE];
    let mut p_492: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_172);
    (*p_492).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_492, tokens_225.as_mut_ptr(), clean);
    static mut rules_219: [*const libc::c_char; 2] = [
        b"provides\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_493: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_172);
    (*p_493).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_493, rules_219.as_mut_ptr());
    optional = 1 as libc::c_int != 0;
    static mut tokens_226: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut p_494: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_494).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_494, tokens_226.as_mut_ptr(), clean);
    static mut rules_220: [*const libc::c_char; 2] = [
        b"members\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_495: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_171);
    (*p_495).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_495, rules_220.as_mut_ptr());
    let mut rule_125: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh413 = (*rule_125).name;
    *fresh413 = b"use_uri\0" as *const u8 as *const libc::c_char;
    let ref mut fresh414 = (*rule_125).sibling;
    *fresh414 = (*parent).child;
    let ref mut fresh415 = (*parent).child;
    *fresh415 = rule_125;
    let mut parent_173: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_125);
    (*rule_125).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_227: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut p_496: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_173);
    (*p_496).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_496, tokens_227.as_mut_ptr(), clean);
    let mut rule_126: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh416 = (*rule_126).name;
    *fresh416 = b"use_ffi\0" as *const u8 as *const libc::c_char;
    let ref mut fresh417 = (*rule_126).sibling;
    *fresh417 = (*parent).child;
    let ref mut fresh418 = (*parent).child;
    *fresh418 = rule_126;
    let mut parent_174: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_126);
    static mut tokens_228: [token_id; 2] = [TK_AT, TK_NONE];
    let mut p_497: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_497).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_497, tokens_228.as_mut_ptr(), clean);
    static mut tokens_229: [token_id; 3] = [TK_ID, TK_STRING, TK_NONE];
    let mut p_498: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_498).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_498, tokens_229.as_mut_ptr(), clean);
    static mut rules_221: [*const libc::c_char; 2] = [
        b"typeargs\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_499: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_499).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_499, rules_221.as_mut_ptr());
    static mut tokens_230: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut p_500: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_500).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_500, tokens_230.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_222: [*const libc::c_char; 2] = [
        b"params\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_501: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_501).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_501, rules_222.as_mut_ptr());
    static mut tokens_231: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut p_502: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_502).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_502, tokens_231.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut tokens_232: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut p_503: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_174);
    (*p_503).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_503, tokens_232.as_mut_ptr(), clean);
    let mut rule_127: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh419 = (*rule_127).name;
    *fresh419 = b"use_name\0" as *const u8 as *const libc::c_char;
    let ref mut fresh420 = (*rule_127).sibling;
    *fresh420 = (*parent).child;
    let ref mut fresh421 = (*parent).child;
    *fresh421 = rule_127;
    let mut parent_175: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_127);
    (*rule_127).inline_rule = 1 as libc::c_int != 0;
    static mut tokens_233: [token_id; 2] = [TK_ID, TK_NONE];
    let mut p_504: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_175);
    (*p_504).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_504, tokens_233.as_mut_ptr(), clean);
    static mut tokens_234: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut p_505: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_175);
    (*p_505).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_505, tokens_234.as_mut_ptr(), clean);
    let mut rule_128: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh422 = (*rule_128).name;
    *fresh422 = b"use\0" as *const u8 as *const libc::c_char;
    let ref mut fresh423 = (*rule_128).sibling;
    *fresh423 = (*parent).child;
    let ref mut fresh424 = (*parent).child;
    *fresh424 = rule_128;
    let mut parent_176: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_128);
    static mut tokens_235: [token_id; 2] = [TK_USE, TK_NONE];
    let mut p_506: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_176);
    (*p_506).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_506, tokens_235.as_mut_ptr(), clean);
    optional = 1 as libc::c_int != 0;
    static mut rules_223: [*const libc::c_char; 2] = [
        b"use_name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_507: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_176);
    (*p_507).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_507, rules_223.as_mut_ptr());
    static mut rules_224: [*const libc::c_char; 3] = [
        b"use_uri\0" as *const u8 as *const libc::c_char,
        b"use_ffi\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_508: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_176);
    (*p_508).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_508, rules_224.as_mut_ptr());
    let mut p_509: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_176);
    (*p_509).optional = 1 as libc::c_int != 0;
    let mut parent_177: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), p_509);
    static mut tokens_236: [token_id; 2] = [TK_IF, TK_NONE];
    let mut p_510: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_177);
    (*p_510).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_510, tokens_236.as_mut_ptr(), clean);
    static mut rules_225: [*const libc::c_char; 2] = [
        b"infix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_511: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_177);
    (*p_511).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_rule_set(p_511, rules_225.as_mut_ptr());
    let mut rule_129: *mut bnf_t = bnf_create(BNF_DEF);
    let ref mut fresh425 = (*rule_129).name;
    *fresh425 = b"module\0" as *const u8 as *const libc::c_char;
    let ref mut fresh426 = (*rule_129).sibling;
    *fresh426 = (*parent).child;
    let ref mut fresh427 = (*parent).child;
    *fresh427 = rule_129;
    let mut parent_178: *mut bnf_t = bnf_add(bnf_create(BNF_SEQ), rule_129);
    optional = 1 as libc::c_int != 0;
    static mut tokens_237: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut p_512: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_178);
    (*p_512).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_512, tokens_237.as_mut_ptr(), clean);
    static mut tokens_238: [*const libc::c_char; 2] = [
        b"use\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_513: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_178);
    p_513 = bnf_add(bnf_create(BNF_OR), p_513);
    bnf_rule_set(p_513, tokens_238.as_mut_ptr());
    static mut tokens_239: [*const libc::c_char; 2] = [
        b"class_def\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p_514: *mut bnf_t = bnf_add(bnf_create(BNF_REPEAT), parent_178);
    p_514 = bnf_add(bnf_create(BNF_OR), p_514);
    bnf_rule_set(p_514, tokens_239.as_mut_ptr());
    static mut tokens_240: [token_id; 2] = [TK_EOF, TK_NONE];
    let mut p_515: *mut bnf_t = bnf_add(bnf_create(BNF_OR), parent_178);
    (*p_515).optional = optional;
    optional = 0 as libc::c_int != 0;
    bnf_token_set(p_515, tokens_240.as_mut_ptr(), clean);
    return parent;
}
#[no_mangle]
#[c2rust::src_loc = "875:1"]
pub unsafe extern "C" fn print_grammar(mut antlr: bool, mut clean: bool) {
    let mut tree: *mut bnf_t = bnf_def(clean);
    if !tree.is_null() {
    } else {
        ponyint_assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/bnfprint.c\0" as *const u8
                as *const libc::c_char,
            878 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"print_grammar\0"))
                .as_ptr(),
        );
    };
    bnf_simplify(tree);
    if antlr {
        bnf_avoid_antlr_bug(tree, tree);
    }
    bnf_mark_used_rules(tree);
    if antlr {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, antlr_pre);
    }
    bnf_print(tree, 0 as libc::c_int != 0);
    if antlr {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, antlr_post);
    }
}
