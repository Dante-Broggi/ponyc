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
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexint.h:1"]
pub mod lexint_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct lexint_t {
        pub low: u64,
        pub high: u64,
    }
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn lexint_cmp(a: *const lexint_t, b: *const lexint_t) -> libc::c_int;
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
    use super::error_h::errors_t;
    use super::lexint_h::lexint_t;
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
        #[c2rust::src_loc = "67:1"]
        pub fn ast_has_scope(ast: *mut ast_t) -> bool;
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
        pub fn ast_checkflag(ast: *mut ast_t, flag: u32) -> libc::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: u32);
        #[c2rust::src_loc = "90:1"]
        pub fn ast_clearflag(ast: *mut ast_t, flag: u32);
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn ast_name_len(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "97:1"]
        pub fn ast_set_name(ast: *mut ast_t, name: *const libc::c_char);
        #[c2rust::src_loc = "98:1"]
        pub fn ast_float(ast: *mut ast_t) -> libc::c_double;
        #[c2rust::src_loc = "99:1"]
        pub fn ast_int(ast: *mut ast_t) -> *mut lexint_t;
        #[c2rust::src_loc = "101:1"]
        pub fn ast_settype(ast: *mut ast_t, type_0: *mut ast_t);
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
        #[c2rust::src_loc = "121:1"]
        pub fn ast_get_case(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "122:1"]
        pub fn ast_set(
            ast: *mut ast_t,
            name: *const libc::c_char,
            value: *mut ast_t,
            status: sym_status_t,
            allow_shadowing: bool,
        ) -> bool;
        #[c2rust::src_loc = "134:1"]
        pub fn ast_clear_local(ast: *mut ast_t);
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
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
        #[c2rust::src_loc = "390:1"]
        pub fn ast_passes_subtree(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.h:2"]
pub mod sugar_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn has_member(members: *mut ast_t, name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:5"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn is_name_dontcare(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:9"]
pub mod reify_h {
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:10"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:11"]
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
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_checkflag, ast_child, ast_childidx, ast_clear_local,
    ast_clearflag, ast_data, ast_error, ast_error_continue, ast_float, ast_free_unattached,
    ast_from, ast_from_string, ast_get, ast_get_case, ast_get_children, ast_has_scope, ast_id,
    ast_inheritflags, ast_int, ast_name, ast_name_len, ast_print_type, ast_ptr_t, ast_replace,
    ast_result_t, ast_scope, ast_set, ast_set_name, ast_setdata, ast_setflag, ast_setid,
    ast_settype, ast_sibling, C2RustUnnamed, AST_ERROR, AST_FATAL, AST_FLAG_AMBIGUOUS,
    AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND, AST_FLAG_CNSM_REASGN,
    AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2, AST_FLAG_FCNSM_REASGN,
    AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS, AST_FLAG_JUMPS_AWAY,
    AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI, AST_FLAG_PASS_MASK,
    AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2, AST_IGNORE, AST_OK,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::is_name_dontcare;
pub use self::lexint_h::{lexint_cmp, lexint_t};
pub use self::pass_h::{
    ast_passes_subtree, ast_visit, ast_visit_t, magic_package_t, pass_id, pass_opt_t, plugins_t,
    verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR,
    PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ,
    PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR,
    PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL,
    VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
use self::reify_h::{reify, reify_defaults};
use self::stringtab_h::stringtab;
use self::sugar_h::has_member;
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "54:16"]
pub struct method_t {
    pub body_donor: *mut ast_t,
    pub trait_ref: *mut ast_t,
    pub local_define: bool,
    pub failed: bool,
}
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn is_method(mut ast: *mut ast_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            69 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"is_method\0")).as_ptr(),
        );
    };
    let mut variety: token_id = ast_id(ast);
    variety as libc::c_uint == TK_BE as libc::c_int as libc::c_uint
        || variety as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
        || variety as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn attach_method_t(mut method: *mut ast_t) -> *mut method_t {
    if !method.is_null() {
    } else {
        ponyint_assert_fail(
            b"method != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            79 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"attach_method_t\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut method_t = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut method_t;
    let ref mut fresh0 = (*p).body_donor;
    *fresh0 = 0 as *mut ast_t;
    let ref mut fresh1 = (*p).trait_ref;
    *fresh1 = 0 as *mut ast_t;
    (*p).local_define = 0 as libc::c_int != 0;
    (*p).failed = 0 as libc::c_int != 0;
    ast_setdata(method, p as *mut libc::c_void);
    p
}
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn setup_local_methods(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            95 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"setup_local_methods\0"))
                .as_ptr(),
        );
    };
    let mut members: *mut ast_t = ast_childidx(ast, 4 as libc::c_int as usize);
    if !members.is_null() {
    } else {
        ponyint_assert_fail(
            b"members != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            98 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"setup_local_methods\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut ast_t = ast_child(members);
    while !p.is_null() {
        if is_method(p) {
            let mut info: *mut method_t = attach_method_t(p);
            (*info).local_define = 1 as libc::c_int != 0;
            if ast_id(ast_childidx(p, 6 as libc::c_int as usize)) as libc::c_uint
                != TK_NONE as libc::c_int as libc::c_uint
            {
                let ref mut fresh2 = (*info).body_donor;
                *fresh2 = ast;
            }
        }
        p = ast_sibling(p);
    }
}
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn tidy_up(mut entity: *mut ast_t) {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            117 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"tidy_up\0")).as_ptr(),
        );
    };
    let mut members: *mut ast_t = ast_childidx(entity, 4 as libc::c_int as usize);
    if !members.is_null() {
    } else {
        ponyint_assert_fail(
            b"members != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            120 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"tidy_up\0")).as_ptr(),
        );
    };
    let mut p: *mut ast_t = ast_child(members);
    while !p.is_null() {
        if is_method(p) {
            let mut info: *mut method_t = ast_data(p) as *mut method_t;
            if !info.is_null() {
            } else {
                ponyint_assert_fail(
                    b"info != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    127 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"tidy_up\0"))
                        .as_ptr(),
                );
            };
            let mut body_donor: *mut ast_t = (*info).body_donor;
            ponyint_pool_free(0 as libc::c_int as usize, info as *mut libc::c_void);
            if body_donor.is_null() {
                body_donor = entity;
            }
            ast_setdata(p, body_donor as *mut libc::c_void);
        }
        p = ast_sibling(p);
    }
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn compare_signatures(mut sig_a: *mut ast_t, mut sig_b: *mut ast_t) -> bool {
    if sig_a.is_null() && sig_b.is_null() {
        return 1 as libc::c_int != 0;
    }
    if sig_a.is_null() || sig_b.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut a_id: token_id = ast_id(sig_a);
    if a_id as libc::c_uint != ast_id(sig_b) as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    match a_id as libc::c_uint {
        90 | 89 | 88 => {
            let mut a_child: *mut ast_t = ast_child(sig_a);
            let mut b_child: *mut ast_t = ast_child(sig_b);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                if a_child.is_null() || b_child.is_null() {
                    return 0 as libc::c_int != 0;
                }
                if !compare_signatures(a_child, b_child) {
                    return 0 as libc::c_int != 0;
                }
                a_child = ast_sibling(a_child);
                b_child = ast_sibling(b_child);
                i += 1;
            }
            return 1 as libc::c_int != 0;
        }
        5 | 8 => {
            let mut a_len: usize = ast_name_len(sig_a);
            let mut b_len: usize = ast_name_len(sig_b);
            if a_len != b_len {
                return 0 as libc::c_int != 0;
            }
            let mut a_text: *const libc::c_char = ast_name(sig_a);
            let mut b_text: *const libc::c_char = ast_name(sig_b);
            let mut i_0: usize = 0;
            while i_0 < a_len {
                if *a_text.offset(i_0 as isize) as libc::c_int
                    != *b_text.offset(i_0 as isize) as libc::c_int
                {
                    return 0 as libc::c_int != 0;
                }
                i_0 = i_0.wrapping_add(1);
            }
            return 1 as libc::c_int != 0;
        }
        6 => return lexint_cmp(ast_int(sig_a), ast_int(sig_b)) == 0 as libc::c_int,
        7 => return ast_float(sig_a) == ast_float(sig_b),
        151 => {
            if ast_data(sig_a) != ast_data(sig_b) {
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    let mut a_child_0: *mut ast_t = ast_child(sig_a);
    let mut b_child_0: *mut ast_t = ast_child(sig_b);
    while !a_child_0.is_null() && !b_child_0.is_null() {
        if !compare_signatures(a_child_0, b_child_0) {
            return 0 as libc::c_int != 0;
        }
        a_child_0 = ast_sibling(a_child_0);
        b_child_0 = ast_sibling(b_child_0);
    }
    if !a_child_0.is_null() || !b_child_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "238:1"]
unsafe extern "C" fn reify_provides_type(
    mut method: *mut ast_t,
    mut trait_ref: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> *mut ast_t {
    if !method.is_null() {
    } else {
        ponyint_assert_fail(
            b"method != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            241 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"reify_provides_type\0"))
                .as_ptr(),
        );
    };
    if !trait_ref.is_null() {
    } else {
        ponyint_assert_fail(
            b"trait_ref != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            242 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"reify_provides_type\0"))
                .as_ptr(),
        );
    };
    let mut trait_def: *mut ast_t = ast_data(trait_ref) as *mut ast_t;
    if !trait_def.is_null() {
    } else {
        ponyint_assert_fail(
            b"trait_def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            247 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"reify_provides_type\0"))
                .as_ptr(),
        );
    };
    let mut type_args: *mut ast_t = ast_childidx(trait_ref, 2 as libc::c_int as usize);
    let mut type_params: *mut ast_t = ast_childidx(trait_def, 1 as libc::c_int as usize);
    if !reify_defaults(type_params, type_args, 1 as libc::c_int != 0, opt) {
        return 0 as *mut ast_t;
    }
    let mut reified: *mut ast_t = reify(method, type_params, type_args, opt, 1 as libc::c_int != 0);
    if reified.is_null() {
        return 0 as *mut ast_t;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut doc: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        &mut doc,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        reified,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    return reified;
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn find_method(
    mut entity: *mut ast_t,
    mut name: *const libc::c_char,
) -> *mut ast_t {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            271 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"find_method\0")).as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"find_method\0")).as_ptr(),
        );
    };
    let mut method: *mut ast_t = ast_get(entity, name, 0 as *mut sym_status_t);
    if method.is_null() {
        return 0 as *mut ast_t;
    }
    if is_method(method) {
        return method;
    }
    return 0 as *mut ast_t;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn add_method(
    mut entity: *mut ast_t,
    mut trait_ref: *mut ast_t,
    mut basis_method: *mut ast_t,
    mut adjective: *const libc::c_char,
    mut opt: *mut pass_opt_t,
) -> *mut ast_t {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            297 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_method\0")).as_ptr(),
        );
    };
    if !trait_ref.is_null() {
    } else {
        ponyint_assert_fail(
            b"trait_ref != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            298 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_method\0")).as_ptr(),
        );
    };
    if !basis_method.is_null() {
    } else {
        ponyint_assert_fail(
            b"basis_method != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            299 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_method\0")).as_ptr(),
        );
    };
    if !adjective.is_null() {
    } else {
        ponyint_assert_fail(
            b"adjective != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            300 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_method\0")).as_ptr(),
        );
    };
    let mut name: *const libc::c_char =
        ast_name(ast_childidx(basis_method, 1 as libc::c_int as usize));
    if ast_id(basis_method) as libc::c_uint == TK_BE as libc::c_int as libc::c_uint {
        match ast_id(entity) as libc::c_uint {
            74 => {
                ast_error(
                    (*opt).check.errors,
                    trait_ref,
                    b"cannot add a behaviour (%s) to a primitive\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
                return 0 as *mut ast_t;
            }
            75 => {
                ast_error(
                    (*opt).check.errors,
                    trait_ref,
                    b"cannot add a behaviour (%s) to a struct\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
                return 0 as *mut ast_t;
            }
            76 => {
                ast_error(
                    (*opt).check.errors,
                    trait_ref,
                    b"cannot add a behaviour (%s) to a class\0" as *const u8 as *const libc::c_char,
                    name,
                );
                return 0 as *mut ast_t;
            }
            _ => {}
        }
    }
    let mut existing: *mut ast_t = ast_get(entity, name, 0 as *mut sym_status_t);
    if !existing.is_null() {
        let mut variety: token_id = ast_id(existing);
        if variety as libc::c_uint == TK_FVAR as libc::c_int as libc::c_uint
            || variety as libc::c_uint == TK_FLET as libc::c_int as libc::c_uint
            || variety as libc::c_uint == TK_EMBED as libc::c_int as libc::c_uint
        {
        } else {
            ponyint_assert_fail(
                b"(variety == TK_FVAR) || (variety == TK_FLET) || (variety == TK_EMBED)\0"
                    as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                    as *const u8 as *const libc::c_char,
                337 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_method\0"))
                    .as_ptr(),
            );
        };
        ast_error(
            (*opt).check.errors,
            trait_ref,
            b"%s method '%s' clashes with field\0" as *const u8 as *const libc::c_char,
            adjective,
            name,
        );
        ast_error_continue(
            (*opt).check.errors,
            basis_method,
            b"method is defined here\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut ast_t;
    }
    let mut case_clash: *mut ast_t = ast_get_case(entity, name, 0 as *mut sym_status_t);
    if !case_clash.is_null() {
        let mut clash_name: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
        match ast_id(case_clash) as libc::c_uint {
            89 | 90 | 88 => {
                clash_name = ast_name(ast_childidx(case_clash, 1 as libc::c_int as usize));
            }
            85 | 84 | 86 => {
                clash_name = ast_name(ast_child(case_clash));
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                            as *const u8 as *const libc::c_char,
                        368 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(
                            b"add_method\0",
                        ))
                        .as_ptr(),
                    );
                };
            }
        }
        ast_error(
            (*opt).check.errors,
            trait_ref,
            b"%s method '%s' differs only in case from '%s'\0" as *const u8 as *const libc::c_char,
            adjective,
            name,
            clash_name,
        );
        ast_error_continue(
            (*opt).check.errors,
            basis_method,
            b"clashing method is defined here\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut ast_t;
    }
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut doc: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        &mut doc,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        basis_method,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if ast_id(doc) as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        ast_set_name(doc, b"\0" as *const u8 as *const libc::c_char);
        ast_setid(doc, TK_NONE);
        ast_settype(doc, 0 as *mut ast_t);
    }
    let mut local: *mut ast_t = ast_append(
        ast_childidx(entity, 4 as libc::c_int as usize),
        basis_method,
    );
    ast_set(entity, name, local, SYM_DEFINED, 0 as libc::c_int != 0);
    let mut body_donor: *mut ast_t = ast_data(basis_method) as *mut ast_t;
    let mut info: *mut method_t = attach_method_t(local);
    let ref mut fresh3 = (*info).trait_ref;
    *fresh3 = trait_ref;
    if ast_id(body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        let ref mut fresh4 = (*info).body_donor;
        *fresh4 = body_donor;
    }
    return local;
}
#[c2rust::src_loc = "405:1"]
unsafe extern "C" fn rescope(
    mut astp: *mut *mut ast_t,
    mut _options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    if ast_has_scope(ast) {
        ast_clear_local(ast);
    }
    match ast_id(ast) as libc::c_uint {
        140 | 141 | 86 | 165 | 182 => {
            if !(ast_child(ast)).is_null() {
            } else {
                ponyint_assert_fail(
                    b"ast_child(ast) != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    421 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"rescope\0"))
                        .as_ptr(),
                );
            };
            ast_set(
                ast,
                ast_name(ast_child(ast)),
                ast,
                SYM_DEFINED,
                1 as libc::c_int != 0,
            );
        }
        85 | 84 => {
            if !(ast_child(ast)).is_null() {
            } else {
                ponyint_assert_fail(
                    b"ast_child(ast) != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    429 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"rescope\0"))
                        .as_ptr(),
                );
            };
            ast_set(
                ast,
                ast_name(ast_child(ast)),
                ast,
                SYM_UNDEFINED,
                1 as libc::c_int != 0,
            );
        }
        162 => {
            let mut typeparam: *mut ast_t = ast_child(ast);
            while !typeparam.is_null() {
                if !(ast_child(typeparam)).is_null() {
                } else {
                    ponyint_assert_fail(
                        b"ast_child(typeparam) != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                            as *const u8 as *const libc::c_char,
                        439 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"rescope\0"))
                            .as_ptr(),
                    );
                };
                ast_set(
                    ast,
                    ast_name(ast_child(typeparam)),
                    typeparam,
                    SYM_DEFINED,
                    1 as libc::c_int != 0,
                );
                typeparam = ast_sibling(typeparam);
            }
        }
        187 => {
            if !(ast_child(ast)).is_null() {
            } else {
                ponyint_assert_fail(
                    b"ast_child(ast) != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    449 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"rescope\0"))
                        .as_ptr(),
                );
            };
            let mut def: *mut ast_t =
                ast_get(ast, ast_name(ast_child(ast)), 0 as *mut sym_status_t);
            ast_setdata(ast, def as *mut libc::c_void);
        }
        _ => {}
    }
    AST_OK
}
#[c2rust::src_loc = "469:1"]
unsafe extern "C" fn add_method_from_trait(
    mut entity: *mut ast_t,
    mut method: *mut ast_t,
    mut trait_ref: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            472 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"add_method_from_trait\0"))
                .as_ptr(),
        );
    };
    if !method.is_null() {
    } else {
        ponyint_assert_fail(
            b"method != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            473 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"add_method_from_trait\0"))
                .as_ptr(),
        );
    };
    if !trait_ref.is_null() {
    } else {
        ponyint_assert_fail(
            b"trait_ref != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            474 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"add_method_from_trait\0"))
                .as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut t_params: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut error: ast_ptr_t = 0 as *mut ast_t;
    let mut method_body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut t_params,
        &mut params,
        &mut result,
        &mut error,
        &mut method_body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        method,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut method_name: *const libc::c_char = ast_name(id);
    let mut existing_method: *mut ast_t = find_method(entity, method_name);
    if existing_method.is_null() {
        let mut m: *mut ast_t = add_method(
            entity,
            trait_ref,
            method,
            b"provided\0" as *const u8 as *const libc::c_char,
            opt,
        );
        if m.is_null() {
            return 0 as libc::c_int != 0;
        }
        if ast_id(ast_childidx(m, 6 as libc::c_int as usize)) as libc::c_uint
            != TK_NONE as libc::c_int as libc::c_uint
        {
            ast_visit(
                &mut m,
                Some(
                    rescope
                        as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t,
                ),
                None,
                opt,
                PASS_ALL,
            );
        }
        return 1 as libc::c_int != 0;
    }
    let mut info: *mut method_t = ast_data(existing_method) as *mut method_t;
    if !info.is_null() {
    } else {
        ponyint_assert_fail(
            b"info != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            498 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"add_method_from_trait\0"))
                .as_ptr(),
        );
    };
    if (*info).failed {
        return 0 as libc::c_int != 0;
    }
    if (*info).local_define {
        return 1 as libc::c_int != 0;
    }
    if !compare_signatures(existing_method, method) {
        if !((*info).trait_ref).is_null() {
        } else {
            ponyint_assert_fail(
                b"info->trait_ref != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                    as *const u8 as *const libc::c_char,
                510 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"add_method_from_trait\0",
                ))
                .as_ptr(),
            );
        };
        ast_error(
            (*opt).check.errors,
            trait_ref,
            b"clashing definitions for method '%s' provided, local disambiguation required\0"
                as *const u8 as *const libc::c_char,
            method_name,
        );
        ast_error_continue(
            (*opt).check.errors,
            trait_ref,
            b"provided here, type: %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(method),
        );
        ast_error_continue(
            (*opt).check.errors,
            (*info).trait_ref,
            b"and here, type: %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(existing_method),
        );
        (*info).failed = 1 as libc::c_int != 0;
        return 0 as libc::c_int != 0;
    }
    let mut existing_body: *mut ast_t = ast_childidx(existing_method, 6 as libc::c_int as usize);
    let mut multiple_bodies: bool = !((*info).body_donor).is_null()
        && ast_id(method_body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
        && (*info).body_donor != ast_data(method) as *mut ast_t;
    if multiple_bodies as libc::c_int != 0
        || ast_checkflag(existing_method, AST_FLAG_AMBIGUOUS as libc::c_int as u32) != 0
        || ast_checkflag(method, AST_FLAG_AMBIGUOUS as libc::c_int as u32) != 0
    {
        ast_setflag(existing_method, AST_FLAG_AMBIGUOUS as libc::c_int as u32);
        if ast_id(existing_body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            ast_replace(&mut existing_body, ast_from(existing_method, TK_NONE));
        }
        let ref mut fresh5 = (*info).body_donor;
        *fresh5 = 0 as *mut ast_t;
        return 1 as libc::c_int != 0;
    }
    if ast_id(method_body) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
        || (*info).body_donor == ast_data(method) as *mut ast_t
    {
        return 1 as libc::c_int != 0;
    }
    if ast_id(existing_body) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(existing_body) == TK_NONE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            555 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"add_method_from_trait\0"))
                .as_ptr(),
        );
    };
    ast_replace(&mut existing_body, method_body);
    ast_visit(
        &mut existing_body,
        Some(rescope as unsafe extern "C" fn(*mut *mut ast_t, *mut pass_opt_t) -> ast_result_t),
        None,
        opt,
        PASS_ALL,
    );
    let ref mut fresh6 = (*info).body_donor;
    *fresh6 = ast_data(method) as *mut ast_t;
    let ref mut fresh7 = (*info).trait_ref;
    *fresh7 = trait_ref;
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn provided_methods(mut entity: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            569 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"provided_methods\0"))
                .as_ptr(),
        );
    };
    let mut provides: *mut ast_t = ast_childidx(entity, 3 as libc::c_int as usize);
    let mut r: bool = 1 as libc::c_int != 0;
    let mut trait_ref: *mut ast_t = ast_child(provides);
    while !trait_ref.is_null() {
        let mut trait_0: *mut ast_t = ast_data(trait_ref) as *mut ast_t;
        if !trait_0.is_null() {
        } else {
            ponyint_assert_fail(
                b"trait != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                    as *const u8 as *const libc::c_char,
                579 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"provided_methods\0"))
                    .as_ptr(),
            );
        };
        if !trait_entity(trait_0, opt) {
            return 0 as libc::c_int != 0;
        }
        let mut members: *mut ast_t = ast_childidx(trait_0, 4 as libc::c_int as usize);
        let mut method: *mut ast_t = ast_child(members);
        while !method.is_null() {
            if is_method(method) as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"is_method(method)\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    590 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"provided_methods\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut reified: *mut ast_t = reify_provides_type(method, trait_ref, opt);
            if reified.is_null() {
                r = 0 as libc::c_int != 0;
            } else {
                if !add_method_from_trait(entity, reified, trait_ref, opt) {
                    r = 0 as libc::c_int != 0;
                }
                ast_free_unattached(reified);
            }
            method = ast_sibling(method);
        }
        trait_ref = ast_sibling(trait_ref);
    }
    return r;
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn check_concrete_bodies(
    mut entity: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            616 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"check_concrete_bodies\0"))
                .as_ptr(),
        );
    };
    let mut variety: token_id = ast_id(entity);
    if variety as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint
        && variety as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint
        && variety as libc::c_uint != TK_CLASS as libc::c_int as libc::c_uint
        && variety as libc::c_uint != TK_ACTOR as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    let mut r: bool = 1 as libc::c_int != 0;
    let mut members: *mut ast_t = ast_childidx(entity, 4 as libc::c_int as usize);
    if !members.is_null() {
    } else {
        ponyint_assert_fail(
            b"members != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            625 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"check_concrete_bodies\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut ast_t = ast_child(members);
    while !p.is_null() {
        if is_method(p) {
            let mut info: *mut method_t = ast_data(p) as *mut method_t;
            if !info.is_null() {
            } else {
                ponyint_assert_fail(
                    b"info != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    632 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"check_concrete_bodies\0",
                    ))
                    .as_ptr(),
                );
            };
            if !(*info).failed {
                let mut name: *const libc::c_char =
                    ast_name(ast_childidx(p, 1 as libc::c_int as usize));
                if ast_checkflag(p, AST_FLAG_AMBIGUOUS as libc::c_int as u32) != 0 {
                    ast_error(
                        (*opt).check.errors,
                        entity,
                        b"multiple possible bodies for method %s, local disambiguation required\0"
                            as *const u8 as *const libc::c_char,
                        name,
                    );
                    r = 0 as libc::c_int != 0;
                } else if ((*info).body_donor).is_null() {
                    if !((*info).trait_ref).is_null() {
                    } else {
                        ponyint_assert_fail(
                            b"info->trait_ref != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                                as *const u8 as *const libc::c_char,
                            648 as libc::c_int as usize,
                            (*::core::mem::transmute::<
                                &[u8; 22],
                                &[libc::c_char; 22],
                            >(b"check_concrete_bodies\0"))
                                .as_ptr(),
                        );
                    };
                    ast_error(
                        (*opt).check.errors,
                        (*info).trait_ref,
                        b"no body found for method '%s'\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    r = 0 as libc::c_int != 0;
                }
            }
        }
        p = ast_sibling(p);
    }
    r
}
#[c2rust::src_loc = "662:1"]
unsafe extern "C" fn trait_entity(mut entity: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            664 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"trait_entity\0")).as_ptr(),
        );
    };
    let mut state: libc::c_int = ast_checkflag(
        entity,
        (AST_FLAG_RECURSE_1 as libc::c_int
            | AST_FLAG_DONE_1 as libc::c_int
            | AST_FLAG_ERROR_1 as libc::c_int) as u32,
    );
    match state {
        0 => {
            ast_setflag(entity, AST_FLAG_RECURSE_1 as libc::c_int as u32);
        }
        16384 => {
            ast_error(
                (*opt).check.errors,
                entity,
                b"traits and interfaces can't be recursive\0" as *const u8 as *const libc::c_char,
            );
            ast_clearflag(entity, AST_FLAG_RECURSE_1 as libc::c_int as u32);
            ast_setflag(entity, AST_FLAG_ERROR_1 as libc::c_int as u32);
            return 0 as libc::c_int != 0;
        }
        32768 => return 1 as libc::c_int != 0,
        65536 | 98304 => return 0 as libc::c_int != 0,
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    691 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"trait_entity\0"))
                        .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    setup_local_methods(entity);
    let mut r: bool = provided_methods(entity, opt) as libc::c_int != 0
        && check_concrete_bodies(entity, opt) as libc::c_int != 0;
    tidy_up(entity);
    ast_clearflag(entity, AST_FLAG_RECURSE_1 as libc::c_int as u32);
    ast_setflag(entity, AST_FLAG_DONE_1 as libc::c_int as u32);
    r
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn embed_fields(mut entity: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if !entity.is_null() {
    } else {
        ponyint_assert_fail(
            b"entity != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            712 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"embed_fields\0")).as_ptr(),
        );
    };
    let mut state: libc::c_int = ast_checkflag(
        entity,
        (AST_FLAG_RECURSE_2 as libc::c_int
            | AST_FLAG_DONE_2 as libc::c_int
            | AST_FLAG_ERROR_2 as libc::c_int) as u32,
    );
    match state {
        0 => {
            ast_setflag(entity, AST_FLAG_RECURSE_2 as libc::c_int as u32);
        }
        131072 => {
            ast_error(
                (*opt).check.errors,
                entity,
                b"embedded fields can't be recursive\0" as *const u8 as *const libc::c_char,
            );
            ast_clearflag(entity, AST_FLAG_RECURSE_2 as libc::c_int as u32);
            ast_setflag(entity, AST_FLAG_ERROR_2 as libc::c_int as u32);
            return 0 as libc::c_int != 0;
        }
        262144 => return 1 as libc::c_int != 0,
        524288 => return 0 as libc::c_int != 0,
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    738 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"embed_fields\0"))
                        .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut provides: ast_ptr_t = 0 as *mut ast_t;
    let mut members: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut id,
        &mut typeparams,
        &mut cap,
        &mut provides,
        &mut members,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        entity,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        if ast_id(member) as libc::c_uint == TK_EMBED as libc::c_int as libc::c_uint {
            let mut f_id: ast_ptr_t = 0 as *mut ast_t;
            let mut f_type: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut f_id, &mut f_type, 0 as *mut *mut ast_t];
            ast_get_children(
                member,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            let mut def: *mut ast_t = ast_data(f_type) as *mut ast_t;
            if !def.is_null() {
            } else {
                ponyint_assert_fail(
                    b"def != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0"
                        as *const u8 as *const libc::c_char,
                    751 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"embed_fields\0"))
                        .as_ptr(),
                );
            };
            if !embed_fields(def, opt) {
                return 0 as libc::c_int != 0;
            }
        }
        member = ast_sibling(member);
    }
    ast_clearflag(entity, AST_FLAG_RECURSE_2 as libc::c_int as u32);
    ast_setflag(entity, AST_FLAG_DONE_2 as libc::c_int as u32);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "770:1"]
unsafe extern "C" fn local_types(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            772 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"local_types\0")).as_ptr(),
        );
    };
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut id, &mut type_0, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    if !type_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"type != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            777 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"local_types\0")).as_ptr(),
        );
    };
    if ast_id(type_0) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        if is_name_dontcare(ast_name(id)) {
            type_0 = ast_from(id, TK_DONTCARETYPE);
        } else {
            type_0 = ast_from(id, TK_INFERTYPE);
        }
    }
    ast_settype(id, type_0);
    ast_settype(ast, type_0);
}
#[c2rust::src_loc = "793:1"]
unsafe extern "C" fn add_comparable(mut ast: *mut ast_t, mut options: *mut pass_opt_t) -> bool {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/traits.c\0" as *const u8
                as *const libc::c_char,
            795 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"add_comparable\0"))
                .as_ptr(),
        );
    };
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut defcap: ast_ptr_t = 0 as *mut ast_t;
    let mut traits: ast_ptr_t = 0 as *mut ast_t;
    let mut members: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut id,
        &mut typeparams,
        &mut defcap,
        &mut traits,
        &mut members,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut typeargs: *mut ast_t = ast_from(typeparams, TK_NONE);
    let mut r: bool = 1 as libc::c_int != 0;
    let mut p: *mut ast_t = ast_child(typeparams);
    while !p.is_null() {
        let mut p_id: *mut ast_t = ast_child(p);
        let mut type_0: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = p_id;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_NOMINAL);
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
            parent_0 = ast_from(basis_ast, TK_NONE);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
        }
        if parent_0.is_null() {
            parent_0 = p_id;
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, p_id);
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, p_id);
        }
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
        if parent_0.is_null() {
            parent_0 = ast_from(basis_ast, TK_NONE);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
        }
        ast_inheritflags(parent_0);
        type_0 = parent;
        ast_append(typeargs, type_0);
        ast_setid(typeargs, TK_TYPEARGS);
        p = ast_sibling(p);
    }
    if !has_member(members, b"eq\0" as *const u8 as *const libc::c_char) {
        let mut eq: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_0: *mut ast_t = members;
        let mut parent_1: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
        let mut node_1: *mut ast_t = 0 as *mut ast_t;
        node_1 = ast_from(basis_ast_0, TK_FUN);
        if parent_1.is_null() {
            parent_1 = node_1;
        } else if last_sibling_1.is_null() {
            last_sibling_1 = ast_add(parent_1, node_1);
        } else {
            last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
        }
        let mut parent_2: *mut ast_t = node_1;
        let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
        let mut node_2: *mut ast_t = 0 as *mut ast_t;
        ast_scope(parent_2);
        node_2 = ast_from(basis_ast_0, TK_BOX);
        if parent_2.is_null() {
            parent_2 = node_2;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, node_2);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
        }
        let mut parent_3: *mut ast_t = node_2;
        let mut _last_sibling_3: *mut ast_t = 0 as *mut ast_t;
        let mut _node_3: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_3);
        if parent_2.is_null() {
            parent_2 = ast_from_string(basis_ast_0, b"eq\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(
                parent_2,
                ast_from_string(basis_ast_0, b"eq\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_2 = ast_add_sibling(
                last_sibling_2,
                ast_from_string(basis_ast_0, b"eq\0" as *const u8 as *const libc::c_char),
            );
        }
        if parent_2.is_null() {
            parent_2 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
        }
        node_2 = ast_from(basis_ast_0, TK_PARAMS);
        if parent_2.is_null() {
            parent_2 = node_2;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, node_2);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
        }
        let mut parent_4: *mut ast_t = node_2;
        let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
        let mut node_4: *mut ast_t = 0 as *mut ast_t;
        node_4 = ast_from(basis_ast_0, TK_PARAM);
        if parent_4.is_null() {
            parent_4 = node_4;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, node_4);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, node_4);
        }
        let mut parent_5: *mut ast_t = node_4;
        let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
        let mut node_5: *mut ast_t = 0 as *mut ast_t;
        if parent_5.is_null() {
            parent_5 = ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_5.is_null() {
            last_sibling_5 = ast_add(
                parent_5,
                ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_5 = ast_add_sibling(
                last_sibling_5,
                ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char),
            );
        }
        node_5 = ast_from(basis_ast_0, TK_NOMINAL);
        if parent_5.is_null() {
            parent_5 = node_5;
        } else if last_sibling_5.is_null() {
            last_sibling_5 = ast_add(parent_5, node_5);
        } else {
            last_sibling_5 = ast_add_sibling(last_sibling_5, node_5);
        }
        let mut parent_6: *mut ast_t = node_5;
        let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
        let mut _node_6: *mut ast_t = 0 as *mut ast_t;
        if parent_6.is_null() {
            parent_6 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast_0, TK_NONE));
        }
        if parent_6.is_null() {
            parent_6 = id;
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, id);
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, id);
        }
        if parent_6.is_null() {
            parent_6 = typeargs;
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, typeargs);
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, typeargs);
        }
        if parent_6.is_null() {
            parent_6 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast_0, TK_NONE));
        }
        if parent_6.is_null() {
            parent_6 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast_0, TK_NONE));
        }
        ast_inheritflags(parent_6);
        if parent_5.is_null() {
            parent_5 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_5.is_null() {
            last_sibling_5 = ast_add(parent_5, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_5 = ast_add_sibling(last_sibling_5, ast_from(basis_ast_0, TK_NONE));
        }
        ast_inheritflags(parent_5);
        ast_inheritflags(parent_4);
        node_2 = ast_from(basis_ast_0, TK_NOMINAL);
        if parent_2.is_null() {
            parent_2 = node_2;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, node_2);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
        }
        let mut parent_7: *mut ast_t = node_2;
        let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
        let mut _node_7: *mut ast_t = 0 as *mut ast_t;
        if parent_7.is_null() {
            parent_7 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, ast_from(basis_ast_0, TK_NONE));
        }
        if parent_7.is_null() {
            parent_7 = ast_from_string(basis_ast_0, b"Bool\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(
                parent_7,
                ast_from_string(basis_ast_0, b"Bool\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_7 = ast_add_sibling(
                last_sibling_7,
                ast_from_string(basis_ast_0, b"Bool\0" as *const u8 as *const libc::c_char),
            );
        }
        if parent_7.is_null() {
            parent_7 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, ast_from(basis_ast_0, TK_NONE));
        }
        if parent_7.is_null() {
            parent_7 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, ast_from(basis_ast_0, TK_NONE));
        }
        if parent_7.is_null() {
            parent_7 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, ast_from(basis_ast_0, TK_NONE));
        }
        ast_inheritflags(parent_7);
        if parent_2.is_null() {
            parent_2 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
        }
        node_2 = ast_from(basis_ast_0, TK_SEQ);
        if parent_2.is_null() {
            parent_2 = node_2;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, node_2);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
        }
        let mut parent_8: *mut ast_t = node_2;
        let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
        let mut node_8: *mut ast_t = 0 as *mut ast_t;
        node_8 = ast_from(basis_ast_0, TK_IS);
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
        node_9 = ast_from(basis_ast_0, TK_THIS);
        if parent_9.is_null() {
            parent_9 = node_9;
        } else if last_sibling_9.is_null() {
            last_sibling_9 = ast_add(parent_9, node_9);
        } else {
            last_sibling_9 = ast_add_sibling(last_sibling_9, node_9);
        }
        let mut parent_10: *mut ast_t = node_9;
        let mut _last_sibling_10: *mut ast_t = 0 as *mut ast_t;
        let mut _node_10: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_10);
        node_9 = ast_from(basis_ast_0, TK_REFERENCE);
        if parent_9.is_null() {
            parent_9 = node_9;
        } else if last_sibling_9.is_null() {
            last_sibling_9 = ast_add(parent_9, node_9);
        } else {
            last_sibling_9 = ast_add_sibling(last_sibling_9, node_9);
        }
        let mut parent_11: *mut ast_t = node_9;
        let mut last_sibling_11: *mut ast_t = 0 as *mut ast_t;
        let mut _node_11: *mut ast_t = 0 as *mut ast_t;
        if parent_11.is_null() {
            parent_11 = ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_11.is_null() {
            last_sibling_11 = ast_add(
                parent_11,
                ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_11 = ast_add_sibling(
                last_sibling_11,
                ast_from_string(basis_ast_0, b"that\0" as *const u8 as *const libc::c_char),
            );
        }
        ast_inheritflags(parent_11);
        ast_inheritflags(parent_9);
        ast_inheritflags(parent_8);
        if parent_2.is_null() {
            parent_2 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
        }
        ast_inheritflags(parent_2);
        eq = parent_1;
        ast_setdata(eq, ast as *mut libc::c_void);
        ast_append(members, eq);
        ast_set(
            ast,
            stringtab(b"eq\0" as *const u8 as *const libc::c_char),
            eq,
            SYM_DEFINED,
            0 as libc::c_int != 0,
        );
        if !ast_passes_subtree(&mut eq, options, PASS_TRAITS) {
            r = 0 as libc::c_int != 0;
        }
    }
    if !has_member(members, b"ne\0" as *const u8 as *const libc::c_char) {
        let mut ne: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_1: *mut ast_t = members;
        let mut parent_12: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_12: *mut ast_t = 0 as *mut ast_t;
        let mut node_12: *mut ast_t = 0 as *mut ast_t;
        node_12 = ast_from(basis_ast_1, TK_FUN);
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
        ast_scope(parent_13);
        node_13 = ast_from(basis_ast_1, TK_BOX);
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
        if parent_13.is_null() {
            parent_13 = ast_from_string(basis_ast_1, b"ne\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(
                parent_13,
                ast_from_string(basis_ast_1, b"ne\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_13 = ast_add_sibling(
                last_sibling_13,
                ast_from_string(basis_ast_1, b"ne\0" as *const u8 as *const libc::c_char),
            );
        }
        if parent_13.is_null() {
            parent_13 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, ast_from(basis_ast_1, TK_NONE));
        }
        node_13 = ast_from(basis_ast_1, TK_PARAMS);
        if parent_13.is_null() {
            parent_13 = node_13;
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, node_13);
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
        }
        let mut parent_15: *mut ast_t = node_13;
        let mut last_sibling_15: *mut ast_t = 0 as *mut ast_t;
        let mut node_15: *mut ast_t = 0 as *mut ast_t;
        node_15 = ast_from(basis_ast_1, TK_PARAM);
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
            parent_16 = ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_16.is_null() {
            last_sibling_16 = ast_add(
                parent_16,
                ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_16 = ast_add_sibling(
                last_sibling_16,
                ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char),
            );
        }
        node_16 = ast_from(basis_ast_1, TK_NOMINAL);
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
            parent_17 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_17.is_null() {
            last_sibling_17 = ast_add(parent_17, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_17 = ast_add_sibling(last_sibling_17, ast_from(basis_ast_1, TK_NONE));
        }
        if parent_17.is_null() {
            parent_17 = id;
        } else if last_sibling_17.is_null() {
            last_sibling_17 = ast_add(parent_17, id);
        } else {
            last_sibling_17 = ast_add_sibling(last_sibling_17, id);
        }
        if parent_17.is_null() {
            parent_17 = typeargs;
        } else if last_sibling_17.is_null() {
            last_sibling_17 = ast_add(parent_17, typeargs);
        } else {
            last_sibling_17 = ast_add_sibling(last_sibling_17, typeargs);
        }
        if parent_17.is_null() {
            parent_17 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_17.is_null() {
            last_sibling_17 = ast_add(parent_17, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_17 = ast_add_sibling(last_sibling_17, ast_from(basis_ast_1, TK_NONE));
        }
        if parent_17.is_null() {
            parent_17 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_17.is_null() {
            last_sibling_17 = ast_add(parent_17, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_17 = ast_add_sibling(last_sibling_17, ast_from(basis_ast_1, TK_NONE));
        }
        ast_inheritflags(parent_17);
        if parent_16.is_null() {
            parent_16 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_16.is_null() {
            last_sibling_16 = ast_add(parent_16, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast_1, TK_NONE));
        }
        ast_inheritflags(parent_16);
        ast_inheritflags(parent_15);
        node_13 = ast_from(basis_ast_1, TK_NOMINAL);
        if parent_13.is_null() {
            parent_13 = node_13;
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, node_13);
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
        }
        let mut parent_18: *mut ast_t = node_13;
        let mut last_sibling_18: *mut ast_t = 0 as *mut ast_t;
        let mut _node_18: *mut ast_t = 0 as *mut ast_t;
        if parent_18.is_null() {
            parent_18 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_18.is_null() {
            last_sibling_18 = ast_add(parent_18, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast_1, TK_NONE));
        }
        if parent_18.is_null() {
            parent_18 = ast_from_string(basis_ast_1, b"Bool\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_18.is_null() {
            last_sibling_18 = ast_add(
                parent_18,
                ast_from_string(basis_ast_1, b"Bool\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_18 = ast_add_sibling(
                last_sibling_18,
                ast_from_string(basis_ast_1, b"Bool\0" as *const u8 as *const libc::c_char),
            );
        }
        if parent_18.is_null() {
            parent_18 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_18.is_null() {
            last_sibling_18 = ast_add(parent_18, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast_1, TK_NONE));
        }
        if parent_18.is_null() {
            parent_18 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_18.is_null() {
            last_sibling_18 = ast_add(parent_18, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast_1, TK_NONE));
        }
        if parent_18.is_null() {
            parent_18 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_18.is_null() {
            last_sibling_18 = ast_add(parent_18, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast_1, TK_NONE));
        }
        ast_inheritflags(parent_18);
        if parent_13.is_null() {
            parent_13 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, ast_from(basis_ast_1, TK_NONE));
        }
        node_13 = ast_from(basis_ast_1, TK_SEQ);
        if parent_13.is_null() {
            parent_13 = node_13;
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, node_13);
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
        }
        let mut parent_19: *mut ast_t = node_13;
        let mut last_sibling_19: *mut ast_t = 0 as *mut ast_t;
        let mut node_19: *mut ast_t = 0 as *mut ast_t;
        node_19 = ast_from(basis_ast_1, TK_ISNT);
        if parent_19.is_null() {
            parent_19 = node_19;
        } else if last_sibling_19.is_null() {
            last_sibling_19 = ast_add(parent_19, node_19);
        } else {
            last_sibling_19 = ast_add_sibling(last_sibling_19, node_19);
        }
        let mut parent_20: *mut ast_t = node_19;
        let mut last_sibling_20: *mut ast_t = 0 as *mut ast_t;
        let mut node_20: *mut ast_t = 0 as *mut ast_t;
        node_20 = ast_from(basis_ast_1, TK_THIS);
        if parent_20.is_null() {
            parent_20 = node_20;
        } else if last_sibling_20.is_null() {
            last_sibling_20 = ast_add(parent_20, node_20);
        } else {
            last_sibling_20 = ast_add_sibling(last_sibling_20, node_20);
        }
        let mut parent_21: *mut ast_t = node_20;
        let mut _last_sibling_21: *mut ast_t = 0 as *mut ast_t;
        let mut _node_21: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_21);
        node_20 = ast_from(basis_ast_1, TK_REFERENCE);
        if parent_20.is_null() {
            parent_20 = node_20;
        } else if last_sibling_20.is_null() {
            last_sibling_20 = ast_add(parent_20, node_20);
        } else {
            last_sibling_20 = ast_add_sibling(last_sibling_20, node_20);
        }
        let mut parent_22: *mut ast_t = node_20;
        let mut last_sibling_22: *mut ast_t = 0 as *mut ast_t;
        let mut _node_22: *mut ast_t = 0 as *mut ast_t;
        if parent_22.is_null() {
            parent_22 = ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_22.is_null() {
            last_sibling_22 = ast_add(
                parent_22,
                ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_22 = ast_add_sibling(
                last_sibling_22,
                ast_from_string(basis_ast_1, b"that\0" as *const u8 as *const libc::c_char),
            );
        }
        ast_inheritflags(parent_22);
        ast_inheritflags(parent_20);
        ast_inheritflags(parent_19);
        if parent_13.is_null() {
            parent_13 = ast_from(basis_ast_1, TK_NONE);
        } else if last_sibling_13.is_null() {
            last_sibling_13 = ast_add(parent_13, ast_from(basis_ast_1, TK_NONE));
        } else {
            last_sibling_13 = ast_add_sibling(last_sibling_13, ast_from(basis_ast_1, TK_NONE));
        }
        ast_inheritflags(parent_13);
        ne = parent_12;
        ast_setdata(ne, ast as *mut libc::c_void);
        ast_append(members, ne);
        ast_set(
            ast,
            stringtab(b"ne\0" as *const u8 as *const libc::c_char),
            ne,
            SYM_DEFINED,
            0 as libc::c_int != 0,
        );
        if !ast_passes_subtree(&mut ne, options, PASS_TRAITS) {
            r = 0 as libc::c_int != 0;
        }
    }
    ast_free_unattached(typeargs);
    r
}
#[no_mangle]
#[c2rust::src_loc = "877:1"]
pub unsafe extern "C" fn pass_traits(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    match ast_id(ast) as libc::c_uint {
        75 | 76 | 77 => {
            if !trait_entity(ast, options) {
                return AST_ERROR;
            }
            if !embed_fields(ast, options) {
                return AST_ERROR;
            }
        }
        74 => {
            if !trait_entity(ast, options) {
                return AST_ERROR;
            }
            if !add_comparable(ast, options) {
                return AST_FATAL;
            }
        }
        72 | 73 => {
            if !trait_entity(ast, options) {
                return AST_ERROR;
            }
        }
        85 | 84 => {
            local_types(ast);
        }
        _ => {}
    }
    AST_OK
}
