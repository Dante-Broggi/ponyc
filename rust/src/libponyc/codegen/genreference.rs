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
        LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
    };
    extern "C" {
        #[c2rust::src_loc = "1106:1"]
        pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
        #[c2rust::src_loc = "1169:1"]
        pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> libc::c_uint;
        #[c2rust::src_loc = "1246:1"]
        pub fn LLVMFunctionType(
            ReturnType: LLVMTypeRef,
            ParamTypes: *mut LLVMTypeRef,
            ParamCount: libc::c_uint,
            IsVarArg: LLVMBool,
        ) -> LLVMTypeRef;
        #[c2rust::src_loc = "1338:1"]
        pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> libc::c_uint;
        #[c2rust::src_loc = "1350:1"]
        pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
        #[c2rust::src_loc = "1443:1"]
        pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: libc::c_uint)
            -> LLVMTypeRef;
        #[c2rust::src_loc = "1669:1"]
        pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1885:1"]
        pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "1934:1"]
        pub fn LLVMConstInt(
            IntTy: LLVMTypeRef,
            N: libc::c_ulonglong,
            SignExtend: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "1970:1"]
        pub fn LLVMConstReal(RealTy: LLVMTypeRef, N: libc::c_double) -> LLVMValueRef;
        #[c2rust::src_loc = "2087:1"]
        pub fn LLVMConstNamedStruct(
            StructTy: LLVMTypeRef,
            ConstantVals: *mut LLVMValueRef,
            Count: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2126:1"]
        pub fn LLVMConstAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2153:1"]
        pub fn LLVMConstShl(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2171:1"]
        pub fn LLVMConstTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2176:1"]
        pub fn LLVMConstUIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
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
        #[c2rust::src_loc = "3015:1"]
        pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3120:1"]
        pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3538:1"]
        pub fn LLVMAddIncoming(
            PhiNode: LLVMValueRef,
            IncomingValues: *mut LLVMValueRef,
            IncomingBlocks: *mut LLVMBasicBlockRef,
            Count: libc::c_uint,
        );
        #[c2rust::src_loc = "3606:1"]
        pub fn LLVMPositionBuilderBefore(Builder: LLVMBuilderRef, Instr: LLVMValueRef);
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3682:1"]
        pub fn LLVMBuildRet(_: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3685:1"]
        pub fn LLVMBuildBr(_: LLVMBuilderRef, Dest: LLVMBasicBlockRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3686:1"]
        pub fn LLVMBuildCondBr(
            _: LLVMBuilderRef,
            If: LLVMValueRef,
            Then: LLVMBasicBlockRef,
            Else: LLVMBasicBlockRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3833:1"]
        pub fn LLVMBuildLShr(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3837:1"]
        pub fn LLVMBuildAnd(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3841:1"]
        pub fn LLVMBuildXor(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3887:1"]
        pub fn LLVMBuildAlloca(
            _: LLVMBuilderRef,
            Ty: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3935:1"]
        pub fn LLVMBuildTrunc(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3937:1"]
        pub fn LLVMBuildZExt(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3953:1"]
        pub fn LLVMBuildPtrToInt(
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
        #[c2rust::src_loc = "3990:1"]
        pub fn LLVMBuildPhi(
            _: LLVMBuilderRef,
            Ty: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "4012:1"]
        pub fn LLVMBuildExtractValue(
            _: LLVMBuilderRef,
            AggVal: LLVMValueRef,
            Index: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "4014:1"]
        pub fn LLVMBuildInsertValue(
            _: LLVMBuilderRef,
            AggVal: LLVMValueRef,
            EltVal: LLVMValueRef,
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
        #[c2rust::src_loc = "261:1"]
        pub fn LLVMABIAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> libc::c_uint;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/DebugInfo.h:1"]
pub mod DebugInfo_h {
    #[c2rust::src_loc = "34:9"]
    pub type LLVMDIFlags = libc::c_uint;
    #[c2rust::src_loc = "70:3"]
    pub const LLVMDIFlagPtrToMemberRep: LLVMDIFlags = 196608;
    #[c2rust::src_loc = "68:3"]
    pub const LLVMDIFlagAccessibility: LLVMDIFlags = 3;
    #[c2rust::src_loc = "67:3"]
    pub const LLVMDIFlagIndirectVirtualBase: LLVMDIFlags = 36;
    #[c2rust::src_loc = "66:3"]
    pub const LLVMDIFlagLittleEndian: LLVMDIFlags = 268435456;
    #[c2rust::src_loc = "65:3"]
    pub const LLVMDIFlagBigEndian: LLVMDIFlags = 134217728;
    #[c2rust::src_loc = "64:3"]
    pub const LLVMDIFlagNonTrivial: LLVMDIFlags = 67108864;
    #[c2rust::src_loc = "63:3"]
    pub const LLVMDIFlagThunk: LLVMDIFlags = 33554432;
    #[c2rust::src_loc = "62:3"]
    pub const LLVMDIFlagFixedEnum: LLVMDIFlags = 16777216;
    #[c2rust::src_loc = "61:3"]
    pub const LLVMDIFlagEnumClass: LLVMDIFlags = 16777216;
    #[c2rust::src_loc = "60:3"]
    pub const LLVMDIFlagTypePassByReference: LLVMDIFlags = 8388608;
    #[c2rust::src_loc = "59:3"]
    pub const LLVMDIFlagTypePassByValue: LLVMDIFlags = 4194304;
    #[c2rust::src_loc = "58:3"]
    pub const LLVMDIFlagNoReturn: LLVMDIFlags = 1048576;
    #[c2rust::src_loc = "57:3"]
    pub const LLVMDIFlagBitField: LLVMDIFlags = 524288;
    #[c2rust::src_loc = "56:3"]
    pub const LLVMDIFlagIntroducedVirtual: LLVMDIFlags = 262144;
    #[c2rust::src_loc = "55:3"]
    pub const LLVMDIFlagVirtualInheritance: LLVMDIFlags = 196608;
    #[c2rust::src_loc = "54:3"]
    pub const LLVMDIFlagMultipleInheritance: LLVMDIFlags = 131072;
    #[c2rust::src_loc = "53:3"]
    pub const LLVMDIFlagSingleInheritance: LLVMDIFlags = 65536;
    #[c2rust::src_loc = "52:3"]
    pub const LLVMDIFlagReserved: LLVMDIFlags = 32768;
    #[c2rust::src_loc = "51:3"]
    pub const LLVMDIFlagRValueReference: LLVMDIFlags = 16384;
    #[c2rust::src_loc = "50:3"]
    pub const LLVMDIFlagLValueReference: LLVMDIFlags = 8192;
    #[c2rust::src_loc = "49:3"]
    pub const LLVMDIFlagStaticMember: LLVMDIFlags = 4096;
    #[c2rust::src_loc = "48:3"]
    pub const LLVMDIFlagVector: LLVMDIFlags = 2048;
    #[c2rust::src_loc = "47:3"]
    pub const LLVMDIFlagObjectPointer: LLVMDIFlags = 1024;
    #[c2rust::src_loc = "46:3"]
    pub const LLVMDIFlagObjcClassComplete: LLVMDIFlags = 512;
    #[c2rust::src_loc = "45:3"]
    pub const LLVMDIFlagPrototyped: LLVMDIFlags = 256;
    #[c2rust::src_loc = "44:3"]
    pub const LLVMDIFlagExplicit: LLVMDIFlags = 128;
    #[c2rust::src_loc = "43:3"]
    pub const LLVMDIFlagArtificial: LLVMDIFlags = 64;
    #[c2rust::src_loc = "42:3"]
    pub const LLVMDIFlagVirtual: LLVMDIFlags = 32;
    #[c2rust::src_loc = "41:3"]
    pub const LLVMDIFlagReservedBit4: LLVMDIFlags = 16;
    #[c2rust::src_loc = "40:3"]
    pub const LLVMDIFlagAppleBlock: LLVMDIFlags = 8;
    #[c2rust::src_loc = "39:3"]
    pub const LLVMDIFlagFwdDecl: LLVMDIFlags = 4;
    #[c2rust::src_loc = "38:3"]
    pub const LLVMDIFlagPublic: LLVMDIFlags = 3;
    #[c2rust::src_loc = "37:3"]
    pub const LLVMDIFlagProtected: LLVMDIFlags = 2;
    #[c2rust::src_loc = "36:3"]
    pub const LLVMDIFlagPrivate: LLVMDIFlags = 1;
    #[c2rust::src_loc = "35:3"]
    pub const LLVMDIFlagZero: LLVMDIFlags = 0;
    use super::Types_h::{LLVMBool, LLVMDIBuilderRef, LLVMMetadataRef};
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "1104:1"]
        pub fn LLVMDIBuilderCreateExpression(
            Builder: LLVMDIBuilderRef,
            Addr: *mut u64,
            Length: usize,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "1309:1"]
        pub fn LLVMDIBuilderCreateAutoVariable(
            Builder: LLVMDIBuilderRef,
            Scope: LLVMMetadataRef,
            Name: *const libc::c_char,
            NameLen: usize,
            File: LLVMMetadataRef,
            LineNo: libc::c_uint,
            Ty: LLVMMetadataRef,
            AlwaysPreserve: LLVMBool,
            Flags: LLVMDIFlags,
            AlignInBits: u32,
        ) -> LLVMMetadataRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/lexint.h:1"]
pub mod lexint_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:16"]
    pub struct lexint_t {
        pub low: u64,
        pub high: u64,
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/fun.h:1"]
pub mod fun_h {
    #[c2rust::src_loc = "13:1"]
    pub type cmp_fn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool>;
    #[c2rust::src_loc = "19:1"]
    pub type free_fn = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> usize;
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
    use super::_size_t_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/stringtab.h:1"]
pub mod stringtab_h {
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::lexint_h::lexint_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn ast_name_len(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "98:1"]
        pub fn ast_float(ast: *mut ast_t) -> libc::c_double;
        #[c2rust::src_loc = "99:1"]
        pub fn ast_int(ast: *mut ast_t) -> *mut lexint_t;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "118:1"]
        pub fn ast_index(ast: *mut ast_t) -> usize;
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
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn deferred_reify(
            deferred: *mut deferred_reification_t,
            ast: *mut ast_t,
            opt: *mut pass_opt_t,
        ) -> *mut ast_t;
    }
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
        #[c2rust::src_loc = "136:1"]
        pub fn reach_type(r: *mut reach_t, type_0: *mut ast_t) -> *mut reach_type_t;
        #[c2rust::src_loc = "140:1"]
        pub fn reach_method(
            t: *mut reach_type_t,
            cap: token_id,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
        ) -> *mut reach_method_t;
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
        #[c2rust::src_loc = "294:1"]
        pub fn codegen_string(
            c: *mut compile_t,
            str: *const libc::c_char,
            len: usize,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "291:1"]
        pub fn codegen_call(
            c: *mut compile_t,
            fun: LLVMValueRef,
            args: *mut LLVMValueRef,
            count: usize,
            setcc: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "289:1"]
        pub fn codegen_block(c: *mut compile_t, name: *const libc::c_char) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "287:1"]
        pub fn codegen_fun(c: *mut compile_t) -> LLVMValueRef;
        #[c2rust::src_loc = "281:1"]
        pub fn codegen_setlocal(c: *mut compile_t, name: *const libc::c_char, alloca: LLVMValueRef);
        #[c2rust::src_loc = "279:1"]
        pub fn codegen_getlocal(c: *mut compile_t, name: *const libc::c_char) -> LLVMValueRef;
        #[c2rust::src_loc = "266:1"]
        pub fn codegen_discope(c: *mut compile_t) -> LLVMMetadataRef;
        #[c2rust::src_loc = "264:1"]
        pub fn codegen_difile(c: *mut compile_t) -> LLVMMetadataRef;
        #[c2rust::src_loc = "252:1"]
        pub fn codegen_finishfun(c: *mut compile_t);
        #[c2rust::src_loc = "249:1"]
        pub fn codegen_startfun(
            c: *mut compile_t,
            fun: LLVMValueRef,
            file: LLVMMetadataRef,
            scope: LLVMMetadataRef,
            reify: *mut deferred_reification_t,
            bare: bool,
        );
        #[c2rust::src_loc = "246:1"]
        pub fn codegen_addfun(
            c: *mut compile_t,
            name: *const libc::c_char,
            type_0: LLVMTypeRef,
            pony_abi: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMBuildStructGEP_P(
            B: LLVMBuilderRef,
            Pointer: LLVMValueRef,
            Idx: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "41:1"]
        pub fn LLVMBuildLoad_P(
            B: LLVMBuilderRef,
            PointerVal: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.h:6"]
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn get_fieldinfo(
            l_type: *mut ast_t,
            right: *mut ast_t,
            l_def: *mut *mut ast_t,
            field: *mut *mut ast_t,
            index: *mut u32,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/subtype.h:11"]
pub mod subtype_h {
    #[c2rust::src_loc = "13:3"]
    pub const SUBTYPE_KIND_UNBOXED: subtype_kind_t = 4;
    #[c2rust::src_loc = "15:3"]
    pub const SUBTYPE_KIND_BOXED: subtype_kind_t = 3;
    #[c2rust::src_loc = "8:1"]
    pub type subtype_kind_t = libc::c_uint;
    #[c2rust::src_loc = "16:3"]
    pub const SUBTYPE_KIND_ALL: subtype_kind_t = 7;
    #[c2rust::src_loc = "12:3"]
    pub const SUBTYPE_KIND_TUPLE: subtype_kind_t = 2;
    #[c2rust::src_loc = "11:3"]
    pub const SUBTYPE_KIND_NUMERIC: subtype_kind_t = 1;
    #[c2rust::src_loc = "10:3"]
    pub const SUBTYPE_KIND_NONE: subtype_kind_t = 0;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn subtype_kind(type_0: *mut reach_type_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.h:6"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendebug.h:1"]
pub mod gendebug_h {
    use super::Types_h::{LLVMBasicBlockRef, LLVMDIBuilderRef, LLVMMetadataRef, LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn LLVMDIBuilderInsertDeclare(
            d: LLVMDIBuilderRef,
            value: LLVMValueRef,
            info: LLVMMetadataRef,
            expr: LLVMMetadataRef,
            line: libc::c_uint,
            col: libc::c_uint,
            scope: LLVMMetadataRef,
            block: LLVMBasicBlockRef,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genbox.h:2"]
pub mod genbox_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn gen_unbox(
            c: *mut compile_t,
            type_0: *mut ast_t,
            object: LLVMValueRef,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.h:3"]
pub mod gencall_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "12:1"]
        pub fn gen_funptr(c: *mut compile_t, ast: *mut ast_t) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.h:4"]
pub mod gendesc_h {
    use super::codegen_h::compile_t;
    use super::Types_h::LLVMValueRef;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn gendesc_fetch(c: *mut compile_t, object: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "19:1"]
        pub fn gendesc_typeid(c: *mut compile_t, desc: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "27:1"]
        pub fn gendesc_vtable(c: *mut compile_t, desc: LLVMValueRef, colour: usize)
            -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:5"]
pub mod genexpr_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::{LLVMTypeRef, LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gen_expr(c: *mut compile_t, ast: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "16:1"]
        pub fn gen_assign_cast(
            c: *mut compile_t,
            l_type: LLVMTypeRef,
            r_value: LLVMValueRef,
            type_0: *mut ast_t,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:8"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:13"]
pub mod type_subtype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn is_literal(type_0: *mut ast_t, name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "56:1"]
        pub fn is_known(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "64:1"]
        pub fn contains_dontcare(ast: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:15"]
pub mod pool_h {
    use super::_size_t_h::size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:16"]
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
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::ast_h::{
    ast_child, ast_data, ast_float, ast_free_unattached, ast_get_children, ast_id, ast_index,
    ast_int, ast_line, ast_name, ast_name_len, ast_pos, ast_ptr_t, ast_sibling, ast_type,
};
pub use self::codegen_h::{
    codegen_addfun, codegen_block, codegen_call, codegen_difile, codegen_discope,
    codegen_finishfun, codegen_fun, codegen_getlocal, codegen_setlocal, codegen_startfun,
    codegen_string, compile_frame_t, compile_locals_t, compile_t, ffi_decls_t, genned_strings_t,
    LLVMBuildLoad_P, LLVMBuildStructGEP_P,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
use self::genbox_h::gen_unbox;
use self::gencall_h::gen_funptr;
use self::gendebug_h::LLVMDIBuilderInsertDeclare;
use self::gendesc_h::{gendesc_fetch, gendesc_typeid, gendesc_vtable};
use self::genexpr_h::{gen_assign_cast, gen_expr};
pub use self::genfun_h::compile_method_t;
use self::genopt_h::target_is_ilp32;
pub use self::gentype_h::{compile_type_t, get_fieldinfo};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::lexint_h::lexint_t;
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_method, reach_method_names_t,
    reach_method_stack_t, reach_method_t, reach_param_t, reach_t, reach_type, reach_type_cache_t,
    reach_type_t, reach_types_t,
};
pub use self::reify_h::{deferred_reification_t, deferred_reify};
use self::string_h::strlen;
use self::stringtab_h::stringtab;
pub use self::subtype_h::{
    subtype_kind, subtype_kind_t, SUBTYPE_KIND_ALL, SUBTYPE_KIND_BOXED, SUBTYPE_KIND_NONE,
    SUBTYPE_KIND_NUMERIC, SUBTYPE_KIND_TUPLE, SUBTYPE_KIND_UNBOXED,
};
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
use self::type_subtype_h::{contains_dontcare, is_known, is_literal};
pub use self::Core_h::{
    LLVMAMDGPUCSCallConv, LLVMAMDGPUESCallConv, LLVMAMDGPUGSCallConv, LLVMAMDGPUHSCallConv,
    LLVMAMDGPUKERNELCallConv, LLVMAMDGPULSCallConv, LLVMAMDGPUPSCallConv, LLVMAMDGPUVSCallConv,
    LLVMARMAAPCSCallConv, LLVMARMAAPCSVFPCallConv, LLVMARMAPCSCallConv, LLVMAVRBUILTINCallConv,
    LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddGlobal, LLVMAddIncoming, LLVMAnyRegCallConv,
    LLVMAppendingLinkage, LLVMArrayTypeKind, LLVMAvailableExternallyLinkage, LLVMBFloatTypeKind,
    LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBitCast, LLVMBuildBr, LLVMBuildCondBr,
    LLVMBuildExtractValue, LLVMBuildICmp, LLVMBuildInsertValue, LLVMBuildLShr, LLVMBuildPhi,
    LLVMBuildPtrToInt, LLVMBuildRet, LLVMBuildTrunc, LLVMBuildXor, LLVMBuildZExt, LLVMCCallConv,
    LLVMCXXFASTTLSCallConv, LLVMCallConv, LLVMColdCallConv, LLVMCommonLinkage, LLVMConstAdd,
    LLVMConstInt, LLVMConstNamedStruct, LLVMConstReal, LLVMConstShl, LLVMConstTrunc,
    LLVMConstUIToFP, LLVMCountStructElementTypes, LLVMDLLExportLinkage, LLVMDLLImportLinkage,
    LLVMDoubleTypeKind, LLVMExternalLinkage, LLVMExternalWeakLinkage, LLVMFP128TypeKind,
    LLVMFastCallConv, LLVMFloatTypeKind, LLVMFunctionType, LLVMFunctionTypeKind, LLVMGHCCallConv,
    LLVMGetEntryBasicBlock, LLVMGetFirstInstruction, LLVMGetInsertBlock, LLVMGetIntTypeWidth,
    LLVMGetParam, LLVMGetStructElementTypes, LLVMGetTypeKind, LLVMGetUndef, LLVMGhostLinkage,
    LLVMHHVMCCallConv, LLVMHHVMCallConv, LLVMHalfTypeKind, LLVMHiPECallConv, LLVMIntEQ, LLVMIntNE,
    LLVMIntPredicate, LLVMIntSGE, LLVMIntSGT, LLVMIntSLE, LLVMIntSLT, LLVMIntUGE, LLVMIntUGT,
    LLVMIntULE, LLVMIntULT, LLVMIntegerTypeKind, LLVMIntelOCLBICallConv, LLVMInternalLinkage,
    LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage, LLVMLinkOnceODRAutoHideLinkage,
    LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage, LLVMLinkerPrivateWeakLinkage,
    LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv, LLVMMetadataTypeKind, LLVMPPC_FP128TypeKind,
    LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv, LLVMPointerType, LLVMPointerTypeKind,
    LLVMPositionBuilderAtEnd, LLVMPositionBuilderBefore, LLVMPreserveAllCallConv,
    LLVMPreserveMostCallConv, LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv,
    LLVMScalableVectorTypeKind, LLVMSetGlobalConstant, LLVMSetInitializer, LLVMSetLinkage,
    LLVMSetUnnamedAddr, LLVMStructTypeKind, LLVMSwiftCallConv, LLVMTokenTypeKind, LLVMTypeKind,
    LLVMTypeOf, LLVMVectorTypeKind, LLVMVoidTypeKind, LLVMWeakAnyLinkage, LLVMWeakODRLinkage,
    LLVMWebKitJSCallConv, LLVMWin64CallConv, LLVMX8664SysVCallConv, LLVMX86FastcallCallConv,
    LLVMX86INTRCallConv, LLVMX86RegCallCallConv, LLVMX86StdcallCallConv, LLVMX86ThisCallCallConv,
    LLVMX86VectorCallCallConv, LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind, LLVMX86_MMXTypeKind,
};
pub use self::DebugInfo_h::{
    LLVMDIBuilderCreateAutoVariable, LLVMDIBuilderCreateExpression, LLVMDIFlagAccessibility,
    LLVMDIFlagAppleBlock, LLVMDIFlagArtificial, LLVMDIFlagBigEndian, LLVMDIFlagBitField,
    LLVMDIFlagEnumClass, LLVMDIFlagExplicit, LLVMDIFlagFixedEnum, LLVMDIFlagFwdDecl,
    LLVMDIFlagIndirectVirtualBase, LLVMDIFlagIntroducedVirtual, LLVMDIFlagLValueReference,
    LLVMDIFlagLittleEndian, LLVMDIFlagMultipleInheritance, LLVMDIFlagNoReturn,
    LLVMDIFlagNonTrivial, LLVMDIFlagObjcClassComplete, LLVMDIFlagObjectPointer, LLVMDIFlagPrivate,
    LLVMDIFlagProtected, LLVMDIFlagPrototyped, LLVMDIFlagPtrToMemberRep, LLVMDIFlagPublic,
    LLVMDIFlagRValueReference, LLVMDIFlagReserved, LLVMDIFlagReservedBit4,
    LLVMDIFlagSingleInheritance, LLVMDIFlagStaticMember, LLVMDIFlagThunk,
    LLVMDIFlagTypePassByReference, LLVMDIFlagTypePassByValue, LLVMDIFlagVector, LLVMDIFlagVirtual,
    LLVMDIFlagVirtualInheritance, LLVMDIFlagZero, LLVMDIFlags,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{LLVMABIAlignmentOfType, LLVMOpaqueTargetData, LLVMTargetDataRef};
pub use self::Types_h::{
    LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    LLVMModuleRef, LLVMOpaqueBasicBlock, LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder,
    LLVMOpaqueMetadata, LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef,
    LLVMValueRef,
};
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "19:8"]
pub struct genned_string_t {
    pub string: *const libc::c_char,
    pub global: LLVMValueRef,
}
#[c2rust::src_loc = "40:1"]
pub type genned_strings_free_fn = Option<unsafe extern "C" fn(*mut genned_string_t) -> ()>;
#[c2rust::src_loc = "40:1"]
pub type genned_strings_cmp_fn =
    Option<unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool>;
#[c2rust::src_loc = "25:1"]
unsafe extern "C" fn genned_string_hash(mut s: *mut genned_string_t) -> usize {
    ponyint_hash_ptr((*s).string as *const libc::c_void)
}
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn genned_string_cmp(
    mut a: *mut genned_string_t,
    mut b: *mut genned_string_t,
) -> bool {
    (*a).string == (*b).string
}
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn genned_string_free(mut s: *mut genned_string_t) {
    ponyint_pool_free(0 as libc::c_int as usize, s as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_init(mut map: *mut genned_strings_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_mem_size(mut map: *mut genned_strings_t) -> usize {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_size(mut map: *mut genned_strings_t) -> usize {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_clearindex(
    mut map: *mut genned_strings_t,
    mut index: usize,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_removeindex(
    mut map: *mut genned_strings_t,
    mut index: usize,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_alloc_size(mut map: *mut genned_strings_t) -> usize {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_putindex(
    mut map: *mut genned_strings_t,
    mut entry: *mut genned_string_t,
    mut index: usize,
) {
    let mut cmpf: genned_strings_cmp_fn = Some(
        genned_string_cmp
            as unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        genned_string_hash(entry),
        ::core::mem::transmute::<genned_strings_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_optimize(mut map: *mut genned_strings_t) {
    let mut cmpf: genned_strings_cmp_fn = Some(
        genned_string_cmp
            as unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<genned_strings_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn genned_strings_destroy(mut map: *mut genned_strings_t) {
    let mut freef: genned_strings_free_fn =
        Some(genned_string_free as unsafe extern "C" fn(*mut genned_string_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<genned_strings_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "40:50"]
pub unsafe extern "C" fn genned_strings_get(
    mut map: *mut genned_strings_t,
    mut key: *mut genned_string_t,
    mut index: *mut usize,
) -> *mut genned_string_t {
    let mut cmpf: genned_strings_cmp_fn = Some(
        genned_string_cmp
            as unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        genned_string_hash(key),
        ::core::mem::transmute::<genned_strings_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut genned_string_t
}
#[no_mangle]
#[c2rust::src_loc = "40:50"]
pub unsafe extern "C" fn genned_strings_remove(
    mut map: *mut genned_strings_t,
    mut entry: *mut genned_string_t,
) -> *mut genned_string_t {
    let mut cmpf: genned_strings_cmp_fn = Some(
        genned_string_cmp
            as unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        genned_string_hash(entry),
        ::core::mem::transmute::<genned_strings_cmp_fn, cmp_fn>(cmpf),
    ) as *mut genned_string_t
}
#[no_mangle]
#[c2rust::src_loc = "40:50"]
pub unsafe extern "C" fn genned_strings_next(
    mut map: *mut genned_strings_t,
    mut i: *mut usize,
) -> *mut genned_string_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut genned_string_t
}
#[no_mangle]
#[c2rust::src_loc = "40:50"]
pub unsafe extern "C" fn genned_strings_put(
    mut map: *mut genned_strings_t,
    mut entry: *mut genned_string_t,
) -> *mut genned_string_t {
    let mut cmpf: genned_strings_cmp_fn = Some(
        genned_string_cmp
            as unsafe extern "C" fn(*mut genned_string_t, *mut genned_string_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        genned_string_hash(entry),
        ::core::mem::transmute::<genned_strings_cmp_fn, cmp_fn>(cmpf),
    ) as *mut genned_string_t
}
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn gen_this(mut c: *mut compile_t, mut _ast: *mut ast_t) -> LLVMValueRef {
    return LLVMGetParam(codegen_fun(c), 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn gen_param(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut def: *mut ast_t = ast_data(ast) as *mut ast_t;
    if !def.is_null() {
    } else {
        ponyint_assert_fail(
            b"def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            52 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"gen_param\0")).as_ptr(),
        );
    };
    let mut index: libc::c_int = ast_index(def) as libc::c_int;
    if !(*(*c).frame).bare_function {
        index += 1;
    }
    LLVMGetParam(codegen_fun(c), index as libc::c_uint)
}
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn make_fieldptr(
    mut c: *mut compile_t,
    mut l_value: LLVMValueRef,
    mut l_type: *mut ast_t,
    mut right: *mut ast_t,
) -> LLVMValueRef {
    if ast_id(l_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(l_type) == TK_NOMINAL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            64 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"make_fieldptr\0"))
                .as_ptr(),
        );
    };
    if ast_id(right) as libc::c_uint == TK_ID as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(right) == TK_ID\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            65 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"make_fieldptr\0"))
                .as_ptr(),
        );
    };
    let mut def: *mut ast_t = 0 as *mut ast_t;
    let mut field: *mut ast_t = 0 as *mut ast_t;
    let mut index: u32 = 0;
    get_fieldinfo(l_type, right, &mut def, &mut field, &mut index);
    if ast_id(def) as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
        index = index.wrapping_add(1);
    }
    if ast_id(def) as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
        index = index.wrapping_add(1);
    }
    LLVMBuildStructGEP_P(
        (*c).builder,
        l_value,
        index,
        b"\0" as *const u8 as *const libc::c_char,
    )
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn gen_fieldptr(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut l_value: LLVMValueRef = gen_expr(c, left);
    if l_value.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut l_type: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(left), (*c).opt);
    let mut ret: LLVMValueRef = make_fieldptr(c, l_value, l_type, right);
    ast_free_unattached(l_type);
    ret
}
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn gen_fieldload(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut field: LLVMValueRef = gen_fieldptr(c, ast);
    if field.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    let mut type_0: *mut ast_t = deferred_reify(reify, ast_type(right), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            109 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gen_fieldload\0"))
                .as_ptr(),
        );
    };
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    field = LLVMBuildLoad_P(
        (*c).builder,
        field,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gen_assign_cast(c, (*c_t).use_type, field, (*t).ast_cap)
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn gen_fieldembed(
    mut c: *mut compile_t,
    mut ast: *mut ast_t,
) -> LLVMValueRef {
    let mut field: LLVMValueRef = gen_fieldptr(c, ast);
    if field.is_null() {
        return 0 as LLVMValueRef;
    }
    field
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn make_tupleelemptr(
    mut c: *mut compile_t,
    mut l_value: LLVMValueRef,
    mut l_type: *mut ast_t,
    mut right: *mut ast_t,
) -> LLVMValueRef {
    if ast_id(l_type) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
    } else {
        ponyint_assert_fail(
            b"ast_id(l_type) == TK_TUPLETYPE\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            133 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"make_tupleelemptr\0"))
                .as_ptr(),
        );
    };
    let mut index: libc::c_int = (*ast_int(right)).low as libc::c_int;
    LLVMBuildExtractValue(
        (*c).builder,
        l_value,
        index as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    )
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn gen_tupleelemptr(
    mut c: *mut compile_t,
    mut ast: *mut ast_t,
) -> LLVMValueRef {
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut right: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut left, &mut right, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut l_value: LLVMValueRef = gen_expr(c, left);
    if l_value.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    let mut type_0: *mut ast_t = deferred_reify(reify, ast_type(ast), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            152 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"gen_tupleelemptr\0"))
                .as_ptr(),
        );
    };
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut l_type: *mut ast_t = deferred_reify(reify, ast_type(left), (*c).opt);
    let mut value: LLVMValueRef = make_tupleelemptr(c, l_value, l_type, right);
    ast_free_unattached(l_type);
    gen_assign_cast(c, (*c_t).use_type, value, (*t).ast_cap)
}
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub unsafe extern "C" fn gen_tuple(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut child: *mut ast_t = ast_child(ast);
    if (ast_sibling(child)).is_null() {
        return gen_expr(c, child);
    }
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    let mut type_0: *mut ast_t = deferred_reify(reify, ast_type(ast), (*c).opt);
    if contains_dontcare(type_0) {
        ast_free_unattached(type_0);
        return 2 as libc::c_int as LLVMValueRef;
    }
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut count: libc::c_int = LLVMCountStructElementTypes((*c_t).primitive) as libc::c_int;
    let mut buf_size: usize = (count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
    let mut elements: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
    LLVMGetStructElementTypes((*c_t).primitive, elements);
    let mut tuple: LLVMValueRef = LLVMGetUndef((*c_t).primitive);
    let mut i: libc::c_int = 0 as libc::c_int;
    while !child.is_null() {
        let mut value: LLVMValueRef = gen_expr(c, child);
        if value.is_null() {
            ponyint_pool_free_size(buf_size, elements as *mut libc::c_void);
            return 0 as LLVMValueRef;
        }
        if value == 1 as libc::c_int as LLVMValueRef || value == 2 as libc::c_int as LLVMValueRef {
            ponyint_pool_free_size(buf_size, elements as *mut libc::c_void);
            return value;
        }
        let mut child_type: *mut ast_t = deferred_reify(reify, ast_type(child), (*c).opt);
        value = gen_assign_cast(c, *elements.offset(i as isize), value, child_type);
        ast_free_unattached(child_type);
        let fresh0 = i;
        i += 1;
        tuple = LLVMBuildInsertValue(
            (*c).builder,
            tuple,
            value,
            fresh0 as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        child = ast_sibling(child);
    }
    ponyint_pool_free_size(buf_size, elements as *mut libc::c_void);
    tuple
}
#[no_mangle]
#[c2rust::src_loc = "220:1"]
pub unsafe extern "C" fn gen_localdecl(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut id: *mut ast_t = ast_child(ast);
    let mut name: *const libc::c_char = ast_name(id);
    let mut value: LLVMValueRef = codegen_getlocal(c, name);
    if !value.is_null() {
        return 1 as libc::c_int as LLVMValueRef;
    }
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(id), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut this_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
    let mut entry_block: LLVMBasicBlockRef = LLVMGetEntryBasicBlock(codegen_fun(c));
    let mut inst: LLVMValueRef = LLVMGetFirstInstruction(entry_block);
    if !inst.is_null() {
        LLVMPositionBuilderBefore((*c).builder, inst);
    } else {
        LLVMPositionBuilderAtEnd((*c).builder, entry_block);
    }
    let mut alloc: LLVMValueRef = LLVMBuildAlloca((*c).builder, (*c_t).mem_type, name);
    codegen_setlocal(c, name, alloc);
    let mut file: LLVMMetadataRef = codegen_difile(c);
    let mut scope: LLVMMetadataRef = codegen_discope(c);
    let mut align_bytes: u32 = LLVMABIAlignmentOfType((*c).target_data, (*c_t).mem_type);
    let mut info: LLVMMetadataRef = LLVMDIBuilderCreateAutoVariable(
        (*c).di,
        scope,
        name,
        strlen(name),
        file,
        ast_line(ast) as libc::c_uint,
        (*c_t).di_type,
        1 as libc::c_int,
        LLVMDIFlagZero,
        align_bytes.wrapping_mul(8 as libc::c_int as libc::c_uint),
    );
    let mut expr: LLVMMetadataRef =
        LLVMDIBuilderCreateExpression((*c).di, 0 as *mut u64, 0 as libc::c_int as usize);
    LLVMDIBuilderInsertDeclare(
        (*c).di,
        alloc,
        info,
        expr,
        ast_line(ast) as libc::c_uint,
        ast_pos(ast) as libc::c_uint,
        scope,
        LLVMGetInsertBlock((*c).builder),
    );
    LLVMPositionBuilderAtEnd((*c).builder, this_block);
    return 2 as libc::c_int as LLVMValueRef;
}
#[no_mangle]
#[c2rust::src_loc = "273:1"]
pub unsafe extern "C" fn gen_localptr(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut id: *mut ast_t = ast_child(ast);
    let mut name: *const libc::c_char = ast_name(id);
    let mut value: LLVMValueRef = codegen_getlocal(c, name);
    if !value.is_null() {
    } else {
        ponyint_assert_fail(
            b"value != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            279 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"gen_localptr\0")).as_ptr(),
        );
    };
    return value;
}
#[no_mangle]
#[c2rust::src_loc = "284:1"]
pub unsafe extern "C" fn gen_localload(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut local_ptr: LLVMValueRef = gen_localptr(c, ast);
    if local_ptr.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(ast), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut value: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        local_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return gen_assign_cast(c, (*c_t).use_type, value, (*t).ast_cap);
}
#[no_mangle]
#[c2rust::src_loc = "300:1"]
pub unsafe extern "C" fn gen_addressof(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut expr: *mut ast_t = ast_child(ast);
    match ast_id(expr) as libc::c_uint {
        196 => return gen_localptr(c, expr),
        192 => return gen_fieldptr(c, expr),
        191 | 190 => return gen_funptr(c, expr),
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            319 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gen_addressof\0"))
                .as_ptr(),
        );
    };
    return 0 as LLVMValueRef;
}
#[c2rust::src_loc = "323:1"]
unsafe extern "C" fn gen_digestof_box(
    mut c: *mut compile_t,
    mut type_0: *mut reach_type_t,
    mut value: LLVMValueRef,
    mut boxed_subtype: libc::c_int,
) -> LLVMValueRef {
    if LLVMGetTypeKind(LLVMTypeOf(value)) as libc::c_uint
        == LLVMPointerTypeKind as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"LLVMGetTypeKind(LLVMTypeOf(value)) == LLVMPointerTypeKind\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            326 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"gen_digestof_box\0"))
                .as_ptr(),
        );
    };
    let mut box_block: LLVMBasicBlockRef = 0 as LLVMBasicBlockRef;
    let mut nonbox_block: LLVMBasicBlockRef = 0 as LLVMBasicBlockRef;
    let mut post_block: LLVMBasicBlockRef = 0 as LLVMBasicBlockRef;
    let mut desc: LLVMValueRef = gendesc_fetch(c, value);
    if boxed_subtype & SUBTYPE_KIND_UNBOXED as libc::c_int != 0 as libc::c_int {
        box_block = codegen_block(c, b"digestof_box\0" as *const u8 as *const libc::c_char);
        nonbox_block = codegen_block(c, b"digestof_nonbox\0" as *const u8 as *const libc::c_char);
        post_block = codegen_block(c, b"digestof_post\0" as *const u8 as *const libc::c_char);
        let mut type_id: LLVMValueRef = gendesc_typeid(c, desc);
        let mut boxed_mask: LLVMValueRef = LLVMConstInt(
            (*c).i32_0,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        let mut is_boxed: LLVMValueRef = LLVMBuildAnd(
            (*c).builder,
            type_id,
            boxed_mask,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut zero: LLVMValueRef = LLVMConstInt(
            (*c).i32_0,
            0 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        is_boxed = LLVMBuildICmp(
            (*c).builder,
            LLVMIntEQ,
            is_boxed,
            zero,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildCondBr((*c).builder, is_boxed, box_block, nonbox_block);
        LLVMPositionBuilderAtEnd((*c).builder, box_block);
    }
    let mut digest_fn: *mut reach_method_t = reach_method(
        type_0,
        TK_BOX,
        stringtab(b"__digestof\0" as *const u8 as *const libc::c_char),
        0 as *mut ast_t,
    );
    if !digest_fn.is_null() {
    } else {
        ponyint_assert_fail(
            b"digest_fn != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            353 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"gen_digestof_box\0"))
                .as_ptr(),
        );
    };
    let mut func: LLVMValueRef = gendesc_vtable(c, desc, (*digest_fn).vtable_index as usize);
    let mut fn_type: LLVMTypeRef = LLVMFunctionType(
        (*c).intptr,
        &mut (*c).object_ptr,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    func = LLVMBuildBitCast(
        (*c).builder,
        func,
        LLVMPointerType(fn_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut box_digest: LLVMValueRef = codegen_call(
        c,
        func,
        &mut value,
        1 as libc::c_int as usize,
        1 as libc::c_int != 0,
    );
    if boxed_subtype & SUBTYPE_KIND_UNBOXED as libc::c_int != 0 as libc::c_int {
        LLVMBuildBr((*c).builder, post_block);
        LLVMPositionBuilderAtEnd((*c).builder, nonbox_block);
        let mut nonbox_digest: LLVMValueRef = LLVMBuildPtrToInt(
            (*c).builder,
            value,
            (*c).intptr,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildBr((*c).builder, post_block);
        LLVMPositionBuilderAtEnd((*c).builder, post_block);
        let mut phi: LLVMValueRef = LLVMBuildPhi(
            (*c).builder,
            (*c).intptr,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMAddIncoming(
            phi,
            &mut box_digest,
            &mut box_block,
            1 as libc::c_int as libc::c_uint,
        );
        LLVMAddIncoming(
            phi,
            &mut nonbox_digest,
            &mut nonbox_block,
            1 as libc::c_int as libc::c_uint,
        );
        return phi;
    } else {
        return box_digest;
    };
}
#[c2rust::src_loc = "379:1"]
unsafe extern "C" fn gen_digestof_int64(
    mut c: *mut compile_t,
    mut value: LLVMValueRef,
) -> LLVMValueRef {
    if LLVMTypeOf(value) == (*c).i64_0 {
    } else {
        ponyint_assert_fail(
            b"LLVMTypeOf(value) == c->i64\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            381 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"gen_digestof_int64\0"))
                .as_ptr(),
        );
    };
    if target_is_ilp32((*(*c).opt).triple) {
        let mut shift: LLVMValueRef = LLVMConstInt(
            (*c).i64_0,
            32 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        let mut high: LLVMValueRef = LLVMBuildLShr(
            (*c).builder,
            value,
            shift,
            b"\0" as *const u8 as *const libc::c_char,
        );
        high = LLVMBuildTrunc(
            (*c).builder,
            high,
            (*c).i32_0,
            b"\0" as *const u8 as *const libc::c_char,
        );
        value = LLVMBuildTrunc(
            (*c).builder,
            value,
            (*c).i32_0,
            b"\0" as *const u8 as *const libc::c_char,
        );
        value = LLVMBuildXor(
            (*c).builder,
            value,
            high,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    return value;
}
#[c2rust::src_loc = "395:1"]
unsafe extern "C" fn gen_digestof_value(
    mut c: *mut compile_t,
    mut type_0: *mut ast_t,
    mut value: LLVMValueRef,
) -> LLVMValueRef {
    let mut impl_type: LLVMTypeRef = LLVMTypeOf(value);
    match LLVMGetTypeKind(impl_type) as libc::c_uint {
        2 => {
            value = LLVMBuildBitCast(
                (*c).builder,
                value,
                (*c).i32_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
            return LLVMBuildZExt(
                (*c).builder,
                value,
                (*c).intptr,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            value = LLVMBuildBitCast(
                (*c).builder,
                value,
                (*c).i64_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
            return gen_digestof_int64(c, value);
        }
        8 => {
            let mut width: u32 = LLVMGetIntTypeWidth(impl_type);
            if width < 64 as libc::c_int as libc::c_uint {
                return LLVMBuildZExt(
                    (*c).builder,
                    value,
                    (*c).intptr,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            } else {
                if width == 64 as libc::c_int as libc::c_uint {
                    return gen_digestof_int64(c, value);
                } else {
                    if width == 128 as libc::c_int as libc::c_uint {
                        let mut shift: LLVMValueRef = LLVMConstInt(
                            (*c).i128_0,
                            64 as libc::c_int as libc::c_ulonglong,
                            0 as libc::c_int,
                        );
                        let mut high: LLVMValueRef = LLVMBuildLShr(
                            (*c).builder,
                            value,
                            shift,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        high = LLVMBuildTrunc(
                            (*c).builder,
                            high,
                            (*c).i64_0,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        value = LLVMBuildTrunc(
                            (*c).builder,
                            value,
                            (*c).i64_0,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        high = gen_digestof_int64(c, high);
                        value = gen_digestof_int64(c, value);
                        return LLVMBuildXor(
                            (*c).builder,
                            value,
                            high,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        10 => {
            let mut count: u32 = LLVMCountStructElementTypes(impl_type);
            let mut result: LLVMValueRef = LLVMConstInt(
                (*c).intptr,
                0 as libc::c_int as libc::c_ulonglong,
                0 as libc::c_int,
            );
            let mut child: *mut ast_t = ast_child(type_0);
            let mut i: u32 = 0 as libc::c_int as u32;
            while i < count {
                let mut elem: LLVMValueRef = LLVMBuildExtractValue(
                    (*c).builder,
                    value,
                    i,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                elem = gen_digestof_value(c, child, elem);
                result = LLVMBuildXor(
                    (*c).builder,
                    result,
                    elem,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                child = ast_sibling(child);
                i = i.wrapping_add(1);
            }
            if child.is_null() {
            } else {
                ponyint_assert_fail(
                    b"child == NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                        as *const u8 as *const libc::c_char,
                    445 as libc::c_int as usize,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"gen_digestof_value\0"))
                        .as_ptr(),
                );
            };
            return result;
        }
        12 => {
            if !is_known(type_0) {
                let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
                let mut sub_kind: libc::c_int = subtype_kind(t);
                if sub_kind & SUBTYPE_KIND_BOXED as libc::c_int != 0 as libc::c_int {
                    return gen_digestof_box(c, t, value, sub_kind);
                }
            }
            return LLVMBuildPtrToInt(
                (*c).builder,
                value,
                (*c).intptr,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            465 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"gen_digestof_value\0"))
                .as_ptr(),
        );
    };
    return 0 as LLVMValueRef;
}
#[no_mangle]
#[c2rust::src_loc = "469:1"]
pub unsafe extern "C" fn gen_digestof(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut expr: *mut ast_t = ast_child(ast);
    let mut value: LLVMValueRef = gen_expr(c, expr);
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(expr), (*c).opt);
    let mut ret: LLVMValueRef = gen_digestof_value(c, type_0, value);
    ast_free_unattached(type_0);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "479:1"]
pub unsafe extern "C" fn gen_digestof_fun(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    if (*t).can_be_boxed as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"t->can_be_boxed\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            481 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"gen_digestof_fun\0"))
                .as_ptr(),
        );
    };
    let mut m: *mut reach_method_t = reach_method(
        t,
        TK_BOX,
        stringtab(b"__digestof\0" as *const u8 as *const libc::c_char),
        0 as *mut ast_t,
    );
    if m.is_null() {
        return;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let ref mut fresh1 = (*c_m).func_type;
    *fresh1 = LLVMFunctionType(
        (*c).intptr,
        &mut (*c_t).structure_ptr,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh2 = (*c_m).func;
    *fresh2 = codegen_addfun(c, (*m).full_name, (*c_m).func_type, 1 as libc::c_int != 0);
    codegen_startfun(
        c,
        (*c_m).func,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    let mut value: LLVMValueRef = LLVMGetParam(codegen_fun(c), 0 as libc::c_int as libc::c_uint);
    value = gen_unbox(c, (*t).ast_cap, value);
    LLVMBuildRet((*c).builder, gen_digestof_value(c, (*t).ast_cap, value));
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "502:1"]
pub unsafe extern "C" fn gen_int(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(ast), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut value: *mut lexint_t = ast_int(ast);
    let mut vlow: LLVMValueRef = LLVMConstInt((*c).i128_0, (*value).low, 0 as libc::c_int);
    let mut vhigh: LLVMValueRef = LLVMConstInt((*c).i128_0, (*value).high, 0 as libc::c_int);
    let mut shift: LLVMValueRef = LLVMConstInt(
        (*c).i128_0,
        64 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    vhigh = LLVMConstShl(vhigh, shift);
    vhigh = LLVMConstAdd(vhigh, vlow);
    if (*c_t).primitive == (*c).i128_0 {
        return vhigh;
    }
    if (*c_t).primitive == (*c).f32_0 || (*c_t).primitive == (*c).f64_0 {
        return LLVMConstUIToFP(vhigh, (*c_t).primitive);
    }
    return LLVMConstTrunc(vhigh, (*c_t).primitive);
}
#[no_mangle]
#[c2rust::src_loc = "525:1"]
pub unsafe extern "C" fn gen_float(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(ast), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    ast_free_unattached(type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    return LLVMConstReal((*c_t).primitive, ast_float(ast));
}
#[no_mangle]
#[c2rust::src_loc = "535:1"]
pub unsafe extern "C" fn gen_string(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut name: *const libc::c_char = ast_name(ast);
    let mut k: genned_string_t = genned_string_t {
        string: 0 as *const libc::c_char,
        global: 0 as *mut LLVMOpaqueValue,
    };
    k.string = name;
    let mut index: usize = -(1 as libc::c_int) as usize;
    let mut string: *mut genned_string_t =
        genned_strings_get(&mut (*c).strings, &mut k, &mut index);
    if !string.is_null() {
        return (*string).global;
    }
    let mut type_0: *mut ast_t = ast_type(ast);
    if is_literal(type_0, b"String\0" as *const u8 as *const libc::c_char) as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"is_literal(type, \"String\")\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.c\0"
                as *const u8 as *const libc::c_char,
            548 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"gen_string\0")).as_ptr(),
        );
    };
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut len: usize = ast_name_len(ast);
    let mut args: [LLVMValueRef; 4] = [0 as *mut LLVMOpaqueValue; 4];
    args[0 as libc::c_int as usize] = (*c_t).desc;
    args[1 as libc::c_int as usize] =
        LLVMConstInt((*c).intptr, len as libc::c_ulonglong, 0 as libc::c_int);
    args[2 as libc::c_int as usize] = LLVMConstInt(
        (*c).intptr,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[3 as libc::c_int as usize] = codegen_string(c, name, len);
    let mut inst: LLVMValueRef = LLVMConstNamedStruct(
        (*c_t).structure,
        args.as_mut_ptr(),
        4 as libc::c_int as libc::c_uint,
    );
    let mut g_inst: LLVMValueRef = LLVMAddGlobal(
        (*c).module,
        (*c_t).structure,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMSetInitializer(g_inst, inst);
    LLVMSetGlobalConstant(g_inst, 1 as libc::c_int);
    LLVMSetLinkage(g_inst, LLVMPrivateLinkage);
    LLVMSetUnnamedAddr(g_inst, 1 as libc::c_int);
    string = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut genned_string_t;
    let ref mut fresh3 = (*string).string;
    *fresh3 = name;
    let ref mut fresh4 = (*string).global;
    *fresh4 = g_inst;
    genned_strings_putindex(&mut (*c).strings, string, index);
    return g_inst;
}
