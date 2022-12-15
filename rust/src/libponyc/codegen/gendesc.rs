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
        LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
    };
    extern "C" {
        #[c2rust::src_loc = "1246:1"]
        pub fn LLVMFunctionType(
            ReturnType: LLVMTypeRef,
            ParamTypes: *mut LLVMTypeRef,
            ParamCount: libc::c_uint,
            IsVarArg: LLVMBool,
        ) -> LLVMTypeRef;
        #[c2rust::src_loc = "1258:1"]
        pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1263:1"]
        pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> libc::c_uint;
        #[c2rust::src_loc = "1276:1"]
        pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
        #[c2rust::src_loc = "1316:1"]
        pub fn LLVMStructCreateNamed(C: LLVMContextRef, Name: *const libc::c_char) -> LLVMTypeRef;
        #[c2rust::src_loc = "1330:1"]
        pub fn LLVMStructSetBody(
            StructTy: LLVMTypeRef,
            ElementTypes: *mut LLVMTypeRef,
            ElementCount: libc::c_uint,
            Packed: LLVMBool,
        );
        #[c2rust::src_loc = "1400:1"]
        pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1424:1"]
        pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: libc::c_uint) -> LLVMTypeRef;
        #[c2rust::src_loc = "1443:1"]
        pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: libc::c_uint)
            -> LLVMTypeRef;
        #[c2rust::src_loc = "1669:1"]
        pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1868:1"]
        pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "1934:1"]
        pub fn LLVMConstInt(
            IntTy: LLVMTypeRef,
            N: libc::c_ulonglong,
            SignExtend: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2059:1"]
        pub fn LLVMConstStructInContext(
            C: LLVMContextRef,
            ConstantVals: *mut LLVMValueRef,
            Count: libc::c_uint,
            Packed: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2079:1"]
        pub fn LLVMConstArray(
            ElementTy: LLVMTypeRef,
            ConstantVals: *mut LLVMValueRef,
            Length: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2087:1"]
        pub fn LLVMConstNamedStruct(
            StructTy: LLVMTypeRef,
            ConstantVals: *mut LLVMValueRef,
            Count: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2146:1"]
        pub fn LLVMConstAnd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2153:1"]
        pub fn LLVMConstShl(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2154:1"]
        pub fn LLVMConstLShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2182:1"]
        pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2236:1"]
        pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
        #[c2rust::src_loc = "2257:1"]
        pub fn LLVMSetUnnamedAddr(Global: LLVMValueRef, HasUnnamedAddr: LLVMBool);
        #[c2rust::src_loc = "2353:1"]
        pub fn LLVMAddGlobal(
            M: LLVMModuleRef,
            Ty: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2364:1"]
        pub fn LLVMSetInitializer(GlobalVar: LLVMValueRef, ConstantVal: LLVMValueRef);
        #[c2rust::src_loc = "2368:1"]
        pub fn LLVMSetGlobalConstant(GlobalVar: LLVMValueRef, IsConstant: LLVMBool);
        #[c2rust::src_loc = "2665:1"]
        pub fn LLVMGetParam(Fn: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
        #[c2rust::src_loc = "3682:1"]
        pub fn LLVMBuildRet(_: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3837:1"]
        pub fn LLVMBuildAnd(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3937:1"]
        pub fn LLVMBuildZExt(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Target.h:1"]
pub mod Target_h {
    #[c2rust::src_loc = "37:1"]
    pub type LLVMTargetDataRef = *mut LLVMOpaqueTargetData;
    use super::Types_h::LLVMTypeRef;
    extern "C" {
        #[c2rust::src_loc = "37:16"]
        pub type LLVMOpaqueTargetData;
        #[c2rust::src_loc = "283:1"]
        pub fn LLVMOffsetOfElement(
            TD: LLVMTargetDataRef,
            StructTy: LLVMTypeRef,
            Element: libc::c_uint,
        ) -> libc::c_ulonglong;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:16"]
    pub struct reach_t {
        pub types: reach_types_t,
        pub method_stack: *mut reach_method_stack_t,
        pub object_type_count: u32,
        pub numeric_type_count: u32,
        pub tuple_type_count: u32,
        pub total_type_count: u32,
        pub trait_type_count: u32,
    }
    use super::_size_t_h::size_t;
    use super::hash_h::hashmap_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "18:35"]
        pub type reach_method_stack_t;
        #[c2rust::src_loc = "20:59"]
        pub fn reach_mangled_next(map: *mut reach_mangled_t, i: *mut usize) -> *mut reach_method_t;
        #[c2rust::src_loc = "22:3"]
        pub fn reach_method_names_next(
            map: *mut reach_method_names_t,
            i: *mut usize,
        ) -> *mut reach_method_name_t;
        #[c2rust::src_loc = "23:55"]
        pub fn reach_types_next(map: *mut reach_types_t, i: *mut usize) -> *mut reach_type_t;
        #[c2rust::src_loc = "24:65"]
        pub fn reach_type_cache_next(
            map: *mut reach_type_cache_t,
            i: *mut usize,
        ) -> *mut reach_type_t;
        #[c2rust::src_loc = "136:1"]
        pub fn reach_type(r: *mut reach_t, type_0: *mut ast_t) -> *mut reach_type_t;
        #[c2rust::src_loc = "146:1"]
        pub fn reach_vtable_index(t: *mut reach_type_t, name: *const libc::c_char) -> u32;
        #[c2rust::src_loc = "148:1"]
        pub fn reach_max_type_id(r: *mut reach_t) -> u32;
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
        pub trait_bitmap_size: u32,
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
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "291:1"]
        pub fn codegen_call(
            c: *mut compile_t,
            fun: LLVMValueRef,
            args: *mut LLVMValueRef,
            count: usize,
            setcc: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMBuildStructGEP_P(
            B: LLVMBuilderRef,
            Pointer: LLVMValueRef,
            Idx: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "39:1"]
        pub fn LLVMBuildInBoundsGEP_P(
            B: LLVMBuilderRef,
            Pointer: LLVMValueRef,
            Indices: *mut LLVMValueRef,
            NumIndices: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "41:1"]
        pub fn LLVMBuildLoad_P(
            B: LLVMBuilderRef,
            PointerVal: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "246:1"]
        pub fn codegen_addfun(
            c: *mut compile_t,
            name: *const libc::c_char,
            type_0: LLVMTypeRef,
            pony_abi: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "249:1"]
        pub fn codegen_startfun(
            c: *mut compile_t,
            fun: LLVMValueRef,
            file: LLVMMetadataRef,
            scope: LLVMMetadataRef,
            reify: *mut deferred_reification_t,
            bare: bool,
        );
        #[c2rust::src_loc = "252:1"]
        pub fn codegen_finishfun(c: *mut compile_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.h:3"]
pub mod gentype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "12:16"]
    pub struct compile_type_t {
        pub free_fn: compile_opaque_free_fn,
        pub abi_size: usize,
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.h:3"]
pub mod genfun_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:16"]
    pub struct compile_method_t {
        pub free_fn: compile_opaque_free_fn,
        pub func_type: LLVMTypeRef,
        pub msg_type: LLVMTypeRef,
        pub func: LLVMValueRef,
        pub func_handler: LLVMValueRef,
        pub di_method: LLVMMetadataRef,
        pub di_file: LLVMMetadataRef,
    }
    use super::reach_h::compile_opaque_free_fn;
    use super::Types_h::{LLVMMetadataRef, LLVMTypeRef, LLVMValueRef};
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:2"]
pub mod genexpr_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::{LLVMTypeRef, LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "16:1"]
        pub fn gen_assign_cast(
            c: *mut compile_t,
            l_type: LLVMTypeRef,
            r_value: LLVMValueRef,
            type_0: *mut ast_t,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:4"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn genname_traitmap(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "17:1"]
        pub fn genname_fieldlist(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "29:1"]
        pub fn genname_descriptor(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "39:1"]
        pub fn genname_unbox(name: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:5"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:9"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: usize) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: usize, p: *mut libc::c_void);
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
            line: usize,
            func: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:11"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "73:7"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
use self::ast_h::{ast_data, ast_id};
pub use self::codegen_h::{
    codegen_addfun, codegen_call, codegen_finishfun, codegen_startfun, compile_frame_t,
    compile_locals_t, compile_t, ffi_decls_t, genned_strings_t, LLVMBuildInBoundsGEP_P,
    LLVMBuildLoad_P, LLVMBuildStructGEP_P,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::genexpr_h::gen_assign_cast;
pub use self::genfun_h::compile_method_t;
use self::genname_h::{genname_descriptor, genname_fieldlist, genname_traitmap, genname_unbox};
use self::genopt_h::target_is_ilp32;
pub use self::gentype_h::compile_type_t;
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{ponyint_pool_alloc_size, ponyint_pool_free_size};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_mangled_next, reach_mangled_t,
    reach_max_type_id, reach_method_name_t, reach_method_names_next, reach_method_names_t,
    reach_method_stack_t, reach_method_t, reach_methods_t, reach_param_t, reach_t, reach_type,
    reach_type_cache_next, reach_type_cache_t, reach_type_t, reach_types_next, reach_types_t,
    reach_vtable_index,
};
pub use self::reify_h::deferred_reification_t;
use self::string_h::{memmove, memset};

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
    LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddGlobal, LLVMAnyRegCallConv,
    LLVMAppendingLinkage, LLVMArrayType, LLVMAvailableExternallyLinkage, LLVMBuildAnd,
    LLVMBuildBitCast, LLVMBuildExtractValue, LLVMBuildICmp, LLVMBuildRet, LLVMBuildZExt,
    LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv, LLVMColdCallConv, LLVMCommonLinkage,
    LLVMConstAnd, LLVMConstArray, LLVMConstBitCast, LLVMConstInt, LLVMConstLShr,
    LLVMConstNamedStruct, LLVMConstNull, LLVMConstShl, LLVMConstStructInContext,
    LLVMCountParamTypes, LLVMDLLExportLinkage, LLVMDLLImportLinkage, LLVMExternalLinkage,
    LLVMExternalWeakLinkage, LLVMFastCallConv, LLVMFunctionType, LLVMGHCCallConv,
    LLVMGetElementType, LLVMGetParam, LLVMGetParamTypes, LLVMGetReturnType, LLVMGhostLinkage,
    LLVMHHVMCCallConv, LLVMHHVMCallConv, LLVMHiPECallConv, LLVMIntEQ, LLVMIntNE, LLVMIntPredicate,
    LLVMIntSGE, LLVMIntSGT, LLVMIntSLE, LLVMIntSLT, LLVMIntUGE, LLVMIntUGT, LLVMIntULE, LLVMIntULT,
    LLVMIntelOCLBICallConv, LLVMInternalLinkage, LLVMLinkOnceAnyLinkage,
    LLVMLinkOnceODRAutoHideLinkage, LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage,
    LLVMLinkerPrivateWeakLinkage, LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv,
    LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv, LLVMPointerType, LLVMPreserveAllCallConv,
    LLVMPreserveMostCallConv, LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv,
    LLVMSetGlobalConstant, LLVMSetInitializer, LLVMSetLinkage, LLVMSetUnnamedAddr,
    LLVMStructCreateNamed, LLVMStructSetBody, LLVMSwiftCallConv, LLVMTypeOf, LLVMWeakAnyLinkage,
    LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv, LLVMX8664SysVCallConv,
    LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv, LLVMX86StdcallCallConv,
    LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{LLVMOffsetOfElement, LLVMOpaqueTargetData, LLVMTargetDataRef};
pub use self::Types_h::{
    LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    LLVMModuleRef, LLVMOpaqueBasicBlock, LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder,
    LLVMOpaqueMetadata, LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef,
    LLVMValueRef,
};
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
#[c2rust::src_loc = "33:1"]
unsafe extern "C" fn make_unbox_function(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> LLVMValueRef {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut f_type: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf((*c_m).func));
    let mut count: libc::c_int = LLVMCountParamTypes(f_type) as libc::c_int;
    let mut buf_size: usize = ((count + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>());
    let mut params: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
    LLVMGetParamTypes(f_type, params);
    let mut ret_type: LLVMTypeRef = LLVMGetReturnType(f_type);
    let mut unbox_name: *const libc::c_char = genname_unbox((*m).full_name);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if ast_id((*(*m).fun).ast) as libc::c_uint != TK_NEW as libc::c_int as libc::c_uint {
        let ref mut fresh0 = *params.offset(0 as libc::c_int as isize);
        *fresh0 = (*c_t).structure_ptr;
    } else {
        memmove(
            &mut *params.offset(1 as libc::c_int as isize) as *mut LLVMTypeRef as *mut libc::c_void,
            &mut *params.offset(0 as libc::c_int as isize) as *mut LLVMTypeRef
                as *const libc::c_void,
            (count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut LLVMTypeRef>() as libc::c_ulong),
        );
        let ref mut fresh1 = *params.offset(0 as libc::c_int as isize);
        *fresh1 = (*c_t).structure_ptr;
        count += 1;
    }
    let mut unbox_type: LLVMTypeRef =
        LLVMFunctionType(ret_type, params, count as libc::c_uint, 0 as libc::c_int);
    let mut unbox_fun: LLVMValueRef =
        codegen_addfun(c, unbox_name, unbox_type, 1 as libc::c_int != 0);
    codegen_startfun(
        c,
        unbox_fun,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    let mut this_ptr: LLVMValueRef = LLVMGetParam(unbox_fun, 0 as libc::c_int as libc::c_uint);
    let mut primitive_ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        this_ptr,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut primitive: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        primitive_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    primitive = gen_assign_cast(c, (*c_t).use_type, primitive, (*t).ast_cap);
    let mut args: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    if ast_id((*(*m).fun).ast) as libc::c_uint != TK_NEW as libc::c_int as libc::c_uint {
        let ref mut fresh2 = *args.offset(0 as libc::c_int as isize);
        *fresh2 = primitive;
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < count {
            let ref mut fresh3 = *args.offset(i as isize);
            *fresh3 = LLVMGetParam(unbox_fun, i as libc::c_uint);
            i += 1;
        }
    } else {
        count -= 1;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < count {
            let ref mut fresh4 = *args.offset(i_0 as isize);
            *fresh4 = LLVMGetParam(unbox_fun, (i_0 + 1 as libc::c_int) as libc::c_uint);
            i_0 += 1;
        }
    }
    let mut result: LLVMValueRef = codegen_call(
        c,
        (*c_m).func,
        args,
        count as usize,
        (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
    ponyint_pool_free_size(buf_size, params as *mut libc::c_void);
    ponyint_pool_free_size(buf_size, args as *mut libc::c_void);
    return LLVMConstBitCast(unbox_fun, (*c).void_ptr);
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn make_desc_ptr(
    mut func: LLVMValueRef,
    mut type_0: LLVMTypeRef,
) -> LLVMValueRef {
    if func.is_null() {
        return LLVMConstNull(type_0);
    }
    return LLVMConstBitCast(func, type_0);
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn trait_bitmap32(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> *mut LLVMValueRef {
    let mut bm_size: usize =
        ((*c).trait_bitmap_size as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u32>());
    let mut bm: *mut u32 = ponyint_pool_alloc_size(bm_size) as *mut u32;
    memset(bm as *mut libc::c_void, 0 as libc::c_int, bm_size);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut provide: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        provide = reach_type_cache_next(&mut (*t).subtypes, &mut i);
        if provide.is_null() {
            break;
        }
        if (*provide).type_id != -(1 as libc::c_int) as u32 {
        } else {
            ponyint_assert_fail(
                b"provide->type_id != (uint32_t)-1\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                    as *const u8 as *const libc::c_char,
                120 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"trait_bitmap32\0"))
                    .as_ptr(),
            );
        };
        let mut index: u32 = (*provide).type_id >> 5 as libc::c_int;
        if index < (*c).trait_bitmap_size {
        } else {
            ponyint_assert_fail(
                b"index < c->trait_bitmap_size\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                    as *const u8 as *const libc::c_char,
                122 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"trait_bitmap32\0"))
                    .as_ptr(),
            );
        };
        let mut bit: u32 = (*provide).type_id & 31 as libc::c_int as libc::c_uint;
        let ref mut fresh5 = *bm.offset(index as isize);
        *fresh5 |= ((1 as libc::c_int) << bit) as libc::c_uint;
    }
    let mut bitmap: *mut LLVMValueRef = ponyint_pool_alloc_size(
        ((*c).trait_bitmap_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMValueRef>()),
    ) as *mut LLVMValueRef;
    i = 0 as libc::c_int as usize;
    while i < (*c).trait_bitmap_size as libc::c_ulong {
        let ref mut fresh6 = *bitmap.offset(i as isize);
        *fresh6 = LLVMConstInt(
            (*c).intptr,
            *bm.offset(i as isize) as libc::c_ulonglong,
            0 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    ponyint_pool_free_size(bm_size, bm as *mut libc::c_void);
    return bitmap;
}
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn trait_bitmap64(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> *mut LLVMValueRef {
    let mut bm_size: usize =
        ((*c).trait_bitmap_size as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u64>());
    let mut bm: *mut u64 = ponyint_pool_alloc_size(bm_size) as *mut u64;
    memset(bm as *mut libc::c_void, 0 as libc::c_int, bm_size);
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut provide: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        provide = reach_type_cache_next(&mut (*t).subtypes, &mut i);
        if provide.is_null() {
            break;
        }
        if (*provide).type_id != -(1 as libc::c_int) as u32 {
        } else {
            ponyint_assert_fail(
                b"provide->type_id != (uint32_t)-1\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                    as *const u8 as *const libc::c_char,
                150 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"trait_bitmap64\0"))
                    .as_ptr(),
            );
        };
        let mut index: u64 = ((*provide).type_id >> 6 as libc::c_int) as u64;
        if index < (*c).trait_bitmap_size as libc::c_ulonglong {
        } else {
            ponyint_assert_fail(
                b"index < c->trait_bitmap_size\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                    as *const u8 as *const libc::c_char,
                152 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"trait_bitmap64\0"))
                    .as_ptr(),
            );
        };
        let mut bit: u64 = ((*provide).type_id & 63 as libc::c_int as libc::c_uint) as u64;
        let ref mut fresh7 = *bm.offset(index as isize);
        *fresh7 |= (1 as libc::c_int as u64) << bit;
    }
    let mut bitmap: *mut LLVMValueRef = ponyint_pool_alloc_size(
        ((*c).trait_bitmap_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMValueRef>()),
    ) as *mut LLVMValueRef;
    i = 0 as libc::c_int as usize;
    while i < (*c).trait_bitmap_size as libc::c_ulong {
        let ref mut fresh8 = *bitmap.offset(i as isize);
        *fresh8 = LLVMConstInt((*c).intptr, *bm.offset(i as isize), 0 as libc::c_int);
        i = i.wrapping_add(1);
    }
    ponyint_pool_free_size(bm_size, bm as *mut libc::c_void);
    return bitmap;
}
#[c2rust::src_loc = "168:1"]
unsafe extern "C" fn make_trait_bitmap(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> LLVMValueRef {
    let mut map_type: LLVMTypeRef = LLVMArrayType((*c).intptr, (*c).trait_bitmap_size);
    if !((*t).bare_method).is_null() {
        return LLVMConstNull(LLVMPointerType(map_type, 0 as libc::c_int as libc::c_uint));
    }
    let mut bitmap: *mut LLVMValueRef = 0 as *mut LLVMValueRef;
    if target_is_ilp32((*(*c).opt).triple) {
        bitmap = trait_bitmap32(c, t);
    } else {
        bitmap = trait_bitmap64(c, t);
    }
    let mut bitmap_array: LLVMValueRef =
        LLVMConstArray((*c).intptr, bitmap, (*c).trait_bitmap_size);
    ponyint_pool_free_size(
        ((*c).trait_bitmap_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMValueRef>()),
        bitmap as *mut libc::c_void,
    );
    let mut name: *const libc::c_char = genname_traitmap((*t).name);
    let mut global: LLVMValueRef = LLVMAddGlobal((*c).module, map_type, name);
    LLVMSetGlobalConstant(global, 1 as libc::c_int);
    LLVMSetLinkage(global, LLVMPrivateLinkage);
    LLVMSetInitializer(global, bitmap_array);
    LLVMSetUnnamedAddr(global, 1 as libc::c_int);
    return global;
}
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn make_field_count(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> LLVMValueRef {
    if (*t).underlying as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint {
        return LLVMConstInt((*c).i32_0, 0, 0 as libc::c_int);
    }
    return LLVMConstInt(
        (*c).i32_0,
        (*t).field_count as libc::c_ulonglong,
        0 as libc::c_int,
    );
}
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn make_field_offset(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> LLVMValueRef {
    if (*t).field_count == 0 as libc::c_int as libc::c_uint {
        return LLVMConstInt((*c).i32_0, 0, 0 as libc::c_int);
    }
    let mut index: libc::c_int = 0 as libc::c_int;
    if (*t).underlying as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
        index += 1;
    }
    if (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
        index += 1;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    return LLVMConstInt(
        (*c).i32_0,
        LLVMOffsetOfElement((*c).target_data, (*c_t).structure, index as libc::c_uint),
        0 as libc::c_int,
    );
}
#[c2rust::src_loc = "223:1"]
unsafe extern "C" fn make_field_list(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> LLVMValueRef {
    let mut count: u32 = 0;
    if (*t).underlying as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
        count = (*t).field_count;
    } else {
        count = 0 as libc::c_int as u32;
    }
    let mut field_type: LLVMTypeRef = LLVMArrayType((*c).field_descriptor, count);
    if count == 0 as libc::c_int as libc::c_uint {
        return LLVMConstNull(LLVMPointerType(
            field_type,
            0 as libc::c_int as libc::c_uint,
        ));
    }
    let mut buf_size: usize =
        (count as libc::c_ulong).wrapping_mul(::core::mem::size_of::<LLVMValueRef>());
    let mut list: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut i: u32 = 0 as libc::c_int as u32;
    while i < count {
        let mut fdesc: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
        fdesc[0 as libc::c_int as usize] = LLVMConstInt(
            (*c).i32_0,
            LLVMOffsetOfElement((*c).target_data, (*c_t).primitive, i),
            0 as libc::c_int,
        );
        let mut f_c_t: *mut compile_type_t =
            (*(*((*t).fields).offset(i as isize)).type_0).c_type as *mut compile_type_t;
        if !((*f_c_t).desc).is_null() {
            fdesc[1 as libc::c_int as usize] = LLVMConstBitCast((*f_c_t).desc, (*c).descriptor_ptr);
        } else {
            fdesc[1 as libc::c_int as usize] = LLVMConstNull((*c).descriptor_ptr);
        }
        let ref mut fresh9 = *list.offset(i as isize);
        *fresh9 = LLVMConstStructInContext(
            (*c).context,
            fdesc.as_mut_ptr(),
            2 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    let mut field_array: LLVMValueRef = LLVMConstArray((*c).field_descriptor, list, count);
    let mut name: *const libc::c_char = genname_fieldlist((*t).name);
    let mut global: LLVMValueRef = LLVMAddGlobal((*c).module, field_type, name);
    LLVMSetGlobalConstant(global, 1 as libc::c_int);
    LLVMSetLinkage(global, LLVMPrivateLinkage);
    LLVMSetInitializer(global, field_array);
    LLVMSetUnnamedAddr(global, 1 as libc::c_int);
    ponyint_pool_free_size(buf_size, list as *mut libc::c_void);
    return global;
}
#[c2rust::src_loc = "277:1"]
unsafe extern "C" fn make_vtable(mut c: *mut compile_t, mut t: *mut reach_type_t) -> LLVMValueRef {
    if (*t).vtable_size == 0 as libc::c_int as libc::c_uint {
        return LLVMConstArray(
            (*c).void_ptr,
            0 as *mut LLVMValueRef,
            0 as libc::c_int as libc::c_uint,
        );
    }
    let mut buf_size: usize =
        ((*t).vtable_size as libc::c_ulong).wrapping_mul(::core::mem::size_of::<LLVMValueRef>());
    let mut vtable: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    memset(vtable as *mut libc::c_void, 0 as libc::c_int, buf_size);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
    loop {
        n = reach_method_names_next(&mut (*t).methods, &mut i);
        if n.is_null() {
            break;
        }
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            let mut index: u32 = (*m).vtable_index;
            if index != -(1 as libc::c_int) as u32 {
            } else {
                ponyint_assert_fail(
                    b"index != (uint32_t)-1\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                        as *const u8 as *const libc::c_char,
                    298 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"make_vtable\0"))
                        .as_ptr(),
                );
            };
            if (*vtable.offset(index as isize)).is_null() {
            } else {
                ponyint_assert_fail(
                    b"vtable[index] == NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                        as *const u8 as *const libc::c_char,
                    299 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"make_vtable\0"))
                        .as_ptr(),
                );
            };
            let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
            if !((*c_t).primitive).is_null() && !(*m).internal {
                let ref mut fresh10 = *vtable.offset(index as isize);
                *fresh10 = make_unbox_function(c, t, m);
            } else {
                let ref mut fresh11 = *vtable.offset(index as isize);
                *fresh11 = make_desc_ptr((*c_m).func, (*c).void_ptr);
            }
        }
    }
    let mut i_0: u32 = 0 as libc::c_int as u32;
    while i_0 < (*t).vtable_size {
        if (*vtable.offset(i_0 as isize)).is_null() {
            let ref mut fresh12 = *vtable.offset(i_0 as isize);
            *fresh12 = LLVMConstNull((*c).void_ptr);
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut r: LLVMValueRef = LLVMConstArray((*c).void_ptr, vtable, (*t).vtable_size);
    ponyint_pool_free_size(buf_size, vtable as *mut libc::c_void);
    return r;
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn gendesc_basetype(mut c: *mut compile_t, mut desc_type: LLVMTypeRef) {
    let mut params: [LLVMTypeRef; 17] = [0 as *mut LLVMOpaqueType; 17];
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    params[2 as libc::c_int as usize] = (*c).i32_0;
    params[3 as libc::c_int as usize] = (*c).i32_0;
    params[4 as libc::c_int as usize] = (*c).object_ptr;
    params[5 as libc::c_int as usize] = (*c).trace_fn;
    params[6 as libc::c_int as usize] = (*c).trace_fn;
    params[7 as libc::c_int as usize] = (*c).serialise_fn;
    params[8 as libc::c_int as usize] = (*c).trace_fn;
    params[9 as libc::c_int as usize] = (*c).custom_serialise_space_fn;
    params[10 as libc::c_int as usize] = (*c).custom_deserialise_fn;
    params[11 as libc::c_int as usize] = (*c).dispatch_fn;
    params[12 as libc::c_int as usize] = (*c).final_fn;
    params[13 as libc::c_int as usize] = (*c).i32_0;
    params[14 as libc::c_int as usize] = LLVMPointerType(
        LLVMArrayType((*c).intptr, 0 as libc::c_int as libc::c_uint),
        0 as libc::c_int as libc::c_uint,
    );
    params[15 as libc::c_int as usize] = LLVMPointerType(
        LLVMArrayType((*c).field_descriptor, 0 as libc::c_int as libc::c_uint),
        0 as libc::c_int as libc::c_uint,
    );
    params[16 as libc::c_int as usize] =
        LLVMArrayType((*c).void_ptr, 0 as libc::c_int as libc::c_uint);
    LLVMStructSetBody(
        desc_type,
        params.as_mut_ptr(),
        17 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "347:1"]
pub unsafe extern "C" fn gendesc_type(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    match (*t).underlying as libc::c_uint {
        150 | 74 | 75 | 76 | 77 => {}
        _ => return,
    }
    let mut desc_name: *const libc::c_char = genname_descriptor((*t).name);
    let mut fields: u32 = 0 as libc::c_int as u32;
    let mut vtable_size: u32 = (*t).vtable_size;
    if (*t).underlying as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
        fields = (*t).field_count;
    } else {
        vtable_size = (*t).vtable_size;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh13 = (*c_t).desc_type;
    *fresh13 = LLVMStructCreateNamed((*c).context, desc_name);
    let mut params: [LLVMTypeRef; 17] = [0 as *mut LLVMOpaqueType; 17];
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    params[2 as libc::c_int as usize] = (*c).i32_0;
    params[3 as libc::c_int as usize] = (*c).i32_0;
    params[4 as libc::c_int as usize] = (*c).object_ptr;
    params[5 as libc::c_int as usize] = (*c).trace_fn;
    params[6 as libc::c_int as usize] = (*c).trace_fn;
    params[7 as libc::c_int as usize] = (*c).serialise_fn;
    params[8 as libc::c_int as usize] = (*c).trace_fn;
    params[9 as libc::c_int as usize] = (*c).custom_serialise_space_fn;
    params[10 as libc::c_int as usize] = (*c).custom_deserialise_fn;
    params[11 as libc::c_int as usize] = (*c).dispatch_fn;
    params[12 as libc::c_int as usize] = (*c).final_fn;
    params[13 as libc::c_int as usize] = (*c).i32_0;
    params[14 as libc::c_int as usize] = LLVMPointerType(
        LLVMArrayType((*c).intptr, (*c).trait_bitmap_size),
        0 as libc::c_int as libc::c_uint,
    );
    params[15 as libc::c_int as usize] = LLVMPointerType(
        LLVMArrayType((*c).field_descriptor, fields),
        0 as libc::c_int as libc::c_uint,
    );
    params[16 as libc::c_int as usize] = LLVMArrayType((*c).void_ptr, vtable_size);
    LLVMStructSetBody(
        (*c_t).desc_type,
        params.as_mut_ptr(),
        17 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh14 = (*c_t).desc;
    *fresh14 = LLVMAddGlobal((*c).module, (*c_t).desc_type, desc_name);
    LLVMSetGlobalConstant((*c_t).desc, 1 as libc::c_int);
    LLVMSetLinkage((*c_t).desc, LLVMPrivateLinkage);
}
#[no_mangle]
#[c2rust::src_loc = "402:1"]
pub unsafe extern "C" fn gendesc_init(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if ((*c_t).desc_type).is_null() {
        return;
    }
    let mut event_notify_index: u32 = reach_vtable_index(t, (*c).str__event_notify);
    let mut args: [LLVMValueRef; 17] = [0 as *mut LLVMOpaqueValue; 17];
    args[0 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        (*t).type_id as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[1 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        (*c_t).abi_size as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[2 as libc::c_int as usize] = make_field_count(c, t);
    args[3 as libc::c_int as usize] = make_field_offset(c, t);
    args[4 as libc::c_int as usize] = make_desc_ptr((*c_t).instance, (*c).object_ptr);
    args[5 as libc::c_int as usize] = make_desc_ptr((*c_t).trace_fn, (*c).trace_fn);
    args[6 as libc::c_int as usize] = make_desc_ptr((*c_t).serialise_trace_fn, (*c).trace_fn);
    args[7 as libc::c_int as usize] = make_desc_ptr((*c_t).serialise_fn, (*c).serialise_fn);
    args[8 as libc::c_int as usize] = make_desc_ptr((*c_t).deserialise_fn, (*c).trace_fn);
    args[9 as libc::c_int as usize] = make_desc_ptr(
        (*c_t).custom_serialise_space_fn,
        (*c).custom_serialise_space_fn,
    );
    args[10 as libc::c_int as usize] =
        make_desc_ptr((*c_t).custom_deserialise_fn, (*c).custom_deserialise_fn);
    args[11 as libc::c_int as usize] = make_desc_ptr((*c_t).dispatch_fn, (*c).dispatch_fn);
    args[12 as libc::c_int as usize] = make_desc_ptr((*c_t).final_fn, (*c).final_fn);
    args[13 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        event_notify_index as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[14 as libc::c_int as usize] = make_trait_bitmap(c, t);
    args[15 as libc::c_int as usize] = make_field_list(c, t);
    args[16 as libc::c_int as usize] = make_vtable(c, t);
    let mut desc: LLVMValueRef = LLVMConstNamedStruct(
        (*c_t).desc_type,
        args.as_mut_ptr(),
        17 as libc::c_int as libc::c_uint,
    );
    LLVMSetInitializer((*c_t).desc, desc);
    LLVMSetGlobalConstant((*c_t).desc, 1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "439:1"]
pub unsafe extern "C" fn gendesc_table(mut c: *mut compile_t) {
    let mut len: u32 = reach_max_type_id((*c).reach);
    let mut size: usize =
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<LLVMValueRef>());
    let mut args: *mut LLVMValueRef = ponyint_pool_alloc_size(size) as *mut LLVMValueRef;
    let mut null: LLVMValueRef = LLVMConstNull((*c).descriptor_ptr);
    let mut i: usize = 0;
    while i < len as libc::c_ulong {
        let ref mut fresh15 = *args.offset(i as isize);
        *fresh15 = null;
        i = i.wrapping_add(1);
    }
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    let mut i_0: usize = -(1 as libc::c_int) as usize;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i_0);
        if t.is_null() {
            break;
        }
        if (*t).is_trait as libc::c_int != 0
            || (*t).underlying as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint
        {
            continue;
        }
        let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
        let mut desc: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
        if !((*c_t).desc).is_null() {
            desc = LLVMBuildBitCast(
                (*c).builder,
                (*c_t).desc,
                (*c).descriptor_ptr,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            desc = LLVMConstNull((*c).descriptor_ptr);
        }
        let ref mut fresh16 = *args.offset((*t).type_id as isize);
        *fresh16 = desc;
    }
    let mut type_0: LLVMTypeRef = LLVMArrayType((*c).descriptor_ptr, len);
    let mut table: LLVMValueRef = LLVMAddGlobal(
        (*c).module,
        type_0,
        b"__DescTable\0" as *const u8 as *const libc::c_char,
    );
    let mut value: LLVMValueRef = LLVMConstArray((*c).descriptor_ptr, args, len);
    LLVMSetInitializer(table, value);
    LLVMSetGlobalConstant(table, 1 as libc::c_int);
    LLVMSetLinkage(table, LLVMPrivateLinkage);
    let ref mut fresh17 = (*c).desc_table;
    *fresh17 = table;
    ponyint_pool_free_size(size, args as *mut libc::c_void);
}
#[c2rust::src_loc = "480:1"]
unsafe extern "C" fn desc_field(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut index: libc::c_int,
) -> LLVMValueRef {
    let mut ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        desc,
        index as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut field: LLVMValueRef =
        LLVMBuildLoad_P((*c).builder, ptr, b"\0" as *const u8 as *const libc::c_char);
    return field;
}
#[no_mangle]
#[c2rust::src_loc = "487:1"]
pub unsafe extern "C" fn gendesc_fetch(
    mut c: *mut compile_t,
    mut object: LLVMValueRef,
) -> LLVMValueRef {
    let mut ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        object,
        0 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut desc: LLVMValueRef =
        LLVMBuildLoad_P((*c).builder, ptr, b"\0" as *const u8 as *const libc::c_char);
    return desc;
}
#[no_mangle]
#[c2rust::src_loc = "494:1"]
pub unsafe extern "C" fn gendesc_typeid(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    return desc_field(c, desc, 0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "499:1"]
pub unsafe extern "C" fn gendesc_instance(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    return desc_field(c, desc, 4 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "504:1"]
pub unsafe extern "C" fn gendesc_trace(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    return desc_field(c, desc, 5 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "509:1"]
pub unsafe extern "C" fn gendesc_dispatch(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    return desc_field(c, desc, 11 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "514:1"]
pub unsafe extern "C" fn gendesc_vtable(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut colour: usize,
) -> LLVMValueRef {
    let mut vtable: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        desc,
        16 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut gep: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    gep[0 as libc::c_int as usize] = LLVMConstInt((*c).i32_0, 0, 0 as libc::c_int);
    gep[1 as libc::c_int as usize] =
        LLVMConstInt((*c).i32_0, colour as libc::c_ulonglong, 0 as libc::c_int);
    let mut func_ptr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        vtable,
        gep.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut fun: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        func_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return fun;
}
#[no_mangle]
#[c2rust::src_loc = "528:1"]
pub unsafe extern "C" fn gendesc_ptr_to_fields(
    mut c: *mut compile_t,
    mut object: LLVMValueRef,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    let mut offset: LLVMValueRef = desc_field(c, desc, 3 as libc::c_int);
    offset = LLVMBuildZExt(
        (*c).builder,
        offset,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut base: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        object,
        (*c).void_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildInBoundsGEP_P(
        (*c).builder,
        base,
        &mut offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "539:1"]
pub unsafe extern "C" fn gendesc_fieldcount(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
) -> LLVMValueRef {
    return desc_field(c, desc, 2 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "544:1"]
pub unsafe extern "C" fn gendesc_fieldinfo(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut index: usize,
) -> LLVMValueRef {
    let mut fields: LLVMValueRef = desc_field(c, desc, 15 as libc::c_int);
    let mut gep: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    gep[0 as libc::c_int as usize] = LLVMConstInt((*c).i32_0, 0, 0 as libc::c_int);
    gep[1 as libc::c_int as usize] =
        LLVMConstInt((*c).i32_0, index as libc::c_ulonglong, 0 as libc::c_int);
    let mut field_desc: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        fields,
        gep.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut field_info: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        field_desc,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return field_info;
}
#[no_mangle]
#[c2rust::src_loc = "558:1"]
pub unsafe extern "C" fn gendesc_fieldptr(
    mut c: *mut compile_t,
    mut ptr: LLVMValueRef,
    mut field_info: LLVMValueRef,
) -> LLVMValueRef {
    let mut offset: LLVMValueRef = LLVMBuildExtractValue(
        (*c).builder,
        field_info,
        0 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    offset = LLVMBuildZExt(
        (*c).builder,
        offset,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildInBoundsGEP_P(
        (*c).builder,
        ptr,
        &mut offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "566:1"]
pub unsafe extern "C" fn gendesc_fieldload(
    mut c: *mut compile_t,
    mut ptr: LLVMValueRef,
    mut field_info: LLVMValueRef,
) -> LLVMValueRef {
    let mut field_ptr: LLVMValueRef = gendesc_fieldptr(c, ptr, field_info);
    let mut object_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        field_ptr,
        LLVMPointerType((*c).object_ptr, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildLoad_P(
        (*c).builder,
        object_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "577:1"]
pub unsafe extern "C" fn gendesc_fielddesc(
    mut c: *mut compile_t,
    mut field_info: LLVMValueRef,
) -> LLVMValueRef {
    return LLVMBuildExtractValue(
        (*c).builder,
        field_info,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "582:1"]
pub unsafe extern "C" fn gendesc_isnominal(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut type_0: *mut ast_t,
) -> LLVMValueRef {
    let mut def: *mut ast_t = ast_data(type_0) as *mut ast_t;
    match ast_id(def) as libc::c_uint {
        75 => {
            return LLVMConstInt((*c).i1, 1, 0 as libc::c_int);
        }
        72 | 73 => return gendesc_istrait(c, desc, type_0),
        74 | 76 | 77 => return gendesc_isentity(c, desc, type_0),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                as *const u8 as *const libc::c_char,
            605 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"gendesc_isnominal\0"))
                .as_ptr(),
        );
    };
    return 1 as libc::c_int as LLVMValueRef;
}
#[no_mangle]
#[c2rust::src_loc = "609:1"]
pub unsafe extern "C" fn gendesc_istrait(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut type_0: *mut ast_t,
) -> LLVMValueRef {
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.c\0"
                as *const u8 as *const libc::c_char,
            612 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"gendesc_istrait\0"))
                .as_ptr(),
        );
    };
    let mut trait_id: LLVMValueRef = LLVMConstInt(
        (*c).intptr,
        (*t).type_id as libc::c_ulonglong,
        0 as libc::c_int,
    );
    let mut shift: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    let mut mask: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if target_is_ilp32((*(*c).opt).triple) {
        shift = LLVMConstInt((*c).intptr, 5, 0 as libc::c_int);
        mask = LLVMConstInt((*c).intptr, 31, 0 as libc::c_int);
    } else {
        shift = LLVMConstInt((*c).intptr, 6, 0 as libc::c_int);
        mask = LLVMConstInt((*c).intptr, 63, 0 as libc::c_int);
    }
    shift = LLVMConstLShr(trait_id, shift);
    mask = LLVMConstAnd(trait_id, mask);
    mask = LLVMConstShl(LLVMConstInt((*c).intptr, 1, 0 as libc::c_int), mask);
    let mut bitmap: LLVMValueRef = desc_field(c, desc, 14 as libc::c_int);
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] = LLVMConstInt((*c).intptr, 0, 0 as libc::c_int);
    args[1 as libc::c_int as usize] = shift;
    let mut index: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        bitmap,
        args.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    index = LLVMBuildLoad_P(
        (*c).builder,
        index,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut has_trait: LLVMValueRef = LLVMBuildAnd(
        (*c).builder,
        index,
        mask,
        b"\0" as *const u8 as *const libc::c_char,
    );
    has_trait = LLVMBuildICmp(
        (*c).builder,
        LLVMIntNE,
        has_trait,
        LLVMConstInt((*c).intptr, 0, 0 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char,
    );
    return has_trait;
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn gendesc_isentity(
    mut c: *mut compile_t,
    mut desc: LLVMValueRef,
    mut type_0: *mut ast_t,
) -> LLVMValueRef {
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if t.is_null() {
        return 1 as libc::c_int as LLVMValueRef;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut dptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        (*c_t).desc,
        (*c).descriptor_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildICmp(
        (*c).builder,
        LLVMIntEQ,
        desc,
        dptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
