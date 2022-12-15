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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;

    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_hash_str(str: *const libc::c_char) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = usize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: usize,
        pub size: usize,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }

    use super::fun_h::{cmp_fn, free_fn};
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: usize);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: *mut usize,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
            index: usize,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: usize,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: usize);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> usize;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut usize,
            count: usize,
            item_bitmap: *mut bitmap_t,
            size: usize,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/symtab.h:1"]
pub mod symtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type ast_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/reify.h:1"]
pub mod reify_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct deferred_reification_t {
        pub ast: *mut ast_t,
        pub type_typeparams: *mut ast_t,
        pub type_typeargs: *mut ast_t,
        pub method_typeparams: *mut ast_t,
        pub method_typeargs: *mut ast_t,
        pub thistype: *mut ast_t,
    }
    use super::symtab_h::ast_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.h:1"]
pub mod reach_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct reach_method_t {
        pub name: *const libc::c_char,
        pub mangled_name: *const libc::c_char,
        pub full_name: *const libc::c_char,
        pub cap: token_id,
        pub fun: *mut deferred_reification_t,
        pub typeargs: *mut ast_t,
        pub vtable_index: u32,
        pub intrinsic: bool,
        pub internal: bool,
        pub forwarding: bool,
        pub subordinate: *mut reach_method_t,
        pub param_count: usize,
        pub params: *mut reach_param_t,
        pub result: *mut reach_type_t,
        pub c_method: *mut compile_opaque_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct compile_opaque_t {
        pub free_fn: compile_opaque_free_fn,
    }
    #[c2rust::src_loc = "26:1"]
    pub type compile_opaque_free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:8"]
    pub struct reach_type_t {
        pub name: *const libc::c_char,
        pub mangle: *const libc::c_char,
        pub ast: *mut ast_t,
        pub ast_cap: *mut ast_t,
        pub underlying: token_id,
        pub methods: reach_method_names_t,
        pub bare_method: *mut reach_method_t,
        pub subtypes: reach_type_cache_t,
        pub type_id: u32,
        pub vtable_size: u32,
        pub can_be_boxed: bool,
        pub is_trait: bool,
        pub field_count: u32,
        pub fields: *mut reach_field_t,
        pub c_type: *mut compile_opaque_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:8"]
    pub struct reach_field_t {
        pub ast: *mut ast_t,
        pub type_0: *mut reach_type_t,
        pub embed: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:45"]
    pub struct reach_type_cache_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:47"]
    pub struct reach_method_names_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:8"]
    pub struct reach_param_t {
        pub name: *const libc::c_char,
        pub ast: *mut ast_t,
        pub type_0: *mut reach_type_t,
        pub cap: token_id,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:8"]
    pub struct reach_method_name_t {
        pub id: token_id,
        pub cap: token_id,
        pub name: *const libc::c_char,
        pub r_methods: reach_methods_t,
        pub r_mangled: reach_mangled_t,
        pub internal: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:42"]
    pub struct reach_mangled_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:42"]
    pub struct reach_methods_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:40"]
    pub struct reach_types_t {
        pub contents: hashmap_t,
    }

    use super::hash_h::hashmap_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "23:55"]
        pub fn reach_types_next(map: *mut reach_types_t, i: *mut usize) -> *mut reach_type_t;
        #[c2rust::src_loc = "23:1"]
        pub fn reach_types_size(map: *mut reach_types_t) -> usize;
        #[c2rust::src_loc = "22:3"]
        pub fn reach_method_names_next(
            map: *mut reach_method_names_t,
            i: *mut usize,
        ) -> *mut reach_method_name_t;
        #[c2rust::src_loc = "21:1"]
        pub fn reach_method_names_size(map: *mut reach_method_names_t) -> usize;
        #[c2rust::src_loc = "20:59"]
        pub fn reach_mangled_next(map: *mut reach_mangled_t, i: *mut usize) -> *mut reach_method_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:3"]
pub mod pool_h {

    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:4"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:5"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_str};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_mangled_next, reach_mangled_t,
    reach_method_name_t, reach_method_names_next, reach_method_names_size, reach_method_names_t,
    reach_method_t, reach_methods_t, reach_param_t, reach_type_cache_t, reach_type_t,
    reach_types_next, reach_types_size, reach_types_t,
};
pub use self::reify_h::deferred_reification_t;
use self::stdio_h::printf;
use self::string_h::memset;

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
#[c2rust::src_loc = "116:16"]
pub struct painter_t {
    pub names: name_records_t,
    pub colours: *mut colour_record_t,
    pub colour_next: *mut *mut colour_record_t,
    pub typemap_size: usize,
    pub colour_count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "108:16"]
pub struct colour_record_t {
    pub colour: u32,
    pub type_map: *mut u64,
    pub next: *mut colour_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "103:31"]
pub struct name_records_t {
    pub contents: hashmap_t,
}
#[c2rust::src_loc = "104:1"]
pub type name_records_free_fn = Option<unsafe extern "C" fn(*mut name_record_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "77:16"]
pub struct name_record_t {
    pub name: *const libc::c_char,
    pub colour: u32,
    pub typemap_size: usize,
    pub type_map: *mut u64,
}
#[c2rust::src_loc = "104:1"]
pub type name_records_cmp_fn =
    Option<unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool>;
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn name_record_hash(mut p: *mut name_record_t) -> usize {
    ponyint_hash_str((*p).name)
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn name_record_cmp(mut a: *mut name_record_t, mut b: *mut name_record_t) -> bool {
    (*a).name == (*b).name
}
#[c2rust::src_loc = "97:1"]
unsafe extern "C" fn name_record_free(mut p: *mut name_record_t) {
    ponyint_pool_free_size(
        ((*p).typemap_size).wrapping_mul(::core::mem::size_of::<u64>()),
        (*p).type_map as *mut libc::c_void,
    );
    ponyint_pool_free(0 as libc::c_int as usize, p as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_destroy(mut map: *mut name_records_t) {
    let mut freef: name_records_free_fn =
        Some(name_record_free as unsafe extern "C" fn(*mut name_record_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<name_records_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_size(mut map: *mut name_records_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_alloc_size(mut map: *mut name_records_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_mem_size(mut map: *mut name_records_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_optimize(mut map: *mut name_records_t) {
    let mut cmpf: name_records_cmp_fn = Some(
        name_record_cmp as unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<name_records_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_putindex(
    mut map: *mut name_records_t,
    mut entry: *mut name_record_t,
    mut index: usize,
) {
    let mut cmpf: name_records_cmp_fn = Some(
        name_record_cmp as unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        name_record_hash(entry),
        ::core::mem::transmute::<name_records_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_removeindex(mut map: *mut name_records_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_clearindex(mut map: *mut name_records_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn name_records_init(mut map: *mut name_records_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "104:46"]
pub unsafe extern "C" fn name_records_remove(
    mut map: *mut name_records_t,
    mut entry: *mut name_record_t,
) -> *mut name_record_t {
    let mut cmpf: name_records_cmp_fn = Some(
        name_record_cmp as unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        name_record_hash(entry),
        ::core::mem::transmute::<name_records_cmp_fn, cmp_fn>(cmpf),
    ) as *mut name_record_t
}
#[no_mangle]
#[c2rust::src_loc = "104:46"]
pub unsafe extern "C" fn name_records_next(
    mut map: *mut name_records_t,
    mut i: *mut usize,
) -> *mut name_record_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut name_record_t
}
#[no_mangle]
#[c2rust::src_loc = "104:46"]
pub unsafe extern "C" fn name_records_get(
    mut map: *mut name_records_t,
    mut key: *mut name_record_t,
    mut index: *mut usize,
) -> *mut name_record_t {
    let mut cmpf: name_records_cmp_fn = Some(
        name_record_cmp as unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        name_record_hash(key),
        ::core::mem::transmute::<name_records_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut name_record_t
}
#[no_mangle]
#[c2rust::src_loc = "104:46"]
pub unsafe extern "C" fn name_records_put(
    mut map: *mut name_records_t,
    mut entry: *mut name_record_t,
) -> *mut name_record_t {
    let mut cmpf: name_records_cmp_fn = Some(
        name_record_cmp as unsafe extern "C" fn(*mut name_record_t, *mut name_record_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        name_record_hash(entry),
        ::core::mem::transmute::<name_records_cmp_fn, cmp_fn>(cmpf),
    ) as *mut name_record_t
}
#[c2rust::src_loc = "127:1"]
unsafe extern "C" fn print_typemap(mut size: usize, mut map: *mut u64) {
    if !map.is_null() {
    } else {
        ponyint_assert_fail(
            b"map != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            129 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"print_typemap\0"))
                .as_ptr(),
        );
    };
    let mut i: usize = 0;
    while i < size {
        printf(b"  \0" as *const u8 as *const libc::c_char);
        let mut mask: u64 = 1 as libc::c_int as u64;
        while mask != 0 {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                if *map.offset(i as isize) & mask == 0 {
                    '.' as i32
                } else {
                    'T' as i32
                },
            );
            mask <<= 1 as libc::c_int;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn painter_print(mut painter: *mut painter_t) {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            146 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"painter_print\0"))
                .as_ptr(),
        );
    };
    printf(
        b"Painter typemaps are %zu bits\n\0" as *const u8 as *const libc::c_char,
        ((*painter).typemap_size)
            .wrapping_mul(::core::mem::size_of::<u64>())
            .wrapping_mul((8 as libc::c_int as libc::c_ulong).try_into().unwrap()),
    );
    printf(b"Painter names:\n\0" as *const u8 as *const libc::c_char);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut name: *mut name_record_t = 0 as *mut name_record_t;
    loop {
        name = name_records_next(&mut (*painter).names, &mut i);
        if name.is_null() {
            break;
        }
        printf(
            b"\"%s\" colour \0" as *const u8 as *const libc::c_char,
            (*name).name,
        );
        if (*name).colour == -(1 as libc::c_int) as u32 {
            printf(b"unassigned\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(
                b"%u\n\0" as *const u8 as *const libc::c_char,
                (*name).colour,
            );
        }
        print_typemap((*name).typemap_size, (*name).type_map);
    }
    printf(
        b"Painter has %u colours:\n\0" as *const u8 as *const libc::c_char,
        (*painter).colour_count,
    );
    let mut c: *mut colour_record_t = (*painter).colours;
    while !c.is_null() {
        printf(
            b"  Colour %u\n\0" as *const u8 as *const libc::c_char,
            (*c).colour,
        );
        print_typemap((*painter).typemap_size, (*c).type_map);
        c = (*c).next;
    }
    printf(b"Painter end\n\0" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn add_name(
    mut painter: *mut painter_t,
    mut name: *const libc::c_char,
) -> *mut name_record_t {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_name\0")).as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            184 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"add_name\0")).as_ptr(),
        );
    };
    let mut map_byte_count: usize =
        ((*painter).typemap_size).wrapping_mul(::core::mem::size_of::<u64>());
    let mut n: *mut name_record_t =
        ponyint_pool_alloc(0 as libc::c_int as usize) as *mut name_record_t;
    let ref mut fresh0 = (*n).name;
    *fresh0 = name;
    (*n).colour = -(1 as libc::c_int) as u32;
    (*n).typemap_size = (*painter).typemap_size;
    let ref mut fresh1 = (*n).type_map;
    *fresh1 = ponyint_pool_alloc_size(map_byte_count) as *mut u64;
    memset(
        (*n).type_map as *mut libc::c_void,
        0 as libc::c_int,
        map_byte_count.try_into().unwrap(),
    );
    name_records_put(&mut (*painter).names, n);
    n
}
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn add_colour(mut painter: *mut painter_t) -> *mut colour_record_t {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"add_colour\0")).as_ptr(),
        );
    };
    let mut map_byte_count: usize =
        ((*painter).typemap_size).wrapping_mul(::core::mem::size_of::<u64>());
    let mut n: *mut colour_record_t =
        ponyint_pool_alloc(0 as libc::c_int as usize) as *mut colour_record_t;
    (*n).colour = (*painter).colour_count;
    let ref mut fresh2 = (*n).type_map;
    *fresh2 = ponyint_pool_alloc_size(map_byte_count) as *mut u64;
    let ref mut fresh3 = (*n).next;
    *fresh3 = 0 as *mut colour_record_t;
    memset(
        (*n).type_map as *mut libc::c_void,
        0 as libc::c_int,
        map_byte_count.try_into().unwrap(),
    );
    let ref mut fresh4 = *(*painter).colour_next;
    *fresh4 = n;
    let ref mut fresh5 = (*painter).colour_next;
    *fresh5 = &mut (*n).next;
    let ref mut fresh6 = (*painter).colour_count;
    *fresh6 = (*fresh6).wrapping_add(1);
    n
}
#[c2rust::src_loc = "221:1"]
unsafe extern "C" fn find_name(
    mut painter: *mut painter_t,
    mut name: *const libc::c_char,
) -> *mut name_record_t {
    let mut n: name_record_t = {
        let mut init = name_record_t {
            name,
            colour: 0 as libc::c_int as u32,
            typemap_size: 0 as libc::c_int as usize,
            type_map: 0 as *mut u64,
        };
        init
    };
    let mut index: usize = -(1 as libc::c_int) as usize;
    return name_records_get(&mut (*painter).names, &mut n, &mut index);
}
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn is_name_compatible(
    mut colour: *mut colour_record_t,
    mut name: *mut name_record_t,
) -> bool {
    if !colour.is_null() {
    } else {
        ponyint_assert_fail(
            b"colour != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            234 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"is_name_compatible\0"))
                .as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            235 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"is_name_compatible\0"))
                .as_ptr(),
        );
    };
    let mut i: usize = 0;
    while i < (*name).typemap_size {
        if *((*colour).type_map).offset(i as isize) & *((*name).type_map).offset(i as isize) != 0 {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "250:1"]
unsafe extern "C" fn assign_name_to_colour(
    mut colour: *mut colour_record_t,
    mut name: *mut name_record_t,
) {
    if !colour.is_null() {
    } else {
        ponyint_assert_fail(
            b"colour != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            252 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"assign_name_to_colour\0"))
                .as_ptr(),
        );
    };
    if !name.is_null() {
    } else {
        ponyint_assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            253 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"assign_name_to_colour\0"))
                .as_ptr(),
        );
    };
    let mut i: usize = 0;
    while i < (*name).typemap_size {
        let ref mut fresh7 = *((*colour).type_map).offset(i as isize);
        *fresh7 |= *((*name).type_map).offset(i as isize);
        i = i.wrapping_add(1);
    }
    (*name).colour = (*colour).colour;
}
#[c2rust::src_loc = "264:1"]
unsafe extern "C" fn find_names_types_use(
    mut painter: *mut painter_t,
    mut types: *mut reach_types_t,
) {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            266 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"find_names_types_use\0"))
                .as_ptr(),
        );
    };
    if !types.is_null() {
    } else {
        ponyint_assert_fail(
            b"types != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            267 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"find_names_types_use\0"))
                .as_ptr(),
        );
    };
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut typemap_index: usize = 0;
    let mut typemap_mask: u64 = 1 as libc::c_int as u64;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(types, &mut i);
        if t.is_null() {
            break;
        }
        if !((*t).bare_method).is_null() {
            continue;
        }
        if typemap_index < (*painter).typemap_size {
        } else {
            ponyint_assert_fail(
                b"typemap_index < painter->typemap_size\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0"
                    as *const u8 as *const libc::c_char,
                279 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"find_names_types_use\0",
                ))
                .as_ptr(),
            );
        };
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
        loop {
            n = reach_method_names_next(&mut (*t).methods, &mut j);
            if n.is_null() {
                break;
            }
            let mut k: usize = -(1 as libc::c_int) as usize;
            let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
            loop {
                m = reach_mangled_next(&mut (*n).r_mangled, &mut k);
                if m.is_null() {
                    break;
                }
                let mut name: *const libc::c_char = (*m).mangled_name;
                let mut name_rec: *mut name_record_t = find_name(painter, name);
                if name_rec.is_null() {
                    name_rec = add_name(painter, name);
                }
                let ref mut fresh8 = *((*name_rec).type_map).offset(typemap_index as isize);
                *fresh8 |= typemap_mask;
            }
        }
        typemap_mask <<= 1 as libc::c_int;
        if typemap_mask == 0 {
            typemap_mask = 1 as libc::c_int as u64;
            typemap_index = typemap_index.wrapping_add(1);
        }
    }
}
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn assign_colours_to_names(mut painter: *mut painter_t) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut name: *mut name_record_t = 0 as *mut name_record_t;
    loop {
        name = name_records_next(&mut (*painter).names, &mut i);
        if name.is_null() {
            break;
        }
        let mut colour: *mut colour_record_t = 0 as *mut colour_record_t;
        let mut c: *mut colour_record_t = (*painter).colours;
        while !c.is_null() {
            if is_name_compatible(c, name) {
                colour = c;
                break;
            } else {
                c = (*c).next;
            }
        }
        if colour.is_null() {
            colour = add_colour(painter);
        }
        assign_name_to_colour(colour, name);
    }
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn distribute_info(mut painter: *mut painter_t, mut types: *mut reach_types_t) {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            348 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"distribute_info\0"))
                .as_ptr(),
        );
    };
    if !types.is_null() {
    } else {
        ponyint_assert_fail(
            b"types != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            349 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"distribute_info\0"))
                .as_ptr(),
        );
    };
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(types, &mut i);
        if t.is_null() {
            break;
        }
        if reach_method_names_size(&mut (*t).methods) == 0 || !((*t).bare_method).is_null() {
            continue;
        }
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
        let mut max_colour: u32 = 0 as libc::c_int as u32;
        loop {
            n = reach_method_names_next(&mut (*t).methods, &mut j);
            if n.is_null() {
                break;
            }
            let mut k: usize = -(1 as libc::c_int) as usize;
            let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
            loop {
                m = reach_mangled_next(&mut (*n).r_mangled, &mut k);
                if m.is_null() {
                    break;
                }
                let mut name: *const libc::c_char = (*m).mangled_name;
                let mut name_rec: *mut name_record_t = find_name(painter, name);
                if !name_rec.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"name_rec != NULL\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0"
                            as *const u8 as *const libc::c_char,
                        375 as libc::c_int as usize,
                        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"distribute_info\0",
                        ))
                        .as_ptr(),
                    );
                };
                let mut colour: u32 = (*name_rec).colour;
                (*m).vtable_index = colour;
                if colour > max_colour {
                    max_colour = colour;
                }
            }
        }
        (*t).vtable_size = max_colour.wrapping_add(1 as libc::c_int as libc::c_uint);
    }
}
#[c2rust::src_loc = "392:1"]
unsafe extern "C" fn painter_tidy(mut painter: *mut painter_t) {
    if !painter.is_null() {
    } else {
        ponyint_assert_fail(
            b"painter != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            394 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"painter_tidy\0")).as_ptr(),
        );
    };
    let mut map_byte_count: usize =
        ((*painter).typemap_size).wrapping_mul(::core::mem::size_of::<u64>());
    name_records_destroy(&mut (*painter).names);
    let mut c: *mut colour_record_t = (*painter).colours;
    while !c.is_null() {
        let mut next: *mut colour_record_t = (*c).next;
        ponyint_pool_free_size(map_byte_count, (*c).type_map as *mut libc::c_void);
        ponyint_pool_free(0 as libc::c_int as usize, c as *mut libc::c_void);
        c = next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "412:1"]
pub unsafe extern "C" fn paint(mut types: *mut reach_types_t) {
    if !types.is_null() {
    } else {
        ponyint_assert_fail(
            b"types != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.c\0" as *const u8
                as *const libc::c_char,
            414 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"paint\0")).as_ptr(),
        );
    };
    let mut type_count: usize = reach_types_size(types);
    if type_count == 0 {
        return;
    }
    let mut painter: painter_t = painter_t {
        names: name_records_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        colours: 0 as *mut colour_record_t,
        colour_next: 0 as *mut *mut colour_record_t,
        typemap_size: 0,
        colour_count: 0,
    };
    name_records_init(&mut painter.names, 8 as libc::c_int as usize);
    painter.colours = 0 as *mut colour_record_t;
    painter.colour_next = &mut painter.colours;
    painter.colour_count = 0 as libc::c_int as u32;
    painter.typemap_size = type_count
        .wrapping_sub(1)
        .wrapping_div((64 as libc::c_int as libc::c_ulong).try_into().unwrap())
        .wrapping_add(1);
    find_names_types_use(&mut painter, types);
    assign_colours_to_names(&mut painter);
    distribute_info(&mut painter, types);
    painter_tidy(&mut painter);
}
