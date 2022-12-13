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
    use super::token_h::{token_id, TK_EOF};
    extern "C" {
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
            out_children: *mut *mut *mut ast_t,
        );
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "63:1"]
        pub fn ast_dup(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "87:1"]
        pub fn ast_inheritflags(ast: *mut ast_t);
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
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:3"]
pub mod cap_h {
    #[c2rust::src_loc = "76:9"]
    pub type direction = libc::c_uint;
    #[c2rust::src_loc = "78:3"]
    pub const EXTRACT: direction = 1;
    #[c2rust::src_loc = "77:3"]
    pub const WRITE: direction = 0;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn is_cap_sub_cap_bound(
            sub: token_id,
            subalias: token_id,
            super_0: token_id,
            supalias: token_id,
        ) -> bool;
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "81:1"]
        pub fn cap_safetomove(into: token_id, cap: token_id, direction: direction) -> bool;
        #[c2rust::src_loc = "95:1"]
        pub fn cap_sendable(cap: token_id) -> bool;
        #[c2rust::src_loc = "97:1"]
        pub fn cap_immutable_or_opaque(cap: token_id) -> bool;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/compattype.h:4"]
pub mod compattype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn is_compat_type(a: *mut ast_t, b: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/viewpoint.h:5"]
pub mod viewpoint_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn viewpoint_lower(type_0: *mut ast_t) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:8"]
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
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
use self::assemble_h::set_cap_and_ephemeral;
pub use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_child, ast_dup, ast_free_unattached, ast_from,
    ast_get_children, ast_id, ast_inheritflags, ast_ptr_t, ast_setid, ast_sibling,
};
pub use self::cap_h::{
    cap_fetch, cap_immutable_or_opaque, cap_safetomove, cap_sendable, direction,
    is_cap_sub_cap_bound, EXTRACT, WRITE,
};
use self::compattype_h::is_compat_type;
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
use self::viewpoint_h::viewpoint_lower;
#[c2rust::src_loc = "10:9"]
pub type recovery_t = libc::c_uint;
#[c2rust::src_loc = "14:3"]
pub const RECOVERY_MUTABLE: recovery_t = 2;
#[c2rust::src_loc = "13:3"]
pub const RECOVERY_IMMUTABLE: recovery_t = 1;
#[c2rust::src_loc = "12:3"]
pub const RECOVERY_NONE: recovery_t = 0;
#[c2rust::src_loc = "17:1"]
unsafe extern "C" fn alias_single(mut type_0: *mut ast_t) -> *mut ast_t {
    let mut cap: *mut ast_t = cap_fetch(type_0);
    let mut ephemeral: *mut ast_t = ast_sibling(cap);
    match ast_id(ephemeral) as libc::c_uint {
        57 => {
            type_0 = ast_dup(type_0);
            ephemeral = ast_sibling(cap_fetch(type_0));
            ast_setid(ephemeral, TK_NONE);
            return type_0;
        }
        2 => {
            match ast_id(cap) as libc::c_uint {
                91 | 92 | 98 | 101 | 2 => {
                    type_0 = ast_dup(type_0);
                    ephemeral = ast_sibling(cap_fetch(type_0));
                    ast_setid(ephemeral, TK_ALIASED);
                }
                _ => {}
            }
            return type_0;
        }
        58 => return type_0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            56 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"alias_single\0")).as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn recover_single(
    mut type_0: *mut ast_t,
    mut rcap: token_id,
    mut tuple_elem_recover: *mut recovery_t,
) -> *mut ast_t {
    let mut cap: *mut ast_t = cap_fetch(type_0);
    let mut tcap: token_id = ast_id(cap);
    let mut rec_eph: bool = 0 as libc::c_int != 0;
    match tcap as libc::c_uint {
        91 => {
            if rcap as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                rcap = TK_ISO;
            }
            let mut eph: *mut ast_t = ast_sibling(cap);
            if ast_id(eph) as libc::c_uint != TK_EPHEMERAL as libc::c_int as libc::c_uint
                && rcap as libc::c_uint != TK_ISO as libc::c_int as libc::c_uint
                && rcap as libc::c_uint != TK_TAG as libc::c_int as libc::c_uint
            {
                return 0 as *mut ast_t;
            }
        }
        92 | 93 => {
            if rcap as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                rcap = TK_ISO;
            }
            if !tuple_elem_recover.is_null() && !cap_safetomove(rcap, tcap, WRITE) {
                match rcap as libc::c_uint {
                    91 | 92 => {
                        if *tuple_elem_recover as libc::c_uint
                            != RECOVERY_NONE as libc::c_int as libc::c_uint
                        {
                            return 0 as *mut ast_t;
                        }
                        *tuple_elem_recover = RECOVERY_MUTABLE;
                    }
                    94 => {
                        if *tuple_elem_recover as libc::c_uint
                            == RECOVERY_MUTABLE as libc::c_int as libc::c_uint
                        {
                            return 0 as *mut ast_t;
                        }
                        *tuple_elem_recover = RECOVERY_IMMUTABLE;
                    }
                    _ => {}
                }
            }
            rec_eph = 1 as libc::c_int != 0;
        }
        94 | 95 | 97 => {
            if rcap as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                rcap = TK_VAL;
            }
            match rcap as libc::c_uint {
                91 | 92 | 93 | 97 | 98 | 100 | 101 => return 0 as *mut ast_t,
                94 => {
                    if !tuple_elem_recover.is_null()
                        && tcap as libc::c_uint != TK_VAL as libc::c_int as libc::c_uint
                    {
                        if *tuple_elem_recover as libc::c_uint
                            == RECOVERY_MUTABLE as libc::c_int as libc::c_uint
                        {
                            return 0 as *mut ast_t;
                        }
                        *tuple_elem_recover = RECOVERY_IMMUTABLE;
                    }
                }
                95 | 96 | 99 => {}
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                                as *const u8 as *const libc::c_char,
                            151 as libc::c_int as size_t,
                            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"recover_single\0",
                            ))
                            .as_ptr(),
                        );
                    };
                    return 0 as *mut ast_t;
                }
            }
        }
        96 | 98 | 99 | 100 | 101 => {
            if rcap as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                rcap = tcap;
            }
            if rcap as libc::c_uint != TK_TAG as libc::c_int as libc::c_uint
                && rcap as libc::c_uint != tcap as libc::c_uint
            {
                return 0 as *mut ast_t;
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                        as *const u8 as *const libc::c_char,
                    170 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"recover_single\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as *mut ast_t;
        }
    }
    type_0 = ast_dup(type_0);
    cap = cap_fetch(type_0);
    ast_setid(cap, rcap);
    if rec_eph {
        ast_setid(ast_sibling(cap), TK_EPHEMERAL);
    }
    return type_0;
}
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn consume_single(
    mut type_0: *mut ast_t,
    mut ccap: token_id,
    mut keep_double_ephemeral: bool,
) -> *mut ast_t {
    type_0 = ast_dup(type_0);
    let mut cap: *mut ast_t = cap_fetch(type_0);
    let mut eph: *mut ast_t = ast_sibling(cap);
    let mut tcap: token_id = ast_id(cap);
    let mut teph: token_id = ast_id(eph);
    ast_setid(cap, tcap);
    ast_setid(eph, teph);
    match teph as libc::c_uint {
        2 => {
            ast_setid(eph, TK_EPHEMERAL);
        }
        58 => {
            if ccap as libc::c_uint == TK_ALIASED as libc::c_int as libc::c_uint {
                ast_setid(eph, TK_NONE);
            }
        }
        57 => match tcap as libc::c_uint {
            91 | 92 => {
                if !keep_double_ephemeral {
                    ast_free_unattached(type_0);
                    return 0 as *mut ast_t;
                }
            }
            _ => {}
        },
        _ => {}
    }
    match ccap as libc::c_uint {
        2 | 58 => {
            ccap = tcap;
        }
        _ => {}
    }
    if !is_cap_sub_cap_bound(tcap, TK_EPHEMERAL, ccap, TK_NONE) {
        ast_free_unattached(type_0);
        return 0 as *mut ast_t;
    }
    ast_setid(cap, ccap);
    return type_0;
}
#[no_mangle]
#[c2rust::src_loc = "243:1"]
pub unsafe extern "C" fn alias(mut type_0: *mut ast_t) -> *mut ast_t {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {
            let mut r_type: *mut ast_t = ast_from(type_0, ast_id(type_0));
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                ast_append(r_type, alias(child));
                child = ast_sibling(child);
            }
            return r_type;
        }
        151 | 187 => return alias_single(type_0),
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut r_type_0: *mut ast_t = 0 as *mut ast_t;
            let mut basis_ast: *mut ast_t = type_0;
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
            let mut node_0: *mut ast_t = 0 as *mut ast_t;
            if parent_0.is_null() {
                parent_0 = left;
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(parent_0, left);
            } else {
                last_sibling_0 = ast_add_sibling(last_sibling_0, left);
            }
            if parent_0.is_null() {
                parent_0 = alias(right);
            } else if last_sibling_0.is_null() {
                last_sibling_0 = ast_add(parent_0, alias(right));
            } else {
                last_sibling_0 = ast_add_sibling(last_sibling_0, alias(right));
            }
            ast_inheritflags(parent_0);
            r_type_0 = parent;
            return r_type_0;
        }
        154 | 155 | 153 | 157 | 158 | 156 => return type_0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            293 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"alias\0")).as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn consume_type(
    mut type_0: *mut ast_t,
    mut cap: token_id,
    mut keep_double_ephemeral: bool,
) -> *mut ast_t {
    's_347: {
        match ast_id(type_0) as libc::c_uint {
            156 => return type_0,
            150 => {
                let mut r_type: *mut ast_t = ast_from(type_0, ast_id(type_0));
                let mut child: *mut ast_t = ast_child(type_0);
                while !child.is_null() {
                    let mut r_right: *mut ast_t = consume_type(child, cap, keep_double_ephemeral);
                    if r_right.is_null() {
                        ast_free_unattached(r_type);
                        return 0 as *mut ast_t;
                    }
                    ast_append(r_type, r_right);
                    child = ast_sibling(child);
                }
                return r_type;
            }
            56 => {
                let mut first: *mut ast_t = ast_child(type_0);
                let mut second: *mut ast_t = ast_sibling(first);
                if !is_compat_type(first, second) {
                    return 0 as *mut ast_t;
                }
            }
            149 => {}
            151 | 187 => return consume_single(type_0, cap, keep_double_ephemeral),
            17 => {
                let mut left: ast_ptr_t = 0 as *mut ast_t;
                let mut right: ast_ptr_t = 0 as *mut ast_t;
                let mut children: [*mut *mut ast_t; 3] =
                    [&mut left, &mut right, 0 as *mut *mut ast_t];
                ast_get_children(
                    type_0,
                    (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    children.as_mut_ptr(),
                );
                let mut r_right_1: *mut ast_t = consume_type(right, cap, keep_double_ephemeral);
                if r_right_1.is_null() {
                    return 0 as *mut ast_t;
                }
                let mut r_type_1: *mut ast_t = 0 as *mut ast_t;
                let mut basis_ast: *mut ast_t = type_0;
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
                let mut node_0: *mut ast_t = 0 as *mut ast_t;
                if parent_0.is_null() {
                    parent_0 = left;
                } else if last_sibling_0.is_null() {
                    last_sibling_0 = ast_add(parent_0, left);
                } else {
                    last_sibling_0 = ast_add_sibling(last_sibling_0, left);
                }
                if parent_0.is_null() {
                    parent_0 = r_right_1;
                } else if last_sibling_0.is_null() {
                    last_sibling_0 = ast_add(parent_0, r_right_1);
                } else {
                    last_sibling_0 = ast_add_sibling(last_sibling_0, r_right_1);
                }
                ast_inheritflags(parent_0);
                r_type_1 = parent;
                return r_type_1;
            }
            153 | 157 | 158 => return type_0,
            _ => {
                break 's_347;
            }
        }
        let mut r_type_0: *mut ast_t = ast_from(type_0, ast_id(type_0));
        let mut child_0: *mut ast_t = ast_child(type_0);
        while !child_0.is_null() {
            let mut r_right_0: *mut ast_t = consume_type(child_0, cap, keep_double_ephemeral);
            if !r_right_0.is_null() {
                ast_append(r_type_0, r_right_0);
            }
            child_0 = ast_sibling(child_0);
        }
        return r_type_0;
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            393 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"consume_type\0")).as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[c2rust::src_loc = "400:1"]
unsafe extern "C" fn recover_complex(
    mut type_0: *mut ast_t,
    mut cap: token_id,
    mut tuple_elem_recover: *mut recovery_t,
) -> *mut ast_t {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"false\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                        as *const u8 as *const libc::c_char,
                    411 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"recover_complex\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    }
    let mut r_type: *mut ast_t = ast_from(type_0, ast_id(type_0));
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        let mut r_right: *mut ast_t = recover_type_inner(child, cap, tuple_elem_recover);
        if r_right.is_null() {
            ast_free_unattached(r_type);
            return 0 as *mut ast_t;
        }
        ast_append(r_type, r_right);
        child = ast_sibling(child);
    }
    return r_type;
}
#[c2rust::src_loc = "436:1"]
unsafe extern "C" fn recover_type_inner(
    mut type_0: *mut ast_t,
    mut cap: token_id,
    mut tuple_elem_recover: *mut recovery_t,
) -> *mut ast_t {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 => return recover_complex(type_0, cap, tuple_elem_recover),
        150 => {
            if !tuple_elem_recover.is_null() {
                return recover_complex(type_0, cap, tuple_elem_recover);
            }
            let mut elem_recover: recovery_t = RECOVERY_NONE;
            return recover_complex(type_0, cap, &mut elem_recover);
        }
        151 | 187 => return recover_single(type_0, cap, tuple_elem_recover),
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut r_right: *mut ast_t = recover_type_inner(right, cap, tuple_elem_recover);
            if r_right.is_null() {
                return 0 as *mut ast_t;
            }
            let mut r_type: *mut ast_t = ast_from(type_0, TK_ARROW);
            ast_add(r_type, r_right);
            ast_add(r_type, left);
            return r_type;
        }
        153 | 157 | 158 => return type_0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            482 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"recover_type_inner\0"))
                .as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "486:1"]
pub unsafe extern "C" fn recover_type(mut type_0: *mut ast_t, mut cap: token_id) -> *mut ast_t {
    return recover_type_inner(type_0, cap, 0 as *mut recovery_t);
}
#[no_mangle]
#[c2rust::src_loc = "491:1"]
pub unsafe extern "C" fn chain_type(
    mut type_0: *mut ast_t,
    mut fun_cap: token_id,
    mut recovered_call: bool,
) -> *mut ast_t {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {
            let mut c_type: *mut ast_t = ast_from(type_0, ast_id(type_0));
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                ast_append(c_type, chain_type(child, fun_cap, recovered_call));
                child = ast_sibling(child);
            }
            return c_type;
        }
        151 | 187 => {
            let mut cap: *mut ast_t = cap_fetch(type_0);
            let mut eph: *mut ast_t = ast_sibling(cap);
            match ast_id(cap) as libc::c_uint {
                93 | 94 | 95 | 96 | 97 | 99 | 100 => return ast_dup(type_0),
                101 => {
                    if fun_cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint {
                    } else {
                        ponyint_assert_fail(
                            b"fun_cap == TK_TAG\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                                as *const u8 as *const libc::c_char,
                            533 as libc::c_int as size_t,
                            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(
                                b"chain_type\0",
                            ))
                            .as_ptr(),
                        );
                    };
                    return ast_dup(type_0);
                }
                _ => {}
            }
            if ast_id(eph) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint {
                if recovered_call as libc::c_int != 0
                    || fun_cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint
                {
                } else {
                    ponyint_assert_fail(
                        b"recovered_call || (fun_cap == TK_TAG)\0" as *const u8
                            as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                            as *const u8 as *const libc::c_char,
                        541 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(
                            b"chain_type\0",
                        ))
                        .as_ptr(),
                    );
                };
                return ast_dup(type_0);
            }
            if ast_id(eph) as libc::c_uint == TK_EPHEMERAL as libc::c_int as libc::c_uint {
            } else {
                ponyint_assert_fail(
                    b"ast_id(eph) == TK_EPHEMERAL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                        as *const u8 as *const libc::c_char,
                    547 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"chain_type\0"))
                        .as_ptr(),
                );
            };
            if recovered_call {
                return ast_dup(type_0);
            }
            match ast_id(cap) as libc::c_uint {
                91 | 98 => match fun_cap as libc::c_uint {
                    96 => return ast_dup(type_0),
                    91 => return alias(alias(type_0)),
                    _ => {}
                },
                92 => match fun_cap as libc::c_uint {
                    96 | 95 => return ast_dup(type_0),
                    92 => return alias(alias(type_0)),
                    _ => {}
                },
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        ponyint_assert_fail(
                            b"false\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0"
                                as *const u8 as *const libc::c_char,
                            585 as libc::c_int as size_t,
                            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(
                                b"chain_type\0",
                            ))
                            .as_ptr(),
                        );
                    };
                    return 0 as *mut ast_t;
                }
            }
            return set_cap_and_ephemeral(type_0, fun_cap, TK_NONE);
        }
        17 => {
            let mut left: ast_ptr_t = 0 as *mut ast_t;
            let mut right: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut c_right: *mut ast_t = chain_type(right, fun_cap, recovered_call);
            let mut c_type_0: *mut ast_t = ast_from(type_0, TK_ARROW);
            ast_add(c_type_0, c_right);
            ast_add(c_type_0, left);
            return c_type_0;
        }
        153 | 157 | 158 => return type_0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            613 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"chain_type\0")).as_ptr(),
        );
    };
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "617:1"]
pub unsafe extern "C" fn sendable(mut type_0: *mut ast_t) -> bool {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if !sendable(child) {
                    return 0 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 1 as libc::c_int != 0;
        }
        17 => {
            let mut lower: *mut ast_t = viewpoint_lower(type_0);
            if lower.is_null() {
                return 0 as libc::c_int != 0;
            }
            let mut ok: bool = sendable(lower);
            ast_free_unattached(lower);
            return ok;
        }
        151 => {
            let mut pkg: ast_ptr_t = 0 as *mut ast_t;
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
            let mut cap: ast_ptr_t = 0 as *mut ast_t;
            let mut eph: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 6] = [
                &mut pkg,
                &mut id,
                &mut typeparams,
                &mut cap,
                &mut eph,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            return cap_sendable(ast_id(cap));
        }
        187 => {
            let mut id_0: ast_ptr_t = 0 as *mut ast_t;
            let mut cap_0: ast_ptr_t = 0 as *mut ast_t;
            let mut eph_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 4] =
                [&mut id_0, &mut cap_0, &mut eph_0, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            return cap_sendable(ast_id(cap_0));
        }
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            671 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"sendable\0")).as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "675:1"]
pub unsafe extern "C" fn immutable_or_opaque(mut type_0: *mut ast_t) -> bool {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 | 150 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if !immutable_or_opaque(child) {
                    return 0 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 1 as libc::c_int != 0;
        }
        17 => {
            let mut lower: *mut ast_t = viewpoint_lower(type_0);
            if lower.is_null() {
                return 0 as libc::c_int != 0;
            }
            let mut ok: bool = immutable_or_opaque(lower);
            ast_free_unattached(lower);
            return ok;
        }
        151 => {
            let mut pkg: ast_ptr_t = 0 as *mut ast_t;
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
            let mut cap: ast_ptr_t = 0 as *mut ast_t;
            let mut eph: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 6] = [
                &mut pkg,
                &mut id,
                &mut typeparams,
                &mut cap,
                &mut eph,
                0 as *mut *mut ast_t,
            ];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            return cap_immutable_or_opaque(ast_id(cap));
        }
        187 => {
            let mut id_0: ast_ptr_t = 0 as *mut ast_t;
            let mut cap_0: ast_ptr_t = 0 as *mut ast_t;
            let mut eph_0: ast_ptr_t = 0 as *mut ast_t;
            let mut children_0: [*mut *mut ast_t; 4] =
                [&mut id_0, &mut cap_0, &mut eph_0, 0 as *mut *mut ast_t];
            ast_get_children(
                type_0,
                (::core::mem::size_of::<[*mut *mut ast_t; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children_0.as_mut_ptr(),
            );
            return cap_immutable_or_opaque(ast_id(cap_0));
        }
        153 | 157 | 158 => return 0 as libc::c_int != 0,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/alias.c\0" as *const u8
                as *const libc::c_char,
            729 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"immutable_or_opaque\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
