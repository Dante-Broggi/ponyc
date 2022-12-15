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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: usize,
    }
    use super::_size_t_h::size_t;
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
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::error_h::errors_t;
    use super::source_h::source_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "60:1"]
        pub fn ast_from_string(ast: *mut ast_t, name: *const libc::c_char) -> *mut ast_t;
        #[c2rust::src_loc = "61:1"]
        pub fn ast_from_int(ast: *mut ast_t, value: u64) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "76:1"]
        pub fn ast_source(ast: *mut ast_t) -> *mut source_t;
        #[c2rust::src_loc = "79:1"]
        pub fn ast_setdata(ast: *mut ast_t, data: *mut libc::c_void) -> *mut ast_t;
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn ast_name_len(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "103:1"]
        pub fn ast_setannotation(ast: *mut ast_t, annotation: *mut ast_t);
        #[c2rust::src_loc = "104:1"]
        pub fn ast_consumeannotation(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
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
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "141:1"]
        pub fn ast_remove(ast: *mut ast_t);
        #[c2rust::src_loc = "142:1"]
        pub fn ast_swap(prev: *mut ast_t, next: *mut ast_t);
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
        #[c2rust::src_loc = "205:1"]
        pub fn ast_extract_children(
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
        #[c2rust::src_loc = "377:1"]
        pub fn ast_passes_type(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
        #[c2rust::src_loc = "390:1"]
        pub fn ast_passes_subtree(
            astp: *mut *mut ast_t,
            options: *mut pass_opt_t,
            last_pass: pass_id,
        ) -> bool;
        #[c2rust::src_loc = "399:1"]
        pub fn ast_pass_record(ast: *mut ast_t, pass: pass_id);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/printbuf.h:4"]
pub mod printbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct printbuf_t {
        pub m: *mut libc::c_char,
        pub size: usize,
        pub offset: usize,
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "344:6"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:3"]
pub mod id_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn check_id_param(opt: *mut pass_opt_t, id_node: *mut ast_t) -> bool;
        #[c2rust::src_loc = "48:1"]
        pub fn is_name_internal_test(name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "51:1"]
        pub fn is_name_dontcare(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/ifdef.h:6"]
pub mod ifdef_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn ifdef_cond_normalise(astp: *mut *mut ast_t, opt: *mut pass_opt_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:7"]
pub mod package_h {
    use super::frame_h::typecheck_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "110:1"]
        pub fn package_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "143:1"]
        pub fn package_hygienic_id(t: *mut typecheck_t) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:9"]
pub mod assemble_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn type_sugar(
            from: *mut ast_t,
            package: *const libc::c_char,
            name: *const libc::c_char,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn type_for_this(
            opt: *mut pass_opt_t,
            ast: *mut ast_t,
            cap: token_id,
            ephemeral: token_id,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/sanitise.h:10"]
pub mod sanitise_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn collect_type_params(
            ast: *mut ast_t,
            out_params: *mut *mut ast_t,
            out_args: *mut *mut ast_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:11"]
pub mod subtype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn is_none(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:14"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "70:7"]
        pub fn memchr(
            _: *const libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "72:7"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::assemble_h::{type_for_this, type_sugar};
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_child, ast_childcount, ast_childidx, ast_childlast,
    ast_consumeannotation, ast_error, ast_error_continue, ast_extract_children,
    ast_free_unattached, ast_from, ast_from_int, ast_from_string, ast_get_children, ast_id,
    ast_inheritflags, ast_line, ast_name, ast_name_len, ast_nearest, ast_parent, ast_pop, ast_pos,
    ast_print_type, ast_ptr_t, ast_remove, ast_replace, ast_result_t, ast_scope, ast_setannotation,
    ast_setdata, ast_setid, ast_sibling, ast_source, ast_swap, AST_ERROR, AST_FATAL, AST_IGNORE,
    AST_OK,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::id_h::{check_id_param, is_name_dontcare, is_name_internal_test};
use self::ifdef_h::ifdef_cond_normalise;
use self::package_h::{package_hygienic_id, package_name};
pub use self::pass_h::{
    ast_pass_record, ast_passes_subtree, ast_passes_type, magic_package_t, pass_id, pass_opt_t,
    plugins_t, verbosity_level, PASS_ALL, PASS_ASM, PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS,
    PASS_EXPR, PASS_FINALISER, PASS_FLATTEN, PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION,
    PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH, PASS_REFER, PASS_SCOPE, PASS_SERIALISER,
    PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY, VERBOSITY_ALL, VERBOSITY_INFO,
    VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::ponyint_pool_alloc_size;
pub use self::printbuf_h::{printbuf, printbuf_free, printbuf_new, printbuf_t};
use self::sanitise_h::collect_type_params;
pub use self::source_h::source_t;
use self::stdio_h::snprintf;
use self::string_h::{memchr, memcpy, strcmp};
use self::stringtab_h::{stringtab, stringtab_consume};
use self::subtype_h::is_none;
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
#[c2rust::src_loc = "20:1"]
unsafe extern "C" fn make_runtime_override_defaults(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            22 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"make_runtime_override_defaults\0",
            ))
            .as_ptr(),
        );
    };
    let mut cap: token_id = TK_AT;
    let mut runtime_override_defaults: *mut ast_t = 0 as *mut ast_t;
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
    let mut node_0: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_0);
    node_0 = ast_from(basis_ast, cap);
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
        parent_0 = ast_from_string(
            basis_ast,
            b"runtime_override_defaults\0" as *const u8 as *const libc::c_char,
        );
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(
            parent_0,
            ast_from_string(
                basis_ast,
                b"runtime_override_defaults\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        last_sibling_0 = ast_add_sibling(
            last_sibling_0,
            ast_from_string(
                basis_ast,
                b"runtime_override_defaults\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    node_0 = ast_from(basis_ast, TK_PARAMS);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_2: *mut ast_t = node_0;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    node_2 = ast_from(basis_ast, TK_PARAM);
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
    if parent_3.is_null() {
        parent_3 = ast_from_string(basis_ast, b"rto\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(
            parent_3,
            ast_from_string(basis_ast, b"rto\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_3 = ast_add_sibling(
            last_sibling_3,
            ast_from_string(basis_ast, b"rto\0" as *const u8 as *const libc::c_char),
        );
    }
    node_3 = ast_from(basis_ast, TK_NOMINAL);
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
        parent_4 = ast_from_string(basis_ast, b"$0\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(
            parent_4,
            ast_from_string(basis_ast, b"$0\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_4 = ast_add_sibling(
            last_sibling_4,
            ast_from_string(basis_ast, b"$0\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_4.is_null() {
        parent_4 = ast_from_string(
            basis_ast,
            b"RuntimeOptions\0" as *const u8 as *const libc::c_char,
        );
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(
            parent_4,
            ast_from_string(
                basis_ast,
                b"RuntimeOptions\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        last_sibling_4 = ast_add_sibling(
            last_sibling_4,
            ast_from_string(
                basis_ast,
                b"RuntimeOptions\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_4);
    if parent_3.is_null() {
        parent_3 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_3);
    ast_inheritflags(parent_2);
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
    node_0 = ast_from(basis_ast, TK_SEQ);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_5: *mut ast_t = node_0;
    let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
    let mut node_5: *mut ast_t = 0 as *mut ast_t;
    node_5 = ast_from(basis_ast, TK_TRUE);
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
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    runtime_override_defaults = parent;
    runtime_override_defaults
}
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn make_create(mut ast: *mut ast_t) -> *mut ast_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            44 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"make_create\0")).as_ptr(),
        );
    };
    let mut cap: token_id = TK_NONE;
    match ast_id(ast) as libc::c_uint {
        75 | 76 => {
            cap = TK_ISO;
        }
        _ => {}
    }
    let mut create: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_NEW);
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
    node_0 = ast_from(basis_ast, cap);
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
        parent_0 = ast_from_string(basis_ast, b"create\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(
            parent_0,
            ast_from_string(basis_ast, b"create\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_0 = ast_add_sibling(
            last_sibling_0,
            ast_from_string(basis_ast, b"create\0" as *const u8 as *const libc::c_char),
        );
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
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    node_0 = ast_from(basis_ast, TK_SEQ);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_2: *mut ast_t = node_0;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    node_2 = ast_from(basis_ast, TK_TRUE);
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
    ast_inheritflags(parent_2);
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    create = parent;
    create
}
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn has_member(
    mut members: *mut ast_t,
    mut name: *const libc::c_char,
) -> bool {
    name = stringtab(name);
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        let mut id: *mut ast_t = 0 as *mut ast_t;
        match ast_id(member) as libc::c_uint {
            140 | 141 | 86 => {
                id = ast_child(member);
            }
            _ => {
                id = ast_childidx(member, 1 as libc::c_int as usize);
            }
        }
        if ast_name(id) == name {
            return 1 as libc::c_int != 0;
        }
        member = ast_sibling(member);
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn add_default_runtime_override_defaults_method(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            109 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"add_default_runtime_override_defaults_method\0",
            ))
            .as_ptr(),
        );
    };
    let mut members: *mut ast_t = ast_childidx(ast, 4 as libc::c_int as usize);
    if has_member(
        members,
        b"runtime_override_defaults\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    ast_append(members, make_runtime_override_defaults(ast));
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn add_default_constructor(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            122 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"add_default_constructor\0",
            ))
            .as_ptr(),
        );
    };
    let mut members: *mut ast_t = ast_childidx(ast, 4 as libc::c_int as usize);
    if has_member(members, b"create\0" as *const u8 as *const libc::c_char) {
        return;
    }
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            88 => return,
            _ => {}
        }
        member = ast_sibling(member);
    }
    ast_append(members, make_create(ast));
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn sugar_module(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut docstring: *mut ast_t = ast_child(ast);
    let mut package: *mut ast_t = ast_parent(ast);
    if ast_id(package) as libc::c_uint == TK_PACKAGE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(package) == TK_PACKAGE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            154 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sugar_module\0")).as_ptr(),
        );
    };
    if strcmp(
        package_name(package),
        b"$0\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        let mut builtin: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = ast;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_USE);
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
            parent_0 = ast_setid(
                ast_from_string(
                    basis_ast,
                    stringtab(b"builtin\0" as *const u8 as *const libc::c_char),
                ),
                TK_STRING,
            );
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(
                parent_0,
                ast_setid(
                    ast_from_string(
                        basis_ast,
                        stringtab(b"builtin\0" as *const u8 as *const libc::c_char),
                    ),
                    TK_STRING,
                ),
            );
        } else {
            last_sibling_0 = ast_add_sibling(
                last_sibling_0,
                ast_setid(
                    ast_from_string(
                        basis_ast,
                        stringtab(b"builtin\0" as *const u8 as *const libc::c_char),
                    ),
                    TK_STRING,
                ),
            );
        }
        if parent_0.is_null() {
            parent_0 = ast_from(basis_ast, TK_NONE);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
        }
        ast_inheritflags(parent_0);
        builtin = parent;
        ast_add(ast, builtin);
    }
    if docstring.is_null()
        || ast_id(docstring) as libc::c_uint != TK_STRING as libc::c_int as libc::c_uint
    {
        return AST_OK;
    }
    let mut package_docstring: *mut ast_t = ast_childlast(package);
    if ast_id(package_docstring) as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint {
        ast_error(
            (*opt).check.errors,
            docstring,
            b"the package already has a docstring\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*opt).check.errors,
            package_docstring,
            b"the existing docstring is here\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    ast_append(package, docstring);
    ast_remove(docstring);
    AST_OK
}
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn sugar_docstring(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            191 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sugar_docstring\0"))
                .as_ptr(),
        );
    };
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
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(docstring) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        let mut first: *mut ast_t = ast_child(body);
        if !first.is_null()
            && ast_id(first) as libc::c_uint == TK_STRING as libc::c_int as libc::c_uint
            && !(ast_sibling(first)).is_null()
        {
            ast_pop(body);
            ast_replace(&mut docstring, first);
        }
    }
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn sugar_entity(
    mut _opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
    mut add_create: bool,
    mut def_def_cap: token_id,
) -> ast_result_t {
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
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_name(id) == stringtab(b"Main\0" as *const u8 as *const libc::c_char) {
        add_default_runtime_override_defaults_method(ast);
    }
    if add_create {
        add_default_constructor(ast);
    }
    if ast_id(defcap) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        ast_setid(defcap, def_def_cap);
    }
    let mut init_seq: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = members;
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
    let mut _last_sibling_0: *mut ast_t = 0 as *mut ast_t;
    let mut _node_0: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_0);
    init_seq = parent;
    let mut member: *mut ast_t = ast_child(members);
    while !member.is_null() {
        match ast_id(member) as libc::c_uint {
            141 | 140 | 86 => {
                let mut f_id: ast_ptr_t = 0 as *mut ast_t;
                let mut f_type: ast_ptr_t = 0 as *mut ast_t;
                let mut f_init: ast_ptr_t = 0 as *mut ast_t;
                let mut children_0: [*mut *mut ast_t; 4] =
                    [&mut f_id, &mut f_type, &mut f_init, 0 as *mut *mut ast_t];
                ast_get_children(
                    member,
                    (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                        .wrapping_sub(1),
                    children_0.as_mut_ptr(),
                );
                if ast_id(f_init) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
                    ast_swap(f_init, ast_from(f_init, TK_NONE));
                    let mut init: *mut ast_t = 0 as *mut ast_t;
                    let mut basis_ast_0: *mut ast_t = member;
                    let mut parent_1: *mut ast_t = 0 as *mut ast_t;
                    let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
                    let mut node_1: *mut ast_t = 0 as *mut ast_t;
                    node_1 = ast_from(basis_ast_0, TK_ASSIGN);
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
                    node_2 = ast_from(basis_ast_0, TK_REFERENCE);
                    if parent_2.is_null() {
                        parent_2 = node_2;
                    } else if last_sibling_2.is_null() {
                        last_sibling_2 = ast_add(parent_2, node_2);
                    } else {
                        last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
                    }
                    let mut parent_3: *mut ast_t = node_2;
                    let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
                    let mut _node_3: *mut ast_t = 0 as *mut ast_t;
                    if parent_3.is_null() {
                        parent_3 = f_id;
                    } else if last_sibling_3.is_null() {
                        last_sibling_3 = ast_add(parent_3, f_id);
                    } else {
                        last_sibling_3 = ast_add_sibling(last_sibling_3, f_id);
                    }
                    ast_inheritflags(parent_3);
                    if parent_2.is_null() {
                        parent_2 = f_init;
                    } else if last_sibling_2.is_null() {
                        last_sibling_2 = ast_add(parent_2, f_init);
                    } else {
                        last_sibling_2 = ast_add_sibling(last_sibling_2, f_init);
                    }
                    ast_inheritflags(parent_2);
                    init = parent_1;
                    ast_add(init_seq, init);
                }
            }
            _ => {}
        }
        member = ast_sibling(member);
    }
    if !(ast_child(init_seq)).is_null() {
        member = ast_child(members);
        while !member.is_null() {
            match ast_id(member) as libc::c_uint {
                88 => {
                    let mut n_cap: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_id: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_typeparam: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_params: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_result: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_partial: ast_ptr_t = 0 as *mut ast_t;
                    let mut n_body: ast_ptr_t = 0 as *mut ast_t;
                    let mut children_1: [*mut *mut ast_t; 8] = [
                        &mut n_cap,
                        &mut n_id,
                        &mut n_typeparam,
                        &mut n_params,
                        &mut n_result,
                        &mut n_partial,
                        &mut n_body,
                        0 as *mut *mut ast_t,
                    ];
                    ast_get_children(
                        member,
                        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1),
                        children_1.as_mut_ptr(),
                    );
                    if ast_id(n_body) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
                    } else {
                        ponyint_assert_fail(
                            b"ast_id(n_body) == TK_SEQ\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                                as *const u8 as *const libc::c_char,
                            279 as libc::c_int as usize,
                            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(
                                b"sugar_entity\0",
                            ))
                            .as_ptr(),
                        );
                    };
                    sugar_docstring(member);
                    let mut init_0: *mut ast_t = ast_child(init_seq);
                    while !init_0.is_null() {
                        ast_add(n_body, init_0);
                        init_0 = ast_sibling(init_0);
                    }
                }
                _ => {}
            }
            member = ast_sibling(member);
        }
    }
    ast_free_unattached(init_seq);
    return AST_OK;
}
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn sugar_typeparam(mut ast: *mut ast_t) -> ast_result_t {
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut constraint: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut id, &mut constraint, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(constraint) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        let mut basis_ast: *mut ast_t = constraint;
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
            parent_0 = id;
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, id);
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, id);
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
        ast_replace(&mut constraint, parent);
    }
    return AST_OK;
}
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn check_params(
    mut opt: *mut pass_opt_t,
    mut params: *mut ast_t,
) -> ast_result_t {
    if !params.is_null() {
    } else {
        ponyint_assert_fail(
            b"params != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            326 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"check_params\0")).as_ptr(),
        );
    };
    let mut result: ast_result_t = AST_OK;
    let mut p: *mut ast_t = ast_child(params);
    while !p.is_null() {
        if !(ast_id(p) as libc::c_uint == TK_ELLIPSIS as libc::c_int as libc::c_uint) {
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut type_0: ast_ptr_t = 0 as *mut ast_t;
            let mut def_arg: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 4] =
                [&mut id, &mut type_0, &mut def_arg, 0 as *mut *mut ast_t];
            ast_get_children(
                p,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children.as_mut_ptr(),
            );
            if ast_id(id) as libc::c_uint != TK_ID as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    p,
                    b"expected parameter name\0" as *const u8 as *const libc::c_char,
                );
                result = AST_ERROR;
            } else if !is_name_internal_test(ast_name(id)) && !check_id_param(opt, id) {
                result = AST_ERROR;
            }
            if ast_id(type_0) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    type_0,
                    b"expected parameter type\0" as *const u8 as *const libc::c_char,
                );
                result = AST_ERROR;
            }
        }
        p = ast_sibling(p);
    }
    return result;
}
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn check_method(
    mut opt: *mut pass_opt_t,
    mut method: *mut ast_t,
) -> ast_result_t {
    if !method.is_null() {
    } else {
        ponyint_assert_fail(
            b"method != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            360 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"check_method\0")).as_ptr(),
        );
    };
    let mut result: ast_result_t = AST_OK;
    let mut params: *mut ast_t = ast_childidx(method, 3 as libc::c_int as usize);
    result = check_params(opt, params);
    return result;
}
#[c2rust::src_loc = "370:1"]
unsafe extern "C" fn sugar_new(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(result) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        let mut tcap: token_id = ast_id(cap);
        if tcap as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
            match ast_id((*(*opt).check.frame).type_0) as libc::c_uint {
                74 => {
                    tcap = TK_VAL;
                }
                77 => {
                    tcap = TK_TAG;
                }
                _ => {
                    tcap = TK_REF;
                }
            }
            ast_setid(cap, tcap);
        }
        ast_replace(&mut result, type_for_this(opt, ast, tcap, TK_EPHEMERAL));
    }
    sugar_docstring(ast);
    return check_method(opt, ast);
}
#[c2rust::src_loc = "400:1"]
unsafe extern "C" fn sugar_be(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    ast_setid(cap, TK_TAG);
    if ast_id(result) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        let mut type_0: *mut ast_t = type_sugar(
            ast,
            0 as *const libc::c_char,
            b"None\0" as *const u8 as *const libc::c_char,
        );
        ast_replace(&mut result, type_0);
    }
    sugar_docstring(ast);
    return check_method(opt, ast);
}
#[no_mangle]
#[c2rust::src_loc = "419:1"]
pub unsafe extern "C" fn fun_defaults(mut ast: *mut ast_t) {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            421 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"fun_defaults\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut docstring: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        &mut docstring,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(cap) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        ast_setid(cap, TK_BOX);
    }
    if ast_id(result) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        let mut type_0: *mut ast_t = type_sugar(
            ast,
            0 as *const libc::c_char,
            b"None\0" as *const u8 as *const libc::c_char,
        );
        ast_replace(&mut result, type_0);
    }
    if is_none(result) as libc::c_int != 0
        && ast_id(body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
    {
        let mut last_cmd: *mut ast_t = ast_childlast(body);
        if ast_id(last_cmd) as libc::c_uint != TK_ERROR as libc::c_int as libc::c_uint
            && ast_id(last_cmd) as libc::c_uint != TK_RETURN as libc::c_int as libc::c_uint
        {
            let mut ref_0: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast: *mut ast_t = body;
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
                parent_0 =
                    ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char);
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(
                    parent_0,
                    ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char),
                );
            } else {
                last_sibling_0 = ast_add_sibling(
                    last_sibling_0,
                    ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char),
                );
            }
            ast_inheritflags(parent_0);
            ref_0 = parent;
            ast_append(body, ref_0);
        }
    }
}
#[c2rust::src_loc = "451:1"]
unsafe extern "C" fn sugar_fun(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    fun_defaults(ast);
    sugar_docstring(ast);
    return check_method(opt, ast);
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn expand_none(mut ast: *mut ast_t, mut is_scope: bool) {
    if ast_id(ast) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        return;
    }
    if is_scope {
        ast_scope(ast);
    }
    ast_setid(ast, TK_SEQ);
    let mut ref_0: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
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
        parent_0 = ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(
            parent_0,
            ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_0 = ast_add_sibling(
            last_sibling_0,
            ast_from_string(basis_ast, b"None\0" as *const u8 as *const libc::c_char),
        );
    }
    ast_inheritflags(parent_0);
    ref_0 = parent;
    ast_add(ast, ref_0);
}
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn sugar_return(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut return_value: *mut ast_t = ast_child(ast);
    if ast_id(ast) as libc::c_uint == TK_RETURN as libc::c_int as libc::c_uint
        && ast_id((*(*opt).check.frame).method) as libc::c_uint
            == TK_NEW as libc::c_int as libc::c_uint
    {
        if ast_id(return_value) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(return_value) == TK_NONE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                480 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sugar_return\0"))
                    .as_ptr(),
            );
        };
        ast_setid(return_value, TK_THIS);
    } else {
        expand_none(return_value, 0 as libc::c_int != 0);
    }
    return AST_OK;
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn sugar_else(mut ast: *mut ast_t, mut index: usize) -> ast_result_t {
    let mut else_clause: *mut ast_t = ast_childidx(ast, index);
    expand_none(else_clause, 1 as libc::c_int != 0);
    return AST_OK;
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn sugar_try(mut ast: *mut ast_t) -> ast_result_t {
    let mut ignore: ast_ptr_t = 0 as *mut ast_t;
    let mut else_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut then_clause: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [
        &mut ignore,
        &mut else_clause,
        &mut then_clause,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(else_clause) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
        && ast_id(then_clause) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
    {
        ast_setid(ast, TK_TRY_NO_CHECK);
    }
    expand_none(else_clause, 1 as libc::c_int != 0);
    expand_none(then_clause, 1 as libc::c_int != 0);
    return AST_OK;
}
#[c2rust::src_loc = "513:1"]
unsafe extern "C" fn sugar_for(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    let mut for_idseq: ast_ptr_t = 0 as *mut ast_t;
    let mut for_iter: ast_ptr_t = 0 as *mut ast_t;
    let mut for_body: ast_ptr_t = 0 as *mut ast_t;
    let mut for_else: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut for_idseq,
        &mut for_iter,
        &mut for_body,
        &mut for_else,
        0 as *mut *mut ast_t,
    ];
    ast_extract_children(
        *astp,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut annotation: *mut ast_t = ast_consumeannotation(*astp);
    expand_none(for_else, 1 as libc::c_int != 0);
    let mut iter_name: *const libc::c_char = package_hygienic_id(&mut (*opt).check);
    let mut try_next: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = for_iter;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_TRY_NO_CHECK);
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
    node_0 = ast_from(basis_ast, TK_SEQ);
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
    ast_scope(parent_1);
    node_1 = ast_from(basis_ast, TK_CALL);
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
    node_2 = ast_from(basis_ast, TK_DOT);
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
    node_3 = ast_from(basis_ast, TK_REFERENCE);
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
        parent_4 = ast_from_string(basis_ast, iter_name);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from_string(basis_ast, iter_name));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from_string(basis_ast, iter_name));
    }
    ast_inheritflags(parent_4);
    if parent_3.is_null() {
        parent_3 = ast_from_string(basis_ast, b"next\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(
            parent_3,
            ast_from_string(basis_ast, b"next\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_3 = ast_add_sibling(
            last_sibling_3,
            ast_from_string(basis_ast, b"next\0" as *const u8 as *const libc::c_char),
        );
    }
    ast_inheritflags(parent_3);
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    node_2 = ast_from(basis_ast, TK_QUESTION);
    if parent_2.is_null() {
        parent_2 = node_2;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, node_2);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
    }
    let mut parent_5: *mut ast_t = node_2;
    let mut _last_sibling_5: *mut ast_t = 0 as *mut ast_t;
    let mut _node_5: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_5);
    ast_inheritflags(parent_2);
    ast_inheritflags(parent_1);
    node_0 = ast_from(basis_ast, TK_SEQ);
    if parent_0.is_null() {
        parent_0 = node_0;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, node_0);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
    }
    let mut parent_6: *mut ast_t = node_0;
    let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
    let mut node_6: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_6);
    node_6 = ast_from(basis_ast, TK_BREAK);
    if parent_6.is_null() {
        parent_6 = node_6;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, node_6);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
    }
    let mut parent_7: *mut ast_t = node_6;
    let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
    let mut _node_7: *mut ast_t = 0 as *mut ast_t;
    if parent_7.is_null() {
        parent_7 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_7.is_null() {
        last_sibling_7 = ast_add(parent_7, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_7 = ast_add_sibling(last_sibling_7, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_7);
    ast_inheritflags(parent_6);
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    try_next = parent;
    sugar_try(try_next);
    let mut basis_ast_0: *mut ast_t = *astp;
    let mut parent_8: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
    let mut node_8: *mut ast_t = 0 as *mut ast_t;
    node_8 = ast_from(basis_ast_0, TK_SEQ);
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
    node_9 = ast_from(basis_ast_0, TK_ASSIGN);
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
    node_10 = ast_from(basis_ast_0, TK_LET);
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
        parent_11 = ast_setdata(
            ast_from_string(basis_ast_0, iter_name),
            stringtab(b"for loop iterator\0" as *const u8 as *const libc::c_char)
                as *mut libc::c_void,
        );
    } else if last_sibling_11.is_null() {
        last_sibling_11 = ast_add(
            parent_11,
            ast_setdata(
                ast_from_string(basis_ast_0, iter_name),
                stringtab(b"for loop iterator\0" as *const u8 as *const libc::c_char)
                    as *mut libc::c_void,
            ),
        );
    } else {
        last_sibling_11 = ast_add_sibling(
            last_sibling_11,
            ast_setdata(
                ast_from_string(basis_ast_0, iter_name),
                stringtab(b"for loop iterator\0" as *const u8 as *const libc::c_char)
                    as *mut libc::c_void,
            ),
        );
    }
    if parent_11.is_null() {
        parent_11 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_11.is_null() {
        last_sibling_11 = ast_add(parent_11, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_11 = ast_add_sibling(last_sibling_11, ast_from(basis_ast_0, TK_NONE));
    }
    ast_inheritflags(parent_11);
    if parent_10.is_null() {
        parent_10 = for_iter;
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, for_iter);
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, for_iter);
    }
    ast_inheritflags(parent_10);
    node_9 = ast_from(basis_ast_0, TK_WHILE);
    if parent_9.is_null() {
        parent_9 = node_9;
    } else if last_sibling_9.is_null() {
        last_sibling_9 = ast_add(parent_9, node_9);
    } else {
        last_sibling_9 = ast_add_sibling(last_sibling_9, node_9);
    }
    let mut parent_12: *mut ast_t = node_9;
    let mut last_sibling_12: *mut ast_t = 0 as *mut ast_t;
    let mut node_12: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_12);
    ast_setannotation(parent_12, annotation);
    node_12 = ast_from(basis_ast_0, TK_SEQ);
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
    node_13 = ast_from(for_iter, TK_CALL);
    if parent_13.is_null() {
        parent_13 = node_13;
    } else if last_sibling_13.is_null() {
        last_sibling_13 = ast_add(parent_13, node_13);
    } else {
        last_sibling_13 = ast_add_sibling(last_sibling_13, node_13);
    }
    let mut parent_14: *mut ast_t = node_13;
    let mut last_sibling_14: *mut ast_t = 0 as *mut ast_t;
    let mut node_14: *mut ast_t = 0 as *mut ast_t;
    node_14 = ast_from(basis_ast_0, TK_DOT);
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
    node_15 = ast_from(basis_ast_0, TK_REFERENCE);
    if parent_15.is_null() {
        parent_15 = node_15;
    } else if last_sibling_15.is_null() {
        last_sibling_15 = ast_add(parent_15, node_15);
    } else {
        last_sibling_15 = ast_add_sibling(last_sibling_15, node_15);
    }
    let mut parent_16: *mut ast_t = node_15;
    let mut last_sibling_16: *mut ast_t = 0 as *mut ast_t;
    let mut _node_16: *mut ast_t = 0 as *mut ast_t;
    if parent_16.is_null() {
        parent_16 = ast_from_string(basis_ast_0, iter_name);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(parent_16, ast_from_string(basis_ast_0, iter_name));
    } else {
        last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from_string(basis_ast_0, iter_name));
    }
    ast_inheritflags(parent_16);
    if parent_15.is_null() {
        parent_15 = ast_from_string(
            basis_ast_0,
            b"has_next\0" as *const u8 as *const libc::c_char,
        );
    } else if last_sibling_15.is_null() {
        last_sibling_15 = ast_add(
            parent_15,
            ast_from_string(
                basis_ast_0,
                b"has_next\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        last_sibling_15 = ast_add_sibling(
            last_sibling_15,
            ast_from_string(
                basis_ast_0,
                b"has_next\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    ast_inheritflags(parent_15);
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast_0, TK_NONE));
    }
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast_0, TK_NONE));
    }
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast_0, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast_0, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast_0, TK_NONE));
    }
    ast_inheritflags(parent_14);
    ast_inheritflags(parent_13);
    node_12 = ast_from(basis_ast_0, TK_SEQ);
    if parent_12.is_null() {
        parent_12 = node_12;
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, node_12);
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, node_12);
    }
    let mut parent_17: *mut ast_t = node_12;
    let mut last_sibling_17: *mut ast_t = 0 as *mut ast_t;
    let mut node_17: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_17);
    node_17 = ast_from(for_idseq, TK_ASSIGN);
    if parent_17.is_null() {
        parent_17 = node_17;
    } else if last_sibling_17.is_null() {
        last_sibling_17 = ast_add(parent_17, node_17);
    } else {
        last_sibling_17 = ast_add_sibling(last_sibling_17, node_17);
    }
    let mut parent_18: *mut ast_t = node_17;
    let mut last_sibling_18: *mut ast_t = 0 as *mut ast_t;
    let mut _node_18: *mut ast_t = 0 as *mut ast_t;
    if parent_18.is_null() {
        parent_18 = for_idseq;
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, for_idseq);
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, for_idseq);
    }
    if parent_18.is_null() {
        parent_18 = try_next;
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, try_next);
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, try_next);
    }
    ast_inheritflags(parent_18);
    if parent_17.is_null() {
        parent_17 = for_body;
    } else if last_sibling_17.is_null() {
        last_sibling_17 = ast_add(parent_17, for_body);
    } else {
        last_sibling_17 = ast_add_sibling(last_sibling_17, for_body);
    }
    ast_inheritflags(parent_17);
    if parent_12.is_null() {
        parent_12 = for_else;
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, for_else);
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, for_else);
    }
    ast_inheritflags(parent_12);
    ast_inheritflags(parent_9);
    ast_replace(astp, parent_8);
    return AST_OK;
}
#[c2rust::src_loc = "559:1"]
unsafe extern "C" fn build_with_dispose(mut dispose_clause: *mut ast_t, mut idseq: *mut ast_t) {
    if !dispose_clause.is_null() {
    } else {
        ponyint_assert_fail(
            b"dispose_clause != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            561 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"build_with_dispose\0"))
                .as_ptr(),
        );
    };
    if !idseq.is_null() {
    } else {
        ponyint_assert_fail(
            b"idseq != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            562 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"build_with_dispose\0"))
                .as_ptr(),
        );
    };
    if ast_id(idseq) as libc::c_uint == TK_LET as libc::c_int as libc::c_uint {
        let mut id: *mut ast_t = ast_child(idseq);
        if !id.is_null() {
        } else {
            ponyint_assert_fail(
                b"id != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                568 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"build_with_dispose\0",
                ))
                .as_ptr(),
            );
        };
        if is_name_dontcare(ast_name(id)) {
            return;
        }
        let mut dispose: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = idseq;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_CALL);
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
            parent_2 = id;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, id);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, id);
        }
        ast_inheritflags(parent_2);
        if parent_1.is_null() {
            parent_1 = ast_from_string(basis_ast, b"dispose\0" as *const u8 as *const libc::c_char);
        } else if last_sibling_1.is_null() {
            last_sibling_1 = ast_add(
                parent_1,
                ast_from_string(basis_ast, b"dispose\0" as *const u8 as *const libc::c_char),
            );
        } else {
            last_sibling_1 = ast_add_sibling(
                last_sibling_1,
                ast_from_string(basis_ast, b"dispose\0" as *const u8 as *const libc::c_char),
            );
        }
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
        if parent_0.is_null() {
            parent_0 = ast_from(basis_ast, TK_NONE);
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
        }
        ast_inheritflags(parent_0);
        dispose = parent;
        ast_add(dispose_clause, dispose);
        return;
    }
    if ast_id(idseq) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(idseq) == TK_TUPLE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            586 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"build_with_dispose\0"))
                .as_ptr(),
        );
    };
    let mut p: *mut ast_t = ast_child(idseq);
    while !p.is_null() {
        if ast_id(p) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(p) == TK_SEQ\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                590 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"build_with_dispose\0",
                ))
                .as_ptr(),
            );
        };
        let mut let_0: *mut ast_t = ast_child(p);
        build_with_dispose(dispose_clause, let_0);
        p = ast_sibling(p);
    }
}
#[c2rust::src_loc = "597:1"]
unsafe extern "C" fn sugar_with(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    let mut withexpr: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut withexpr, &mut body, 0 as *mut *mut ast_t];
    ast_extract_children(
        *astp,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut main_annotation: *mut ast_t = ast_consumeannotation(*astp);
    let mut replace: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = *astp;
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
    node_0 = ast_from(basis_ast, TK_DISPOSING_BLOCK);
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
    ast_setannotation(parent_1, main_annotation);
    node_1 = ast_from(basis_ast, TK_SEQ);
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
    ast_scope(parent_2);
    if parent_2.is_null() {
        parent_2 = body;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, body);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, body);
    }
    ast_inheritflags(parent_2);
    node_1 = ast_from(basis_ast, TK_SEQ);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_3: *mut ast_t = node_1;
    let mut _last_sibling_3: *mut ast_t = 0 as *mut ast_t;
    let mut _node_3: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_3);
    ast_inheritflags(parent_3);
    ast_inheritflags(parent_1);
    ast_inheritflags(parent_0);
    replace = parent;
    let mut dblock: *mut ast_t = ast_child(replace);
    let mut dbody: ast_ptr_t = 0 as *mut ast_t;
    let mut dexit: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut dbody, &mut dexit, 0 as *mut *mut ast_t];
    ast_get_children(
        dblock,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut p: *mut ast_t = ast_child(withexpr);
    while !p.is_null() {
        if ast_id(p) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(p) == TK_SEQ\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                617 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sugar_with\0"))
                    .as_ptr(),
            );
        };
        let mut idseq: ast_ptr_t = 0 as *mut ast_t;
        let mut init: ast_ptr_t = 0 as *mut ast_t;
        let mut children_1: [*mut *mut ast_t; 3] = [&mut idseq, &mut init, 0 as *mut *mut ast_t];
        ast_get_children(
            p,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1),
            children_1.as_mut_ptr(),
        );
        let mut init_name: *const libc::c_char = package_hygienic_id(&mut (*opt).check);
        let mut assign: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_0: *mut ast_t = idseq;
        let mut parent_4: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
        let mut node_4: *mut ast_t = 0 as *mut ast_t;
        node_4 = ast_from(basis_ast_0, TK_ASSIGN);
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
        node_5 = ast_from(basis_ast_0, TK_LET);
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
            parent_6 = ast_from_string(basis_ast_0, init_name);
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, ast_from_string(basis_ast_0, init_name));
        } else {
            last_sibling_6 =
                ast_add_sibling(last_sibling_6, ast_from_string(basis_ast_0, init_name));
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
            parent_5 = init;
        } else if last_sibling_5.is_null() {
            last_sibling_5 = ast_add(parent_5, init);
        } else {
            last_sibling_5 = ast_add_sibling(last_sibling_5, init);
        }
        ast_inheritflags(parent_5);
        assign = parent_4;
        let mut local: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_1: *mut ast_t = idseq;
        let mut parent_7: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
        let mut node_7: *mut ast_t = 0 as *mut ast_t;
        node_7 = ast_from(basis_ast_1, TK_ASSIGN);
        if parent_7.is_null() {
            parent_7 = node_7;
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, node_7);
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, node_7);
        }
        let mut parent_8: *mut ast_t = node_7;
        let mut last_sibling_8: *mut ast_t = 0 as *mut ast_t;
        let mut node_8: *mut ast_t = 0 as *mut ast_t;
        if parent_8.is_null() {
            parent_8 = idseq;
        } else if last_sibling_8.is_null() {
            last_sibling_8 = ast_add(parent_8, idseq);
        } else {
            last_sibling_8 = ast_add_sibling(last_sibling_8, idseq);
        }
        node_8 = ast_from(basis_ast_1, TK_REFERENCE);
        if parent_8.is_null() {
            parent_8 = node_8;
        } else if last_sibling_8.is_null() {
            last_sibling_8 = ast_add(parent_8, node_8);
        } else {
            last_sibling_8 = ast_add_sibling(last_sibling_8, node_8);
        }
        let mut parent_9: *mut ast_t = node_8;
        let mut last_sibling_9: *mut ast_t = 0 as *mut ast_t;
        let mut _node_9: *mut ast_t = 0 as *mut ast_t;
        if parent_9.is_null() {
            parent_9 = ast_from_string(basis_ast_1, init_name);
        } else if last_sibling_9.is_null() {
            last_sibling_9 = ast_add(parent_9, ast_from_string(basis_ast_1, init_name));
        } else {
            last_sibling_9 =
                ast_add_sibling(last_sibling_9, ast_from_string(basis_ast_1, init_name));
        }
        ast_inheritflags(parent_9);
        ast_inheritflags(parent_8);
        local = parent_7;
        ast_add(replace, assign);
        ast_add(dbody, local);
        build_with_dispose(dexit, idseq);
        ast_add(dexit, local);
        p = ast_sibling(p);
    }
    ast_replace(astp, replace);
    return AST_OK;
}
#[c2rust::src_loc = "643:1"]
unsafe extern "C" fn sugar_match_capture(
    mut opt: *mut pass_opt_t,
    mut pattern: *mut ast_t,
) -> bool {
    match ast_id(pattern) as libc::c_uint {
        84 => {
            ast_error(
                (*opt).check.errors,
                pattern,
                b"match captures may not be declared with `var`, use `let`\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        85 => {
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut capture_type: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] =
                [&mut id, &mut capture_type, 0 as *mut *mut ast_t];
            ast_get_children(
                pattern,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children.as_mut_ptr(),
            );
            if ast_id(capture_type) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    pattern,
                    b"capture types cannot be inferred, please specify type of %s\0" as *const u8
                        as *const libc::c_char,
                    ast_name(id),
                );
                return 0 as libc::c_int != 0;
            }
            if ast_id(capture_type) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
                ast_error(
                    (*opt).check.errors,
                    capture_type,
                    b"can't capture a tuple, change this into a tuple of capture expressions\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            ast_setid(pattern, TK_MATCH_CAPTURE);
            return 1 as libc::c_int != 0;
        }
        178 => {
            let mut r: bool = 1 as libc::c_int != 0;
            let mut p: *mut ast_t = ast_child(pattern);
            while !p.is_null() {
                if !sugar_match_capture(opt, p) {
                    r = 0 as libc::c_int != 0;
                }
                p = ast_sibling(p);
            }
            return r;
        }
        175 => {
            if ast_childcount(pattern) != 1 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int != 0;
            }
            return sugar_match_capture(opt, ast_child(pattern));
        }
        _ => return 1 as libc::c_int != 0,
    };
}
#[c2rust::src_loc = "709:1"]
unsafe extern "C" fn sugar_case(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut r: ast_result_t = AST_OK;
    let mut pattern: ast_ptr_t = 0 as *mut ast_t;
    let mut guard: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut pattern, &mut guard, &mut body, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if !sugar_match_capture(opt, pattern) {
        r = AST_ERROR;
    }
    if ast_id(body) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        return r;
    }
    let mut next: *mut ast_t = ast;
    let mut next_body: *mut ast_t = body;
    while ast_id(next_body) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
        next = ast_sibling(next);
        if !next.is_null() {
        } else {
            ponyint_assert_fail(
                b"next != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                728 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sugar_case\0"))
                    .as_ptr(),
            );
        };
        if ast_id(next) as libc::c_uint == TK_CASE as libc::c_int as libc::c_uint {
        } else {
            ponyint_assert_fail(
                b"ast_id(next) == TK_CASE\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                729 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sugar_case\0"))
                    .as_ptr(),
            );
        };
        next_body = ast_childidx(next, 2 as libc::c_int as usize);
    }
    ast_replace(&mut body, next_body);
    return r;
}
#[c2rust::src_loc = "738:1"]
unsafe extern "C" fn sugar_update(mut astp: *mut *mut ast_t) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    if ast_id(ast) as libc::c_uint == TK_ASSIGN as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_ASSIGN\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            741 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sugar_update\0")).as_ptr(),
        );
    };
    let mut call: ast_ptr_t = 0 as *mut ast_t;
    let mut value: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut call, &mut value, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(call) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint {
        return AST_OK;
    }
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 5] = [
        &mut expr,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_extract_children(
        call,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    ast_setid(named, TK_NAMEDARGS);
    let mut namedarg: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = ast;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_UPDATEARG);
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
        parent_0 = ast_from_string(basis_ast, b"value\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(
            parent_0,
            ast_from_string(basis_ast, b"value\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_0 = ast_add_sibling(
            last_sibling_0,
            ast_from_string(basis_ast, b"value\0" as *const u8 as *const libc::c_char),
        );
    }
    node_0 = ast_from(basis_ast, TK_SEQ);
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
        parent_1 = value;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, value);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, value);
    }
    ast_inheritflags(parent_1);
    ast_inheritflags(parent_0);
    namedarg = parent;
    ast_append(named, namedarg);
    let mut basis_ast_0: *mut ast_t = *astp;
    let mut parent_2: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_2: *mut ast_t = 0 as *mut ast_t;
    let mut node_2: *mut ast_t = 0 as *mut ast_t;
    node_2 = ast_from(basis_ast_0, TK_CALL);
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
    let mut _node_4: *mut ast_t = 0 as *mut ast_t;
    if parent_4.is_null() {
        parent_4 = expr;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, expr);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, expr);
    }
    if parent_4.is_null() {
        parent_4 = ast_from_string(basis_ast_0, b"update\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(
            parent_4,
            ast_from_string(basis_ast_0, b"update\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_4 = ast_add_sibling(
            last_sibling_4,
            ast_from_string(basis_ast_0, b"update\0" as *const u8 as *const libc::c_char),
        );
    }
    ast_inheritflags(parent_4);
    if parent_3.is_null() {
        parent_3 = positional;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, positional);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, positional);
    }
    if parent_3.is_null() {
        parent_3 = named;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, named);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, named);
    }
    if parent_3.is_null() {
        parent_3 = question;
    } else if last_sibling_3.is_null() {
        last_sibling_3 = ast_add(parent_3, question);
    } else {
        last_sibling_3 = ast_add_sibling(last_sibling_3, question);
    }
    ast_inheritflags(parent_3);
    ast_replace(astp, parent_2);
    return AST_OK;
}
#[c2rust::src_loc = "776:1"]
unsafe extern "C" fn sugar_as(
    mut _opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    if ast_id(ast) as libc::c_uint == TK_AS as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_AS\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            780 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"sugar_as\0")).as_ptr(),
        );
    };
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut type_0: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut expr, &mut type_0, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(type_0) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
        let mut new_type: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = type_0;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_TUPLETYPE);
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
        new_type = parent;
        let mut p: *mut ast_t = ast_child(type_0);
        while !p.is_null() {
            if ast_id(p) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint
                && is_name_dontcare(ast_name(ast_childidx(p, 1 as libc::c_int as usize)))
                    as libc::c_int
                    != 0
            {
                let mut dontcare: *mut ast_t = 0 as *mut ast_t;
                let mut basis_ast_0: *mut ast_t = new_type;
                let mut parent_1: *mut ast_t = 0 as *mut ast_t;
                let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
                let mut node_1: *mut ast_t = 0 as *mut ast_t;
                node_1 = ast_from(basis_ast_0, TK_DONTCARETYPE);
                if parent_1.is_null() {
                    parent_1 = node_1;
                } else if last_sibling_1.is_null() {
                    last_sibling_1 = ast_add(parent_1, node_1);
                } else {
                    last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
                }
                let mut parent_2: *mut ast_t = node_1;
                let mut _last_sibling_2: *mut ast_t = 0 as *mut ast_t;
                let mut _node_2: *mut ast_t = 0 as *mut ast_t;
                ast_inheritflags(parent_2);
                dontcare = parent_1;
                ast_append(new_type, dontcare);
            } else {
                ast_append(new_type, p);
            }
            p = ast_sibling(p);
        }
        let mut basis_ast_1: *mut ast_t = *astp;
        let mut parent_3: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
        let mut node_3: *mut ast_t = 0 as *mut ast_t;
        node_3 = ast_from(basis_ast_1, TK_AS);
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
            parent_4 = expr;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, expr);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, expr);
        }
        if parent_4.is_null() {
            parent_4 = new_type;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, new_type);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, new_type);
        }
        ast_inheritflags(parent_4);
        ast_replace(astp, parent_3);
    }
    return AST_OK;
}
#[c2rust::src_loc = "807:1"]
unsafe extern "C" fn sugar_binop(
    mut astp: *mut *mut ast_t,
    mut fn_name: *const libc::c_char,
    mut fn_name_partial: *const libc::c_char,
) -> ast_result_t {
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] =
        [&mut left, &mut right, &mut question, 0 as *mut *mut ast_t];
    ast_get_children(
        *astp,
        (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut positional: *mut ast_t = ast_from(right, TK_POSITIONALARGS);
    if ast_id(right) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
        let mut value: *mut ast_t = ast_child(right);
        while !value.is_null() {
            let mut arg: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast: *mut ast_t = right;
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
            let mut _node_0: *mut ast_t = 0 as *mut ast_t;
            if parent_0.is_null() {
                parent_0 = value;
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(parent_0, value);
            } else {
                last_sibling_0 = ast_add_sibling(last_sibling_0, value);
            }
            ast_inheritflags(parent_0);
            arg = parent;
            ast_append(positional, arg);
            value = ast_sibling(value);
        }
    } else {
        let mut arg_0: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_0: *mut ast_t = right;
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
            parent_2 = right;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, right);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, right);
        }
        ast_inheritflags(parent_2);
        arg_0 = parent_1;
        ast_add(positional, arg_0);
    }
    let mut name: *const libc::c_char = fn_name;
    if !fn_name_partial.is_null()
        && ast_id(question) as libc::c_uint == TK_QUESTION as libc::c_int as libc::c_uint
    {
        name = fn_name_partial;
    }
    let mut basis_ast_1: *mut ast_t = *astp;
    let mut parent_3: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
    let mut node_3: *mut ast_t = 0 as *mut ast_t;
    node_3 = ast_from(basis_ast_1, TK_CALL);
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
    node_4 = ast_from(basis_ast_1, TK_DOT);
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
        parent_5 = left;
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(parent_5, left);
    } else {
        last_sibling_5 = ast_add_sibling(last_sibling_5, left);
    }
    if parent_5.is_null() {
        parent_5 = ast_from_string(basis_ast_1, name);
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(parent_5, ast_from_string(basis_ast_1, name));
    } else {
        last_sibling_5 = ast_add_sibling(last_sibling_5, ast_from_string(basis_ast_1, name));
    }
    ast_inheritflags(parent_5);
    if parent_4.is_null() {
        parent_4 = positional;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, positional);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, positional);
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast_1, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = question;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, question);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, question);
    }
    ast_inheritflags(parent_4);
    ast_replace(astp, parent_3);
    return AST_OK;
}
#[c2rust::src_loc = "843:1"]
unsafe extern "C" fn sugar_unop(
    mut astp: *mut *mut ast_t,
    mut fn_name: *const libc::c_char,
) -> ast_result_t {
    let mut expr: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 2] = [&mut expr, 0 as *mut *mut ast_t];
    ast_get_children(
        *astp,
        (::core::mem::size_of::<[*mut *mut ast_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut basis_ast: *mut ast_t = *astp;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_CALL);
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
    let mut _node_1: *mut ast_t = 0 as *mut ast_t;
    if parent_1.is_null() {
        parent_1 = expr;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, expr);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, expr);
    }
    if parent_1.is_null() {
        parent_1 = ast_from_string(basis_ast, fn_name);
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, ast_from_string(basis_ast, fn_name));
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, ast_from_string(basis_ast, fn_name));
    }
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
    if parent_0.is_null() {
        parent_0 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_0);
    ast_replace(astp, parent);
    return AST_OK;
}
#[c2rust::src_loc = "858:1"]
unsafe extern "C" fn sugar_ffi(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut args: ast_ptr_t = 0 as *mut ast_t;
    let mut named_args: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut id,
        &mut typeargs,
        &mut args,
        &mut named_args,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut name: *const libc::c_char = ast_name(id);
    let mut len: usize = ast_name_len(id);
    if !(memchr(name as *const libc::c_void, '\0' as i32, len)).is_null() {
        ast_error(
            (*opt).check.errors,
            ast,
            b"FFI function names cannot include null characters\0" as *const u8
                as *const libc::c_char,
        );
        return AST_ERROR;
    }
    let mut new_name: *mut libc::c_char =
        ponyint_pool_alloc_size(len.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    *new_name.offset(0 as libc::c_int as isize) = '@' as i32 as libc::c_char;
    memcpy(
        new_name.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        name as *const libc::c_void,
        len,
    );
    *new_name.offset(len.wrapping_add(1) as isize) =
        '\0' as i32 as libc::c_char;
    let mut new_id: *mut ast_t = ast_from_string(
        id,
        stringtab_consume(
            new_name,
            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
        ),
    );
    ast_replace(&mut id, new_id);
    return AST_OK;
}
#[c2rust::src_loc = "886:1"]
unsafe extern "C" fn sugar_ifdef(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !opt.is_null() {
    } else {
        ponyint_assert_fail(
            b"opt != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            888 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"sugar_ifdef\0")).as_ptr(),
        );
    };
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            889 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"sugar_ifdef\0")).as_ptr(),
        );
    };
    let mut cond: ast_ptr_t = 0 as *mut ast_t;
    let mut then_block: ast_ptr_t = 0 as *mut ast_t;
    let mut else_block: ast_ptr_t = 0 as *mut ast_t;
    let mut else_cond: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut cond,
        &mut then_block,
        &mut else_block,
        &mut else_cond,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut parent_ifdef_cond: *mut ast_t = (*(*opt).check.frame).ifdef_cond;
    if !parent_ifdef_cond.is_null() {
        if ast_id(ast_parent(parent_ifdef_cond)) as libc::c_uint
            == TK_IFDEF as libc::c_int as libc::c_uint
        {
        } else {
            ponyint_assert_fail(
                b"ast_id(ast_parent(parent_ifdef_cond)) == TK_IFDEF\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0"
                    as *const u8 as *const libc::c_char,
                899 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"sugar_ifdef\0"))
                    .as_ptr(),
            );
        };
        let mut basis_ast: *mut ast_t = else_cond;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_AND);
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
            parent_0 = parent_ifdef_cond;
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, parent_ifdef_cond);
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, parent_ifdef_cond);
        }
        node_0 = ast_from(basis_ast, TK_NOT);
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
            parent_1 = cond;
        } else if last_sibling_1.is_null() {
            last_sibling_1 = ast_add(parent_1, cond);
        } else {
            last_sibling_1 = ast_add_sibling(last_sibling_1, cond);
        }
        ast_inheritflags(parent_1);
        node_0 = ast_from(basis_ast, TK_NONE);
        if parent_0.is_null() {
            parent_0 = node_0;
        } else if last_sibling_0.is_null() {
            last_sibling_0 = ast_add(parent_0, node_0);
        } else {
            last_sibling_0 = ast_add_sibling(last_sibling_0, node_0);
        }
        let mut parent_2: *mut ast_t = node_0;
        let mut _last_sibling_2: *mut ast_t = 0 as *mut ast_t;
        let mut _node_2: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_2);
        ast_inheritflags(parent_0);
        ast_replace(&mut else_cond, parent);
        let mut basis_ast_0: *mut ast_t = cond;
        let mut parent_3: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
        let mut node_3: *mut ast_t = 0 as *mut ast_t;
        node_3 = ast_from(basis_ast_0, TK_AND);
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
        if parent_4.is_null() {
            parent_4 = parent_ifdef_cond;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, parent_ifdef_cond);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, parent_ifdef_cond);
        }
        if parent_4.is_null() {
            parent_4 = cond;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, cond);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, cond);
        }
        node_4 = ast_from(basis_ast_0, TK_NONE);
        if parent_4.is_null() {
            parent_4 = node_4;
        } else if last_sibling_4.is_null() {
            last_sibling_4 = ast_add(parent_4, node_4);
        } else {
            last_sibling_4 = ast_add_sibling(last_sibling_4, node_4);
        }
        let mut parent_5: *mut ast_t = node_4;
        let mut _last_sibling_5: *mut ast_t = 0 as *mut ast_t;
        let mut _node_5: *mut ast_t = 0 as *mut ast_t;
        ast_inheritflags(parent_5);
        ast_inheritflags(parent_4);
        ast_replace(&mut cond, parent_3);
    } else {
        let mut basis_ast_1: *mut ast_t = else_cond;
        let mut parent_6: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
        let mut node_6: *mut ast_t = 0 as *mut ast_t;
        node_6 = ast_from(basis_ast_1, TK_NOT);
        if parent_6.is_null() {
            parent_6 = node_6;
        } else if last_sibling_6.is_null() {
            last_sibling_6 = ast_add(parent_6, node_6);
        } else {
            last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
        }
        let mut parent_7: *mut ast_t = node_6;
        let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
        let mut _node_7: *mut ast_t = 0 as *mut ast_t;
        if parent_7.is_null() {
            parent_7 = cond;
        } else if last_sibling_7.is_null() {
            last_sibling_7 = ast_add(parent_7, cond);
        } else {
            last_sibling_7 = ast_add_sibling(last_sibling_7, cond);
        }
        ast_inheritflags(parent_7);
        ast_replace(&mut else_cond, parent_6);
    }
    if !ifdef_cond_normalise(&mut cond, opt) {
        ast_error(
            (*opt).check.errors,
            ast,
            b"ifdef condition will never be true\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    if !ifdef_cond_normalise(&mut else_cond, opt) {
        ast_error(
            (*opt).check.errors,
            ast,
            b"ifdef condition is always true\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    return sugar_else(ast, 2 as libc::c_int as usize);
}
#[c2rust::src_loc = "937:1"]
unsafe extern "C" fn sugar_use(mut opt: *mut pass_opt_t, mut ast: *mut ast_t) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            939 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"sugar_use\0")).as_ptr(),
        );
    };
    let mut guard: *mut ast_t = ast_childidx(ast, 2 as libc::c_int as usize);
    if !ifdef_cond_normalise(&mut guard, opt) {
        ast_error(
            (*opt).check.errors,
            ast,
            b"use guard condition will never be true\0" as *const u8 as *const libc::c_char,
        );
        return AST_ERROR;
    }
    return AST_OK;
}
#[c2rust::src_loc = "955:1"]
unsafe extern "C" fn sugar_semi(
    mut options: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    if ast_id(ast) as libc::c_uint == TK_SEMI as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_SEMI\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            958 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sugar_semi\0")).as_ptr(),
        );
    };
    if (ast_child(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_child(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            961 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sugar_semi\0")).as_ptr(),
        );
    };
    *astp = ast_sibling(ast);
    ast_remove(ast);
    return pass_sugar(astp, options);
}
#[c2rust::src_loc = "971:1"]
unsafe extern "C" fn sugar_lambdatype(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            973 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sugar_lambdatype\0"))
                .as_ptr(),
        );
    };
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            975 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sugar_lambdatype\0"))
                .as_ptr(),
        );
    };
    let mut apply_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut apply_name: ast_ptr_t = 0 as *mut ast_t;
    let mut apply_t_params: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut ret_type: ast_ptr_t = 0 as *mut ast_t;
    let mut error: ast_ptr_t = 0 as *mut ast_t;
    let mut interface_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut ephemeral: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 9] = [
        &mut apply_cap,
        &mut apply_name,
        &mut apply_t_params,
        &mut params,
        &mut ret_type,
        &mut error,
        &mut interface_cap,
        &mut ephemeral,
        0 as *mut *mut ast_t,
    ];
    ast_extract_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut bare: bool =
        ast_id(ast) as libc::c_uint == TK_BARELAMBDATYPE as libc::c_int as libc::c_uint;
    if bare {
        ast_setid(apply_cap, TK_AT);
        ast_setid(interface_cap, TK_VAL);
    }
    let mut i_name: *const libc::c_char = package_hygienic_id(&mut (*opt).check);
    let mut interface_t_params: *mut ast_t = 0 as *mut ast_t;
    let mut t_args: *mut ast_t = 0 as *mut ast_t;
    collect_type_params(ast, 0 as *mut *mut ast_t, &mut t_args);
    let mut basis_ast: *mut ast_t = *astp;
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
        parent_0 = ast_from_string(basis_ast, i_name);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_from_string(basis_ast, i_name));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_from_string(basis_ast, i_name));
    }
    if parent_0.is_null() {
        parent_0 = t_args;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, t_args);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, t_args);
    }
    if parent_0.is_null() {
        parent_0 = interface_cap;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, interface_cap);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, interface_cap);
    }
    if parent_0.is_null() {
        parent_0 = ephemeral;
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ephemeral);
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ephemeral);
    }
    ast_inheritflags(parent_0);
    ast_replace(astp, parent);
    ast = *astp;
    collect_type_params(ast, &mut interface_t_params, 0 as *mut *mut ast_t);
    let mut buf: *mut printbuf_t = printbuf_new();
    if ast_id(apply_cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        printbuf(buf, b"@{(\0" as *const u8 as *const libc::c_char);
    } else if ast_id(apply_cap) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        printbuf(
            buf,
            b"{%s(\0" as *const u8 as *const libc::c_char,
            ast_print_type(apply_cap),
        );
    } else {
        printbuf(buf, b"{(\0" as *const u8 as *const libc::c_char);
    }
    let mut p_no: libc::c_int = 1 as libc::c_int;
    let mut p: *mut ast_t = ast_child(params);
    while !p.is_null() {
        if p_no > 1 as libc::c_int {
            printbuf(buf, b", \0" as *const u8 as *const libc::c_char);
        }
        printbuf(
            buf,
            b"%s\0" as *const u8 as *const libc::c_char,
            ast_print_type(p),
        );
        let mut name: [libc::c_char; 12] = [0; 12];
        snprintf(
            name.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"p%d\0" as *const u8 as *const libc::c_char,
            p_no,
        );
        let mut basis_ast_0: *mut ast_t = p;
        let mut parent_1: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
        let mut node_1: *mut ast_t = 0 as *mut ast_t;
        node_1 = ast_from(basis_ast_0, TK_PARAM);
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
            parent_2 = ast_from_string(basis_ast_0, name.as_mut_ptr());
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, ast_from_string(basis_ast_0, name.as_mut_ptr()));
        } else {
            last_sibling_2 = ast_add_sibling(
                last_sibling_2,
                ast_from_string(basis_ast_0, name.as_mut_ptr()),
            );
        }
        if parent_2.is_null() {
            parent_2 = p;
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, p);
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, p);
        }
        if parent_2.is_null() {
            parent_2 = ast_from(basis_ast_0, TK_NONE);
        } else if last_sibling_2.is_null() {
            last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
        } else {
            last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
        }
        ast_inheritflags(parent_2);
        ast_replace(&mut p, parent_1);
        p_no += 1;
        p = ast_sibling(p);
    }
    printbuf(buf, b")\0" as *const u8 as *const libc::c_char);
    if ast_id(ret_type) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        printbuf(
            buf,
            b": %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(ret_type),
        );
    }
    if ast_id(error) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        printbuf(buf, b" ?\0" as *const u8 as *const libc::c_char);
    }
    printbuf(buf, b"}\0" as *const u8 as *const libc::c_char);
    let mut fn_name: *const libc::c_char = b"apply\0" as *const u8 as *const libc::c_char;
    if ast_id(apply_name) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
        fn_name = ast_name(apply_name);
    }
    ast_setdata(
        ast_childidx(ast, 1 as libc::c_int as usize),
        stringtab((*buf).m) as *mut libc::c_void,
    );
    let mut def: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast_1: *mut ast_t = ast;
    let mut parent_3: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling_3: *mut ast_t = 0 as *mut ast_t;
    let mut node_3: *mut ast_t = 0 as *mut ast_t;
    node_3 = ast_from(basis_ast_1, TK_INTERFACE);
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
    ast_scope(parent_4);
    if parent_4.is_null() {
        parent_4 = ast_setdata(
            ast_from_string(basis_ast_1, i_name),
            stringtab((*buf).m) as *mut libc::c_void,
        );
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(
            parent_4,
            ast_setdata(
                ast_from_string(basis_ast_1, i_name),
                stringtab((*buf).m) as *mut libc::c_void,
            ),
        );
    } else {
        last_sibling_4 = ast_add_sibling(
            last_sibling_4,
            ast_setdata(
                ast_from_string(basis_ast_1, i_name),
                stringtab((*buf).m) as *mut libc::c_void,
            ),
        );
    }
    if parent_4.is_null() {
        parent_4 = interface_t_params;
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, interface_t_params);
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, interface_t_params);
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast_1, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast_1, TK_NONE));
    }
    node_4 = ast_from(basis_ast_1, TK_MEMBERS);
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
    node_5 = ast_from(basis_ast_1, TK_FUN);
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
    ast_scope(parent_6);
    if parent_6.is_null() {
        parent_6 = apply_cap;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, apply_cap);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, apply_cap);
    }
    if parent_6.is_null() {
        parent_6 = ast_from_string(basis_ast_1, fn_name);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from_string(basis_ast_1, fn_name));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from_string(basis_ast_1, fn_name));
    }
    if parent_6.is_null() {
        parent_6 = apply_t_params;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, apply_t_params);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, apply_t_params);
    }
    if parent_6.is_null() {
        parent_6 = params;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, params);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, params);
    }
    if parent_6.is_null() {
        parent_6 = ret_type;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ret_type);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ret_type);
    }
    if parent_6.is_null() {
        parent_6 = error;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, error);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, error);
    }
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast_1, TK_NONE));
    }
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast_1, TK_NONE));
    }
    ast_inheritflags(parent_6);
    ast_inheritflags(parent_5);
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast_1, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast_1, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast_1, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast_1, TK_NONE));
    }
    ast_inheritflags(parent_4);
    def = parent_3;
    printbuf_free(buf);
    if bare {
        let mut bare_annotation: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast_2: *mut ast_t = def;
        let mut parent_7: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling_7: *mut ast_t = 0 as *mut ast_t;
        let mut node_7: *mut ast_t = 0 as *mut ast_t;
        node_7 = ast_from(basis_ast_2, TK_ANNOTATION);
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
            parent_8 = ast_from_string(
                basis_ast_2,
                b"ponyint_bare\0" as *const u8 as *const libc::c_char,
            );
        } else if last_sibling_8.is_null() {
            last_sibling_8 = ast_add(
                parent_8,
                ast_from_string(
                    basis_ast_2,
                    b"ponyint_bare\0" as *const u8 as *const libc::c_char,
                ),
            );
        } else {
            last_sibling_8 = ast_add_sibling(
                last_sibling_8,
                ast_from_string(
                    basis_ast_2,
                    b"ponyint_bare\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        ast_inheritflags(parent_8);
        bare_annotation = parent_7;
        ast_pass_record(bare_annotation, PASS_SYNTAX);
        ast_setannotation(def, bare_annotation);
    }
    let mut module: *mut ast_t = ast_nearest(ast, TK_MODULE);
    ast_append(module, def);
    if !ast_passes_type(&mut def, opt, (*opt).program_pass) {
        return AST_FATAL;
    }
    if !ast_passes_subtree(astp, opt, PASS_SUGAR) {
        return AST_FATAL;
    }
    return AST_OK;
}
#[c2rust::src_loc = "1107:1"]
unsafe extern "C" fn sugar_barelambda(
    mut _opt: *mut pass_opt_t,
    mut ast: *mut ast_t,
) -> ast_result_t {
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            1111 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sugar_barelambda\0"))
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
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    ast_setid(receiver_cap, TK_AT);
    ast_setid(reference_cap, TK_VAL);
    return AST_OK;
}
#[no_mangle]
#[c2rust::src_loc = "1123:1"]
pub unsafe extern "C" fn expand_location(mut location: *mut ast_t) -> *mut ast_t {
    if !location.is_null() {
    } else {
        ponyint_assert_fail(
            b"location != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            1125 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"expand_location\0"))
                .as_ptr(),
        );
    };
    let mut file_name: *const libc::c_char = (*ast_source(location)).file;
    if file_name.is_null() {
        file_name = b"\0" as *const u8 as *const libc::c_char;
    }
    let mut method_name: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut method: *mut ast_t = location;
    while !method.is_null() {
        let mut variety: token_id = ast_id(method);
        if variety as libc::c_uint == TK_FUN as libc::c_int as libc::c_uint
            || variety as libc::c_uint == TK_BE as libc::c_int as libc::c_uint
            || variety as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
        {
            method_name = ast_name(ast_childidx(method, 1 as libc::c_int as usize));
            break;
        } else {
            method = ast_parent(method);
        }
    }
    let mut type_name: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut typ: *mut ast_t = location;
    while !typ.is_null() {
        let mut variety_0: token_id = ast_id(typ);
        if variety_0 as libc::c_uint == TK_INTERFACE as libc::c_int as libc::c_uint
            || variety_0 as libc::c_uint == TK_TRAIT as libc::c_int as libc::c_uint
            || variety_0 as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint
            || variety_0 as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint
            || variety_0 as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint
            || variety_0 as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint
        {
            type_name = ast_name(ast_child(typ));
            break;
        } else {
            typ = ast_parent(typ);
        }
    }
    let mut ast: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = location;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_OBJECT);
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
    ast_setdata(
        parent_0,
        b"__loc\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
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
    node_0 = ast_from(basis_ast, TK_MEMBERS);
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
    node_1 = ast_from(basis_ast, TK_FUN);
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
    node_2 = ast_from(basis_ast, TK_TAG);
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
        parent_2 = ast_from_string(basis_ast, b"file\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(
            parent_2,
            ast_from_string(basis_ast, b"file\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_2 = ast_add_sibling(
            last_sibling_2,
            ast_from_string(basis_ast, b"file\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    node_2 = ast_from(basis_ast, TK_NOMINAL);
    if parent_2.is_null() {
        parent_2 = node_2;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, node_2);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
    }
    let mut parent_4: *mut ast_t = node_2;
    let mut last_sibling_4: *mut ast_t = 0 as *mut ast_t;
    let mut _node_4: *mut ast_t = 0 as *mut ast_t;
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(
            parent_4,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_4 = ast_add_sibling(
            last_sibling_4,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    if parent_4.is_null() {
        parent_4 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_4.is_null() {
        last_sibling_4 = ast_add(parent_4, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_4 = ast_add_sibling(last_sibling_4, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_4);
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    node_2 = ast_from(basis_ast, TK_SEQ);
    if parent_2.is_null() {
        parent_2 = node_2;
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, node_2);
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, node_2);
    }
    let mut parent_5: *mut ast_t = node_2;
    let mut last_sibling_5: *mut ast_t = 0 as *mut ast_t;
    let mut _node_5: *mut ast_t = 0 as *mut ast_t;
    if parent_5.is_null() {
        parent_5 = ast_setid(ast_from_string(basis_ast, file_name), TK_STRING);
    } else if last_sibling_5.is_null() {
        last_sibling_5 = ast_add(
            parent_5,
            ast_setid(ast_from_string(basis_ast, file_name), TK_STRING),
        );
    } else {
        last_sibling_5 = ast_add_sibling(
            last_sibling_5,
            ast_setid(ast_from_string(basis_ast, file_name), TK_STRING),
        );
    }
    ast_inheritflags(parent_5);
    if parent_2.is_null() {
        parent_2 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_2.is_null() {
        last_sibling_2 = ast_add(parent_2, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_2);
    node_1 = ast_from(basis_ast, TK_FUN);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_6: *mut ast_t = node_1;
    let mut last_sibling_6: *mut ast_t = 0 as *mut ast_t;
    let mut node_6: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_6);
    node_6 = ast_from(basis_ast, TK_TAG);
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
    if parent_6.is_null() {
        parent_6 = ast_from_string(
            basis_ast,
            b"type_name\0" as *const u8 as *const libc::c_char,
        );
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(
            parent_6,
            ast_from_string(
                basis_ast,
                b"type_name\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        last_sibling_6 = ast_add_sibling(
            last_sibling_6,
            ast_from_string(
                basis_ast,
                b"type_name\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast, TK_NONE));
    }
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast, TK_NONE));
    }
    node_6 = ast_from(basis_ast, TK_NOMINAL);
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
        parent_8 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, ast_from(basis_ast, TK_NONE));
    }
    if parent_8.is_null() {
        parent_8 = ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(
            parent_8,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_8 = ast_add_sibling(
            last_sibling_8,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_8.is_null() {
        parent_8 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, ast_from(basis_ast, TK_NONE));
    }
    if parent_8.is_null() {
        parent_8 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, ast_from(basis_ast, TK_NONE));
    }
    if parent_8.is_null() {
        parent_8 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_8.is_null() {
        last_sibling_8 = ast_add(parent_8, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_8 = ast_add_sibling(last_sibling_8, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_8);
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast, TK_NONE));
    }
    node_6 = ast_from(basis_ast, TK_SEQ);
    if parent_6.is_null() {
        parent_6 = node_6;
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, node_6);
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, node_6);
    }
    let mut parent_9: *mut ast_t = node_6;
    let mut last_sibling_9: *mut ast_t = 0 as *mut ast_t;
    let mut _node_9: *mut ast_t = 0 as *mut ast_t;
    if parent_9.is_null() {
        parent_9 = ast_setid(ast_from_string(basis_ast, type_name), TK_STRING);
    } else if last_sibling_9.is_null() {
        last_sibling_9 = ast_add(
            parent_9,
            ast_setid(ast_from_string(basis_ast, type_name), TK_STRING),
        );
    } else {
        last_sibling_9 = ast_add_sibling(
            last_sibling_9,
            ast_setid(ast_from_string(basis_ast, type_name), TK_STRING),
        );
    }
    ast_inheritflags(parent_9);
    if parent_6.is_null() {
        parent_6 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_6.is_null() {
        last_sibling_6 = ast_add(parent_6, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_6 = ast_add_sibling(last_sibling_6, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_6);
    node_1 = ast_from(basis_ast, TK_FUN);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_10: *mut ast_t = node_1;
    let mut last_sibling_10: *mut ast_t = 0 as *mut ast_t;
    let mut node_10: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_10);
    node_10 = ast_from(basis_ast, TK_TAG);
    if parent_10.is_null() {
        parent_10 = node_10;
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, node_10);
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, node_10);
    }
    let mut parent_11: *mut ast_t = node_10;
    let mut _last_sibling_11: *mut ast_t = 0 as *mut ast_t;
    let mut _node_11: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_11);
    if parent_10.is_null() {
        parent_10 = ast_from_string(
            basis_ast,
            b"method_name\0" as *const u8 as *const libc::c_char,
        );
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(
            parent_10,
            ast_from_string(
                basis_ast,
                b"method_name\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        last_sibling_10 = ast_add_sibling(
            last_sibling_10,
            ast_from_string(
                basis_ast,
                b"method_name\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if parent_10.is_null() {
        parent_10 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, ast_from(basis_ast, TK_NONE));
    }
    if parent_10.is_null() {
        parent_10 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, ast_from(basis_ast, TK_NONE));
    }
    node_10 = ast_from(basis_ast, TK_NOMINAL);
    if parent_10.is_null() {
        parent_10 = node_10;
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, node_10);
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, node_10);
    }
    let mut parent_12: *mut ast_t = node_10;
    let mut last_sibling_12: *mut ast_t = 0 as *mut ast_t;
    let mut _node_12: *mut ast_t = 0 as *mut ast_t;
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast, TK_NONE));
    }
    if parent_12.is_null() {
        parent_12 = ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(
            parent_12,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_12 = ast_add_sibling(
            last_sibling_12,
            ast_from_string(basis_ast, b"String\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast, TK_NONE));
    }
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast, TK_NONE));
    }
    if parent_12.is_null() {
        parent_12 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_12.is_null() {
        last_sibling_12 = ast_add(parent_12, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_12 = ast_add_sibling(last_sibling_12, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_12);
    if parent_10.is_null() {
        parent_10 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, ast_from(basis_ast, TK_NONE));
    }
    node_10 = ast_from(basis_ast, TK_SEQ);
    if parent_10.is_null() {
        parent_10 = node_10;
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, node_10);
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, node_10);
    }
    let mut parent_13: *mut ast_t = node_10;
    let mut last_sibling_13: *mut ast_t = 0 as *mut ast_t;
    let mut _node_13: *mut ast_t = 0 as *mut ast_t;
    if parent_13.is_null() {
        parent_13 = ast_setid(ast_from_string(basis_ast, method_name), TK_STRING);
    } else if last_sibling_13.is_null() {
        last_sibling_13 = ast_add(
            parent_13,
            ast_setid(ast_from_string(basis_ast, method_name), TK_STRING),
        );
    } else {
        last_sibling_13 = ast_add_sibling(
            last_sibling_13,
            ast_setid(ast_from_string(basis_ast, method_name), TK_STRING),
        );
    }
    ast_inheritflags(parent_13);
    if parent_10.is_null() {
        parent_10 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_10.is_null() {
        last_sibling_10 = ast_add(parent_10, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_10 = ast_add_sibling(last_sibling_10, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_10);
    node_1 = ast_from(basis_ast, TK_FUN);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_14: *mut ast_t = node_1;
    let mut last_sibling_14: *mut ast_t = 0 as *mut ast_t;
    let mut node_14: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_14);
    node_14 = ast_from(basis_ast, TK_TAG);
    if parent_14.is_null() {
        parent_14 = node_14;
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, node_14);
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, node_14);
    }
    let mut parent_15: *mut ast_t = node_14;
    let mut _last_sibling_15: *mut ast_t = 0 as *mut ast_t;
    let mut _node_15: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_15);
    if parent_14.is_null() {
        parent_14 = ast_from_string(basis_ast, b"line\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(
            parent_14,
            ast_from_string(basis_ast, b"line\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_14 = ast_add_sibling(
            last_sibling_14,
            ast_from_string(basis_ast, b"line\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast, TK_NONE));
    }
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast, TK_NONE));
    }
    node_14 = ast_from(basis_ast, TK_NOMINAL);
    if parent_14.is_null() {
        parent_14 = node_14;
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, node_14);
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, node_14);
    }
    let mut parent_16: *mut ast_t = node_14;
    let mut last_sibling_16: *mut ast_t = 0 as *mut ast_t;
    let mut _node_16: *mut ast_t = 0 as *mut ast_t;
    if parent_16.is_null() {
        parent_16 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(parent_16, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast, TK_NONE));
    }
    if parent_16.is_null() {
        parent_16 = ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(
            parent_16,
            ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_16 = ast_add_sibling(
            last_sibling_16,
            ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_16.is_null() {
        parent_16 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(parent_16, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast, TK_NONE));
    }
    if parent_16.is_null() {
        parent_16 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(parent_16, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast, TK_NONE));
    }
    if parent_16.is_null() {
        parent_16 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_16.is_null() {
        last_sibling_16 = ast_add(parent_16, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_16 = ast_add_sibling(last_sibling_16, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_16);
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast, TK_NONE));
    }
    node_14 = ast_from(basis_ast, TK_SEQ);
    if parent_14.is_null() {
        parent_14 = node_14;
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, node_14);
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, node_14);
    }
    let mut parent_17: *mut ast_t = node_14;
    let mut last_sibling_17: *mut ast_t = 0 as *mut ast_t;
    let mut _node_17: *mut ast_t = 0 as *mut ast_t;
    if parent_17.is_null() {
        parent_17 = ast_from_int(basis_ast, ast_line(location) as u64);
    } else if last_sibling_17.is_null() {
        last_sibling_17 = ast_add(
            parent_17,
            ast_from_int(basis_ast, ast_line(location) as u64),
        );
    } else {
        last_sibling_17 = ast_add_sibling(
            last_sibling_17,
            ast_from_int(basis_ast, ast_line(location) as u64),
        );
    }
    ast_inheritflags(parent_17);
    if parent_14.is_null() {
        parent_14 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_14.is_null() {
        last_sibling_14 = ast_add(parent_14, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_14 = ast_add_sibling(last_sibling_14, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_14);
    node_1 = ast_from(basis_ast, TK_FUN);
    if parent_1.is_null() {
        parent_1 = node_1;
    } else if last_sibling_1.is_null() {
        last_sibling_1 = ast_add(parent_1, node_1);
    } else {
        last_sibling_1 = ast_add_sibling(last_sibling_1, node_1);
    }
    let mut parent_18: *mut ast_t = node_1;
    let mut last_sibling_18: *mut ast_t = 0 as *mut ast_t;
    let mut node_18: *mut ast_t = 0 as *mut ast_t;
    ast_scope(parent_18);
    node_18 = ast_from(basis_ast, TK_TAG);
    if parent_18.is_null() {
        parent_18 = node_18;
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, node_18);
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, node_18);
    }
    let mut parent_19: *mut ast_t = node_18;
    let mut _last_sibling_19: *mut ast_t = 0 as *mut ast_t;
    let mut _node_19: *mut ast_t = 0 as *mut ast_t;
    ast_inheritflags(parent_19);
    if parent_18.is_null() {
        parent_18 = ast_from_string(basis_ast, b"pos\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(
            parent_18,
            ast_from_string(basis_ast, b"pos\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_18 = ast_add_sibling(
            last_sibling_18,
            ast_from_string(basis_ast, b"pos\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_18.is_null() {
        parent_18 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast, TK_NONE));
    }
    if parent_18.is_null() {
        parent_18 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast, TK_NONE));
    }
    node_18 = ast_from(basis_ast, TK_NOMINAL);
    if parent_18.is_null() {
        parent_18 = node_18;
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, node_18);
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, node_18);
    }
    let mut parent_20: *mut ast_t = node_18;
    let mut last_sibling_20: *mut ast_t = 0 as *mut ast_t;
    let mut _node_20: *mut ast_t = 0 as *mut ast_t;
    if parent_20.is_null() {
        parent_20 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_20.is_null() {
        last_sibling_20 = ast_add(parent_20, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_20 = ast_add_sibling(last_sibling_20, ast_from(basis_ast, TK_NONE));
    }
    if parent_20.is_null() {
        parent_20 = ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char);
    } else if last_sibling_20.is_null() {
        last_sibling_20 = ast_add(
            parent_20,
            ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char),
        );
    } else {
        last_sibling_20 = ast_add_sibling(
            last_sibling_20,
            ast_from_string(basis_ast, b"USize\0" as *const u8 as *const libc::c_char),
        );
    }
    if parent_20.is_null() {
        parent_20 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_20.is_null() {
        last_sibling_20 = ast_add(parent_20, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_20 = ast_add_sibling(last_sibling_20, ast_from(basis_ast, TK_NONE));
    }
    if parent_20.is_null() {
        parent_20 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_20.is_null() {
        last_sibling_20 = ast_add(parent_20, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_20 = ast_add_sibling(last_sibling_20, ast_from(basis_ast, TK_NONE));
    }
    if parent_20.is_null() {
        parent_20 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_20.is_null() {
        last_sibling_20 = ast_add(parent_20, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_20 = ast_add_sibling(last_sibling_20, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_20);
    if parent_18.is_null() {
        parent_18 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast, TK_NONE));
    }
    node_18 = ast_from(basis_ast, TK_SEQ);
    if parent_18.is_null() {
        parent_18 = node_18;
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, node_18);
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, node_18);
    }
    let mut parent_21: *mut ast_t = node_18;
    let mut last_sibling_21: *mut ast_t = 0 as *mut ast_t;
    let mut _node_21: *mut ast_t = 0 as *mut ast_t;
    if parent_21.is_null() {
        parent_21 = ast_from_int(basis_ast, ast_pos(location) as u64);
    } else if last_sibling_21.is_null() {
        last_sibling_21 = ast_add(parent_21, ast_from_int(basis_ast, ast_pos(location) as u64));
    } else {
        last_sibling_21 = ast_add_sibling(
            last_sibling_21,
            ast_from_int(basis_ast, ast_pos(location) as u64),
        );
    }
    ast_inheritflags(parent_21);
    if parent_18.is_null() {
        parent_18 = ast_from(basis_ast, TK_NONE);
    } else if last_sibling_18.is_null() {
        last_sibling_18 = ast_add(parent_18, ast_from(basis_ast, TK_NONE));
    } else {
        last_sibling_18 = ast_add_sibling(last_sibling_18, ast_from(basis_ast, TK_NONE));
    }
    ast_inheritflags(parent_18);
    ast_inheritflags(parent_1);
    ast_inheritflags(parent_0);
    ast = parent;
    return ast;
}
#[c2rust::src_loc = "1201:1"]
unsafe extern "C" fn sugar_location(
    mut opt: *mut pass_opt_t,
    mut astp: *mut *mut ast_t,
) -> ast_result_t {
    if !astp.is_null() {
    } else {
        ponyint_assert_fail(
            b"astp != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            1203 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sugar_location\0"))
                .as_ptr(),
        );
    };
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            1205 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sugar_location\0"))
                .as_ptr(),
        );
    };
    if ast_id(ast_parent(ast_parent(ast))) as libc::c_uint
        == TK_PARAM as libc::c_int as libc::c_uint
    {
        return AST_OK;
    }
    let mut location: *mut ast_t = expand_location(ast);
    ast_replace(astp, location);
    if !ast_passes_subtree(astp, opt, PASS_SUGAR) {
        return AST_FATAL;
    }
    return AST_OK;
}
#[no_mangle]
#[c2rust::src_loc = "1222:1"]
pub unsafe extern "C" fn pass_sugar(
    mut astp: *mut *mut ast_t,
    mut options: *mut pass_opt_t,
) -> ast_result_t {
    let mut ast: *mut ast_t = *astp;
    if !ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/sugar.c\0" as *const u8
                as *const libc::c_char,
            1225 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"pass_sugar\0")).as_ptr(),
        );
    };
    match ast_id(ast) as libc::c_uint {
        138 => return sugar_module(options, ast),
        74 => return sugar_entity(options, ast, 1 as libc::c_int != 0, TK_VAL),
        75 => return sugar_entity(options, ast, 1 as libc::c_int != 0, TK_REF),
        76 => return sugar_entity(options, ast, 1 as libc::c_int != 0, TK_REF),
        77 => return sugar_entity(options, ast, 1 as libc::c_int != 0, TK_TAG),
        73 | 72 => return sugar_entity(options, ast, 0 as libc::c_int != 0, TK_REF),
        163 => return sugar_typeparam(ast),
        88 => return sugar_new(options, ast),
        90 => return sugar_be(options, ast),
        89 => return sugar_fun(options, ast),
        103 => return sugar_return(options, ast),
        108 | 116 | 118 => return sugar_else(ast, 2 as libc::c_int as usize),
        111 => return sugar_else(ast, 1 as libc::c_int as usize),
        124 => return sugar_try(ast),
        120 => return sugar_for(options, astp),
        126 => return sugar_with(options, astp),
        181 => return sugar_case(options, ast),
        24 => return sugar_update(astp),
        81 => return sugar_as(options, astp),
        25 => {
            return sugar_binop(
                astp,
                b"add\0" as *const u8 as *const libc::c_char,
                b"add_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        27 => {
            return sugar_binop(
                astp,
                b"sub\0" as *const u8 as *const libc::c_char,
                b"sub_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        29 => {
            return sugar_binop(
                astp,
                b"mul\0" as *const u8 as *const libc::c_char,
                b"mul_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        31 => {
            return sugar_binop(
                astp,
                b"div\0" as *const u8 as *const libc::c_char,
                b"div_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        33 => {
            return sugar_binop(
                astp,
                b"rem\0" as *const u8 as *const libc::c_char,
                b"rem_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        35 => {
            return sugar_binop(
                astp,
                b"mod\0" as *const u8 as *const libc::c_char,
                b"mod_partial\0" as *const u8 as *const libc::c_char,
            );
        }
        26 => {
            return sugar_binop(
                astp,
                b"add_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        28 => {
            return sugar_binop(
                astp,
                b"sub_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        30 => {
            return sugar_binop(
                astp,
                b"mul_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        32 => {
            return sugar_binop(
                astp,
                b"div_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        34 => {
            return sugar_binop(
                astp,
                b"rem_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        36 => {
            return sugar_binop(
                astp,
                b"mod_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        39 => {
            return sugar_binop(
                astp,
                b"shl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        41 => {
            return sugar_binop(
                astp,
                b"shr\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        40 => {
            return sugar_binop(
                astp,
                b"shl_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        42 => {
            return sugar_binop(
                astp,
                b"shr_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        130 => {
            return sugar_binop(
                astp,
                b"op_and\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        131 => {
            return sugar_binop(
                astp,
                b"op_or\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        132 => {
            return sugar_binop(
                astp,
                b"op_xor\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        51 => {
            return sugar_binop(
                astp,
                b"eq\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        53 => {
            return sugar_binop(
                astp,
                b"ne\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        43 => {
            return sugar_binop(
                astp,
                b"lt\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        45 => {
            return sugar_binop(
                astp,
                b"le\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        47 => {
            return sugar_binop(
                astp,
                b"ge\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        49 => {
            return sugar_binop(
                astp,
                b"gt\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        52 => {
            return sugar_binop(
                astp,
                b"eq_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        54 => {
            return sugar_binop(
                astp,
                b"ne_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        44 => {
            return sugar_binop(
                astp,
                b"lt_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        46 => {
            return sugar_binop(
                astp,
                b"le_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        48 => {
            return sugar_binop(
                astp,
                b"ge_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        50 => {
            return sugar_binop(
                astp,
                b"gt_unsafe\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        61 => return sugar_unop(astp, b"neg\0" as *const u8 as *const libc::c_char),
        62 => {
            return sugar_unop(astp, b"neg_unsafe\0" as *const u8 as *const libc::c_char);
        }
        129 => return sugar_unop(astp, b"op_not\0" as *const u8 as *const libc::c_char),
        142 | 143 => return sugar_ffi(options, ast),
        109 => return sugar_ifdef(options, ast),
        70 => return sugar_use(options, ast),
        23 => return sugar_semi(options, astp),
        154 | 155 => return sugar_lambdatype(options, astp),
        80 => return sugar_barelambda(options, ast),
        135 => return sugar_location(options, astp),
        _ => return AST_OK,
    };
}
