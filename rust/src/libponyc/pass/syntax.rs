use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_ct_rune_t = libc::c_int;
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
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
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
    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "161:1"]
        pub fn ast_error_continue(
            errors: *mut errors_t,
            ast: *mut ast_t,
            fmt: *const libc::c_char,
            _: ...
        );
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "156:1"]
        pub fn ast_print_type(type_0: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "88:1"]
        pub fn ast_checkflag(ast: *mut ast_t, flag: uint32_t) -> libc::c_int;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "102:1"]
        pub fn ast_annotation(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "105:1"]
        pub fn ast_has_annotation(ast: *mut ast_t, name: *const libc::c_char) -> bool;
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:2"]
pub mod id_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn check_id_type(
            opt: *mut pass_opt_t,
            id_node: *mut ast_t,
            entity_desc: *const libc::c_char,
        ) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn check_id_type_param(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "20:1"]
        pub fn check_id_package(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn check_id_field(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "28:1"]
        pub fn check_id_method(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn check_id_local(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "51:1"]
        pub fn is_name_dontcare(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/platformfuns.h:7"]
pub mod platformfuns_h {
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn os_is_target(
            attribute: *const libc::c_char,
            release: bool,
            out_is_target: *mut bool,
            options: *mut pass_opt_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:9"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:10"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:11"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "84:6"]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:12"]
pub mod _ctype_h {
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "291:1"]
    pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
        __tolower(_c)
    }
    use super::_types_h::__darwin_ct_rune_t;
    extern "C" {
        #[c2rust::src_loc = "189:1"]
        pub fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    }
}
pub use self::_ctype_h::{__tolower, tolower};
pub use self::_size_t_h::size_t;
pub use self::_types_h::{__darwin_ct_rune_t, __darwin_size_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::ast_h::{
    ast_annotation, ast_checkflag, ast_child, ast_childcount, ast_childidx, ast_error,
    ast_error_continue, ast_get_children, ast_has_annotation, ast_id, ast_name, ast_parent,
    ast_print_type, ast_ptr_t, ast_result_t, ast_sibling, C2RustUnnamed, AST_ERROR, AST_FATAL,
    AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2, AST_IGNORE,
    AST_OK,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::{
    check_id_field, check_id_local, check_id_method, check_id_package, check_id_type,
    check_id_type_param, is_name_dontcare,
};
pub use self::pass_h::{
    ast_visit, ast_visit_t, magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level,
    PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER,
    PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT,
    PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX,
    PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET,
    VERBOSITY_TOOL_INFO,
};
use self::platformfuns_h::os_is_target;
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc_size, ponyint_pool_free_size};
use self::string_h::{strcmp, strlen, strncmp};
use self::stringtab_h::{stringtab};
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
#[c2rust::src_loc = "30:16"]
pub struct permission_def_t {
    pub desc: *const libc::c_char,
    pub permissions: *const libc::c_char,
}
#[c2rust::src_loc = "50:31"]
static mut _entity_def: [permission_def_t; 7] = [
    {
        let mut init = permission_def_t {
            desc: b"actor\0" as *const u8 as *const libc::c_char,
            permissions: b"X X N X\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"class\0" as *const u8 as *const libc::c_char,
            permissions: b"N X X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"struct\0" as *const u8 as *const libc::c_char,
            permissions: b"N X X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"primitive\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"trait\0" as *const u8 as *const libc::c_char,
            permissions: b"N N X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"interface\0" as *const u8 as *const libc::c_char,
            permissions: b"N N X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"type alias\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
#[c2rust::src_loc = "71:31"]
static mut _method_def: [permission_def_t; 21] = [
    {
        let mut init = permission_def_t {
            desc: b"actor function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"class function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"struct function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"primitive function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"trait function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X X\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"interface function\0" as *const u8 as *const libc::c_char,
            permissions: b"X X X X X\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"type alias function\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"actor behaviour\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"class behaviour\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"struct behaviour\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"primitive behaviour\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"trait behaviour\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N X\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"interface behaviour\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N X\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"type alias behaviour\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"actor constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N N Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"class constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"X N N X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"struct constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"X N N X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"primitive constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"N N N X Y\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"trait constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"X N N X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"interface constructor\0" as *const u8 as *const libc::c_char,
            permissions: b"X N N X N\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = permission_def_t {
            desc: b"type alias constructor\0" as *const u8 as *const libc::c_char,
            permissions: 0 as *const libc::c_char,
        };
        init
    },
];
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn is_expr_infix(mut id: token_id) -> bool {
    match id as libc::c_uint {
        130 | 131 | 132 | 25 | 27 | 29 | 31 | 33 | 35 | 26 | 28 | 30 | 32 | 34 | 36 | 39 | 41
        | 40 | 42 | 82 | 83 | 51 | 53 | 43 | 45 | 47 | 49 | 52 | 54 | 44 | 46 | 48 | 50 | 149
        | 56 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn check_provides_type(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut description: *const libc::c_char,
) -> bool {
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            152 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"check_provides_type\0"))
                .as_ptr(),
        );
    };
    if !description.is_null() {
    } else {
        ponyint_assert_fail(
            b"description != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"check_provides_type\0"))
                .as_ptr(),
        );
    };
    match ast_id(type_0) as libc::c_uint {
        151 => {
            let mut ignore0: ast_ptr_t = 0 as *mut ast_t;
            let mut ignore1: ast_ptr_t = 0 as *mut ast_t;
            let mut ignore2: ast_ptr_t = 0 as *mut ast_t;
            let mut cap: ast_ptr_t = 0 as *mut ast_t;
            let mut ephemeral: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 6] = [
                &mut ignore0,
                &mut ignore1,
                &mut ignore2,
                &mut cap,
                &mut ephemeral,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            if ast_id(cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    cap,
                    b"can't specify a capability in a provides type\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            if ast_id(ephemeral) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    ephemeral,
                    b"can't specify ephemeral in a provides type\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        148 | 56 => {
            let mut p: *mut ast_t = ast_child(type_0);
            while !p.is_null() {
                if !check_provides_type(opt, p, description) {
                    return 0 as libc::c_int != 0;
                }
                p = ast_sibling(p);
            }
            return 1 as libc::c_int != 0;
        }
        _ => {
            ast_error(
                (*opt).check.errors,
                type_0,
                b"invalid %s type. Can only be interfaces, traits and intersects of those.\0"
                    as *const u8 as *const libc::c_char,
                description,
            );
            return 0 as libc::c_int != 0;
        }
    };
}
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn check_permission(
    mut opt: *mut pass_opt_t,
    mut def: *const permission_def_t,
    mut element: libc::c_int,
    mut actual: *mut ast_t,
    mut context: *const libc::c_char,
    mut report_at: *mut ast_t,
) -> bool {
    if !def.is_null() {
    } else {
        ponyint_assert_fail(
            b"def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            201 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"check_permission\0"))
                .as_ptr(),
        );
    };
    if !actual.is_null() {
    } else {
        ponyint_assert_fail(
            b"actual != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            202 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"check_permission\0"))
                .as_ptr(),
        );
    };
    if !context.is_null() {
    } else {
        ponyint_assert_fail(
            b"context != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"check_permission\0"))
                .as_ptr(),
        );
    };
    let mut permission: libc::c_char = *((*def).permissions).offset(element as isize);
    if permission as libc::c_int == 'Y' as i32
        || permission as libc::c_int == 'N' as i32
        || permission as libc::c_int == 'X' as i32
    {
    } else {
        ponyint_assert_fail(
            b"permission == 'Y' || permission == 'N' || permission == 'X'\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            207 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"check_permission\0"))
                .as_ptr(),
        );
    };
    if permission as libc::c_int == 'N' as i32
        && ast_id(actual) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            actual,
            b"%s cannot specify %s\0" as *const u8 as *const libc::c_char,
            (*def).desc,
            context,
        );
        return 0 as libc::c_int != 0;
    }
    if permission as libc::c_int == 'Y' as i32
        && ast_id(actual) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            report_at,
            b"%s must specify %s\0" as *const u8 as *const libc::c_char,
            (*def).desc,
            context,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "228:1"]
unsafe extern "C" fn check_method(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut method_def_index: libc::c_int,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            230 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"check_method\0")).as_ptr(),
        );
    };
    if method_def_index >= 0 as libc::c_int
        && method_def_index < 7 as libc::c_int * 3 as libc::c_int
    {
    } else {
        ponyint_assert_fail(
            b"method_def_index >= 0 && method_def_index < DEF_METHOD_COUNT\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            231 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"check_method\0")).as_ptr(),
        );
    };
    let mut r: bool = 1 as libc::c_int != 0;
    let mut def: *const permission_def_t =
        &*_method_def.as_ptr().offset(method_def_index as isize) as *const permission_def_t;
    if ((*def).permissions).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"%ss are not allowed\0" as *const u8 as *const libc::c_char,
            (*def).desc,
        );
        return 0 as libc::c_int != 0;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut type_params: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut return_type: ast_ptr_t = 0 as *mut ast_t;
    let mut error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut docstring: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut cap,
        &mut id,
        &mut type_params,
        &mut params,
        &mut return_type,
        &mut error,
        &mut body,
        &mut docstring,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        if !check_permission(
            opt,
            def,
            2 as libc::c_int,
            cap,
            b"bareness\0" as *const u8 as *const libc::c_char,
            cap,
        ) {
            r = 0 as libc::c_int != 0;
        }
    } else if !check_permission(
        opt,
        def,
        0 as libc::c_int,
        cap,
        b"receiver capability\0" as *const u8 as *const libc::c_char,
        cap,
    ) {
        r = 0 as libc::c_int != 0;
    }
    if !check_id_method(opt, id) {
        r = 0 as libc::c_int != 0;
    }
    if !check_permission(
        opt,
        def,
        4 as libc::c_int,
        return_type,
        b"return type\0" as *const u8 as *const libc::c_char,
        ast,
    ) {
        r = 0 as libc::c_int != 0;
    }
    if !check_permission(
        opt,
        def,
        6 as libc::c_int,
        error,
        b"?\0" as *const u8 as *const libc::c_char,
        ast,
    ) {
        r = 0 as libc::c_int != 0;
    }
    if !check_permission(
        opt,
        def,
        8 as libc::c_int,
        body,
        b"body\0" as *const u8 as *const libc::c_char,
        ast,
    ) {
        r = 0 as libc::c_int != 0;
    }
    if ast_id(docstring) as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        if ast_id(body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                docstring,
                b"methods with bodies must put docstrings in the body\0" as *const u8
                    as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
    }
    return r;
}
#[c2rust::src_loc = "283:1"]
unsafe extern "C" fn check_members(
    mut opt: *mut pass_opt_t,
    mut members: *mut ast_t,
    mut entity_def_index: libc::c_int,
) -> bool {
    if !members.is_null() {
    } else {
        ponyint_assert_fail(
            b"members != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            285 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"check_members\0"))
                .as_ptr(),
        );
    };
    if entity_def_index >= 0 as libc::c_int && entity_def_index < 7 as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"entity_def_index >= 0 && entity_def_index < DEF_ENTITY_COUNT\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            286 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"check_members\0"))
                .as_ptr(),
        );
    };
    let mut r: bool = 1 as libc::c_int != 0;
    let mut def: *const permission_def_t =
        &*_entity_def.as_ptr().offset(entity_def_index as isize) as *const permission_def_t;
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            141 | 140 | 86 => {
                if *((*def).permissions).offset(2 as libc::c_int as isize) as libc::c_int
                    == 'N' as i32
                {
                    ast_error(
                        (*opt).check.errors,
                        member,
                        b"Can't have fields in %s\0" as *const u8 as *const libc::c_char,
                        (*def).desc,
                    );
                    r = 0 as libc::c_int != 0;
                }
                if ast_id(ast_parent(members)) as libc::c_uint
                    == TK_OBJECT as libc::c_int as libc::c_uint
                    && ast_id(ast_childidx(member, 2 as libc::c_int as size_t)) as libc::c_uint
                        == TK_NONE as libc::c_int as libc::c_uint
                {
                    ast_error(
                        (*opt).check.errors,
                        member,
                        b"object literal fields must be initialized\0" as *const u8
                            as *const libc::c_char,
                    );
                    r = 0 as libc::c_int != 0;
                }
                if !check_id_field(opt, ast_child(member)) {
                    r = 0 as libc::c_int != 0;
                }
            }
            88 => {
                if !check_method(
                    opt,
                    member,
                    entity_def_index + 7 as libc::c_int * 2 as libc::c_int,
                ) {
                    r = 0 as libc::c_int != 0;
                }
            }
            90 => {
                if !check_method(
                    opt,
                    member,
                    entity_def_index + 7 as libc::c_int * 1 as libc::c_int,
                ) {
                    r = 0 as libc::c_int != 0;
                }
            }
            89 => {
                if !check_method(
                    opt,
                    member,
                    entity_def_index + 7 as libc::c_int * 0 as libc::c_int,
                ) {
                    r = 0 as libc::c_int != 0;
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0"
                            as *const u8 as *const libc::c_char,
                        341 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            b"check_members\0",
                        ))
                        .as_ptr(),
                    );
                };
                return 0 as libc::c_int != 0;
            }
        }
        member = ast_sibling(member);
    }
    r
}
#[c2rust::src_loc = "353:1"]
unsafe extern "C" fn syntax_entity(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut entity_def_index: libc::c_int,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            356 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_entity\0"))
                .as_ptr(),
        );
    };
    if entity_def_index >= 0 as libc::c_int && entity_def_index < 7 as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"entity_def_index >= 0 && entity_def_index < DEF_ENTITY_COUNT\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            357 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_entity\0"))
                .as_ptr(),
        );
    };
    let mut r: ast_result_t = AST_OK;
    let mut def: *const permission_def_t =
        &*_entity_def.as_ptr().offset(entity_def_index as isize) as *const permission_def_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut defcap: ast_ptr_t = 0 as *mut ast_t;
    let mut provides: ast_ptr_t = 0 as *mut ast_t;
    let mut members: ast_ptr_t = 0 as *mut ast_t;
    let mut c_api: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 7] = [
        &mut id,
        &mut typeparams,
        &mut defcap,
        &mut provides,
        &mut members,
        &mut c_api,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_name(id) == stringtab(b"Main\0" as *const u8 as *const libc::c_char) {
        if ast_id(typeparams) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                typeparams,
                b"the Main actor cannot have type parameters\0" as *const u8 as *const libc::c_char,
            );
            r = AST_ERROR;
        }
        if *((*def).permissions).offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
            ast_error(
                (*opt).check.errors,
                ast,
                b"Main must be an actor\0" as *const u8 as *const libc::c_char,
            );
            r = AST_ERROR;
        }
    }
    if !check_id_type(opt, id, (*def).desc) {
        r = AST_ERROR;
    }
    if !check_permission(
        opt,
        def,
        4 as libc::c_int,
        defcap,
        b"default capability\0" as *const u8 as *const libc::c_char,
        defcap,
    ) {
        r = AST_ERROR;
    }
    if !check_permission(
        opt,
        def,
        6 as libc::c_int,
        c_api,
        b"C api\0" as *const u8 as *const libc::c_char,
        c_api,
    ) {
        r = AST_ERROR;
    }
    if ast_id(c_api) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        if ast_id(typeparams) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                typeparams,
                b"generic actor cannot specify C api\0" as *const u8 as *const libc::c_char,
            );
            r = AST_ERROR;
        }
    }
    if entity_def_index != 6 as libc::c_int {
        if ast_id(provides) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            if ast_has_annotation(ast, b"nosupertype\0" as *const u8 as *const libc::c_char) {
                ast_error(
                    (*opt).check.errors,
                    provides,
                    b"a 'nosupertype' type cannot specify a provides list\0" as *const u8
                        as *const libc::c_char,
                );
                r = AST_ERROR;
            } else if !check_provides_type(
                opt,
                provides,
                b"provides\0" as *const u8 as *const libc::c_char,
            ) {
                r = AST_ERROR;
            }
        }
    } else if ast_id(provides) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            provides,
            b"a type alias must specify a type\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if !check_members(opt, members, entity_def_index) {
        r = AST_ERROR;
    }
    return r;
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn syntax_thistype(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            436 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_thistype\0"))
                .as_ptr(),
        );
    };
    let mut parent: *mut ast_t = ast_parent(ast);
    if !parent.is_null() {
    } else {
        ponyint_assert_fail(
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            438 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_thistype\0"))
                .as_ptr(),
        );
    };
    let mut r: ast_result_t = AST_OK;
    if ast_id(parent) as libc::c_uint != TK_ARROW as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"in a type, 'this' can only be used as a viewpoint\0" as *const u8
                as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if ((*(*opt).check.frame).method).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"can only use 'this' for a viewpoint in a method\0" as *const u8
                as *const libc::c_char,
        );
        r = AST_ERROR;
    } else {
        let mut cap: *mut ast_t = ast_child((*(*opt).check.frame).method);
        match ast_id(cap) as libc::c_uint {
            95 | 2 => {}
            _ => {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"can only use 'this' for a viewpoint in a box function\0" as *const u8
                        as *const libc::c_char,
                );
                r = AST_ERROR;
            }
        }
    }
    return r;
}
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn syntax_arrow(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            475 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_arrow\0")).as_ptr(),
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
    if !((*(*opt).check.frame).constraint).is_null()
        || !((*(*opt).check.frame).iftype_constraint).is_null()
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"arrow types can't be used as type constraints\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    match ast_id(right) as libc::c_uint {
        152 => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"'this' cannot appear to the right of a viewpoint\0" as *const u8
                    as *const libc::c_char,
            );
            return AST_ERROR;
        }
        91 | 92 | 93 | 94 | 95 | 96 => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"refcaps cannot appear to the right of a viewpoint\0" as *const u8
                    as *const libc::c_char,
            );
            return AST_ERROR;
        }
        _ => {}
    }
    AST_OK
}
#[c2rust::src_loc = "510:1"]
unsafe extern "C" fn syntax_tupletype(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !((*(*opt).check.frame).constraint).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"tuple types can't be used as type constraints\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "523:1"]
unsafe extern "C" fn syntax_nominal(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            525 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"syntax_nominal\0"))
                .as_ptr(),
        );
    };
    let mut package: ast_ptr_t = 0 as *mut ast_t;
    let mut name: ast_ptr_t = 0 as *mut ast_t;
    let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut eph: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut package,
        &mut name,
        &mut typeargs,
        &mut cap,
        &mut eph,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if !is_name_dontcare(ast_name(name)) {
        return AST_OK;
    }
    let mut r: ast_result_t = AST_OK;
    if ast_id(package) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            package,
            b"'_' cannot be in a package\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if ast_id(typeargs) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            typeargs,
            b"'_' cannot have generic arguments\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if ast_id(cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            cap,
            b"'_' cannot specify capability\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if ast_id(eph) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            eph,
            b"'_' cannot specify capability modifier\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    r
}
#[c2rust::src_loc = "565:1"]
unsafe extern "C" fn syntax_match(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            567 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_match\0")).as_ptr(),
        );
    };
    let mut cases: *mut ast_t = ast_childidx(ast, 1 as libc::c_int as size_t);
    if !cases.is_null() {
    } else {
        ponyint_assert_fail(
            b"cases != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            571 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_match\0")).as_ptr(),
        );
    };
    if ast_id(cases) as libc::c_uint == TK_CASES as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(cases) == TK_CASES\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            572 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_match\0")).as_ptr(),
        );
    };
    let mut case_ast: *mut ast_t = ast_child(cases);
    if case_ast.is_null() {
        return AST_OK;
    }
    while !(ast_sibling(case_ast)).is_null() {
        case_ast = ast_sibling(case_ast);
    }
    let mut body: *mut ast_t = ast_childidx(case_ast, 2 as libc::c_int as size_t);
    if ast_id(body) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            case_ast,
            b"Last case in match must have a body\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "595:1"]
unsafe extern "C" fn syntax_ffi(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut is_declaration: bool,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            598 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"syntax_ffi\0")).as_ptr(),
        );
    };
    let mut r: ast_result_t = AST_OK;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut ffi_args: ast_ptr_t = 0 as *mut ast_t;
    let mut ffi_named_args: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut id,
        &mut typeargs,
        &mut ffi_args,
        &mut ffi_named_args,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if (ast_child(typeargs)).is_null() && is_declaration as libc::c_int != 0
        || !(ast_childidx(typeargs, 1 as libc::c_int as size_t)).is_null()
    {
        ast_error(
            (*opt).check.errors,
            typeargs,
            b"FFI functions must specify a single return type\0" as *const u8
                as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    let mut p: *mut ast_t = ast_child(ffi_args);
    while !p.is_null() {
        if ast_id(p) as libc::c_uint == TK_PARAM as libc::c_int as libc::c_uint {
            let mut def_val: *mut ast_t = ast_childidx(p, 2 as libc::c_int as size_t);
            if !def_val.is_null() {
            } else {
                ponyint_assert_fail(
                    b"def_val != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0"
                        as *const u8 as *const libc::c_char,
                    616 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"syntax_ffi\0"))
                        .as_ptr(),
                );
            };
            if ast_id(def_val) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    def_val,
                    b"FFIs parameters cannot have default values\0" as *const u8
                        as *const libc::c_char,
                );
                r = AST_ERROR;
            }
        }
        p = ast_sibling(p);
    }
    if ast_id(ffi_named_args) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ffi_named_args,
            b"FFIs cannot take named arguments\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    r
}
#[c2rust::src_loc = "638:1"]
unsafe extern "C" fn syntax_ffi_decl(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_FFIDECL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_FFIDECL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            640 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_ffi_decl\0"))
                .as_ptr(),
        );
    };
    return syntax_ffi(opt, ast, 1 as libc::c_int != 0);
}
#[c2rust::src_loc = "644:1"]
unsafe extern "C" fn syntax_ffi_call(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_FFICALL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_FFICALL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            646 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_ffi_call\0"))
                .as_ptr(),
        );
    };
    let mut r: ast_result_t = AST_OK;
    let mut in_method: *mut ast_t = (*(*opt).check.frame).method;
    if !in_method.is_null()
        && (ast_id(in_method) as libc::c_uint == TK_BE as libc::c_int as libc::c_uint
            || ast_id(in_method) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint)
    {
        let mut parent: *mut ast_t = ast_parent(ast);
        match ast_id((*(*opt).check.frame).method) as libc::c_uint {
            90 | 89 => {
                while !parent.is_null() {
                    match ast_id(parent) as libc::c_uint {
                        72 | 73 => {
                            ast_error(
                                (*opt).check.errors,
                                ast,
                                b"Can't call an FFI function in a default method or behavior\0"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                            r = AST_ERROR;
                        }
                        _ => {}
                    }
                    parent = ast_parent(parent);
                }
            }
            _ => {}
        }
    }
    if syntax_ffi(opt, ast, 0 as libc::c_int != 0) as libc::c_uint
        == AST_ERROR as libc::c_int as libc::c_uint
    {
        r = AST_ERROR;
    }
    r
}
#[c2rust::src_loc = "683:1"]
unsafe extern "C" fn syntax_ellipsis(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            685 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_ellipsis\0"))
                .as_ptr(),
        );
    };
    let mut r: ast_result_t = AST_OK;
    let mut fn_0: *mut ast_t = ast_parent(ast_parent(ast));
    if !fn_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"fn != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            689 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"syntax_ellipsis\0"))
                .as_ptr(),
        );
    };
    if ast_id(fn_0) as libc::c_uint != TK_FFIDECL as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"... may only appear in FFI declarations\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    if !(ast_sibling(ast)).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"... must be the last parameter\0" as *const u8 as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    r
}
#[c2rust::src_loc = "708:1"]
unsafe extern "C" fn syntax_infix_expr(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            710 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_infix_expr\0"))
                .as_ptr(),
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
    let mut op: token_id = ast_id(ast);
    if !left.is_null() {
    } else {
        ponyint_assert_fail(
            b"left != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            715 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_infix_expr\0"))
                .as_ptr(),
        );
    };
    let mut left_op: token_id = ast_id(left);
    let mut left_clash: bool = left_op as libc::c_uint != op as libc::c_uint
        && is_expr_infix(left_op) as libc::c_int != 0
        && ast_checkflag(left, AST_FLAG_IN_PARENS as libc::c_int as uint32_t) == 0;
    if !right.is_null() {
    } else {
        ponyint_assert_fail(
            b"right != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            720 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_infix_expr\0"))
                .as_ptr(),
        );
    };
    let mut right_op: token_id = ast_id(right);
    let mut right_clash: bool = right_op as libc::c_uint != op as libc::c_uint
        && is_expr_infix(right_op) as libc::c_int != 0
        && ast_checkflag(right, AST_FLAG_IN_PARENS as libc::c_int as uint32_t) == 0;
    if left_clash as libc::c_int != 0 || right_clash as libc::c_int != 0 {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Operator precedence is not supported. Parentheses required.\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "736:1"]
unsafe extern "C" fn syntax_consume(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
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
    match ast_id(term) as libc::c_uint {
        102 | 184 => return AST_OK,
        19 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                term,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            if ast_id(left) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint
                && ast_id(left) as libc::c_uint != TK_SEQ as libc::c_int as libc::c_uint
            {
                return AST_OK;
            }
        }
        _ => {}
    }
    ast_error(
        (*opt).check.errors,
        term,
        b"Consume expressions must specify an identifier or field\0" as *const u8
            as *const libc::c_char,
    );
    AST_ERROR
}
#[c2rust::src_loc = "761:1"]
unsafe extern "C" fn syntax_return(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut max_value_count: size_t,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            764 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_return\0"))
                .as_ptr(),
        );
    };
    let mut value_seq: *mut ast_t = ast_child(ast);
    if ast_id(value_seq) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
        || ast_id(value_seq) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"ast_id(value_seq) == TK_SEQ || ast_id(value_seq) == TK_NONE\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            767 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_return\0"))
                .as_ptr(),
        );
    };
    let mut value_count: size_t = ast_childcount(value_seq);
    if value_count > max_value_count {
        ast_error(
            (*opt).check.errors,
            ast_childidx(value_seq, max_value_count),
            b"Unreachable code\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    let mut parent: *mut ast_t = ast_parent(ast);
    let mut current: *mut ast_t = ast;
    while ast_id(parent) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        if !(ast_sibling(current)).is_null() {
            ast_error(
                (*opt).check.errors,
                ast_sibling(current),
                b"Unreachable code\0" as *const u8 as *const libc::c_char,
            );
            return AST_ERROR;
        }
        current = parent;
        parent = ast_parent(parent);
    }
    if ast_id(ast) as libc::c_uint == TK_RETURN as libc::c_int as libc::c_uint {
        if ((*(*opt).check.frame).method_body).is_null() {
            ast_error(
                (*opt).check.errors,
                ast,
                b"return must occur in a method body\0" as *const u8 as *const libc::c_char,
            );
            return AST_ERROR;
        }
        if value_count > 0 as libc::c_int as libc::c_ulong {
            if ast_id((*(*opt).check.frame).method) as libc::c_uint
                == TK_BE as libc::c_int as libc::c_uint
                || ast_id((*(*opt).check.frame).method) as libc::c_uint
                    == TK_NEW as libc::c_int as libc::c_uint
            {
                let mut should_error: bool = 1 as libc::c_int != 0;
                let mut pparent: *mut ast_t = parent;
                match ast_id((*(*opt).check.frame).method) as libc::c_uint {
                    90 | 88 => {
                        while !pparent.is_null() {
                            match ast_id(pparent) as libc::c_uint {
                                79 => {
                                    should_error = 0 as libc::c_int != 0;
                                }
                                90 | 88 | _ => {}
                            }
                            pparent = ast_parent(pparent);
                        }
                    }
                    _ => {}
                }
                if should_error {
                    ast_error(
                        (*opt).check.errors,
                        ast,
                        b"A return in a constructor or a behaviour can't return a value\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return AST_ERROR;
                }
            }
        }
    }
    AST_OK
}
#[c2rust::src_loc = "842:1"]
unsafe extern "C" fn syntax_semi(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !(ast_parent(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_parent(ast) != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            844 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"syntax_semi\0")).as_ptr(),
        );
    };
    if ast_id(ast_parent(ast)) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast_parent(ast)) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            845 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"syntax_semi\0")).as_ptr(),
        );
    };
    if ast_checkflag(ast, AST_FLAG_BAD_SEMI as libc::c_int as uint32_t) != 0 {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Unexpected semicolon, only use to separate expressions on the same line\0"
                as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "858:1"]
unsafe extern "C" fn syntax_local(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !check_id_local(opt, ast_child(ast)) {
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "867:1"]
unsafe extern "C" fn syntax_embed(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ast_id(ast_parent(ast)) as libc::c_uint != TK_MEMBERS as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"Local variables cannot be embedded\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "879:1"]
unsafe extern "C" fn syntax_type_param(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !check_id_type_param(opt, ast_child(ast)) {
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "889:26"]
static mut _illegal_flags: [*const libc::c_char; 4] = [
    b"ndebug\0" as *const u8 as *const libc::c_char,
    b"unknown_os\0" as *const u8 as *const libc::c_char,
    b"unknown_size\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[c2rust::src_loc = "901:1"]
unsafe extern "C" fn syntax_ifdef_cond(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut context: *const libc::c_char,
) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            903 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_ifdef_cond\0"))
                .as_ptr(),
        );
    };
    if !context.is_null() {
    } else {
        ponyint_assert_fail(
            b"context != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            904 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_ifdef_cond\0"))
                .as_ptr(),
        );
    };
    match ast_id(ast) as libc::c_uint {
        130 | 131 | 129 => {}
        2 => {}
        5 => {
            let mut name: *const libc::c_char = ast_name(ast);
            let mut len: size_t = (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut lower_case: *mut libc::c_char =
                ponyint_pool_alloc_size(len) as *mut libc::c_char;
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < len {
                *lower_case.offset(i as isize) =
                    tolower(*name.offset(i as isize) as libc::c_int) as libc::c_char;
                i = i.wrapping_add(1);
            }
            let mut r: bool = 1 as libc::c_int != 0;
            let mut result: bool = false;
            if os_is_target(lower_case, 1 as libc::c_int != 0, &mut result, opt) {
                r = 0 as libc::c_int != 0;
            }
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while !(_illegal_flags[i_0 as usize]).is_null() {
                if strcmp(lower_case, _illegal_flags[i_0 as usize]) == 0 as libc::c_int {
                    r = 0 as libc::c_int != 0;
                }
                i_0 += 1;
            }
            ponyint_pool_free_size(len, lower_case as *mut libc::c_void);
            if !r {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"\"%s\" is not a valid user build flag\n\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
                return 0 as libc::c_int != 0;
            }
        }
        184 => {
            let mut name_0: *const libc::c_char = ast_name(ast_child(ast));
            let mut result_0: bool = false;
            if !os_is_target(name_0, 1 as libc::c_int != 0, &mut result_0, opt) {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"\"%s\" is not a valid platform flag\n\0" as *const u8 as *const libc::c_char,
                    name_0,
                );
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        175 => {
            if ast_childcount(ast) != 1 as libc::c_int as libc::c_ulong {
                ast_error(
                    (*opt).check.errors,
                    ast,
                    b"Sequence not allowed in %s\0" as *const u8 as *const libc::c_char,
                    context,
                );
                return 0 as libc::c_int != 0;
            }
        }
        _ => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"Invalid %s\0" as *const u8 as *const libc::c_char,
                context,
            );
            return 0 as libc::c_int != 0;
        }
    }
    let mut p: *mut ast_t = ast_child(ast);
    while !p.is_null() {
        if !syntax_ifdef_cond(opt, p, context) {
            return 0 as libc::c_int != 0;
        }
        p = ast_sibling(p);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "993:1"]
unsafe extern "C" fn syntax_ifdef(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            995 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_ifdef\0")).as_ptr(),
        );
    };
    if !syntax_ifdef_cond(
        opt,
        ast_child(ast),
        b"ifdef condition\0" as *const u8 as *const libc::c_char,
    ) {
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1004:1"]
unsafe extern "C" fn syntax_use(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1006 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"syntax_use\0")).as_ptr(),
        );
    };
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut url: ast_ptr_t = 0 as *mut ast_t;
    let mut guard: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [&mut id, &mut url, &mut guard, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(id) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && !check_id_package(opt, id)
    {
        return AST_ERROR;
    }
    if ast_id(guard) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && !syntax_ifdef_cond(
            opt,
            guard,
            b"use guard\0" as *const u8 as *const libc::c_char,
        )
    {
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1019:1"]
unsafe extern "C" fn syntax_lambda_capture(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    let mut name: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut value: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut name, &mut type_0, &mut value, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(type_0) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && ast_id(value) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"value missing for lambda expression capture (cannot specify type without value)\0"
                as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    return AST_OK;
}
#[c2rust::src_loc = "1034:1"]
unsafe extern "C" fn syntax_barelambdatype(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    let mut fun_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut return_type: ast_ptr_t = 0 as *mut ast_t;
    let mut partial: ast_ptr_t = 0 as *mut ast_t;
    let mut obj_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut obj_mod: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut fun_cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut return_type,
        &mut partial,
        &mut obj_cap,
        &mut obj_mod,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(fun_cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            fun_cap,
            b"a bare lambda cannot specify a receiver capability\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    if ast_id(typeparams) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            typeparams,
            b"a bare lambda cannot specify type parameters\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    match ast_id(obj_cap) as libc::c_uint {
        94 | 2 => {}
        _ => {
            ast_error(
                (*opt).check.errors,
                obj_cap,
                b"a bare lambda can only have a 'val' capability\0" as *const u8
                    as *const libc::c_char,
            );
            return AST_ERROR;
        }
    }
    AST_OK
}
#[c2rust::src_loc = "1069:1"]
unsafe extern "C" fn syntax_compile_intrinsic(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    let mut parent: *mut ast_t = ast_parent(ast);
    if ast_id(parent) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(parent) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1072 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"syntax_compile_intrinsic\0",
            ))
            .as_ptr(),
        );
    };
    let mut method: *mut ast_t = ast_parent(parent);
    match ast_id(method) as libc::c_uint {
        88 | 90 | 89 => {}
        _ => {
            ast_error(
                (*opt).check.errors,
                ast,
                b"a compile intrinsic must be a method body\0" as *const u8 as *const libc::c_char,
            );
            return AST_ERROR;
        }
    }
    let mut child: *mut ast_t = ast_child(parent);
    if ast_id(child) as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        child = ast_sibling(child);
    }
    let mut value: *mut ast_t = ast_child(ast);
    if child != ast
        || !(ast_sibling(child)).is_null()
        || ast_id(value) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a compile intrinsic must be the entire body\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1110:1"]
unsafe extern "C" fn syntax_compile_error(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    let mut parent: *mut ast_t = ast_parent(ast);
    if ast_id(parent) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(parent) == TK_SEQ\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1113 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"syntax_compile_error\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast_parent(parent)) as libc::c_uint != TK_IFDEF as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a compile error must be in an ifdef\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    let mut reason_seq: *mut ast_t = ast_child(ast);
    if ast_id(reason_seq) as libc::c_uint != TK_SEQ as libc::c_int as libc::c_uint
        || ast_id(ast_child(reason_seq)) as libc::c_uint != TK_STRING as libc::c_int as libc::c_uint
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a compile error must have a string literal reason for the error\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    let mut child: *mut ast_t = ast_child(parent);
    if child != ast
        || !(ast_sibling(child)).is_null()
        || ast_childcount(reason_seq) != 1 as libc::c_int as libc::c_ulong
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a compile error must be the entire ifdef clause\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1147:1"]
unsafe extern "C" fn syntax_lambda(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_LAMBDA as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_BARELAMBDA as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_LAMBDA) || (ast_id(ast) == TK_BARELAMBDA)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1149 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_lambda\0"))
                .as_ptr(),
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
    let mut reference_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 10] = [
        &mut receiver_cap,
        &mut name,
        &mut t_params,
        &mut params,
        &mut captures,
        &mut ret_type,
        &mut raises,
        &mut body,
        &mut reference_cap,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut r: bool = 1 as libc::c_int != 0;
    match ast_id(ret_type) as libc::c_uint {
        91 | 92 | 93 | 94 | 95 | 96 => {
            ast_error(
                (*opt).check.errors,
                ret_type,
                b"lambda return type: %s\0" as *const u8 as *const libc::c_char,
                ast_print_type(ret_type),
            );
            ast_error_continue(
                (*opt).check.errors,
                ret_type,
                b"lambda return type cannot be capability\0" as *const u8 as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
        _ => {}
    }
    if ast_id(ast) as libc::c_uint == TK_BARELAMBDA as libc::c_int as libc::c_uint {
        if ast_id(receiver_cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                receiver_cap,
                b"a bare lambda cannot specify a receiver capability\0" as *const u8
                    as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
        if ast_id(t_params) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                t_params,
                b"a bare lambda cannot specify type parameters\0" as *const u8
                    as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
        if ast_id(captures) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                captures,
                b"a bare lambda cannot specify captures\0" as *const u8 as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
        match ast_id(reference_cap) as libc::c_uint {
            94 | 2 => {}
            _ => {
                ast_error(
                    (*opt).check.errors,
                    reference_cap,
                    b"a bare lambda can only have a 'val' capability\0" as *const u8
                        as *const libc::c_char,
                );
                r = 0 as libc::c_int != 0;
            }
        }
    }
    let mut capture: *mut ast_t = ast_child(captures);
    while !capture.is_null() {
        if ast_id(capture) as libc::c_uint == TK_THIS as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                capture,
                b"use a named capture to capture 'this'\0" as *const u8 as *const libc::c_char,
            );
            r = 0 as libc::c_int != 0;
        }
        capture = ast_sibling(capture);
    }
    (if r as libc::c_int != 0 {
        AST_OK as libc::c_int
    } else {
        AST_ERROR as libc::c_int
    }) as ast_result_t
}
#[c2rust::src_loc = "1224:1"]
unsafe extern "C" fn syntax_object(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_OBJECT as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_OBJECT\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1226 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"syntax_object\0"))
                .as_ptr(),
        );
    };
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
    if !check_members(opt, members, 0 as libc::c_int) {
        return AST_ERROR;
    }
    if ast_id(provides) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && !check_provides_type(
            opt,
            provides,
            b"provides\0" as *const u8 as *const libc::c_char,
        )
    {
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1241:1"]
unsafe extern "C" fn syntax_fun(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_FUN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1243 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"syntax_fun\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut type_0,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    match ast_id(type_0) as libc::c_uint {
        91 | 92 | 93 | 94 | 95 | 96 => {
            ast_error(
                (*opt).check.errors,
                type_0,
                b"function return type: %s\0" as *const u8 as *const libc::c_char,
                ast_print_type(type_0),
            );
            ast_error_continue(
                (*opt).check.errors,
                type_0,
                b"function return type cannot be capability\0" as *const u8 as *const libc::c_char,
            );
            return AST_ERROR;
        }
        _ => {}
    }
    AST_OK
}
#[c2rust::src_loc = "1267:1"]
unsafe extern "C" fn syntax_cap(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    match ast_id(ast_parent(ast)) as libc::c_uint {
        151 | 17 | 78 | 79 | 80 | 107 | 106 | 89 | 90 | 88 | 71 | 72 | 73 | 74 | 75 | 76 | 77
        | 154 | 155 => return AST_OK,
        _ => {}
    }
    ast_error(
        (*opt).check.errors,
        ast,
        b"a type cannot be only a capability\0" as *const u8 as *const libc::c_char,
    );
    AST_ERROR
}
#[c2rust::src_loc = "1300:1"]
unsafe extern "C" fn syntax_cap_set(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ((*(*opt).check.frame).constraint).is_null()
        && ((*(*opt).check.frame).iftype_constraint).is_null()
    {
        ast_error(
            (*opt).check.errors,
            ast,
            b"a capability set can only appear in a type constraint\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    AST_OK
}
#[c2rust::src_loc = "1315:1"]
unsafe extern "C" fn check_annotation_location(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut loc: *mut ast_t,
    mut str: *const libc::c_char,
) -> bool {
    if strcmp(str, b"likely\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(str, b"unlikely\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        let mut parent: *mut ast_t = ast_parent(ast);
        match ast_id(parent) as libc::c_uint {
            108 | 116 | 181 => {}
            _ => {
                let mut grandparent: *mut ast_t = ast_parent(parent);
                if !(ast_id(grandparent) as libc::c_uint
                    == TK_REPEAT as libc::c_int as libc::c_uint
                    && ast_childidx(grandparent, 1 as libc::c_int as size_t) == parent)
                {
                    ast_error(
                        (*opt).check.errors,
                        loc,
                        b"a '%s' annotation can only appear on the condition of an if, while, or until, or on the case of a match\0"
                            as *const u8 as *const libc::c_char,
                        str,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        }
    } else if strcmp(str, b"packed\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if ast_id(ast_parent(ast)) as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
            ast_error(
                (*opt).check.errors,
                loc,
                b"a 'packed' annotation can only appear on a struct declaration\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
    } else if strcmp(str, b"nosupertype\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        match ast_id(ast_parent(ast)) as libc::c_uint {
            76 | 77 | 74 | 75 => {}
            _ => {
                ast_error(
                    (*opt).check.errors,
                    loc,
                    b"a 'nosupertype' annotation can only appear on a concrete type declaration\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
        }
    } else if strcmp(str, b"nodoc\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        match ast_id(ast_parent(ast)) as libc::c_uint {
            77 | 76 | 75 | 74 | 73 | 72 | 88 | 89 | 90 => {}
            _ => {
                ast_error(
                    (*opt).check.errors,
                    loc,
                    b"'nodoc' annotation isn't valid here\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1390:1"]
unsafe extern "C" fn syntax_annotation(
    mut opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_ANNOTATION as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ANNOTATION\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1392 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"syntax_annotation\0"))
                .as_ptr(),
        );
    };
    let ponyint: [libc::c_char; 8] =
        *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ponyint\0");
    let mut ok: ast_result_t = AST_OK;
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        let mut str: *const libc::c_char = ast_name(child);
        if strlen(str)
            >= (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && strncmp(
                str,
                ponyint.as_ptr(),
                (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            ast_error(
                (*opt).check.errors,
                child,
                b"annotations starting with 'ponyint' are reserved for internal use\0" as *const u8
                    as *const libc::c_char,
            );
            ok = AST_ERROR;
        } else if !check_annotation_location(opt, ast, child, str) {
            ok = AST_ERROR;
        }
        child = ast_sibling(child);
    }
    return ok;
}
#[c2rust::src_loc = "1418:1"]
unsafe extern "C" fn syntax_as(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if ast_id(ast) as libc::c_uint == TK_AS as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_AS\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1420 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"syntax_as\0")).as_ptr(),
        );
    };
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 2] = [&mut expr, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    match ast_id(expr) as libc::c_uint {
        6 | 7 => {
            ast_error(
                (*opt).check.errors,
                expr,
                b"Cannot cast uninferred numeric literal\0" as *const u8 as *const libc::c_char,
            );
            ast_error_continue(
                (*opt).check.errors,
                expr,
                b"To give a numeric literal a specific type, use the constructor of that numeric type\0"
                    as *const u8 as *const libc::c_char,
            );
            return AST_ERROR;
        }
        _ => {}
    }
    return AST_OK;
}
#[no_mangle]
#[c2rust::src_loc = "1441:1"]
pub unsafe extern "C" fn pass_syntax(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1443 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"pass_syntax\0")).as_ptr(),
        );
    };
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/syntax.c\0" as *const u8
                as *const libc::c_char,
            1445 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"pass_syntax\0")).as_ptr(),
        );
    };
    let mut id: token_id = ast_id(ast);
    let mut r: ast_result_t = AST_OK;
    match id as libc::c_uint {
        23 => {
            r = syntax_semi(options, ast);
        }
        71 => {
            r = syntax_entity(options, ast, 6 as libc::c_int);
        }
        74 => {
            r = syntax_entity(options, ast, 3 as libc::c_int);
        }
        75 => {
            r = syntax_entity(options, ast, 2 as libc::c_int);
        }
        76 => {
            r = syntax_entity(options, ast, 1 as libc::c_int);
        }
        77 => {
            r = syntax_entity(options, ast, 0 as libc::c_int);
        }
        73 => {
            r = syntax_entity(options, ast, 4 as libc::c_int);
        }
        72 => {
            r = syntax_entity(options, ast, 5 as libc::c_int);
        }
        152 => {
            r = syntax_thistype(options, ast);
        }
        17 => {
            r = syntax_arrow(options, ast);
        }
        150 => {
            r = syntax_tupletype(options, ast);
        }
        151 => {
            r = syntax_nominal(options, ast);
        }
        122 => {
            r = syntax_match(options, ast);
        }
        142 => {
            r = syntax_ffi_decl(options, ast);
        }
        143 => {
            r = syntax_ffi_call(options, ast);
        }
        63 => {
            r = syntax_ellipsis(options, ast);
        }
        106 => {
            r = syntax_consume(options, ast);
        }
        103 | 104 => {
            r = syntax_return(options, ast, 1 as libc::c_int as size_t);
        }
        105 | 127 => {
            r = syntax_return(options, ast, 0 as libc::c_int as size_t);
        }
        85 | 84 => {
            r = syntax_local(options, ast);
        }
        86 => {
            r = syntax_embed(options, ast);
        }
        163 => {
            r = syntax_type_param(options, ast);
        }
        109 => {
            r = syntax_ifdef(options, ast);
        }
        70 => {
            r = syntax_use(options, ast);
        }
        174 => {
            r = syntax_lambda_capture(options, ast);
        }
        155 => {
            r = syntax_barelambdatype(options, ast);
        }
        69 => {
            r = syntax_compile_intrinsic(options, ast);
        }
        128 => {
            r = syntax_compile_error(options, ast);
        }
        91 | 92 | 93 | 94 | 95 | 96 => {
            r = syntax_cap(options, ast);
        }
        79 | 80 => {
            r = syntax_lambda(options, ast);
        }
        78 => {
            r = syntax_object(options, ast);
        }
        89 => {
            r = syntax_fun(options, ast);
        }
        97 | 98 | 99 | 100 | 101 => {
            r = syntax_cap_set(options, ast);
        }
        205 => {
            r = syntax_annotation(options, ast);
        }
        168 | 167 => {
            ast_error(
                (*options).check.errors,
                ast,
                b"Value formal parameters not yet supported\0" as *const u8 as *const libc::c_char,
            );
            ast_error_continue(
                (*options).check.errors,
                ast_parent(ast),
                b"Note that many functions including array indexing use the apply method rather than square brackets\0"
                    as *const u8 as *const libc::c_char,
            );
            r = AST_ERROR;
        }
        64 => {
            ast_error(
                (*options).check.errors,
                ast,
                b"Compile time expressions not yet supported\0" as *const u8 as *const libc::c_char,
            );
            r = AST_ERROR;
        }
        81 => {
            r = syntax_as(options, ast);
        }
        _ => {}
    }
    if is_expr_infix(id) {
        r = syntax_infix_expr(options, ast);
    }
    if ast_checkflag(ast, AST_FLAG_MISSING_SEMI as libc::c_int as uint32_t) != 0 {
        ast_error(
            (*options).check.errors,
            ast,
            b"Use a semi colon to separate expressions on the same line\0" as *const u8
                as *const libc::c_char,
        );
        r = AST_ERROR;
    }
    let mut annotation: *mut ast_t = ast_annotation(ast);
    if !annotation.is_null() {
        let mut r2: ast_result_t = ast_visit(
            &mut annotation,
            Some(
                pass_syntax
                    as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
            ),
            None,
            options,
            PASS_SYNTAX,
        );
        if r2 as libc::c_uint > r as libc::c_uint {
            r = r2;
        }
    }
    return r;
}
