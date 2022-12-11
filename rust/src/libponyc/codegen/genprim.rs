use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:1"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
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
    #[c2rust::src_loc = "143:1"]
    pub type LLVMAttributeRef = *mut LLVMOpaqueAttributeRef;
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
        #[c2rust::src_loc = "143:16"]
        pub type LLVMOpaqueAttributeRef;
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
    #[c2rust::src_loc = "303:9"]
    pub type LLVMRealPredicate = libc::c_uint;
    #[c2rust::src_loc = "319:3"]
    pub const LLVMRealPredicateTrue: LLVMRealPredicate = 15;
    #[c2rust::src_loc = "318:3"]
    pub const LLVMRealUNE: LLVMRealPredicate = 14;
    #[c2rust::src_loc = "317:3"]
    pub const LLVMRealULE: LLVMRealPredicate = 13;
    #[c2rust::src_loc = "316:3"]
    pub const LLVMRealULT: LLVMRealPredicate = 12;
    #[c2rust::src_loc = "315:3"]
    pub const LLVMRealUGE: LLVMRealPredicate = 11;
    #[c2rust::src_loc = "314:3"]
    pub const LLVMRealUGT: LLVMRealPredicate = 10;
    #[c2rust::src_loc = "313:3"]
    pub const LLVMRealUEQ: LLVMRealPredicate = 9;
    #[c2rust::src_loc = "312:3"]
    pub const LLVMRealUNO: LLVMRealPredicate = 8;
    #[c2rust::src_loc = "311:3"]
    pub const LLVMRealORD: LLVMRealPredicate = 7;
    #[c2rust::src_loc = "310:3"]
    pub const LLVMRealONE: LLVMRealPredicate = 6;
    #[c2rust::src_loc = "309:3"]
    pub const LLVMRealOLE: LLVMRealPredicate = 5;
    #[c2rust::src_loc = "308:3"]
    pub const LLVMRealOLT: LLVMRealPredicate = 4;
    #[c2rust::src_loc = "307:3"]
    pub const LLVMRealOGE: LLVMRealPredicate = 3;
    #[c2rust::src_loc = "306:3"]
    pub const LLVMRealOGT: LLVMRealPredicate = 2;
    #[c2rust::src_loc = "305:3"]
    pub const LLVMRealOEQ: LLVMRealPredicate = 1;
    #[c2rust::src_loc = "304:3"]
    pub const LLVMRealPredicateFalse: LLVMRealPredicate = 0;
    #[c2rust::src_loc = "395:9"]
    pub type LLVMInlineAsmDialect = libc::c_uint;
    #[c2rust::src_loc = "397:3"]
    pub const LLVMInlineAsmDialectIntel: LLVMInlineAsmDialect = 1;
    #[c2rust::src_loc = "396:3"]
    pub const LLVMInlineAsmDialectATT: LLVMInlineAsmDialect = 0;
    #[c2rust::src_loc = "455:1"]
    pub type C2RustUnnamed = libc::c_int;
    #[c2rust::src_loc = "460:3"]
    pub const LLVMAttributeFunctionIndex: C2RustUnnamed = -1;
    #[c2rust::src_loc = "456:3"]
    pub const LLVMAttributeReturnIndex: C2RustUnnamed = 0;
    #[c2rust::src_loc = "463:1"]
    pub type LLVMAttributeIndex = libc::c_uint;
    use super::Types_h::{
        LLVMAttributeRef, LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef,
        LLVMModuleRef, LLVMTypeRef,
        LLVMValueRef,
    };
    use super::_size_t_h::size_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "589:1"]
        pub fn LLVMGetEnumAttributeKindForName(
            Name: *const libc::c_char,
            SLen: size_t,
        ) -> libc::c_uint;
        #[c2rust::src_loc = "595:1"]
        pub fn LLVMCreateEnumAttribute(
            C: LLVMContextRef,
            KindID: libc::c_uint,
            Val: uint64_t,
        ) -> LLVMAttributeRef;
        #[c2rust::src_loc = "877:1"]
        pub fn LLVMGetInlineAsm(
            Ty: LLVMTypeRef,
            AsmString: *const libc::c_char,
            AsmStringSize: size_t,
            Constraints: *const libc::c_char,
            ConstraintsSize: size_t,
            HasSideEffects: LLVMBool,
            IsAlignStack: LLVMBool,
            Dialect: LLVMInlineAsmDialect,
            CanThrow: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "1023:1"]
        pub fn LLVMAddFunction(
            M: LLVMModuleRef,
            Name: *const libc::c_char,
            FunctionTy: LLVMTypeRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "1106:1"]
        pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
        #[c2rust::src_loc = "1246:1"]
        pub fn LLVMFunctionType(
            ReturnType: LLVMTypeRef,
            ParamTypes: *mut LLVMTypeRef,
            ParamCount: libc::c_uint,
            IsVarArg: LLVMBool,
        ) -> LLVMTypeRef;
        #[c2rust::src_loc = "1300:1"]
        pub fn LLVMStructTypeInContext(
            C: LLVMContextRef,
            ElementTypes: *mut LLVMTypeRef,
            ElementCount: libc::c_uint,
            Packed: LLVMBool,
        ) -> LLVMTypeRef;
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
        #[c2rust::src_loc = "2214:1"]
        pub fn LLVMConstInlineAsm(
            Ty: LLVMTypeRef,
            AsmString: *const libc::c_char,
            Constraints: *const libc::c_char,
            HasSideEffects: LLVMBool,
            IsAlignStack: LLVMBool,
        ) -> LLVMValueRef;
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
        #[c2rust::src_loc = "2582:1"]
        pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "2604:1"]
        pub fn LLVMAddAttributeAtIndex(
            F: LLVMValueRef,
            Idx: LLVMAttributeIndex,
            A: LLVMAttributeRef,
        );
        #[c2rust::src_loc = "2665:1"]
        pub fn LLVMGetParam(Fn: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
        #[c2rust::src_loc = "3112:1"]
        pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3312:1"]
        pub fn LLVMAddCallSiteAttribute(
            C: LLVMValueRef,
            Idx: LLVMAttributeIndex,
            A: LLVMAttributeRef,
        );
        #[c2rust::src_loc = "3538:1"]
        pub fn LLVMAddIncoming(
            PhiNode: LLVMValueRef,
            IncomingValues: *mut LLVMValueRef,
            IncomingBlocks: *mut LLVMBasicBlockRef,
            Count: libc::c_uint,
        );
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3681:1"]
        pub fn LLVMBuildRetVoid(_: LLVMBuilderRef) -> LLVMValueRef;
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
        #[c2rust::src_loc = "3791:1"]
        pub fn LLVMBuildAdd(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3807:1"]
        pub fn LLVMBuildMul(
            _: LLVMBuilderRef,
            LHS: LLVMValueRef,
            RHS: LLVMValueRef,
            Name: *const libc::c_char,
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
        #[c2rust::src_loc = "3852:1"]
        pub fn LLVMBuildNot(
            _: LLVMBuilderRef,
            V: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3887:1"]
        pub fn LLVMBuildAlloca(
            _: LLVMBuilderRef,
            Ty: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3897:1"]
        pub fn LLVMBuildStore(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            Ptr: LLVMValueRef,
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
        #[c2rust::src_loc = "3939:1"]
        pub fn LLVMBuildSExt(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3941:1"]
        pub fn LLVMBuildFPToUI(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3943:1"]
        pub fn LLVMBuildFPToSI(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3945:1"]
        pub fn LLVMBuildUIToFP(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3947:1"]
        pub fn LLVMBuildSIToFP(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3949:1"]
        pub fn LLVMBuildFPTrunc(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3951:1"]
        pub fn LLVMBuildFPExt(
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
        #[c2rust::src_loc = "3985:1"]
        pub fn LLVMBuildFCmp(
            _: LLVMBuilderRef,
            Op: LLVMRealPredicate,
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
        #[c2rust::src_loc = "3999:1"]
        pub fn LLVMBuildSelect(
            _: LLVMBuilderRef,
            If: LLVMValueRef,
            Then: LLVMValueRef,
            Else: LLVMValueRef,
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
    use super::Types_h::{LLVMTypeRef};
    extern "C" {
        #[c2rust::src_loc = "37:16"]
        pub type LLVMOpaqueTargetData;
        #[c2rust::src_loc = "257:1"]
        pub fn LLVMABISizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> libc::c_ulonglong;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/pony.h:1"]
pub mod pony_h {
    #[c2rust::src_loc = "371:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "375:3"]
    pub const PONY_TRACE_OPAQUE: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "374:3"]
    pub const PONY_TRACE_IMMUTABLE: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "373:3"]
    pub const PONY_TRACE_MUTABLE: C2RustUnnamed_0 = 0;
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
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::{token_id};
    extern "C" {
        #[c2rust::src_loc = "70:1"]
        pub fn ast_setid(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "108:1"]
        pub fn ast_nearest(ast: *mut ast_t, id: token_id) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "190:1"]
        pub fn ast_get_children(
            parent: *mut ast_t,
            child_count: size_t,
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
        pub object_type_count: uint32_t,
        pub numeric_type_count: uint32_t,
        pub tuple_type_count: uint32_t,
        pub total_type_count: uint32_t,
        pub trait_type_count: uint32_t,
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::hash_h::hashmap_t;
    use super::pass_h::pass_opt_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "18:35"]
        pub type reach_method_stack_t;
        #[c2rust::src_loc = "19:59"]
        pub fn reach_methods_next(map: *mut reach_methods_t, i: *mut size_t)
            -> *mut reach_method_t;
        #[c2rust::src_loc = "133:1"]
        pub fn reach(
            r: *mut reach_t,
            type_0: *mut ast_t,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
        );
        #[c2rust::src_loc = "136:1"]
        pub fn reach_type(r: *mut reach_t, type_0: *mut ast_t) -> *mut reach_type_t;
        #[c2rust::src_loc = "138:1"]
        pub fn reach_type_name(r: *mut reach_t, name: *const libc::c_char) -> *mut reach_type_t;
        #[c2rust::src_loc = "140:1"]
        pub fn reach_method(
            t: *mut reach_type_t,
            cap: token_id,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
        ) -> *mut reach_method_t;
        #[c2rust::src_loc = "143:1"]
        pub fn reach_method_name(
            t: *mut reach_type_t,
            name: *const libc::c_char,
        ) -> *mut reach_method_name_t;
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
        LLVMModuleRef,
        LLVMTypeRef, LLVMValueRef,
    };
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn LLVMConstNaN(type_0: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMConstInf(type_0: LLVMTypeRef, negative: bool) -> LLVMValueRef;
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMBuildStructGEP_P(
            B: LLVMBuilderRef,
            Pointer: LLVMValueRef,
            Idx: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "37:1"]
        pub fn LLVMConstInBoundsGEP_P(
            ConstantVal: LLVMValueRef,
            ConstantIndices: *mut LLVMValueRef,
            NumIndices: libc::c_uint,
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
        #[c2rust::src_loc = "43:1"]
        pub fn LLVMBuildCall_P(
            B: LLVMBuilderRef,
            Fn: LLVMValueRef,
            Args: *mut LLVMValueRef,
            NumArgs: libc::c_uint,
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
        #[c2rust::src_loc = "283:1"]
        pub fn codegen_ctx(c: *mut compile_t) -> LLVMValueRef;
        #[c2rust::src_loc = "289:1"]
        pub fn codegen_block(c: *mut compile_t, name: *const libc::c_char) -> LLVMBasicBlockRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.h:1"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.h:4"]
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
    use super::codegen_h::compile_t;
    use super::reach_h::{compile_opaque_free_fn, reach_method_t, reach_type_t};
    use super::Types_h::{LLVMMetadataRef, LLVMTypeRef, LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "22:1"]
        pub fn genfun_param_attrs(
            c: *mut compile_t,
            t: *mut reach_type_t,
            m: *mut reach_method_t,
            fun: LLVMValueRef,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.h:2"]
pub mod gencall_h {
    use super::codegen_h::compile_t;
    use super::Types_h::{LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn gencall_runtime(
            c: *mut compile_t,
            name: *const libc::c_char,
            args: *mut LLVMValueRef,
            count: libc::c_int,
            ret: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "30:1"]
        pub fn gencall_error(c: *mut compile_t);
        #[c2rust::src_loc = "32:1"]
        pub fn gencall_memcpy(
            c: *mut compile_t,
            dst: LLVMValueRef,
            src: LLVMValueRef,
            n: LLVMValueRef,
        );
        #[c2rust::src_loc = "35:1"]
        pub fn gencall_memmove(
            c: *mut compile_t,
            dst: LLVMValueRef,
            src: LLVMValueRef,
            n: LLVMValueRef,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:3"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:5"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn genname_serialise_trace(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "23:1"]
        pub fn genname_serialise(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "25:1"]
        pub fn genname_deserialise(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "41:1"]
        pub fn genname_unsafe(name: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:6"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn target_is_linux(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "12:1"]
        pub fn target_is_freebsd(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn target_is_dragonfly(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "14:1"]
        pub fn target_is_openbsd(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "15:1"]
        pub fn target_is_macosx(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "16:1"]
        pub fn target_is_windows(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "18:1"]
        pub fn target_is_x86(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "19:1"]
        pub fn target_is_arm(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn target_is_lp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn target_is_llp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "26:1"]
        pub fn target_is_native128(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "27:1"]
        pub fn target_is_bigendian(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "28:1"]
        pub fn target_is_littleendian(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genserialise.h:7"]
pub mod genserialise_h {
    use super::codegen_h::compile_t;
    use super::gentype_h::compile_type_t;
    use super::reach_h::reach_type_t;
    use super::Types_h::{LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn genserialise_element(
            c: *mut compile_t,
            t: *mut reach_type_t,
            embed: bool,
            ctx: LLVMValueRef,
            ptr: LLVMValueRef,
            offset: LLVMValueRef,
        );
        #[c2rust::src_loc = "14:1"]
        pub fn genserialise_typeid(c: *mut compile_t, t: *mut reach_type_t, offset: LLVMValueRef);
        #[c2rust::src_loc = "16:1"]
        pub fn gendeserialise_typeid(
            c: *mut compile_t,
            t: *mut compile_type_t,
            offset: LLVMValueRef,
        );
        #[c2rust::src_loc = "18:1"]
        pub fn gendeserialise_element(
            c: *mut compile_t,
            t: *mut reach_type_t,
            embed: bool,
            ctx: LLVMValueRef,
            ptr: LLVMValueRef,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.h:8"]
pub mod gentrace_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::{LLVMValueRef};
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gentrace_needed(
            c: *mut compile_t,
            src_type: *mut ast_t,
            dst_type: *mut ast_t,
        ) -> bool;
        #[c2rust::src_loc = "13:1"]
        pub fn gentrace(
            c: *mut compile_t,
            ctx: LLVMValueRef,
            src_value: LLVMValueRef,
            dst_value: LLVMValueRef,
            src_type: *mut ast_t,
            dst_type: *mut ast_t,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/program.h:11"]
pub mod program_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn program_signature(program: *mut ast_t) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:13"]
pub mod assemble_h {
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn type_builtin(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            name: *const libc::c_char,
        ) -> *mut ast_t;
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
            line: size_t,
            func: *const libc::c_char,
        );
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
use self::assemble_h::type_builtin;
pub use self::ast_h::{
    ast_child, ast_childidx, ast_data, ast_free_unattached, ast_get, ast_get_children, ast_id,
    ast_name, ast_nearest, ast_ptr_t, ast_setid, ast_sibling,
};
pub use self::codegen_h::{
    codegen_addfun, codegen_block, codegen_ctx, codegen_finishfun, codegen_startfun,
    compile_frame_t, compile_locals_t, compile_t, ffi_decls_t, genned_strings_t, LLVMBuildCall_P,
    LLVMBuildInBoundsGEP_P, LLVMBuildLoad_P, LLVMBuildStructGEP_P, LLVMConstInBoundsGEP_P,
    LLVMConstInf, LLVMConstNaN,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::gencall_h::{gencall_error, gencall_memcpy, gencall_memmove, gencall_runtime};
use self::genexpr_h::gen_assign_cast;
pub use self::genfun_h::{compile_method_t, genfun_param_attrs};
use self::genname_h::{
    genname_deserialise, genname_serialise, genname_serialise_trace, genname_unsafe,
};
use self::genopt_h::{
    target_is_arm, target_is_bigendian, target_is_dragonfly, target_is_freebsd, target_is_ilp32,
    target_is_linux, target_is_littleendian, target_is_llp64, target_is_lp64, target_is_macosx,
    target_is_native128, target_is_openbsd, target_is_windows, target_is_x86,
};
use self::genserialise_h::{
    gendeserialise_element, gendeserialise_typeid, genserialise_element, genserialise_typeid,
};
use self::gentrace_h::{gentrace, gentrace_needed};
pub use self::gentype_h::compile_type_t;
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
pub use self::pony_h::{
    C2RustUnnamed_0, PONY_TRACE_IMMUTABLE, PONY_TRACE_MUTABLE, PONY_TRACE_OPAQUE,
};
use self::ponyassert_h::ponyint_assert_fail;
use self::program_h::program_signature;
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach, reach_field_t, reach_mangled_t, reach_method,
    reach_method_name, reach_method_name_t, reach_method_names_t, reach_method_stack_t,
    reach_method_t, reach_methods_next, reach_methods_t, reach_param_t, reach_t, reach_type,
    reach_type_cache_t, reach_type_name, reach_type_t, reach_types_t,
};
pub use self::reify_h::deferred_reification_t;
use self::stringtab_h::{stringtab};
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
pub use self::Core_h::{
    C2RustUnnamed, LLVMAMDGPUCSCallConv, LLVMAMDGPUESCallConv, LLVMAMDGPUGSCallConv,
    LLVMAMDGPUHSCallConv, LLVMAMDGPUKERNELCallConv, LLVMAMDGPULSCallConv, LLVMAMDGPUPSCallConv,
    LLVMAMDGPUVSCallConv, LLVMARMAAPCSCallConv, LLVMARMAAPCSVFPCallConv, LLVMARMAPCSCallConv,
    LLVMAVRBUILTINCallConv, LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddAttributeAtIndex,
    LLVMAddCallSiteAttribute, LLVMAddFunction, LLVMAddGlobal, LLVMAddIncoming, LLVMAnyRegCallConv,
    LLVMAppendingLinkage, LLVMArrayTypeKind, LLVMAttributeFunctionIndex, LLVMAttributeIndex,
    LLVMAttributeReturnIndex, LLVMAvailableExternallyLinkage, LLVMBFloatTypeKind, LLVMBuildAdd,
    LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBitCast, LLVMBuildBr, LLVMBuildCondBr,
    LLVMBuildExtractValue, LLVMBuildFCmp, LLVMBuildFPExt, LLVMBuildFPToSI, LLVMBuildFPToUI,
    LLVMBuildFPTrunc, LLVMBuildICmp, LLVMBuildIsNull, LLVMBuildLShr, LLVMBuildMul, LLVMBuildNot,
    LLVMBuildPhi, LLVMBuildPtrToInt, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSExt,
    LLVMBuildSIToFP, LLVMBuildSelect, LLVMBuildStore, LLVMBuildTrunc, LLVMBuildUIToFP,
    LLVMBuildXor, LLVMBuildZExt, LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv,
    LLVMColdCallConv, LLVMCommonLinkage, LLVMConstArray, LLVMConstInlineAsm, LLVMConstInt,
    LLVMConstNamedStruct, LLVMConstNull, LLVMCreateEnumAttribute, LLVMDLLExportLinkage,
    LLVMDLLImportLinkage, LLVMDoubleTypeKind, LLVMExternalLinkage, LLVMExternalWeakLinkage,
    LLVMFP128TypeKind, LLVMFastCallConv, LLVMFloatTypeKind, LLVMFunctionType, LLVMFunctionTypeKind,
    LLVMGHCCallConv, LLVMGetEnumAttributeKindForName, LLVMGetInlineAsm, LLVMGetInsertBlock,
    LLVMGetParam, LLVMGetTypeKind, LLVMGhostLinkage, LLVMHHVMCCallConv, LLVMHHVMCallConv,
    LLVMHalfTypeKind, LLVMHiPECallConv, LLVMInlineAsmDialect, LLVMInlineAsmDialectATT,
    LLVMInlineAsmDialectIntel, LLVMIntEQ, LLVMIntNE, LLVMIntPredicate, LLVMIntSGE, LLVMIntSGT,
    LLVMIntSLE, LLVMIntSLT, LLVMIntUGE, LLVMIntUGT, LLVMIntULE, LLVMIntULT, LLVMIntegerTypeKind,
    LLVMIntelOCLBICallConv, LLVMInternalLinkage, LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage,
    LLVMLinkOnceODRAutoHideLinkage, LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage,
    LLVMLinkerPrivateWeakLinkage, LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv,
    LLVMMetadataTypeKind, LLVMMoveBasicBlockAfter, LLVMPPC_FP128TypeKind, LLVMPTXDeviceCallConv,
    LLVMPTXKernelCallConv, LLVMPointerType, LLVMPointerTypeKind, LLVMPositionBuilderAtEnd,
    LLVMPreserveAllCallConv, LLVMPreserveMostCallConv, LLVMPrivateLinkage, LLVMRealOEQ,
    LLVMRealOGE, LLVMRealOGT, LLVMRealOLE, LLVMRealOLT, LLVMRealONE, LLVMRealORD,
    LLVMRealPredicate, LLVMRealPredicateFalse, LLVMRealPredicateTrue, LLVMRealUEQ, LLVMRealUGE,
    LLVMRealUGT, LLVMRealULE, LLVMRealULT, LLVMRealUNE, LLVMRealUNO, LLVMSPIRFUNCCallConv,
    LLVMSPIRKERNELCallConv, LLVMScalableVectorTypeKind, LLVMSetFunctionCallConv,
    LLVMSetGlobalConstant, LLVMSetInitializer, LLVMSetLinkage, LLVMSetUnnamedAddr,
    LLVMStructTypeInContext, LLVMStructTypeKind, LLVMSwiftCallConv, LLVMTokenTypeKind,
    LLVMTypeKind, LLVMTypeOf, LLVMVectorTypeKind, LLVMVoidTypeKind, LLVMWeakAnyLinkage,
    LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv, LLVMX8664SysVCallConv,
    LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv, LLVMX86StdcallCallConv,
    LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv, LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind,
    LLVMX86_MMXTypeKind,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{
    LLVMABISizeOfType, LLVMOffsetOfElement, LLVMOpaqueTargetData, LLVMTargetDataRef,
};
pub use self::Types_h::{
    LLVMAttributeRef, LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef,
    LLVMDIBuilderRef, LLVMMetadataRef, LLVMModuleRef, LLVMOpaqueAttributeRef, LLVMOpaqueBasicBlock,
    LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder, LLVMOpaqueMetadata,
    LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef, LLVMValueRef,
};
#[c2rust::src_loc = "36:1"]
pub type generate_box_fn =
    Option<unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> ()>;
#[c2rust::src_loc = "37:1"]
pub type generate_gen_fn =
    Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, *mut reach_method_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1352:16"]
pub struct num_conv_t {
    pub type_name: *const libc::c_char,
    pub fun_name: *const libc::c_char,
    pub type_0: LLVMTypeRef,
    pub size: libc::c_int,
    pub is_signed: bool,
    pub is_float: bool,
}
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn start_function(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut result: LLVMTypeRef,
    mut params: *mut LLVMTypeRef,
    mut count: libc::c_uint,
) {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let ref mut fresh0 = (*c_m).func_type;
    *fresh0 = LLVMFunctionType(result, params, count, 0 as libc::c_int);
    let ref mut fresh1 = (*c_m).func;
    *fresh1 = codegen_addfun(c, (*m).full_name, (*c_m).func_type, 1 as libc::c_int != 0);
    genfun_param_attrs(c, t, m, (*c_m).func);
    codegen_startfun(
        c,
        (*c_m).func,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
}
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn box_function(
    mut c: *mut compile_t,
    mut gen: generate_box_fn,
    mut gen_data: *mut libc::c_void,
) {
    gen.expect("non-null function pointer")(c, gen_data, TK_BOX);
    gen.expect("non-null function pointer")(c, gen_data, TK_REF);
    gen.expect("non-null function pointer")(c, gen_data, TK_VAL);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn generic_function(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
    mut gen: generate_gen_fn,
) {
    let mut n: *mut reach_method_name_t = reach_method_name(t, name);
    if n.is_null() {
        return;
    }
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
    loop {
        m = reach_methods_next(&mut (*n).r_methods, &mut i);
        if m.is_null() {
            break;
        }
        (*m).intrinsic = 1 as libc::c_int != 0;
        gen.expect("non-null function pointer")(c, t, m);
    }
}
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn field_loc(
    mut c: *mut compile_t,
    mut offset: LLVMValueRef,
    mut structure: LLVMTypeRef,
    mut ftype: LLVMTypeRef,
    mut index: libc::c_int,
) -> LLVMValueRef {
    let mut size: LLVMValueRef = LLVMConstInt(
        (*c).intptr,
        LLVMOffsetOfElement((*c).target_data, structure, index as libc::c_uint),
        0 as libc::c_int,
    );
    let mut f_offset: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        offset,
        &mut size,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildBitCast(
        (*c).builder,
        f_offset,
        LLVMPointerType(ftype, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn field_value(
    mut c: *mut compile_t,
    mut object: LLVMValueRef,
    mut index: libc::c_int,
) -> LLVMValueRef {
    let mut field: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        object,
        index as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildLoad_P(
        (*c).builder,
        field,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn pointer_create(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"create\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstNull((*c_t).use_type);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn pointer_alloc(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut compile_type_t,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_alloc\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    let mut noalias_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut noalias_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"noalias\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    noalias_attr =
        LLVMCreateEnumAttribute((*c).context, noalias_attr_id, 0 as libc::c_int as uint64_t);
    let mut deref_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable_or_null\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_attr = LLVMCreateEnumAttribute((*c).context, deref_attr_id, size as uint64_t);
    let mut align_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut align_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"align\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    align_attr = LLVMCreateEnumAttribute(
        (*c).context,
        align_attr_id,
        ((1 as libc::c_int) << 5 as libc::c_int) as uint64_t,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_attr,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    let mut len: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] = codegen_ctx(c);
    args[1 as libc::c_int as usize] = LLVMBuildMul(
        (*c).builder,
        len,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef = gencall_runtime(
        c,
        b"pony_alloc\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    result = LLVMBuildBitCast(
        (*c).builder,
        result,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn pointer_realloc(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut compile_type_t,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_realloc\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    let mut noalias_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut noalias_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"noalias\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    noalias_attr =
        LLVMCreateEnumAttribute((*c).context, noalias_attr_id, 0 as libc::c_int as uint64_t);
    let mut deref_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable_or_null\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_attr = LLVMCreateEnumAttribute((*c).context, deref_attr_id, size as uint64_t);
    let mut align_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut align_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"align\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    align_attr = LLVMCreateEnumAttribute(
        (*c).context,
        align_attr_id,
        ((1 as libc::c_int) << 5 as libc::c_int) as uint64_t,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_attr,
    );
    LLVMAddAttributeAtIndex(
        (*c_m).func,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    let mut args: [LLVMValueRef; 4] = [0 as *mut LLVMOpaqueValue; 4];
    args[0 as libc::c_int as usize] = codegen_ctx(c);
    args[1 as libc::c_int as usize] = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut len: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    args[2 as libc::c_int as usize] = LLVMBuildMul(
        (*c).builder,
        len,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut copy: LLVMValueRef = LLVMGetParam((*c_m).func, 2 as libc::c_int as libc::c_uint);
    args[3 as libc::c_int as usize] = LLVMBuildMul(
        (*c).builder,
        copy,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef = gencall_runtime(
        c,
        b"pony_realloc\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        4 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    result = LLVMBuildBitCast(
        (*c).builder,
        result,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "176:1"]
unsafe extern "C" fn pointer_unsafe(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_unsafe\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    LLVMBuildRet(
        (*c).builder,
        LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint),
    );
    codegen_finishfun(c);
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn pointer_convert(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) {
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut t_result: *mut compile_type_t = (*(*m).result).c_type as *mut compile_type_t;
    start_function(
        c,
        t,
        m,
        (*t_result).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    ptr = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        (*t_result).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, ptr);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn pointer_apply(
    mut c: *mut compile_t,
    mut data: *mut libc::c_void,
    mut cap: token_id,
) {
    let mut t: *mut reach_type_t =
        *(data as *mut *mut reach_type_t).offset(0 as libc::c_int as isize);
    let mut t_elem: *mut reach_type_t =
        *(data as *mut *mut reach_type_t).offset(1 as libc::c_int as isize);
    let mut c_t_elem: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_apply\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t_elem).use_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut index: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut elem_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        LLVMPointerType((*c_t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut loc: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        elem_ptr,
        &mut index,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef =
        LLVMBuildLoad_P((*c).builder, loc, b"\0" as *const u8 as *const libc::c_char);
    let mut tcap: *mut ast_t = ast_childidx((*t).ast, 3 as libc::c_int as size_t);
    let mut tmp_cap: token_id = ast_id(tcap);
    ast_setid(tcap, cap);
    ast_setid(tcap, tmp_cap);
    result = gen_assign_cast(c, (*c_t_elem).use_type, result, (*t_elem).ast_cap);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn pointer_update(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut reach_type_t,
) {
    let mut c_t_elem: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_update\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    params[2 as libc::c_int as usize] = (*c_t_elem).use_type;
    start_function(
        c,
        t,
        m,
        (*c_t_elem).use_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut index: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut elem_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        LLVMPointerType((*c_t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut loc: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        elem_ptr,
        &mut index,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef =
        LLVMBuildLoad_P((*c).builder, loc, b"\0" as *const u8 as *const libc::c_char);
    let mut value: LLVMValueRef = LLVMGetParam((*c_m).func, 2 as libc::c_int as libc::c_uint);
    value = gen_assign_cast(c, (*c_t_elem).mem_type, value, (*t_elem).ast_cap);
    LLVMBuildStore((*c).builder, value, loc);
    result = gen_assign_cast(c, (*c_t_elem).use_type, result, (*t_elem).ast_cap);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "266:1"]
unsafe extern "C" fn pointer_offset(
    mut c: *mut compile_t,
    mut data: *mut libc::c_void,
    mut cap: token_id,
) {
    let mut t: *mut reach_type_t =
        *(data as *mut *mut reach_type_t).offset(0 as libc::c_int as isize);
    let mut t_elem: *mut compile_type_t =
        *(data as *mut *mut compile_type_t).offset(1 as libc::c_int as isize);
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_offset\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut n: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut elem_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        LLVMPointerType((*t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut loc: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        elem_ptr,
        &mut n,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        loc,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn pointer_element_size(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut compile_type_t,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_element_size\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).intptr,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    LLVMBuildRet((*c).builder, l_size);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "304:1"]
unsafe extern "C" fn pointer_insert(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut compile_type_t,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_insert\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut n: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut len: LLVMValueRef = LLVMGetParam((*c_m).func, 2 as libc::c_int as libc::c_uint);
    let mut src: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        LLVMPointerType((*t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut dst: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        src,
        &mut n,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    dst = LLVMBuildBitCast(
        (*c).builder,
        dst,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut elen: LLVMValueRef = LLVMBuildMul(
        (*c).builder,
        len,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gencall_memmove(c, dst, ptr, elen);
    LLVMBuildRet((*c).builder, ptr);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn pointer_delete(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut reach_type_t,
) {
    let mut c_t_elem: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_delete\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c).intptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t_elem).use_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*c_t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut n: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut len: LLVMValueRef = LLVMGetParam((*c_m).func, 2 as libc::c_int as libc::c_uint);
    let mut elem_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        LLVMPointerType((*c_t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        elem_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut src: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        elem_ptr,
        &mut n,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    src = LLVMBuildBitCast(
        (*c).builder,
        src,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut elen: LLVMValueRef = LLVMBuildMul(
        (*c).builder,
        len,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gencall_memmove(c, ptr, src, elen);
    result = gen_assign_cast(c, (*c_t_elem).use_type, result, (*t_elem).ast_cap);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "375:1"]
unsafe extern "C" fn pointer_copy_to(
    mut c: *mut compile_t,
    mut data: *mut libc::c_void,
    mut cap: token_id,
) {
    let mut t: *mut reach_type_t =
        *(data as *mut *mut reach_type_t).offset(0 as libc::c_int as isize);
    let mut t_elem: *mut compile_type_t =
        *(data as *mut *mut compile_type_t).offset(1 as libc::c_int as isize);
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_copy_to\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 3] = [0 as *mut LLVMOpaqueType; 3];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c_t).use_type;
    params[2 as libc::c_int as usize] = (*c).intptr;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    let mut size: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut ptr2: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut n: LLVMValueRef = LLVMGetParam((*c_m).func, 2 as libc::c_int as libc::c_uint);
    let mut elen: LLVMValueRef = LLVMBuildMul(
        (*c).builder,
        n,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gencall_memcpy(c, ptr2, ptr, elen);
    LLVMBuildRet((*c).builder, ptr);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "404:1"]
unsafe extern "C" fn pointer_usize(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"usize\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).intptr,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = LLVMBuildPtrToInt(
        (*c).builder,
        ptr,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn pointer_is_null(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"is_null\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut test: LLVMValueRef =
        LLVMBuildIsNull((*c).builder, ptr, b"\0" as *const u8 as *const libc::c_char);
    LLVMBuildRet((*c).builder, test);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "428:1"]
unsafe extern "C" fn pointer_eq(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"eq\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c_t).use_type;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut ptr2: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut test: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntEQ,
        ptr,
        ptr2,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, test);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn pointer_lt(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"lt\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*c_t).use_type;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut ptr: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut ptr2: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut test: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntULT,
        ptr,
        ptr2,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, test);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "460:1"]
pub unsafe extern "C" fn genprim_pointer_methods(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    let mut t_elem: *mut reach_type_t = reach_type((*c).reach, typearg);
    let mut c_t_elem: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut box_args: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    box_args[0 as libc::c_int as usize] = t as *mut libc::c_void;
    box_args[1 as libc::c_int as usize] = t_elem as *mut libc::c_void;
    let mut c_box_args: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    c_box_args[0 as libc::c_int as usize] = t as *mut libc::c_void;
    c_box_args[1 as libc::c_int as usize] = c_t_elem as *mut libc::c_void;
    pointer_create(c, t);
    pointer_alloc(c, t, c_t_elem);
    pointer_realloc(c, t, c_t_elem);
    pointer_unsafe(c, t);
    generic_function(
        c,
        t,
        stringtab(b"_convert\0" as *const u8 as *const libc::c_char),
        Some(
            pointer_convert
                as unsafe extern "C" fn(
                    *mut compile_t,
                    *mut reach_type_t,
                    *mut reach_method_t,
                ) -> (),
        ),
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            pointer_apply
                as unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> (),
        )),
        box_args.as_mut_ptr() as *mut libc::c_void,
    );
    pointer_update(c, t, t_elem);
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            pointer_offset
                as unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> (),
        )),
        c_box_args.as_mut_ptr() as *mut libc::c_void,
    );
    pointer_element_size(c, t, c_t_elem);
    pointer_insert(c, t, c_t_elem);
    pointer_delete(c, t, t_elem);
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            pointer_copy_to
                as unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> (),
        )),
        c_box_args.as_mut_ptr() as *mut libc::c_void,
    );
    pointer_usize(c, t);
    pointer_is_null(c, t);
    pointer_eq(c, t);
    pointer_lt(c, t);
}
#[c2rust::src_loc = "494:1"]
unsafe extern "C" fn nullable_pointer_create(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut t_elem: *mut compile_type_t,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"create\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*t_elem).use_type;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut param: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        param,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn nullable_pointer_none(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"none\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c_t).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*c_t).use_type));
    codegen_finishfun(c);
}
#[c2rust::src_loc = "518:1"]
unsafe extern "C" fn nullable_pointer_apply(
    mut c: *mut compile_t,
    mut data: *mut libc::c_void,
    mut cap: token_id,
) {
    let mut t: *mut reach_type_t =
        *(data as *mut *mut reach_type_t).offset(0 as libc::c_int as isize);
    let mut t_elem: *mut compile_type_t =
        *(data as *mut *mut compile_type_t).offset(1 as libc::c_int as isize);
    let mut strtab_name: *const libc::c_char =
        stringtab(b"apply\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*t_elem).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut test: LLVMValueRef = LLVMBuildIsNull(
        (*c).builder,
        result,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_false: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut is_true: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    LLVMBuildCondBr((*c).builder, test, is_true, is_false);
    LLVMPositionBuilderAtEnd((*c).builder, is_false);
    result = LLVMBuildBitCast(
        (*c).builder,
        result,
        (*t_elem).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    LLVMPositionBuilderAtEnd((*c).builder, is_true);
    gencall_error(c);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "545:1"]
unsafe extern "C" fn nullable_pointer_is_none(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"is_none\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut receiver: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut test: LLVMValueRef = LLVMBuildIsNull(
        (*c).builder,
        receiver,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, test);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "558:1"]
pub unsafe extern "C" fn genprim_nullable_pointer_methods(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    let mut t_elem: *mut compile_type_t =
        (*reach_type((*c).reach, typearg)).c_type as *mut compile_type_t;
    let mut box_args: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    box_args[0 as libc::c_int as usize] = t as *mut libc::c_void;
    box_args[1 as libc::c_int as usize] = t_elem as *mut libc::c_void;
    nullable_pointer_create(c, t, t_elem);
    nullable_pointer_none(c, t);
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            nullable_pointer_apply
                as unsafe extern "C" fn(*mut compile_t, *mut libc::c_void, token_id) -> (),
        )),
        box_args.as_mut_ptr() as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            nullable_pointer_is_none
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "575:1"]
unsafe extern "C" fn donotoptimise_apply(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) {
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut typearg: *mut ast_t = ast_child((*m).typeargs);
    let mut t_elem: *mut compile_type_t =
        (*reach_type((*c).reach, typearg)).c_type as *mut compile_type_t;
    let mut t_result: *mut compile_type_t = (*(*m).result).c_type as *mut compile_type_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c_t).use_type;
    params[1 as libc::c_int as usize] = (*t_elem).use_type;
    start_function(
        c,
        t,
        m,
        (*t_result).use_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut obj: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut void_fn: LLVMTypeRef = LLVMFunctionType(
        (*c).void_type,
        &mut (*t_elem).use_type,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut asmstr: LLVMValueRef = LLVMConstInlineAsm(
        void_fn,
        b"\0" as *const u8 as *const libc::c_char,
        b"imr,~{memory}\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let mut call: LLVMValueRef = LLVMBuildCall_P(
        (*c).builder,
        asmstr,
        &mut obj,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_ptr: bool = LLVMGetTypeKind((*t_elem).use_type) as libc::c_uint
        == LLVMPointerTypeKind as libc::c_int as libc::c_uint;
    let mut nounwind_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut nounwind_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"nounwind\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    nounwind_attr =
        LLVMCreateEnumAttribute((*c).context, nounwind_attr_id, 0 as libc::c_int as uint64_t);
    let mut readonly_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut readonly_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"readonly\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    readonly_attr =
        LLVMCreateEnumAttribute((*c).context, readonly_attr_id, 0 as libc::c_int as uint64_t);
    let mut inacc_or_arg_mem_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut inacc_or_arg_mem_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"inaccessiblemem_or_argmemonly\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    inacc_or_arg_mem_attr = LLVMCreateEnumAttribute(
        (*c).context,
        inacc_or_arg_mem_attr_id,
        0 as libc::c_int as uint64_t,
    );
    LLVMAddCallSiteAttribute(
        call,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    if is_ptr {
        LLVMAddCallSiteAttribute(call, 1 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    }
    LLVMAddCallSiteAttribute(
        call,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMBuildRet((*c).builder, (*t_result).instance);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "619:1"]
unsafe extern "C" fn donotoptimise_observe(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"observe\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut t_result: *mut compile_type_t = (*(*m).result).c_type as *mut compile_type_t;
    start_function(
        c,
        t,
        m,
        (*t_result).use_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut void_fn: LLVMTypeRef = LLVMFunctionType(
        (*c).void_type,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut asmstr: LLVMValueRef = LLVMConstInlineAsm(
        void_fn,
        b"\0" as *const u8 as *const libc::c_char,
        b"~{memory}\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let mut call: LLVMValueRef = LLVMBuildCall_P(
        (*c).builder,
        asmstr,
        0 as *mut LLVMValueRef,
        0 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut nounwind_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut nounwind_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"nounwind\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    nounwind_attr =
        LLVMCreateEnumAttribute((*c).context, nounwind_attr_id, 0 as libc::c_int as uint64_t);
    let mut inacc_or_arg_mem_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut inacc_or_arg_mem_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"inaccessiblemem_or_argmemonly\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    inacc_or_arg_mem_attr = LLVMCreateEnumAttribute(
        (*c).context,
        inacc_or_arg_mem_attr_id,
        0 as libc::c_int as uint64_t,
    );
    LLVMAddCallSiteAttribute(
        call,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddCallSiteAttribute(
        call,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMBuildRet((*c).builder, (*t_result).instance);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "643:1"]
pub unsafe extern "C" fn genprim_donotoptimise_methods(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    generic_function(
        c,
        t,
        stringtab(b"apply\0" as *const u8 as *const libc::c_char),
        Some(
            donotoptimise_apply
                as unsafe extern "C" fn(
                    *mut compile_t,
                    *mut reach_type_t,
                    *mut reach_method_t,
                ) -> (),
        ),
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            donotoptimise_observe
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "649:1"]
unsafe extern "C" fn trace_array_elements(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut ctx: LLVMValueRef,
    mut object: LLVMValueRef,
    mut pointer: LLVMValueRef,
) {
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    if !gentrace_needed(c, typearg, typearg) {
        return;
    }
    let mut t_elem: *mut reach_type_t = reach_type((*c).reach, typearg);
    let mut c_t_elem: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    pointer = LLVMBuildBitCast(
        (*c).builder,
        pointer,
        LLVMPointerType((*c_t_elem).mem_type, 0 as libc::c_int as libc::c_uint),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut entry_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
    let mut cond_block: LLVMBasicBlockRef =
        codegen_block(c, b"cond\0" as *const u8 as *const libc::c_char);
    let mut body_block: LLVMBasicBlockRef =
        codegen_block(c, b"body\0" as *const u8 as *const libc::c_char);
    let mut post_block: LLVMBasicBlockRef =
        codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
    let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
    LLVMBuildBr((*c).builder, cond_block);
    LLVMPositionBuilderAtEnd((*c).builder, cond_block);
    let mut phi: LLVMValueRef = LLVMBuildPhi(
        (*c).builder,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut zero: LLVMValueRef = LLVMConstInt(
        (*c).intptr,
        0 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMAddIncoming(
        phi,
        &mut zero,
        &mut entry_block,
        1 as libc::c_int as libc::c_uint,
    );
    let mut test: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntULT,
        phi,
        size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, test, body_block, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, body_block);
    let mut elem_ptr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        pointer,
        &mut phi,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut elem: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        elem_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    elem = gen_assign_cast(c, (*c_t_elem).use_type, elem, typearg);
    gentrace(c, ctx, elem, elem, typearg, typearg);
    let mut one: LLVMValueRef = LLVMConstInt(
        (*c).intptr,
        1 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    let mut inc: LLVMValueRef = LLVMBuildAdd(
        (*c).builder,
        phi,
        one,
        b"\0" as *const u8 as *const libc::c_char,
    );
    body_block = LLVMGetInsertBlock((*c).builder);
    LLVMAddIncoming(
        phi,
        &mut inc,
        &mut body_block,
        1 as libc::c_int as libc::c_uint,
    );
    LLVMBuildBr((*c).builder, cond_block);
    LLVMMoveBasicBlockAfter(post_block, LLVMGetInsertBlock((*c).builder));
    LLVMPositionBuilderAtEnd((*c).builder, post_block);
}
#[no_mangle]
#[c2rust::src_loc = "703:1"]
pub unsafe extern "C" fn genprim_array_trace(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    codegen_startfun(
        c,
        (*c_t).trace_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).trace_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).trace_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef = LLVMGetParam((*c_t).trace_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef = LLVMGetParam((*c_t).trace_fn, 1 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut pointer: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = pointer;
    gencall_runtime(
        c,
        b"pony_trace\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    trace_array_elements(c, t, ctx, object, pointer);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "727:1"]
pub unsafe extern "C" fn genprim_array_serialise_trace(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh2 = (*c_t).serialise_trace_fn;
    *fresh2 = codegen_addfun(
        c,
        genname_serialise_trace((*t).name),
        (*c).trace_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).serialise_trace_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).serialise_trace_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).serialise_trace_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_trace_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_trace_fn, 1 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
    let mut trace_block: LLVMBasicBlockRef =
        codegen_block(c, b"trace\0" as *const u8 as *const libc::c_char);
    let mut post_block: LLVMBasicBlockRef =
        codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
    let mut cond: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntNE,
        size,
        LLVMConstInt(
            (*c).intptr,
            0 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, cond, trace_block, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, trace_block);
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    let mut t_elem: *mut compile_type_t =
        (*reach_type((*c).reach, typearg)).c_type as *mut compile_type_t;
    let mut abisize: size_t = LLVMABISizeOfType((*c).target_data, (*t_elem).use_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, abisize as libc::c_ulonglong, 0 as libc::c_int);
    let mut pointer: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = pointer;
    args[2 as libc::c_int as usize] = LLVMBuildMul(
        (*c).builder,
        size,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gencall_runtime(
        c,
        b"pony_serialise_reserve\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    trace_array_elements(c, t, ctx, object, pointer);
    LLVMBuildBr((*c).builder, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, post_block);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "782:1"]
pub unsafe extern "C" fn genprim_array_serialise(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh3 = (*c_t).serialise_fn;
    *fresh3 = codegen_addfun(
        c,
        genname_serialise((*t).name),
        (*c).serialise_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).serialise_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).serialise_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).serialise_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef = LLVMGetParam((*c_t).serialise_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef = LLVMGetParam((*c_t).serialise_fn, 1 as libc::c_int as libc::c_uint);
    let mut addr: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 2 as libc::c_int as libc::c_uint);
    let mut offset: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 3 as libc::c_int as libc::c_uint);
    let mut mut_0: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 4 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).structure_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut offset_addr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        addr,
        &mut offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    genserialise_typeid(c, t, offset_addr);
    let mut body_block: LLVMBasicBlockRef =
        codegen_block(c, b"body\0" as *const u8 as *const libc::c_char);
    let mut post_block: LLVMBasicBlockRef =
        codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
    let mut test: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntNE,
        mut_0,
        LLVMConstInt(
            (*c).i32_0,
            PONY_TRACE_OPAQUE as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, test, body_block, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, body_block);
    let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
    let mut size_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        1 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, size, size_loc);
    let mut alloc_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        2 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, size, alloc_loc);
    let mut ptr: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    let mut args: [LLVMValueRef; 5] = [0 as *mut LLVMOpaqueValue; 5];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = ptr;
    let mut ptr_offset: LLVMValueRef = gencall_runtime(
        c,
        b"pony_serialise_offset\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        3 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, ptr_offset, ptr_loc);
    let mut ptr_offset_addr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        addr,
        &mut ptr_offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    let mut t_elem: *mut reach_type_t = reach_type((*c).reach, typearg);
    let mut c_t_e: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut abisize: size_t = LLVMABISizeOfType((*c).target_data, (*c_t_e).mem_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, abisize as libc::c_ulonglong, 0 as libc::c_int);
    if (*t_elem).underlying as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint
        && !((*c_t_e).primitive).is_null()
    {
        size = LLVMBuildMul(
            (*c).builder,
            size,
            l_size,
            b"\0" as *const u8 as *const libc::c_char,
        );
        gencall_memcpy(c, ptr_offset_addr, ptr, size);
    } else {
        ptr = LLVMBuildBitCast(
            (*c).builder,
            ptr,
            LLVMPointerType((*c_t_e).use_type, 0 as libc::c_int as libc::c_uint),
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut entry_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
        let mut cond_elem_block: LLVMBasicBlockRef =
            codegen_block(c, b"cond\0" as *const u8 as *const libc::c_char);
        let mut body_elem_block: LLVMBasicBlockRef =
            codegen_block(c, b"body\0" as *const u8 as *const libc::c_char);
        let mut post_elem_block: LLVMBasicBlockRef =
            codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
        let mut offset_var: LLVMValueRef = LLVMBuildAlloca(
            (*c).builder,
            (*c).void_ptr,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildStore((*c).builder, ptr_offset_addr, offset_var);
        LLVMBuildBr((*c).builder, cond_elem_block);
        LLVMPositionBuilderAtEnd((*c).builder, cond_elem_block);
        let mut phi: LLVMValueRef = LLVMBuildPhi(
            (*c).builder,
            (*c).intptr,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut zero: LLVMValueRef = LLVMConstInt(
            (*c).intptr,
            0 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        LLVMAddIncoming(
            phi,
            &mut zero,
            &mut entry_block,
            1 as libc::c_int as libc::c_uint,
        );
        let mut test_0: LLVMValueRef = LLVMBuildICmp(
            (*c).builder,
            LLVMIntULT,
            phi,
            size,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildCondBr((*c).builder, test_0, body_elem_block, post_elem_block);
        LLVMPositionBuilderAtEnd((*c).builder, body_elem_block);
        let mut elem_ptr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
            (*c).builder,
            ptr,
            &mut phi,
            1 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        ptr_offset_addr = LLVMBuildLoad_P(
            (*c).builder,
            offset_var,
            b"\0" as *const u8 as *const libc::c_char,
        );
        genserialise_element(
            c,
            t_elem,
            0 as libc::c_int != 0,
            ctx,
            elem_ptr,
            ptr_offset_addr,
        );
        ptr_offset_addr = LLVMBuildInBoundsGEP_P(
            (*c).builder,
            ptr_offset_addr,
            &mut l_size,
            1 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildStore((*c).builder, ptr_offset_addr, offset_var);
        let mut one: LLVMValueRef = LLVMConstInt(
            (*c).intptr,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        let mut inc: LLVMValueRef = LLVMBuildAdd(
            (*c).builder,
            phi,
            one,
            b"\0" as *const u8 as *const libc::c_char,
        );
        body_block = LLVMGetInsertBlock((*c).builder);
        LLVMAddIncoming(
            phi,
            &mut inc,
            &mut body_block,
            1 as libc::c_int as libc::c_uint,
        );
        LLVMBuildBr((*c).builder, cond_elem_block);
        LLVMMoveBasicBlockAfter(post_elem_block, LLVMGetInsertBlock((*c).builder));
        LLVMPositionBuilderAtEnd((*c).builder, post_elem_block);
    }
    LLVMBuildBr((*c).builder, post_block);
    LLVMMoveBasicBlockAfter(post_block, LLVMGetInsertBlock((*c).builder));
    LLVMPositionBuilderAtEnd((*c).builder, post_block);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "910:1"]
pub unsafe extern "C" fn genprim_array_deserialise(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh4 = (*c_t).deserialise_fn;
    *fresh4 = codegen_addfun(
        c,
        genname_deserialise((*t).name),
        (*c).trace_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).deserialise_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).deserialise_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).deserialise_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef =
        LLVMGetParam((*c_t).deserialise_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef =
        LLVMGetParam((*c_t).deserialise_fn, 1 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).structure_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gendeserialise_typeid(c, c_t, object);
    let mut alloc: LLVMValueRef = field_value(c, object, 2 as libc::c_int);
    let mut ptr_offset: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    ptr_offset = LLVMBuildPtrToInt(
        (*c).builder,
        ptr_offset,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut typeargs: *mut ast_t = ast_childidx((*t).ast, 2 as libc::c_int as size_t);
    let mut typearg: *mut ast_t = ast_child(typeargs);
    let mut t_elem: *mut reach_type_t = reach_type((*c).reach, typearg);
    let mut c_t_e: *mut compile_type_t = (*t_elem).c_type as *mut compile_type_t;
    let mut abisize: size_t = LLVMABISizeOfType((*c).target_data, (*c_t_e).use_type) as size_t;
    let mut l_size: LLVMValueRef =
        LLVMConstInt((*c).intptr, abisize as libc::c_ulonglong, 0 as libc::c_int);
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = ptr_offset;
    args[2 as libc::c_int as usize] = LLVMBuildMul(
        (*c).builder,
        alloc,
        l_size,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr: LLVMValueRef = gencall_runtime(
        c,
        b"pony_deserialise_block\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr_loc: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        object,
        3 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildStore((*c).builder, ptr, ptr_loc);
    if !((*t_elem).underlying as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint
        && !((*c_t_e).primitive).is_null())
    {
        let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
        ptr = LLVMBuildBitCast(
            (*c).builder,
            ptr,
            LLVMPointerType((*c_t_e).use_type, 0 as libc::c_int as libc::c_uint),
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut entry_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
        let mut cond_block: LLVMBasicBlockRef =
            codegen_block(c, b"cond\0" as *const u8 as *const libc::c_char);
        let mut body_block: LLVMBasicBlockRef =
            codegen_block(c, b"body\0" as *const u8 as *const libc::c_char);
        let mut post_block: LLVMBasicBlockRef =
            codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
        LLVMBuildBr((*c).builder, cond_block);
        LLVMPositionBuilderAtEnd((*c).builder, cond_block);
        let mut phi: LLVMValueRef = LLVMBuildPhi(
            (*c).builder,
            (*c).intptr,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut zero: LLVMValueRef = LLVMConstInt(
            (*c).intptr,
            0 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        LLVMAddIncoming(
            phi,
            &mut zero,
            &mut entry_block,
            1 as libc::c_int as libc::c_uint,
        );
        let mut test: LLVMValueRef = LLVMBuildICmp(
            (*c).builder,
            LLVMIntULT,
            phi,
            size,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildCondBr((*c).builder, test, body_block, post_block);
        LLVMPositionBuilderAtEnd((*c).builder, body_block);
        let mut elem_ptr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
            (*c).builder,
            ptr,
            &mut phi,
            1 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        gendeserialise_element(c, t_elem, 0 as libc::c_int != 0, ctx, elem_ptr);
        let mut one: LLVMValueRef = LLVMConstInt(
            (*c).intptr,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        );
        let mut inc: LLVMValueRef = LLVMBuildAdd(
            (*c).builder,
            phi,
            one,
            b"\0" as *const u8 as *const libc::c_char,
        );
        body_block = LLVMGetInsertBlock((*c).builder);
        LLVMAddIncoming(
            phi,
            &mut inc,
            &mut body_block,
            1 as libc::c_int as libc::c_uint,
        );
        LLVMBuildBr((*c).builder, cond_block);
        LLVMMoveBasicBlockAfter(post_block, LLVMGetInsertBlock((*c).builder));
        LLVMPositionBuilderAtEnd((*c).builder, post_block);
    }
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn genprim_string_serialise_trace(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh5 = (*c_t).serialise_trace_fn;
    *fresh5 = codegen_addfun(
        c,
        genname_serialise_trace((*t).name),
        (*c).serialise_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).serialise_trace_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).serialise_trace_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).serialise_trace_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_trace_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_trace_fn, 1 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
    let mut alloc: LLVMValueRef = LLVMBuildAdd(
        (*c).builder,
        size,
        LLVMConstInt(
            (*c).intptr,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = ptr;
    args[2 as libc::c_int as usize] = alloc;
    gencall_runtime(
        c,
        b"pony_serialise_reserve\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "1028:1"]
pub unsafe extern "C" fn genprim_string_serialise(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh6 = (*c_t).serialise_fn;
    *fresh6 = codegen_addfun(
        c,
        genname_serialise((*t).name),
        (*c).serialise_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).serialise_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).serialise_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).serialise_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef = LLVMGetParam((*c_t).serialise_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef = LLVMGetParam((*c_t).serialise_fn, 1 as libc::c_int as libc::c_uint);
    let mut addr: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 2 as libc::c_int as libc::c_uint);
    let mut offset: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 3 as libc::c_int as libc::c_uint);
    let mut mut_0: LLVMValueRef =
        LLVMGetParam((*c_t).serialise_fn, 4 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).structure_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut offset_addr: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        addr,
        &mut offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    genserialise_typeid(c, t, offset_addr);
    let mut body_block: LLVMBasicBlockRef =
        codegen_block(c, b"body\0" as *const u8 as *const libc::c_char);
    let mut post_block: LLVMBasicBlockRef =
        codegen_block(c, b"post\0" as *const u8 as *const libc::c_char);
    let mut test: LLVMValueRef = LLVMBuildICmp(
        (*c).builder,
        LLVMIntNE,
        mut_0,
        LLVMConstInt(
            (*c).i32_0,
            PONY_TRACE_OPAQUE as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, test, body_block, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, body_block);
    let mut size: LLVMValueRef = field_value(c, object, 1 as libc::c_int);
    let mut size_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        1 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, size, size_loc);
    let mut alloc: LLVMValueRef = LLVMBuildAdd(
        (*c).builder,
        size,
        LLVMConstInt(
            (*c).intptr,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut alloc_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        2 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, alloc, alloc_loc);
    let mut ptr: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    let mut args: [LLVMValueRef; 5] = [0 as *mut LLVMOpaqueValue; 5];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = ptr;
    let mut ptr_offset: LLVMValueRef = gencall_runtime(
        c,
        b"pony_serialise_offset\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr_loc: LLVMValueRef = field_loc(
        c,
        offset_addr,
        (*c_t).structure,
        (*c).intptr,
        3 as libc::c_int,
    );
    LLVMBuildStore((*c).builder, ptr_offset, ptr_loc);
    let mut dst: LLVMValueRef = LLVMBuildInBoundsGEP_P(
        (*c).builder,
        addr,
        &mut ptr_offset,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut src: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        field_value(c, object, 3 as libc::c_int),
        (*c).void_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gencall_memcpy(c, dst, src, alloc);
    LLVMBuildBr((*c).builder, post_block);
    LLVMPositionBuilderAtEnd((*c).builder, post_block);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "1098:1"]
pub unsafe extern "C" fn genprim_string_deserialise(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh7 = (*c_t).deserialise_fn;
    *fresh7 = codegen_addfun(
        c,
        genname_deserialise((*t).name),
        (*c).trace_type,
        1 as libc::c_int != 0,
    );
    codegen_startfun(
        c,
        (*c_t).deserialise_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(
        (*c_t).deserialise_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).deserialise_fn, LLVMExternalLinkage);
    let mut ctx: LLVMValueRef =
        LLVMGetParam((*c_t).deserialise_fn, 0 as libc::c_int as libc::c_uint);
    let mut arg: LLVMValueRef =
        LLVMGetParam((*c_t).deserialise_fn, 1 as libc::c_int as libc::c_uint);
    let mut object: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        (*c_t).structure_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    gendeserialise_typeid(c, c_t, object);
    let mut alloc: LLVMValueRef = field_value(c, object, 2 as libc::c_int);
    let mut ptr_offset: LLVMValueRef = field_value(c, object, 3 as libc::c_int);
    ptr_offset = LLVMBuildPtrToInt(
        (*c).builder,
        ptr_offset,
        (*c).intptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = ctx;
    args[1 as libc::c_int as usize] = ptr_offset;
    args[2 as libc::c_int as usize] = alloc;
    let mut ptr_addr: LLVMValueRef = gencall_runtime(
        c,
        b"pony_deserialise_block\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        object,
        3 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildStore((*c).builder, ptr_addr, ptr);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1135:1"]
unsafe extern "C" fn platform_freebsd(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"freebsd\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_freebsd((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1146:1"]
unsafe extern "C" fn platform_dragonfly(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"dragonfly\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_dragonfly((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1157:1"]
unsafe extern "C" fn platform_openbsd(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"openbsd\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_openbsd((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1168:1"]
unsafe extern "C" fn platform_linux(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"linux\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_linux((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1179:1"]
unsafe extern "C" fn platform_osx(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"osx\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_macosx((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1190:1"]
unsafe extern "C" fn platform_windows(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"windows\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_windows((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1201:1"]
unsafe extern "C" fn platform_x86(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"x86\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_x86((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1212:1"]
unsafe extern "C" fn platform_arm(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"arm\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_arm((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1223:1"]
unsafe extern "C" fn platform_lp64(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"lp64\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_lp64((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1234:1"]
unsafe extern "C" fn platform_llp64(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"llp64\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_llp64((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1245:1"]
unsafe extern "C" fn platform_ilp32(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"ilp32\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_ilp32((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1256:1"]
unsafe extern "C" fn platform_bigendian(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"bigendian\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_bigendian((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1267:1"]
unsafe extern "C" fn platform_littleendian(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"littleendian\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_littleendian((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1278:1"]
unsafe extern "C" fn platform_native128(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"native128\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        target_is_native128((*(*c).opt).triple) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1289:1"]
unsafe extern "C" fn platform_debug(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"debug\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        !(*(*c).opt).release as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1299:1"]
unsafe extern "C" fn platform_runtimestats(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"runtimestats\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut runtimestats_enabled: bool = 0 as libc::c_int != 0;
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        runtimestats_enabled as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1315:1"]
unsafe extern "C" fn platform_runtimestatsmessages(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut cap: token_id,
) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"runtimestatsmessages\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i1,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
    );
    let mut runtimestatsmessages_enabled: bool = 0 as libc::c_int != 0;
    let mut result: LLVMValueRef = LLVMConstInt(
        (*c).i1,
        runtimestatsmessages_enabled as libc::c_ulonglong,
        0 as libc::c_int,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "1331:1"]
pub unsafe extern "C" fn genprim_platform_methods(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_freebsd
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_dragonfly
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_openbsd
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_linux
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_osx as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_windows
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_x86 as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_arm as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_lp64
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_llp64
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_ilp32
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_bigendian
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_littleendian
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_native128
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_debug
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_runtimestats
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
    box_function(
        c,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
            generate_box_fn,
        >(Some(
            platform_runtimestatsmessages
                as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
        )),
        t as *mut libc::c_void,
    );
}
#[c2rust::src_loc = "1362:1"]
unsafe extern "C" fn number_value(
    mut c: *mut compile_t,
    mut type_0: *mut num_conv_t,
    mut cap: token_id,
) {
    let mut t: *mut reach_type_t = reach_type_name((*c).reach, (*type_0).type_name);
    if t.is_null() {
        return;
    }
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_value\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*type_0).type_0,
        &mut (*type_0).type_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut arg: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    LLVMBuildRet((*c).builder, arg);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1378:1"]
unsafe extern "C" fn handle_nan(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut int_type: LLVMTypeRef,
    mut exp: uint64_t,
    mut mantissa: uint64_t,
) -> LLVMBasicBlockRef {
    let mut nan: LLVMBasicBlockRef = codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut non_nan: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut exp_mask: LLVMValueRef = LLVMConstInt(int_type, exp, 0 as libc::c_int);
    let mut mant_mask: LLVMValueRef = LLVMConstInt(int_type, mantissa, 0 as libc::c_int);
    let mut bits: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        arg,
        int_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut exp_res: LLVMValueRef = LLVMBuildAnd(
        (*c).builder,
        bits,
        exp_mask,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut mant_res: LLVMValueRef = LLVMBuildAnd(
        (*c).builder,
        bits,
        mant_mask,
        b"\0" as *const u8 as *const libc::c_char,
    );
    exp_res = LLVMBuildICmp(
        (*c).builder,
        LLVMIntEQ,
        exp_res,
        exp_mask,
        b"\0" as *const u8 as *const libc::c_char,
    );
    mant_res = LLVMBuildICmp(
        (*c).builder,
        LLVMIntNE,
        mant_res,
        LLVMConstNull(int_type),
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_nan: LLVMValueRef = LLVMBuildAnd(
        (*c).builder,
        exp_res,
        mant_res,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_nan, nan, non_nan);
    LLVMPositionBuilderAtEnd((*c).builder, nan);
    return non_nan;
}
#[c2rust::src_loc = "1403:1"]
unsafe extern "C" fn handle_overflow_saturate(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut from: LLVMTypeRef,
    mut to: LLVMTypeRef,
    mut to_max: LLVMValueRef,
    mut to_min: LLVMValueRef,
    mut sign: bool,
) -> LLVMValueRef {
    let mut overflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut test_underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut normal: LLVMBasicBlockRef = codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut to_fmax: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if sign {
        to_fmax = LLVMBuildSIToFP(
            (*c).builder,
            to_max,
            from,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        to_fmax = LLVMBuildUIToFP(
            (*c).builder,
            to_max,
            from,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut is_overflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOGT,
        arg,
        to_fmax,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_overflow, overflow, test_underflow);
    LLVMPositionBuilderAtEnd((*c).builder, overflow);
    LLVMBuildRet((*c).builder, to_max);
    LLVMPositionBuilderAtEnd((*c).builder, test_underflow);
    let mut to_fmin: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if sign {
        to_fmin = LLVMBuildSIToFP(
            (*c).builder,
            to_min,
            from,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        to_fmin = LLVMBuildUIToFP(
            (*c).builder,
            to_min,
            from,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut is_underflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOLT,
        arg,
        to_fmin,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_underflow, underflow, normal);
    LLVMPositionBuilderAtEnd((*c).builder, underflow);
    LLVMBuildRet((*c).builder, to_min);
    LLVMPositionBuilderAtEnd((*c).builder, normal);
    if sign {
        return LLVMBuildFPToSI(
            (*c).builder,
            arg,
            to,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    return LLVMBuildFPToUI(
        (*c).builder,
        arg,
        to,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "1445:1"]
unsafe extern "C" fn f32_to_si_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut to: *mut num_conv_t,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i32_0,
        0x7f800000 as libc::c_int as uint64_t,
        0x7fffff as libc::c_int as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*to).type_0));
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut to_max: LLVMValueRef = LLVMConstNull((*to).type_0);
    let mut to_min: LLVMValueRef = LLVMBuildNot(
        (*c).builder,
        to_max,
        b"\0" as *const u8 as *const libc::c_char,
    );
    to_max = LLVMBuildLShr(
        (*c).builder,
        to_min,
        LLVMConstInt(
            (*to).type_0,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    to_min = LLVMBuildXor(
        (*c).builder,
        to_max,
        to_min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return handle_overflow_saturate(
        c,
        arg,
        (*c).f32_0,
        (*to).type_0,
        to_max,
        to_min,
        1 as libc::c_int != 0,
    );
}
#[c2rust::src_loc = "1461:1"]
unsafe extern "C" fn f64_to_si_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut to: *mut num_conv_t,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i64_0,
        0x7ff0000000000000 as libc::c_long as uint64_t,
        0xfffffffffffff as libc::c_long as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*to).type_0));
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut to_max: LLVMValueRef = LLVMConstNull((*to).type_0);
    let mut to_min: LLVMValueRef = LLVMBuildNot(
        (*c).builder,
        to_max,
        b"\0" as *const u8 as *const libc::c_char,
    );
    to_max = LLVMBuildLShr(
        (*c).builder,
        to_min,
        LLVMConstInt(
            (*to).type_0,
            1 as libc::c_int as libc::c_ulonglong,
            0 as libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
    );
    to_min = LLVMBuildXor(
        (*c).builder,
        to_max,
        to_min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return handle_overflow_saturate(
        c,
        arg,
        (*c).f64_0,
        (*to).type_0,
        to_max,
        to_min,
        1 as libc::c_int != 0,
    );
}
#[c2rust::src_loc = "1477:1"]
unsafe extern "C" fn f32_to_ui_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut to: *mut num_conv_t,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i32_0,
        0x7f800000 as libc::c_int as uint64_t,
        0x7fffff as libc::c_int as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*to).type_0));
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut to_min: LLVMValueRef = LLVMConstNull((*to).type_0);
    let mut to_max: LLVMValueRef = LLVMBuildNot(
        (*c).builder,
        to_min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return handle_overflow_saturate(
        c,
        arg,
        (*c).f32_0,
        (*to).type_0,
        to_max,
        to_min,
        0 as libc::c_int != 0,
    );
}
#[c2rust::src_loc = "1490:1"]
unsafe extern "C" fn f32_to_u128_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i32_0,
        0x7f800000 as libc::c_int as uint64_t,
        0x7fffff as libc::c_int as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*c).i128_0));
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut overflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut test_underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut normal: LLVMBasicBlockRef = codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut min: LLVMValueRef = LLVMConstNull((*c).f32_0);
    let mut max: LLVMValueRef = LLVMConstInf((*c).f32_0, 0 as libc::c_int != 0);
    let mut is_overflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOGE,
        arg,
        max,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_overflow, overflow, test_underflow);
    LLVMPositionBuilderAtEnd((*c).builder, overflow);
    LLVMBuildRet(
        (*c).builder,
        LLVMBuildNot(
            (*c).builder,
            LLVMConstNull((*c).i128_0),
            b"\0" as *const u8 as *const libc::c_char,
        ),
    );
    LLVMPositionBuilderAtEnd((*c).builder, test_underflow);
    let mut is_underflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOLT,
        arg,
        min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_underflow, underflow, normal);
    LLVMPositionBuilderAtEnd((*c).builder, underflow);
    LLVMBuildRet((*c).builder, LLVMConstNull((*c).i128_0));
    LLVMPositionBuilderAtEnd((*c).builder, normal);
    return LLVMBuildFPToUI(
        (*c).builder,
        arg,
        (*c).i128_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "1525:1"]
unsafe extern "C" fn f64_to_ui_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
    mut to: *mut num_conv_t,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i64_0,
        0x7ff0000000000000 as libc::c_long as uint64_t,
        0xfffffffffffff as libc::c_long as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNull((*to).type_0));
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut to_min: LLVMValueRef = LLVMConstNull((*to).type_0);
    let mut to_max: LLVMValueRef = LLVMBuildNot(
        (*c).builder,
        to_min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return handle_overflow_saturate(
        c,
        arg,
        (*c).f64_0,
        (*to).type_0,
        to_max,
        to_min,
        0 as libc::c_int != 0,
    );
}
#[c2rust::src_loc = "1538:1"]
unsafe extern "C" fn f64_to_f32_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
) -> LLVMValueRef {
    let mut test_overflow: LLVMBasicBlockRef = handle_nan(
        c,
        arg,
        (*c).i64_0,
        0x7ff0000000000000 as libc::c_long as uint64_t,
        0xfffffffffffff as libc::c_long as uint64_t,
    );
    LLVMBuildRet((*c).builder, LLVMConstNaN((*c).f32_0));
    let mut overflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut test_underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut underflow: LLVMBasicBlockRef =
        codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    let mut normal: LLVMBasicBlockRef = codegen_block(c, b"\0" as *const u8 as *const libc::c_char);
    LLVMPositionBuilderAtEnd((*c).builder, test_overflow);
    let mut f32_max: LLVMValueRef = LLVMConstInt(
        (*c).i32_0,
        0x7f7fffff as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    f32_max = LLVMBuildBitCast(
        (*c).builder,
        f32_max,
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    f32_max = LLVMBuildFPExt(
        (*c).builder,
        f32_max,
        (*c).f64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_overflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOGT,
        arg,
        f32_max,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_overflow, overflow, test_underflow);
    LLVMPositionBuilderAtEnd((*c).builder, overflow);
    LLVMBuildRet(
        (*c).builder,
        LLVMConstInf((*c).f32_0, 0 as libc::c_int != 0),
    );
    LLVMPositionBuilderAtEnd((*c).builder, test_underflow);
    let mut f32_min: LLVMValueRef = LLVMConstInt(
        (*c).i32_0,
        0xff7fffff as libc::c_uint as libc::c_ulonglong,
        0 as libc::c_int,
    );
    f32_min = LLVMBuildBitCast(
        (*c).builder,
        f32_min,
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    f32_min = LLVMBuildFPExt(
        (*c).builder,
        f32_min,
        (*c).f64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_underflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOLT,
        arg,
        f32_min,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCondBr((*c).builder, is_underflow, underflow, normal);
    LLVMPositionBuilderAtEnd((*c).builder, underflow);
    LLVMBuildRet(
        (*c).builder,
        LLVMConstInf((*c).f32_0, 1 as libc::c_int != 0),
    );
    LLVMPositionBuilderAtEnd((*c).builder, normal);
    return LLVMBuildFPTrunc(
        (*c).builder,
        arg,
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "1575:1"]
unsafe extern "C" fn u128_to_f32_saturation(
    mut c: *mut compile_t,
    mut arg: LLVMValueRef,
) -> LLVMValueRef {
    let mut val_f64: LLVMValueRef = LLVMBuildUIToFP(
        (*c).builder,
        arg,
        (*c).f64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut f32_max: LLVMValueRef = LLVMConstInt(
        (*c).i32_0,
        0x7f7fffff as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    f32_max = LLVMBuildBitCast(
        (*c).builder,
        f32_max,
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    f32_max = LLVMBuildFPExt(
        (*c).builder,
        f32_max,
        (*c).f64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut is_overflow: LLVMValueRef = LLVMBuildFCmp(
        (*c).builder,
        LLVMRealOGT,
        val_f64,
        f32_max,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut result: LLVMValueRef = LLVMBuildUIToFP(
        (*c).builder,
        arg,
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildSelect(
        (*c).builder,
        is_overflow,
        LLVMConstInf((*c).f32_0, 0 as libc::c_int != 0),
        result,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[c2rust::src_loc = "1588:1"]
unsafe extern "C" fn number_conversion(
    mut c: *mut compile_t,
    mut data: *mut *mut libc::c_void,
    mut cap: token_id,
) {
    let mut from: *mut num_conv_t = *data.offset(0 as libc::c_int as isize) as *mut num_conv_t;
    let mut to: *mut num_conv_t = *data.offset(1 as libc::c_int as isize) as *mut num_conv_t;
    let mut native128: bool = !(*data.offset(2 as libc::c_int as isize)).is_null();
    if !native128
        && ((*from).is_float as libc::c_int != 0 && (*to).size > 64 as libc::c_int
            || (*to).is_float as libc::c_int != 0 && (*from).size > 64 as libc::c_int)
    {
        return;
    }
    let mut t: *mut reach_type_t = reach_type_name((*c).reach, (*from).type_name);
    if t.is_null() {
        return;
    }
    let mut strtab_name: *const libc::c_char = stringtab((*to).fun_name);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*to).type_0,
        &mut (*from).type_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut arg: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if (*from).is_float {
        if (*to).is_float {
            if (*from).size < (*to).size {
                result = LLVMBuildFPExt(
                    (*c).builder,
                    arg,
                    (*to).type_0,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            } else if (*from).size > (*to).size {
                result = f64_to_f32_saturation(c, arg);
            } else {
                result = arg;
            }
        } else if (*to).is_signed {
            if (*from).size < 64 as libc::c_int {
                result = f32_to_si_saturation(c, arg, to);
            } else {
                result = f64_to_si_saturation(c, arg, to);
            }
        } else if (*from).size < 64 as libc::c_int {
            if (*to).size > 64 as libc::c_int {
                result = f32_to_u128_saturation(c, arg);
            } else {
                result = f32_to_ui_saturation(c, arg, to);
            }
        } else {
            result = f64_to_ui_saturation(c, arg, to);
        }
    } else if (*to).is_float {
        if (*from).is_signed {
            result = LLVMBuildSIToFP(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*from).size > 64 as libc::c_int && (*to).size < 64 as libc::c_int {
            result = u128_to_f32_saturation(c, arg);
        } else {
            result = LLVMBuildUIToFP(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*from).size > (*to).size {
        result = LLVMBuildTrunc(
            (*c).builder,
            arg,
            (*to).type_0,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else if (*from).size < (*to).size {
        if (*from).is_signed {
            result = LLVMBuildSExt(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = LLVMBuildZExt(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        result = arg;
    }
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1660:1"]
unsafe extern "C" fn unsafe_number_conversion(
    mut c: *mut compile_t,
    mut data: *mut *mut libc::c_void,
    mut cap: token_id,
) {
    let mut from: *mut num_conv_t = *data.offset(0 as libc::c_int as isize) as *mut num_conv_t;
    let mut to: *mut num_conv_t = *data.offset(1 as libc::c_int as isize) as *mut num_conv_t;
    let mut native128: bool = !(*data.offset(2 as libc::c_int as isize)).is_null();
    if !native128
        && ((*from).is_float as libc::c_int != 0 && (*to).size > 64 as libc::c_int
            || (*to).is_float as libc::c_int != 0 && (*from).size > 64 as libc::c_int)
    {
        return;
    }
    let mut t: *mut reach_type_t = reach_type_name((*c).reach, (*from).type_name);
    if t.is_null() {
        return;
    }
    let mut name: *const libc::c_char = genname_unsafe((*to).fun_name);
    let mut strtab_name: *const libc::c_char = stringtab(name);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*to).type_0,
        &mut (*from).type_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut arg: LLVMValueRef = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if (*from).is_float {
        if (*to).is_float {
            if (*from).size < (*to).size {
                result = LLVMBuildFPExt(
                    (*c).builder,
                    arg,
                    (*to).type_0,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            } else if (*from).size > (*to).size {
                result = LLVMBuildFPTrunc(
                    (*c).builder,
                    arg,
                    (*to).type_0,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            } else {
                result = arg;
            }
        } else if (*to).is_signed {
            result = LLVMBuildFPToSI(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = LLVMBuildFPToUI(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*to).is_float {
        if (*from).is_signed {
            result = LLVMBuildSIToFP(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = LLVMBuildUIToFP(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*from).size > (*to).size {
        result = LLVMBuildTrunc(
            (*c).builder,
            arg,
            (*to).type_0,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else if (*from).size < (*to).size {
        if (*from).is_signed {
            result = LLVMBuildSExt(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = LLVMBuildZExt(
                (*c).builder,
                arg,
                (*to).type_0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        result = arg;
    }
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1722:1"]
unsafe extern "C" fn number_conversions(mut c: *mut compile_t) {
    let mut ilp32_conv: [num_conv_t; 17] = [
        {
            let mut init = num_conv_t {
                type_name: b"I8\0" as *const u8 as *const libc::c_char,
                fun_name: b"i8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I16\0" as *const u8 as *const libc::c_char,
                fun_name: b"i16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I32\0" as *const u8 as *const libc::c_char,
                fun_name: b"i32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I64\0" as *const u8 as *const libc::c_char,
                fun_name: b"i64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U8\0" as *const u8 as *const libc::c_char,
                fun_name: b"u8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U16\0" as *const u8 as *const libc::c_char,
                fun_name: b"u16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U32\0" as *const u8 as *const libc::c_char,
                fun_name: b"u32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U64\0" as *const u8 as *const libc::c_char,
                fun_name: b"u64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I128\0" as *const u8 as *const libc::c_char,
                fun_name: b"i128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U128\0" as *const u8 as *const libc::c_char,
                fun_name: b"u128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ILong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ilong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ULong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ulong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ISize\0" as *const u8 as *const libc::c_char,
                fun_name: b"isize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"USize\0" as *const u8 as *const libc::c_char,
                fun_name: b"usize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F32\0" as *const u8 as *const libc::c_char,
                fun_name: b"f32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F64\0" as *const u8 as *const libc::c_char,
                fun_name: b"f64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: 0 as *const libc::c_char,
                fun_name: 0 as *const libc::c_char,
                type_0: 0 as LLVMTypeRef,
                size: 0 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
    ];
    let mut lp64_conv: [num_conv_t; 17] = [
        {
            let mut init = num_conv_t {
                type_name: b"I8\0" as *const u8 as *const libc::c_char,
                fun_name: b"i8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I16\0" as *const u8 as *const libc::c_char,
                fun_name: b"i16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I32\0" as *const u8 as *const libc::c_char,
                fun_name: b"i32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I64\0" as *const u8 as *const libc::c_char,
                fun_name: b"i64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U8\0" as *const u8 as *const libc::c_char,
                fun_name: b"u8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U16\0" as *const u8 as *const libc::c_char,
                fun_name: b"u16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U32\0" as *const u8 as *const libc::c_char,
                fun_name: b"u32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U64\0" as *const u8 as *const libc::c_char,
                fun_name: b"u64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I128\0" as *const u8 as *const libc::c_char,
                fun_name: b"i128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U128\0" as *const u8 as *const libc::c_char,
                fun_name: b"u128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ILong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ilong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ULong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ulong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ISize\0" as *const u8 as *const libc::c_char,
                fun_name: b"isize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"USize\0" as *const u8 as *const libc::c_char,
                fun_name: b"usize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F32\0" as *const u8 as *const libc::c_char,
                fun_name: b"f32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F64\0" as *const u8 as *const libc::c_char,
                fun_name: b"f64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: 0 as *const libc::c_char,
                fun_name: 0 as *const libc::c_char,
                type_0: 0 as LLVMTypeRef,
                size: 0 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
    ];
    let mut llp64_conv: [num_conv_t; 17] = [
        {
            let mut init = num_conv_t {
                type_name: b"I8\0" as *const u8 as *const libc::c_char,
                fun_name: b"i8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I16\0" as *const u8 as *const libc::c_char,
                fun_name: b"i16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I32\0" as *const u8 as *const libc::c_char,
                fun_name: b"i32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I64\0" as *const u8 as *const libc::c_char,
                fun_name: b"i64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U8\0" as *const u8 as *const libc::c_char,
                fun_name: b"u8\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i8_0,
                size: 8 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U16\0" as *const u8 as *const libc::c_char,
                fun_name: b"u16\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i16_0,
                size: 16 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U32\0" as *const u8 as *const libc::c_char,
                fun_name: b"u32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U64\0" as *const u8 as *const libc::c_char,
                fun_name: b"u64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"I128\0" as *const u8 as *const libc::c_char,
                fun_name: b"i128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"U128\0" as *const u8 as *const libc::c_char,
                fun_name: b"u128\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i128_0,
                size: 128 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ILong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ilong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ULong\0" as *const u8 as *const libc::c_char,
                fun_name: b"ulong\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"ISize\0" as *const u8 as *const libc::c_char,
                fun_name: b"isize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 1 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"USize\0" as *const u8 as *const libc::c_char,
                fun_name: b"usize\0" as *const u8 as *const libc::c_char,
                type_0: (*c).i64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F32\0" as *const u8 as *const libc::c_char,
                fun_name: b"f32\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f32_0,
                size: 32 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: b"F64\0" as *const u8 as *const libc::c_char,
                fun_name: b"f64\0" as *const u8 as *const libc::c_char,
                type_0: (*c).f64_0,
                size: 64 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = num_conv_t {
                type_name: 0 as *const libc::c_char,
                fun_name: 0 as *const libc::c_char,
                type_0: 0 as LLVMTypeRef,
                size: 0 as libc::c_int,
                is_signed: 0 as libc::c_int != 0,
                is_float: 0 as libc::c_int != 0,
            };
            init
        },
    ];
    let mut conv: *mut num_conv_t = 0 as *mut num_conv_t;
    if target_is_ilp32((*(*c).opt).triple) {
        conv = ilp32_conv.as_mut_ptr();
    } else if target_is_lp64((*(*c).opt).triple) {
        conv = lp64_conv.as_mut_ptr();
    } else if target_is_llp64((*(*c).opt).triple) {
        conv = llp64_conv.as_mut_ptr();
    }
    if !conv.is_null() {
    } else {
        ponyint_assert_fail(
            b"conv != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genprim.c\0"
                as *const u8 as *const libc::c_char,
            1808 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"number_conversions\0"))
                .as_ptr(),
        );
    };
    let mut data: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    data[2 as libc::c_int as usize] = if target_is_native128((*(*c).opt).triple) {1} else {0} as *mut libc::c_void;
    let mut from: *mut num_conv_t = conv;
    while !((*from).type_name).is_null() {
        box_function(
            c,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut compile_t, *mut num_conv_t, token_id) -> ()>,
                generate_box_fn,
            >(Some(
                number_value
                    as unsafe extern "C" fn(*mut compile_t, *mut num_conv_t, token_id) -> (),
            )),
            from as *mut libc::c_void,
        );
        data[0 as libc::c_int as usize] = from as *mut libc::c_void;
        let mut to: *mut num_conv_t = conv;
        while !((*to).type_name).is_null() {
            data[1 as libc::c_int as usize] = to as *mut libc::c_void;
            box_function(
                c,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut compile_t,
                            *mut *mut libc::c_void,
                            token_id,
                        ) -> (),
                    >,
                    generate_box_fn,
                >(Some(
                    number_conversion
                        as unsafe extern "C" fn(
                            *mut compile_t,
                            *mut *mut libc::c_void,
                            token_id,
                        ) -> (),
                )),
                data.as_mut_ptr() as *mut libc::c_void,
            );
            box_function(
                c,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut compile_t,
                            *mut *mut libc::c_void,
                            token_id,
                        ) -> (),
                    >,
                    generate_box_fn,
                >(Some(
                    unsafe_number_conversion
                        as unsafe extern "C" fn(
                            *mut compile_t,
                            *mut *mut libc::c_void,
                            token_id,
                        ) -> (),
                )),
                data.as_mut_ptr() as *mut libc::c_void,
            );
            to = to.offset(1);
        }
        from = from.offset(1);
    }
}
#[c2rust::src_loc = "1826:1"]
unsafe extern "C" fn f32__nan(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_nan\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).f32_0,
        &mut (*c).f32_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstNaN((*c).f32_0);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1836:1"]
unsafe extern "C" fn f32__inf(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_inf\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c).f32_0;
    params[1 as libc::c_int as usize] = (*c).i1;
    start_function(
        c,
        t,
        m,
        (*c).f32_0,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut sign: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = LLVMBuildSelect(
        (*c).builder,
        sign,
        LLVMConstInf((*c).f32_0, 1 as libc::c_int != 0),
        LLVMConstInf((*c).f32_0, 0 as libc::c_int != 0),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1851:1"]
unsafe extern "C" fn f32_from_bits(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"from_bits\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c).f32_0;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    start_function(
        c,
        t,
        m,
        (*c).f32_0,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint),
        (*c).f32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1866:1"]
unsafe extern "C" fn f32_bits(mut c: *mut compile_t, mut t: *mut reach_type_t, mut cap: token_id) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"bits\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i32_0,
        &mut (*c).f32_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint),
        (*c).i32_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1877:1"]
unsafe extern "C" fn f64__nan(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_nan\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut _c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).f64_0,
        &mut (*c).f64_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMConstNaN((*c).f64_0);
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1887:1"]
unsafe extern "C" fn f64__inf(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"_inf\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c).f64_0;
    params[1 as libc::c_int as usize] = (*c).i1;
    start_function(
        c,
        t,
        m,
        (*c).f64_0,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut sign: LLVMValueRef = LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint);
    let mut result: LLVMValueRef = LLVMBuildSelect(
        (*c).builder,
        sign,
        LLVMConstInf((*c).f64_0, 1 as libc::c_int != 0),
        LLVMConstInf((*c).f64_0, 0 as libc::c_int != 0),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1902:1"]
unsafe extern "C" fn f64_from_bits(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"from_bits\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, TK_NONE, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut params: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    params[0 as libc::c_int as usize] = (*c).f64_0;
    params[1 as libc::c_int as usize] = (*c).i64_0;
    start_function(
        c,
        t,
        m,
        (*c).f64_0,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        LLVMGetParam((*c_m).func, 1 as libc::c_int as libc::c_uint),
        (*c).f64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1917:1"]
unsafe extern "C" fn f64_bits(mut c: *mut compile_t, mut t: *mut reach_type_t, mut cap: token_id) {
    let mut strtab_name: *const libc::c_char =
        stringtab(b"bits\0" as *const u8 as *const libc::c_char);
    let mut m: *mut reach_method_t = reach_method(t, cap, strtab_name, 0 as *mut ast_t);
    if m.is_null() {
        return;
    }
    (*m).intrinsic = 1 as libc::c_int != 0;
    let mut _c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    start_function(
        c,
        t,
        m,
        (*c).i64_0,
        &mut (*c).f64_0,
        1 as libc::c_int as libc::c_uint,
    );
    let mut result: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint),
        (*c).i64_0,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "1928:1"]
unsafe extern "C" fn fp_intrinsics(mut c: *mut compile_t) {
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    t = reach_type_name((*c).reach, b"F32\0" as *const u8 as *const libc::c_char);
    if !t.is_null() {
        f32__nan(c, t);
        f32__inf(c, t);
        f32_from_bits(c, t);
        box_function(
            c,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
                generate_box_fn,
            >(Some(
                f32_bits as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
            )),
            t as *mut libc::c_void,
        );
    }
    t = reach_type_name((*c).reach, b"F64\0" as *const u8 as *const libc::c_char);
    if !t.is_null() {
        f64__nan(c, t);
        f64__inf(c, t);
        f64_from_bits(c, t);
        box_function(
            c,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> ()>,
                generate_box_fn,
            >(Some(
                f64_bits as unsafe extern "C" fn(*mut compile_t, *mut reach_type_t, token_id) -> (),
            )),
            t as *mut libc::c_void,
        );
    }
}
#[c2rust::src_loc = "1949:1"]
unsafe extern "C" fn make_cpuid(mut c: *mut compile_t) {
    if target_is_x86((*(*c).opt).triple) {
        let mut elems: [LLVMTypeRef; 4] = [(*c).i32_0, (*c).i32_0, (*c).i32_0, (*c).i32_0];
        let mut r_type: LLVMTypeRef = LLVMStructTypeInContext(
            (*c).context,
            elems.as_mut_ptr(),
            4 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        let mut f_type: LLVMTypeRef = LLVMFunctionType(
            r_type,
            &mut (*c).i32_0,
            1 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        let mut fun: LLVMValueRef = codegen_addfun(
            c,
            b"internal.x86.cpuid\0" as *const u8 as *const libc::c_char,
            f_type,
            0 as libc::c_int != 0,
        );
        LLVMSetFunctionCallConv(fun, LLVMCCallConv as libc::c_int as libc::c_uint);
        codegen_startfun(
            c,
            fun,
            0 as LLVMMetadataRef,
            0 as LLVMMetadataRef,
            0 as *mut deferred_reification_t,
            0 as libc::c_int != 0,
        );
        let mut cpuid: LLVMValueRef = LLVMGetInlineAsm(
            f_type,
            b"cpuid\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
            b"={ax},={bx},={cx},={dx},{ax}\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as libc::c_int,
            LLVMInlineAsmDialectATT,
            0 as libc::c_int,
        );
        let mut arg: LLVMValueRef = LLVMGetParam(fun, 0 as libc::c_int as libc::c_uint);
        let mut result: LLVMValueRef = LLVMBuildCall_P(
            (*c).builder,
            cpuid,
            &mut arg,
            1 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildRet((*c).builder, result);
        codegen_finishfun(c);
    }
}
#[c2rust::src_loc = "1973:1"]
unsafe extern "C" fn make_rdtscp(mut c: *mut compile_t) {
    if target_is_x86((*(*c).opt).triple) {
        let mut r_type_fields: [LLVMTypeRef; 2] = [(*c).i64_0, (*c).i32_0];
        let mut r_type: LLVMTypeRef = LLVMStructTypeInContext(
            (*c).context,
            r_type_fields.as_mut_ptr(),
            2 as libc::c_int as libc::c_uint,
            1 as libc::c_int,
        );
        let mut f_type_r: LLVMTypeRef = LLVMFunctionType(
            r_type,
            0 as *mut LLVMTypeRef,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        let mut rdtscp: LLVMValueRef = LLVMAddFunction(
            (*c).module,
            b"llvm.x86.rdtscp\0" as *const u8 as *const libc::c_char,
            f_type_r,
        );
        let mut i32_ptr: LLVMTypeRef =
            LLVMPointerType((*c).i32_0, 0 as libc::c_int as libc::c_uint);
        let mut f_type_f: LLVMTypeRef = LLVMFunctionType(
            (*c).i64_0,
            &mut i32_ptr,
            1 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        let mut fun: LLVMValueRef = codegen_addfun(
            c,
            b"internal.x86.rdtscp\0" as *const u8 as *const libc::c_char,
            f_type_f,
            0 as libc::c_int != 0,
        );
        LLVMSetFunctionCallConv(fun, LLVMCCallConv as libc::c_int as libc::c_uint);
        codegen_startfun(
            c,
            fun,
            0 as LLVMMetadataRef,
            0 as LLVMMetadataRef,
            0 as *mut deferred_reification_t,
            0 as libc::c_int != 0,
        );
        let mut result: LLVMValueRef = LLVMBuildCall_P(
            (*c).builder,
            rdtscp,
            0 as *mut LLVMValueRef,
            0 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut second: LLVMValueRef = LLVMBuildExtractValue(
            (*c).builder,
            result,
            1 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut argptr: LLVMValueRef = LLVMGetParam(fun, 0 as libc::c_int as libc::c_uint);
        LLVMBuildStore((*c).builder, second, argptr);
        let mut first: LLVMValueRef = LLVMBuildExtractValue(
            (*c).builder,
            result,
            0 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildRet((*c).builder, first);
        codegen_finishfun(c);
    }
}
#[c2rust::src_loc = "2005:1"]
unsafe extern "C" fn make_signature_array(
    mut c: *mut compile_t,
    mut c_t: *mut compile_type_t,
    mut signature: *const libc::c_char,
) -> LLVMValueRef {
    let mut args: [LLVMValueRef; 64] = [0 as *mut LLVMOpaqueValue; 64];
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 64 as libc::c_int as libc::c_ulong {
        args[i as usize] = LLVMConstInt(
            (*c).i8_0,
            *signature.offset(i as isize) as libc::c_ulonglong,
            0 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    let mut sig: LLVMValueRef = LLVMConstArray(
        (*c).i8_0,
        args.as_mut_ptr(),
        64 as libc::c_int as libc::c_uint,
    );
    let mut g_sig: LLVMValueRef = LLVMAddGlobal(
        (*c).module,
        LLVMTypeOf(sig),
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMSetLinkage(g_sig, LLVMPrivateLinkage);
    LLVMSetInitializer(g_sig, sig);
    LLVMSetGlobalConstant(g_sig, 1 as libc::c_int);
    LLVMSetUnnamedAddr(g_sig, 1 as libc::c_int);
    args[0 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        0 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[1 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        0 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    let mut ptr: LLVMValueRef =
        LLVMConstInBoundsGEP_P(g_sig, args.as_mut_ptr(), 2 as libc::c_int as libc::c_uint);
    args[0 as libc::c_int as usize] = (*c_t).desc;
    args[1 as libc::c_int as usize] = LLVMConstInt(
        (*c).intptr,
        64 as libc::c_int as libc::c_ulonglong,
        0 as libc::c_int,
    );
    args[2 as libc::c_int as usize] = args[1 as libc::c_int as usize];
    args[3 as libc::c_int as usize] = ptr;
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
    return g_inst;
}
#[no_mangle]
#[c2rust::src_loc = "2040:1"]
pub unsafe extern "C" fn genprim_signature(mut c: *mut compile_t) {
    let mut t: *mut reach_type_t = reach_type_name(
        (*c).reach,
        b"Array_U8_val\0" as *const u8 as *const libc::c_char,
    );
    if t.is_null() {
        return;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
    if !def.is_null() {
    } else {
        ponyint_assert_fail(
            b"def != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genprim.c\0"
                as *const u8 as *const libc::c_char,
            2050 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"genprim_signature\0"))
                .as_ptr(),
        );
    };
    let mut program: *mut ast_t = ast_nearest(def, TK_PROGRAM);
    if !program.is_null() {
    } else {
        ponyint_assert_fail(
            b"program != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genprim.c\0"
                as *const u8 as *const libc::c_char,
            2053 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"genprim_signature\0"))
                .as_ptr(),
        );
    };
    let mut signature: *const libc::c_char = program_signature(program);
    let mut g_array: LLVMValueRef = make_signature_array(c, c_t, signature);
    let mut f_type: LLVMTypeRef = LLVMFunctionType(
        (*c_t).use_type,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut fun: LLVMValueRef = codegen_addfun(
        c,
        b"internal.signature\0" as *const u8 as *const libc::c_char,
        f_type,
        0 as libc::c_int != 0,
    );
    LLVMSetFunctionCallConv(fun, LLVMCCallConv as libc::c_int as libc::c_uint);
    codegen_startfun(
        c,
        fun,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    LLVMBuildRet((*c).builder, g_array);
    codegen_finishfun(c);
}
#[no_mangle]
#[c2rust::src_loc = "2067:1"]
pub unsafe extern "C" fn genprim_builtins(mut c: *mut compile_t) {
    number_conversions(c);
    fp_intrinsics(c);
    make_cpuid(c);
    make_rdtscp(c);
}
#[no_mangle]
#[c2rust::src_loc = "2075:1"]
pub unsafe extern "C" fn genprim_reachable_init(mut c: *mut compile_t, mut program: *mut ast_t) {
    let mut package: *mut ast_t = ast_child(program);
    while !package.is_null() {
        let mut module: *mut ast_t = ast_child(package);
        while !module.is_null() {
            let mut entity: *mut ast_t = ast_child(module);
            while !entity.is_null() {
                if ast_id(entity) as libc::c_uint == TK_PRIMITIVE as libc::c_int as libc::c_uint {
                    let mut id: ast_ptr_t = 0 as *mut ast_t;
                    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
                    let mut children: [*mut *mut ast_t; 3] =
                        [&mut id, &mut typeparams, 0 as *mut *mut ast_t];
                    ast_get_children(
                        entity,
                        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        children.as_mut_ptr(),
                    );
                    if ast_id(typeparams) as libc::c_uint == TK_NONE as libc::c_int as libc::c_uint
                    {
                        let mut type_0: *mut ast_t = type_builtin((*c).opt, entity, ast_name(id));
                        let mut finit: *mut ast_t =
                            ast_get(entity, (*c).str__init, 0 as *mut sym_status_t);
                        let mut ffinal: *mut ast_t =
                            ast_get(entity, (*c).str__final, 0 as *mut sym_status_t);
                        if !finit.is_null() {
                            reach(
                                (*c).reach,
                                type_0,
                                (*c).str__init,
                                0 as *mut ast_t,
                                (*c).opt,
                            );
                            ast_free_unattached(finit);
                        }
                        if !ffinal.is_null() {
                            reach(
                                (*c).reach,
                                type_0,
                                (*c).str__final,
                                0 as *mut ast_t,
                                (*c).opt,
                            );
                            ast_free_unattached(ffinal);
                        }
                        ast_free_unattached(type_0);
                    }
                }
                entity = ast_sibling(entity);
            }
            module = ast_sibling(module);
        }
        package = ast_sibling(package);
    }
}
