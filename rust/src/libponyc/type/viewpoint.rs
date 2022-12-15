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
    #[c2rust::src_loc = "9:16"]
    pub use crate::libponyc::ast::ast::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;

    use super::symtab_h::ast_t;
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
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
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
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: usize,
            out_children: *mut *mut *mut ast_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.h:2"]
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
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "86:1"]
        pub fn cap_view_upper(
            left_cap: token_id,
            left_eph: token_id,
            right_cap: *mut token_id,
            right_eph: *mut token_id,
        ) -> bool;
        #[c2rust::src_loc = "92:1"]
        pub fn cap_view_lower(
            left_cap: token_id,
            left_eph: token_id,
            right_cap: *mut token_id,
            right_eph: *mut token_id,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:6"]
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
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::alias_h::{alias, consume_type};
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_child, ast_data, ast_dup, ast_free_unattached,
    ast_from, ast_get_children, ast_id, ast_inheritflags, ast_ptr_t, ast_replace, ast_sibling,
    ast_type,
};
use self::cap_h::{cap_fetch, cap_view_lower, cap_view_upper};
use self::ponyassert_h::ponyint_assert_fail;
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
#[c2rust::src_loc = "10:3"]
pub const VIEW_UPPER_NO: C2RustUnnamed = 0;
#[c2rust::src_loc = "11:3"]
pub const VIEW_UPPER_YES: C2RustUnnamed = 1;
#[c2rust::src_loc = "12:3"]
pub const VIEW_UPPER_FORCE: C2RustUnnamed = 2;
#[c2rust::src_loc = "8:1"]
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
#[c2rust::src_loc = "15:1"]
pub unsafe extern "C" fn viewpoint_type(
    mut l_type: *mut ast_t,
    mut r_type: *mut ast_t,
) -> *mut ast_t {
    let mut upper: libc::c_int = VIEW_UPPER_NO as libc::c_int;
    match ast_id(r_type) as libc::c_uint {
        149 | 56 | 150 => {
            let mut type_0: *mut ast_t = ast_from(r_type, ast_id(r_type));
            let mut child: *mut ast_t = ast_child(r_type);
            while !child.is_null() {
                ast_append(type_0, viewpoint_type(l_type, child));
                child = ast_sibling(child);
            }
            return type_0;
        }
        187 => {
            upper = VIEW_UPPER_NO as libc::c_int;
        }
        151 => {
            let mut cap: *mut ast_t = cap_fetch(r_type);
            match ast_id(cap) as libc::c_uint {
                91 | 92 | 93 | 95 => {
                    upper = VIEW_UPPER_YES as libc::c_int;
                }
                94 | 96 | 99 => {
                    upper = VIEW_UPPER_FORCE as libc::c_int;
                }
                _ => {}
            }
        }
        17 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    72 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"viewpoint_type\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    match ast_id(l_type) as libc::c_uint {
        151 | 187 => {
            let mut cap_0: *mut ast_t = cap_fetch(l_type);
            match ast_id(cap_0) as libc::c_uint {
                93 => return ast_dup(r_type),
                98 | 99 | 97 | 100 | 101 => {
                    if upper == VIEW_UPPER_YES as libc::c_int {
                        upper = VIEW_UPPER_NO as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        152 => {
            if upper == VIEW_UPPER_YES as libc::c_int {
                upper = VIEW_UPPER_NO as libc::c_int;
            }
        }
        91 | 92 | 93 | 94 | 95 | 96 => {}
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                l_type,
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
            let mut r_right: *mut ast_t = viewpoint_type(right, r_type);
            return viewpoint_type(left, r_right);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    126 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"viewpoint_type\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    let mut arrow: *mut ast_t = 0 as *mut ast_t;
    let mut basis_ast: *mut ast_t = l_type;
    let mut parent: *mut ast_t = 0 as *mut ast_t;
    let mut last_sibling: *mut ast_t = 0 as *mut ast_t;
    let mut node: *mut ast_t = 0 as *mut ast_t;
    node = ast_from(basis_ast, TK_ARROW);
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
        parent_0 = ast_dup(l_type);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_dup(l_type));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_dup(l_type));
    }
    if parent_0.is_null() {
        parent_0 = ast_dup(r_type);
    } else if last_sibling_0.is_null() {
        last_sibling_0 = ast_add(parent_0, ast_dup(r_type));
    } else {
        last_sibling_0 = ast_add_sibling(last_sibling_0, ast_dup(r_type));
    }
    ast_inheritflags(parent_0);
    arrow = parent;
    if upper != VIEW_UPPER_NO as libc::c_int {
        let mut arrow_upper: *mut ast_t = viewpoint_upper(arrow);
        if arrow_upper.is_null() {
            return arrow;
        }
        if arrow != arrow_upper {
            ast_free_unattached(arrow);
            arrow = arrow_upper;
        }
    }
    return arrow;
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn viewpoint_upper(mut type_0: *mut ast_t) -> *mut ast_t {
    if ast_id(type_0) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_ARROW\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            157 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"viewpoint_upper\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        type_0,
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
    let mut r_right: *mut ast_t = right;
    match ast_id(right) as libc::c_uint {
        151 | 187 => {}
        17 => {
            r_right = viewpoint_upper(right);
            if r_right.is_null() {
                return 0 as *mut ast_t;
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    176 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"viewpoint_upper\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    let mut l_cap: token_id = TK_NONE;
    let mut l_eph: token_id = TK_NONE;
    match ast_id(left) as libc::c_uint {
        91 | 92 | 93 | 94 | 95 | 96 => {
            l_cap = ast_id(left);
        }
        152 => {
            l_cap = TK_BOX;
        }
        151 | 187 => {
            let mut left_cap: *mut ast_t = cap_fetch(left);
            let mut left_eph: *mut ast_t = ast_sibling(left_cap);
            l_cap = ast_id(left_cap);
            l_eph = ast_id(left_eph);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    210 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"viewpoint_upper\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    let mut right_cap: *mut ast_t = cap_fetch(r_right);
    let mut right_eph: *mut ast_t = ast_sibling(right_cap);
    let mut r_cap: token_id = ast_id(right_cap);
    let mut r_eph: token_id = ast_id(right_eph);
    if !cap_view_upper(l_cap, l_eph, &mut r_cap, &mut r_eph) {
        return 0 as *mut ast_t;
    }
    let mut rr_right: *mut ast_t = set_cap_and_ephemeral(r_right, r_cap, r_eph);
    if r_right != right {
        ast_free_unattached(r_right);
    }
    return rr_right;
}
#[no_mangle]
#[c2rust::src_loc = "232:1"]
pub unsafe extern "C" fn viewpoint_lower(mut type_0: *mut ast_t) -> *mut ast_t {
    if ast_id(type_0) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_ARROW\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            239 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"viewpoint_lower\0"))
                .as_ptr(),
        );
    };
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        type_0,
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
    let mut r_right: *mut ast_t = right;
    match ast_id(right) as libc::c_uint {
        151 | 187 => {}
        17 => {
            r_right = viewpoint_lower(right);
            if r_right.is_null() {
                return 0 as *mut ast_t;
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    258 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"viewpoint_lower\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    let mut l_cap: token_id = TK_NONE;
    let mut l_eph: token_id = TK_NONE;
    match ast_id(left) as libc::c_uint {
        91 | 92 | 93 | 94 | 95 | 96 => {
            l_cap = ast_id(left);
        }
        152 => {
            l_cap = TK_CAP_READ;
        }
        151 | 187 => {
            let mut left_cap: *mut ast_t = cap_fetch(left);
            let mut left_eph: *mut ast_t = ast_sibling(left_cap);
            l_cap = ast_id(left_cap);
            l_eph = ast_id(left_eph);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                        as *const u8 as *const libc::c_char,
                    292 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"viewpoint_lower\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    let mut right_cap: *mut ast_t = cap_fetch(r_right);
    let mut right_eph: *mut ast_t = ast_sibling(right_cap);
    let mut r_cap: token_id = ast_id(right_cap);
    let mut r_eph: token_id = ast_id(right_eph);
    if !cap_view_lower(l_cap, l_eph, &mut r_cap, &mut r_eph) {
        return 0 as *mut ast_t;
    }
    let mut rr_right: *mut ast_t = set_cap_and_ephemeral(r_right, r_cap, r_eph);
    if r_right != right {
        ast_free_unattached(r_right);
    }
    return rr_right;
}
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn replace_type(
    mut astp: *mut *mut ast_t,
    mut target: *mut ast_t,
    mut with: *mut ast_t,
) {
    let mut ast: *mut ast_t = *astp;
    let mut child: *mut ast_t = ast_child(ast);
    while !child.is_null() {
        replace_type(&mut child, target, with);
        child = ast_sibling(child);
    }
    let mut node_type: *mut ast_t = ast_type(ast);
    if !node_type.is_null() {
        replace_type(&mut node_type, target, with);
    }
    if ast_id(ast) as libc::c_uint == ast_id(target) as libc::c_uint {
        match ast_id(target) as libc::c_uint {
            152 => {
                ast_replace(astp, ast_dup(with));
            }
            187 => {
                let mut target_def: *mut ast_t = ast_data(target) as *mut ast_t;
                let mut left_def: *mut ast_t = ast_data(ast) as *mut ast_t;
                if target_def == left_def {
                    let mut id: ast_ptr_t = 0 as *mut ast_t;
                    let mut cap: ast_ptr_t = 0 as *mut ast_t;
                    let mut eph: ast_ptr_t = 0 as *mut ast_t;
                    let mut children: [*mut *mut ast_t; 4] =
                        [&mut id, &mut cap, &mut eph, 0 as *mut *mut ast_t];
                    ast_get_children(
                        ast,
                        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
                            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
                            .wrapping_sub(1),
                        children.as_mut_ptr(),
                    );
                    let mut a_with: *mut ast_t = with;
                    match ast_id(eph) as libc::c_uint {
                        57 => {
                            let mut c_with: *mut ast_t =
                                consume_type(with, TK_NONE, 1 as libc::c_int != 0);
                            if !c_with.is_null() {
                                a_with = c_with;
                            }
                        }
                        58 => {
                            a_with = alias(with);
                        }
                        _ => {}
                    }
                    if a_with == with {
                        a_with = ast_dup(with);
                    }
                    ast_replace(astp, a_with);
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                            as *const u8 as *const libc::c_char,
                        378 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(
                            b"replace_type\0",
                        ))
                        .as_ptr(),
                    );
                };
            }
        }
    } else if ast_id(ast) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
        let mut left: ast_ptr_t = 0 as *mut ast_t;
        let mut right: ast_ptr_t = 0 as *mut ast_t;
        let mut children_0: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
        ast_get_children(
            ast,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
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
        let mut r_type: *mut ast_t = viewpoint_type(left, right);
        ast_replace(astp, r_type);
    }
}
#[no_mangle]
#[c2rust::src_loc = "388:1"]
pub unsafe extern "C" fn viewpoint_replace(
    mut ast: *mut ast_t,
    mut target: *mut ast_t,
    mut with: *mut ast_t,
    mut duplicate: bool,
) -> *mut ast_t {
    if ast_id(target) as libc::c_uint == TK_THISTYPE as libc::c_int as libc::c_uint
        || ast_id(target) as libc::c_uint == TK_TYPEPARAMREF as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(target) == TK_THISTYPE) || (ast_id(target) == TK_TYPEPARAMREF)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            395 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"viewpoint_replace\0"))
                .as_ptr(),
        );
    };
    let mut r_ast: *mut ast_t = 0 as *mut ast_t;
    if duplicate {
        r_ast = ast_dup(ast);
    } else {
        r_ast = ast;
    }
    replace_type(&mut r_ast, target, with);
    return r_ast;
}
#[no_mangle]
#[c2rust::src_loc = "408:1"]
pub unsafe extern "C" fn viewpoint_replacethis(
    mut ast: *mut ast_t,
    mut with: *mut ast_t,
    mut duplicate: bool,
) -> *mut ast_t {
    let mut thistype: *mut ast_t = ast_from(ast, TK_THISTYPE);
    let mut r_ast: *mut ast_t = viewpoint_replace(ast, thistype, with, duplicate);
    ast_free_unattached(thistype);
    return r_ast;
}
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn replace_typeparam(
    mut tuple: *mut ast_t,
    mut type_0: *mut ast_t,
    mut typeparamref: *mut ast_t,
    mut cap: token_id,
    mut eph: token_id,
) {
    let mut r_tp: *mut ast_t = set_cap_and_ephemeral(typeparamref, cap, eph);
    let mut r_type: *mut ast_t =
        viewpoint_replace(type_0, typeparamref, r_tp, 1 as libc::c_int != 0);
    ast_append(tuple, r_type);
}
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn viewpoint_reifytypeparam(
    mut type_0: *mut ast_t,
    mut typeparamref: *mut ast_t,
) -> *mut ast_t {
    if ast_id(typeparamref) as libc::c_uint == TK_TYPEPARAMREF as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(typeparamref) == TK_TYPEPARAMREF\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            426 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"viewpoint_reifytypeparam\0",
            ))
            .as_ptr(),
        );
    };
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut eph: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 4] = [&mut id, &mut cap, &mut eph, 0 as *mut *mut ast_t];
    ast_get_children(
        typeparamref,
        ::core::mem::size_of::<[*mut *mut ast_t; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>())
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    match ast_id(cap) as libc::c_uint {
        91 | 92 | 93 | 94 | 95 | 96 => return 0 as *mut ast_t,
        98 => {
            let mut tuple: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
            replace_typeparam(tuple, type_0, typeparamref, TK_ISO, ast_id(eph));
            replace_typeparam(tuple, type_0, typeparamref, TK_VAL, TK_NONE);
            replace_typeparam(tuple, type_0, typeparamref, TK_TAG, TK_NONE);
            return tuple;
        }
        99 => {
            let mut tuple_0: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
            replace_typeparam(tuple_0, type_0, typeparamref, TK_VAL, TK_NONE);
            replace_typeparam(tuple_0, type_0, typeparamref, TK_TAG, TK_NONE);
            return tuple_0;
        }
        97 => {
            let mut tuple_1: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
            replace_typeparam(tuple_1, type_0, typeparamref, TK_REF, TK_NONE);
            replace_typeparam(tuple_1, type_0, typeparamref, TK_VAL, TK_NONE);
            replace_typeparam(tuple_1, type_0, typeparamref, TK_BOX, TK_NONE);
            return tuple_1;
        }
        100 => {
            let mut tuple_2: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
            replace_typeparam(tuple_2, type_0, typeparamref, TK_REF, TK_NONE);
            replace_typeparam(tuple_2, type_0, typeparamref, TK_VAL, TK_NONE);
            replace_typeparam(tuple_2, type_0, typeparamref, TK_BOX, TK_NONE);
            replace_typeparam(tuple_2, type_0, typeparamref, TK_TAG, TK_NONE);
            return tuple_2;
        }
        101 => {
            let mut tuple_3: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
            replace_typeparam(tuple_3, type_0, typeparamref, TK_ISO, ast_id(eph));
            replace_typeparam(tuple_3, type_0, typeparamref, TK_TRN, ast_id(eph));
            replace_typeparam(tuple_3, type_0, typeparamref, TK_REF, TK_NONE);
            replace_typeparam(tuple_3, type_0, typeparamref, TK_VAL, TK_NONE);
            replace_typeparam(tuple_3, type_0, typeparamref, TK_BOX, TK_NONE);
            replace_typeparam(tuple_3, type_0, typeparamref, TK_TAG, TK_NONE);
            return tuple_3;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            490 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"viewpoint_reifytypeparam\0",
            ))
            .as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "494:1"]
pub unsafe extern "C" fn viewpoint_reifythis(mut type_0: *mut ast_t) -> *mut ast_t {
    let mut tuple: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
    let mut this_ref: *mut ast_t = ast_from(type_0, TK_REF);
    ast_append(
        tuple,
        viewpoint_replacethis(type_0, this_ref, 1 as libc::c_int != 0),
    );
    ast_free_unattached(this_ref);
    let mut this_val: *mut ast_t = ast_from(type_0, TK_VAL);
    ast_append(
        tuple,
        viewpoint_replacethis(type_0, this_val, 1 as libc::c_int != 0),
    );
    ast_free_unattached(this_val);
    let mut this_box: *mut ast_t = ast_from(type_0, TK_BOX);
    ast_append(
        tuple,
        viewpoint_replacethis(type_0, this_box, 1 as libc::c_int != 0),
    );
    ast_free_unattached(this_box);
    return tuple;
}
#[no_mangle]
#[c2rust::src_loc = "513:1"]
pub unsafe extern "C" fn viewpoint_reifypair(
    mut a: *mut ast_t,
    mut b: *mut ast_t,
    mut r_a: *mut *mut ast_t,
    mut r_b: *mut *mut ast_t,
) -> bool {
    if ast_id(a) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(a) == TK_ARROW\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            515 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"viewpoint_reifypair\0"))
                .as_ptr(),
        );
    };
    if ast_id(b) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(b) == TK_ARROW\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.c\0"
                as *const u8 as *const libc::c_char,
            516 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"viewpoint_reifypair\0"))
                .as_ptr(),
        );
    };
    let mut test: *mut ast_t = a;
    while ast_id(test) as libc::c_uint == TK_ARROW as libc::c_int as libc::c_uint {
        let mut left: ast_ptr_t = 0 as *mut ast_t;
        let mut right: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
        ast_get_children(
            test,
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
        match ast_id(left) as libc::c_uint {
            152 => {
                *r_a = viewpoint_reifythis(a);
                *r_b = viewpoint_reifythis(b);
                return 1 as libc::c_int != 0;
            }
            187 => {
                let mut r: *mut ast_t = viewpoint_reifytypeparam(a, left);
                if !r.is_null() {
                    *r_a = r;
                    *r_b = viewpoint_reifytypeparam(b, left);
                    return 1 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        test = right;
    }
    if ast_id(test) as libc::c_uint == TK_TYPEPARAMREF as libc::c_int as libc::c_uint {
        let mut r_0: *mut ast_t = viewpoint_reifytypeparam(a, test);
        if r_0.is_null() {
            return 0 as libc::c_int != 0;
        }
        *r_a = r_0;
        *r_b = viewpoint_reifytypeparam(b, test);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
