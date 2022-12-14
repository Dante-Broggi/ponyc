use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_ct_rune_t = libc::c_int;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "106:1"]
    pub type __darwin_wchar_t = libc::c_int;
    #[c2rust::src_loc = "111:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
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
#[c2rust::header_src = "/usr/local/Cellar/llvm@13/13.0.1_2/lib/clang/13.0.1/include/stdarg.h:1"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    use super::_size_t_h::size_t;

    use super::source_h::source_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "75:1"]
        pub fn errorv(
            errors: *mut errors_t,
            source: *mut source_t,
            line: usize,
            pos: usize,
            fmt: *const libc::c_char,
            ap: ::core::ffi::VaList,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexint.h:2"]
pub mod lexint_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct lexint_t {
        pub low: u64,
        pub high: u64,
    }
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn lexint_zero(i: *mut lexint_t);
        #[c2rust::src_loc = "41:1"]
        pub fn lexint_char(i: *mut lexint_t, c: libc::c_int);
        #[c2rust::src_loc = "43:1"]
        pub fn lexint_accum(i: *mut lexint_t, digit: u64, base: u64) -> bool;
        #[c2rust::src_loc = "45:1"]
        pub fn lexint_double(i: *mut lexint_t) -> libc::c_double;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/token.h:2"]
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
    use super::lexint_h::lexint_t;
    use super::source_h::source_t;
    extern "C" {
        #[c2rust::src_loc = "16:16"]
        pub type token_t;
        #[c2rust::src_loc = "282:1"]
        pub fn token_new(id: token_id) -> *mut token_t;
        #[c2rust::src_loc = "380:1"]
        pub fn token_set_string(token: *mut token_t, value: *const libc::c_char, length: usize);
        #[c2rust::src_loc = "383:1"]
        pub fn token_set_float(token: *mut token_t, value: libc::c_double);
        #[c2rust::src_loc = "386:1"]
        pub fn token_set_int(token: *mut token_t, value: *mut lexint_t);
        #[c2rust::src_loc = "391:1"]
        pub fn token_set_pos(token: *mut token_t, source: *mut source_t, line: usize, pos: usize);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:10"]
pub mod runetype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct _RuneLocale {
        pub __magic: [libc::c_char; 8],
        pub __encoding: [libc::c_char; 32],
        pub __sgetrune: Option<
            unsafe extern "C" fn(
                *const libc::c_char,
                usize,
                *mut *const libc::c_char,
            ) -> __darwin_rune_t,
        >,
        pub __sputrune: Option<
            unsafe extern "C" fn(
                __darwin_rune_t,
                *mut libc::c_char,
                usize,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        pub __invalid_rune: __darwin_rune_t,
        pub __runetype: [u32; 256],
        pub __maplower: [__darwin_rune_t; 256],
        pub __mapupper: [__darwin_rune_t; 256],
        pub __runetype_ext: _RuneRange,
        pub __maplower_ext: _RuneRange,
        pub __mapupper_ext: _RuneRange,
        pub __variable: *mut libc::c_void,
        pub __variable_len: libc::c_int,
        pub __ncharclasses: libc::c_int,
        pub __charclasses: *mut _RuneCharClass,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "72:9"]
    pub struct _RuneCharClass {
        pub __name: [libc::c_char; 14],
        pub __mask: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub struct _RuneRange {
        pub __nranges: libc::c_int,
        pub __ranges: *mut _RuneEntry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:9"]
    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut u32,
    }
    use super::_types_h::{__darwin_rune_t, __darwin_size_t};
    extern "C" {
        #[c2rust::src_loc = "111:20"]
        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:5"]
pub mod stringtab_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "15:1"]
        pub fn stringtab_len(string_0: *const libc::c_char, len: usize) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:6"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_pool_realloc_size(
            old_size: usize,
            new_size: usize,
            p: *mut libc::c_void,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:7"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:9"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "70:7"]
        pub fn memchr(
            _: *const libc::c_void,
            _: libc::c_int,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:7"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "77:6"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:10"]
pub mod _ctype_h {
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "211:1"]
    pub unsafe extern "C" fn isalnum(mut _c: libc::c_int) -> libc::c_int {
        return __istype(
            _c,
            (0x100 as libc::c_long | 0x400 as libc::c_long) as libc::c_ulong,
        );
    }
    #[inline]
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn __istype(
        mut _c: __darwin_ct_rune_t,
        mut _f: libc::c_ulong,
    ) -> libc::c_int {
        if isascii(_c) != 0 {
            (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
        } else {
            (__maskrune(_c, _f) != 0) as libc::c_int
        }
    }
    #[inline]
    #[c2rust::src_loc = "134:1"]
    pub unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
        return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "217:1"]
    pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
        return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "236:1"]
    pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
        return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "163:1"]
    pub unsafe extern "C" fn __isctype(
        mut _c: __darwin_ct_rune_t,
        mut _f: libc::c_ulong,
    ) -> __darwin_ct_rune_t {
        return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
            0 as libc::c_int
        } else {
            (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
        };
    }
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "266:1"]
    pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
        return __istype(_c, 0x4000 as libc::c_long as libc::c_ulong);
    }
    use super::_types_h::__darwin_ct_rune_t;
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        #[c2rust::src_loc = "148:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/math.h:11"]
pub mod math_h {
    extern "C" {
        #[c2rust::src_loc = "429:15"]
        pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    }
}
pub use self::_ctype_h::{
    __isctype, __istype, __maskrune, isalnum, isalpha, isascii, isdigit, isspace,
};
pub use self::_size_t_h::size_t;
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, usize, __darwin_wchar_t, __uint32_t,
};
use self::error_h::{errors_t, errorv};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::lexint_h::{lexint_accum, lexint_char, lexint_double, lexint_t, lexint_zero};
use self::math_h::pow;
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_free, ponyint_pool_free_size, ponyint_pool_realloc_size,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::source_h::source_t;
pub use self::stdarg_h::va_list;
use self::string_h::{memchr, memmove, memset, strcmp};
use self::stringtab_h::{stringtab, stringtab_len};
pub use self::token_h::{
    token_id, token_new, token_set_float, token_set_int, token_set_pos, token_set_string, token_t,
    TK_ACTOR, TK_ADDRESS, TK_ALIASED, TK_AND, TK_ANNOTATION, TK_ARRAY, TK_ARROW, TK_AS, TK_ASSIGN,
    TK_AT, TK_AT_LBRACE, TK_BACKSLASH, TK_BARELAMBDA, TK_BARELAMBDATYPE, TK_BE, TK_BEAPP,
    TK_BECHAIN, TK_BEREF, TK_BOX, TK_BREAK, TK_CALL, TK_CAP_ALIAS, TK_CAP_ANY, TK_CAP_READ,
    TK_CAP_SEND, TK_CAP_SHARE, TK_CASE, TK_CASES, TK_CHAIN, TK_CLASS, TK_COLON, TK_COMMA,
    TK_COMPILE_ERROR, TK_COMPILE_INTRINSIC, TK_CONSTANT, TK_CONSUME, TK_CONTINUE, TK_DBLARROW,
    TK_DIGESTOF, TK_DISPOSING_BLOCK, TK_DIVIDE, TK_DIVIDE_TILDE, TK_DO, TK_DONTCARE,
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
#[c2rust::src_loc = "14:8"]
pub struct lexer_t {
    pub source: *mut source_t,
    pub errors: *mut errors_t,
    pub allow_test_symbols: bool,
    pub ptr: usize,
    pub len: usize,
    pub line: usize,
    pub pos: usize,
    pub newline: bool,
    pub token_line: usize,
    pub token_pos: usize,
    pub buffer: *mut libc::c_char,
    pub buflen: usize,
    pub alloc: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:16"]
pub struct lextoken_t {
    pub text: *const libc::c_char,
    pub id: token_id,
}
#[c2rust::src_loc = "49:25"]
static mut symbols: [lextoken_t; 60] = [
    {
        let mut init = lextoken_t {
            text: b"...\0" as *const u8 as *const libc::c_char,
            id: TK_ELLIPSIS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"->\0" as *const u8 as *const libc::c_char,
            id: TK_ARROW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"=>\0" as *const u8 as *const libc::c_char,
            id: TK_DBLARROW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<<~\0" as *const u8 as *const libc::c_char,
            id: TK_LSHIFT_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">>~\0" as *const u8 as *const libc::c_char,
            id: TK_RSHIFT_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"==~\0" as *const u8 as *const libc::c_char,
            id: TK_EQ_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"!=~\0" as *const u8 as *const libc::c_char,
            id: TK_NE_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<=~\0" as *const u8 as *const libc::c_char,
            id: TK_LE_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">=~\0" as *const u8 as *const libc::c_char,
            id: TK_GE_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<~\0" as *const u8 as *const libc::c_char,
            id: TK_LT_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">~\0" as *const u8 as *const libc::c_char,
            id: TK_GT_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"+~\0" as *const u8 as *const libc::c_char,
            id: TK_PLUS_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"-~\0" as *const u8 as *const libc::c_char,
            id: TK_MINUS_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"*~\0" as *const u8 as *const libc::c_char,
            id: TK_MULTIPLY_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"/~\0" as *const u8 as *const libc::c_char,
            id: TK_DIVIDE_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"%%~\0" as *const u8 as *const libc::c_char,
            id: TK_MOD_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"%~\0" as *const u8 as *const libc::c_char,
            id: TK_REM_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<<\0" as *const u8 as *const libc::c_char,
            id: TK_LSHIFT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">>\0" as *const u8 as *const libc::c_char,
            id: TK_RSHIFT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"==\0" as *const u8 as *const libc::c_char,
            id: TK_EQ,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"!=\0" as *const u8 as *const libc::c_char,
            id: TK_NE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<=\0" as *const u8 as *const libc::c_char,
            id: TK_LE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">=\0" as *const u8 as *const libc::c_char,
            id: TK_GE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b".>\0" as *const u8 as *const libc::c_char,
            id: TK_CHAIN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<:\0" as *const u8 as *const libc::c_char,
            id: TK_SUBTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"\\\0" as *const u8 as *const libc::c_char,
            id: TK_BACKSLASH,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"@{\0" as *const u8 as *const libc::c_char,
            id: TK_AT_LBRACE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"{\0" as *const u8 as *const libc::c_char,
            id: TK_LBRACE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"}\0" as *const u8 as *const libc::c_char,
            id: TK_RBRACE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"(\0" as *const u8 as *const libc::c_char,
            id: TK_LPAREN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b")\0" as *const u8 as *const libc::c_char,
            id: TK_RPAREN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"[\0" as *const u8 as *const libc::c_char,
            id: TK_LSQUARE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"]\0" as *const u8 as *const libc::c_char,
            id: TK_RSQUARE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b",\0" as *const u8 as *const libc::c_char,
            id: TK_COMMA,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b".\0" as *const u8 as *const libc::c_char,
            id: TK_DOT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"~\0" as *const u8 as *const libc::c_char,
            id: TK_TILDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b":\0" as *const u8 as *const libc::c_char,
            id: TK_COLON,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b";\0" as *const u8 as *const libc::c_char,
            id: TK_SEMI,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"=\0" as *const u8 as *const libc::c_char,
            id: TK_ASSIGN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"+\0" as *const u8 as *const libc::c_char,
            id: TK_PLUS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"-\0" as *const u8 as *const libc::c_char,
            id: TK_MINUS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"*\0" as *const u8 as *const libc::c_char,
            id: TK_MULTIPLY,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"/\0" as *const u8 as *const libc::c_char,
            id: TK_DIVIDE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"%%\0" as *const u8 as *const libc::c_char,
            id: TK_MOD,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"%\0" as *const u8 as *const libc::c_char,
            id: TK_REM,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"@\0" as *const u8 as *const libc::c_char,
            id: TK_AT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"<\0" as *const u8 as *const libc::c_char,
            id: TK_LT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b">\0" as *const u8 as *const libc::c_char,
            id: TK_GT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"|\0" as *const u8 as *const libc::c_char,
            id: TK_PIPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"&\0" as *const u8 as *const libc::c_char,
            id: TK_ISECTTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"^\0" as *const u8 as *const libc::c_char,
            id: TK_EPHEMERAL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"!\0" as *const u8 as *const libc::c_char,
            id: TK_ALIASED,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"?\0" as *const u8 as *const libc::c_char,
            id: TK_QUESTION,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"-\0" as *const u8 as *const libc::c_char,
            id: TK_UNARY_MINUS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#\0" as *const u8 as *const libc::c_char,
            id: TK_CONSTANT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"(\0" as *const u8 as *const libc::c_char,
            id: TK_LPAREN_NEW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"[\0" as *const u8 as *const libc::c_char,
            id: TK_LSQUARE_NEW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"-~\0" as *const u8 as *const libc::c_char,
            id: TK_MINUS_TILDE_NEW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"-\0" as *const u8 as *const libc::c_char,
            id: TK_MINUS_NEW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: 0 as *const libc::c_char,
            id: TK_EOF,
        };
        init
    },
];
#[c2rust::src_loc = "133:25"]
static mut keywords: [lextoken_t; 78] = [
    {
        let mut init = lextoken_t {
            text: b"compile_intrinsic\0" as *const u8 as *const libc::c_char,
            id: TK_COMPILE_INTRINSIC,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"use\0" as *const u8 as *const libc::c_char,
            id: TK_USE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"type\0" as *const u8 as *const libc::c_char,
            id: TK_TYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"interface\0" as *const u8 as *const libc::c_char,
            id: TK_INTERFACE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"trait\0" as *const u8 as *const libc::c_char,
            id: TK_TRAIT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"primitive\0" as *const u8 as *const libc::c_char,
            id: TK_PRIMITIVE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"struct\0" as *const u8 as *const libc::c_char,
            id: TK_STRUCT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"class\0" as *const u8 as *const libc::c_char,
            id: TK_CLASS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"actor\0" as *const u8 as *const libc::c_char,
            id: TK_ACTOR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"object\0" as *const u8 as *const libc::c_char,
            id: TK_OBJECT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"as\0" as *const u8 as *const libc::c_char,
            id: TK_AS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"is\0" as *const u8 as *const libc::c_char,
            id: TK_IS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"isnt\0" as *const u8 as *const libc::c_char,
            id: TK_ISNT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"var\0" as *const u8 as *const libc::c_char,
            id: TK_VAR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"let\0" as *const u8 as *const libc::c_char,
            id: TK_LET,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"embed\0" as *const u8 as *const libc::c_char,
            id: TK_EMBED,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"new\0" as *const u8 as *const libc::c_char,
            id: TK_NEW,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"fun\0" as *const u8 as *const libc::c_char,
            id: TK_FUN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"be\0" as *const u8 as *const libc::c_char,
            id: TK_BE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"iso\0" as *const u8 as *const libc::c_char,
            id: TK_ISO,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"trn\0" as *const u8 as *const libc::c_char,
            id: TK_TRN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"ref\0" as *const u8 as *const libc::c_char,
            id: TK_REF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"val\0" as *const u8 as *const libc::c_char,
            id: TK_VAL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"box\0" as *const u8 as *const libc::c_char,
            id: TK_BOX,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"tag\0" as *const u8 as *const libc::c_char,
            id: TK_TAG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"this\0" as *const u8 as *const libc::c_char,
            id: TK_THIS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"return\0" as *const u8 as *const libc::c_char,
            id: TK_RETURN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"break\0" as *const u8 as *const libc::c_char,
            id: TK_BREAK,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"continue\0" as *const u8 as *const libc::c_char,
            id: TK_CONTINUE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"consume\0" as *const u8 as *const libc::c_char,
            id: TK_CONSUME,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"recover\0" as *const u8 as *const libc::c_char,
            id: TK_RECOVER,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"if\0" as *const u8 as *const libc::c_char,
            id: TK_IF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"ifdef\0" as *const u8 as *const libc::c_char,
            id: TK_IFDEF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"iftype\0" as *const u8 as *const libc::c_char,
            id: TK_IFTYPE_SET,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"then\0" as *const u8 as *const libc::c_char,
            id: TK_THEN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"else\0" as *const u8 as *const libc::c_char,
            id: TK_ELSE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"elseif\0" as *const u8 as *const libc::c_char,
            id: TK_ELSEIF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"end\0" as *const u8 as *const libc::c_char,
            id: TK_END,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"for\0" as *const u8 as *const libc::c_char,
            id: TK_FOR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"in\0" as *const u8 as *const libc::c_char,
            id: TK_IN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"while\0" as *const u8 as *const libc::c_char,
            id: TK_WHILE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"do\0" as *const u8 as *const libc::c_char,
            id: TK_DO,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"repeat\0" as *const u8 as *const libc::c_char,
            id: TK_REPEAT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"until\0" as *const u8 as *const libc::c_char,
            id: TK_UNTIL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"match\0" as *const u8 as *const libc::c_char,
            id: TK_MATCH,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"where\0" as *const u8 as *const libc::c_char,
            id: TK_WHERE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"try\0" as *const u8 as *const libc::c_char,
            id: TK_TRY,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"with\0" as *const u8 as *const libc::c_char,
            id: TK_WITH,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"error\0" as *const u8 as *const libc::c_char,
            id: TK_ERROR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"compile_error\0" as *const u8 as *const libc::c_char,
            id: TK_COMPILE_ERROR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"not\0" as *const u8 as *const libc::c_char,
            id: TK_NOT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"and\0" as *const u8 as *const libc::c_char,
            id: TK_AND,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"or\0" as *const u8 as *const libc::c_char,
            id: TK_OR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"xor\0" as *const u8 as *const libc::c_char,
            id: TK_XOR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"digestof\0" as *const u8 as *const libc::c_char,
            id: TK_DIGESTOF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"addressof\0" as *const u8 as *const libc::c_char,
            id: TK_ADDRESS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"__loc\0" as *const u8 as *const libc::c_char,
            id: TK_LOCATION,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"true\0" as *const u8 as *const libc::c_char,
            id: TK_TRUE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"false\0" as *const u8 as *const libc::c_char,
            id: TK_FALSE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#read\0" as *const u8 as *const libc::c_char,
            id: TK_CAP_READ,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#send\0" as *const u8 as *const libc::c_char,
            id: TK_CAP_SEND,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#share\0" as *const u8 as *const libc::c_char,
            id: TK_CAP_SHARE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#alias\0" as *const u8 as *const libc::c_char,
            id: TK_CAP_ALIAS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"#any\0" as *const u8 as *const libc::c_char,
            id: TK_CAP_ANY,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$noseq\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_NO_SEQ,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$scope\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_SEQ_SCOPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$try_no_check\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_TRY_NO_CHECK,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$aliased\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_ALIASED,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$updatearg\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_UPDATEARG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$extra\0" as *const u8 as *const libc::c_char,
            id: TK_TEST_EXTRA,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$ifdefand\0" as *const u8 as *const libc::c_char,
            id: TK_IFDEFAND,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$ifdefor\0" as *const u8 as *const libc::c_char,
            id: TK_IFDEFOR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$ifdefnot\0" as *const u8 as *const libc::c_char,
            id: TK_IFDEFNOT,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$flag\0" as *const u8 as *const libc::c_char,
            id: TK_IFDEFFLAG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$let\0" as *const u8 as *const libc::c_char,
            id: TK_MATCH_CAPTURE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$dontcare\0" as *const u8 as *const libc::c_char,
            id: TK_MATCH_DONTCARE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"$iftype\0" as *const u8 as *const libc::c_char,
            id: TK_IFTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: 0 as *const libc::c_char,
            id: TK_EOF,
        };
        init
    },
];
#[c2rust::src_loc = "229:25"]
static mut abstract_0: [lextoken_t; 72] = [
    {
        let mut init = lextoken_t {
            text: b"x\0" as *const u8 as *const libc::c_char,
            id: TK_NONE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"program\0" as *const u8 as *const libc::c_char,
            id: TK_PROGRAM,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"package\0" as *const u8 as *const libc::c_char,
            id: TK_PACKAGE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"module\0" as *const u8 as *const libc::c_char,
            id: TK_MODULE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"members\0" as *const u8 as *const libc::c_char,
            id: TK_MEMBERS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"fvar\0" as *const u8 as *const libc::c_char,
            id: TK_FVAR,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"flet\0" as *const u8 as *const libc::c_char,
            id: TK_FLET,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"dontcare\0" as *const u8 as *const libc::c_char,
            id: TK_DONTCARE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"ffidecl\0" as *const u8 as *const libc::c_char,
            id: TK_FFIDECL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"fficall\0" as *const u8 as *const libc::c_char,
            id: TK_FFICALL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"provides\0" as *const u8 as *const libc::c_char,
            id: TK_PROVIDES,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"uniontype\0" as *const u8 as *const libc::c_char,
            id: TK_UNIONTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"tupletype\0" as *const u8 as *const libc::c_char,
            id: TK_TUPLETYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"nominal\0" as *const u8 as *const libc::c_char,
            id: TK_NOMINAL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"thistype\0" as *const u8 as *const libc::c_char,
            id: TK_THISTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"funtype\0" as *const u8 as *const libc::c_char,
            id: TK_FUNTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"lambdatype\0" as *const u8 as *const libc::c_char,
            id: TK_LAMBDATYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"barelambdatype\0" as *const u8 as *const libc::c_char,
            id: TK_BARELAMBDATYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"dontcaretype\0" as *const u8 as *const libc::c_char,
            id: TK_DONTCARETYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"infer\0" as *const u8 as *const libc::c_char,
            id: TK_INFERTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"errortype\0" as *const u8 as *const libc::c_char,
            id: TK_ERRORTYPE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"literal\0" as *const u8 as *const libc::c_char,
            id: TK_LITERAL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"branch\0" as *const u8 as *const libc::c_char,
            id: TK_LITERALBRANCH,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"opliteral\0" as *const u8 as *const libc::c_char,
            id: TK_OPERATORLITERAL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"typeparams\0" as *const u8 as *const libc::c_char,
            id: TK_TYPEPARAMS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"typeparam\0" as *const u8 as *const libc::c_char,
            id: TK_TYPEPARAM,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"valueformalparam\0" as *const u8 as *const libc::c_char,
            id: TK_VALUEFORMALPARAM,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"params\0" as *const u8 as *const libc::c_char,
            id: TK_PARAMS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"param\0" as *const u8 as *const libc::c_char,
            id: TK_PARAM,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"typeargs\0" as *const u8 as *const libc::c_char,
            id: TK_TYPEARGS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"valueformalarg\0" as *const u8 as *const libc::c_char,
            id: TK_VALUEFORMALARG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"positionalargs\0" as *const u8 as *const libc::c_char,
            id: TK_POSITIONALARGS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"namedargs\0" as *const u8 as *const libc::c_char,
            id: TK_NAMEDARGS,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"namedarg\0" as *const u8 as *const libc::c_char,
            id: TK_NAMEDARG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"updatearg\0" as *const u8 as *const libc::c_char,
            id: TK_UPDATEARG,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"lambdacaptures\0" as *const u8 as *const libc::c_char,
            id: TK_LAMBDACAPTURES,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"lambdacapture\0" as *const u8 as *const libc::c_char,
            id: TK_LAMBDACAPTURE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"lambda\0" as *const u8 as *const libc::c_char,
            id: TK_LAMBDA,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"barelambda\0" as *const u8 as *const libc::c_char,
            id: TK_BARELAMBDA,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"seq\0" as *const u8 as *const libc::c_char,
            id: TK_SEQ,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"qualify\0" as *const u8 as *const libc::c_char,
            id: TK_QUALIFY,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"call\0" as *const u8 as *const libc::c_char,
            id: TK_CALL,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"tuple\0" as *const u8 as *const libc::c_char,
            id: TK_TUPLE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"array\0" as *const u8 as *const libc::c_char,
            id: TK_ARRAY,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"cases\0" as *const u8 as *const libc::c_char,
            id: TK_CASES,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"case\0" as *const u8 as *const libc::c_char,
            id: TK_CASE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"try\0" as *const u8 as *const libc::c_char,
            id: TK_TRY_NO_CHECK,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"reference\0" as *const u8 as *const libc::c_char,
            id: TK_REFERENCE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"packageref\0" as *const u8 as *const libc::c_char,
            id: TK_PACKAGEREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"typeref\0" as *const u8 as *const libc::c_char,
            id: TK_TYPEREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"typeparamref\0" as *const u8 as *const libc::c_char,
            id: TK_TYPEPARAMREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"newref\0" as *const u8 as *const libc::c_char,
            id: TK_NEWREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"newberef\0" as *const u8 as *const libc::c_char,
            id: TK_NEWBEREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"beref\0" as *const u8 as *const libc::c_char,
            id: TK_BEREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"funref\0" as *const u8 as *const libc::c_char,
            id: TK_FUNREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"fvarref\0" as *const u8 as *const libc::c_char,
            id: TK_FVARREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"fletref\0" as *const u8 as *const libc::c_char,
            id: TK_FLETREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"tupleelemref\0" as *const u8 as *const libc::c_char,
            id: TK_TUPLEELEMREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"embedref\0" as *const u8 as *const libc::c_char,
            id: TK_EMBEDREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"varref\0" as *const u8 as *const libc::c_char,
            id: TK_VARREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"letref\0" as *const u8 as *const libc::c_char,
            id: TK_LETREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"paramref\0" as *const u8 as *const libc::c_char,
            id: TK_PARAMREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"dontcareref\0" as *const u8 as *const libc::c_char,
            id: TK_DONTCAREREF,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"newapp\0" as *const u8 as *const libc::c_char,
            id: TK_NEWAPP,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"beapp\0" as *const u8 as *const libc::c_char,
            id: TK_BEAPP,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"funapp\0" as *const u8 as *const libc::c_char,
            id: TK_FUNAPP,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"bechain\0" as *const u8 as *const libc::c_char,
            id: TK_BECHAIN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"funchain\0" as *const u8 as *const libc::c_char,
            id: TK_FUNCHAIN,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"annotation\0" as *const u8 as *const libc::c_char,
            id: TK_ANNOTATION,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"disposingblock\0" as *const u8 as *const libc::c_char,
            id: TK_DISPOSING_BLOCK,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: b"\\n\0" as *const u8 as *const libc::c_char,
            id: TK_NEWLINE,
        };
        init
    },
    {
        let mut init = lextoken_t {
            text: 0 as *const libc::c_char,
            id: TK_EOF,
        };
        init
    },
];
#[c2rust::src_loc = "319:1"]
unsafe extern "C" fn lex_error_at(
    mut lexer: *mut lexer_t,
    mut line: usize,
    mut pos: usize,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv(
        (*lexer).errors,
        (*lexer).source,
        line,
        pos,
        fmt,
        ap.as_va_list(),
    );
}
#[c2rust::src_loc = "330:1"]
unsafe extern "C" fn lex_error(
    mut lexer: *mut lexer_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    errorv(
        (*lexer).errors,
        (*lexer).source,
        (*lexer).token_line,
        (*lexer).token_pos,
        fmt,
        ap.as_va_list(),
    );
}
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn is_eof(mut lexer: *mut lexer_t) -> bool {
    return (*lexer).len == 0 as libc::c_int as libc::c_ulong;
}
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn append_to_token(mut lexer: *mut lexer_t, mut c: libc::c_char) {
    if (*lexer).buflen >= (*lexer).alloc {
        let mut new_len: usize = if (*lexer).alloc > 0 as libc::c_int as libc::c_ulong {
            (*lexer).alloc << 1 as libc::c_int
        } else {
            64 as libc::c_int as libc::c_ulong
        };
        let ref mut fresh0 = (*lexer).buffer;
        *fresh0 = ponyint_pool_realloc_size(
            (*lexer).alloc,
            new_len,
            (*lexer).buffer as *mut libc::c_void,
        ) as *mut libc::c_char;
        (*lexer).alloc = new_len;
    }
    *((*lexer).buffer).offset((*lexer).buflen as isize) = c;
    let ref mut fresh1 = (*lexer).buflen;
    *fresh1 = (*fresh1).wrapping_add(1);
}
#[c2rust::src_loc = "363:1"]
unsafe extern "C" fn make_token(mut lexer: *mut lexer_t, mut id: token_id) -> *mut token_t {
    let mut t: *mut token_t = token_new(id);
    token_set_pos(t, (*lexer).source, (*lexer).token_line, (*lexer).token_pos);
    return t;
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn make_token_with_text(
    mut lexer: *mut lexer_t,
    mut id: token_id,
) -> *mut token_t {
    let mut t: *mut token_t = make_token(lexer, id);
    if ((*lexer).buffer).is_null() {
        token_set_string(
            t,
            stringtab(b"\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int as usize,
        );
    } else {
        token_set_string(
            t,
            stringtab_len((*lexer).buffer, (*lexer).buflen),
            (*lexer).buflen,
        );
    }
    return t;
}
#[c2rust::src_loc = "389:1"]
unsafe extern "C" fn consume_chars(mut lexer: *mut lexer_t, mut count: usize) {
    if (*lexer).len >= count {
    } else {
        ponyint_assert_fail(
            b"lexer->len >= count\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0" as *const u8
                as *const libc::c_char,
            391 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"consume_chars\0"))
                .as_ptr(),
        );
    };
    if count == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if *((*(*lexer).source).m).offset((*lexer).ptr as isize) as libc::c_int == '\n' as i32 {
        let ref mut fresh2 = (*lexer).line;
        *fresh2 = (*fresh2).wrapping_add(1);
        (*lexer).pos = 0 as libc::c_int as usize;
    }
    let ref mut fresh3 = (*lexer).ptr;
    *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(count) as usize as usize;
    let ref mut fresh4 = (*lexer).len;
    *fresh4 = (*fresh4 as libc::c_ulong).wrapping_sub(count) as usize as usize;
    let ref mut fresh5 = (*lexer).pos;
    *fresh5 = (*fresh5 as libc::c_ulong).wrapping_add(count) as usize as usize;
}
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn look(mut lexer: *mut lexer_t) -> libc::c_char {
    if is_eof(lexer) {
        return '\0' as i32 as libc::c_char;
    }
    return *((*(*lexer).source).m).offset((*lexer).ptr as isize);
}
#[c2rust::src_loc = "419:1"]
unsafe extern "C" fn lookn(mut lexer: *mut lexer_t, mut chars: usize) -> libc::c_char {
    if (*lexer).len < chars {
        return '\0' as i32 as libc::c_char;
    }
    return *((*(*lexer).source).m).offset(
        ((*lexer).ptr)
            .wrapping_add(chars)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
    );
}
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn literal_doesnt_terminate(mut lexer: *mut lexer_t) -> *mut token_t {
    lex_error(
        lexer,
        b"Literal doesn't terminate\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh6 = (*lexer).ptr;
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add((*lexer).len) as usize as usize;
    (*lexer).len = 0 as libc::c_int as usize;
    return make_token(lexer, TK_LEX_ERROR);
}
#[c2rust::src_loc = "440:1"]
unsafe extern "C" fn nested_comment(mut lexer: *mut lexer_t) -> *mut token_t {
    consume_chars(lexer, 2 as libc::c_int as usize);
    let mut depth: usize = 1 as libc::c_int as usize;
    while depth > 0 as libc::c_int as libc::c_ulong {
        if (*lexer).len <= 1 as libc::c_int as libc::c_ulong {
            lex_error(
                lexer,
                b"Nested comment doesn't terminate\0" as *const u8 as *const libc::c_char,
            );
            let ref mut fresh7 = (*lexer).ptr;
            *fresh7 = (*fresh7 as libc::c_ulong).wrapping_add((*lexer).len) as usize as usize;
            (*lexer).len = 0 as libc::c_int as usize;
            return make_token(lexer, TK_LEX_ERROR);
        }
        if look(lexer) as libc::c_int == '*' as i32
            && lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '/' as i32
        {
            consume_chars(lexer, 2 as libc::c_int as usize);
            depth = depth.wrapping_sub(1);
        } else if look(lexer) as libc::c_int == '/' as i32
            && lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '*' as i32
        {
            consume_chars(lexer, 2 as libc::c_int as usize);
            depth = depth.wrapping_add(1);
        } else {
            consume_chars(lexer, 1 as libc::c_int as usize);
        }
    }
    (*lexer).newline = 0 as libc::c_int != 0;
    return 0 as *mut token_t;
}
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn line_comment(mut lexer: *mut lexer_t) -> *mut token_t {
    consume_chars(lexer, 2 as libc::c_int as usize);
    while !is_eof(lexer) && look(lexer) as libc::c_int != '\n' as i32 {
        consume_chars(lexer, 1 as libc::c_int as usize);
    }
    return 0 as *mut token_t;
}
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn slash(mut lexer: *mut lexer_t) -> *mut token_t {
    if lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '*' as i32 {
        return nested_comment(lexer);
    }
    if lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '/' as i32 {
        return line_comment(lexer);
    }
    if lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '~' as i32 {
        consume_chars(lexer, 2 as libc::c_int as usize);
        return make_token(lexer, TK_DIVIDE_TILDE);
    }
    consume_chars(lexer, 1 as libc::c_int as usize);
    return make_token(lexer, TK_DIVIDE);
}
#[c2rust::src_loc = "517:1"]
unsafe extern "C" fn normalise_string(mut lexer: *mut lexer_t) {
    if (*lexer).buflen == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if (memchr(
        (*lexer).buffer as *const libc::c_void,
        '\n' as i32,
        (*lexer).buflen,
    ))
    .is_null()
    {
        return;
    }
    let mut buf: *mut libc::c_char = (*lexer).buffer;
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
        && *buf.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        let ref mut fresh8 = (*lexer).buflen;
        *fresh8 = (*fresh8 as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as usize as usize;
        memmove(
            &mut *buf.offset(0 as libc::c_int as isize) as *mut libc::c_char as *mut libc::c_void,
            &mut *buf.offset(2 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void,
            (*lexer).buflen,
        );
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        let ref mut fresh9 = (*lexer).buflen;
        *fresh9 = (*fresh9).wrapping_sub(1);
        memmove(
            &mut *buf.offset(0 as libc::c_int as isize) as *mut libc::c_char as *mut libc::c_void,
            &mut *buf.offset(1 as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void,
            (*lexer).buflen,
        );
    }
    let mut ws: usize = (*lexer).buflen;
    let mut ws_this_line: usize = 0;
    let mut in_leading_ws: bool = 1 as libc::c_int != 0;
    let mut has_non_ws: bool = 0 as libc::c_int != 0;
    let mut i: usize = 0;
    while i < (*lexer).buflen {
        let mut c: libc::c_char = *((*lexer).buffer).offset(i as isize);
        if in_leading_ws {
            if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
                ws_this_line = ws_this_line.wrapping_add(1);
            } else {
                if isspace(c as libc::c_int) == 0 {
                    has_non_ws = 1 as libc::c_int != 0;
                }
                if has_non_ws as libc::c_int != 0 && ws_this_line < ws {
                    ws = ws_this_line;
                }
                in_leading_ws = 0 as libc::c_int != 0;
            }
        }
        if c as libc::c_int == '\n' as i32 {
            ws_this_line = 0 as libc::c_int as usize;
            has_non_ws = 0 as libc::c_int != 0;
            in_leading_ws = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    if ws > 0 as libc::c_int as libc::c_ulong {
        let mut line_start: *mut libc::c_char = (*lexer).buffer;
        let mut compacted: *mut libc::c_char = (*lexer).buffer;
        let mut rem: usize = (*lexer).buflen;
        while rem > 0 as libc::c_int as libc::c_ulong {
            let mut line_end: *mut libc::c_char =
                memchr(line_start as *const libc::c_void, '\n' as i32, rem) as *mut libc::c_char;
            let mut line_len: usize = if line_end.is_null() {
                rem
            } else {
                (line_end.offset_from(line_start) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as usize
            };
            if line_start != line_end {
                let mut trim: usize = if line_len < ws { line_len } else { ws };
                memmove(
                    compacted as *mut libc::c_void,
                    line_start.offset(trim as isize) as *const libc::c_void,
                    line_len.wrapping_sub(trim),
                );
                compacted = compacted.offset(line_len.wrapping_sub(trim) as isize);
            } else {
                memmove(
                    compacted as *mut libc::c_void,
                    line_start as *const libc::c_void,
                    line_len,
                );
                compacted = compacted.offset(line_len as isize);
            }
            line_start = line_start.offset(line_len as isize);
            rem = (rem as libc::c_ulong).wrapping_sub(line_len) as usize as usize;
        }
        (*lexer).buflen = compacted.offset_from((*lexer).buffer) as libc::c_long as usize;
    }
    let mut trim_0: usize = 0;
    let mut i_0: usize = ((*lexer).buflen).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i_0 > 0 as libc::c_int as libc::c_ulong {
        let mut c_0: libc::c_char = *((*lexer).buffer).offset(i_0 as isize);
        if c_0 as libc::c_int == '\n' as i32 {
            let ref mut fresh10 = (*lexer).buflen;
            *fresh10 = (*fresh10 as libc::c_ulong).wrapping_sub(trim_0) as usize as usize;
            break;
        } else {
            if !(isspace(c_0 as libc::c_int) != 0) {
                break;
            }
            trim_0 = trim_0.wrapping_add(1);
            i_0 = i_0.wrapping_sub(1);
        }
    }
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn triple_string(mut lexer: *mut lexer_t) -> *mut token_t {
    consume_chars(lexer, 3 as libc::c_int as usize);
    let mut start_line: usize = (*lexer).line;
    let mut non_space_on_first_line: bool = 0 as libc::c_int != 0;
    loop {
        if is_eof(lexer) {
            return literal_doesnt_terminate(lexer);
        }
        let mut c: libc::c_char = look(lexer);
        if c as libc::c_int == '"' as i32
            && lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '"' as i32
            && lookn(lexer, 3 as libc::c_int as usize) as libc::c_int == '"' as i32
        {
            consume_chars(lexer, 3 as libc::c_int as usize);
            while look(lexer) as libc::c_int == '"' as i32 {
                append_to_token(lexer, '"' as i32 as libc::c_char);
                consume_chars(lexer, 1 as libc::c_int as usize);
            }
            if (*lexer).line > start_line && non_space_on_first_line as libc::c_int != 0 {
                lex_error(
                    lexer,
                    b"multi-line triple-quoted string must be started below the opening triple-quote\0"
                        as *const u8 as *const libc::c_char,
                );
                return make_token(lexer, TK_LEX_ERROR);
            }
            normalise_string(lexer);
            return make_token_with_text(lexer, TK_STRING);
        }
        if (*lexer).line == start_line && isspace(c as libc::c_int) == 0 {
            non_space_on_first_line = 1 as libc::c_int != 0;
        }
        consume_chars(lexer, 1 as libc::c_int as usize);
        append_to_token(lexer, c);
    }
}
#[c2rust::src_loc = "683:1"]
unsafe extern "C" fn read_hex_escape(
    mut lexer: *mut lexer_t,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut value: u32 = 0 as libc::c_int as u32;
    let mut text_len: libc::c_int = 2 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        let mut c: libc::c_char = look(lexer);
        let mut digit: libc::c_int = 0 as libc::c_int;
        if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
            digit = c as libc::c_int - '0' as i32;
        } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
            digit = c as libc::c_int + 10 as libc::c_int - 'a' as i32;
        } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
            digit = c as libc::c_int + 10 as libc::c_int - 'A' as i32;
        } else {
            return -text_len;
        }
        text_len += 1;
        consume_chars(lexer, 1 as libc::c_int as usize);
        value = (value << 4 as libc::c_int).wrapping_add(digit as libc::c_uint);
        i += 1;
    }
    return value as libc::c_int;
}
#[c2rust::src_loc = "716:1"]
unsafe extern "C" fn escape(
    mut lexer: *mut lexer_t,
    mut unicode_allowed: bool,
    mut is_string: bool,
) -> libc::c_int {
    let mut start: *const libc::c_char =
        &mut *((*(*lexer).source).m).offset((*lexer).ptr as isize) as *mut libc::c_char;
    let mut line: usize = (*lexer).line;
    let mut pos: usize = (*lexer).pos;
    let mut c: libc::c_char = lookn(lexer, 2 as libc::c_int as usize);
    consume_chars(lexer, 2 as libc::c_int as usize);
    let mut value: libc::c_int = -(2 as libc::c_int);
    let mut hex_digits: libc::c_int = 0 as libc::c_int;
    match c as libc::c_int {
        97 => {
            value = 0x7 as libc::c_int;
        }
        98 => {
            value = 0x8 as libc::c_int;
        }
        101 => {
            value = 0x1b as libc::c_int;
        }
        102 => {
            value = 0xc as libc::c_int;
        }
        110 => {
            value = 0xa as libc::c_int;
        }
        114 => {
            value = 0xd as libc::c_int;
        }
        116 => {
            value = 0x9 as libc::c_int;
        }
        118 => {
            value = 0xb as libc::c_int;
        }
        92 => {
            value = 0x5c as libc::c_int;
        }
        48 => {
            value = 0 as libc::c_int;
        }
        120 => {
            hex_digits = 2 as libc::c_int;
        }
        34 => {
            if is_string {
                value = 0x22 as libc::c_int;
            }
        }
        39 => {
            if !is_string {
                value = 0x27 as libc::c_int;
            }
        }
        117 => {
            if unicode_allowed {
                hex_digits = 4 as libc::c_int;
            }
        }
        85 => {
            if unicode_allowed {
                hex_digits = 6 as libc::c_int;
            }
        }
        _ => {}
    }
    if hex_digits > 0 as libc::c_int {
        value = read_hex_escape(lexer, hex_digits);
        if value < 0 as libc::c_int {
            lex_error_at(
                lexer,
                line,
                pos,
                b"Invalid escape sequence \"%.*s\", %d hex digits required\0" as *const u8
                    as *const libc::c_char,
                -value,
                start,
                hex_digits,
            );
            return -(1 as libc::c_int);
        }
        if value > 0x10ffff as libc::c_int {
            lex_error_at(
                lexer,
                line,
                pos,
                b"Escape sequence \"%8s\" exceeds unicode range (0x10FFFF)\0" as *const u8
                    as *const libc::c_char,
                start,
            );
            return -(1 as libc::c_int);
        }
    }
    if value < 0 as libc::c_int {
        lex_error_at(
            lexer,
            line,
            pos,
            b"Invalid escape sequence \"%.*s\"\0" as *const u8 as *const libc::c_char,
            -value,
            start,
        );
        return -(1 as libc::c_int);
    }
    return value;
}
#[c2rust::src_loc = "796:1"]
unsafe extern "C" fn append_utf8(mut lexer: *mut lexer_t, mut value: libc::c_int) {
    if value >= 0 as libc::c_int && value <= 0x10ffff as libc::c_int {
    } else {
        ponyint_assert_fail(
            b"value >= 0 && value <= 0x10FFFF\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0" as *const u8
                as *const libc::c_char,
            798 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"append_utf8\0")).as_ptr(),
        );
    };
    if value <= 0x7f as libc::c_int {
        append_to_token(lexer, (value & 0x7f as libc::c_int) as libc::c_char);
    } else if value <= 0x7ff as libc::c_int {
        append_to_token(
            lexer,
            (0xc0 as libc::c_int | value >> 6 as libc::c_int) as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value & 0x3f as libc::c_int) as libc::c_char,
        );
    } else if value <= 0xffff as libc::c_int {
        append_to_token(
            lexer,
            (0xe0 as libc::c_int | value >> 12 as libc::c_int) as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value >> 6 as libc::c_int & 0x3f as libc::c_int) as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value & 0x3f as libc::c_int) as libc::c_char,
        );
    } else {
        append_to_token(
            lexer,
            (0xf0 as libc::c_int | value >> 18 as libc::c_int) as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value >> 12 as libc::c_int & 0x3f as libc::c_int)
                as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value >> 6 as libc::c_int & 0x3f as libc::c_int) as libc::c_char,
        );
        append_to_token(
            lexer,
            (0x80 as libc::c_int | value & 0x3f as libc::c_int) as libc::c_char,
        );
    };
}
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn string(mut lexer: *mut lexer_t) -> *mut token_t {
    if lookn(lexer, 2 as libc::c_int as usize) as libc::c_int == '"' as i32
        && lookn(lexer, 3 as libc::c_int as usize) as libc::c_int == '"' as i32
    {
        return triple_string(lexer);
    }
    consume_chars(lexer, 1 as libc::c_int as usize);
    loop {
        if is_eof(lexer) {
            return literal_doesnt_terminate(lexer);
        }
        let mut c: libc::c_char = look(lexer);
        if c as libc::c_int == '"' as i32 {
            consume_chars(lexer, 1 as libc::c_int as usize);
            return make_token_with_text(lexer, TK_STRING);
        }
        if c as libc::c_int == '\\' as i32 {
            let mut value: libc::c_int =
                escape(lexer, 1 as libc::c_int != 0, 1 as libc::c_int != 0);
            if value >= 0 as libc::c_int {
                append_utf8(lexer, value);
            }
        } else {
            append_to_token(lexer, c);
            consume_chars(lexer, 1 as libc::c_int as usize);
        }
    }
}
#[c2rust::src_loc = "866:1"]
unsafe extern "C" fn character(mut lexer: *mut lexer_t) -> *mut token_t {
    consume_chars(lexer, 1 as libc::c_int as usize);
    let mut chars_consumed: usize = 0;
    let mut value: lexint_t = lexint_t { low: 0, high: 0 };
    lexint_zero(&mut value);
    loop {
        if is_eof(lexer) {
            return literal_doesnt_terminate(lexer);
        }
        let mut c: libc::c_int = look(lexer) as libc::c_int & 0xff as libc::c_int;
        if c == '\'' as i32 {
            let mut t: *mut token_t = 0 as *mut token_t;
            consume_chars(lexer, 1 as libc::c_int as usize);
            if chars_consumed == 0 as libc::c_int as libc::c_ulong {
                lex_error(
                    lexer,
                    b"Empty character literal\0" as *const u8 as *const libc::c_char,
                );
                t = make_token(lexer, TK_LEX_ERROR);
            } else {
                t = make_token(lexer, TK_INT);
                token_set_int(t, &mut value);
            }
            return t;
        }
        if c == '\\' as i32 {
            c = escape(lexer, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
        } else {
            consume_chars(lexer, 1 as libc::c_int as usize);
        }
        chars_consumed = chars_consumed.wrapping_add(1);
        if c >= 0 as libc::c_int {
            lexint_char(&mut value, c);
        }
    }
}
#[c2rust::src_loc = "924:1"]
unsafe extern "C" fn lex_integer(
    mut lexer: *mut lexer_t,
    mut base: u32,
    mut out_value: *mut lexint_t,
    mut out_digit_count: *mut u32,
    mut end_on_e: bool,
    mut context: *const libc::c_char,
) -> bool {
    let mut digit_count: u32 = 0 as libc::c_int as u32;
    let mut previous_underscore: bool = 0 as libc::c_int != 0;
    while !is_eof(lexer) {
        let mut c: libc::c_char = look(lexer);
        let mut digit: u32 = 0 as libc::c_int as u32;
        if c as libc::c_int == '_' as i32 {
            if previous_underscore {
                lex_error(
                    lexer,
                    b"Invalid duplicate underscore in %s\0" as *const u8 as *const libc::c_char,
                    context,
                );
                return 0 as libc::c_int != 0;
            }
            previous_underscore = 1 as libc::c_int != 0;
            consume_chars(lexer, 1 as libc::c_int as usize);
        } else {
            if end_on_e as libc::c_int != 0
                && (c as libc::c_int == 'e' as i32 || c as libc::c_int == 'E' as i32)
            {
                break;
            }
            if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                digit = (c as libc::c_int - '0' as i32) as u32;
            } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
                digit = (c as libc::c_int - 'a' as i32 + 10 as libc::c_int) as u32;
            } else {
                if !(c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32) {
                    break;
                }
                digit = (c as libc::c_int - 'A' as i32 + 10 as libc::c_int) as u32;
            }
            if digit >= base {
                lex_error(
                    lexer,
                    b"Invalid character in %s: %c\0" as *const u8 as *const libc::c_char,
                    context,
                    c as libc::c_int,
                );
                return 0 as libc::c_int != 0;
            }
            if !lexint_accum(out_value, digit as u64, base as u64) {
                lex_error(
                    lexer,
                    b"overflow in numeric literal\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            previous_underscore = 0 as libc::c_int != 0;
            consume_chars(lexer, 1 as libc::c_int as usize);
            digit_count = digit_count.wrapping_add(1);
        }
    }
    if digit_count == 0 as libc::c_int as libc::c_uint {
        lex_error(
            lexer,
            b"No digits in %s\0" as *const u8 as *const libc::c_char,
            context,
        );
        return 0 as libc::c_int != 0;
    }
    if previous_underscore {
        lex_error(
            lexer,
            b"Numeric literal cannot end with underscore in %s\0" as *const u8
                as *const libc::c_char,
            context,
        );
        return 0 as libc::c_int != 0;
    }
    if !out_digit_count.is_null() {
        *out_digit_count = digit_count;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "999:1"]
unsafe extern "C" fn real(
    mut lexer: *mut lexer_t,
    mut integral_value: *mut lexint_t,
) -> *mut token_t {
    let mut significand: lexint_t = *integral_value;
    let mut e: lexint_t = lexint_t { low: 0, high: 0 };
    lexint_zero(&mut e);
    let mut exp_neg: bool = 0 as libc::c_int != 0;
    let mut mantissa_digit_count: u32 = 0 as libc::c_int as u32;
    let mut c: libc::c_char = look(lexer);
    if c as libc::c_int == '.' as i32
        || c as libc::c_int == 'e' as i32
        || c as libc::c_int == 'E' as i32
    {
    } else {
        ponyint_assert_fail(
            b"c == '.' || c == 'e' || c == 'E'\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0" as *const u8
                as *const libc::c_char,
            1009 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"real\0")).as_ptr(),
        );
    };
    if c as libc::c_int == '.' as i32 {
        c = lookn(lexer, 2 as libc::c_int as usize);
        if (c as libc::c_int) < '0' as i32 || c as libc::c_int > '9' as i32 {
            let mut t: *mut token_t = make_token(lexer, TK_INT);
            token_set_int(t, integral_value);
            return t;
        }
        consume_chars(lexer, 1 as libc::c_int as usize);
        if !lex_integer(
            lexer,
            10 as libc::c_int as u32,
            &mut significand,
            &mut mantissa_digit_count,
            1 as libc::c_int != 0,
            b"real number mantissa\0" as *const u8 as *const libc::c_char,
        ) {
            return make_token(lexer, TK_LEX_ERROR);
        }
    }
    if look(lexer) as libc::c_int == 'e' as i32 || look(lexer) as libc::c_int == 'E' as i32 {
        consume_chars(lexer, 1 as libc::c_int as usize);
        if look(lexer) as libc::c_int == '+' as i32 || look(lexer) as libc::c_int == '-' as i32 {
            exp_neg = look(lexer) as libc::c_int == '-' as i32;
            consume_chars(lexer, 1 as libc::c_int as usize);
        }
        if !lex_integer(
            lexer,
            10 as libc::c_int as u32,
            &mut e,
            0 as *mut u32,
            0 as libc::c_int != 0,
            b"real number exponent\0" as *const u8 as *const libc::c_char,
        ) {
            return make_token(lexer, TK_LEX_ERROR);
        }
    }
    let mut t_0: *mut token_t = make_token(lexer, TK_FLOAT);
    let mut ds: libc::c_double = lexint_double(&mut significand);
    let mut de: libc::c_double = lexint_double(&mut e);
    if exp_neg {
        de = -de;
    }
    de -= mantissa_digit_count as libc::c_double;
    token_set_float(t_0, ds * pow(10.0f64, de));
    return t_0;
}
#[c2rust::src_loc = "1064:1"]
unsafe extern "C" fn nondecimal_number(
    mut lexer: *mut lexer_t,
    mut base: libc::c_int,
    mut context: *const libc::c_char,
) -> *mut token_t {
    let mut value: lexint_t = lexint_t { low: 0, high: 0 };
    lexint_zero(&mut value);
    if !lex_integer(
        lexer,
        base as u32,
        &mut value,
        0 as *mut u32,
        0 as libc::c_int != 0,
        context,
    ) {
        return make_token(lexer, TK_LEX_ERROR);
    }
    let mut t: *mut token_t = make_token(lexer, TK_INT);
    token_set_int(t, &mut value);
    return t;
}
#[c2rust::src_loc = "1081:1"]
unsafe extern "C" fn number(mut lexer: *mut lexer_t) -> *mut token_t {
    if look(lexer) as libc::c_int == '0' as i32 {
        match lookn(lexer, 2 as libc::c_int as usize) as libc::c_int {
            120 | 88 => {
                consume_chars(lexer, 2 as libc::c_int as usize);
                return nondecimal_number(
                    lexer,
                    16 as libc::c_int,
                    b"hexadecimal number\0" as *const u8 as *const libc::c_char,
                );
            }
            98 | 66 => {
                consume_chars(lexer, 2 as libc::c_int as usize);
                return nondecimal_number(
                    lexer,
                    2 as libc::c_int,
                    b"binary number\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    }
    let mut value: lexint_t = lexint_t { low: 0, high: 0 };
    lexint_zero(&mut value);
    if !lex_integer(
        lexer,
        10 as libc::c_int as u32,
        &mut value,
        0 as *mut u32,
        1 as libc::c_int != 0,
        b"decimal number\0" as *const u8 as *const libc::c_char,
    ) {
        return make_token(lexer, TK_LEX_ERROR);
    }
    if look(lexer) as libc::c_int == '.' as i32
        || look(lexer) as libc::c_int == 'e' as i32
        || look(lexer) as libc::c_int == 'E' as i32
    {
        return real(lexer, &mut value);
    }
    let mut t: *mut token_t = make_token(lexer, TK_INT);
    token_set_int(t, &mut value);
    return t;
}
#[c2rust::src_loc = "1120:1"]
unsafe extern "C" fn read_id(mut lexer: *mut lexer_t) -> usize {
    let mut len: usize = 0;
    let mut c: libc::c_char = 0;
    loop {
        c = lookn(lexer, len.wrapping_add(1 as libc::c_int as libc::c_ulong));
        if c as libc::c_int != '_' as i32
            && c as libc::c_int != '\'' as i32
            && isalnum(c as libc::c_int) == 0
        {
            break;
        }
        append_to_token(lexer, c);
        len = len.wrapping_add(1);
    }
    append_to_token(lexer, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*lexer).buflen;
    *fresh11 = (*fresh11).wrapping_sub(1);
    return len;
}
#[c2rust::src_loc = "1152:1"]
unsafe extern "C" fn keyword(mut lexer: *mut lexer_t, mut allow_identifiers: bool) -> *mut token_t {
    let mut len: usize = read_id(lexer);
    let mut p: *const lextoken_t = keywords.as_ptr();
    while !((*p).text).is_null() {
        if strcmp((*lexer).buffer, (*p).text) == 0 {
            consume_chars(lexer, len);
            return make_token(lexer, (*p).id);
        }
        p = p.offset(1);
    }
    if allow_identifiers as libc::c_int != 0 && len > 0 as libc::c_int as libc::c_ulong {
        consume_chars(lexer, len);
        return make_token_with_text(lexer, TK_ID);
    }
    return 0 as *mut token_t;
}
#[c2rust::src_loc = "1176:1"]
unsafe extern "C" fn hash(mut lexer: *mut lexer_t) -> *mut token_t {
    append_to_token(lexer, look(lexer));
    consume_chars(lexer, 1 as libc::c_int as usize);
    let mut t: *mut token_t = keyword(lexer, 0 as libc::c_int != 0);
    if !t.is_null() {
        return t;
    }
    return make_token(lexer, TK_CONSTANT);
}
#[c2rust::src_loc = "1191:1"]
unsafe extern "C" fn dollar(mut lexer: *mut lexer_t) -> *mut token_t {
    append_to_token(lexer, look(lexer));
    consume_chars(lexer, 1 as libc::c_int as usize);
    if (*lexer).allow_test_symbols {
        let mut t: *mut token_t = keyword(lexer, 1 as libc::c_int != 0);
        if !t.is_null() {
            return t;
        }
    }
    lex_error(
        lexer,
        b"Unrecognized character: $\0" as *const u8 as *const libc::c_char,
    );
    return make_token(lexer, TK_LEX_ERROR);
}
#[c2rust::src_loc = "1215:1"]
unsafe extern "C" fn newline_symbols(mut raw_token: token_id, mut newline: bool) -> token_id {
    if !newline {
        return raw_token;
    }
    match raw_token as libc::c_uint {
        11 => return TK_LPAREN_NEW,
        13 => return TK_LSQUARE_NEW,
        27 => return TK_MINUS_NEW,
        28 => return TK_MINUS_TILDE_NEW,
        _ => return raw_token,
    };
}
#[c2rust::src_loc = "1233:1"]
unsafe extern "C" fn symbol(mut lexer: *mut lexer_t) -> *mut token_t {
    let mut sym: [libc::c_char; 3] = [0; 3];
    let mut i: usize = 0;
    while i < ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong {
        sym[i as usize] = lookn(lexer, i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
    }
    let mut p: *const lextoken_t = symbols.as_ptr();
    while !((*p).text).is_null() {
        let mut symbol_0: *const libc::c_char = (*p).text;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while *symbol_0.offset(i_0 as isize) as libc::c_int == '\0' as i32
            || *symbol_0.offset(i_0 as isize) as libc::c_int == sym[i_0 as usize] as libc::c_int
        {
            if *symbol_0.offset(i_0 as isize) as libc::c_int == '\0' as i32 {
                consume_chars(lexer, i_0 as usize);
                return make_token(lexer, newline_symbols((*p).id, (*lexer).newline));
            }
            i_0 += 1;
        }
        p = p.offset(1);
    }
    lex_error(
        lexer,
        b"Unrecognized character: %c\0" as *const u8 as *const libc::c_char,
        sym[0 as libc::c_int as usize] as libc::c_int,
    );
    consume_chars(lexer, 1 as libc::c_int as usize);
    return make_token(lexer, TK_LEX_ERROR);
}
#[no_mangle]
#[c2rust::src_loc = "1260:1"]
pub unsafe extern "C" fn lexer_open(
    mut source: *mut source_t,
    mut errors: *mut errors_t,
    mut allow_test_symbols: bool,
) -> *mut lexer_t {
    if !source.is_null() {
    } else {
        ponyint_assert_fail(
            b"source != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0" as *const u8
                as *const libc::c_char,
            1263 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"lexer_open\0")).as_ptr(),
        );
    };
    let mut lexer: *mut lexer_t = ponyint_pool_alloc(2 as libc::c_int as usize) as *mut lexer_t;
    memset(
        lexer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<lexer_t>() as libc::c_ulong,
    );
    let ref mut fresh12 = (*lexer).source;
    *fresh12 = source;
    let ref mut fresh13 = (*lexer).errors;
    *fresh13 = errors;
    (*lexer).allow_test_symbols = allow_test_symbols;
    (*lexer).len = ((*source).len).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*lexer).line = 1 as libc::c_int as usize;
    (*lexer).pos = 1 as libc::c_int as usize;
    (*lexer).newline = 1 as libc::c_int != 0;
    return lexer;
}
#[no_mangle]
#[c2rust::src_loc = "1280:1"]
pub unsafe extern "C" fn lexer_close(mut lexer: *mut lexer_t) {
    if lexer.is_null() {
        return;
    }
    if !((*lexer).buffer).is_null() {
        ponyint_pool_free_size((*lexer).alloc, (*lexer).buffer as *mut libc::c_void);
    }
    ponyint_pool_free(2 as libc::c_int as usize, lexer as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1292:1"]
pub unsafe extern "C" fn lexer_next(mut lexer: *mut lexer_t) -> *mut token_t {
    if !lexer.is_null() {
    } else {
        ponyint_assert_fail(
            b"lexer != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0" as *const u8
                as *const libc::c_char,
            1294 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"lexer_next\0")).as_ptr(),
        );
    };
    let mut t: *mut token_t = 0 as *mut token_t;
    while t.is_null() {
        (*lexer).token_line = (*lexer).line;
        (*lexer).token_pos = (*lexer).pos;
        (*lexer).buflen = 0 as libc::c_int as usize;
        if is_eof(lexer) {
            t = make_token(lexer, TK_EOF);
            break;
        } else {
            let mut c: libc::c_char = look(lexer);
            match c as libc::c_int {
                10 => {
                    (*lexer).newline = 1 as libc::c_int != 0;
                    consume_chars(lexer, 1 as libc::c_int as usize);
                }
                13 | 9 | 32 => {
                    consume_chars(lexer, 1 as libc::c_int as usize);
                }
                47 => {
                    t = slash(lexer);
                }
                34 => {
                    t = string(lexer);
                }
                39 => {
                    t = character(lexer);
                }
                35 => {
                    t = hash(lexer);
                }
                36 => {
                    t = dollar(lexer);
                }
                _ => {
                    if isdigit(c as libc::c_int) != 0 {
                        t = number(lexer);
                    } else if isalpha(c as libc::c_int) != 0 || c as libc::c_int == '_' as i32 {
                        t = keyword(lexer, 1 as libc::c_int != 0);
                        if !t.is_null() {
                        } else {
                            ponyint_assert_fail(
                                b"t != NULL\0" as *const u8 as *const libc::c_char,
                                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexer.c\0"
                                    as *const u8 as *const libc::c_char,
                                1353 as libc::c_int as usize,
                                (*::core::mem::transmute::<
                                    &[u8; 11],
                                    &[libc::c_char; 11],
                                >(b"lexer_next\0"))
                                    .as_ptr(),
                            );
                        };
                    } else {
                        t = symbol(lexer);
                    }
                }
            }
        }
    }
    (*lexer).newline = 0 as libc::c_int != 0;
    return t;
}
#[no_mangle]
#[c2rust::src_loc = "1367:1"]
pub unsafe extern "C" fn lexer_print(mut id: token_id) -> *const libc::c_char {
    let mut p: *const lextoken_t = abstract_0.as_ptr();
    while !((*p).text).is_null() {
        if id as libc::c_uint == (*p).id as libc::c_uint {
            return (*p).text;
        }
        p = p.offset(1);
    }
    let mut p_0: *const lextoken_t = keywords.as_ptr();
    while !((*p_0).text).is_null() {
        if id as libc::c_uint == (*p_0).id as libc::c_uint {
            return (*p_0).text;
        }
        p_0 = p_0.offset(1);
    }
    let mut p_1: *const lextoken_t = symbols.as_ptr();
    while !((*p_1).text).is_null() {
        if id as libc::c_uint == (*p_1).id as libc::c_uint {
            return (*p_1).text;
        }
        p_1 = p_1.offset(1);
    }
    return 0 as *const libc::c_char;
}
