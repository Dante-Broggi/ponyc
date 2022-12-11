use ::libc;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:4"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
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
            line: size_t,
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
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::ast_h::{ast_child, ast_childidx, ast_get_children, ast_id, ast_parent, ast_ptr_t};
use self::error_h::errors_t;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::string_h::{memcpy, memset};
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
#[c2rust::src_loc = "8:1"]
unsafe extern "C" fn push_frame(mut t: *mut typecheck_t) -> bool {
    let mut f: *mut typecheck_frame_t =
        ponyint_pool_alloc(3 as libc::c_int as size_t) as *mut typecheck_frame_t;
    if !((*t).frame).is_null() {
        memcpy(
            f as *mut libc::c_void,
            (*t).frame as *const libc::c_void,
            ::core::mem::size_of::<typecheck_frame_t>() as libc::c_ulong,
        );
        let ref mut fresh0 = (*f).prev;
        *fresh0 = (*t).frame;
    } else {
        memset(
            f as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<typecheck_frame_t>() as libc::c_ulong,
        );
    }
    let ref mut fresh1 = (*t).frame;
    *fresh1 = f;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "24:1"]
pub unsafe extern "C" fn frame_push(mut t: *mut typecheck_t, mut ast: *mut ast_t) -> bool {
    let mut pop: bool = 0 as libc::c_int != 0;
    if ast.is_null() {
        let mut f: *mut typecheck_frame_t =
            ponyint_pool_alloc(3 as libc::c_int as size_t) as *mut typecheck_frame_t;
        memset(
            f as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<typecheck_frame_t>() as libc::c_ulong,
        );
        let ref mut fresh2 = (*f).prev;
        *fresh2 = (*t).frame;
        let ref mut fresh3 = (*t).frame;
        *fresh3 = f;
        return 1 as libc::c_int != 0;
    }
    match ast_id(ast) as libc::c_uint {
        136 => {
            pop = push_frame(t);
            let ref mut fresh4 = (*(*t).frame).package;
            *fresh4 = 0 as *mut ast_t;
            let ref mut fresh5 = (*(*t).frame).module;
            *fresh5 = 0 as *mut ast_t;
        }
        137 => {
            pop = push_frame(t);
            let ref mut fresh6 = (*(*t).frame).package;
            *fresh6 = ast;
            let ref mut fresh7 = (*(*t).frame).module;
            *fresh7 = 0 as *mut ast_t;
        }
        138 => {
            pop = push_frame(t);
            let ref mut fresh8 = (*(*t).frame).module;
            *fresh8 = ast;
            let ref mut fresh9 = (*(*t).frame).ifdef_cond;
            *fresh9 = 0 as *mut ast_t;
            let ref mut fresh10 = (*(*t).frame).ifdef_clause;
            *fresh10 = ast;
        }
        72 | 73 | 74 | 75 | 76 | 77 => {
            pop = push_frame(t);
            let ref mut fresh11 = (*(*t).frame).type_0;
            *fresh11 = ast;
        }
        148 => {
            pop = push_frame(t);
            let ref mut fresh12 = (*(*t).frame).provides;
            *fresh12 = ast;
        }
        88 | 90 | 89 => {
            pop = push_frame(t);
            let ref mut fresh13 = (*(*t).frame).method;
            *fresh13 = ast;
        }
        166 => {
            pop = push_frame(t);
            let ref mut fresh14 = (*(*t).frame).method_type;
            *fresh14 = 0 as *mut ast_t;
            let ref mut fresh15 = (*(*t).frame).ffi_type;
            *fresh15 = 0 as *mut ast_t;
            let ref mut fresh16 = (*(*t).frame).local_type;
            *fresh16 = 0 as *mut ast_t;
            let ref mut fresh17 = (*(*t).frame).constraint;
            *fresh17 = 0 as *mut ast_t;
            let ref mut fresh18 = (*(*t).frame).iftype_constraint;
            *fresh18 = 0 as *mut ast_t;
        }
        181 => {
            pop = push_frame(t);
            let ref mut fresh19 = (*(*t).frame).the_case;
            *fresh19 = ast;
        }
        116 | 118 => {
            pop = push_frame(t);
            let ref mut fresh20 = (*(*t).frame).loop_0;
            *fresh20 = ast;
        }
        124 | 125 => {
            pop = push_frame(t);
            let ref mut fresh21 = (*(*t).frame).try_expr;
            *fresh21 = ast;
        }
        206 => {
            pop = push_frame(t);
        }
        107 => {
            pop = push_frame(t);
            let ref mut fresh22 = (*(*t).frame).recover;
            *fresh22 = ast;
        }
        _ => {
            let mut parent: *mut ast_t = ast_parent(ast);
            if parent.is_null() {
                return pop;
            }
            match ast_id(parent) as libc::c_uint {
                163 => {
                    let mut id: ast_ptr_t = 0 as *mut ast_t;
                    let mut constraint: ast_ptr_t = 0 as *mut ast_t;
                    let mut def_type: ast_ptr_t = 0 as *mut ast_t;
                    let mut children: [*mut *mut ast_t; 4] = [
                        &mut id,
                        &mut constraint,
                        &mut def_type,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children.as_mut_ptr(),
                    );
                    if constraint == ast {
                        pop = push_frame(t);
                        let ref mut fresh23 = (*(*t).frame).constraint;
                        *fresh23 = ast;
                    }
                }
                88 | 90 | 89 => {
                    let mut cap: ast_ptr_t = 0 as *mut ast_t;
                    let mut id_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
                    let mut params: ast_ptr_t = 0 as *mut ast_t;
                    let mut result: ast_ptr_t = 0 as *mut ast_t;
                    let mut error: ast_ptr_t = 0 as *mut ast_t;
                    let mut body: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_0: [*mut *mut ast_t; 8] = [
                        &mut cap,
                        &mut id_0,
                        &mut typeparams,
                        &mut params,
                        &mut result,
                        &mut error,
                        &mut body,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_0.as_mut_ptr(),
                    );
                    if params == ast {
                        pop = push_frame(t);
                        let ref mut fresh24 = (*(*t).frame).method_params;
                        *fresh24 = ast;
                    } else if result == ast {
                        pop = push_frame(t);
                        let ref mut fresh25 = (*(*t).frame).method_type;
                        *fresh25 = ast;
                    } else if body == ast {
                        pop = push_frame(t);
                        let ref mut fresh26 = (*(*t).frame).method_body;
                        *fresh26 = ast;
                    }
                }
                166 => match ast_id(ast_parent(parent)) as libc::c_uint {
                    142 | 143 => {
                        pop = push_frame(t);
                        let ref mut fresh27 = (*(*t).frame).ffi_type;
                        *fresh27 = ast;
                    }
                    _ => {}
                },
                165 => {
                    let mut id_1: ast_ptr_t = 0 as *mut ast_t;
                    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut def_arg: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_1: [*mut *mut ast_t; 4] =
                        [&mut id_1, &mut type_0, &mut def_arg, 0 as *mut *mut ast_t];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_1.as_mut_ptr(),
                    );
                    if type_0 == ast {
                        pop = push_frame(t);
                        let ref mut fresh28 = (*(*t).frame).local_type;
                        *fresh28 = ast;
                    } else if def_arg == ast {
                        pop = push_frame(t);
                        let ref mut fresh29 = (*(*t).frame).def_arg;
                        *fresh29 = ast;
                    }
                }
                84 | 85 => {
                    if ast_childidx(parent, 1 as libc::c_int as size_t) == ast {
                        pop = push_frame(t);
                        let ref mut fresh30 = (*(*t).frame).local_type;
                        *fresh30 = ast;
                    }
                }
                181 => {
                    if ast_child(parent) == ast {
                        pop = push_frame(t);
                        let ref mut fresh31 = (*(*t).frame).pattern;
                        *fresh31 = ast;
                    }
                }
                81 => {
                    let mut expr: ast_ptr_t = 0 as *mut ast_t;
                    let mut type_1: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_2: [*mut *mut ast_t; 3] =
                        [&mut expr, &mut type_1, 0 as *mut *mut ast_t];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_2.as_mut_ptr(),
                    );
                    if type_1 == ast {
                        pop = push_frame(t);
                        let ref mut fresh32 = (*(*t).frame).as_type;
                        *fresh32 = ast;
                    }
                }
                116 => {
                    let mut cond: ast_ptr_t = 0 as *mut ast_t;
                    let mut body_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_3: [*mut *mut ast_t; 4] = [
                        &mut cond,
                        &mut body_0,
                        &mut else_clause,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_3.as_mut_ptr(),
                    );
                    if cond == ast {
                        pop = push_frame(t);
                        let ref mut fresh33 = (*(*t).frame).loop_cond;
                        *fresh33 = ast;
                    } else if body_0 == ast {
                        pop = push_frame(t);
                        let ref mut fresh34 = (*(*t).frame).loop_body;
                        *fresh34 = ast;
                    } else if else_clause == ast {
                        pop = push_frame(t);
                        let ref mut fresh35 = (*(*t).frame).loop_else;
                        *fresh35 = ast;
                    }
                }
                118 => {
                    let mut body_1: ast_ptr_t = 0 as *mut ast_t;
                    let mut cond_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut else_clause_0: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_4: [*mut *mut ast_t; 4] = [
                        &mut body_1,
                        &mut cond_0,
                        &mut else_clause_0,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_4.as_mut_ptr(),
                    );
                    if body_1 == ast {
                        pop = push_frame(t);
                        let ref mut fresh36 = (*(*t).frame).loop_body;
                        *fresh36 = ast;
                    } else if cond_0 == ast {
                        pop = push_frame(t);
                        let ref mut fresh37 = (*(*t).frame).loop_cond;
                        *fresh37 = ast;
                    } else if else_clause_0 == ast {
                        pop = push_frame(t);
                        let ref mut fresh38 = (*(*t).frame).loop_else;
                        *fresh38 = ast;
                    }
                }
                109 => {
                    let mut cond_1: ast_ptr_t = 0 as *mut ast_t;
                    let mut body_2: ast_ptr_t = 0 as *mut ast_t;
                    let mut else_clause_1: ast_ptr_t = 0 as *mut ast_t;
                    let mut else_cond: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_5: [*mut *mut ast_t; 5] = [
                        &mut cond_1,
                        &mut body_2,
                        &mut else_clause_1,
                        &mut else_cond,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_5.as_mut_ptr(),
                    );
                    if body_2 == ast {
                        pop = push_frame(t);
                        let ref mut fresh39 = (*(*t).frame).ifdef_cond;
                        *fresh39 = cond_1;
                        let ref mut fresh40 = (*(*t).frame).ifdef_clause;
                        *fresh40 = body_2;
                    } else if else_clause_1 == ast {
                        pop = push_frame(t);
                        let ref mut fresh41 = (*(*t).frame).ifdef_cond;
                        *fresh41 = else_cond;
                        let ref mut fresh42 = (*(*t).frame).ifdef_clause;
                        *fresh42 = else_clause_1;
                    }
                }
                110 => {
                    let mut l_type: ast_ptr_t = 0 as *mut ast_t;
                    let mut r_type: ast_ptr_t = 0 as *mut ast_t;
                    let mut body_3: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_6: [*mut *mut ast_t; 4] =
                        [&mut l_type, &mut r_type, &mut body_3, 0 as *mut *mut ast_t];
                    ast_get_children(
                        parent,
                        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children_6.as_mut_ptr(),
                    );
                    pop = push_frame(t);
                    if r_type == ast {
                        let ref mut fresh43 = (*(*t).frame).iftype_constraint;
                        *fresh43 = ast;
                    } else if body_3 == ast {
                        let ref mut fresh44 = (*(*t).frame).iftype_body;
                        *fresh44 = ast;
                    }
                }
                _ => {}
            }
        }
    }
    return pop;
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn frame_pop(mut t: *mut typecheck_t) {
    let mut f: *mut typecheck_frame_t = (*t).frame;
    if !f.is_null() {
    } else {
        ponyint_assert_fail(
            b"f != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/frame.c\0" as *const u8
                as *const libc::c_char,
            297 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"frame_pop\0")).as_ptr(),
        );
    };
    let ref mut fresh45 = (*t).frame;
    *fresh45 = (*f).prev;
    ponyint_pool_free(3 as libc::c_int as size_t, f as *mut libc::c_void);
}