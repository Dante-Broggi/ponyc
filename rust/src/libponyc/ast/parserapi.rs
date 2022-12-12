use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = __int64_t;
    use super::_types_h::__int64_t;
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
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn source_close(source: *mut source_t);
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
    use super::_size_t_h::size_t;
    use super::source_h::source_t;
    extern "C" {
        #[c2rust::src_loc = "16:16"]
        pub type token_t;
        #[c2rust::src_loc = "282:1"]
        pub fn token_new(id: token_id) -> *mut token_t;
        #[c2rust::src_loc = "298:1"]
        pub fn token_free(token: *mut token_t);
        #[c2rust::src_loc = "317:1"]
        pub fn token_get_id(token: *mut token_t) -> token_id;
        #[c2rust::src_loc = "341:1"]
        pub fn token_print(token: *mut token_t) -> *const libc::c_char;
        #[c2rust::src_loc = "351:1"]
        pub fn token_id_desc(id: token_id) -> *const libc::c_char;
        #[c2rust::src_loc = "354:1"]
        pub fn token_source(token: *mut token_t) -> *mut source_t;
        #[c2rust::src_loc = "357:1"]
        pub fn token_line_number(token: *mut token_t) -> size_t;
        #[c2rust::src_loc = "360:1"]
        pub fn token_line_position(token: *mut token_t) -> size_t;
        #[c2rust::src_loc = "391:1"]
        pub fn token_set_pos(token: *mut token_t, source: *mut source_t, line: size_t, pos: size_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.h:1"]
pub mod lexer_h {
    use super::source_h::source_t;
    use super::token_h::{token_id, token_t};
    extern "C" {
        #[c2rust::src_loc = "11:16"]
        pub type lexer_t;
        #[c2rust::src_loc = "12:16"]
        pub type errors_t;
        #[c2rust::src_loc = "18:1"]
        pub fn lexer_open(
            source: *mut source_t,
            errors: *mut errors_t,
            allow_test_symbols: bool,
        ) -> *mut lexer_t;
        #[c2rust::src_loc = "24:1"]
        pub fn lexer_close(lexer: *mut lexer_t);
        #[c2rust::src_loc = "29:1"]
        pub fn lexer_next(lexer: *mut lexer_t) -> *mut token_t;
        #[c2rust::src_loc = "37:1"]
        pub fn lexer_print(id: token_id) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:1"]
pub mod _stdio_h {
    #[c2rust::src_loc = "81:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct __sbuf {
        pub _base: *mut libc::c_uchar,
        pub _size: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:16"]
    pub struct __sFILE {
        pub _p: *mut libc::c_uchar,
        pub _r: libc::c_int,
        pub _w: libc::c_int,
        pub _flags: libc::c_short,
        pub _file: libc::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: libc::c_int,
        pub _cookie: *mut libc::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: libc::c_int,
        pub _ubuf: [libc::c_uchar; 3],
        pub _nbuf: [libc::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: libc::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "126:1"]
    pub type FILE = __sFILE;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "98:8"]
        pub type __sFILEX;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
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
    use super::parser_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::_uint32_t_h::uint32_t;
    use super::lexer_h::errors_t;
    use super::symtab_h::ast_t;
    use super::token_h::{token_id, token_t};
    extern "C" {
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ast_pop(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "137:1"]
        pub fn ast_add_sibling(older_sibling: *mut ast_t, new_sibling: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "136:1"]
        pub fn ast_add(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "103:1"]
        pub fn ast_setannotation(ast: *mut ast_t, annotation: *mut ast_t);
        #[c2rust::src_loc = "89:1"]
        pub fn ast_setflag(ast: *mut ast_t, flag: uint32_t);
        #[c2rust::src_loc = "56:1"]
        pub fn ast_new(t: *mut token_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "58:1"]
        pub fn ast_token(t: *mut token_t) -> *mut ast_t;
        #[c2rust::src_loc = "66:1"]
        pub fn ast_scope(ast: *mut ast_t);
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ast_setdata(ast: *mut ast_t, data: *mut libc::c_void) -> *mut ast_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    use super::_size_t_h::size_t;
    use super::lexer_h::errors_t;
    use super::source_h::source_t;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn errors_get_count(errors: *mut errors_t) -> size_t;
        #[c2rust::src_loc = "79:1"]
        pub fn error(
            errors: *mut errors_t,
            source: *mut source_t,
            line: size_t,
            pos: size_t,
            fmt: *const libc::c_char,
            _: ...
        );
        #[c2rust::src_loc = "87:1"]
        pub fn error_continue(
            errors: *mut errors_t,
            source: *mut source_t,
            line: size_t,
            pos: size_t,
            fmt: *const libc::c_char,
            _: ...
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:2"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::_types_h::{__darwin_size_t, __int64_t};
pub use self::_uint32_t_h::uint32_t;
use self::ast_h::{
    ast_add, ast_add_sibling, ast_append, ast_data, ast_error, ast_free, ast_id, ast_new, ast_pop,
    ast_scope, ast_setannotation, ast_setdata, ast_setflag, ast_token,
};
use self::error_h::{error, error_continue, errors_get_count};
use self::lexer_h::{errors_t, lexer_close, lexer_next, lexer_open, lexer_print, lexer_t};
pub use self::parserapi_h::{builder_fn_t, rule_state_t, rule_t};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc, ponyint_pool_free};
pub use self::source_h::{source_close, source_t};
use self::stdio_h::{__stderrp, fprintf};
use self::symtab_h::ast_t;
pub use self::sys__types_h::__darwin_off_t;
pub use self::token_h::{
    token_free, token_get_id, token_id, token_id_desc, token_line_number, token_line_position,
    token_new, token_print, token_set_pos, token_source, token_t, TK_ACTOR, TK_ADDRESS, TK_ALIASED,
    TK_AND, TK_ANNOTATION, TK_ARRAY, TK_ARROW, TK_AS, TK_ASSIGN, TK_AT, TK_AT_LBRACE, TK_BACKSLASH,
    TK_BARELAMBDA, TK_BARELAMBDATYPE, TK_BE, TK_BEAPP, TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK,
    TK_CALL, TK_CAP_ALIAS, TK_CAP_ANY, TK_CAP_READ, TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES,
    TK_CHAIN, TK_CLASS, TK_COLON, TK_COMMA, TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT,
    TK_CONSUME, TK_CONTINUE, TK_DBLARROW, TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE,
    TK_DIVIDE_TILDE, TK_DO, TK_DONTCARE, TK_DONTCAREREF, TK_DONTCARETYPE, TK_DOT, TK_ELLIPSIS,
    TK_ELSE, TK_ELSEIF, TK_EMBED, TK_EMBEDREF, TK_END, TK_EOF, TK_EPHEMERAL, TK_EQ, TK_EQ_TILDE,
    TK_ERROR, TK_ERRORTYPE, TK_FALSE, TK_FFICALL, TK_FFIDECL, TK_FLATTEN, TK_FLET, TK_FLETREF,
    TK_FLOAT, TK_FOR, TK_FUN, TK_FUNAPP, TK_FUNCHAIN, TK_FUNREF, TK_FUNTYPE, TK_FVAR, TK_FVARREF,
    TK_GE, TK_GE_TILDE, TK_GT, TK_GT_TILDE, TK_ID, TK_IF, TK_IFDEF, TK_IFDEFAND, TK_IFDEFFLAG,
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "8:8"]
pub struct parser_t {
    pub source: *mut source_t,
    pub lexer: *mut lexer_t,
    pub token: *mut token_t,
    pub last_token: *mut token_t,
    pub last_matched: *const libc::c_char,
    pub next_flags: uint32_t,
    pub free_last_token: bool,
    pub failed: bool,
    pub errors: *mut errors_t,
    pub trace_enable: bool,
}
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn current_token_id(mut parser: *mut parser_t) -> token_id {
    token_get_id((*parser).token)
}
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn fetch_next_lexer_token(mut parser: *mut parser_t, mut free_last_token: bool) {
    let mut old_token: *mut token_t = (*parser).token;
    let mut new_token: *mut token_t = lexer_next((*parser).lexer);
    if !old_token.is_null()
        && token_get_id(new_token) as libc::c_uint == TK_EOF as libc::c_int as libc::c_uint
    {
        token_set_pos(
            new_token,
            token_source(old_token),
            token_line_number(old_token),
            token_line_position(old_token),
        );
    }
    if !old_token.is_null() {
        if (*parser).free_last_token {
            token_free((*parser).last_token);
        }
        (*parser).free_last_token = free_last_token;
        let ref mut fresh0 = (*parser).last_token;
        *fresh0 = old_token;
    }
    let ref mut fresh1 = (*parser).token;
    *fresh1 = new_token;
}
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn consume_token(mut parser: *mut parser_t) -> *mut ast_t {
    let mut ast: *mut ast_t = ast_token((*parser).token);
    ast_setflag(ast, (*parser).next_flags);
    (*parser).next_flags = 0 as libc::c_int as uint32_t;
    fetch_next_lexer_token(parser, 0 as libc::c_int != 0);
    return ast;
}
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn consume_token_no_ast(mut parser: *mut parser_t) {
    fetch_next_lexer_token(parser, 1 as libc::c_int != 0);
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn syntax_error(
    mut parser: *mut parser_t,
    mut expected: *const libc::c_char,
    mut ast: *mut ast_t,
    mut terminating: *const libc::c_char,
) {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            73 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_error\0")).as_ptr(),
        );
    };
    if !expected.is_null() {
    } else {
        ponyint_assert_fail(
            b"expected != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            74 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_error\0")).as_ptr(),
        );
    };
    if !((*parser).token).is_null() {
    } else {
        ponyint_assert_fail(
            b"parser->token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            75 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_error\0")).as_ptr(),
        );
    };
    if ((*parser).last_matched).is_null() {
        error(
            (*parser).errors,
            (*parser).source,
            token_line_number((*parser).token),
            token_line_position((*parser).token),
            b"syntax error: no code found\0" as *const u8 as *const libc::c_char,
        );
    } else if terminating.is_null() {
        error(
            (*parser).errors,
            (*parser).source,
            token_line_number((*parser).token),
            token_line_position((*parser).token),
            b"syntax error: expected %s after %s\0" as *const u8 as *const libc::c_char,
            expected,
            (*parser).last_matched,
        );
    } else {
        if !ast.is_null() {
        } else {
            ponyint_assert_fail(
                b"ast != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0"
                    as *const u8 as *const libc::c_char,
                92 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"syntax_error\0"))
                    .as_ptr(),
            );
        };
        ast_error(
            (*parser).errors,
            ast,
            b"syntax error: unterminated %s\0" as *const u8 as *const libc::c_char,
            terminating,
        );
        error_continue(
            (*parser).errors,
            (*parser).source,
            token_line_number((*parser).token),
            token_line_position((*parser).token),
            b"expected terminating %s before here\0" as *const u8 as *const libc::c_char,
            expected,
        );
    };
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn default_builder(mut state: *mut rule_state_t, mut new_ast: *mut ast_t) {
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            108 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"default_builder\0"))
                .as_ptr(),
        );
    };
    if !new_ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            109 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"default_builder\0"))
                .as_ptr(),
        );
    };
    if ast_id(new_ast) as libc::c_uint == TK_FLATTEN as libc::c_int as libc::c_uint {
        let mut new_child: *mut ast_t = 0 as *mut ast_t;
        loop {
            new_child = ast_pop(new_ast);
            if new_child.is_null() {
                break;
            }
            default_builder(state, new_child);
        }
        ast_free(new_ast);
        return;
    }
    if ((*state).last_child).is_null() {
        ast_append((*state).ast, new_ast);
    } else {
        ast_add_sibling((*state).last_child, new_ast);
    }
    let ref mut fresh2 = (*state).last_child;
    *fresh2 = new_ast;
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn infix_builder(mut state: *mut rule_state_t, mut new_ast: *mut ast_t) {
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            136 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"infix_builder\0"))
                .as_ptr(),
        );
    };
    if !new_ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            137 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"infix_builder\0"))
                .as_ptr(),
        );
    };
    ast_add(new_ast, (*state).ast);
    let ref mut fresh3 = (*state).ast;
    *fresh3 = new_ast;
    let ref mut fresh4 = (*state).last_child;
    *fresh4 = 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn infix_reverse_builder(
    mut state: *mut rule_state_t,
    mut new_ast: *mut ast_t,
) {
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            148 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"infix_reverse_builder\0"))
                .as_ptr(),
        );
    };
    if !new_ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"infix_reverse_builder\0"))
                .as_ptr(),
        );
    };
    ast_append(new_ast, (*state).ast);
    let ref mut fresh5 = (*state).ast;
    *fresh5 = new_ast;
}
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn annotation_builder(mut state: *mut rule_state_t, mut new_ast: *mut ast_t) {
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            161 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"annotation_builder\0"))
                .as_ptr(),
        );
    };
    if !new_ast.is_null() {
    } else {
        ponyint_assert_fail(
            b"new_ast != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            162 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"annotation_builder\0"))
                .as_ptr(),
        );
    };
    ast_setannotation((*state).ast, new_ast);
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn process_deferred_ast(mut parser: *mut parser_t, mut state: *mut rule_state_t) {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            173 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"process_deferred_ast\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            174 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"process_deferred_ast\0"))
                .as_ptr(),
        );
    };
    if (*state).deferred {
        let mut deferred_token: *mut token_t = token_new((*state).deferred_id);
        token_set_pos(
            deferred_token,
            (*parser).source,
            (*state).line,
            (*state).pos,
        );
        let ref mut fresh6 = (*state).ast;
        *fresh6 = ast_token(deferred_token);
        (*state).deferred = 0 as libc::c_int != 0;
    }
}
#[c2rust::src_loc = "187:1"]
unsafe extern "C" fn add_ast(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut new_ast: *mut ast_t,
    mut build_fn: builder_fn_t,
) {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            190 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"add_ast\0")).as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            191 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"add_ast\0")).as_ptr(),
        );
    };
    if !new_ast.is_null() && new_ast != 2 as libc::c_int as *mut ast_t {
    } else {
        ponyint_assert_fail(
            b"new_ast != NULL && new_ast != PARSE_ERROR\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            192 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"add_ast\0")).as_ptr(),
        );
    };
    if build_fn.is_some() {
    } else {
        ponyint_assert_fail(
            b"build_fn != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            193 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"add_ast\0")).as_ptr(),
        );
    };
    process_deferred_ast(parser, state);
    if ((*state).ast).is_null() {
        let ref mut fresh7 = (*state).ast;
        *fresh7 = new_ast;
        let ref mut fresh8 = (*state).last_child;
        *fresh8 = 0 as *mut ast_t;
    } else {
        build_fn.expect("non-null function pointer")(state, new_ast);
    };
}
#[no_mangle]
#[c2rust::src_loc = "212:1"]
pub unsafe extern "C" fn add_deferrable_ast(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut id: token_id,
    mut token_for_pos: *mut token_t,
) {
    if token_for_pos.is_null() {
        token_for_pos = (*parser).token;
    }
    if !token_for_pos.is_null() {
    } else {
        ponyint_assert_fail(
            b"token_for_pos != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            218 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"add_deferrable_ast\0"))
                .as_ptr(),
        );
    };
    if !(*state).matched && ((*state).ast).is_null() && !(*state).deferred {
        (*state).deferred = 1 as libc::c_int != 0;
        (*state).deferred_id = id;
        (*state).line = token_line_number(token_for_pos);
        (*state).pos = token_line_position(token_for_pos);
        return;
    }
    add_ast(
        parser,
        state,
        ast_new(token_for_pos, id),
        Some(default_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ()),
    );
}
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn ditch_restart(mut parser: *mut parser_t, mut state: *mut rule_state_t) {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            238 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ditch_restart\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            239 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ditch_restart\0"))
                .as_ptr(),
        );
    };
    if !((*state).restart).is_null() {
    } else {
        ponyint_assert_fail(
            b"state->restart != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            240 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"ditch_restart\0"))
                .as_ptr(),
        );
    };
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Attempting recovery:\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
        );
    }
    loop {
        let mut id: token_id = current_token_id(parser);
        let mut p: *const token_id = (*state).restart;
        while *p as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
            if *p as libc::c_uint == id as libc::c_uint {
                if (*parser).trace_enable {
                    fprintf(
                        __stderrp,
                        b"  recovered with %s\n\0" as *const u8 as *const libc::c_char,
                        token_print((*parser).token),
                    );
                }
                return;
            }
            p = p.offset(1);
        }
        if (*parser).trace_enable {
            fprintf(
                __stderrp,
                b"  ignoring %d %s %s\n\0" as *const u8 as *const libc::c_char,
                id as libc::c_uint,
                lexer_print(id),
                token_print((*parser).token),
            );
        }
        consume_token_no_ast(parser);
    }
}
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn propogate_error(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            274 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"propogate_error\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            275 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"propogate_error\0"))
                .as_ptr(),
        );
    };
    ast_free((*state).ast);
    let ref mut fresh9 = (*state).ast;
    *fresh9 = 0 as *mut ast_t;
    (*parser).failed = 1 as libc::c_int != 0;
    if ((*state).restart).is_null() {
        if (*parser).trace_enable {
            fprintf(
                __stderrp,
                b"Rule %s: Propagate failure\n\0" as *const u8 as *const libc::c_char,
                (*state).fn_name,
            );
        }
        return 2 as libc::c_int as *mut ast_t;
    }
    ditch_restart(parser, state);
    return 0 as *mut ast_t;
}
#[c2rust::src_loc = "303:1"]
unsafe extern "C" fn handle_found(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut new_ast: *mut ast_t,
    mut build_fn: builder_fn_t,
    mut out_found: *mut bool,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            306 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"handle_found\0")).as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            307 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"handle_found\0")).as_ptr(),
        );
    };
    if !out_found.is_null() {
        *out_found = 1 as libc::c_int != 0;
    }
    if !(*state).matched {
        if (*parser).trace_enable {
            fprintf(
                __stderrp,
                b"Rule %s: Matched\n\0" as *const u8 as *const libc::c_char,
                (*state).fn_name,
            );
        }
        (*state).matched = 1 as libc::c_int != 0;
    }
    if !new_ast.is_null() {
        add_ast(parser, state, new_ast, build_fn);
    }
    (*state).deflt_id = TK_LEX_ERROR;
    return 1 as libc::c_int as *mut ast_t;
}
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn handle_not_found(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut desc: *const libc::c_char,
    mut terminating: *const libc::c_char,
    mut out_found: *mut bool,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            342 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"handle_not_found\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            343 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"handle_not_found\0"))
                .as_ptr(),
        );
    };
    if !desc.is_null() {
    } else {
        ponyint_assert_fail(
            b"desc != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            344 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"handle_not_found\0"))
                .as_ptr(),
        );
    };
    if !out_found.is_null() {
        *out_found = 0 as libc::c_int != 0;
    }
    if (*state).deflt_id as libc::c_uint != TK_LEX_ERROR as libc::c_int as libc::c_uint {
        if (*state).deflt_id as libc::c_uint != TK_EOF as libc::c_int as libc::c_uint {
            add_deferrable_ast(parser, state, (*state).deflt_id, (*parser).last_token);
        }
        (*state).deflt_id = TK_LEX_ERROR;
        return 1 as libc::c_int as *mut ast_t;
    }
    if !(*state).matched {
        if (*parser).trace_enable {
            fprintf(
                __stderrp,
                b"Rule %s: Not matched\n\0" as *const u8 as *const libc::c_char,
                (*state).fn_name,
            );
        }
        ast_free((*state).ast);
        let ref mut fresh10 = (*state).ast;
        *fresh10 = 0 as *mut ast_t;
        return 3 as libc::c_int as *mut ast_t;
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Error\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
        );
    }
    syntax_error(parser, desc, (*state).ast, terminating);
    (*parser).failed = 1 as libc::c_int != 0;
    ast_free((*state).ast);
    let ref mut fresh11 = (*state).ast;
    *fresh11 = 0 as *mut ast_t;
    if ((*state).restart).is_null() {
        return 2 as libc::c_int as *mut ast_t;
    }
    ditch_restart(parser, state);
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "406:1"]
pub unsafe extern "C" fn parse_token_set(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut desc: *const libc::c_char,
    mut terminating: *const libc::c_char,
    mut id_set: *const token_id,
    mut make_ast: bool,
    mut out_found: *mut bool,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            410 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"parse_token_set\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            411 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"parse_token_set\0"))
                .as_ptr(),
        );
    };
    if !id_set.is_null() {
    } else {
        ponyint_assert_fail(
            b"id_set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            412 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"parse_token_set\0"))
                .as_ptr(),
        );
    };
    let mut id: token_id = current_token_id(parser);
    if id as libc::c_uint == TK_LEX_ERROR as libc::c_int as libc::c_uint {
        return propogate_error(parser, state);
    }
    if desc.is_null() {
        desc = token_id_desc(*id_set.offset(0 as libc::c_int as isize));
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Looking for %s token%s %s. Found %s. \0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
            if (*state).deflt_id as libc::c_uint == TK_LEX_ERROR as libc::c_int as libc::c_uint {
                b"required\0" as *const u8 as *const libc::c_char
            } else {
                b"optional\0" as *const u8 as *const libc::c_char
            },
            if *id_set.offset(1 as libc::c_int as isize) as libc::c_uint
                == TK_NONE as libc::c_int as libc::c_uint
            {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            desc,
            token_print((*parser).token),
        );
    }
    let mut p: *const token_id = id_set;
    while *p as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        if *p as libc::c_uint == TK_NEWLINE as libc::c_int as libc::c_uint {
            if !((*parser).token).is_null() {
            } else {
                ponyint_assert_fail(
                    b"parser->token != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0"
                        as *const u8 as *const libc::c_char,
                    436 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"parse_token_set\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut last_token_line: size_t = token_line_number((*parser).last_token);
            let mut next_token_line: size_t = token_line_number((*parser).token);
            let mut is_newline: bool = next_token_line != last_token_line;
            if !out_found.is_null() {
                *out_found = is_newline;
            }
            if (*parser).trace_enable {
                fprintf(
                    __stderrp,
                    b"\\n %smatched\n\0" as *const u8 as *const libc::c_char,
                    if is_newline as libc::c_int != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            (*state).deflt_id = TK_LEX_ERROR;
            return 1 as libc::c_int as *mut ast_t;
        }
        if id as libc::c_uint == *p as libc::c_uint {
            if (*parser).trace_enable {
                fprintf(
                    __stderrp,
                    b"Compatible\n\0" as *const u8 as *const libc::c_char,
                );
            }
            let ref mut fresh12 = (*parser).last_matched;
            *fresh12 = token_print((*parser).token);
            if make_ast {
                return handle_found(
                    parser,
                    state,
                    consume_token(parser),
                    Some(
                        default_builder
                            as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> (),
                    ),
                    out_found,
                );
            }
            consume_token_no_ast(parser);
            return handle_found(parser, state, 0 as *mut ast_t, None, out_found);
        }
        p = p.offset(1);
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Not compatible\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return handle_not_found(parser, state, desc, terminating, out_found);
}
#[no_mangle]
#[c2rust::src_loc = "489:1"]
pub unsafe extern "C" fn parse_rule_set(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
    mut desc: *const libc::c_char,
    mut rule_set: *const rule_t,
    mut out_found: *mut bool,
    mut annotate: bool,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            492 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_rule_set\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            493 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_rule_set\0"))
                .as_ptr(),
        );
    };
    if !desc.is_null() {
    } else {
        ponyint_assert_fail(
            b"desc != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            494 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_rule_set\0"))
                .as_ptr(),
        );
    };
    if !rule_set.is_null() {
    } else {
        ponyint_assert_fail(
            b"rule_set != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            495 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_rule_set\0"))
                .as_ptr(),
        );
    };
    let mut id: token_id = current_token_id(parser);
    if id as libc::c_uint == TK_LEX_ERROR as libc::c_int as libc::c_uint {
        return propogate_error(parser, state);
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Looking for %s rule%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
            if (*state).deflt_id as libc::c_uint == TK_LEX_ERROR as libc::c_int as libc::c_uint {
                b"required\0" as *const u8 as *const libc::c_char
            } else {
                b"optional\0" as *const u8 as *const libc::c_char
            },
            if (*rule_set.offset(1 as libc::c_int as isize)).is_none() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            desc,
        );
    }
    let mut build_fn: builder_fn_t = if annotate as libc::c_int != 0 {
        Some(annotation_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ())
    } else {
        Some(default_builder as unsafe extern "C" fn(*mut rule_state_t, *mut ast_t) -> ())
    };
    let mut p: *const rule_t = rule_set;
    while (*p).is_some() {
        let mut rule_ast: *mut ast_t =
            (*p).expect("non-null function pointer")(parser, &mut build_fn, desc);
        if rule_ast == 2 as libc::c_int as *mut ast_t {
            return propogate_error(parser, state);
        }
        if rule_ast != 3 as libc::c_int as *mut ast_t {
            let ref mut fresh13 = (*parser).last_matched;
            *fresh13 = desc;
            return handle_found(parser, state, rule_ast, build_fn, out_found);
        }
        p = p.offset(1);
    }
    return handle_not_found(parser, state, desc, 0 as *const libc::c_char, out_found);
}
#[no_mangle]
#[c2rust::src_loc = "532:1"]
pub unsafe extern "C" fn parse_set_next_flags(mut parser: *mut parser_t, mut flags: uint32_t) {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            534 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"parse_set_next_flags\0"))
                .as_ptr(),
        );
    };
    (*parser).next_flags = flags;
}
#[no_mangle]
#[c2rust::src_loc = "548:1"]
pub unsafe extern "C" fn parse_rule_complete(
    mut parser: *mut parser_t,
    mut state: *mut rule_state_t,
) -> *mut ast_t {
    if !parser.is_null() {
    } else {
        ponyint_assert_fail(
            b"parser != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            550 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_rule_complete\0"))
                .as_ptr(),
        );
    };
    if !state.is_null() {
    } else {
        ponyint_assert_fail(
            b"state != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            551 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_rule_complete\0"))
                .as_ptr(),
        );
    };
    process_deferred_ast(parser, state);
    if (*state).scope as libc::c_int != 0 && !((*state).ast).is_null() {
        ast_scope((*state).ast);
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Complete\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
        );
    }
    if ((*state).restart).is_null() {
        return (*state).ast;
    }
    let mut id: token_id = current_token_id(parser);
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Check restart set for next token %s\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
            token_print((*parser).token),
        );
    }
    let mut p: *const token_id = (*state).restart;
    while *p as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint {
        if *p as libc::c_uint == id as libc::c_uint {
            if (*parser).trace_enable {
                fprintf(
                    __stderrp,
                    b"Rule %s: Restart check successful\n\0" as *const u8 as *const libc::c_char,
                    (*state).fn_name,
                );
            }
            return (*state).ast;
        }
        p = p.offset(1);
    }
    if (*parser).trace_enable {
        fprintf(
            __stderrp,
            b"Rule %s: Restart check error\n\0" as *const u8 as *const libc::c_char,
            (*state).fn_name,
        );
    }
    if !((*parser).token).is_null() {
    } else {
        ponyint_assert_fail(
            b"parser->token != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            587 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_rule_complete\0"))
                .as_ptr(),
        );
    };
    error(
        (*parser).errors,
        (*parser).source,
        token_line_number((*parser).token),
        token_line_position((*parser).token),
        b"syntax error: unexpected token %s after %s\0" as *const u8 as *const libc::c_char,
        token_print((*parser).token),
        (*state).desc,
    );
    ast_free((*state).ast);
    (*parser).failed = 1 as libc::c_int != 0;
    ditch_restart(parser, state);
    return 0 as *mut ast_t;
}
#[no_mangle]
#[c2rust::src_loc = "602:1"]
pub unsafe extern "C" fn parse(
    mut package: *mut ast_t,
    mut source: *mut source_t,
    mut start: rule_t,
    mut expected: *const libc::c_char,
    mut errors: *mut errors_t,
    mut allow_test_symbols: bool,
    mut trace: bool,
) -> bool {
    if !package.is_null() {
    } else {
        ponyint_assert_fail(
            b"package != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            605 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"parse\0")).as_ptr(),
        );
    };
    if !source.is_null() {
    } else {
        ponyint_assert_fail(
            b"source != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            606 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"parse\0")).as_ptr(),
        );
    };
    if !expected.is_null() {
    } else {
        ponyint_assert_fail(
            b"expected != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            607 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"parse\0")).as_ptr(),
        );
    };
    let mut lexer: *mut lexer_t = lexer_open(source, errors, allow_test_symbols);
    if lexer.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut parser: *mut parser_t = ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut parser_t;
    let ref mut fresh14 = (*parser).source;
    *fresh14 = source;
    let ref mut fresh15 = (*parser).lexer;
    *fresh15 = lexer;
    let ref mut fresh16 = (*parser).token;
    *fresh16 = lexer_next(lexer);
    let ref mut fresh17 = (*parser).last_token;
    *fresh17 = (*parser).token;
    let ref mut fresh18 = (*parser).last_matched;
    *fresh18 = 0 as *const libc::c_char;
    (*parser).next_flags = 0 as libc::c_int as uint32_t;
    (*parser).free_last_token = 0 as libc::c_int != 0;
    (*parser).failed = 0 as libc::c_int != 0;
    let ref mut fresh19 = (*parser).errors;
    *fresh19 = errors;
    (*parser).trace_enable = trace;
    let error_count: size_t = errors_get_count(errors);
    let mut build_fn: builder_fn_t = None;
    let mut ast: *mut ast_t =
        start.expect("non-null function pointer")(parser, &mut build_fn, expected);
    if ast == 2 as libc::c_int as *mut ast_t {
        ast = 0 as *mut ast_t;
    }
    if ast == 3 as libc::c_int as *mut ast_t {
        syntax_error(parser, expected, 0 as *mut ast_t, 0 as *const libc::c_char);
        ast = 0 as *mut ast_t;
    }
    if (*parser).failed {
        ast_free(ast);
        ast = 0 as *mut ast_t;
    }
    if errors_get_count(errors) > error_count {
        ast_free(ast);
        ast = 0 as *mut ast_t;
    }
    lexer_close(lexer);
    token_free((*parser).token);
    ponyint_pool_free(1 as libc::c_int as size_t, parser as *mut libc::c_void);
    if ast.is_null() {
        source_close(source);
        return 0 as libc::c_int != 0;
    }
    if ast_id(ast) as libc::c_uint == TK_MODULE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(ast) == TK_MODULE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            665 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"parse\0")).as_ptr(),
        );
    };
    if (ast_data(ast)).is_null() {
    } else {
        ponyint_assert_fail(
            b"ast_data(ast) == NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/parserapi.c\0" as *const u8
                as *const libc::c_char,
            666 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"parse\0")).as_ptr(),
        );
    };
    ast_setdata(ast, source as *mut libc::c_void);
    ast_add(package, ast);
    return 1 as libc::c_int != 0;
}
