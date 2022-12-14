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
        LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
    };
    use super::_size_t_h::size_t;
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
            Val: u64,
        ) -> LLVMAttributeRef;
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
        #[c2rust::src_loc = "1258:1"]
        pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1263:1"]
        pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> libc::c_uint;
        #[c2rust::src_loc = "1276:1"]
        pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
        #[c2rust::src_loc = "1300:1"]
        pub fn LLVMStructTypeInContext(
            C: LLVMContextRef,
            ElementTypes: *mut LLVMTypeRef,
            ElementCount: libc::c_uint,
            Packed: LLVMBool,
        ) -> LLVMTypeRef;
        #[c2rust::src_loc = "1400:1"]
        pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1443:1"]
        pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: libc::c_uint)
            -> LLVMTypeRef;
        #[c2rust::src_loc = "1669:1"]
        pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1750:1"]
        pub fn LLVMSetValueName(Val: LLVMValueRef, Name: *const libc::c_char);
        #[c2rust::src_loc = "1934:1"]
        pub fn LLVMConstInt(
            IntTy: LLVMTypeRef,
            N: libc::c_ulonglong,
            SignExtend: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2182:1"]
        pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2236:1"]
        pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
        #[c2rust::src_loc = "2257:1"]
        pub fn LLVMSetUnnamedAddr(Global: LLVMValueRef, HasUnnamedAddr: LLVMBool);
        #[c2rust::src_loc = "2582:1"]
        pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "2604:1"]
        pub fn LLVMAddAttributeAtIndex(
            F: LLVMValueRef,
            Idx: LLVMAttributeIndex,
            A: LLVMAttributeRef,
        );
        #[c2rust::src_loc = "2643:1"]
        pub fn LLVMCountParams(Fn: LLVMValueRef) -> libc::c_uint;
        #[c2rust::src_loc = "2656:1"]
        pub fn LLVMGetParams(Fn: LLVMValueRef, Params: *mut LLVMValueRef);
        #[c2rust::src_loc = "2665:1"]
        pub fn LLVMGetParam(Fn: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
        #[c2rust::src_loc = "2683:1"]
        pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2699:1"]
        pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3297:1"]
        pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3681:1"]
        pub fn LLVMBuildRetVoid(_: LLVMBuilderRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3682:1"]
        pub fn LLVMBuildRet(_: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3724:1"]
        pub fn LLVMAddCase(Switch: LLVMValueRef, OnVal: LLVMValueRef, Dest: LLVMBasicBlockRef);
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
        #[c2rust::src_loc = "3957:1"]
        pub fn LLVMBuildBitCast(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
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
        #[c2rust::src_loc = "257:1"]
        pub fn LLVMABISizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> libc::c_ulonglong;
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
        #[c2rust::src_loc = "551:1"]
        pub fn LLVMDIBuilderCreateSubroutineType(
            Builder: LLVMDIBuilderRef,
            File: LLVMMetadataRef,
            ParameterTypes: *mut LLVMMetadataRef,
            NumParameterTypes: libc::c_uint,
            Flags: LLVMDIFlags,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "1104:1"]
        pub fn LLVMDIBuilderCreateExpression(
            Builder: LLVMDIBuilderRef,
            Addr: *mut u64,
            Length: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "1327:1"]
        pub fn LLVMDIBuilderCreateParameterVariable(
            Builder: LLVMDIBuilderRef,
            Scope: LLVMMetadataRef,
            Name: *const libc::c_char,
            NameLen: size_t,
            ArgNo: libc::c_uint,
            File: LLVMMetadataRef,
            LineNo: libc::c_uint,
            Ty: LLVMMetadataRef,
            AlwaysPreserve: LLVMBool,
            Flags: LLVMDIFlags,
        ) -> LLVMMetadataRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "63:1"]
        pub fn errors_get_count(errors: *mut errors_t) -> size_t;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::error_h::errors_t;
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "114:1"]
        pub fn ast_childlast(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
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
        pub fn reach_mangled_next(map: *mut reach_mangled_t, i: *mut size_t)
            -> *mut reach_method_t;
        #[c2rust::src_loc = "22:3"]
        pub fn reach_method_names_next(
            map: *mut reach_method_names_t,
            i: *mut size_t,
        ) -> *mut reach_method_name_t;
        #[c2rust::src_loc = "23:55"]
        pub fn reach_types_next(map: *mut reach_types_t, i: *mut size_t) -> *mut reach_type_t;
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
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "291:1"]
        pub fn codegen_call(
            c: *mut compile_t,
            fun: LLVMValueRef,
            args: *mut LLVMValueRef,
            count: size_t,
            setcc: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "289:1"]
        pub fn codegen_block(c: *mut compile_t, name: *const libc::c_char) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "281:1"]
        pub fn codegen_setlocal(c: *mut compile_t, name: *const libc::c_char, alloca: LLVMValueRef);
        #[c2rust::src_loc = "277:1"]
        pub fn codegen_debugloc(c: *mut compile_t, ast: *mut ast_t);
        #[c2rust::src_loc = "262:1"]
        pub fn codegen_scope_lifetime_end(c: *mut compile_t);
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
        #[c2rust::src_loc = "43:1"]
        pub fn LLVMBuildCall_P(
            B: LLVMBuilderRef,
            Fn: LLVMValueRef,
            Args: *mut LLVMValueRef,
            NumArgs: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.h:1"]
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
        #[c2rust::src_loc = "57:1"]
        pub fn LLVMDIBuilderCreateMethod(
            d: LLVMDIBuilderRef,
            scope: LLVMMetadataRef,
            name: *const libc::c_char,
            linkage: *const libc::c_char,
            file: LLVMMetadataRef,
            line: libc::c_uint,
            type_0: LLVMMetadataRef,
            func: LLVMValueRef,
            optimized: libc::c_int,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "62:1"]
        pub fn LLVMDIBuilderCreateArtificialVariable(
            d: LLVMDIBuilderRef,
            scope: LLVMMetadataRef,
            name: *const libc::c_char,
            arg: libc::c_uint,
            file: LLVMMetadataRef,
            line: libc::c_uint,
            type_0: LLVMMetadataRef,
        ) -> LLVMMetadataRef;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:2"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn genname_alloc(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "35:1"]
        pub fn genname_be(name: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "43:1"]
        pub fn genname_program_fn(
            program: *const libc::c_char,
            name: *const libc::c_char,
        ) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.h:3"]
pub mod gencall_h {
    use super::codegen_h::compile_t;
    use super::reach_h::{reach_method_t, reach_type_t};
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gen_send_message(
            c: *mut compile_t,
            m: *mut reach_method_t,
            args: *mut LLVMValueRef,
            args_ast: *mut ast_t,
        );
        #[c2rust::src_loc = "21:1"]
        pub fn gencall_runtime(
            c: *mut compile_t,
            name: *const libc::c_char,
            args: *mut LLVMValueRef,
            count: libc::c_int,
            ret: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "24:1"]
        pub fn gencall_create(
            c: *mut compile_t,
            t: *mut reach_type_t,
            call: *mut ast_t,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "26:1"]
        pub fn gencall_alloc(
            c: *mut compile_t,
            t: *mut reach_type_t,
            call: *mut ast_t,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.h:4"]
pub mod gentrace_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:6"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.h:7"]
pub mod genreference_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gen_this(c: *mut compile_t, ast: *mut ast_t) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:10"]
pub mod subtype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn is_pointer(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "34:1"]
        pub fn is_nullable_pointer(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn is_none(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:14"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::ast_h::{
    ast_child, ast_childidx, ast_childlast, ast_error, ast_free_unattached, ast_get_children,
    ast_id, ast_line, ast_name, ast_pos, ast_ptr_t, ast_sibling, ast_type,
};
pub use self::codegen_h::{
    codegen_addfun, codegen_block, codegen_call, codegen_debugloc, codegen_finishfun,
    codegen_scope_lifetime_end, codegen_setlocal, codegen_startfun, compile_frame_t,
    compile_locals_t, compile_t, ffi_decls_t, genned_strings_t, LLVMBuildCall_P, LLVMBuildLoad_P,
    LLVMBuildStructGEP_P,
};
use self::error_h::errors_get_count;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
use self::gencall_h::{gen_send_message, gencall_alloc, gencall_create, gencall_runtime};
use self::gendebug_h::{
    LLVMDIBuilderCreateArtificialVariable, LLVMDIBuilderCreateMethod, LLVMDIBuilderInsertDeclare,
};
use self::genexpr_h::{gen_assign_cast, gen_expr};
pub use self::genfun_h::compile_method_t;
use self::genname_h::{genname_alloc, genname_be, genname_program_fn};
use self::genreference_h::gen_this;
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
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_mangled_next, reach_mangled_t,
    reach_method, reach_method_name, reach_method_name_t, reach_method_names_next,
    reach_method_names_t, reach_method_stack_t, reach_method_t, reach_methods_t, reach_param_t,
    reach_t, reach_type_cache_t, reach_type_t, reach_types_next, reach_types_t,
};
pub use self::reify_h::{deferred_reification_t, deferred_reify};
use self::string_h::{memset, strlen};

use self::subtype_h::{is_none, is_nullable_pointer, is_pointer};
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
    C2RustUnnamed, LLVMAMDGPUCSCallConv, LLVMAMDGPUESCallConv, LLVMAMDGPUGSCallConv,
    LLVMAMDGPUHSCallConv, LLVMAMDGPUKERNELCallConv, LLVMAMDGPULSCallConv, LLVMAMDGPUPSCallConv,
    LLVMAMDGPUVSCallConv, LLVMARMAAPCSCallConv, LLVMARMAAPCSVFPCallConv, LLVMARMAPCSCallConv,
    LLVMAVRBUILTINCallConv, LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddAttributeAtIndex,
    LLVMAddCase, LLVMAddFunction, LLVMAnyRegCallConv, LLVMAppendingLinkage, LLVMArrayTypeKind,
    LLVMAttributeFunctionIndex, LLVMAttributeIndex, LLVMAttributeReturnIndex,
    LLVMAvailableExternallyLinkage, LLVMBFloatTypeKind, LLVMBuildAlloca, LLVMBuildBitCast,
    LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildStore, LLVMCCallConv, LLVMCXXFASTTLSCallConv,
    LLVMCallConv, LLVMColdCallConv, LLVMCommonLinkage, LLVMConstBitCast, LLVMConstInt,
    LLVMCountParamTypes, LLVMCountParams, LLVMCreateEnumAttribute, LLVMDLLExportLinkage,
    LLVMDLLImportLinkage, LLVMDoubleTypeKind, LLVMExternalLinkage, LLVMExternalWeakLinkage,
    LLVMFP128TypeKind, LLVMFastCallConv, LLVMFloatTypeKind, LLVMFunctionType, LLVMFunctionTypeKind,
    LLVMGHCCallConv, LLVMGetElementType, LLVMGetEnumAttributeKindForName, LLVMGetFirstParam,
    LLVMGetInsertBlock, LLVMGetNextParam, LLVMGetParam, LLVMGetParamTypes, LLVMGetParams,
    LLVMGetReturnType, LLVMGetTypeKind, LLVMGhostLinkage, LLVMHHVMCCallConv, LLVMHHVMCallConv,
    LLVMHalfTypeKind, LLVMHiPECallConv, LLVMIntegerTypeKind, LLVMIntelOCLBICallConv,
    LLVMInternalLinkage, LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage, LLVMLinkOnceODRAutoHideLinkage,
    LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage, LLVMLinkerPrivateWeakLinkage,
    LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv, LLVMMetadataTypeKind, LLVMPPC_FP128TypeKind,
    LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv, LLVMPointerType, LLVMPointerTypeKind,
    LLVMPositionBuilderAtEnd, LLVMPreserveAllCallConv, LLVMPreserveMostCallConv,
    LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv, LLVMScalableVectorTypeKind,
    LLVMSetFunctionCallConv, LLVMSetInstructionCallConv, LLVMSetLinkage, LLVMSetUnnamedAddr,
    LLVMSetValueName, LLVMStructTypeInContext, LLVMStructTypeKind, LLVMSwiftCallConv,
    LLVMTokenTypeKind, LLVMTypeKind, LLVMTypeOf, LLVMVectorTypeKind, LLVMVoidTypeKind,
    LLVMWeakAnyLinkage, LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv,
    LLVMX8664SysVCallConv, LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv,
    LLVMX86StdcallCallConv, LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv,
    LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind, LLVMX86_MMXTypeKind,
};
pub use self::DebugInfo_h::{
    LLVMDIBuilderCreateExpression, LLVMDIBuilderCreateParameterVariable,
    LLVMDIBuilderCreateSubroutineType, LLVMDIFlagAccessibility, LLVMDIFlagAppleBlock,
    LLVMDIFlagArtificial, LLVMDIFlagBigEndian, LLVMDIFlagBitField, LLVMDIFlagEnumClass,
    LLVMDIFlagExplicit, LLVMDIFlagFixedEnum, LLVMDIFlagFwdDecl, LLVMDIFlagIndirectVirtualBase,
    LLVMDIFlagIntroducedVirtual, LLVMDIFlagLValueReference, LLVMDIFlagLittleEndian,
    LLVMDIFlagMultipleInheritance, LLVMDIFlagNoReturn, LLVMDIFlagNonTrivial,
    LLVMDIFlagObjcClassComplete, LLVMDIFlagObjectPointer, LLVMDIFlagPrivate, LLVMDIFlagProtected,
    LLVMDIFlagPrototyped, LLVMDIFlagPtrToMemberRep, LLVMDIFlagPublic, LLVMDIFlagRValueReference,
    LLVMDIFlagReserved, LLVMDIFlagReservedBit4, LLVMDIFlagSingleInheritance,
    LLVMDIFlagStaticMember, LLVMDIFlagThunk, LLVMDIFlagTypePassByReference,
    LLVMDIFlagTypePassByValue, LLVMDIFlagVector, LLVMDIFlagVirtual, LLVMDIFlagVirtualInheritance,
    LLVMDIFlagZero, LLVMDIFlags,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{LLVMABISizeOfType, LLVMOpaqueTargetData, LLVMTargetDataRef};
pub use self::Types_h::{
    LLVMAttributeRef, LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef,
    LLVMDIBuilderRef, LLVMMetadataRef, LLVMModuleRef, LLVMOpaqueAttributeRef, LLVMOpaqueBasicBlock,
    LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder, LLVMOpaqueMetadata,
    LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef, LLVMValueRef,
};
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn compile_method_free(mut p: *mut libc::c_void) {
    ponyint_pool_free(1 as libc::c_int as size_t, p);
}
#[c2rust::src_loc = "24:1"]
unsafe extern "C" fn name_param(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut func: LLVMValueRef,
    mut name: *const libc::c_char,
    mut index: libc::c_uint,
    mut line: size_t,
    mut pos: size_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut value: LLVMValueRef = LLVMGetParam(func, index);
    LLVMSetValueName(value, name);
    let mut alloc: LLVMValueRef = LLVMBuildAlloca((*c).builder, (*c_t).mem_type, name);
    value = gen_assign_cast(c, (*c_t).mem_type, value, (*t).ast_cap);
    LLVMBuildStore((*c).builder, value, alloc);
    codegen_setlocal(c, name, alloc);
    let mut info: LLVMMetadataRef = 0 as *mut LLVMOpaqueMetadata;
    if index == 0 as libc::c_int as libc::c_uint {
        info = LLVMDIBuilderCreateArtificialVariable(
            (*c).di,
            (*c_m).di_method,
            name,
            index.wrapping_add(1 as libc::c_int as libc::c_uint),
            (*c_m).di_file,
            ast_line((*(*m).fun).ast) as libc::c_uint,
            (*c_t).di_type,
        );
    } else {
        info = LLVMDIBuilderCreateParameterVariable(
            (*c).di,
            (*c_m).di_method,
            name,
            strlen(name),
            index.wrapping_add(1 as libc::c_int as libc::c_uint),
            (*c_m).di_file,
            ast_line((*(*m).fun).ast) as libc::c_uint,
            (*c_t).di_type,
            0 as libc::c_int,
            LLVMDIFlagZero,
        );
    }
    let mut expr: LLVMMetadataRef =
        LLVMDIBuilderCreateExpression((*c).di, 0 as *mut u64, 0 as libc::c_int as size_t);
    LLVMDIBuilderInsertDeclare(
        (*c).di,
        alloc,
        info,
        expr,
        line as libc::c_uint,
        pos as libc::c_uint,
        (*c_m).di_method,
        LLVMGetInsertBlock((*c).builder),
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn name_params(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut func: LLVMValueRef,
) {
    let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint {
        name_param(
            c,
            t,
            m,
            func,
            (*c).str_this,
            0 as libc::c_int as libc::c_uint,
            ast_line((*(*m).fun).ast),
            ast_pos((*(*m).fun).ast),
        );
        offset = 1 as libc::c_int as libc::c_uint;
    }
    let mut params: *mut ast_t = ast_childidx((*(*m).fun).ast, 3 as libc::c_int as size_t);
    let mut param: *mut ast_t = ast_child(params);
    let mut i: size_t = 0;
    while i < (*m).param_count {
        let mut r_param: *mut reach_param_t =
            &mut *((*m).params).offset(i as isize) as *mut reach_param_t;
        name_param(
            c,
            (*r_param).type_0,
            m,
            func,
            (*r_param).name,
            (i as libc::c_uint).wrapping_add(offset),
            ast_line(param),
            ast_pos(param),
        );
        param = ast_sibling(param);
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn make_signature(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
    mut message_type: bool,
) {
    let mut count: size_t = (*m).param_count;
    let mut offset: size_t = 0;
    if (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint {
        count = count.wrapping_add(1);
        offset = offset.wrapping_add(1);
    }
    let mut tparam_size: size_t =
        count.wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
    if message_type {
        tparam_size = (tparam_size as libc::c_ulong).wrapping_add(
            tparam_size.wrapping_add(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong),
            ),
        ) as size_t as size_t;
    }
    let mut tparams: *mut LLVMTypeRef = ponyint_pool_alloc_size(tparam_size) as *mut LLVMTypeRef;
    let mut mparams: *mut LLVMTypeRef = 0 as *mut LLVMTypeRef;
    if message_type {
        mparams = &mut *tparams.offset(count as isize) as *mut LLVMTypeRef;
    }
    let mut bare_void: bool = 0 as libc::c_int != 0;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if (*m).cap as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        bare_void = is_none((*(*m).result).ast);
    } else {
        let ref mut fresh0 = *tparams.offset(0 as libc::c_int as isize);
        *fresh0 = (*c_t).use_type;
    }
    let mut i: size_t = 0;
    while i < (*m).param_count {
        let mut p_c_t: *mut compile_type_t =
            (*(*((*m).params).offset(i as isize)).type_0).c_type as *mut compile_type_t;
        let ref mut fresh1 = *tparams.offset(i.wrapping_add(offset) as isize);
        *fresh1 = (*p_c_t).use_type;
        if message_type {
            let ref mut fresh2 = *mparams.offset(
                i.wrapping_add(offset)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
            *fresh2 = (*p_c_t).mem_type;
        }
        i = i.wrapping_add(1);
    }
    if bare_void as libc::c_int != 0
        || (*n).name == (*c).str__final
        || ast_id((*(*m).fun).ast) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint
            && (*t).underlying as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint
    {
        let ref mut fresh3 = (*c_m).func_type;
        *fresh3 = LLVMFunctionType(
            (*c).void_type,
            tparams,
            count as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
    } else {
        let ref mut fresh4 = (*c_m).func_type;
        *fresh4 = LLVMFunctionType(
            (*((*(*m).result).c_type as *mut compile_type_t)).use_type,
            tparams,
            count as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
    }
    if message_type {
        let ref mut fresh5 = *mparams.offset(0 as libc::c_int as isize);
        *fresh5 = (*c).i32_0;
        let ref mut fresh6 = *mparams.offset(1 as libc::c_int as isize);
        *fresh6 = (*c).i32_0;
        let ref mut fresh7 = *mparams.offset(2 as libc::c_int as isize);
        *fresh7 = (*c).void_ptr;
        let ref mut fresh8 = (*c_m).msg_type;
        *fresh8 = LLVMStructTypeInContext(
            (*c).context,
            mparams,
            (count as libc::c_int + 2 as libc::c_int) as libc::c_uint,
            0 as libc::c_int,
        );
    }
    ponyint_pool_free_size(tparam_size, tparams as *mut libc::c_void);
}
#[c2rust::src_loc = "154:1"]
unsafe extern "C" fn make_function_debug(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut func: LLVMValueRef,
) {
    let mut count: size_t = ((*m).param_count).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut offset: size_t = 1 as libc::c_int as size_t;
    if (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint {
        count = count.wrapping_add(1);
        offset = offset.wrapping_add(1);
    }
    let mut md_size: size_t =
        count.wrapping_mul(::core::mem::size_of::<*mut reach_type_t>() as libc::c_ulong);
    let mut md: *mut LLVMMetadataRef = ponyint_pool_alloc_size(md_size) as *mut LLVMMetadataRef;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let ref mut fresh9 = *md.offset(0 as libc::c_int as isize);
    *fresh9 = (*((*(*m).result).c_type as *mut compile_type_t)).di_type;
    if (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint {
        let ref mut fresh10 = *md.offset(1 as libc::c_int as isize);
        *fresh10 = (*c_t).di_type;
    }
    let mut i: size_t = 0;
    while i < (*m).param_count {
        let ref mut fresh11 = *md.offset(i.wrapping_add(offset) as isize);
        *fresh11 = (*((*(*((*m).params).offset(i as isize)).type_0).c_type as *mut compile_type_t))
            .di_type;
        i = i.wrapping_add(1);
    }
    let ref mut fresh12 = (*c_m).di_file;
    *fresh12 = (*c_t).di_file;
    let mut subroutine: LLVMMetadataRef = LLVMDIBuilderCreateSubroutineType(
        (*c).di,
        (*c_m).di_file,
        md,
        count as libc::c_uint,
        LLVMDIFlagZero,
    );
    let mut scope: LLVMMetadataRef = 0 as *mut LLVMOpaqueMetadata;
    if !((*c_t).di_type_embed).is_null() {
        scope = (*c_t).di_type_embed;
    } else {
        scope = (*c_t).di_type;
    }
    let mut id: *mut ast_t = ast_childidx((*(*m).fun).ast, 1 as libc::c_int as size_t);
    let ref mut fresh13 = (*c_m).di_method;
    *fresh13 = LLVMDIBuilderCreateMethod(
        (*c).di,
        scope,
        ast_name(id),
        (*m).full_name,
        (*c_m).di_file,
        ast_line((*(*m).fun).ast) as libc::c_uint,
        subroutine,
        func,
        (*(*c).opt).release as libc::c_int,
    );
    ponyint_pool_free_size(md_size, md as *mut libc::c_void);
}
#[c2rust::src_loc = "208:1"]
unsafe extern "C" fn make_prototype(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
) {
    if (*m).intrinsic {
        return;
    }
    let mut handler: bool = 0 as libc::c_int != 0;
    let mut is_trait: bool = 0 as libc::c_int != 0;
    match ast_id((*(*m).fun).ast) as libc::c_uint {
        88 => {
            handler = (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint;
        }
        90 => {
            handler = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    match (*t).underlying as libc::c_uint {
        74 | 75 | 76 | 77 => {}
        149 | 56 | 72 | 73 => {
            is_trait = 1 as libc::c_int != 0;
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                        as *const u8 as *const libc::c_char,
                    247 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"make_prototype\0",
                    ))
                    .as_ptr(),
                );
            };
            return;
        }
    }
    make_signature(
        c,
        t,
        n,
        m,
        handler as libc::c_int != 0 || is_trait as libc::c_int != 0,
    );
    if is_trait {
        return;
    }
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !handler {
        let ref mut fresh14 = (*c_m).func;
        *fresh14 = codegen_addfun(c, (*m).full_name, (*c_m).func_type, 1 as libc::c_int != 0);
        genfun_param_attrs(c, t, m, (*c_m).func);
        make_function_debug(c, t, m, (*c_m).func);
    } else {
        let mut count: size_t = LLVMCountParamTypes((*c_m).func_type) as size_t;
        let mut buf_size: size_t =
            count.wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
        let mut tparams: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
        LLVMGetParamTypes((*c_m).func_type, tparams);
        let mut sender_name: *const libc::c_char = genname_be((*m).full_name);
        let ref mut fresh15 = (*c_m).func;
        *fresh15 = codegen_addfun(c, sender_name, (*c_m).func_type, 1 as libc::c_int != 0);
        genfun_param_attrs(c, t, m, (*c_m).func);
        if !(*m).forwarding {
            let mut handler_type: LLVMTypeRef = LLVMFunctionType(
                (*c).void_type,
                tparams,
                count as libc::c_int as libc::c_uint,
                0 as libc::c_int,
            );
            let ref mut fresh16 = (*c_m).func_handler;
            *fresh16 = codegen_addfun(c, (*m).full_name, handler_type, 1 as libc::c_int != 0);
            genfun_param_attrs(c, t, m, (*c_m).func_handler);
            make_function_debug(c, t, m, (*c_m).func_handler);
        }
        ponyint_pool_free_size(buf_size, tparams as *mut libc::c_void);
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if (*n).name == (*c).str__final {
        if ((*c_t).final_fn).is_null() {
        } else {
            ponyint_assert_fail(
                b"c_t->final_fn == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                    as *const u8 as *const libc::c_char,
                297 as libc::c_int as size_t,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"make_prototype\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh17 = (*c_t).final_fn;
        *fresh17 = (*c_m).func;
        LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
        LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
    } else if (*n).name == (*c).str__serialise_space {
        let ref mut fresh18 = (*c_t).custom_serialise_space_fn;
        *fresh18 = (*c_m).func;
        LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
        LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
    } else if (*n).name == (*c).str__serialise {
        let ref mut fresh19 = (*c_t).custom_serialise_fn;
        *fresh19 = (*c_m).func;
    } else if (*n).name == (*c).str__deserialise {
        let ref mut fresh20 = (*c_t).custom_deserialise_fn;
        *fresh20 = (*c_m).func;
        LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
        LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
    }
    if (*n).cap as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
        LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
        LLVMSetUnnamedAddr((*c_m).func, 0 as libc::c_int);
        if (*t).bare_method == m {
            if ((*c_t).instance).is_null() {
            } else {
                ponyint_assert_fail(
                    b"c_t->instance == NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                        as *const u8 as *const libc::c_char,
                    321 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"make_prototype\0",
                    ))
                    .as_ptr(),
                );
            };
            let ref mut fresh21 = (*c_t).instance;
            *fresh21 = LLVMConstBitCast((*c_m).func, (*c).void_ptr);
        }
    }
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn add_dispatch_case(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut params: *mut reach_param_t,
    mut index: u32,
    mut handler: LLVMValueRef,
    mut fun_type: LLVMTypeRef,
    mut msg_type: LLVMTypeRef,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    codegen_startfun(
        c,
        (*c_t).dispatch_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    let mut block: LLVMBasicBlockRef =
        codegen_block(c, b"handler\0" as *const u8 as *const libc::c_char);
    let mut id: LLVMValueRef =
        LLVMConstInt((*c).i32_0, index as libc::c_ulonglong, 0 as libc::c_int);
    LLVMAddCase((*c_t).dispatch_switch, id, block);
    LLVMPositionBuilderAtEnd((*c).builder, block);
    let mut ctx: LLVMValueRef = LLVMGetParam((*c_t).dispatch_fn, 0 as libc::c_int as libc::c_uint);
    let mut this_ptr: LLVMValueRef =
        LLVMGetParam((*c_t).dispatch_fn, 1 as libc::c_int as libc::c_uint);
    let mut msg: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        LLVMGetParam((*c_t).dispatch_fn, 2 as libc::c_int as libc::c_uint),
        msg_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut count: size_t = LLVMCountParamTypes(fun_type) as size_t;
    let mut params_buf_size: size_t =
        count.wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
    let mut param_types: *mut LLVMTypeRef =
        ponyint_pool_alloc_size(params_buf_size) as *mut LLVMTypeRef;
    LLVMGetParamTypes(fun_type, param_types);
    let mut args_buf_size: size_t =
        count.wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut args: *mut LLVMValueRef = ponyint_pool_alloc_size(args_buf_size) as *mut LLVMValueRef;
    let ref mut fresh22 = *args.offset(0 as libc::c_int as isize);
    *fresh22 = LLVMBuildBitCast(
        (*c).builder,
        this_ptr,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count as libc::c_int {
        let mut field: LLVMValueRef = LLVMBuildStructGEP_P(
            (*c).builder,
            msg,
            (i + 2 as libc::c_int) as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh23 = *args.offset(i as isize);
        *fresh23 = LLVMBuildLoad_P(
            (*c).builder,
            field,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh24 = *args.offset(i as isize);
        *fresh24 = gen_assign_cast(
            c,
            *param_types.offset(i as isize),
            *args.offset(i as isize),
            (*(*params.offset((i - 1 as libc::c_int) as isize)).type_0).ast_cap,
        );
        i += 1;
    }
    let mut need_trace: bool = 0 as libc::c_int != 0;
    let mut i_0: size_t = 0;
    while i_0 < count.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        if gentrace_needed(
            c,
            (*params.offset(i_0 as isize)).ast,
            (*params.offset(i_0 as isize)).ast,
        ) {
            need_trace = 1 as libc::c_int != 0;
            break;
        } else {
            i_0 = i_0.wrapping_add(1);
        }
    }
    if need_trace {
        gencall_runtime(
            c,
            b"pony_gc_recv\0" as *const u8 as *const libc::c_char,
            &mut ctx,
            1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
        let mut i_1: size_t = 1 as libc::c_int as size_t;
        while i_1 < count {
            gentrace(
                c,
                ctx,
                *args.offset(i_1 as isize),
                *args.offset(i_1 as isize),
                (*params.offset(i_1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)).ast,
                (*params.offset(i_1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)).ast,
            );
            i_1 = i_1.wrapping_add(1);
        }
        gencall_runtime(
            c,
            b"pony_recv_done\0" as *const u8 as *const libc::c_char,
            &mut ctx,
            1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    codegen_call(c, handler, args, count, 1 as libc::c_int != 0);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
    ponyint_pool_free_size(args_buf_size, args as *mut libc::c_void);
    ponyint_pool_free_size(params_buf_size, param_types as *mut libc::c_void);
}
#[c2rust::src_loc = "393:1"]
unsafe extern "C" fn call_embed_finalisers(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut call_location: *mut ast_t,
    mut obj: LLVMValueRef,
) {
    let mut base: u32 = 0 as libc::c_int as u32;
    if (*t).underlying as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
        base = base.wrapping_add(1);
    }
    if (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
        base = base.wrapping_add(1);
    }
    let mut i: u32 = 0 as libc::c_int as u32;
    while i < (*t).field_count {
        let mut field: *mut reach_field_t =
            &mut *((*t).fields).offset(i as isize) as *mut reach_field_t;
        if (*field).embed {
            let mut final_fn: LLVMValueRef =
                (*((*(*field).type_0).c_type as *mut compile_type_t)).final_fn;
            if !final_fn.is_null() {
                let mut field_ref: LLVMValueRef = LLVMBuildStructGEP_P(
                    (*c).builder,
                    obj,
                    base.wrapping_add(i),
                    b"\0" as *const u8 as *const libc::c_char,
                );
                codegen_debugloc(c, call_location);
                LLVMBuildCall_P(
                    (*c).builder,
                    final_fn,
                    &mut field_ref,
                    1 as libc::c_int as libc::c_uint,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                codegen_debugloc(c, 0 as *mut ast_t);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[c2rust::src_loc = "421:1"]
unsafe extern "C" fn genfun_fun(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !((*c_m).func).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            425 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"genfun_fun\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        (*(*m).fun).ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    codegen_startfun(
        c,
        (*c_m).func,
        (*c_m).di_file,
        (*c_m).di_method,
        (*m).fun,
        ast_id(cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint,
    );
    name_params(c, t, m, (*c_m).func);
    let mut finaliser: bool = (*c_m).func == (*c_t).final_fn;
    if finaliser {
        call_embed_finalisers(c, t, body, gen_this(c, 0 as *mut ast_t));
    }
    let mut value: LLVMValueRef = gen_expr(c, body);
    if value.is_null() {
        return 0 as libc::c_int != 0;
    }
    if value != 1 as libc::c_int as LLVMValueRef {
        let mut r_result: *mut ast_t = deferred_reify((*m).fun, result, (*c).opt);
        if finaliser as libc::c_int != 0
            || ast_id(cap) as libc::c_uint == TK_AT as libc::c_int as libc::c_uint
                && is_none(r_result) as libc::c_int != 0
        {
            ast_free_unattached(r_result);
            codegen_scope_lifetime_end(c);
            codegen_debugloc(c, ast_childlast(body));
            LLVMBuildRetVoid((*c).builder);
        } else {
            let mut f_type: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf((*c_m).func));
            let mut r_type: LLVMTypeRef = LLVMGetReturnType(f_type);
            let mut body_type: *mut ast_t = deferred_reify((*m).fun, ast_type(body), (*c).opt);
            let mut ret: LLVMValueRef = gen_assign_cast(c, r_type, value, body_type);
            ast_free_unattached(body_type);
            ast_free_unattached(r_result);
            if ret.is_null() {
                return 0 as libc::c_int != 0;
            }
            codegen_scope_lifetime_end(c);
            codegen_debugloc(c, ast_childlast(body));
            LLVMBuildRet((*c).builder, ret);
        }
        codegen_debugloc(c, 0 as *mut ast_t);
    }
    codegen_finishfun(c);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "480:1"]
unsafe extern "C" fn genfun_be(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !((*c_m).func).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            483 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"genfun_be\0")).as_ptr(),
        );
    };
    if !((*c_m).func_handler).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func_handler != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            484 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"genfun_be\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        (*(*m).fun).ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    codegen_startfun(
        c,
        (*c_m).func_handler,
        (*c_m).di_file,
        (*c_m).di_method,
        (*m).fun,
        0 as libc::c_int != 0,
    );
    name_params(c, t, m, (*c_m).func_handler);
    let mut value: LLVMValueRef = gen_expr(c, body);
    if value.is_null() {
        return 0 as libc::c_int != 0;
    }
    codegen_scope_lifetime_end(c);
    if value != 1 as libc::c_int as LLVMValueRef {
        LLVMBuildRetVoid((*c).builder);
    }
    codegen_finishfun(c);
    codegen_startfun(
        c,
        (*c_m).func,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        (*m).fun,
        0 as libc::c_int != 0,
    );
    let mut buf_size: size_t = ((*m).param_count)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut param_vals: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    LLVMGetParams((*c_m).func, param_vals);
    gen_send_message(c, m, param_vals, params);
    LLVMBuildRet((*c).builder, (*c).none_instance);
    codegen_finishfun(c);
    ponyint_pool_free_size(buf_size, param_vals as *mut libc::c_void);
    let mut msg_type_ptr: LLVMTypeRef =
        LLVMPointerType((*c_m).msg_type, 0 as libc::c_int as libc::c_uint);
    add_dispatch_case(
        c,
        t,
        (*m).params,
        (*m).vtable_index,
        (*c_m).func_handler,
        (*c_m).func_type,
        msg_type_ptr,
    );
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "528:1"]
unsafe extern "C" fn genfun_new(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !((*c_m).func).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            532 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"genfun_new\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        (*(*m).fun).ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    codegen_startfun(
        c,
        (*c_m).func,
        (*c_m).di_file,
        (*c_m).di_method,
        (*m).fun,
        0 as libc::c_int != 0,
    );
    name_params(c, t, m, (*c_m).func);
    let mut value: LLVMValueRef = gen_expr(c, body);
    if value.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*c_t).primitive).is_null() {
        value = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    }
    codegen_scope_lifetime_end(c);
    codegen_debugloc(c, ast_childlast(body));
    if (*t).underlying as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint {
        LLVMBuildRetVoid((*c).builder);
    } else {
        LLVMBuildRet((*c).builder, value);
    }
    codegen_debugloc(c, 0 as *mut ast_t);
    codegen_finishfun(c);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "562:1"]
unsafe extern "C" fn genfun_newbe(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !((*c_m).func).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            565 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"genfun_newbe\0")).as_ptr(),
        );
    };
    if !((*c_m).func_handler).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func_handler != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            566 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"genfun_newbe\0")).as_ptr(),
        );
    };
    let mut cap: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeparams: ast_ptr_t = 0 as *mut ast_t;
    let mut params: ast_ptr_t = 0 as *mut ast_t;
    let mut result: ast_ptr_t = 0 as *mut ast_t;
    let mut can_error: ast_ptr_t = 0 as *mut ast_t;
    let mut body: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 8] = [
        &mut cap,
        &mut id,
        &mut typeparams,
        &mut params,
        &mut result,
        &mut can_error,
        &mut body,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        (*(*m).fun).ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    codegen_startfun(
        c,
        (*c_m).func_handler,
        (*c_m).di_file,
        (*c_m).di_method,
        (*m).fun,
        0 as libc::c_int != 0,
    );
    name_params(c, t, m, (*c_m).func_handler);
    let mut value: LLVMValueRef = gen_expr(c, body);
    if value.is_null() {
        return 0 as libc::c_int != 0;
    }
    codegen_scope_lifetime_end(c);
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
    codegen_startfun(
        c,
        (*c_m).func,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        (*m).fun,
        0 as libc::c_int != 0,
    );
    let mut buf_size: size_t = ((*m).param_count)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut param_vals: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    LLVMGetParams((*c_m).func, param_vals);
    gen_send_message(c, m, param_vals, params);
    LLVMBuildRet((*c).builder, *param_vals.offset(0 as libc::c_int as isize));
    codegen_finishfun(c);
    ponyint_pool_free_size(buf_size, param_vals as *mut libc::c_void);
    let mut msg_type_ptr: LLVMTypeRef =
        LLVMPointerType((*c_m).msg_type, 0 as libc::c_int as libc::c_uint);
    add_dispatch_case(
        c,
        t,
        (*m).params,
        (*m).vtable_index,
        (*c_m).func_handler,
        (*c_m).func_type,
        msg_type_ptr,
    );
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "608:1"]
unsafe extern "C" fn copy_subordinate(mut m: *mut reach_method_t) {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut m2: *mut reach_method_t = (*m).subordinate;
    while !m2.is_null() {
        let mut c_m2: *mut compile_method_t = (*m2).c_method as *mut compile_method_t;
        let ref mut fresh25 = (*c_m2).func_type;
        *fresh25 = (*c_m).func_type;
        let ref mut fresh26 = (*c_m2).func;
        *fresh26 = (*c_m).func;
        m2 = (*m2).subordinate;
    }
}
#[c2rust::src_loc = "622:1"]
unsafe extern "C" fn genfun_implicit_final_prototype(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let ref mut fresh27 = (*c_m).func_type;
    *fresh27 = LLVMFunctionType(
        (*c).void_type,
        &mut (*c_t).use_type,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh28 = (*c_m).func;
    *fresh28 = codegen_addfun(c, (*m).full_name, (*c_m).func_type, 1 as libc::c_int != 0);
    let ref mut fresh29 = (*c_t).final_fn;
    *fresh29 = (*c_m).func;
    LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
    LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
}
#[c2rust::src_loc = "636:1"]
unsafe extern "C" fn genfun_implicit_final(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    codegen_startfun(
        c,
        (*c_m).func,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    call_embed_finalisers(c, t, 0 as *mut ast_t, gen_this(c, 0 as *mut ast_t));
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "649:1"]
unsafe extern "C" fn genfun_allocator(mut c: *mut compile_t, mut t: *mut reach_type_t) -> bool {
    match (*t).underlying as libc::c_uint {
        74 | 75 | 76 | 77 => {}
        _ => return 1 as libc::c_int != 0,
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if !((*c_t).primitive).is_null()
        || is_pointer((*t).ast) as libc::c_int != 0
        || is_nullable_pointer((*t).ast) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    }
    let mut funname: *const libc::c_char = genname_alloc((*t).name);
    let mut ftype: LLVMTypeRef = LLVMFunctionType(
        (*c_t).use_type,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut fun: LLVMValueRef = codegen_addfun(c, funname, ftype, 1 as libc::c_int != 0);
    if (*t).underlying as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
        let mut elem: LLVMTypeRef = LLVMGetElementType((*c_t).use_type);
        let mut size: size_t = LLVMABISizeOfType((*c).target_data, elem) as size_t;
        let mut noalias_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
        let mut noalias_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
            b"noalias\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        noalias_attr =
            LLVMCreateEnumAttribute((*c).context, noalias_attr_id, 0 as libc::c_int as u64);
        let mut deref_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
        let mut deref_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
            b"dereferenceable\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        deref_attr = LLVMCreateEnumAttribute((*c).context, deref_attr_id, size as u64);
        let mut align_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
        let mut align_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
            b"align\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        align_attr = LLVMCreateEnumAttribute(
            (*c).context,
            align_attr_id,
            ((1 as libc::c_int) << 5 as libc::c_int) as u64,
        );
        LLVMAddAttributeAtIndex(
            fun,
            LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
            noalias_attr,
        );
        LLVMAddAttributeAtIndex(
            fun,
            LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
            deref_attr,
        );
        LLVMAddAttributeAtIndex(
            fun,
            LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
            align_attr,
        );
    }
    codegen_startfun(
        c,
        fun,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    let mut result: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    match (*t).underlying as libc::c_uint {
        74 | 75 | 76 => {
            result = gencall_alloc(c, t, 0 as *mut ast_t);
        }
        77 => {
            result = gencall_create(c, t, 0 as *mut ast_t);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                        as *const u8 as *const libc::c_char,
                    704 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"genfun_allocator\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    LLVMBuildRet((*c).builder, result);
    codegen_finishfun(c);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "713:1"]
unsafe extern "C" fn genfun_forward(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
) -> bool {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if !((*c_m).func).is_null() {
    } else {
        ponyint_assert_fail(
            b"c_m->func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            717 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"genfun_forward\0"))
                .as_ptr(),
        );
    };
    let mut m2: *mut reach_method_t = reach_method(t, (*m).cap, (*n).name, (*m).typeargs);
    if !m2.is_null() {
    } else {
        ponyint_assert_fail(
            b"m2 != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            720 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"genfun_forward\0"))
                .as_ptr(),
        );
    };
    if m2 != m {
    } else {
        ponyint_assert_fail(
            b"m2 != m\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                as *const u8 as *const libc::c_char,
            721 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"genfun_forward\0"))
                .as_ptr(),
        );
    };
    let mut c_m2: *mut compile_method_t = (*m2).c_method as *mut compile_method_t;
    codegen_startfun(
        c,
        (*c_m).func,
        (*c_m).di_file,
        (*c_m).di_method,
        (*m).fun,
        (*m).cap as libc::c_uint == TK_AT as libc::c_int as libc::c_uint,
    );
    let mut count: libc::c_int = LLVMCountParams((*c_m).func) as libc::c_int;
    let mut buf_size: size_t = (count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut args: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    let ref mut fresh30 = *args.offset(0 as libc::c_int as isize);
    *fresh30 = LLVMGetParam((*c_m).func, 0 as libc::c_int as libc::c_uint);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count {
        let mut value: LLVMValueRef = LLVMGetParam((*c_m).func, i as libc::c_uint);
        let ref mut fresh31 = *args.offset(i as isize);
        *fresh31 = gen_assign_cast(
            c,
            (*((*(*((*m2).params).offset((i - 1 as libc::c_int) as isize)).type_0).c_type
                as *mut compile_type_t))
                .use_type,
            value,
            (*(*((*m).params).offset((i - 1 as libc::c_int) as isize)).type_0).ast_cap,
        );
        i += 1;
    }
    codegen_debugloc(c, (*(*m2).fun).ast);
    let mut ret: LLVMValueRef = codegen_call(
        c,
        (*c_m2).func,
        args,
        count as size_t,
        (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint,
    );
    codegen_debugloc(c, 0 as *mut ast_t);
    ret = gen_assign_cast(
        c,
        (*((*(*m).result).c_type as *mut compile_type_t)).use_type,
        ret,
        (*(*m2).result).ast_cap,
    );
    LLVMBuildRet((*c).builder, ret);
    codegen_finishfun(c);
    ponyint_pool_free_size(buf_size, args as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "752:1"]
unsafe extern "C" fn genfun_method(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut n: *mut reach_method_name_t,
    mut m: *mut reach_method_t,
) -> bool {
    if (*m).intrinsic {
        if (*m).internal as libc::c_int != 0 && (*n).name == (*c).str__final {
            if !genfun_implicit_final(c, t, m) {
                return 0 as libc::c_int != 0;
            }
        }
    } else if (*m).forwarding {
        if !genfun_forward(c, t, n, m) {
            return 0 as libc::c_int != 0;
        }
    } else {
        match ast_id((*(*m).fun).ast) as libc::c_uint {
            88 => {
                if (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
                    if !genfun_newbe(c, t, m) {
                        return 0 as libc::c_int != 0;
                    }
                } else if !genfun_new(c, t, m) {
                    return 0 as libc::c_int != 0;
                }
            }
            90 => {
                if !genfun_be(c, t, m) {
                    return 0 as libc::c_int != 0;
                }
            }
            89 => {
                if !genfun_fun(c, t, m) {
                    return 0 as libc::c_int != 0;
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                            as *const u8 as *const libc::c_char,
                        790 as libc::c_int as size_t,
                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            b"genfun_method\0",
                        ))
                        .as_ptr(),
                    );
                };
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "798:1"]
pub unsafe extern "C" fn genfun_param_attrs(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut fun: LLVMValueRef,
) {
    let mut noalias_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut noalias_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"noalias\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    noalias_attr =
        LLVMCreateEnumAttribute((*c).context, noalias_attr_id, 0 as libc::c_int as u64);
    let mut readonly_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut readonly_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"readonly\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    readonly_attr =
        LLVMCreateEnumAttribute((*c).context, readonly_attr_id, 0 as libc::c_int as u64);
    let mut param: LLVMValueRef = LLVMGetFirstParam(fun);
    let mut type_0: *mut reach_type_t = t;
    let mut cap: token_id = (*m).cap;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 1 as libc::c_int;
    if cap as libc::c_uint == TK_AT as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int;
        offset = 0 as libc::c_int;
    }
    while !param.is_null() {
        let mut m_type: LLVMTypeRef = LLVMTypeOf(param);
        if LLVMGetTypeKind(m_type) as libc::c_uint
            == LLVMPointerTypeKind as libc::c_int as libc::c_uint
        {
            if i > 0 as libc::c_int {
                type_0 = (*((*m).params).offset((i - 1 as libc::c_int) as isize)).type_0;
                cap = (*((*m).params).offset((i - 1 as libc::c_int) as isize)).cap;
            } else if ast_id((*(*m).fun).ast) as libc::c_uint
                == TK_NEW as libc::c_int as libc::c_uint
            {
                param = LLVMGetNextParam(param);
                i += 1;
                continue;
            }
            if (*type_0).underlying as libc::c_uint != TK_ACTOR as libc::c_int as libc::c_uint {
                match cap as libc::c_uint {
                    91 => {
                        LLVMAddAttributeAtIndex(
                            fun,
                            (i + offset) as LLVMAttributeIndex,
                            noalias_attr,
                        );
                    }
                    92 | 93 => {}
                    94 | 96 | 95 => {
                        LLVMAddAttributeAtIndex(
                            fun,
                            (i + offset) as LLVMAttributeIndex,
                            readonly_attr,
                        );
                    }
                    _ => {
                        if 0 as libc::c_int != 0 {
                        } else {
                            ponyint_assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                                    as *const u8 as *const libc::c_char,
                                850 as libc::c_int as size_t,
                                (*::core::mem::transmute::<
                                    &[u8; 19],
                                    &[libc::c_char; 19],
                                >(b"genfun_param_attrs\0"))
                                    .as_ptr(),
                            );
                        };
                    }
                }
            }
        }
        param = LLVMGetNextParam(param);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "860:1"]
pub unsafe extern "C" fn genfun_allocate_compile_methods(
    mut _c: *mut compile_t,
    mut t: *mut reach_type_t,
) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
    loop {
        n = reach_method_names_next(&mut (*t).methods, &mut i);
        if n.is_null() {
            break;
        }
        let mut j: size_t = -(1 as libc::c_int) as size_t;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            let mut c_m: *mut compile_method_t =
                ponyint_pool_alloc(1 as libc::c_int as size_t) as *mut compile_method_t;
            memset(
                c_m as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<compile_method_t>() as libc::c_ulong,
            );
            let ref mut fresh32 = (*c_m).free_fn;
            *fresh32 = Some(compile_method_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
            let ref mut fresh33 = (*m).c_method;
            *fresh33 = c_m as *mut compile_opaque_t;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "882:1"]
pub unsafe extern "C" fn genfun_method_sigs(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> bool {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
    loop {
        n = reach_method_names_next(&mut (*t).methods, &mut i);
        if n.is_null() {
            break;
        }
        let mut j: size_t = -(1 as libc::c_int) as size_t;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            if (*m).internal as libc::c_int != 0 && (*n).name == (*c).str__final {
                genfun_implicit_final_prototype(c, t, m);
            } else {
                make_prototype(c, t, n, m);
            }
            copy_subordinate(m);
        }
    }
    if !genfun_allocator(c, t) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "909:1"]
pub unsafe extern "C" fn genfun_method_bodies(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> bool {
    match (*t).underlying as libc::c_uint {
        74 | 75 | 76 | 77 => {}
        _ => return 1 as libc::c_int != 0,
    }
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut n: *mut reach_method_name_t = 0 as *mut reach_method_name_t;
    loop {
        n = reach_method_names_next(&mut (*t).methods, &mut i);
        if n.is_null() {
            break;
        }
        let mut j: size_t = -(1 as libc::c_int) as size_t;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            if !genfun_method(c, t, n, m) {
                if errors_get_count((*(*c).opt).check.errors) == 0 as libc::c_int as libc::c_ulong {
                    if !((*m).fun).is_null() {
                    } else {
                        ponyint_assert_fail(
                            b"m->fun != NULL\0" as *const u8 as *const libc::c_char,
                            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.c\0"
                                as *const u8 as *const libc::c_char,
                            937 as libc::c_int as size_t,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"genfun_method_bodies\0"))
                                .as_ptr(),
                        );
                    };
                    ast_error(
                        (*(*c).opt).check.errors,
                        (*(*m).fun).ast,
                        b"internal failure: code generation failed for method %s\0" as *const u8
                            as *const libc::c_char,
                        (*m).full_name,
                    );
                }
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "951:1"]
unsafe extern "C" fn need_primitive_call(
    mut c: *mut compile_t,
    mut method: *const libc::c_char,
) -> bool {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if (*t).underlying as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
            continue;
        }
        let mut n: *mut reach_method_name_t = reach_method_name(t, method);
        if n.is_null() {
            continue;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "972:1"]
unsafe extern "C" fn primitive_call(mut c: *mut compile_t, mut method: *const libc::c_char) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if (*t).underlying as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
            continue;
        }
        let mut m: *mut reach_method_t = reach_method(t, TK_NONE, method, 0 as *mut ast_t);
        if m.is_null() {
            continue;
        }
        let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
        let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
        let mut value: LLVMValueRef = codegen_call(
            c,
            (*c_m).func,
            &mut (*c_t).instance,
            1 as libc::c_int as size_t,
            1 as libc::c_int != 0,
        );
        if (*c).str__final == method {
            LLVMSetInstructionCallConv(value, LLVMCCallConv as libc::c_int as libc::c_uint);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "996:1"]
pub unsafe extern "C" fn genfun_primitive_calls(mut c: *mut compile_t) {
    let mut fn_type: LLVMTypeRef = 0 as LLVMTypeRef;
    if need_primitive_call(c, (*c).str__init) {
        fn_type = LLVMFunctionType(
            (*c).void_type,
            0 as *mut LLVMTypeRef,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        );
        let mut fn_name: *const libc::c_char = genname_program_fn(
            (*c).filename,
            b"primitives_init\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh34 = (*c).primitives_init;
        *fresh34 = LLVMAddFunction((*c).module, fn_name, fn_type);
        codegen_startfun(
            c,
            (*c).primitives_init,
            0 as LLVMMetadataRef,
            0 as LLVMMetadataRef,
            0 as *mut deferred_reification_t,
            0 as libc::c_int != 0,
        );
        primitive_call(c, (*c).str__init);
        LLVMBuildRetVoid((*c).builder);
        codegen_finishfun(c);
    }
    if need_primitive_call(c, (*c).str__final) {
        if fn_type.is_null() {
            fn_type = LLVMFunctionType(
                (*c).void_type,
                0 as *mut LLVMTypeRef,
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int,
            );
        }
        let mut fn_name_0: *const libc::c_char = genname_program_fn(
            (*c).filename,
            b"primitives_final\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh35 = (*c).primitives_final;
        *fresh35 = LLVMAddFunction((*c).module, fn_name_0, fn_type);
        codegen_startfun(
            c,
            (*c).primitives_final,
            0 as LLVMMetadataRef,
            0 as LLVMMetadataRef,
            0 as *mut deferred_reification_t,
            0 as libc::c_int != 0,
        );
        primitive_call(c, (*c).str__final);
        LLVMBuildRetVoid((*c).builder);
        codegen_finishfun(c);
    }
}
