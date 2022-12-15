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
    #[c2rust::src_loc = "9:16"]
    pub use crate::libponyc::ast::ast::ast_t;
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

    use super::error_h::errorframe_t;
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "63:1"]
        pub fn ast_dup(ast: *mut ast_t) -> *mut ast_t;
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
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "105:1"]
        pub fn ast_has_annotation(ast: *mut ast_t, name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: usize) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "156:1"]
        pub fn ast_print_type(type_0: *mut ast_t) -> *const libc::c_char;
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

    use super::frame_h::typecheck_t;
    use super::stringtab_h::strlist_t;
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:2"]
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:4"]
pub mod cap_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn is_cap_sub_cap(
            sub: token_id,
            subalias: token_id,
            super_0: token_id,
            supalias: token_id,
        ) -> bool;
        #[c2rust::src_loc = "27:1"]
        pub fn is_cap_sub_cap_constraint(
            sub: token_id,
            subalias: token_id,
            super_0: token_id,
            supalias: token_id,
        ) -> bool;
        #[c2rust::src_loc = "43:1"]
        pub fn is_cap_sub_cap_bound(
            sub: token_id,
            subalias: token_id,
            super_0: token_id,
            supalias: token_id,
        ) -> bool;
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:6"]
pub mod reify_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "23:1"]
        pub fn reify(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
            duplicate: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "26:1"]
        pub fn reify_method_def(
            ast: *mut ast_t,
            typeparams: *mut ast_t,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
        ) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/typeparam.h:7"]
pub mod typeparam_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn typeparam_constraint(typeparamref: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "18:1"]
        pub fn typeparam_upper(typeparamref: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "25:1"]
        pub fn typeparam_lower(typeparamref: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "31:1"]
        pub fn typeparam_set_cap(typeparamref: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.h:8"]
pub mod viewpoint_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn viewpoint_upper(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "22:1"]
        pub fn viewpoint_lower(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "41:1"]
        pub fn viewpoint_reifytypeparam(type_0: *mut ast_t, typeparamref: *mut ast_t)
            -> *mut ast_t;
        #[c2rust::src_loc = "54:1"]
        pub fn viewpoint_reifypair(
            a: *mut ast_t,
            b: *mut ast_t,
            r_a: *mut *mut ast_t,
            r_b: *mut *mut ast_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:11"]
pub mod ponyassert_h {

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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:12"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::alias_h::consume_type;
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_checkflag, ast_child, ast_childcount, ast_childidx,
    ast_data, ast_dup, ast_error_frame, ast_free_unattached, ast_from, ast_get, ast_get_children,
    ast_has_annotation, ast_id, ast_inheritflags, ast_name, ast_pop, ast_print_type, ast_ptr_t,
    ast_setdata, ast_sibling, C2RustUnnamed, AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI,
    AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND, AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2,
    AST_FLAG_ERROR_1, AST_FLAG_ERROR_2, AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT,
    AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS, AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK,
    AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI, AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE,
    AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
use self::cap_h::{cap_fetch, is_cap_sub_cap, is_cap_sub_cap_bound, is_cap_sub_cap_constraint};
pub use self::error_h::{errorframe_t, errormsg_t, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::reify_h::{reify, reify_method_def};
use self::string_h::strcmp;

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
use self::typeparam_h::{
    typeparam_constraint, typeparam_lower, typeparam_set_cap, typeparam_upper,
};
use self::viewpoint_h::{
    viewpoint_lower, viewpoint_reifypair, viewpoint_reifytypeparam, viewpoint_upper,
};
#[c2rust::src_loc = "15:9"]
pub type check_cap_t = libc::c_uint;
#[c2rust::src_loc = "20:3"]
pub const CHECK_CAP_IGNORE: check_cap_t = 3;
#[c2rust::src_loc = "19:3"]
pub const CHECK_CAP_BOUND: check_cap_t = 2;
#[c2rust::src_loc = "18:3"]
pub const CHECK_CAP_EQ: check_cap_t = 1;
#[c2rust::src_loc = "17:3"]
pub const CHECK_CAP_SUB: check_cap_t = 0;
#[thread_local]
#[c2rust::src_loc = "39:35"]
static mut subtype_assume: *mut ast_t = 0 as *const ast_t as *mut ast_t;
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn struct_cant_be_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut entity: *const libc::c_char,
) {
    if errorf.is_null() {
        return;
    }
    ast_error_frame(
        errorf,
        sub,
        b"%s is not a subtype of %s: a struct can't be a subtype of %s\0" as *const u8
            as *const libc::c_char,
        ast_print_type(sub),
        ast_print_type(super_0),
        entity,
    );
}
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn exact_nominal(
    mut a: *mut ast_t,
    mut b: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut a_pkg: ast_ptr_t = 0 as *mut ast_t;
    let mut a_id: ast_ptr_t = 0 as *mut ast_t;
    let mut a_typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut a_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut a_eph: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut a_pkg,
        &mut a_id,
        &mut a_typeargs,
        &mut a_cap,
        &mut a_eph,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        a,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children.as_mut_ptr(),
    );
    let mut b_pkg: ast_ptr_t = 0 as *mut ast_t;
    let mut b_id: ast_ptr_t = 0 as *mut ast_t;
    let mut b_typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut b_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut b_eph: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 6] = [
        &mut b_pkg,
        &mut b_id,
        &mut b_typeargs,
        &mut b_cap,
        &mut b_eph,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        b,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children_0.as_mut_ptr(),
    );
    let mut a_def: *mut ast_t = ast_data(a) as *mut ast_t;
    let mut b_def: *mut ast_t = ast_data(b) as *mut ast_t;
    return a_def == b_def && is_eq_typeargs(a, b, 0 as *mut errorframe_t, opt) as libc::c_int != 0;
}
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn check_assume(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !subtype_assume.is_null() {
        let mut assumption: *mut ast_t = ast_child(subtype_assume);
        while !assumption.is_null() {
            let mut assume_sub: ast_ptr_t = 0 as *mut ast_t;
            let mut assume_super: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] =
                [&mut assume_sub, &mut assume_super, 0 as *mut *mut ast_t];
            ast_get_children(
                assumption,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut *mut ast_t>()
                            .try_into()
                            .unwrap(),
                    )
                    .wrapping_sub(1)
                    .try_into()
                    .unwrap(),
                children.as_mut_ptr(),
            );
            if exact_nominal(sub, assume_sub, opt) as libc::c_int != 0
                && exact_nominal(super_0, assume_super, opt) as libc::c_int != 0
            {
                return 1 as libc::c_int != 0;
            }
            assumption = ast_sibling(assumption);
        }
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn push_assume(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_assume(sub, super_0, opt) {
        return 1 as libc::c_int != 0;
    }
    if subtype_assume.is_null() {
        subtype_assume = ast_from(sub, TK_NONE);
    }
    let mut assume: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = sub;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_NONE);
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
        parent_0 = ast_dup(sub);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_dup(sub));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_dup(sub));
    }
    if parent_0.is_null() {
        parent_0 = ast_dup(super_0);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_dup(super_0));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_dup(super_0));
    }
    ast_inheritflags(parent_0);
    assume = parent;
    ast_add(subtype_assume, assume);
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn pop_assume() {
    let mut assumption: *mut ast_t = ast_pop(subtype_assume);
    ast_free_unattached(assumption);
    if (ast_child(subtype_assume)).is_null() {
        ast_free_unattached(subtype_assume);
        subtype_assume = 0 as *mut ast_t;
    }
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn is_sub_cap_and_eph(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut _opt: *mut pass_opt_t,
) -> bool {
    let mut sub_cap: *mut ast_t = cap_fetch(sub);
    let mut sub_eph: *mut ast_t = ast_sibling(sub_cap);
    let mut super_cap: *mut ast_t = cap_fetch(super_0);
    let mut super_eph: *mut ast_t = ast_sibling(super_cap);
    match check_cap as libc::c_uint {
        3 => return 1 as libc::c_int != 0,
        0 => {
            if is_cap_sub_cap(
                ast_id(sub_cap),
                ast_id(sub_eph),
                ast_id(super_cap),
                ast_id(super_eph),
            ) {
                return 1 as libc::c_int != 0;
            }
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a subtype of %s: %s%s is not a subcap of %s%s\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                    ast_print_type(sub_cap),
                    ast_print_type(sub_eph),
                    ast_print_type(super_cap),
                    ast_print_type(super_eph),
                );
                if is_cap_sub_cap(
                    ast_id(sub_cap),
                    TK_EPHEMERAL,
                    ast_id(super_cap),
                    ast_id(super_eph),
                ) {
                    ast_error_frame(
                        errorf,
                        sub_cap,
                        b"this would be possible if the subcap were more ephemeral. Perhaps you meant to consume a variable here\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            return 0 as libc::c_int != 0;
        }
        1 => {
            if is_cap_sub_cap_constraint(
                ast_id(sub_cap),
                ast_id(sub_eph),
                ast_id(super_cap),
                ast_id(super_eph),
            ) {
                return 1 as libc::c_int != 0;
            }
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not in constraint %s: %s%s is not in constraint %s%s\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                    ast_print_type(sub_cap),
                    ast_print_type(sub_eph),
                    ast_print_type(super_cap),
                    ast_print_type(super_eph),
                );
            }
            return 0 as libc::c_int != 0;
        }
        2 => {
            if is_cap_sub_cap_bound(
                ast_id(sub_cap),
                ast_id(sub_eph),
                ast_id(super_cap),
                ast_id(super_eph),
            ) {
                return 1 as libc::c_int != 0;
            }
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a bound subtype of %s: %s%s is not a bound subcap of %s%s\0"
                        as *const u8 as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                    ast_print_type(sub_cap),
                    ast_print_type(sub_eph),
                    ast_print_type(super_cap),
                    ast_print_type(super_eph),
                );
            }
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"is_sub_cap_and_eph\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn is_eq_typeargs(
    mut a: *mut ast_t,
    mut b: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if ast_id(a) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(a) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            193 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"is_eq_typeargs\0"))
                .as_ptr(),
        );
    };
    if ast_id(b) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(b) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            194 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"is_eq_typeargs\0"))
                .as_ptr(),
        );
    };
    let mut a_arg: *mut ast_t = ast_child(ast_childidx(a, 2 as libc::c_int as usize));
    let mut b_arg: *mut ast_t = ast_child(ast_childidx(b, 2 as libc::c_int as usize));
    let mut ret: bool = 1 as libc::c_int != 0;
    while !a_arg.is_null() && !b_arg.is_null() {
        if !is_eqtype(a_arg, b_arg, errorf, opt) {
            ret = 0 as libc::c_int != 0;
        }
        a_arg = ast_sibling(a_arg);
        b_arg = ast_sibling(b_arg);
    }
    if !ret && !errorf.is_null() {
        ast_error_frame(
            errorf,
            a,
            b"%s has different type arguments than %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(a),
            ast_print_type(b),
        );
    }
    if !a_arg.is_null() || !b_arg.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                a,
                b"%s has a different number of type arguments than %s\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(a),
                ast_print_type(b),
            );
        }
        ret = 0 as libc::c_int != 0;
    }
    ret
}
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn is_reified_fun_sub_fun(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_id: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_params: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_result: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_throws: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 7] = [
        &mut sub_cap,
        &mut sub_id,
        &mut sub_typeparams,
        &mut sub_params,
        &mut sub_result,
        &mut sub_throws,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        sub,
        (::core::mem::size_of::<[*mut *mut ast_t; 7]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children.as_mut_ptr(),
    );
    let mut super_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut super_id: ast_ptr_t = 0 as *mut ast_t;
    let mut super_typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut super_params: ast_ptr_t = 0 as *mut ast_t;
    let mut super_result: ast_ptr_t = 0 as *mut ast_t;
    let mut super_throws: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 7] = [
        &mut super_cap,
        &mut super_id,
        &mut super_typeparams,
        &mut super_params,
        &mut super_result,
        &mut super_throws,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        super_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 7]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children_0.as_mut_ptr(),
    );
    match ast_id(sub) as libc::c_uint {
        88 => {
            if !is_cap_sub_cap(ast_id(sub_cap), TK_NONE, ast_id(super_cap), TK_NONE) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"%s constructor is not a subtype of %s constructor\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(sub_cap),
                        ast_print_type(super_cap),
                    );
                }
                return 0 as libc::c_int != 0;
            }
            if !is_subtype(sub_result, super_result, errorf, opt) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"constructor result %s is not a subtype of %s\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(sub_result),
                        ast_print_type(super_result),
                    );
                }
                return 0 as libc::c_int != 0;
            }
        }
        89 | 90 => {
            let mut sub_bare: bool =
                ast_id(sub_cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
            if sub_bare as libc::c_int
                == (ast_id(super_cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint)
                    as libc::c_int
            {
            } else {
                ponyint_assert_fail(
                    b"sub_bare == (ast_id(super_cap) == TK_AT)\0" as *const u8
                        as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    277 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"is_reified_fun_sub_fun\0",
                    ))
                    .as_ptr(),
                );
            };
            if !sub_bare && !is_cap_sub_cap(ast_id(super_cap), TK_NONE, ast_id(sub_cap), TK_NONE) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"%s method is not a subtype of %s method\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(sub_cap),
                        ast_print_type(super_cap),
                    );
                }
                return 0 as libc::c_int != 0;
            }
            if !is_subtype(sub_result, super_result, errorf, opt) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"method result %s is not a subtype of %s\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(sub_result),
                        ast_print_type(super_result),
                    );
                }
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    let mut sub_typeparam: *mut ast_t = ast_child(sub_typeparams);
    let mut super_typeparam: *mut ast_t = ast_child(super_typeparams);
    while !sub_typeparam.is_null() && !super_typeparam.is_null() {
        let mut sub_constraint: *mut ast_t = ast_childidx(sub_typeparam, 1 as libc::c_int as usize);
        let mut super_constraint: *mut ast_t =
            ast_childidx(super_typeparam, 1 as libc::c_int as usize);
        if !is_x_sub_x(super_constraint, sub_constraint, CHECK_CAP_EQ, errorf, opt) {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"type parameter constraint %s is not a supertype of %s\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub_constraint),
                    ast_print_type(super_constraint),
                );
            }
            return 0 as libc::c_int != 0;
        }
        sub_typeparam = ast_sibling(sub_typeparam);
        super_typeparam = ast_sibling(super_typeparam);
    }
    let mut sub_param: *mut ast_t = ast_child(sub_params);
    let mut super_param: *mut ast_t = ast_child(super_params);
    while !sub_param.is_null() && !super_param.is_null() {
        let mut sub_type: *mut ast_t = consume_type(
            ast_childidx(sub_param, 1 as libc::c_int as usize),
            TK_NONE,
            0 as libc::c_int != 0,
        );
        let mut super_type: *mut ast_t = consume_type(
            ast_childidx(super_param, 1 as libc::c_int as usize),
            TK_NONE,
            0 as libc::c_int != 0,
        );
        if sub_type.is_null() || super_type.is_null() {
            return 0 as libc::c_int != 0;
        }
        if !is_x_sub_x(super_type, sub_type, CHECK_CAP_SUB, errorf, opt) {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"parameter %s is not a supertype of %s\0" as *const u8 as *const libc::c_char,
                    ast_print_type(sub_type),
                    ast_print_type(super_type),
                );
            }
            ast_free_unattached(sub_type);
            ast_free_unattached(super_type);
            return 0 as libc::c_int != 0;
        }
        ast_free_unattached(sub_type);
        ast_free_unattached(super_type);
        sub_param = ast_sibling(sub_param);
        super_param = ast_sibling(super_param);
    }
    if ast_id(sub_throws) as libc::c_uint == TK_QUESTION as libc::c_int as libc::c_uint
        && ast_id(super_throws) as libc::c_uint != TK_QUESTION as libc::c_int as libc::c_uint
    {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"a partial function is not a subtype of a total function\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn is_fun_sub_fun(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut tsub: token_id = ast_id(sub);
    let mut tsuper: token_id = ast_id(super_0);
    match tsub as libc::c_uint {
        88 | 90 | 89 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    402 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"is_fun_sub_fun\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    match tsuper as libc::c_uint {
        88 | 90 | 89 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    414 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"is_fun_sub_fun\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    let mut sub_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_id: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut sub_params: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut sub_cap,
        &mut sub_id,
        &mut sub_typeparams,
        &mut sub_params,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        sub,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children.as_mut_ptr(),
    );
    let mut super_cap: ast_ptr_t = 0 as *mut ast_t;
    let mut super_id: ast_ptr_t = 0 as *mut ast_t;
    let mut super_typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut super_params: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 5] = [
        &mut super_cap,
        &mut super_id,
        &mut super_typeparams,
        &mut super_params,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        super_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children_0.as_mut_ptr(),
    );
    if ast_name(sub_id) != ast_name(super_id) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"method %s is not a subtype of method %s: they have different names\0" as *const u8
                    as *const libc::c_char,
                ast_name(sub_id),
                ast_name(super_id),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if (tsub as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
        || tsuper as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint)
        && tsub as libc::c_uint != tsuper as libc::c_uint
    {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"only a constructor can be a subtype of a constructor\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if ast_childcount(sub_typeparams) != ast_childcount(super_typeparams) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"methods have a different number of type parameters\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if ast_childcount(sub_params) != ast_childcount(super_params) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"methods have a different number of parameters\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut sub_bare: bool =
        ast_id(sub_cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
    let mut super_bare: bool =
        ast_id(super_cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
    if sub_bare as libc::c_int != super_bare as libc::c_int {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"method %s is not a subtype of method %s: their bareness differ\0" as *const u8
                    as *const libc::c_char,
                ast_name(sub_id),
                ast_name(super_id),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut r_sub: *mut ast_t = sub;
    if ast_id(super_typeparams) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        let mut typeargs: *mut ast_t = 0 as *mut ast_t;
        let mut basis_ast: *mut ast_t = super_typeparams;
        let mut parent: *mut ast_t = 0 as *mut ast_t;
        let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
        let mut node: *mut ast_t = 0 as *mut ast_t;
        node = ast_from(basis_ast, TK_TYPEARGS);
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
        typeargs = parent;
        let mut super_typeparam: *mut ast_t = ast_child(super_typeparams);
        while !super_typeparam.is_null() {
            let mut super_id_0: ast_ptr_t = 0 as *mut ast_t;
            let mut super_constraint: ast_ptr_t = 0 as *mut ast_t;
            let mut children_1: [*mut *mut ast_t; 3] =
                [&mut super_id_0, &mut super_constraint, 0 as *mut *mut ast_t];
            ast_get_children(
                super_typeparam,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut *mut ast_t>()
                            .try_into()
                            .unwrap(),
                    )
                    .wrapping_sub(1)
                    .try_into()
                    .unwrap(),
                children_1.as_mut_ptr(),
            );
            let mut typearg: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast_0: *mut ast_t = super_typeparam;
            let mut parent_1: *mut ast_t = 0 as *mut ast_t;
            let mut last_sibling_1: *mut ast_t = 0 as *mut ast_t;
            let mut node_1: *mut ast_t = 0 as *mut ast_t;
            node_1 = ast_from(basis_ast_0, TK_TYPEPARAMREF);
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
                parent_2 = super_id_0;
            } else if last_sibling_2.is_null() {
                last_sibling_2 = ast_add(parent_2, super_id_0);
            } else {
                last_sibling_2 = ast_add_sibling(last_sibling_2, super_id_0);
            }
            if parent_2.is_null() {
                parent_2 = ast_from(basis_ast_0, TK_NONE);
            } else if last_sibling_2.is_null() {
                last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
            } else {
                last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
            }
            if parent_2.is_null() {
                parent_2 = ast_from(basis_ast_0, TK_NONE);
            } else if last_sibling_2.is_null() {
                last_sibling_2 = ast_add(parent_2, ast_from(basis_ast_0, TK_NONE));
            } else {
                last_sibling_2 = ast_add_sibling(last_sibling_2, ast_from(basis_ast_0, TK_NONE));
            }
            ast_inheritflags(parent_2);
            typearg = parent_1;
            let mut def: *mut ast_t = ast_get(
                super_typeparam,
                ast_name(super_id_0),
                0 as *mut sym_status_t,
            );
            ast_setdata(typearg, def as *mut libc::c_void);
            typeparam_set_cap(typearg);
            ast_append(typeargs, typearg);
            super_typeparam = ast_sibling(super_typeparam);
        }
        r_sub = reify_method_def(sub, sub_typeparams, typeargs, opt);
        ast_free_unattached(typeargs);
    }
    let mut ok: bool = is_reified_fun_sub_fun(r_sub, super_0, errorf, opt);
    if r_sub != sub {
        ast_free_unattached(r_sub);
    }
    return ok;
}
#[c2rust::src_loc = "514:1"]
unsafe extern "C" fn is_x_sub_isect(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut child: *mut ast_t = ast_child(super_0);
    while !child.is_null() {
        if !is_x_sub_x(sub, child, check_cap, errorf, opt) {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a subtype of every element of %s\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                );
            }
            return 0 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "541:1"]
unsafe extern "C" fn is_x_sub_union(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut child: *mut ast_t = ast_child(super_0);
    while !child.is_null() {
        if is_x_sub_x(sub, child, check_cap, 0 as *mut errorframe_t, opt) {
            return 1 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    if !errorf.is_null() {
        let mut child_0: *mut ast_t = ast_child(super_0);
        while !child_0.is_null() {
            is_x_sub_x(sub, child_0, check_cap, errorf, opt);
            child_0 = ast_sibling(child_0);
        }
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a subtype of any element of %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "574:1"]
unsafe extern "C" fn is_union_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut child: *mut ast_t = ast_child(sub);
    while !child.is_null() {
        if !is_x_sub_x(child, super_0, check_cap, errorf, opt) {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    child,
                    b"not every element of %s is a subtype of %s\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                );
            }
            return 0 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn is_isect_sub_isect(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut super_child: *mut ast_t = ast_child(super_0);
    while !super_child.is_null() {
        if !is_isect_sub_x(sub, super_child, check_cap, errorf, opt) {
            return 0 as libc::c_int != 0;
        }
        super_child = ast_sibling(super_child);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "621:1"]
unsafe extern "C" fn todo() {}
#[c2rust::src_loc = "623:1"]
unsafe extern "C" fn is_isect_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    match ast_id(super_0) as libc::c_uint {
        56 => return is_isect_sub_isect(sub, super_0, check_cap, errorf, opt),
        151 => {
            let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
            if ast_id(super_def) as libc::c_uint == TK_INTERFACE as libc::c_int as libc::c_uint {
                todo();
            }
        }
        _ => {}
    }
    let mut child: *mut ast_t = ast_child(sub);
    while !child.is_null() {
        if is_x_sub_x(child, super_0, check_cap, 0 as *mut errorframe_t, opt) {
            return 1 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    if !errorf.is_null() {
        ast_error_frame(
            errorf,
            sub,
            b"no element of %s is a subtype of %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "669:1"]
unsafe extern "C" fn is_tuple_sub_tuple(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if ast_childcount(sub) != ast_childcount(super_0) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: they have a different number of elements\0"
                    as *const u8 as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut sub_child: *mut ast_t = ast_child(sub);
    let mut super_child: *mut ast_t = ast_child(super_0);
    let mut ret: bool = 1 as libc::c_int != 0;
    while !sub_child.is_null() {
        if !is_x_sub_x(sub_child, super_child, check_cap, errorf, opt) {
            ret = 0 as libc::c_int != 0;
        }
        sub_child = ast_sibling(sub_child);
        super_child = ast_sibling(super_child);
    }
    if !ret && !errorf.is_null() {
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a pairwise subtype of %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return ret;
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn is_single_sub_tuple(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut _check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut _opt: *mut pass_opt_t,
) -> bool {
    if !errorf.is_null() {
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a subtype of %s: the supertype is a tuple\0" as *const u8
                as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "726:1"]
unsafe extern "C" fn is_tuple_sub_nominal(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if is_top_type(super_0, 1 as libc::c_int != 0) {
        let mut child: *mut ast_t = ast_child(sub);
        while !child.is_null() {
            if !is_x_sub_x(child, super_0, check_cap, errorf, opt) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        child,
                        b"%s is not a subtype of %s: %s is not a subtype of %s\0" as *const u8
                            as *const libc::c_char,
                        ast_print_type(sub),
                        ast_print_type(super_0),
                        ast_print_type(child),
                        ast_print_type(super_0),
                    );
                }
                return 0 as libc::c_int != 0;
            }
            child = ast_sibling(child);
        }
        return 1 as libc::c_int != 0;
    }
    if !errorf.is_null() {
        let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a subtype of %s: the subtype is a tuple\0" as *const u8
                as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
        ast_error_frame(
            errorf,
            super_def,
            b"this might be possible if the supertype were an empty interface, such as the Any type.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "765:1"]
unsafe extern "C" fn is_tuple_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    match ast_id(super_0) as libc::c_uint {
        149 => return is_x_sub_union(sub, super_0, check_cap, errorf, opt),
        56 => return is_x_sub_isect(sub, super_0, check_cap, errorf, opt),
        150 => return is_tuple_sub_tuple(sub, super_0, check_cap, errorf, opt),
        151 => return is_tuple_sub_nominal(sub, super_0, check_cap, errorf, opt),
        187 => return is_x_sub_typeparam(sub, super_0, check_cap, errorf, opt),
        17 => return is_x_sub_arrow(sub, super_0, check_cap, errorf, opt),
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            796 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"is_tuple_sub_x\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "800:1"]
unsafe extern "C" fn is_nominal_sub_entity(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
    let mut ret: bool = 1 as libc::c_int != 0;
    if is_bare(sub) as libc::c_int != 0 && is_pointer(super_0) as libc::c_int != 0 {
        let mut super_typeargs: *mut ast_t = ast_childidx(super_0, 2 as libc::c_int as usize);
        let mut super_typearg: *mut ast_t = ast_child(super_typeargs);
        if is_none(super_typearg) {
            return 1 as libc::c_int != 0;
        }
    }
    if sub_def != super_def {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s\0" as *const u8 as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if !is_eq_typeargs(sub, super_0, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    if !is_sub_cap_and_eph(sub, super_0, check_cap, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    return ret;
}
#[c2rust::src_loc = "841:1"]
unsafe extern "C" fn is_nominal_sub_structural(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
    let mut sub_pass: pass_id =
        ast_checkflag(sub_def, AST_FLAG_PASS_MASK as libc::c_int as u32) as pass_id;
    let mut super_pass: pass_id =
        ast_checkflag(super_def, AST_FLAG_PASS_MASK as libc::c_int as u32) as pass_id;
    if sub_pass as libc::c_uint >= PASS_TRAITS as libc::c_int as libc::c_uint
        && super_pass as libc::c_uint >= PASS_TRAITS as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(sub_pass >= PASS_TRAITS) && (super_pass >= PASS_TRAITS)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            855 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"is_nominal_sub_structural\0",
            ))
            .as_ptr(),
        );
    };
    if ast_has_annotation(
        sub_def,
        b"nosupertype\0" as *const u8 as *const libc::c_char,
    ) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: it is marked 'nosupertype'\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if is_bare(sub) as libc::c_int != is_bare(super_0) as libc::c_int {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: their bareness differ\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut sub_typeargs: *mut ast_t = ast_childidx(sub, 2 as libc::c_int as usize);
    let mut sub_typeparams: *mut ast_t = ast_childidx(sub_def, 1 as libc::c_int as usize);
    let mut super_typeargs: *mut ast_t = ast_childidx(super_0, 2 as libc::c_int as usize);
    let mut super_typeparams: *mut ast_t = ast_childidx(super_def, 1 as libc::c_int as usize);
    let mut super_members: *mut ast_t = ast_childidx(super_def, 4 as libc::c_int as usize);
    let mut super_member: *mut ast_t = ast_child(super_members);
    while !super_member.is_null() {
        let mut super_member_id: *mut ast_t = ast_childidx(super_member, 1 as libc::c_int as usize);
        let mut sub_member: *mut ast_t =
            ast_get(sub_def, ast_name(super_member_id), 0 as *mut sym_status_t);
        if sub_member.is_null()
            || ast_id(sub_member) as libc::c_uint != TK_FUN as libc::c_int as libc::c_uint
                && ast_id(sub_member) as libc::c_uint != TK_BE as libc::c_int as libc::c_uint
                && ast_id(sub_member) as libc::c_uint != TK_NEW as libc::c_int as libc::c_uint
        {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a subtype of %s: it has no method '%s'\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                    ast_name(super_member_id),
                );
            }
            ret = 0 as libc::c_int != 0;
            super_member = ast_sibling(super_member);
        } else {
            let mut r_sub_member: *mut ast_t =
                reify_method_def(sub_member, sub_typeparams, sub_typeargs, opt);
            if !r_sub_member.is_null() {
            } else {
                ponyint_assert_fail(
                    b"r_sub_member != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    918 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"is_nominal_sub_structural\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut r_super_member: *mut ast_t =
                reify_method_def(super_member, super_typeparams, super_typeargs, opt);
            if !r_super_member.is_null() {
            } else {
                ponyint_assert_fail(
                    b"r_super_member != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    923 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"is_nominal_sub_structural\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut ok: bool = is_fun_sub_fun(r_sub_member, r_super_member, errorf, opt);
            ast_free_unattached(r_sub_member);
            ast_free_unattached(r_super_member);
            if !ok {
                ret = 0 as libc::c_int != 0;
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub_member,
                        b"%s is not a subtype of %s: method '%s' has an incompatible signature\0"
                            as *const u8 as *const libc::c_char,
                        ast_print_type(sub),
                        ast_print_type(super_0),
                        ast_name(super_member_id),
                    );
                }
            }
            super_member = ast_sibling(super_member);
        }
    }
    return ret;
}
#[c2rust::src_loc = "950:1"]
unsafe extern "C" fn is_nominal_sub_interface(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    if ast_id(sub_def) as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint {
        struct_cant_be_x(
            sub,
            super_0,
            errorf,
            b"an interface\0" as *const u8 as *const libc::c_char,
        );
        ret = 0 as libc::c_int != 0;
    }
    if !is_sub_cap_and_eph(sub, super_0, check_cap, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    if !is_nominal_sub_structural(sub, super_0, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    ret
}
#[c2rust::src_loc = "973:1"]
unsafe extern "C" fn nominal_provides_trait(
    mut type_0: *mut ast_t,
    mut trait_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !is_bare(trait_0) {
    } else {
        ponyint_assert_fail(
            b"!is_bare(trait)\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            976 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"nominal_provides_trait\0",
            ))
            .as_ptr(),
        );
    };
    if is_bare(type_0) {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                type_0,
                b"%s is not a subtype of %s: their bareness differ\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(type_0),
                ast_print_type(trait_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut defcap: ast_ptr_t = 0 as *mut ast_t;
    let mut traits: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut id,
        &mut typeparams,
        &mut defcap,
        &mut traits,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        def,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children.as_mut_ptr(),
    );
    let mut typeargs: *mut ast_t = ast_childidx(type_0, 2 as libc::c_int as usize);
    let mut t_pkg: ast_ptr_t = 0 as *mut ast_t;
    let mut t_name: ast_ptr_t = 0 as *mut ast_t;
    let mut t_typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut eph: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 6] = [
        &mut t_pkg,
        &mut t_name,
        &mut t_typeparams,
        &mut cap,
        &mut eph,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        trait_0,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<*mut *mut ast_t>()
                    .try_into()
                    .unwrap(),
            )
            .wrapping_sub(1)
            .try_into()
            .unwrap(),
        children_0.as_mut_ptr(),
    );
    let mut tcap: token_id = ast_id(cap);
    let mut teph: token_id = ast_id(eph);
    let mut child: *mut ast_t = ast_child(traits);
    while !child.is_null() {
        let mut r_child: *mut ast_t =
            reify(child, typeparams, typeargs, opt, 1 as libc::c_int != 0);
        if !r_child.is_null() {
        } else {
            ponyint_assert_fail(
                b"r_child != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                    as *const u8 as *const libc::c_char,
                1006 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"nominal_provides_trait\0",
                ))
                .as_ptr(),
            );
        };
        let mut rr_child: *mut ast_t = set_cap_and_ephemeral(r_child, tcap, teph);
        let mut is_sub: bool =
            is_x_sub_x(rr_child, trait_0, check_cap, 0 as *mut errorframe_t, opt);
        ast_free_unattached(rr_child);
        if r_child != child {
            ast_free_unattached(r_child);
        }
        if is_sub {
            return 1 as libc::c_int != 0;
        }
        child = ast_sibling(child);
    }
    if !errorf.is_null() {
        ast_error_frame(
            errorf,
            type_0,
            b"%s does not implement trait %s\0" as *const u8 as *const libc::c_char,
            ast_print_type(type_0),
            ast_print_type(trait_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1031:1"]
unsafe extern "C" fn is_entity_sub_trait(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !nominal_provides_trait(sub, super_0, check_cap, errorf, opt) {
        return 0 as libc::c_int != 0;
    }
    if !is_sub_cap_and_eph(sub, super_0, check_cap, errorf, opt) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "1047:1"]
unsafe extern "C" fn is_struct_sub_trait(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
) -> bool {
    struct_cant_be_x(
        sub,
        super_0,
        errorf,
        b"a trait\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1053:1"]
unsafe extern "C" fn is_trait_sub_trait(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
    let mut ret: bool = 1 as libc::c_int != 0;
    if sub_def == super_def {
        if !is_eq_typeargs(sub, super_0, errorf, opt) {
            ret = 0 as libc::c_int != 0;
        }
    } else if !nominal_provides_trait(sub, super_0, check_cap, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    if !is_sub_cap_and_eph(sub, super_0, check_cap, errorf, opt) {
        ret = 0 as libc::c_int != 0;
    }
    return ret;
}
#[c2rust::src_loc = "1080:1"]
unsafe extern "C" fn is_interface_sub_trait(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut _check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
) -> bool {
    if !errorf.is_null() {
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a subtype of %s: an interface can't be a subtype of a trait\0" as *const u8
                as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1095:1"]
unsafe extern "C" fn is_nominal_sub_trait(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    match ast_id(sub_def) as libc::c_uint {
        74 | 76 | 77 => return is_entity_sub_trait(sub, super_0, check_cap, errorf, opt),
        75 => return is_struct_sub_trait(sub, super_0, errorf),
        73 => return is_trait_sub_trait(sub, super_0, check_cap, errorf, opt),
        72 => return is_interface_sub_trait(sub, super_0, check_cap, errorf),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1121 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"is_nominal_sub_trait\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1125:1"]
unsafe extern "C" fn is_nominal_sub_nominal(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if push_assume(sub, super_0, opt) {
        return 1 as libc::c_int != 0;
    }
    let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
    let mut ret: bool = 0 as libc::c_int != 0;
    match ast_id(super_def) as libc::c_uint {
        74 | 75 | 76 | 77 => {
            ret = is_nominal_sub_entity(sub, super_0, check_cap, errorf, opt);
        }
        72 => {
            ret = is_nominal_sub_interface(sub, super_0, check_cap, errorf, opt);
        }
        73 => {
            ret = is_nominal_sub_trait(sub, super_0, check_cap, errorf, opt);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0"
                        as *const u8 as *const libc::c_char,
                    1154 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"is_nominal_sub_nominal\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    }
    pop_assume();
    return ret;
}
#[c2rust::src_loc = "1161:1"]
unsafe extern "C" fn is_x_sub_typeparam(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        let mut constraint: *mut ast_t = typeparam_constraint(super_0);
        if constraint.is_null() {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a subtype of %s: the type parameter has no constraint\0"
                        as *const u8 as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                );
            }
            return 0 as libc::c_int != 0;
        }
        return is_x_sub_x(sub, constraint, CHECK_CAP_IGNORE, errorf, opt);
    }
    let mut super_lower: *mut ast_t = typeparam_lower(super_0);
    if super_lower.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the type parameter has no lower bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = is_x_sub_x(sub, super_lower, check_cap, errorf, opt);
    ast_free_unattached(super_lower);
    return ok;
}
#[c2rust::src_loc = "1206:1"]
unsafe extern "C" fn is_x_sub_arrow(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        return is_x_sub_x(
            sub,
            ast_childidx(super_0, 1 as libc::c_int as usize),
            CHECK_CAP_IGNORE,
            errorf,
            opt,
        );
    }
    if !is_x_sub_x(
        sub,
        ast_childidx(super_0, 1 as libc::c_int as usize),
        CHECK_CAP_IGNORE,
        errorf,
        opt,
    ) {
        return 0 as libc::c_int != 0;
    }
    let mut super_lower: *mut ast_t = viewpoint_lower(super_0);
    if super_lower.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the supertype has no lower bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = is_x_sub_x(sub, super_lower, check_cap, errorf, opt);
    ast_free_unattached(super_lower);
    return ok;
}
#[c2rust::src_loc = "1243:1"]
unsafe extern "C" fn is_nominal_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    match ast_id(super_0) as libc::c_uint {
        149 => {
            let mut def: *mut ast_t = ast_data(sub) as *mut ast_t;
            if ast_id(def) as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint {
                struct_cant_be_x(
                    sub,
                    super_0,
                    errorf,
                    b"a union type\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            if is_bare(sub) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"a bare type cannot be in a union type\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            return is_x_sub_union(sub, super_0, check_cap, errorf, opt);
        }
        56 => {
            let mut def_0: *mut ast_t = ast_data(sub) as *mut ast_t;
            if ast_id(def_0) as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint {
                struct_cant_be_x(
                    sub,
                    super_0,
                    errorf,
                    b"an intersection type\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            if is_bare(sub) {
                if !errorf.is_null() {
                    ast_error_frame(
                        errorf,
                        sub,
                        b"a bare type cannot be in an intersection type\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            return is_x_sub_isect(sub, super_0, check_cap, errorf, opt);
        }
        150 => return is_single_sub_tuple(sub, super_0, check_cap, errorf, opt),
        151 => return is_nominal_sub_nominal(sub, super_0, check_cap, errorf, opt),
        187 => return is_x_sub_typeparam(sub, super_0, check_cap, errorf, opt),
        17 => return is_x_sub_arrow(sub, super_0, check_cap, errorf, opt),
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1314 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"is_nominal_sub_x\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1318:1"]
unsafe extern "C" fn is_typeparam_sub_typeparam(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let mut sub_def: *mut ast_t = ast_data(sub) as *mut ast_t;
    let mut super_def: *mut ast_t = ast_data(super_0) as *mut ast_t;
    if check_cap as libc::c_uint == CHECK_CAP_SUB as libc::c_int as libc::c_uint {
        check_cap = CHECK_CAP_BOUND;
    }
    while !(ast_data(sub_def)).is_null() && sub_def != ast_data(sub_def) as *mut ast_t {
        sub_def = ast_data(sub_def) as *mut ast_t;
    }
    while !(ast_data(super_def)).is_null() && super_def != ast_data(super_def) as *mut ast_t {
        super_def = ast_data(super_def) as *mut ast_t;
    }
    if sub_def == super_def {
        return is_sub_cap_and_eph(sub, super_0, check_cap, errorf, opt);
    }
    if !errorf.is_null() {
        ast_error_frame(
            errorf,
            sub,
            b"%s is not a subtype of %s: they are different type parameters\0" as *const u8
                as *const libc::c_char,
            ast_print_type(sub),
            ast_print_type(super_0),
        );
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1353:1"]
unsafe extern "C" fn is_typeparam_sub_arrow(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        return is_typeparam_sub_x(
            sub,
            ast_childidx(super_0, 1 as libc::c_int as usize),
            CHECK_CAP_IGNORE,
            errorf,
            opt,
        );
    }
    if !is_typeparam_sub_x(
        sub,
        ast_childidx(super_0, 1 as libc::c_int as usize),
        CHECK_CAP_IGNORE,
        errorf,
        opt,
    ) {
        return 0 as libc::c_int != 0;
    }
    let mut r_sub: *mut ast_t = viewpoint_reifytypeparam(sub, sub);
    let mut r_super: *mut ast_t = viewpoint_reifytypeparam(super_0, sub);
    if !r_sub.is_null() {
        let mut ok: bool = is_x_sub_x(r_sub, r_super, check_cap, errorf, opt);
        ast_free_unattached(r_sub);
        ast_free_unattached(r_super);
        return ok;
    }
    let mut super_lower: *mut ast_t = viewpoint_lower(super_0);
    if super_lower.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the supertype has no lower bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ok_0: bool = is_x_sub_x(sub, super_lower, check_cap, errorf, opt);
    ast_free_unattached(super_lower);
    return ok_0;
}
#[c2rust::src_loc = "1406:1"]
unsafe extern "C" fn is_typeparam_base_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    match ast_id(super_0) as libc::c_uint {
        149 => return is_x_sub_union(sub, super_0, check_cap, errorf, opt),
        56 => return is_x_sub_isect(sub, super_0, check_cap, errorf, opt),
        150 | 151 => return 0 as libc::c_int != 0,
        187 => return is_typeparam_sub_typeparam(sub, super_0, check_cap, errorf, opt),
        17 => return is_typeparam_sub_arrow(sub, super_0, check_cap, errorf, opt),
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1435 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"is_typeparam_base_sub_x\0",
            ))
            .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1439:1"]
unsafe extern "C" fn is_typeparam_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if is_typeparam_base_sub_x(sub, super_0, check_cap, 0 as *mut errorframe_t, opt) {
        return 1 as libc::c_int != 0;
    }
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        let mut constraint: *mut ast_t = typeparam_constraint(sub);
        if constraint.is_null() {
            if !errorf.is_null() {
                ast_error_frame(
                    errorf,
                    sub,
                    b"%s is not a subtype of %s: the subtype has no constraint\0" as *const u8
                        as *const libc::c_char,
                    ast_print_type(sub),
                    ast_print_type(super_0),
                );
            }
            return 0 as libc::c_int != 0;
        }
        return is_x_sub_x(constraint, super_0, CHECK_CAP_IGNORE, errorf, opt);
    }
    let mut sub_upper: *mut ast_t = typeparam_upper(sub);
    if sub_upper.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the subtype has no constraint\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = is_x_sub_x(sub_upper, super_0, check_cap, errorf, opt);
    ast_free_unattached(sub_upper);
    return ok;
}
#[c2rust::src_loc = "1487:1"]
unsafe extern "C" fn is_arrow_sub_nominal(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        return is_x_sub_x(
            ast_childidx(sub, 1 as libc::c_int as usize),
            super_0,
            CHECK_CAP_IGNORE,
            errorf,
            opt,
        );
    }
    if !is_x_sub_x(
        ast_childidx(sub, 1 as libc::c_int as usize),
        super_0,
        CHECK_CAP_IGNORE,
        errorf,
        opt,
    ) {
        return 0 as libc::c_int != 0;
    }
    let mut sub_upper: *mut ast_t = viewpoint_upper(sub);
    if sub_upper.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the subtype has no upper bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = is_x_sub_x(sub_upper, super_0, check_cap, errorf, opt);
    ast_free_unattached(sub_upper);
    return ok;
}
#[c2rust::src_loc = "1523:1"]
unsafe extern "C" fn is_arrow_sub_typeparam(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        return is_x_sub_x(
            ast_childidx(sub, 1 as libc::c_int as usize),
            super_0,
            CHECK_CAP_IGNORE,
            errorf,
            opt,
        );
    }
    if !is_x_sub_x(
        ast_childidx(sub, 1 as libc::c_int as usize),
        super_0,
        CHECK_CAP_IGNORE,
        errorf,
        opt,
    ) {
        return 0 as libc::c_int != 0;
    }
    let mut r_sub: *mut ast_t = viewpoint_reifytypeparam(sub, super_0);
    let mut r_super: *mut ast_t = viewpoint_reifytypeparam(super_0, super_0);
    if !r_sub.is_null() {
        let mut ok: bool = is_x_sub_x(r_sub, r_super, check_cap, errorf, opt);
        ast_free_unattached(r_sub);
        ast_free_unattached(r_super);
        return ok;
    }
    return is_arrow_sub_nominal(sub, super_0, check_cap, errorf, opt);
}
#[c2rust::src_loc = "1555:1"]
unsafe extern "C" fn is_arrow_sub_arrow(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if check_cap as libc::c_uint == CHECK_CAP_IGNORE as libc::c_int as libc::c_uint {
        return is_x_sub_x(
            ast_childidx(sub, 1 as libc::c_int as usize),
            ast_childidx(super_0, 1 as libc::c_int as usize),
            CHECK_CAP_IGNORE,
            errorf,
            opt,
        );
    }
    if !is_x_sub_x(
        ast_childidx(sub, 1 as libc::c_int as usize),
        ast_childidx(super_0, 1 as libc::c_int as usize),
        CHECK_CAP_IGNORE,
        errorf,
        opt,
    ) {
        return 0 as libc::c_int != 0;
    }
    let mut r_sub: *mut ast_t = 0 as *mut ast_t;
    let mut r_super: *mut ast_t = 0 as *mut ast_t;
    if viewpoint_reifypair(sub, super_0, &mut r_sub, &mut r_super) {
        let mut ok: bool = is_x_sub_x(r_sub, r_super, check_cap, errorf, opt);
        ast_free_unattached(r_sub);
        ast_free_unattached(r_super);
        return ok;
    }
    let mut sub_upper: *mut ast_t = viewpoint_upper(sub);
    let mut super_lower: *mut ast_t = viewpoint_lower(super_0);
    let mut ok_0: bool = 1 as libc::c_int != 0;
    if sub_upper.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the subtype has no upper bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        ok_0 = 0 as libc::c_int != 0;
    }
    if super_lower.is_null() {
        if !errorf.is_null() {
            ast_error_frame(
                errorf,
                sub,
                b"%s is not a subtype of %s: the supertype has no lower bounds\0" as *const u8
                    as *const libc::c_char,
                ast_print_type(sub),
                ast_print_type(super_0),
            );
        }
        ok_0 = 0 as libc::c_int != 0;
    }
    if ok_0 {
        ok_0 = is_x_sub_x(sub_upper, super_lower, check_cap, errorf, opt);
    }
    ast_free_unattached(sub_upper);
    ast_free_unattached(super_lower);
    return ok_0;
}
#[c2rust::src_loc = "1630:1"]
unsafe extern "C" fn is_arrow_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    match ast_id(super_0) as libc::c_uint {
        149 => return is_x_sub_union(sub, super_0, check_cap, errorf, opt),
        56 => return is_x_sub_isect(sub, super_0, check_cap, errorf, opt),
        150 => return is_single_sub_tuple(sub, super_0, check_cap, errorf, opt),
        151 => return is_arrow_sub_nominal(sub, super_0, check_cap, errorf, opt),
        187 => return is_arrow_sub_typeparam(sub, super_0, check_cap, errorf, opt),
        17 => return is_arrow_sub_arrow(sub, super_0, check_cap, errorf, opt),
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1661 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"is_arrow_sub_x\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "1665:1"]
unsafe extern "C" fn is_x_sub_x(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut check_cap: check_cap_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if !sub.is_null() {
    } else {
        ponyint_assert_fail(
            b"sub != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1668 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"is_x_sub_x\0")).as_ptr(),
        );
    };
    if !super_0.is_null() {
    } else {
        ponyint_assert_fail(
            b"super != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1669 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"is_x_sub_x\0")).as_ptr(),
        );
    };
    if ast_id(super_0) as libc::c_uint == TK_DONTCARETYPE as libc::c_int as libc::c_uint
        || ast_id(sub) as libc::c_uint == TK_DONTCARETYPE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    match ast_id(sub) as libc::c_uint {
        149 => return is_union_sub_x(sub, super_0, check_cap, errorf, opt),
        56 => return is_isect_sub_x(sub, super_0, check_cap, errorf, opt),
        150 => return is_tuple_sub_x(sub, super_0, check_cap, errorf, opt),
        151 => return is_nominal_sub_x(sub, super_0, check_cap, errorf, opt),
        187 => return is_typeparam_sub_x(sub, super_0, check_cap, errorf, opt),
        17 => return is_arrow_sub_x(sub, super_0, check_cap, errorf, opt),
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1702 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"is_x_sub_x\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1706:1"]
pub unsafe extern "C" fn is_subtype(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    return is_x_sub_x(sub, super_0, CHECK_CAP_SUB, errorf, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1712:1"]
pub unsafe extern "C" fn is_subtype_constraint(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    return is_x_sub_x(sub, super_0, CHECK_CAP_EQ, errorf, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1718:1"]
pub unsafe extern "C" fn is_subtype_ignore_cap(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    return is_x_sub_x(sub, super_0, CHECK_CAP_IGNORE, errorf, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1724:1"]
pub unsafe extern "C" fn is_subtype_fun(
    mut sub: *mut ast_t,
    mut super_0: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    return is_fun_sub_fun(sub, super_0, errorf, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1730:1"]
pub unsafe extern "C" fn is_eqtype(
    mut a: *mut ast_t,
    mut b: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    return is_subtype(a, b, errorf, opt) as libc::c_int != 0
        && is_subtype(b, a, errorf, opt) as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1735:1"]
pub unsafe extern "C" fn is_sub_provides(
    mut type_0: *mut ast_t,
    mut provides: *mut ast_t,
    mut errorf: *mut errorframe_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    if ast_id(type_0) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1738 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"is_sub_provides\0"))
                .as_ptr(),
        );
    };
    return is_nominal_sub_structural(type_0, provides, errorf, opt);
}
#[no_mangle]
#[c2rust::src_loc = "1742:1"]
pub unsafe extern "C" fn is_literal(mut type_0: *mut ast_t, mut name: *const libc::c_char) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ast_id(type_0) as libc::c_uint != TK_NOMINAL as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    return strcmp(
        ast_name(ast_childidx(type_0, 1 as libc::c_int as usize)),
        name,
    ) == 0;
}
#[no_mangle]
#[c2rust::src_loc = "1754:1"]
pub unsafe extern "C" fn is_pointer(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"Pointer\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1759:1"]
pub unsafe extern "C" fn is_nullable_pointer(mut type_0: *mut ast_t) -> bool {
    return is_literal(
        type_0,
        b"NullablePointer\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1764:1"]
pub unsafe extern "C" fn is_none(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"None\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1769:1"]
pub unsafe extern "C" fn is_env(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"Env\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1774:1"]
pub unsafe extern "C" fn is_runtime_options(mut type_0: *mut ast_t) -> bool {
    return is_literal(
        type_0,
        b"RuntimeOptions\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1779:1"]
pub unsafe extern "C" fn is_bool(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"Bool\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1784:1"]
pub unsafe extern "C" fn is_float(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"F32\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"F64\0" as *const u8 as *const libc::c_char) as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1789:1"]
pub unsafe extern "C" fn is_integer(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"I8\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I16\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I32\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I64\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I128\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"ILong\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"ISize\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"U8\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"U16\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"U32\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"U64\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"U128\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"ULong\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"USize\0" as *const u8 as *const libc::c_char) as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1808:1"]
pub unsafe extern "C" fn is_machine_word(mut type_0: *mut ast_t) -> bool {
    return is_bool(type_0) as libc::c_int != 0
        || is_integer(type_0) as libc::c_int != 0
        || is_float(type_0) as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1813:1"]
pub unsafe extern "C" fn is_signed(mut type_0: *mut ast_t) -> bool {
    return is_literal(type_0, b"I8\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I16\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I32\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I64\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"I128\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"ILong\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        || is_literal(type_0, b"ISize\0" as *const u8 as *const libc::c_char) as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1825:1"]
pub unsafe extern "C" fn is_constructable(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        149 | 150 => return 0 as libc::c_int != 0,
        56 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if is_constructable(child) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        151 => {
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            match ast_id(def) as libc::c_uint {
                72 | 73 => {
                    let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as usize);
                    let mut member: *mut ast_t = ast_child(members);
                    while !member.is_null() {
                        if ast_id(member) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint {
                            return 1 as libc::c_int != 0;
                        }
                        member = ast_sibling(member);
                    }
                    return 0 as libc::c_int != 0;
                }
                74 | 75 | 76 | 77 => return 1 as libc::c_int != 0,
                _ => {}
            }
        }
        187 => return is_constructable(typeparam_constraint(type_0)),
        17 => return is_constructable(ast_childidx(type_0, 1 as libc::c_int as usize)),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1894 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"is_constructable\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1898:1"]
pub unsafe extern "C" fn is_concrete(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        149 | 150 => return 0 as libc::c_int != 0,
        56 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if is_concrete(child) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        151 => {
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            match ast_id(def) as libc::c_uint {
                72 | 73 => return 0 as libc::c_int != 0,
                74 | 75 | 76 | 77 => return 1 as libc::c_int != 0,
                _ => {}
            }
        }
        187 => return is_constructable(typeparam_constraint(type_0)),
        17 => return is_concrete(ast_childidx(type_0, 1 as libc::c_int as usize)),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            1954 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"is_concrete\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "1958:1"]
pub unsafe extern "C" fn is_known(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        149 | 150 => return 0 as libc::c_int != 0,
        56 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if is_known(child) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        151 => {
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            match ast_id(def) as libc::c_uint {
                72 | 73 => return 0 as libc::c_int != 0,
                74 | 75 | 76 | 77 => return 1 as libc::c_int != 0,
                _ => {}
            }
        }
        17 => return is_known(ast_childidx(type_0, 1 as libc::c_int as usize)),
        187 => return is_known(typeparam_constraint(type_0)),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            2014 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"is_known\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "2018:1"]
pub unsafe extern "C" fn is_bare(mut type_0: *mut ast_t) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if is_bare(child) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        151 => {
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            return ast_has_annotation(def, b"ponyint_bare\0" as *const u8 as *const libc::c_char);
        }
        17 => return is_bare(ast_childidx(type_0, 1 as libc::c_int as usize)),
        187 | 153 | 157 | 158 | 156 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            2060 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"is_bare\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "2064:1"]
pub unsafe extern "C" fn is_top_type(mut type_0: *mut ast_t, mut ignore_cap: bool) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        149 | 56 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if !is_top_type(child, ignore_cap) {
                    return 0 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 1 as libc::c_int != 0;
        }
        151 => {
            if !ignore_cap
                && ast_id(cap_fetch(type_0)) as libc::c_uint
                    != TK_TAG as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int != 0;
            }
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            if ast_id(def) as libc::c_uint != TK_INTERFACE as libc::c_int as libc::c_uint {
                return 0 as libc::c_int != 0;
            }
            let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as usize);
            if ast_childcount(members) != 0 {
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        17 => {
            if ignore_cap {
                return is_top_type(
                    ast_childidx(type_0, 1 as libc::c_int as usize),
                    1 as libc::c_int != 0,
                );
            }
            let mut type_lower: *mut ast_t = viewpoint_lower(type_0);
            if type_lower.is_null() {
                return 0 as libc::c_int != 0;
            }
            let mut r: bool = is_top_type(type_lower, 0 as libc::c_int != 0);
            ast_free_unattached(type_lower);
            return r;
        }
        150 | 187 | 153 | 157 | 158 | 156 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            2133 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"is_top_type\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "2137:1"]
pub unsafe extern "C" fn is_entity(mut type_0: *mut ast_t, mut entity: token_id) -> bool {
    if type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    match ast_id(type_0) as libc::c_uint {
        150 => return 0 as libc::c_int != 0,
        149 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if !is_entity(child, entity) {
                    return 0 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 1 as libc::c_int != 0;
        }
        56 => {
            let mut child_0: *mut ast_t = ast_child(type_0);
            while !child_0.is_null() {
                if is_entity(child_0, entity) {
                    return 1 as libc::c_int != 0;
                }
                child_0 = ast_sibling(child_0);
            }
            return 0 as libc::c_int != 0;
        }
        151 => {
            let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
            return ast_id(def) as libc::c_uint == entity as libc::c_uint;
        }
        17 => return is_entity(ast_childidx(type_0, 1 as libc::c_int as usize), entity),
        187 => return is_entity(typeparam_constraint(type_0), entity),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.c\0" as *const u8
                as *const libc::c_char,
            2192 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"is_entity\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "2196:1"]
pub unsafe extern "C" fn contains_dontcare(mut ast: *mut ast_t) -> bool {
    match ast_id(ast) as libc::c_uint {
        156 => return 1 as libc::c_int != 0,
        150 => {
            let mut child: *mut ast_t = ast_child(ast);
            while !child.is_null() {
                if contains_dontcare(child) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
