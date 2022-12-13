use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/source.h:1"]
pub mod source_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:16"]
    pub struct source_t {
        pub file: *const libc::c_char,
        pub m: *mut libc::c_char,
        pub len: size_t,
    }
    use super::_size_t_h::size_t;
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
    extern "C" {
        #[c2rust::src_loc = "16:16"]
        pub type token_t;
        #[c2rust::src_loc = "351:1"]
        pub fn token_id_desc(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:1"]
pub mod lexer_h {
    extern "C" {
        #[c2rust::src_loc = "12:16"]
        pub type errors_t;
    }
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
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "144:1"]
        pub fn ast_reorder_children(
            ast: *mut ast_t,
            new_order: *const size_t,
            shuffle_space: *mut *mut ast_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.h:1"]
pub mod parserapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:16"]
    pub struct rule_state_t {
        pub fn_name: *const libc::c_char,
        pub ast: *mut ast_t,
        pub last_child: *mut ast_t,
        pub desc: *const libc::c_char,
        pub restart: *mut token_id,
        pub deflt_id: token_id,
        pub matched: bool,
        pub scope: bool,
        pub deferred: bool,
        pub deferred_id: token_id,
        pub line: size_t,
        pub pos: size_t,
    }
    #[c2rust::src_loc = "98:1"]
    pub type builder_fn_t = Option<unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ()>;
    #[c2rust::src_loc = "100:1"]
    pub type rule_t = Option<
        unsafe extern "C" fn(*mut parser_t, *mut builder_fn_t, *const libc::c_char) -> *mut ast_t,
    >;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::lexer_h::errors_t;
    use super::source_h::source_t;
    use super::symtab_h::ast_t;
    use super::token_h::{token_id, token_t, TK_EOF};
    extern "C" {
        #[c2rust::src_loc = "75:16"]
        pub type parser_t;
        #[c2rust::src_loc = "138:1"]
        pub fn parse(
            package: *mut ast_t,
            source: *mut source_t,
            start: rule_t,
            expected: *const libc::c_char,
            errors: *mut errors_t,
            allow_test_symbols: bool,
            trace: bool,
        ) -> bool;
        #[c2rust::src_loc = "127:1"]
        pub fn parse_rule_complete(parser: *mut parser_t, state: *mut rule_state_t) -> *mut ast_t;
        #[c2rust::src_loc = "125:1"]
        pub fn parse_set_next_flags(parser: *mut parser_t, flags: uint32_t);
        #[c2rust::src_loc = "122:1"]
        pub fn parse_rule_set(
            parser: *mut parser_t,
            state: *mut rule_state_t,
            desc: *const libc::c_char,
            rule_set: *const rule_t,
            out_found: *mut bool,
            annotate: bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "118:1"]
        pub fn parse_token_set(
            parser: *mut parser_t,
            state: *mut rule_state_t,
            desc: *const libc::c_char,
            terminating: *const libc::c_char,
            id_set: *const token_id,
            make_ast: bool,
            out_found: *mut bool,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn add_deferrable_ast(
            parser: *mut parser_t,
            state: *mut rule_state_t,
            id: token_id,
            token_for_pos: *mut token_t,
        );
        #[c2rust::src_loc = "111:1"]
        pub fn infix_builder(state: *mut rule_state_t, new_ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:1"]
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
pub use self::_uint32_t_h::uint32_t;
pub use self::ast_h::{
    ast_childcount, ast_childidx, ast_id, ast_reorder_children, ast_setflag, ast_setid,
    C2RustUnnamed, AST_FLAG_AMBIGUOUS, AST_FLAG_BAD_SEMI, AST_FLAG_CAN_ERROR, AST_FLAG_CAN_SEND,
    AST_FLAG_CNSM_REASGN, AST_FLAG_DONE_1, AST_FLAG_DONE_2, AST_FLAG_ERROR_1, AST_FLAG_ERROR_2,
    AST_FLAG_FCNSM_REASGN, AST_FLAG_IMPORT, AST_FLAG_INCOMPLETE, AST_FLAG_IN_PARENS,
    AST_FLAG_JUMPS_AWAY, AST_FLAG_MAY_BREAK, AST_FLAG_MIGHT_SEND, AST_FLAG_MISSING_SEMI,
    AST_FLAG_PASS_MASK, AST_FLAG_PRESERVE, AST_FLAG_RECURSE_1, AST_FLAG_RECURSE_2,
};
use self::lexer_h::errors_t;
pub use self::parserapi_h::{
    add_deferrable_ast, builder_fn_t, infix_builder, parse, parse_rule_complete, parse_rule_set,
    parse_set_next_flags, parse_token_set, parser_t, rule_state_t, rule_t,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::source_h::source_t;
use self::symtab_h::ast_t;
pub use self::token_h::{
    token_id, token_id_desc, token_t, TK_ACTOR, TK_ADDRESS, TK_ALIASED, TK_AND, TK_ANNOTATION,
    TK_ARRAY, TK_ARROW, TK_AS, TK_ASSIGN, TK_AT, TK_AT_LBRACE, TK_BACKSLASH, TK_BARELAMBDA,
    TK_BARELAMBDATYPE, TK_BE, TK_BEAPP, TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK, TK_CALL,
    TK_CAP_ALIAS, TK_CAP_ANY, TK_CAP_READ, TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES, TK_CHAIN,
    TK_CLASS, TK_COLON, TK_COMMA, TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT, TK_CONSUME,
    TK_CONTINUE, TK_DBLARROW, TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE, TK_DIVIDE_TILDE, TK_DO,
    TK_DONTCARE, TK_DONTCAREREF, TK_DONTCARETYPE, TK_DOT, TK_ELLIPSIS, TK_ELSE, TK_ELSEIF,
    TK_EMBED, TK_EMBEDREF, TK_END, TK_EOF, TK_EPHEMERAL, TK_EQ, TK_EQ_TILDE, TK_ERROR,
    TK_ERRORTYPE, TK_FALSE, TK_FFICALL, TK_FFIDECL, TK_FLATTEN, TK_FLET, TK_FLETREF, TK_FLOAT,
    TK_FOR, TK_FUN, TK_FUNAPP, TK_FUNCHAIN, TK_FUNREF, TK_FUNTYPE, TK_FVAR, TK_FVARREF, TK_GE,
    TK_GE_TILDE, TK_GT, TK_GT_TILDE, TK_ID, TK_IF, TK_IFDEF, TK_IFDEFAND, TK_IFDEFFLAG,
    TK_IFDEFNOT, TK_IFDEFOR, TK_IFTYPE, TK_IFTYPE_SET, TK_IN, TK_INFERTYPE, TK_INT, TK_INTERFACE,
    TK_IS, TK_ISECTTYPE, TK_ISNT, TK_ISO, TK_LAMBDA, TK_LAMBDACAPTURE, TK_LAMBDACAPTURES,
    TK_LAMBDATYPE, TK_LBRACE, TK_LE, TK_LET, TK_LETREF, TK_LEX_ERROR, TK_LE_TILDE, TK_LITERAL,
    TK_LITERALBRANCH, TK_LOCATION, TK_LPAREN, TK_LPAREN_NEW, TK_LSHIFT, TK_LSHIFT_TILDE,
    TK_LSQUARE, TK_LSQUARE_NEW, TK_LT, TK_LT_TILDE, TK_MATCH, TK_MATCH_CAPTURE, TK_MATCH_DONTCARE,
    TK_MEMBERS, TK_MINUS, TK_MINUS_NEW, TK_MINUS_TILDE, TK_MINUS_TILDE_NEW, TK_MOD, TK_MODULE,
    TK_MOD_TILDE, TK_MULTIPLY, TK_MULTIPLY_TILDE, TK_NAMEDARG, TK_NAMEDARGS, TK_NE, TK_NEW,
    TK_NEWAPP, TK_NEWBEREF, TK_NEWLINE, TK_NEWREF, TK_NE_TILDE, TK_NOMINAL, TK_NONE, TK_NOT,
    TK_OBJECT, TK_OPERATORLITERAL, TK_OR, TK_PACKAGE, TK_PACKAGEREF, TK_PARAM, TK_PARAMREF,
    TK_PARAMS, TK_PIPE, TK_PLUS, TK_PLUS_TILDE, TK_POSITIONALARGS, TK_PRIMITIVE, TK_PROGRAM,
    TK_PROVIDES, TK_QUALIFY, TK_QUESTION, TK_RBRACE, TK_RECOVER, TK_REF, TK_REFERENCE, TK_REM,
    TK_REM_TILDE, TK_REPEAT, TK_RETURN, TK_RPAREN, TK_RSHIFT, TK_RSHIFT_TILDE, TK_RSQUARE, TK_SEMI,
    TK_SEQ, TK_STRING, TK_STRUCT, TK_SUBTYPE, TK_TAG, TK_TEST_ALIASED, TK_TEST_EXTRA,
    TK_TEST_NO_SEQ, TK_TEST_SEQ_SCOPE, TK_TEST_TRY_NO_CHECK, TK_TEST_UPDATEARG, TK_THEN, TK_THIS,
    TK_THISTYPE, TK_TILDE, TK_TRAIT, TK_TRN, TK_TRUE, TK_TRY, TK_TRY_NO_CHECK, TK_TUPLE,
    TK_TUPLEELEMREF, TK_TUPLETYPE, TK_TYPE, TK_TYPEARGS, TK_TYPEPARAM, TK_TYPEPARAMREF,
    TK_TYPEPARAMS, TK_TYPEREF, TK_UNARY_MINUS, TK_UNARY_MINUS_TILDE, TK_UNIONTYPE, TK_UNTIL,
    TK_UPDATEARG, TK_USE, TK_VAL, TK_VALUEFORMALARG, TK_VALUEFORMALPARAM, TK_VAR, TK_VARREF,
    TK_WHERE, TK_WHILE, TK_WITH, TK_XOR,
};
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn provides(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"provides\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PROVIDES, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"provided type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn defaultarg(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"defaultarg\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    state.scope = 1 as libc::c_int != 0;
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                infix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"default value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn param(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"param\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PARAM, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"parameter name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"mandatory type declaration on parameter\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameter type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ASSIGN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    defaultarg
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"default value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn ellipsis(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"ellipsis\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_ELLIPSIS, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn literal(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"literal\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 6] = [TK_TRUE, TK_FALSE, TK_INT, TK_FLOAT, TK_STRING, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"literal\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn const_expr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"const_expr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_CONSTANT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                postfix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"formal argument value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn typeargliteral(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typeargliteral\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_VALUEFORMALARG, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                literal
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn typeargconst(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typeargconst\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_VALUEFORMALARG, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                const_expr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"formal argument value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn typearg(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typearg\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 4] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                typeargliteral
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                typeargconst
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn typeparam(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typeparam\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_TYPEPARAM, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if found {
        static mut rule_set: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type constraint\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ASSIGN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found_0 {
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    typearg
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"default type argument\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn params(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"params\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PARAMS, 0 as *mut token_t);
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                param
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                ellipsis
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameter\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 3] = unsafe {
            [
                Some(
                    param
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                Some(
                    ellipsis
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"parameter\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn typeparams(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typeparams\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_TYPEPARAMS, 0 as *mut token_t);
    static mut id_set: [token_id; 3] = [TK_LSQUARE, TK_LSQUARE_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typeparam
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameter\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    typeparam
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type parameter\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn typeargs(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typeargs\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_TYPEARGS, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_LSQUARE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typearg
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    typearg
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type argument\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"type arguments\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn cap(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"cap\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 7] = [TK_ISO, TK_TRN, TK_REF, TK_VAL, TK_BOX, TK_TAG, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn gencap(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"gencap\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 6] = [
        TK_CAP_READ,
        TK_CAP_SEND,
        TK_CAP_SHARE,
        TK_CAP_ALIAS,
        TK_CAP_ANY,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"generic capability\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn bare(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"bare\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_AT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"@\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn nominal(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nominal\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_NOMINAL, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_DOT, TK_NONE];
    state.deflt_id = TK_EOF;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_DOT),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if found {
        static mut id_set_1: [token_id; 2] = [TK_ID, TK_NONE];
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            b"name\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            id_set_1.as_ptr(),
            1 as libc::c_int != 0,
            0 as *mut bool,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    } else {
        add_deferrable_ast(parser, &mut state, TK_NONE, 0 as *mut token_t);
        static mut order: [size_t; 2] = [1 as libc::c_int as size_t, 0 as libc::c_int as size_t];
        if ast_childcount(state.ast)
            == (::core::mem::size_of::<[size_t; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
        {
        } else {
            ponyint_assert_fail(
                b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0"
                    as *const u8 as *const libc::c_char,
                181 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"nominal\0")).as_ptr(),
            );
        };
        static mut shuffle: [*mut ast_t; 2] = [0 as *const ast_t as *mut ast_t; 2];
        ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
        state.last_child = 0 as *mut ast_t;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typeargs
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type arguments\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                gencap
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_2: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn uniontype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"uniontype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    add_deferrable_ast(parser, &mut state, TK_UNIONTYPE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_PIPE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn isecttype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"isecttype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_ISECTTYPE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "204:1"]
unsafe extern "C" fn infixtype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"infixtype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                uniontype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                isecttype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "210:1"]
unsafe extern "C" fn tupletype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"tupletype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_COMMA as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_TUPLETYPE);
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                infixtype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    infixtype
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "219:1"]
unsafe extern "C" fn groupedtype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"groupedtype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                infixtype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                tupletype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    ast_setflag(state.ast, AST_FLAG_IN_PARENS as libc::c_int as uint32_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "229:1"]
unsafe extern "C" fn thistype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"thistype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_THISTYPE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_THIS, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn typelist(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"typelist\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PARAMS, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameter type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"parameter type\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn lambdatype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambdatype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_LAMBDATYPE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_LBRACE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"function name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typelist
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_3: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_7: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"return type\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_7 != 1 as libc::c_int as *mut ast_t {
            return r_7;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_4: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_8: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_4.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    static mut id_set_5: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut r_9: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_5.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_9 != 1 as libc::c_int as *mut ast_t {
        return r_9;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 3] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                gencap
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_10: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_10 != 1 as libc::c_int as *mut ast_t {
        return r_10;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_6: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut r_11: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_6.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_11 != 1 as libc::c_int as *mut ast_t {
        return r_11;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "263:1"]
unsafe extern "C" fn barelambdatype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"barelambdatype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_BARELAMBDATYPE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_AT_LBRACE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"function name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typelist
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_3: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_7: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"return type\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_7 != 1 as libc::c_int as *mut ast_t {
            return r_7;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_4: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_8: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_4.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    static mut id_set_5: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut r_9: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_5.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_9 != 1 as libc::c_int as *mut ast_t {
        return r_9;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 3] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                gencap
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_10: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_10 != 1 as libc::c_int as *mut ast_t {
        return r_10;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_6: [token_id; 3] = [TK_EPHEMERAL, TK_ALIASED, TK_NONE];
    let mut r_11: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_6.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_11 != 1 as libc::c_int as *mut ast_t {
        return r_11;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "280:1"]
unsafe extern "C" fn atomtype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"atomtype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 7] = unsafe {
        [
            Some(
                thistype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                groupedtype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nominal
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                lambdatype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                barelambdatype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn viewpoint(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"viewpoint\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_ARROW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"viewpoint\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "293:1"]
unsafe extern "C" fn type_0(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"type\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                atomtype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                viewpoint
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"viewpoint\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn namedarg(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"namedarg\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_NAMEDARG, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"argument name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_TEST_UPDATEARG, TK_NONE];
    state.deflt_id = TK_EOF;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_TEST_UPDATEARG),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if found {
        if ast_id(state.ast) as libc::c_uint == TK_NAMEDARG as libc::c_int as libc::c_uint {
            ast_setid(state.ast, TK_UPDATEARG);
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"argument value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "311:1"]
unsafe extern "C" fn named(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"named\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_NAMEDARGS, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_WHERE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                namedarg
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"named argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    namedarg
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"named argument\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "319:1"]
unsafe extern "C" fn positional(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"positional\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_POSITIONALARGS, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    rawseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"argument\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "326:1"]
unsafe extern "C" fn annotations(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"annotations\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_BACKSLASH, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_BACKSLASH as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_ANNOTATION);
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"annotation\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_1: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_1.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut id_set_2: [token_id; 2] = [TK_ID, TK_NONE];
        let mut r_2: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            b"annotation\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            id_set_2.as_ptr(),
            1 as libc::c_int != 0,
            0 as *mut bool,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    static mut id_set_3: [token_id; 2] = [TK_BACKSLASH, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"annotations\0" as *const u8 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "336:1"]
unsafe extern "C" fn object(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"object\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_OBJECT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_IS, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_IS),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    provides
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"provided type\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                members
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"object member\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_1: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"object literal\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    ast_setflag(
        ast_childidx(state.ast, 0 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 1 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 2 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "350:1"]
unsafe extern "C" fn lambdaparam(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambdaparam\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PARAM, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"parameter name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if found {
        static mut rule_set: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"parameter type\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ASSIGN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found_0 {
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    defaultarg
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"default value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn lambdaparams(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambdaparams\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_PARAMS, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                lambdaparam
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameter\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    lambdaparam
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"parameter\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "365:1"]
unsafe extern "C" fn lambdacapture(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambdacapture\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_LAMBDACAPTURE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if found {
        static mut rule_set: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"capture type\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ASSIGN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found_0 {
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    infix
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"capture value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn lambdacaptures(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambdacaptures\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_LAMBDACAPTURES, 0 as *mut token_t);
    static mut id_set: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                lambdacapture
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                thisliteral
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capture\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 3] = unsafe {
            [
                Some(
                    lambdacapture
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                Some(
                    thisliteral
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"capture\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn lambda(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"lambda\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_LAMBDA, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_LBRACE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"receiver capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"function name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                lambdaparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameters\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 2] = unsafe {
        [
            Some(
                lambdacaptures
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_7: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"captures\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_7 != 1 as libc::c_int as *mut ast_t {
        return r_7;
    }
    static mut id_set_3: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_8: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    if found {
        static mut rule_set_4: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_9: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"return type\0" as *const u8 as *const libc::c_char,
            rule_set_4.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_9 != 1 as libc::c_int as *mut ast_t {
            return r_9;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_4: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_10: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_4.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_10 != 1 as libc::c_int as *mut ast_t {
        return r_10;
    }
    static mut id_set_5: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut r_11: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_5.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_11 != 1 as libc::c_int as *mut ast_t {
        return r_11;
    }
    static mut rule_set_5: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_12: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"lambda body\0" as *const u8 as *const libc::c_char,
        rule_set_5.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_12 != 1 as libc::c_int as *mut ast_t {
        return r_12;
    }
    static mut id_set_6: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut r_13: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"lambda expression\0" as *const u8 as *const libc::c_char,
        id_set_6.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_13 != 1 as libc::c_int as *mut ast_t {
        return r_13;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_6: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_14: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"reference capability\0" as *const u8 as *const libc::c_char,
        rule_set_6.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_14 != 1 as libc::c_int as *mut ast_t {
        return r_14;
    }
    ast_setflag(
        ast_childidx(state.ast, 2 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 3 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 5 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 7 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "412:1"]
unsafe extern "C" fn barelambda(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"barelambda\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_BARELAMBDA, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_AT_LBRACE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"receiver capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"function name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                lambdaparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameters\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 2] = unsafe {
        [
            Some(
                lambdacaptures
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_7: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"captures\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_7 != 1 as libc::c_int as *mut ast_t {
        return r_7;
    }
    static mut id_set_3: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_8: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    if found {
        static mut rule_set_4: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_9: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"return type\0" as *const u8 as *const libc::c_char,
            rule_set_4.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_9 != 1 as libc::c_int as *mut ast_t {
            return r_9;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_4: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_10: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_4.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_10 != 1 as libc::c_int as *mut ast_t {
        return r_10;
    }
    static mut id_set_5: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    let mut r_11: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_5.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_11 != 1 as libc::c_int as *mut ast_t {
        return r_11;
    }
    static mut rule_set_5: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_12: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"lambda body\0" as *const u8 as *const libc::c_char,
        rule_set_5.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_12 != 1 as libc::c_int as *mut ast_t {
        return r_12;
    }
    static mut id_set_6: [token_id; 2] = [TK_RBRACE, TK_NONE];
    let mut r_13: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"lambda expression\0" as *const u8 as *const libc::c_char,
        id_set_6.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_13 != 1 as libc::c_int as *mut ast_t {
        return r_13;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_6: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_14: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"reference capability\0" as *const u8 as *const libc::c_char,
        rule_set_6.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_14 != 1 as libc::c_int as *mut ast_t {
        return r_14;
    }
    ast_setflag(
        ast_childidx(state.ast, 2 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 3 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 5 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    ast_setflag(
        ast_childidx(state.ast, 7 as libc::c_int as size_t),
        AST_FLAG_PRESERVE as libc::c_int as uint32_t,
    );
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "437:1"]
unsafe extern "C" fn arraytype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"arraytype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_AS, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn array(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"array\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_ARRAY, 0 as *mut token_t);
    static mut id_set: [token_id; 3] = [TK_LSQUARE, TK_LSQUARE_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                arraytype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"element type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"array elements\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"array literal\0" as *const u8 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "455:1"]
unsafe extern "C" fn nextarray(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextarray\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_ARRAY, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_LSQUARE_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                arraytype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"element type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"array elements\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RSQUARE, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"array literal\0" as *const u8 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "465:1"]
unsafe extern "C" fn tuple(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"tuple\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_COMMA as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_TUPLE);
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    rawseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn groupedexpr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"groupedexpr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                tuple
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    ast_setflag(state.ast, AST_FLAG_IN_PARENS as libc::c_int as uint32_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "484:1"]
unsafe extern "C" fn nextgroupedexpr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextgroupedexpr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_LPAREN_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                tuple
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    ast_setflag(state.ast, AST_FLAG_IN_PARENS as libc::c_int as uint32_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "494:1"]
unsafe extern "C" fn thisliteral(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"thisliteral\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_THIS, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn ref_0(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"ref\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_REFERENCE, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "506:1"]
unsafe extern "C" fn location(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"location\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_LOCATION, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "513:1"]
unsafe extern "C" fn ffi(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"ffi\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_AT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_FFICALL);
    }
    static mut id_set_0: [token_id; 3] = [TK_ID, TK_STRING, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"ffi name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typeargs
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"return type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                positional
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"ffi arguments\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                named
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"ffi arguments\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"ffi arguments\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_3: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "529:1"]
unsafe extern "C" fn atom(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"atom\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 14] = unsafe {
        [
            Some(
                ref_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                thisliteral
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                literal
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                groupedexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                array
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                object
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                lambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                barelambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                ffi as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                location
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                cond as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                whileloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                forloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "537:1"]
unsafe extern "C" fn caseatom(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"caseatom\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 13] = unsafe {
        [
            Some(
                ref_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                thisliteral
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                literal
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                groupedexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                array
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                object
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                lambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                barelambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                ffi as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                location
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                whileloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                forloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "544:1"]
unsafe extern "C" fn nextatom(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextatom\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 14] = unsafe {
        [
            Some(
                ref_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                thisliteral
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                literal
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nextgroupedexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nextarray
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                object
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                lambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                barelambda
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                ffi as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                location
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                cond as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                whileloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                forloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "550:1"]
unsafe extern "C" fn dot(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"dot\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_DOT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"member name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "557:1"]
unsafe extern "C" fn tilde(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"tilde\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_TILDE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"method name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "564:1"]
unsafe extern "C" fn chain(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"chain\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_CHAIN, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"method name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "571:1"]
unsafe extern "C" fn qualify(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"qualify\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    add_deferrable_ast(parser, &mut state, TK_QUALIFY, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typeargs
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type arguments\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "578:1"]
unsafe extern "C" fn call(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"call\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    add_deferrable_ast(parser, &mut state, TK_CALL, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                positional
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"argument\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                named
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"argument\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"call arguments\0" as *const u8 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_1: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "589:1"]
unsafe extern "C" fn postfix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"postfix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                atom as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 6] = unsafe {
        [
            Some(
                dot as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                tilde
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                chain
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                qualify
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                call as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"postfix expression\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "595:1"]
unsafe extern "C" fn casepostfix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"casepostfix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                caseatom
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 6] = unsafe {
        [
            Some(
                dot as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                tilde
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                chain
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                qualify
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                call as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"postfix expression\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn nextpostfix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextpostfix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                nextatom
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 6] = unsafe {
        [
            Some(
                dot as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                tilde
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                chain
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                qualify
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                call as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"postfix expression\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "607:1"]
unsafe extern "C" fn local(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"local\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 5] = [TK_VAR, TK_LET, TK_EMBED, TK_MATCH_CAPTURE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"variable name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_1: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    if found {
        static mut rule_set: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"variable type\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "616:1"]
unsafe extern "C" fn prefix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"prefix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 8] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS,
        TK_MINUS_TILDE,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"prefix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_TILDE as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS_TILDE);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_TILDE_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS_TILDE);
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                parampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"expression\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "629:1"]
unsafe extern "C" fn caseprefix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"caseprefix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 8] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS,
        TK_MINUS_TILDE,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"prefix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_TILDE as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS_TILDE);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_TILDE_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS_TILDE);
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                caseparampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"expression\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "641:1"]
unsafe extern "C" fn nextprefix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextprefix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 6] = [
        TK_NOT,
        TK_ADDRESS,
        TK_MINUS_NEW,
        TK_MINUS_TILDE_NEW,
        TK_DIGESTOF,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"prefix\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS);
    }
    if ast_id(state.ast) as libc::c_uint == TK_MINUS_TILDE_NEW as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_UNARY_MINUS_TILDE);
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                parampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"expression\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "651:1"]
unsafe extern "C" fn parampattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"parampattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                prefix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                postfix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "656:1"]
unsafe extern "C" fn caseparampattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"caseparampattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                caseprefix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                casepostfix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "661:1"]
unsafe extern "C" fn nextparampattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextparampattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                nextprefix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nextpostfix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "666:1"]
unsafe extern "C" fn pattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"pattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                local
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                parampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "671:1"]
unsafe extern "C" fn casepattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"casepattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                local
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                caseparampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "676:1"]
unsafe extern "C" fn nextpattern(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextpattern\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                local
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nextparampattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"pattern\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "681:1"]
unsafe extern "C" fn idseqmulti(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"idseqmulti\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_TUPLE, 0 as *mut token_t);
    static mut id_set: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                idseq_in_seq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"variable name\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_0: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set_0.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    idseq_in_seq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_2: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"variable name\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_2 != 1 as libc::c_int as *mut ast_t {
            return r_2;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "691:1"]
unsafe extern "C" fn idseqsingle(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"idseqsingle\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_LET, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"variable name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    add_deferrable_ast(parser, &mut state, TK_NONE, 0 as *mut token_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "699:1"]
unsafe extern "C" fn idseq_in_seq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"idseq_in_seq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                idseqsingle
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                idseqmulti
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"variable name\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "705:1"]
unsafe extern "C" fn idseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"idseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                idseqsingle
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                idseqmulti
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"variable name\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn elseclause(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"elseclause\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_ELSE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotatedseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "717:1"]
unsafe extern "C" fn elseif(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"elseif\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_IF, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    static mut id_set: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"then value\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_2: [rule_t; 3] = unsafe {
        [
            Some(
                elseif
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "729:1"]
unsafe extern "C" fn cond(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"cond\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_IF, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"then value\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_2: [rule_t; 3] = unsafe {
        [
            Some(
                elseif
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_1: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"if expression\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "742:1"]
unsafe extern "C" fn elseifdef(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"elseifdef\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_IFDEF, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    static mut id_set: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                infix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_TEST_EXTRA, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_TEST_EXTRA),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    infix
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else condition\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"then value\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 3] = unsafe {
        [
            Some(
                elseifdef
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_6: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    static mut order: [size_t; 4] = [
        0 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    ];
    if ast_childcount(state.ast)
        == (::core::mem::size_of::<[size_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
    {
    } else {
        ponyint_assert_fail(
            b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0" as *const u8
                as *const libc::c_char,
            754 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"elseifdef\0")).as_ptr(),
        );
    };
    static mut shuffle: [*mut ast_t; 4] = [0 as *const ast_t as *mut ast_t; 4];
    ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
    state.last_child = 0 as *mut ast_t;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "759:1"]
unsafe extern "C" fn ifdef(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"ifdef\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_IFDEF, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                infix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_TEST_EXTRA, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_TEST_EXTRA),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    infix
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else condition\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"then value\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_3: [rule_t; 3] = unsafe {
        [
            Some(
                elseifdef
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_6: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    static mut id_set_2: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_7: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"ifdef expression\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_7 != 1 as libc::c_int as *mut ast_t {
        return r_7;
    }
    static mut order: [size_t; 4] = [
        0 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    ];
    if ast_childcount(state.ast)
        == (::core::mem::size_of::<[size_t; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
    {
    } else {
        ponyint_assert_fail(
            b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0" as *const u8
                as *const libc::c_char,
            772 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"ifdef\0")).as_ptr(),
        );
    };
    static mut shuffle: [*mut ast_t; 4] = [0 as *const ast_t as *mut ast_t; 4];
    ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
    state.last_child = 0 as *mut ast_t;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "776:1"]
unsafe extern "C" fn iftype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"iftype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_IFTYPE, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_SUBTYPE, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_THEN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"then value\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    add_deferrable_ast(parser, &mut state, TK_NONE, 0 as *mut token_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "788:1"]
unsafe extern "C" fn elseiftype(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"elseiftype\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_IFTYPE_SET, 0 as *mut token_t);
    static mut id_set: [token_id; 2] = [TK_ELSEIF, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                iftype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"iftype clause\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 3] = unsafe {
        [
            Some(
                elseiftype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "798:1"]
unsafe extern "C" fn iftypeset(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"iftypeset\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_IFTYPE_SET, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.scope = 1 as libc::c_int != 0;
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                iftype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"iftype clause\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 3] = unsafe {
        [
            Some(
                elseiftype
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                elseclause
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"else clause\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_0: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"iftype expression\0" as *const u8 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "809:1"]
unsafe extern "C" fn caseexpr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"caseexpr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_CASE, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    static mut id_set: [token_id; 2] = [TK_PIPE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                casepattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"case pattern\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_IF, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_IF),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    rawseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"guard expression\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_DBLARROW),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    if found_0 {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    rawseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_5: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"case body\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_5 != 1 as libc::c_int as *mut ast_t {
            return r_5;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "820:1"]
unsafe extern "C" fn cases(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"cases\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_CASES, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                caseexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"cases\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r != 1 as libc::c_int as *mut ast_t {
            return r;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "828:1"]
unsafe extern "C" fn match_0(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"match\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_MATCH, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"match expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                cases
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"cases\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_0: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_4: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else clause\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_4 != 1 as libc::c_int as *mut ast_t {
            return r_4;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"match expression\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "840:1"]
unsafe extern "C" fn whileloop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"whileloop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_WHILE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_DO, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"while body\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_5: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else clause\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_5 != 1 as libc::c_int as *mut ast_t {
            return r_5;
        }
    }
    static mut id_set_2: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"while loop\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn repeat(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"repeat\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_REPEAT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"repeat body\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_UNTIL, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                annotatedseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"condition expression\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_5: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else clause\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_5 != 1 as libc::c_int as *mut ast_t {
            return r_5;
        }
    }
    static mut id_set_2: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"repeat loop\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "872:1"]
unsafe extern "C" fn forloop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"forloop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_FOR, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                idseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"iterator name\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_IN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"iterator\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 2] = [TK_DO, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"for body\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_2: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    if found {
        static mut rule_set_3: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_7: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"else clause\0" as *const u8 as *const libc::c_char,
            rule_set_3.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_7 != 1 as libc::c_int as *mut ast_t {
            return r_7;
        }
    }
    static mut id_set_3: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_8: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"for loop\0" as *const u8 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "886:1"]
unsafe extern "C" fn withelem(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"withelem\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                idseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"with name\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"initialiser\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "894:1"]
unsafe extern "C" fn withexpr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"withexpr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                withelem
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"with expression\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set: [token_id; 2] = [TK_COMMA, TK_NONE];
    let mut found: bool = 1 as libc::c_int != 0;
    loop {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_token_set(
            parser,
            &mut state,
            token_id_desc(TK_COMMA),
            0 as *const libc::c_char,
            id_set.as_ptr(),
            0 as libc::c_int != 0,
            &mut found,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
        if !found {
            break;
        }
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    withelem
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"with expression\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "902:1"]
unsafe extern "C" fn with(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"with\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_WITH, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                withexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"with expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_DO, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"with body\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"with expression\0" as *const u8 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "913:1"]
unsafe extern "C" fn try_block(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"try_block\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_TRY, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"try body\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"try else body\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_THEN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_THEN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    if found_0 {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_5: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"try then body\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_5 != 1 as libc::c_int as *mut ast_t {
            return r_5;
        }
    }
    static mut id_set_2: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"try expression\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "924:1"]
unsafe extern "C" fn test_try_block(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_try_block\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_TEST_TRY_NO_CHECK, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    if ast_id(state.ast) as libc::c_uint == TK_TEST_TRY_NO_CHECK as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_TRY_NO_CHECK);
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"try body\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_ELSE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ELSE),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"try else body\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    static mut id_set_1: [token_id; 2] = [TK_THEN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_THEN),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    if found_0 {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    annotatedseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_5: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"try then body\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_5 != 1 as libc::c_int as *mut ast_t {
            return r_5;
        }
    }
    static mut id_set_2: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"try expression\0" as *const u8 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "936:1"]
unsafe extern "C" fn recover(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"recover\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_RECOVER, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                seq as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"recover body\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_0: [token_id; 2] = [TK_END, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        b"recover expression\0" as *const u8 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "946:1"]
unsafe extern "C" fn test_aliased(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_aliased\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_TEST_ALIASED, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_TEST_ALIASED as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_ALIASED);
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "953:1"]
unsafe extern "C" fn consume(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"consume\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_CONSUME, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"consume\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                test_aliased
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"expression\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "961:1"]
unsafe extern "C" fn test_prefix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_prefix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_IFDEFNOT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"expression\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "969:1"]
unsafe extern "C" fn test_noseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_noseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_TEST_NO_SEQ, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                infix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"sequence\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_1: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "979:1"]
unsafe extern "C" fn test_seq_scope(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_seq_scope\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_TEST_SEQ_SCOPE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_LPAREN, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"sequence\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_1: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.scope = 1 as libc::c_int != 0;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "990:1"]
unsafe extern "C" fn test_ifdef_flag(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_ifdef_flag\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_IFDEFFLAG, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "998:1"]
unsafe extern "C" fn term(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"term\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 19] = unsafe {
        [
            Some(
                cond as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                ifdef
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                iftypeset
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                match_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                whileloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                repeat
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                forloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                with as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                try_block
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                recover
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                consume
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                pattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                const_expr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_noseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_seq_scope
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_try_block
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_ifdef_flag
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_prefix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1006:1"]
unsafe extern "C" fn nextterm(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextterm\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 19] = unsafe {
        [
            Some(
                cond as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                ifdef
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                iftypeset
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                match_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                whileloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                repeat
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                forloop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                with as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                try_block
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                recover
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                consume
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nextpattern
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                const_expr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_noseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_seq_scope
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_try_block
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_ifdef_flag
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                test_prefix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1022:1"]
unsafe extern "C" fn asop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"asop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_AS, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"as\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1030:1"]
unsafe extern "C" fn binop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"binop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 32] = [
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
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"binary operator\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut order: [size_t; 2] = [1 as libc::c_int as size_t, 0 as libc::c_int as size_t];
    if ast_childcount(state.ast)
        == (::core::mem::size_of::<[size_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
    {
    } else {
        ponyint_assert_fail(
            b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0" as *const u8
                as *const libc::c_char,
            1043 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"binop\0")).as_ptr(),
        );
    };
    static mut shuffle: [*mut ast_t; 2] = [0 as *const ast_t as *mut ast_t; 2];
    ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
    state.last_child = 0 as *mut ast_t;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1047:1"]
unsafe extern "C" fn isop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"isop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 3] = [TK_IS, TK_ISNT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"binary operator\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1055:1"]
unsafe extern "C" fn test_binop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"test_binop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 3] = [TK_IFDEFAND, TK_IFDEFOR, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"binary operator\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1062:1"]
unsafe extern "C" fn infix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"infix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                term as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 5] = unsafe {
        [
            Some(
                binop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                isop as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                asop as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                test_binop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1068:1"]
unsafe extern "C" fn nextinfix(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextinfix\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                nextterm
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 5] = unsafe {
        [
            Some(
                binop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                isop as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                asop as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                test_binop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1074:1"]
unsafe extern "C" fn assignop(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"assignop\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    *out_builder = Some(infix_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ());
    static mut id_set: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"assign operator\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                assignment
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"assign rhs\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1082:1"]
unsafe extern "C" fn assignment(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"assignment\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                infix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                assignop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1088:1"]
unsafe extern "C" fn nextassignment(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextassignment\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                nextinfix
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                assignop
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1094:1"]
unsafe extern "C" fn jump(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"jump\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 7] = [
        TK_RETURN,
        TK_BREAK,
        TK_CONTINUE,
        TK_ERROR,
        TK_COMPILE_INTRINSIC,
        TK_COMPILE_ERROR,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"statement\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"return value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1101:1"]
unsafe extern "C" fn semi(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"semi\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    state.deflt_id = TK_EOF;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_NEWLINE),
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if found {
        parse_set_next_flags(parser, AST_FLAG_BAD_SEMI as libc::c_int as uint32_t);
    } else {
        parse_set_next_flags(parser, 0 as libc::c_int as uint32_t);
    }
    static mut id_set_0: [token_id; 2] = [TK_SEMI, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_1: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_NEWLINE),
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    if found_0 {
        ast_setflag(state.ast, AST_FLAG_BAD_SEMI as libc::c_int as uint32_t);
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1108:1"]
unsafe extern "C" fn semiexpr(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"semiexpr\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_FLATTEN, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                semi as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"semicolon\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                exprseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                jump as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1115:1"]
unsafe extern "C" fn nosemi(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nosemi\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_NEWLINE, TK_NONE];
    state.deflt_id = TK_EOF;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_NEWLINE),
        0 as *const libc::c_char,
        id_set.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if found {
        parse_set_next_flags(parser, 0 as libc::c_int as uint32_t);
    } else {
        parse_set_next_flags(parser, AST_FLAG_MISSING_SEMI as libc::c_int as uint32_t);
    }
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                nextexprseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                jump as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1121:1"]
unsafe extern "C" fn nextexprseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"nextexprseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_FLATTEN, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                nextassignment
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                semiexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nosemi
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    parse_set_next_flags(parser, 0 as libc::c_int as uint32_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1129:1"]
unsafe extern "C" fn exprseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"exprseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_FLATTEN, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                assignment
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                semiexpr
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                nosemi
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    parse_set_next_flags(parser, 0 as libc::c_int as uint32_t);
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1137:1"]
unsafe extern "C" fn rawseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"rawseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    static mut rule_set: [rule_t; 3] = unsafe {
        [
            Some(
                exprseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                jump as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1143:1"]
unsafe extern "C" fn seq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"seq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                rawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.scope = 1 as libc::c_int != 0;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1149:1"]
unsafe extern "C" fn annotatedrawseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"annotatedrawseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_SEQ, 0 as *mut token_t);
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                exprseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                jump as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1156:1"]
unsafe extern "C" fn annotatedseq(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"annotatedseq\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotatedrawseq
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"value\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.scope = 1 as libc::c_int != 0;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1163:1"]
unsafe extern "C" fn method(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"method\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 4] = [TK_FUN, TK_BE, TK_NEW, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            Some(
                bare as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"method name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_2: [rule_t; 2] = unsafe {
        [
            Some(
                params
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_5: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"parameters\0" as *const u8 as *const libc::c_char,
        rule_set_2.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_6: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_6 != 1 as libc::c_int as *mut ast_t {
        return r_6;
    }
    static mut id_set_3: [token_id; 2] = [TK_COLON, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_7: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_COLON),
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_7 != 1 as libc::c_int as *mut ast_t {
        return r_7;
    }
    if found {
        static mut rule_set_3: [rule_t; 2] = unsafe {
            [
                Some(
                    type_0
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_8: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"return type\0" as *const u8 as *const libc::c_char,
            rule_set_3.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_8 != 1 as libc::c_int as *mut ast_t {
            return r_8;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_4: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_9: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_4.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_9 != 1 as libc::c_int as *mut ast_t {
        return r_9;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_5: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut r_10: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_5.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_10 != 1 as libc::c_int as *mut ast_t {
        return r_10;
    }
    static mut id_set_6: [token_id; 2] = [TK_DBLARROW, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found_0: bool = 0 as libc::c_int != 0;
    let mut r_11: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_DBLARROW),
        0 as *const libc::c_char,
        id_set_6.as_ptr(),
        0 as libc::c_int != 0,
        &mut found_0,
    );
    if r_11 != 1 as libc::c_int as *mut ast_t {
        return r_11;
    }
    if found_0 {
        static mut rule_set_4: [rule_t; 2] = unsafe {
            [
                Some(
                    rawseq
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_12: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"method body\0" as *const u8 as *const libc::c_char,
            rule_set_4.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_12 != 1 as libc::c_int as *mut ast_t {
            return r_12;
        }
    }
    static mut order: [size_t; 8] = [
        0 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        5 as libc::c_int as size_t,
        7 as libc::c_int as size_t,
        6 as libc::c_int as size_t,
    ];
    if ast_childcount(state.ast)
        == (::core::mem::size_of::<[size_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
    {
    } else {
        ponyint_assert_fail(
            b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0" as *const u8
                as *const libc::c_char,
            1179 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"method\0")).as_ptr(),
        );
    };
    static mut shuffle: [*mut ast_t; 8] = [0 as *const ast_t as *mut ast_t; 8];
    ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
    state.last_child = 0 as *mut ast_t;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1183:1"]
unsafe extern "C" fn field(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"field\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 4] = [TK_VAR, TK_LET, TK_EMBED, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_VAR as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_FVAR);
    }
    if ast_id(state.ast) as libc::c_uint == TK_LET as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_FLET);
    }
    static mut id_set_0: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"field name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut id_set_1: [token_id; 2] = [TK_COLON, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"mandatory type declaration on field\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                type_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"field type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_2: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_ASSIGN),
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    if found {
        static mut rule_set_0: [rule_t; 2] = unsafe {
            [
                Some(
                    infix
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_4: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"field value\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_4 != 1 as libc::c_int as *mut ast_t {
            return r_4;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_3: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"docstring\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1195:1"]
unsafe extern "C" fn members(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"members\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_MEMBERS, 0 as *mut token_t);
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                field
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"field\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r != 1 as libc::c_int as *mut ast_t {
            return r;
        }
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                method
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found_0: bool = 1 as libc::c_int != 0;
    while found_0 {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"method\0" as *const u8 as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found_0,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1203:1"]
unsafe extern "C" fn class_def(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"class_def\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut restart_set: [token_id; 9] = [
        TK_TYPE,
        TK_INTERFACE,
        TK_TRAIT,
        TK_PRIMITIVE,
        TK_STRUCT,
        TK_CLASS,
        TK_ACTOR,
        TK_EOF,
        TK_NONE,
    ];
    state.restart = restart_set.as_mut_ptr();
    static mut id_set: [token_id; 8] = [
        TK_TYPE,
        TK_INTERFACE,
        TK_TRAIT,
        TK_PRIMITIVE,
        TK_STRUCT,
        TK_CLASS,
        TK_ACTOR,
        TK_NONE,
    ];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"entity\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_EOF;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                annotations
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"annotations\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        1 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    state.scope = 1 as libc::c_int != 0;
    state.deflt_id = TK_NONE;
    static mut id_set_0: [token_id; 2] = [TK_AT, TK_NONE];
    let mut r_1: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                cap as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_2: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"capability\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    static mut id_set_1: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r_3: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_1: [rule_t; 2] = unsafe {
        [
            Some(
                typeparams
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_4: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"type parameters\0" as *const u8 as *const libc::c_char,
        rule_set_1.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    static mut id_set_2: [token_id; 2] = [TK_IS, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_IS),
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    if found {
        static mut rule_set_2: [rule_t; 2] = unsafe {
            [
                Some(
                    provides
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_6: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"provided type\0" as *const u8 as *const libc::c_char,
            rule_set_2.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_6 != 1 as libc::c_int as *mut ast_t {
            return r_6;
        }
    }
    state.deflt_id = TK_NONE;
    static mut id_set_3: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut r_7: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"docstring\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_7 != 1 as libc::c_int as *mut ast_t {
        return r_7;
    }
    static mut rule_set_3: [rule_t; 2] = unsafe {
        [
            Some(
                members
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_8: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"members\0" as *const u8 as *const libc::c_char,
        rule_set_3.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_8 != 1 as libc::c_int as *mut ast_t {
        return r_8;
    }
    static mut order: [size_t; 7] = [
        2 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        6 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        5 as libc::c_int as size_t,
    ];
    if ast_childcount(state.ast)
        == (::core::mem::size_of::<[size_t; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
    {
    } else {
        ponyint_assert_fail(
            b"ast_childcount(state.ast) == (sizeof(order) / sizeof(size_t))\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parser.c\0" as *const u8
                as *const libc::c_char,
            1219 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"class_def\0")).as_ptr(),
        );
    };
    static mut shuffle: [*mut ast_t; 7] = [0 as *const ast_t as *mut ast_t; 7];
    ast_reorder_children(state.ast, order.as_ptr(), shuffle.as_mut_ptr());
    state.last_child = 0 as *mut ast_t;
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1223:1"]
unsafe extern "C" fn use_uri(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"use_uri\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1229:1"]
unsafe extern "C" fn use_ffi(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"use_ffi\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_AT, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    if ast_id(state.ast) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        ast_setid(state.ast, TK_FFIDECL);
    }
    state.scope = 1 as libc::c_int != 0;
    static mut id_set_0: [token_id; 3] = [TK_ID, TK_STRING, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"ffi name\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                typeargs
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"return type\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_1: [token_id; 3] = [TK_LPAREN, TK_LPAREN_NEW, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_1.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                params
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_3: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"ffi parameters\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_3 != 1 as libc::c_int as *mut ast_t {
        return r_3;
    }
    add_deferrable_ast(parser, &mut state, TK_NONE, 0 as *mut token_t);
    static mut id_set_2: [token_id; 2] = [TK_RPAREN, TK_NONE];
    let mut r_4: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_2.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_4 != 1 as libc::c_int as *mut ast_t {
        return r_4;
    }
    state.deflt_id = TK_NONE;
    static mut id_set_3: [token_id; 2] = [TK_QUESTION, TK_NONE];
    let mut r_5: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_3.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_5 != 1 as libc::c_int as *mut ast_t {
        return r_5;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1243:1"]
unsafe extern "C" fn use_name(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"use_name\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    static mut id_set: [token_id; 2] = [TK_ID, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut id_set_0: [token_id; 2] = [TK_ASSIGN, TK_NONE];
    let mut r_0: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1250:1"]
unsafe extern "C" fn use_0(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"use\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut restart_set: [token_id; 10] = [
        TK_USE,
        TK_TYPE,
        TK_INTERFACE,
        TK_TRAIT,
        TK_PRIMITIVE,
        TK_STRUCT,
        TK_CLASS,
        TK_ACTOR,
        TK_EOF,
        TK_NONE,
    ];
    state.restart = restart_set.as_mut_ptr();
    static mut id_set: [token_id; 2] = [TK_USE, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    state.deflt_id = TK_NONE;
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                use_name
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_0: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"name\0" as *const u8 as *const libc::c_char,
        rule_set.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_0 != 1 as libc::c_int as *mut ast_t {
        return r_0;
    }
    static mut rule_set_0: [rule_t; 3] = unsafe {
        [
            Some(
                use_uri
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            Some(
                use_ffi
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut r_1: *mut ast_t = parse_rule_set(
        parser,
        &mut state,
        b"specifier\0" as *const u8 as *const libc::c_char,
        rule_set_0.as_ptr(),
        0 as *mut bool,
        0 as libc::c_int != 0,
    );
    if r_1 != 1 as libc::c_int as *mut ast_t {
        return r_1;
    }
    static mut id_set_0: [token_id; 2] = [TK_IF, TK_NONE];
    state.deflt_id = TK_NONE;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        token_id_desc(TK_IF),
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        &mut found,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    if found {
        static mut rule_set_1: [rule_t; 2] = unsafe {
            [
                Some(
                    infix
                        as unsafe extern "C" fn(
                            *mut parser_t,
                            *mut builder_fn_t,
                            *const libc::c_char,
                        ) -> *mut ast_t,
                ),
                None,
            ]
        };
        let mut r_3: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"use condition\0" as *const u8 as *const libc::c_char,
            rule_set_1.as_ptr(),
            0 as *mut bool,
            0 as libc::c_int != 0,
        );
        if r_3 != 1 as libc::c_int as *mut ast_t {
            return r_3;
        }
    }
    return parse_rule_complete(parser, &mut state);
}
#[c2rust::src_loc = "1260:1"]
unsafe extern "C" fn module(
    mut parser: *mut parser_t,
    mut out_builder: *mut builder_fn_t,
    mut rule_desc: *const libc::c_char,
) -> *mut ast_t {
    let mut state: rule_state_t = {
        let mut init = rule_state_t {
            fn_name: b"module\0" as *const u8 as *const libc::c_char,
            ast: 0 as *mut ast_t,
            last_child: 0 as *mut ast_t,
            desc: rule_desc,
            restart: 0 as *mut token_id,
            deflt_id: TK_LEX_ERROR,
            matched: 0 as libc::c_int != 0,
            scope: 0 as libc::c_int != 0,
            deferred: 0 as libc::c_int != 0,
            deferred_id: TK_NONE,
            line: 0 as libc::c_int as size_t,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    add_deferrable_ast(parser, &mut state, TK_MODULE, 0 as *mut token_t);
    state.scope = 1 as libc::c_int != 0;
    state.deflt_id = TK_EOF;
    static mut id_set: [token_id; 2] = [TK_STRING, TK_NONE];
    let mut r: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"package docstring\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        id_set.as_ptr(),
        1 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r != 1 as libc::c_int as *mut ast_t {
        return r;
    }
    static mut rule_set: [rule_t; 2] = unsafe {
        [
            Some(
                use_0
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found: bool = 1 as libc::c_int != 0;
    while found {
        state.deflt_id = TK_EOF;
        let mut r_0: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"use command\0" as *const u8 as *const libc::c_char,
            rule_set.as_ptr(),
            &mut found,
            0 as libc::c_int != 0,
        );
        if r_0 != 1 as libc::c_int as *mut ast_t {
            return r_0;
        }
    }
    static mut rule_set_0: [rule_t; 2] = unsafe {
        [
            Some(
                class_def
                    as unsafe extern "C" fn(
                        *mut parser_t,
                        *mut builder_fn_t,
                        *const libc::c_char,
                    ) -> *mut ast_t,
            ),
            None,
        ]
    };
    let mut found_0: bool = 1 as libc::c_int != 0;
    while found_0 {
        state.deflt_id = TK_EOF;
        let mut r_1: *mut ast_t = parse_rule_set(
            parser,
            &mut state,
            b"type, interface, trait, primitive, class or actor definition\0" as *const u8
                as *const libc::c_char,
            rule_set_0.as_ptr(),
            &mut found_0,
            0 as libc::c_int != 0,
        );
        if r_1 != 1 as libc::c_int as *mut ast_t {
            return r_1;
        }
    }
    static mut id_set_0: [token_id; 2] = [TK_EOF, TK_NONE];
    let mut r_2: *mut ast_t = parse_token_set(
        parser,
        &mut state,
        b"type, interface, trait, primitive, class, actor, member or method\0" as *const u8
            as *const libc::c_char,
        0 as *const libc::c_char,
        id_set_0.as_ptr(),
        0 as libc::c_int != 0,
        0 as *mut bool,
    );
    if r_2 != 1 as libc::c_int as *mut ast_t {
        return r_2;
    }
    return parse_rule_complete(parser, &mut state);
}
#[no_mangle]
#[c2rust::src_loc = "1275:1"]
pub unsafe extern "C" fn pass_parse(
    mut package: *mut ast_t,
    mut source: *mut source_t,
    mut errors: *mut errors_t,
    mut allow_test_symbols: bool,
    mut trace: bool,
) -> bool {
    return parse(
        package,
        source,
        Some(
            module
                as unsafe extern "C" fn(
                    *mut parser_t,
                    *mut builder_fn_t,
                    *const libc::c_char,
                ) -> *mut ast_t,
        ),
        b"class, actor, primitive or trait\0" as *const u8 as *const libc::c_char,
        errors,
        allow_test_symbols,
        trace,
    );
}
