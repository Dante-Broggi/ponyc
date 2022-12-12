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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Types.h:1"]
pub mod Types_h {
    #[c2rust::src_loc = "28:1"]
    pub type LLVMBool = libc::c_int;
    #[c2rust::src_loc = "53:1"]
    pub type LLVMContextRef = *mut LLVMOpaqueContext;
    #[c2rust::src_loc = "61:1"]
    pub type LLVMModuleRef = *mut LLVMOpaqueModule;
    #[c2rust::src_loc = "68:1"]
    pub type LLVMTypeRef = *mut LLVMOpaqueType;
    #[c2rust::src_loc = "75:1"]
    pub type LLVMValueRef = *mut LLVMOpaqueValue;
    #[c2rust::src_loc = "82:1"]
    pub type LLVMBasicBlockRef = *mut LLVMOpaqueBasicBlock;
    #[c2rust::src_loc = "89:1"]
    pub type LLVMMetadataRef = *mut LLVMOpaqueMetadata;
    #[c2rust::src_loc = "110:1"]
    pub type LLVMBuilderRef = *mut LLVMOpaqueBuilder;
    #[c2rust::src_loc = "117:1"]
    pub type LLVMDIBuilderRef = *mut LLVMOpaqueDIBuilder;
    extern "C" {
        #[c2rust::src_loc = "53:16"]
        pub type LLVMOpaqueContext;
        #[c2rust::src_loc = "61:16"]
        pub type LLVMOpaqueModule;
        #[c2rust::src_loc = "68:16"]
        pub type LLVMOpaqueType;
        #[c2rust::src_loc = "75:16"]
        pub type LLVMOpaqueValue;
        #[c2rust::src_loc = "82:16"]
        pub type LLVMOpaqueBasicBlock;
        #[c2rust::src_loc = "89:16"]
        pub type LLVMOpaqueMetadata;
        #[c2rust::src_loc = "110:16"]
        pub type LLVMOpaqueBuilder;
        #[c2rust::src_loc = "117:16"]
        pub type LLVMOpaqueDIBuilder;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Core.h:1"]
pub mod Core_h {
    #[c2rust::src_loc = "148:9"]
    pub type LLVMTypeKind = libc::c_uint;
    #[c2rust::src_loc = "168:3"]
    pub const LLVMX86_AMXTypeKind: LLVMTypeKind = 19;
    #[c2rust::src_loc = "167:3"]
    pub const LLVMBFloatTypeKind: LLVMTypeKind = 18;
    #[c2rust::src_loc = "166:3"]
    pub const LLVMScalableVectorTypeKind: LLVMTypeKind = 17;
    #[c2rust::src_loc = "165:3"]
    pub const LLVMTokenTypeKind: LLVMTypeKind = 16;
    #[c2rust::src_loc = "164:3"]
    pub const LLVMX86_MMXTypeKind: LLVMTypeKind = 15;
    #[c2rust::src_loc = "163:3"]
    pub const LLVMMetadataTypeKind: LLVMTypeKind = 14;
    #[c2rust::src_loc = "162:3"]
    pub const LLVMVectorTypeKind: LLVMTypeKind = 13;
    #[c2rust::src_loc = "161:3"]
    pub const LLVMPointerTypeKind: LLVMTypeKind = 12;
    #[c2rust::src_loc = "160:3"]
    pub const LLVMArrayTypeKind: LLVMTypeKind = 11;
    #[c2rust::src_loc = "159:3"]
    pub const LLVMStructTypeKind: LLVMTypeKind = 10;
    #[c2rust::src_loc = "158:3"]
    pub const LLVMFunctionTypeKind: LLVMTypeKind = 9;
    #[c2rust::src_loc = "157:3"]
    pub const LLVMIntegerTypeKind: LLVMTypeKind = 8;
    #[c2rust::src_loc = "156:3"]
    pub const LLVMLabelTypeKind: LLVMTypeKind = 7;
    #[c2rust::src_loc = "155:3"]
    pub const LLVMPPC_FP128TypeKind: LLVMTypeKind = 6;
    #[c2rust::src_loc = "154:3"]
    pub const LLVMFP128TypeKind: LLVMTypeKind = 5;
    #[c2rust::src_loc = "153:3"]
    pub const LLVMX86_FP80TypeKind: LLVMTypeKind = 4;
    #[c2rust::src_loc = "152:3"]
    pub const LLVMDoubleTypeKind: LLVMTypeKind = 3;
    #[c2rust::src_loc = "151:3"]
    pub const LLVMFloatTypeKind: LLVMTypeKind = 2;
    #[c2rust::src_loc = "150:3"]
    pub const LLVMHalfTypeKind: LLVMTypeKind = 1;
    #[c2rust::src_loc = "149:3"]
    pub const LLVMVoidTypeKind: LLVMTypeKind = 0;
    #[c2rust::src_loc = "171:9"]
    pub type LLVMLinkage = libc::c_uint;
    #[c2rust::src_loc = "191:3"]
    pub const LLVMLinkerPrivateWeakLinkage: LLVMLinkage = 16;
    #[c2rust::src_loc = "190:3"]
    pub const LLVMLinkerPrivateLinkage: LLVMLinkage = 15;
    #[c2rust::src_loc = "189:3"]
    pub const LLVMCommonLinkage: LLVMLinkage = 14;
    #[c2rust::src_loc = "188:3"]
    pub const LLVMGhostLinkage: LLVMLinkage = 13;
    #[c2rust::src_loc = "187:3"]
    pub const LLVMExternalWeakLinkage: LLVMLinkage = 12;
    #[c2rust::src_loc = "186:3"]
    pub const LLVMDLLExportLinkage: LLVMLinkage = 11;
    #[c2rust::src_loc = "185:3"]
    pub const LLVMDLLImportLinkage: LLVMLinkage = 10;
    #[c2rust::src_loc = "184:3"]
    pub const LLVMPrivateLinkage: LLVMLinkage = 9;
    #[c2rust::src_loc = "182:3"]
    pub const LLVMInternalLinkage: LLVMLinkage = 8;
    #[c2rust::src_loc = "181:3"]
    pub const LLVMAppendingLinkage: LLVMLinkage = 7;
    #[c2rust::src_loc = "179:3"]
    pub const LLVMWeakODRLinkage: LLVMLinkage = 6;
    #[c2rust::src_loc = "178:3"]
    pub const LLVMWeakAnyLinkage: LLVMLinkage = 5;
    #[c2rust::src_loc = "177:3"]
    pub const LLVMLinkOnceODRAutoHideLinkage: LLVMLinkage = 4;
    #[c2rust::src_loc = "175:3"]
    pub const LLVMLinkOnceODRLinkage: LLVMLinkage = 3;
    #[c2rust::src_loc = "174:3"]
    pub const LLVMLinkOnceAnyLinkage: LLVMLinkage = 2;
    #[c2rust::src_loc = "173:3"]
    pub const LLVMAvailableExternallyLinkage: LLVMLinkage = 1;
    #[c2rust::src_loc = "172:3"]
    pub const LLVMExternalLinkage: LLVMLinkage = 0;
    #[c2rust::src_loc = "212:9"]
    pub type LLVMCallConv = libc::c_uint;
    #[c2rust::src_loc = "254:3"]
    pub const LLVMAMDGPUESCallConv: LLVMCallConv = 96;
    #[c2rust::src_loc = "253:3"]
    pub const LLVMAMDGPULSCallConv: LLVMCallConv = 95;
    #[c2rust::src_loc = "252:3"]
    pub const LLVMMSP430BUILTINCallConv: LLVMCallConv = 94;
    #[c2rust::src_loc = "251:3"]
    pub const LLVMAMDGPUHSCallConv: LLVMCallConv = 93;
    #[c2rust::src_loc = "250:3"]
    pub const LLVMX86RegCallCallConv: LLVMCallConv = 92;
    #[c2rust::src_loc = "249:3"]
    pub const LLVMAMDGPUKERNELCallConv: LLVMCallConv = 91;
    #[c2rust::src_loc = "248:3"]
    pub const LLVMAMDGPUCSCallConv: LLVMCallConv = 90;
    #[c2rust::src_loc = "247:3"]
    pub const LLVMAMDGPUPSCallConv: LLVMCallConv = 89;
    #[c2rust::src_loc = "246:3"]
    pub const LLVMAMDGPUGSCallConv: LLVMCallConv = 88;
    #[c2rust::src_loc = "245:3"]
    pub const LLVMAMDGPUVSCallConv: LLVMCallConv = 87;
    #[c2rust::src_loc = "244:3"]
    pub const LLVMAVRBUILTINCallConv: LLVMCallConv = 86;
    #[c2rust::src_loc = "243:3"]
    pub const LLVMAVRSIGNALCallConv: LLVMCallConv = 85;
    #[c2rust::src_loc = "242:3"]
    pub const LLVMAVRINTRCallConv: LLVMCallConv = 84;
    #[c2rust::src_loc = "241:3"]
    pub const LLVMX86INTRCallConv: LLVMCallConv = 83;
    #[c2rust::src_loc = "240:3"]
    pub const LLVMHHVMCCallConv: LLVMCallConv = 82;
    #[c2rust::src_loc = "239:3"]
    pub const LLVMHHVMCallConv: LLVMCallConv = 81;
    #[c2rust::src_loc = "238:3"]
    pub const LLVMX86VectorCallCallConv: LLVMCallConv = 80;
    #[c2rust::src_loc = "237:3"]
    pub const LLVMWin64CallConv: LLVMCallConv = 79;
    #[c2rust::src_loc = "236:3"]
    pub const LLVMX8664SysVCallConv: LLVMCallConv = 78;
    #[c2rust::src_loc = "235:3"]
    pub const LLVMIntelOCLBICallConv: LLVMCallConv = 77;
    #[c2rust::src_loc = "234:3"]
    pub const LLVMSPIRKERNELCallConv: LLVMCallConv = 76;
    #[c2rust::src_loc = "233:3"]
    pub const LLVMSPIRFUNCCallConv: LLVMCallConv = 75;
    #[c2rust::src_loc = "232:3"]
    pub const LLVMPTXDeviceCallConv: LLVMCallConv = 72;
    #[c2rust::src_loc = "231:3"]
    pub const LLVMPTXKernelCallConv: LLVMCallConv = 71;
    #[c2rust::src_loc = "230:3"]
    pub const LLVMX86ThisCallCallConv: LLVMCallConv = 70;
    #[c2rust::src_loc = "229:3"]
    pub const LLVMMSP430INTRCallConv: LLVMCallConv = 69;
    #[c2rust::src_loc = "228:3"]
    pub const LLVMARMAAPCSVFPCallConv: LLVMCallConv = 68;
    #[c2rust::src_loc = "227:3"]
    pub const LLVMARMAAPCSCallConv: LLVMCallConv = 67;
    #[c2rust::src_loc = "226:3"]
    pub const LLVMARMAPCSCallConv: LLVMCallConv = 66;
    #[c2rust::src_loc = "225:3"]
    pub const LLVMX86FastcallCallConv: LLVMCallConv = 65;
    #[c2rust::src_loc = "224:3"]
    pub const LLVMX86StdcallCallConv: LLVMCallConv = 64;
    #[c2rust::src_loc = "223:3"]
    pub const LLVMCXXFASTTLSCallConv: LLVMCallConv = 17;
    #[c2rust::src_loc = "222:3"]
    pub const LLVMSwiftCallConv: LLVMCallConv = 16;
    #[c2rust::src_loc = "221:3"]
    pub const LLVMPreserveAllCallConv: LLVMCallConv = 15;
    #[c2rust::src_loc = "220:3"]
    pub const LLVMPreserveMostCallConv: LLVMCallConv = 14;
    #[c2rust::src_loc = "219:3"]
    pub const LLVMAnyRegCallConv: LLVMCallConv = 13;
    #[c2rust::src_loc = "218:3"]
    pub const LLVMWebKitJSCallConv: LLVMCallConv = 12;
    #[c2rust::src_loc = "217:3"]
    pub const LLVMHiPECallConv: LLVMCallConv = 11;
    #[c2rust::src_loc = "216:3"]
    pub const LLVMGHCCallConv: LLVMCallConv = 10;
    #[c2rust::src_loc = "215:3"]
    pub const LLVMColdCallConv: LLVMCallConv = 9;
    #[c2rust::src_loc = "214:3"]
    pub const LLVMFastCallConv: LLVMCallConv = 8;
    #[c2rust::src_loc = "213:3"]
    pub const LLVMCCallConv: LLVMCallConv = 0;
    #[c2rust::src_loc = "290:9"]
    pub type LLVMIntPredicate = libc::c_uint;
    #[c2rust::src_loc = "300:3"]
    pub const LLVMIntSLE: LLVMIntPredicate = 41;
    #[c2rust::src_loc = "299:3"]
    pub const LLVMIntSLT: LLVMIntPredicate = 40;
    #[c2rust::src_loc = "298:3"]
    pub const LLVMIntSGE: LLVMIntPredicate = 39;
    #[c2rust::src_loc = "297:3"]
    pub const LLVMIntSGT: LLVMIntPredicate = 38;
    #[c2rust::src_loc = "296:3"]
    pub const LLVMIntULE: LLVMIntPredicate = 37;
    #[c2rust::src_loc = "295:3"]
    pub const LLVMIntULT: LLVMIntPredicate = 36;
    #[c2rust::src_loc = "294:3"]
    pub const LLVMIntUGE: LLVMIntPredicate = 35;
    #[c2rust::src_loc = "293:3"]
    pub const LLVMIntUGT: LLVMIntPredicate = 34;
    #[c2rust::src_loc = "292:3"]
    pub const LLVMIntNE: LLVMIntPredicate = 33;
    #[c2rust::src_loc = "291:3"]
    pub const LLVMIntEQ: LLVMIntPredicate = 32;
    use super::Types_h::{
        LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMTypeRef, LLVMValueRef,
    };
    extern "C" {
        #[c2rust::src_loc = "1106:1"]
        pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
        #[c2rust::src_loc = "1669:1"]
        pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1934:1"]
        pub fn LLVMConstInt(
            IntTy: LLVMTypeRef,
            N: libc::c_ulonglong,
            SignExtend: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3112:1"]
        pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3685:1"]
        pub fn LLVMBuildBr(_: LLVMBuilderRef, Dest: LLVMBasicBlockRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3686:1"]
        pub fn LLVMBuildCondBr(
            _: LLVMBuilderRef,
            If: LLVMValueRef,
            Then: LLVMBasicBlockRef,
            Else: LLVMBasicBlockRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3957:1"]
        pub fn LLVMBuildBitCast(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3982:1"]
        pub fn LLVMBuildICmp(
            _: LLVMBuilderRef,
            Op: LLVMIntPredicate,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "4012:1"]
        pub fn LLVMBuildExtractValue(
            _: LLVMBuilderRef,
            AggVal: LLVMValueRef,
            Index: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "4020:1"]
        pub fn LLVMBuildIsNull(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Target.h:1"]
pub mod Target_h {
    #[c2rust::src_loc = "37:1"]
    pub type LLVMTargetDataRef = *mut LLVMOpaqueTargetData;
    extern "C" {
        #[c2rust::src_loc = "37:16"]
        pub type LLVMOpaqueTargetData;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/TargetMachine.h:1"]
pub mod TargetMachine_h {
    #[c2rust::src_loc = "34:1"]
    pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type LLVMOpaqueTargetMachine;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct errormsg_t {
        pub file: *const libc::c_char,
        pub line: size_t,
        pub pos: size_t,
        pub msg: *const libc::c_char,
        pub source: *const libc::c_char,
        pub frame: *mut errormsg_t,
        pub next: *mut errormsg_t,
    }
    #[c2rust::src_loc = "49:1"]
    pub type errorframe_t = *mut errormsg_t;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed = 0;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = size_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: size_t,
        pub size: size_t,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    use super::_size_t_h::size_t;
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
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
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
        pub ast_print_width: size_t,
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
    extern "C" {
        #[c2rust::src_loc = "269:16"]
        pub type magic_package_t;
        #[c2rust::src_loc = "270:16"]
        pub type plugins_t;
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
        pub vtable_index: uint32_t,
        pub intrinsic: bool,
        pub internal: bool,
        pub forwarding: bool,
        pub subordinate: *mut reach_method_t,
        pub param_count: size_t,
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
        pub type_id: uint32_t,
        pub vtable_size: uint32_t,
        pub can_be_boxed: bool,
        pub is_trait: bool,
        pub field_count: uint32_t,
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
    #[c2rust::src_loc = "23:40"]
    pub struct reach_types_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:16"]
    pub struct reach_t {
        pub types: reach_types_t,
        pub method_stack: *mut reach_method_stack_t,
        pub object_type_count: uint32_t,
        pub numeric_type_count: uint32_t,
        pub tuple_type_count: uint32_t,
        pub total_type_count: uint32_t,
        pub trait_type_count: uint32_t,
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::hash_h::hashmap_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "18:35"]
        pub type reach_method_stack_t;
        #[c2rust::src_loc = "136:1"]
        pub fn reach_type(r: *mut reach_t, type_0: *mut ast_t) -> *mut reach_type_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/codegen.h:1"]
pub mod codegen_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:33"]
    pub struct genned_strings_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:33"]
    pub struct compile_locals_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:28"]
    pub struct ffi_decls_t {
        pub contents: hashmap_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct compile_frame_t {
        pub fun: LLVMValueRef,
        pub ctx: LLVMValueRef,
        pub break_target: LLVMBasicBlockRef,
        pub break_novalue_target: LLVMBasicBlockRef,
        pub continue_target: LLVMBasicBlockRef,
        pub invoke_target: LLVMBasicBlockRef,
        pub locals: compile_locals_t,
        pub di_file: LLVMMetadataRef,
        pub di_scope: LLVMMetadataRef,
        pub is_function: bool,
        pub early_termination: bool,
        pub bare_function: bool,
        pub reify: *mut deferred_reification_t,
        pub prev: *mut compile_frame_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:16"]
    pub struct compile_t {
        pub opt: *mut pass_opt_t,
        pub reach: *mut reach_t,
        pub strings: genned_strings_t,
        pub ffi_decls: ffi_decls_t,
        pub filename: *const libc::c_char,
        pub str_builtin: *const libc::c_char,
        pub str_Bool: *const libc::c_char,
        pub str_I8: *const libc::c_char,
        pub str_I16: *const libc::c_char,
        pub str_I32: *const libc::c_char,
        pub str_I64: *const libc::c_char,
        pub str_I128: *const libc::c_char,
        pub str_ILong: *const libc::c_char,
        pub str_ISize: *const libc::c_char,
        pub str_U8: *const libc::c_char,
        pub str_U16: *const libc::c_char,
        pub str_U32: *const libc::c_char,
        pub str_U64: *const libc::c_char,
        pub str_U128: *const libc::c_char,
        pub str_ULong: *const libc::c_char,
        pub str_USize: *const libc::c_char,
        pub str_F32: *const libc::c_char,
        pub str_F64: *const libc::c_char,
        pub str_Pointer: *const libc::c_char,
        pub str_NullablePointer: *const libc::c_char,
        pub str_DoNotOptimise: *const libc::c_char,
        pub str_Array: *const libc::c_char,
        pub str_String: *const libc::c_char,
        pub str_Platform: *const libc::c_char,
        pub str_Main: *const libc::c_char,
        pub str_Env: *const libc::c_char,
        pub str_add: *const libc::c_char,
        pub str_sub: *const libc::c_char,
        pub str_mul: *const libc::c_char,
        pub str_div: *const libc::c_char,
        pub str_rem: *const libc::c_char,
        pub str_neg: *const libc::c_char,
        pub str_add_unsafe: *const libc::c_char,
        pub str_sub_unsafe: *const libc::c_char,
        pub str_mul_unsafe: *const libc::c_char,
        pub str_div_unsafe: *const libc::c_char,
        pub str_rem_unsafe: *const libc::c_char,
        pub str_neg_unsafe: *const libc::c_char,
        pub str_and: *const libc::c_char,
        pub str_or: *const libc::c_char,
        pub str_xor: *const libc::c_char,
        pub str_not: *const libc::c_char,
        pub str_shl: *const libc::c_char,
        pub str_shr: *const libc::c_char,
        pub str_shl_unsafe: *const libc::c_char,
        pub str_shr_unsafe: *const libc::c_char,
        pub str_eq: *const libc::c_char,
        pub str_ne: *const libc::c_char,
        pub str_lt: *const libc::c_char,
        pub str_le: *const libc::c_char,
        pub str_ge: *const libc::c_char,
        pub str_gt: *const libc::c_char,
        pub str_eq_unsafe: *const libc::c_char,
        pub str_ne_unsafe: *const libc::c_char,
        pub str_lt_unsafe: *const libc::c_char,
        pub str_le_unsafe: *const libc::c_char,
        pub str_ge_unsafe: *const libc::c_char,
        pub str_gt_unsafe: *const libc::c_char,
        pub str_this: *const libc::c_char,
        pub str_create: *const libc::c_char,
        pub str__create: *const libc::c_char,
        pub str__init: *const libc::c_char,
        pub str__final: *const libc::c_char,
        pub str__event_notify: *const libc::c_char,
        pub str__serialise_space: *const libc::c_char,
        pub str__serialise: *const libc::c_char,
        pub str__deserialise: *const libc::c_char,
        pub trait_bitmap_size: uint32_t,
        pub callconv: LLVMCallConv,
        pub linkage: LLVMLinkage,
        pub context: LLVMContextRef,
        pub machine: LLVMTargetMachineRef,
        pub target_data: LLVMTargetDataRef,
        pub module: LLVMModuleRef,
        pub builder: LLVMBuilderRef,
        pub di: LLVMDIBuilderRef,
        pub di_unit: LLVMMetadataRef,
        pub none_instance: LLVMValueRef,
        pub primitives_init: LLVMValueRef,
        pub primitives_final: LLVMValueRef,
        pub desc_table: LLVMValueRef,
        pub numeric_sizes: LLVMValueRef,
        pub void_type: LLVMTypeRef,
        pub i1: LLVMTypeRef,
        pub i8_0: LLVMTypeRef,
        pub i16_0: LLVMTypeRef,
        pub i32_0: LLVMTypeRef,
        pub i64_0: LLVMTypeRef,
        pub i128_0: LLVMTypeRef,
        pub f32_0: LLVMTypeRef,
        pub f64_0: LLVMTypeRef,
        pub intptr: LLVMTypeRef,
        pub void_ptr: LLVMTypeRef,
        pub descriptor_type: LLVMTypeRef,
        pub descriptor_ptr: LLVMTypeRef,
        pub field_descriptor: LLVMTypeRef,
        pub object_type: LLVMTypeRef,
        pub object_ptr: LLVMTypeRef,
        pub msg_type: LLVMTypeRef,
        pub msg_ptr: LLVMTypeRef,
        pub actor_pad: LLVMTypeRef,
        pub trace_type: LLVMTypeRef,
        pub trace_fn: LLVMTypeRef,
        pub serialise_type: LLVMTypeRef,
        pub serialise_fn: LLVMTypeRef,
        pub dispatch_type: LLVMTypeRef,
        pub dispatch_fn: LLVMTypeRef,
        pub final_fn: LLVMTypeRef,
        pub custom_serialise_space_fn: LLVMTypeRef,
        pub custom_deserialise_fn: LLVMTypeRef,
        pub personality: LLVMValueRef,
        pub frame: *mut compile_frame_t,
    }
    use super::hash_h::hashmap_t;
    use super::pass_h::pass_opt_t;
    use super::reach_h::reach_t;
    use super::reify_h::deferred_reification_t;
    use super::Core_h::{LLVMCallConv, LLVMLinkage};
    use super::TargetMachine_h::LLVMTargetMachineRef;
    use super::Target_h::LLVMTargetDataRef;
    use super::Types_h::{
        LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
        LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
    };
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "289:1"]
        pub fn codegen_block(c: *mut compile_t, name: *const libc::c_char) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "246:1"]
        pub fn codegen_addfun(
            c: *mut compile_t,
            name: *const libc::c_char,
            type_0: LLVMTypeRef,
            pony_abi: bool,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.h:4"]
pub mod gentype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct compile_type_t {
        pub free_fn: compile_opaque_free_fn,
        pub abi_size: size_t,
        pub structure: LLVMTypeRef,
        pub structure_ptr: LLVMTypeRef,
        pub primitive: LLVMTypeRef,
        pub use_type: LLVMTypeRef,
        pub mem_type: LLVMTypeRef,
        pub desc_type: LLVMTypeRef,
        pub desc: LLVMValueRef,
        pub instance: LLVMValueRef,
        pub trace_fn: LLVMValueRef,
        pub serialise_trace_fn: LLVMValueRef,
        pub serialise_fn: LLVMValueRef,
        pub deserialise_fn: LLVMValueRef,
        pub custom_serialise_space_fn: LLVMValueRef,
        pub custom_serialise_fn: LLVMValueRef,
        pub custom_deserialise_fn: LLVMValueRef,
        pub final_fn: LLVMValueRef,
        pub dispatch_fn: LLVMValueRef,
        pub dispatch_switch: LLVMValueRef,
        pub di_file: LLVMMetadataRef,
        pub di_type: LLVMMetadataRef,
        pub di_type_embed: LLVMMetadataRef,
    }
    use super::reach_h::compile_opaque_free_fn;
    use super::Types_h::{LLVMMetadataRef, LLVMTypeRef, LLVMValueRef};
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/matchtype.h:8"]
pub mod matchtype_h {
    #[c2rust::src_loc = "13:3"]
    pub const MATCHTYPE_ACCEPT: matchtype_t = 0;
    #[c2rust::src_loc = "11:9"]
    pub type matchtype_t = libc::c_uint;
    #[c2rust::src_loc = "16:3"]
    pub const MATCHTYPE_DENY_NODESC: matchtype_t = 3;
    #[c2rust::src_loc = "15:3"]
    pub const MATCHTYPE_DENY_CAP: matchtype_t = 2;
    #[c2rust::src_loc = "14:3"]
    pub const MATCHTYPE_REJECT: matchtype_t = 1;
    use super::error_h::errorframe_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn is_matchtype(
            operand: *mut ast_t,
            pattern: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> matchtype_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::_size_t_h::size_t;
    use super::symtab_h::ast_t;
    use super::token_h::{token_id};
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn ast_from(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "63:1"]
        pub fn ast_dup(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "139:1"]
        pub fn ast_append(parent: *mut ast_t, child: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "142:1"]
        pub fn ast_swap(prev: *mut ast_t, next: *mut ast_t);
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.h:2"]
pub mod gencall_h {
    use super::codegen_h::compile_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn gencall_runtime(
            c: *mut compile_t,
            name: *const libc::c_char,
            args: *mut LLVMValueRef,
            count: libc::c_int,
            ret: *const libc::c_char,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.h:3"]
pub mod gendesc_h {
    use super::codegen_h::compile_t;
    use super::Types_h::{LLVMValueRef};
    use super::_size_t_h::size_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn gendesc_fetch(c: *mut compile_t, object: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "29:1"]
        pub fn gendesc_ptr_to_fields(
            c: *mut compile_t,
            object: LLVMValueRef,
            desc: LLVMValueRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "32:1"]
        pub fn gendesc_fieldcount(c: *mut compile_t, desc: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "34:1"]
        pub fn gendesc_fieldinfo(
            c: *mut compile_t,
            desc: LLVMValueRef,
            index: size_t,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "36:1"]
        pub fn gendesc_fieldptr(
            c: *mut compile_t,
            ptr: LLVMValueRef,
            field_info: LLVMValueRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "39:1"]
        pub fn gendesc_fieldload(
            c: *mut compile_t,
            ptr: LLVMValueRef,
            field_info: LLVMValueRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "42:1"]
        pub fn gendesc_fielddesc(c: *mut compile_t, field_info: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "44:1"]
        pub fn gendesc_isnominal(
            c: *mut compile_t,
            desc: LLVMValueRef,
            trait_0: *mut ast_t,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:5"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn genname_trace(type_0: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:7"]
pub mod cap_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn cap_fetch(type_0: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "62:1"]
        pub fn cap_single(type_0: *mut ast_t) -> token_id;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:9"]
pub mod subtype_h {
    use super::error_h::errorframe_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn is_subtype(
            sub: *mut ast_t,
            super_0: *mut ast_t,
            errorf: *mut errorframe_t,
            opt: *mut pass_opt_t,
        ) -> bool;
        #[c2rust::src_loc = "34:1"]
        pub fn is_nullable_pointer(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "48:1"]
        pub fn is_machine_word(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "56:1"]
        pub fn is_known(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "58:1"]
        pub fn is_bare(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:10"]
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
use self::ast_h::{
    ast_append, ast_child, ast_childcount, ast_childidx, ast_data, ast_dup, ast_free_unattached,
    ast_from, ast_id, ast_setid, ast_sibling, ast_swap,
};
use self::cap_h::{cap_fetch, cap_single};
pub use self::codegen_h::{
    codegen_addfun, codegen_block, compile_frame_t, compile_locals_t, compile_t, ffi_decls_t,
    genned_strings_t,
};
pub use self::error_h::{errorframe_t, errormsg_t, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::gencall_h::gencall_runtime;
use self::gendesc_h::{
    gendesc_fetch, gendesc_fieldcount, gendesc_fielddesc, gendesc_fieldinfo, gendesc_fieldload,
    gendesc_fieldptr, gendesc_isnominal, gendesc_ptr_to_fields,
};
use self::genname_h::genname_trace;
pub use self::gentype_h::compile_type_t;
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::matchtype_h::{
    is_matchtype, matchtype_t, MATCHTYPE_ACCEPT, MATCHTYPE_DENY_CAP, MATCHTYPE_DENY_NODESC,
    MATCHTYPE_REJECT,
};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
pub use self::pony_h::{
    C2RustUnnamed, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_method_names_t,
    reach_method_stack_t, reach_method_t, reach_param_t, reach_t, reach_type, reach_type_cache_t,
    reach_type_t, reach_types_t,
};
pub use self::reify_h::deferred_reification_t;

use self::subtype_h::{is_bare, is_known, is_machine_word, is_nullable_pointer, is_subtype};
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
pub use self::Core_h::{
    LLVMAMDGPUCSCallConv, LLVMAMDGPUESCallConv, LLVMAMDGPUGSCallConv, LLVMAMDGPUHSCallConv,
    LLVMAMDGPUKERNELCallConv, LLVMAMDGPULSCallConv, LLVMAMDGPUPSCallConv, LLVMAMDGPUVSCallConv,
    LLVMARMAAPCSCallConv, LLVMARMAAPCSVFPCallConv, LLVMARMAPCSCallConv, LLVMAVRBUILTINCallConv,
    LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAnyRegCallConv, LLVMAppendingLinkage,
    LLVMArrayTypeKind, LLVMAvailableExternallyLinkage, LLVMBFloatTypeKind, LLVMBuildBitCast,
    LLVMBuildBr, LLVMBuildCondBr, LLVMBuildExtractValue, LLVMBuildICmp, LLVMBuildIsNull,
    LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv, LLVMColdCallConv, LLVMCommonLinkage,
    LLVMConstInt, LLVMDLLExportLinkage, LLVMDLLImportLinkage, LLVMDoubleTypeKind,
    LLVMExternalLinkage, LLVMExternalWeakLinkage, LLVMFP128TypeKind, LLVMFastCallConv,
    LLVMFloatTypeKind, LLVMFunctionTypeKind, LLVMGHCCallConv, LLVMGetInsertBlock, LLVMGetTypeKind,
    LLVMGhostLinkage, LLVMHHVMCCallConv, LLVMHHVMCallConv, LLVMHalfTypeKind, LLVMHiPECallConv,
    LLVMIntEQ, LLVMIntNE, LLVMIntPredicate, LLVMIntSGE, LLVMIntSGT, LLVMIntSLE, LLVMIntSLT,
    LLVMIntUGE, LLVMIntUGT, LLVMIntULE, LLVMIntULT, LLVMIntegerTypeKind, LLVMIntelOCLBICallConv,
    LLVMInternalLinkage, LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage, LLVMLinkOnceODRAutoHideLinkage,
    LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage, LLVMLinkerPrivateWeakLinkage,
    LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv, LLVMMetadataTypeKind,
    LLVMMoveBasicBlockAfter, LLVMPPC_FP128TypeKind, LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv,
    LLVMPointerTypeKind, LLVMPositionBuilderAtEnd, LLVMPreserveAllCallConv,
    LLVMPreserveMostCallConv, LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv,
    LLVMScalableVectorTypeKind, LLVMStructTypeKind, LLVMSwiftCallConv, LLVMTokenTypeKind,
    LLVMTypeKind, LLVMTypeOf, LLVMVectorTypeKind, LLVMVoidTypeKind, LLVMWeakAnyLinkage,
    LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv, LLVMX8664SysVCallConv,
    LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv, LLVMX86StdcallCallConv,
    LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv, LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind,
    LLVMX86_MMXTypeKind,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{LLVMOpaqueTargetData, LLVMTargetDataRef};
pub use self::Types_h::{
    LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    LLVMModuleRef, LLVMOpaqueBasicBlock, LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder,
    LLVMOpaqueMetadata, LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef,
    LLVMValueRef,
};
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
#[c2rust::src_loc = "27:3"]
pub const TRACE_TUPLE: trace_t = 12;
#[c2rust::src_loc = "18:3"]
pub const TRACE_PRIMITIVE: trace_t = 3;
#[c2rust::src_loc = "17:3"]
pub const TRACE_MACHINE_WORD: trace_t = 2;
#[c2rust::src_loc = "15:3"]
pub const TRACE_NONE: trace_t = 0;
#[c2rust::src_loc = "13:9"]
pub type trace_t = libc::c_uint;
#[c2rust::src_loc = "26:3"]
pub const TRACE_DYNAMIC: trace_t = 11;
#[c2rust::src_loc = "25:3"]
pub const TRACE_STATIC: trace_t = 10;
#[c2rust::src_loc = "24:3"]
pub const TRACE_TAG_UNKNOWN: trace_t = 9;
#[c2rust::src_loc = "23:3"]
pub const TRACE_TAG_KNOWN: trace_t = 8;
#[c2rust::src_loc = "22:3"]
pub const TRACE_MUT_UNKNOWN: trace_t = 7;
#[c2rust::src_loc = "21:3"]
pub const TRACE_MUT_KNOWN: trace_t = 6;
#[c2rust::src_loc = "20:3"]
pub const TRACE_VAL_UNKNOWN: trace_t = 5;
#[c2rust::src_loc = "19:3"]
pub const TRACE_VAL_KNOWN: trace_t = 4;
#[c2rust::src_loc = "16:3"]
pub const TRACE_NULLABLE_POINTER: trace_t = 1;
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn trace_union_machine_word(mut a: trace_t) -> trace_t {
    match a as libc::c_uint {
        2 | 3 | 4 | 5 => return TRACE_VAL_UNKNOWN,
        6 | 7 => return TRACE_MUT_UNKNOWN,
        8 | 9 => return TRACE_TAG_UNKNOWN,
        11 | 12 => return TRACE_DYNAMIC,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            71 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"trace_union_machine_word\0",
            ))
            .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn trace_union_primitive(mut a: trace_t) -> trace_t {
    match a as libc::c_uint {
        3 => return TRACE_PRIMITIVE,
        2 | 4 | 5 => return TRACE_VAL_UNKNOWN,
        6 | 7 => return TRACE_MUT_UNKNOWN,
        8 | 9 => return TRACE_TAG_UNKNOWN,
        11 | 12 => return TRACE_DYNAMIC,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            102 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"trace_union_primitive\0"))
                .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn trace_union_val(mut a: trace_t) -> trace_t {
    match a as libc::c_uint {
        2 | 3 | 4 | 5 => return TRACE_VAL_UNKNOWN,
        6 | 7 | 8 | 9 | 11 | 12 => return TRACE_DYNAMIC,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            127 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"trace_union_val\0"))
                .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "131:1"]
unsafe extern "C" fn trace_union_mut(mut a: trace_t) -> trace_t {
    match a as libc::c_uint {
        2 | 3 | 6 | 7 => return TRACE_MUT_UNKNOWN,
        4 | 5 | 8 | 9 | 11 | 12 => return TRACE_DYNAMIC,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            152 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"trace_union_mut\0"))
                .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn trace_union_tag(mut a: trace_t) -> trace_t {
    match a as libc::c_uint {
        2 | 3 | 8 | 9 => return TRACE_TAG_UNKNOWN,
        4 | 5 | 6 | 7 | 11 | 12 => return TRACE_DYNAMIC,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            177 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"trace_union_tag\0"))
                .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn trace_type_union(mut type_0: *mut ast_t) -> trace_t {
    let mut trace: trace_t = TRACE_NONE;
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        let mut t: trace_t = trace_type(child);
        match trace as libc::c_uint {
            0 => {
                trace = t;
            }
            1 => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                            as *const u8 as *const libc::c_char,
                        199 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"trace_type_union\0"))
                            .as_ptr(),
                    );
                };
                return TRACE_NONE;
            }
            2 => {
                trace = trace_union_machine_word(t);
            }
            3 => {
                trace = trace_union_primitive(t);
            }
            6 | 7 => {
                trace = trace_union_mut(t);
            }
            4 | 5 => {
                trace = trace_union_val(t);
            }
            8 | 9 => {
                trace = trace_union_tag(t);
            }
            11 | 12 => return TRACE_DYNAMIC,
            10 => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                            as *const u8 as *const libc::c_char,
                        231 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"trace_type_union\0"))
                            .as_ptr(),
                    );
                };
                return TRACE_DYNAMIC;
            }
            _ => {}
        }
        child = ast_sibling(child);
    }
    trace
}
#[c2rust::src_loc = "239:1"]
unsafe extern "C" fn trace_type_isect(mut type_0: *mut ast_t) -> trace_t {
    let mut trace: trace_t = TRACE_DYNAMIC;
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        let mut t: trace_t = trace_type(child);
        match t as libc::c_uint {
            0 | 1 => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                            as *const u8 as *const libc::c_char,
                        254 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"trace_type_isect\0"))
                            .as_ptr(),
                    );
                };
                return TRACE_NONE;
            }
            3 => return TRACE_PRIMITIVE,
            2 | 4 | 5 => {
                trace = TRACE_VAL_UNKNOWN;
            }
            6 | 7 => {
                if trace as libc::c_uint != TRACE_VAL_UNKNOWN as libc::c_int as libc::c_uint {
                    trace = TRACE_MUT_UNKNOWN;
                }
            }
            8 | 9 => {
                if trace as libc::c_uint != TRACE_MUT_UNKNOWN as libc::c_int as libc::c_uint
                    && trace as libc::c_uint != TRACE_VAL_UNKNOWN as libc::c_int as libc::c_uint
                {
                    trace = TRACE_TAG_UNKNOWN;
                }
            }
            10 => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                            as *const u8 as *const libc::c_char,
                        284 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"trace_type_isect\0"))
                            .as_ptr(),
                    );
                };
            }
            11 | 12 | _ => {}
        }
        child = ast_sibling(child);
    }
    trace
}
#[c2rust::src_loc = "292:1"]
unsafe extern "C" fn trace_type_nominal(mut type_0: *mut ast_t) -> trace_t {
    if is_bare(type_0) {
        return TRACE_PRIMITIVE;
    }
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    match ast_id(def) as libc::c_uint {
        72 | 73 => {
            match cap_single(type_0) as libc::c_uint {
                94 => return TRACE_VAL_UNKNOWN,
                96 => return TRACE_TAG_UNKNOWN,
                _ => {}
            }
            return TRACE_MUT_UNKNOWN;
        }
        74 => {
            if is_machine_word(type_0) {
                return TRACE_MACHINE_WORD;
            }
            return TRACE_PRIMITIVE;
        }
        75 | 76 => {
            if is_nullable_pointer(type_0) {
                return TRACE_NULLABLE_POINTER;
            }
            match cap_single(type_0) as libc::c_uint {
                94 => return TRACE_VAL_KNOWN,
                96 => return TRACE_TAG_KNOWN,
                _ => {}
            }
            return TRACE_MUT_KNOWN;
        }
        77 => return TRACE_TAG_KNOWN,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            347 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"trace_type_nominal\0"))
                .as_ptr(),
        );
    };
    TRACE_NONE
}
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn trace_type(mut type_0: *mut ast_t) -> trace_t {
    match ast_id(type_0) as libc::c_uint {
        149 => return trace_type_union(type_0),
        56 => return trace_type_isect(type_0),
        150 => return TRACE_TUPLE,
        151 => return trace_type_nominal(type_0),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            370 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"trace_type\0")).as_ptr(),
        );
    };
    TRACE_DYNAMIC
}
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn trace_type_dst_cap(
    mut src_trace: trace_t,
    mut dst_trace: trace_t,
    mut src_type: *mut ast_t,
) -> trace_t {
    match src_trace as libc::c_uint {
        0 | 2 | 1 | 3 | 11 | 8 | 9 => return src_trace,
        12 => {
            if dst_trace as libc::c_uint == TRACE_TUPLE as libc::c_int as libc::c_uint {
                return TRACE_TUPLE;
            }
            TRACE_DYNAMIC
        }
        4 => match dst_trace as libc::c_uint {
            11 => return TRACE_STATIC,
            8 | 9 => return TRACE_TAG_KNOWN,
            _ => return TRACE_VAL_KNOWN,
        },
        5 => match dst_trace as libc::c_uint {
            11 => {
                if ast_id(src_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
                    return TRACE_STATIC;
                }
                TRACE_DYNAMIC
            }
            9 => TRACE_TAG_UNKNOWN,
            _ => TRACE_VAL_UNKNOWN,
        },
        6 => match dst_trace as libc::c_uint {
            11 => TRACE_STATIC,
            8 | 9 => TRACE_TAG_KNOWN,
            4 | 5 => TRACE_VAL_KNOWN,
            _ => TRACE_MUT_KNOWN,
        },
        7 => match dst_trace as libc::c_uint {
            11 => {
                if ast_id(src_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
                    return TRACE_STATIC;
                }
                TRACE_DYNAMIC
            }
            9 => TRACE_TAG_UNKNOWN,
            5 => TRACE_VAL_UNKNOWN,
            _ => TRACE_MUT_UNKNOWN,
        },
        10 => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                        as *const u8 as *const libc::c_char,
                    461 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"trace_type_dst_cap\0",
                    ))
                    .as_ptr(),
                );
            };
            src_trace
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                        as *const u8 as *const libc::c_char,
                    465 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"trace_type_dst_cap\0",
                    ))
                    .as_ptr(),
                );
            };
            src_trace
        }
    }
}
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn trace_nullable_pointer(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut type_0: *mut ast_t,
) {
    let mut type_args: *mut ast_t = ast_childidx(type_0, 2 as libc::c_int as size_t);
    let mut elem: *mut ast_t = ast_child(type_args);
    let mut test: LLVMValueRef = LLVMBuildIsNull(
        (*c).builder,
        object,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_false: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut is_true: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    LLVMBuildCondBr((*c).builder, test, is_true, is_false);
    LLVMPositionBuilderAtEnd((*c).builder, is_false);
    gentrace(c, ctx, object, object, elem, elem);
    LLVMBuildBr((*c).builder, is_true);
    LLVMMoveBasicBlockAfter(is_true, LLVMGetInsertBlock((*c).builder));
    LLVMPositionBuilderAtEnd((*c).builder, is_true);
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn trace_known(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut type_0: *mut ast_t,
    mut mutability: libc::c_int,
) {
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    let mut args: [LLVMValueRef; 4] = [0 as *mut LLVMOpaqueValue; 4];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        object,
        (*c).object_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    args[2 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        (*((*t).c_type as *mut compile_type_t)).desc,
        (*c).descriptor_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    args[3 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        mutability as libc::c_ulonglong,
        0 as libc::c_int,
    );
    gencall_runtime(
        c,
        b"pony_traceknown\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        4 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "505:1"]
unsafe extern "C" fn trace_unknown(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut mutability: libc::c_int,
) {
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        object,
        (*c).object_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    args[2 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        mutability as libc::c_ulonglong,
        0 as libc::c_int,
    );
    gencall_runtime(
        c,
        b"pony_traceunknown\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "516:1"]
unsafe extern "C" fn trace_cap_nominal(
    mut opt: *mut pass_opt_t,
    mut type_0: *mut ast_t,
    mut orig: *mut ast_t,
    mut tuple: *mut ast_t,
) -> libc::c_int {
    if ast_id(type_0) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            519 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"trace_cap_nominal\0"))
                .as_ptr(),
        );
    };
    let mut cap: *mut ast_t = cap_fetch(type_0);
    if !tuple.is_null() {
        type_0 = tuple;
    }
    let mut orig_cap: token_id = ast_id(cap);
    match orig_cap as libc::c_uint {
        92 | 93 | 95 => return PONY_TRACE_MUTABLE as libc::c_int,
        _ => {}
    }
    if orig_cap as libc::c_uint == TK_ISO as libc::c_int as libc::c_uint {
        if is_matchtype(orig, type_0, 0 as *mut errorframe_t, opt) as libc::c_uint
            == MATCHTYPE_ACCEPT as libc::c_int as libc::c_uint
        {
            return PONY_TRACE_MUTABLE as libc::c_int;
        } else {
            ast_setid(cap, TK_VAL);
        }
    }
    if ast_id(cap) as libc::c_uint == TK_VAL as libc::c_int as libc::c_uint {
        if is_matchtype(orig, type_0, 0 as *mut errorframe_t, opt) as libc::c_uint
            == MATCHTYPE_ACCEPT as libc::c_int as libc::c_uint
        {
            ast_setid(cap, orig_cap);
            return PONY_TRACE_IMMUTABLE as libc::c_int;
        } else {
            ast_setid(cap, TK_TAG);
        }
    }
    if ast_id(cap) as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(cap) == TK_TAG\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            569 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"trace_cap_nominal\0"))
                .as_ptr(),
        );
    };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if is_matchtype(orig, type_0, 0 as *mut errorframe_t, opt) as libc::c_uint
        == MATCHTYPE_ACCEPT as libc::c_int as libc::c_uint
    {
        ret = PONY_TRACE_OPAQUE as libc::c_int;
    }
    ast_setid(cap, orig_cap);
    return ret;
}
#[c2rust::src_loc = "579:1"]
unsafe extern "C" fn trace_static(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut src_type: *mut ast_t,
    mut dst_type: *mut ast_t,
) {
    if ast_id(src_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(src_type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            582 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"trace_static\0")).as_ptr(),
        );
    };
    let mut mutability: libc::c_int =
        trace_cap_nominal((*c).opt, src_type, dst_type, 0 as *mut ast_t);
    if mutability != -(1 as libc::c_int) {
    } else {
        ponyint_assert_fail(
            b"mutability != -1\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            585 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"trace_static\0")).as_ptr(),
        );
    };
    if is_known(src_type) {
        trace_known(c, ctx, object, src_type, mutability);
    } else {
        trace_unknown(c, ctx, object, mutability);
    };
}
#[c2rust::src_loc = "593:1"]
unsafe extern "C" fn trace_tuple(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut src_value: LLVMValueRef,
    mut dst_value: LLVMValueRef,
    mut src_type: *mut ast_t,
    mut dst_type: *mut ast_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if ast_id(dst_type) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
        let mut src_child: *mut ast_t = ast_child(src_type);
        let mut dst_child: *mut ast_t = ast_child(dst_type);
        while !src_child.is_null() && !dst_child.is_null() {
            let mut elem: LLVMValueRef = LLVMBuildExtractValue(
                (*c).builder,
                dst_value,
                i as libc::c_uint,
                b"\0" as *const u8 as *const libc::c_char,
            );
            gentrace(c, ctx, elem, elem, src_child, dst_child);
            i += 1;
            src_child = ast_sibling(src_child);
            dst_child = ast_sibling(dst_child);
        }
        if src_child.is_null() && dst_child.is_null() {
        } else {
            ponyint_assert_fail(
                b"src_child == NULL && dst_child == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                    as *const u8 as *const libc::c_char,
                612 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"trace_tuple\0"))
                    .as_ptr(),
            );
        };
    } else {
        trace_unknown(c, ctx, dst_value, PONY_TRACE_OPAQUE as libc::c_int);
        if LLVMGetTypeKind(LLVMTypeOf(src_value)) as libc::c_uint
            == LLVMStructTypeKind as libc::c_int as libc::c_uint
        {
        } else {
            ponyint_assert_fail(
                b"LLVMGetTypeKind(LLVMTypeOf(src_value)) == LLVMStructTypeKind\0" as *const u8
                    as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                    as *const u8 as *const libc::c_char,
                618 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"trace_tuple\0"))
                    .as_ptr(),
            );
        };
        let mut src_child_0: *mut ast_t = ast_child(src_type);
        while !src_child_0.is_null() {
            let mut elem_0: LLVMValueRef = LLVMBuildExtractValue(
                (*c).builder,
                src_value,
                i as libc::c_uint,
                b"\0" as *const u8 as *const libc::c_char,
            );
            gentrace(c, ctx, elem_0, elem_0, src_child_0, src_child_0);
            i += 1;
            src_child_0 = ast_sibling(src_child_0);
        }
    };
}
#[c2rust::src_loc = "631:1"]
unsafe extern "C" fn trace_dynamic_union_or_isect(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut type_0: *mut ast_t,
    mut orig: *mut ast_t,
    mut tuple: *mut ast_t,
    mut next_block: LLVMBasicBlockRef,
) {
    let mut child: *mut ast_t = ast_child(type_0);
    while !child.is_null() {
        trace_dynamic(c, ctx, object, child, orig, tuple, next_block);
        child = ast_sibling(child);
    }
    trace_unknown(c, ctx, object, PONY_TRACE_OPAQUE as libc::c_int);
}
#[c2rust::src_loc = "648:1"]
unsafe extern "C" fn trace_dynamic_tuple(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut ptr: LLVMValueRef,
    mut desc: LLVMValueRef,
    mut type_0: *mut ast_t,
    mut orig: *mut ast_t,
    mut tuple: *mut ast_t,
) {
    let mut cardinality: size_t = ast_childcount(type_0);
    let mut dontcare: *mut ast_t = ast_from(type_0, TK_TUPLETYPE);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < cardinality {
        ast_append(dontcare, ast_from(type_0, TK_DONTCARETYPE));
        i = i.wrapping_add(1);
    }
    let mut in_tuple: bool = !tuple.is_null();
    if in_tuple {
        ast_swap(type_0, dontcare);
    } else {
        tuple = dontcare;
    }
    let mut is_true: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut is_false: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    if !is_subtype(orig, tuple, 0 as *mut errorframe_t, (*c).opt) {
        let mut dynamic_count: LLVMValueRef = gendesc_fieldcount(c, desc);
        let mut static_count: LLVMValueRef = LLVMConstInt(
            (*c).i32_0,
            cardinality as libc::c_ulonglong,
            0 as libc::c_int,
        );
        let mut test: LLVMValueRef = LLVMBuildICmp(
            (*c).builder,
            LLVMIntEQ,
            static_count,
            dynamic_count,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildCondBr((*c).builder, test, is_true, is_false);
    } else {
        LLVMBuildBr((*c).builder, is_true);
    }
    LLVMPositionBuilderAtEnd((*c).builder, is_true);
    let mut index: size_t = 0 as libc::c_int as size_t;
    let mut child: *mut ast_t = ast_child(type_0);
    let mut dc_child: *mut ast_t = ast_child(dontcare);
    while !child.is_null() {
        match trace_type(child) as libc::c_uint {
            6 | 7 | 4 | 5 | 8 | 9 | 11 => {
                let mut swap: *mut ast_t = ast_dup(child);
                ast_swap(dc_child, swap);
                let mut next_block: LLVMBasicBlockRef =
                    codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
                let mut field_info: LLVMValueRef = gendesc_fieldinfo(c, desc, index);
                let mut object: LLVMValueRef = gendesc_fieldload(c, ptr, field_info);
                trace_dynamic(c, ctx, object, swap, orig, tuple, next_block);
                LLVMBuildBr((*c).builder, next_block);
                LLVMPositionBuilderAtEnd((*c).builder, next_block);
                ast_swap(swap, dc_child);
                ast_free_unattached(swap);
            }
            12 => {
                let mut swap_0: *mut ast_t = ast_dup(child);
                ast_swap(dc_child, swap_0);
                let mut field_info_0: LLVMValueRef = gendesc_fieldinfo(c, desc, index);
                let mut field_ptr: LLVMValueRef = gendesc_fieldptr(c, ptr, field_info_0);
                let mut field_desc: LLVMValueRef = gendesc_fielddesc(c, field_info_0);
                trace_dynamic_tuple(c, ctx, field_ptr, field_desc, swap_0, orig, tuple);
                ast_swap(swap_0, dc_child);
                ast_free_unattached(swap_0);
            }
            2 | 3 | _ => {}
        }
        index = index.wrapping_add(1);
        child = ast_sibling(child);
        dc_child = ast_sibling(dc_child);
    }
    if in_tuple {
        ast_swap(dontcare, type_0);
    }
    ast_free_unattached(dontcare);
    LLVMBuildBr((*c).builder, is_false);
    LLVMMoveBasicBlockAfter(is_false, LLVMGetInsertBlock((*c).builder));
    LLVMPositionBuilderAtEnd((*c).builder, is_false);
}
#[c2rust::src_loc = "772:1"]
unsafe extern "C" fn trace_dynamic_nominal(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut type_0: *mut ast_t,
    mut orig: *mut ast_t,
    mut tuple: *mut ast_t,
    mut next_block: LLVMBasicBlockRef,
) {
    if ast_id(type_0) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                as *const u8 as *const libc::c_char,
            776 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"trace_dynamic_nominal\0"))
                .as_ptr(),
        );
    };
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    if ast_id(def) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint {
        return;
    }
    let mut mutability: libc::c_int = trace_cap_nominal((*c).opt, type_0, orig, tuple);
    if mutability == -(1 as libc::c_int) {
        return;
    }
    let mut dst_cap: token_id = TK_TAG;
    match mutability {
        0 => {
            dst_cap = TK_ISO;
        }
        1 => {
            dst_cap = TK_VAL;
        }
        _ => {}
    }
    let mut dst_type: *mut ast_t = ast_dup(type_0);
    let mut dst_cap_ast: *mut ast_t = cap_fetch(dst_type);
    ast_setid(dst_cap_ast, dst_cap);
    let mut dst_eph: *mut ast_t = ast_sibling(dst_cap_ast);
    if ast_id(dst_eph) as libc::c_uint == TK_EPHEMERAL as libc::c_int as libc::c_uint {
        ast_setid(dst_eph, TK_NONE);
    }
    let mut desc: LLVMValueRef = gendesc_fetch(c, object);
    let mut test: LLVMValueRef = gendesc_isnominal(c, desc, type_0);
    let mut is_true: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut is_false: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    LLVMBuildCondBr((*c).builder, test, is_true, is_false);
    LLVMPositionBuilderAtEnd((*c).builder, is_true);
    gentrace(c, ctx, object, object, type_0, dst_type);
    ast_free_unattached(dst_type);
    if mutability != PONY_TRACE_OPAQUE as libc::c_int {
        LLVMBuildBr((*c).builder, next_block);
    } else {
        LLVMBuildBr((*c).builder, is_false);
    }
    LLVMMoveBasicBlockAfter(is_false, LLVMGetInsertBlock((*c).builder));
    LLVMPositionBuilderAtEnd((*c).builder, is_false);
}
#[c2rust::src_loc = "837:1"]
unsafe extern "C" fn trace_dynamic(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut type_0: *mut ast_t,
    mut orig: *mut ast_t,
    mut tuple: *mut ast_t,
    mut next_block: LLVMBasicBlockRef,
) {
    match ast_id(type_0) as libc::c_uint {
        149 | 56 => {
            trace_dynamic_union_or_isect(c, ctx, object, type_0, orig, tuple, next_block);
        }
        150 => {
            trace_unknown(c, ctx, object, PONY_TRACE_OPAQUE as libc::c_int);
            let mut desc: LLVMValueRef = gendesc_fetch(c, object);
            let mut ptr: LLVMValueRef = gendesc_ptr_to_fields(c, object, desc);
            trace_dynamic_tuple(c, ctx, ptr, desc, type_0, orig, tuple);
        }
        151 => {
            trace_dynamic_nominal(c, ctx, object, type_0, orig, tuple, next_block);
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "867:1"]
pub unsafe extern "C" fn gentrace_needed(
    mut c: *mut compile_t,
    mut src_type: *mut ast_t,
    mut dst_type: *mut ast_t,
) -> bool {
    match trace_type(src_type) as libc::c_uint {
        0 => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                        as *const u8 as *const libc::c_char,
                    872 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"gentrace_needed\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
        2 => {
            if ast_id(dst_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
                let mut def: *mut ast_t = ast_data(dst_type) as *mut ast_t;
                if ast_id(def) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int != 0;
                }
            }
            return 1 as libc::c_int != 0;
        }
        3 => return 0 as libc::c_int != 0,
        12 => {
            if ast_id(dst_type) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
                let mut src_child: *mut ast_t = ast_child(src_type);
                let mut dst_child: *mut ast_t = ast_child(dst_type);
                while !src_child.is_null() && !dst_child.is_null() {
                    if gentrace_needed(c, src_child, dst_child) {
                        return 1 as libc::c_int != 0;
                    }
                    src_child = ast_sibling(src_child);
                    dst_child = ast_sibling(dst_child);
                }
                if src_child.is_null() && dst_child.is_null() {
                } else {
                    ponyint_assert_fail(
                        b"src_child == NULL && dst_child == NULL\0" as *const u8
                            as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                            as *const u8 as *const libc::c_char,
                        902 as libc::c_int as size_t,
                        (*::core::mem::transmute::<
                            &[u8; 16],
                            &[libc::c_char; 16],
                        >(b"gentrace_needed\0"))
                            .as_ptr(),
                    );
                };
            } else {
                return 1 as libc::c_int != 0;
            }
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "918:1"]
pub unsafe extern "C" fn gentrace_prototype(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    match (*t).underlying as libc::c_uint {
        76 | 75 | 77 | 150 => {}
        _ => return,
    }
    let mut need_trace: bool = 0 as libc::c_int != 0;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*t).field_count {
        let mut field_type: *mut ast_t = (*((*t).fields).offset(i as isize)).ast;
        if gentrace_needed(c, field_type, field_type) {
            need_trace = 1 as libc::c_int != 0;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if !need_trace {
        return;
    }
    let ref mut fresh0 = (*((*t).c_type as *mut compile_type_t)).trace_fn;
    *fresh0 = codegen_addfun(
        c,
        genname_trace((*t).name),
        (*c).trace_type,
        1 as libc::c_int != 0,
    );
}
#[no_mangle]
#[c2rust::src_loc = "951:1"]
pub unsafe extern "C" fn gentrace(
    mut c: *mut compile_t,
    mut ctx: LLVMValueRef,
    mut src_value: LLVMValueRef,
    mut dst_value: LLVMValueRef,
    mut src_type: *mut ast_t,
    mut dst_type: *mut ast_t,
) {
    let mut trace_method: trace_t = trace_type(src_type);
    if src_type != dst_type {
        trace_method = trace_type_dst_cap(trace_method, trace_type(dst_type), src_type);
    }
    match trace_method as libc::c_uint {
        0 => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.c\0"
                        as *const u8 as *const libc::c_char,
                    964 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"gentrace\0"))
                        .as_ptr(),
                );
            };
            
        }
        2 => {
            let mut boxed: bool = 1 as libc::c_int != 0;
            if ast_id(dst_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
                let mut def: *mut ast_t = ast_data(dst_type) as *mut ast_t;
                if ast_id(def) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint {
                    boxed = 0 as libc::c_int != 0;
                }
            }
            if boxed {
                trace_known(
                    c,
                    ctx,
                    dst_value,
                    src_type,
                    PONY_TRACE_IMMUTABLE as libc::c_int,
                );
            }
            
        }
        3 => {},
        1 => {
            trace_nullable_pointer(c, ctx, dst_value, src_type);
            
        }
        4 => {
            trace_known(
                c,
                ctx,
                dst_value,
                src_type,
                PONY_TRACE_IMMUTABLE as libc::c_int,
            );
            
        }
        5 => {
            trace_unknown(c, ctx, dst_value, PONY_TRACE_IMMUTABLE as libc::c_int);
            
        }
        6 => {
            trace_known(
                c,
                ctx,
                dst_value,
                src_type,
                PONY_TRACE_MUTABLE as libc::c_int,
            );
            
        }
        7 => {
            trace_unknown(c, ctx, dst_value, PONY_TRACE_MUTABLE as libc::c_int);
            
        }
        8 => {
            trace_known(
                c,
                ctx,
                dst_value,
                src_type,
                PONY_TRACE_OPAQUE as libc::c_int,
            );
            
        }
        9 => {
            trace_unknown(c, ctx, dst_value, PONY_TRACE_OPAQUE as libc::c_int);
            
        }
        10 => {
            trace_static(c, ctx, dst_value, src_type, dst_type);
            
        }
        11 => {
            let mut next_block: LLVMBasicBlockRef =
                codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
            trace_dynamic(
                c,
                ctx,
                dst_value,
                src_type,
                dst_type,
                0 as *mut ast_t,
                next_block,
            );
            LLVMBuildBr((*c).builder, next_block);
            LLVMMoveBasicBlockAfter(next_block, LLVMGetInsertBlock((*c).builder));
            LLVMPositionBuilderAtEnd((*c).builder, next_block);
            
        }
        12 => {
            trace_tuple(c, ctx, src_value, dst_value, src_type, dst_type);
            
        }
        _ => {}
    };
}
