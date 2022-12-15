use ::libc;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:1"]
pub mod _types_h {
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = libc::c_ulong;
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
    use super::Types_h::{
        LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef,
        LLVMValueRef,
    };
    extern "C" {
        #[c2rust::src_loc = "1023:1"]
        pub fn LLVMAddFunction(
            M: LLVMModuleRef,
            Name: *const libc::c_char,
            FunctionTy: LLVMTypeRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "1033:1"]
        pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const libc::c_char) -> LLVMValueRef;
        #[c2rust::src_loc = "1106:1"]
        pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
        #[c2rust::src_loc = "1246:1"]
        pub fn LLVMFunctionType(
            ReturnType: LLVMTypeRef,
            ParamTypes: *mut LLVMTypeRef,
            ParamCount: libc::c_uint,
            IsVarArg: LLVMBool,
        ) -> LLVMTypeRef;
        #[c2rust::src_loc = "1253:1"]
        pub fn LLVMIsFunctionVarArg(FunctionTy: LLVMTypeRef) -> LLVMBool;
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
        #[c2rust::src_loc = "1338:1"]
        pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> libc::c_uint;
        #[c2rust::src_loc = "1350:1"]
        pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
        #[c2rust::src_loc = "1400:1"]
        pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
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
        #[c2rust::src_loc = "2182:1"]
        pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2236:1"]
        pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
        #[c2rust::src_loc = "2357:1"]
        pub fn LLVMGetNamedGlobal(M: LLVMModuleRef, Name: *const libc::c_char) -> LLVMValueRef;
        #[c2rust::src_loc = "2582:1"]
        pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "2900:1"]
        pub fn LLVMMDNodeInContext(
            C: LLVMContextRef,
            Vals: *mut LLVMValueRef,
            Count: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3068:1"]
        pub fn LLVMInsertBasicBlockInContext(
            C: LLVMContextRef,
            BB: LLVMBasicBlockRef,
            Name: *const libc::c_char,
        ) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3112:1"]
        pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3297:1"]
        pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3702:1"]
        pub fn LLVMBuildUnreachable(_: LLVMBuilderRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3897:1"]
        pub fn LLVMBuildStore(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            Ptr: LLVMValueRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3953:1"]
        pub fn LLVMBuildPtrToInt(
            _: LLVMBuilderRef,
            Val: LLVMValueRef,
            DestTy: LLVMTypeRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3955:1"]
        pub fn LLVMBuildIntToPtr(
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
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "80:1"]
        pub fn ast_canerror(ast: *mut ast_t) -> bool;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "100:1"]
        pub fn ast_type(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "111:1"]
        pub fn ast_parent(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: usize) -> *mut ast_t;
        #[c2rust::src_loc = "114:1"]
        pub fn ast_childlast(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ast_childcount(ast: *mut ast_t) -> usize;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "147:1"]
        pub fn ast_free_unattached(ast: *mut ast_t);
        #[c2rust::src_loc = "159:1"]
        pub fn ast_error(errors: *mut errors_t, ast: *mut ast_t, fmt: *const libc::c_char, _: ...);
        #[c2rust::src_loc = "161:1"]
        pub fn ast_error_continue(
            errors: *mut errors_t,
            ast: *mut ast_t,
            fmt: *const libc::c_char,
            _: ...
        );
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
        #[c2rust::src_loc = "24:65"]
        pub fn reach_type_cache_next(
            map: *mut reach_type_cache_t,
            i: *mut usize,
        ) -> *mut reach_type_t;
        #[c2rust::src_loc = "136:1"]
        pub fn reach_type(r: *mut reach_t, type_0: *mut ast_t) -> *mut reach_type_t;
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
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMHasMetadataStr(val: LLVMValueRef, str: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMSetMetadataStr(val: LLVMValueRef, str: *const libc::c_char, node: LLVMValueRef);
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMBuildStructGEP_P(
            B: LLVMBuilderRef,
            Pointer: LLVMValueRef,
            Idx: libc::c_uint,
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
        #[c2rust::src_loc = "45:1"]
        pub fn LLVMBuildInvoke_P(
            B: LLVMBuilderRef,
            Fn: LLVMValueRef,
            Args: *mut LLVMValueRef,
            NumArgs: libc::c_uint,
            Then: LLVMBasicBlockRef,
            Catch: LLVMBasicBlockRef,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "50:1"]
        pub fn LLVMMemcpy(module: LLVMModuleRef, ilp32: bool) -> LLVMValueRef;
        #[c2rust::src_loc = "51:1"]
        pub fn LLVMMemmove(module: LLVMModuleRef, ilp32: bool) -> LLVMValueRef;
        #[c2rust::src_loc = "52:1"]
        pub fn LLVMLifetimeStart(module: LLVMModuleRef, type_0: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "53:1"]
        pub fn LLVMLifetimeEnd(module: LLVMModuleRef, type_0: LLVMTypeRef) -> LLVMValueRef;
        #[c2rust::src_loc = "291:1"]
        pub fn codegen_call(
            c: *mut compile_t,
            fun: LLVMValueRef,
            args: *mut LLVMValueRef,
            count: usize,
            setcc: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "283:1"]
        pub fn codegen_ctx(c: *mut compile_t) -> LLVMValueRef;
        #[c2rust::src_loc = "277:1"]
        pub fn codegen_debugloc(c: *mut compile_t, ast: *mut ast_t);
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
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genoperator.h:2"]
pub mod genoperator_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gen_add(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "11:1"]
        pub fn gen_sub(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "13:1"]
        pub fn gen_mul(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "15:1"]
        pub fn gen_div(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "17:1"]
        pub fn gen_rem(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "19:1"]
        pub fn gen_neg(c: *mut compile_t, ast: *mut ast_t, safe: bool) -> LLVMValueRef;
        #[c2rust::src_loc = "21:1"]
        pub fn gen_shl(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "23:1"]
        pub fn gen_shr(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "25:1"]
        pub fn gen_and_sc(c: *mut compile_t, left: *mut ast_t, right: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "27:1"]
        pub fn gen_or_sc(c: *mut compile_t, left: *mut ast_t, right: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "29:1"]
        pub fn gen_and(c: *mut compile_t, left: *mut ast_t, right: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "31:1"]
        pub fn gen_or(c: *mut compile_t, left: *mut ast_t, right: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "33:1"]
        pub fn gen_xor(c: *mut compile_t, left: *mut ast_t, right: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "35:1"]
        pub fn gen_not(c: *mut compile_t, ast: *mut ast_t) -> LLVMValueRef;
        #[c2rust::src_loc = "37:1"]
        pub fn gen_lt(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "39:1"]
        pub fn gen_le(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "41:1"]
        pub fn gen_ge(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "43:1"]
        pub fn gen_gt(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "45:1"]
        pub fn gen_eq(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "47:1"]
        pub fn gen_eq_rvalue(
            c: *mut compile_t,
            left: *mut ast_t,
            r_value: LLVMValueRef,
            safe: bool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "50:1"]
        pub fn gen_ne(
            c: *mut compile_t,
            left: *mut ast_t,
            right: *mut ast_t,
            safe: bool,
        ) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.h:3"]
pub mod genreference_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn gen_fieldptr(c: *mut compile_t, ast: *mut ast_t) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:4"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.h:5"]
pub mod gendesc_h {
    use super::codegen_h::compile_t;
    use super::Types_h::LLVMValueRef;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "17:1"]
        pub fn gendesc_fetch(c: *mut compile_t, object: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "27:1"]
        pub fn gendesc_vtable(c: *mut compile_t, desc: LLVMValueRef, colour: usize)
            -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:8"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "26:1"]
        pub fn target_is_native128(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.h:9"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/platformfuns.h:10"]
pub mod platformfuns_h {
    use super::pass_h::pass_opt_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn os_is_target(
            attribute: *const libc::c_char,
            release: bool,
            out_is_target: *mut bool,
            options: *mut pass_opt_t,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/cap.h:11"]
pub mod cap_h {
    use super::symtab_h::ast_t;
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn cap_dispatch(type_0: *mut ast_t) -> token_id;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:12"]
pub mod subtype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn is_pointer(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "34:1"]
        pub fn is_nullable_pointer(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "36:1"]
        pub fn is_none(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "42:1"]
        pub fn is_bool(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "48:1"]
        pub fn is_machine_word(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pass/expr.h:14"]
pub mod expr_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn is_result_needed(ast: *mut ast_t) -> bool;
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
        #[c2rust::src_loc = "30:1"]
        pub fn ponyint_pool_realloc_size(
            old_size: usize,
            new_size: usize,
            p: *mut libc::c_void,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "34:1"]
        pub fn ponyint_pool_index(size: usize) -> usize;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/heap.h:16"]
pub mod heap_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ponyint_heap_index(size: usize) -> u32;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:17"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:18"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "84:6"]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;
    }
}
pub use self::ast_h::{
    ast_canerror, ast_child, ast_childcount, ast_childidx, ast_childlast, ast_data, ast_error,
    ast_error_continue, ast_free_unattached, ast_get_children, ast_id, ast_name, ast_parent,
    ast_ptr_t, ast_sibling, ast_type,
};
use self::cap_h::cap_dispatch;
pub use self::codegen_h::{
    codegen_call, codegen_ctx, codegen_debugloc, compile_frame_t, compile_locals_t, compile_t,
    ffi_decls_t, genned_strings_t, LLVMBuildCall_P, LLVMBuildInvoke_P, LLVMBuildStructGEP_P,
    LLVMHasMetadataStr, LLVMLifetimeEnd, LLVMLifetimeStart, LLVMMemcpy, LLVMMemmove,
    LLVMSetMetadataStr,
};

use self::expr_h::is_result_needed;
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
use self::gendesc_h::{gendesc_fetch, gendesc_vtable};
use self::genexpr_h::{gen_assign_cast, gen_expr};
pub use self::genfun_h::compile_method_t;
use self::genoperator_h::{
    gen_add, gen_and, gen_and_sc, gen_div, gen_eq, gen_eq_rvalue, gen_ge, gen_gt, gen_le, gen_lt,
    gen_mul, gen_ne, gen_neg, gen_not, gen_or, gen_or_sc, gen_rem, gen_shl, gen_shr, gen_sub,
    gen_xor,
};
use self::genopt_h::{target_is_ilp32, target_is_native128};
use self::genreference_h::gen_fieldptr;
use self::gentrace_h::{gentrace, gentrace_needed};
pub use self::gentype_h::compile_type_t;
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
use self::heap_h::ponyint_heap_index;
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::platformfuns_h::os_is_target;
use self::ponyassert_h::ponyint_assert_fail;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
    ponyint_pool_index, ponyint_pool_realloc_size,
};
pub use self::reach_h::{
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_mangled_next, reach_mangled_t,
    reach_method, reach_method_name, reach_method_name_t, reach_method_names_t,
    reach_method_stack_t, reach_method_t, reach_methods_t, reach_param_t, reach_t, reach_type,
    reach_type_cache_next, reach_type_cache_t, reach_type_t, reach_types_t,
};
pub use self::reify_h::{deferred_reification_t, deferred_reify};
use self::string_h::strncmp;

use self::subtype_h::{is_bool, is_machine_word, is_none, is_nullable_pointer, is_pointer};
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
    LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddFunction, LLVMAnyRegCallConv,
    LLVMAppendingLinkage, LLVMArrayTypeKind, LLVMAvailableExternallyLinkage, LLVMBFloatTypeKind,
    LLVMBuildBitCast, LLVMBuildIntToPtr, LLVMBuildPtrToInt, LLVMBuildStore, LLVMBuildUnreachable,
    LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv, LLVMColdCallConv, LLVMCommonLinkage,
    LLVMConstBitCast, LLVMConstInt, LLVMConstNull, LLVMCountParamTypes,
    LLVMCountStructElementTypes, LLVMDLLExportLinkage, LLVMDLLImportLinkage, LLVMDoubleTypeKind,
    LLVMExternalLinkage, LLVMExternalWeakLinkage, LLVMFP128TypeKind, LLVMFastCallConv,
    LLVMFloatTypeKind, LLVMFunctionType, LLVMFunctionTypeKind, LLVMGHCCallConv, LLVMGetElementType,
    LLVMGetInsertBlock, LLVMGetNamedFunction, LLVMGetNamedGlobal, LLVMGetParamTypes,
    LLVMGetReturnType, LLVMGetStructElementTypes, LLVMGetTypeKind, LLVMGhostLinkage,
    LLVMHHVMCCallConv, LLVMHHVMCallConv, LLVMHalfTypeKind, LLVMHiPECallConv,
    LLVMInsertBasicBlockInContext, LLVMIntegerTypeKind, LLVMIntelOCLBICallConv,
    LLVMInternalLinkage, LLVMIsFunctionVarArg, LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage,
    LLVMLinkOnceODRAutoHideLinkage, LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage,
    LLVMLinkerPrivateWeakLinkage, LLVMMDNodeInContext, LLVMMSP430BUILTINCallConv,
    LLVMMSP430INTRCallConv, LLVMMetadataTypeKind, LLVMMoveBasicBlockAfter, LLVMPPC_FP128TypeKind,
    LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv, LLVMPointerType, LLVMPointerTypeKind,
    LLVMPositionBuilderAtEnd, LLVMPreserveAllCallConv, LLVMPreserveMostCallConv,
    LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv, LLVMScalableVectorTypeKind,
    LLVMSetFunctionCallConv, LLVMSetInstructionCallConv, LLVMSetLinkage, LLVMStructTypeInContext,
    LLVMStructTypeKind, LLVMSwiftCallConv, LLVMTokenTypeKind, LLVMTypeKind, LLVMTypeOf,
    LLVMVectorTypeKind, LLVMVoidTypeKind, LLVMWeakAnyLinkage, LLVMWeakODRLinkage,
    LLVMWebKitJSCallConv, LLVMWin64CallConv, LLVMX8664SysVCallConv, LLVMX86FastcallCallConv,
    LLVMX86INTRCallConv, LLVMX86RegCallCallConv, LLVMX86StdcallCallConv, LLVMX86ThisCallCallConv,
    LLVMX86VectorCallCallConv, LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind, LLVMX86_MMXTypeKind,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{LLVMABISizeOfType, LLVMOpaqueTargetData, LLVMTargetDataRef};
pub use self::Types_h::{
    LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    LLVMModuleRef, LLVMOpaqueBasicBlock, LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder,
    LLVMOpaqueMetadata, LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef,
    LLVMValueRef,
};
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uintptr_t_h::uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:8"]
pub struct ffi_decl_t {
    pub func: LLVMValueRef,
    pub decl: *mut ast_t,
    pub call: *mut ast_t,
}
#[c2rust::src_loc = "85:1"]
pub type ffi_decls_free_fn = Option<unsafe extern "C" fn(*mut ffi_decl_t) -> ()>;
#[c2rust::src_loc = "85:1"]
pub type ffi_decls_cmp_fn = Option<unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "20:16"]
pub struct call_tuple_indices_t {
    pub data: *mut usize,
    pub count: usize,
    pub alloc: usize,
}
#[c2rust::src_loc = "27:1"]
unsafe extern "C" fn tuple_indices_init(mut ti: *mut call_tuple_indices_t) {
    let ref mut fresh0 = (*ti).data;
    *fresh0 = ponyint_pool_alloc_size(
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<usize>() as libc::c_ulong),
    ) as *mut usize;
    (*ti).count = 0 as libc::c_int as usize;
    (*ti).alloc = 4 as libc::c_int as usize;
}
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn tuple_indices_destroy(mut ti: *mut call_tuple_indices_t) {
    ponyint_pool_free_size(
        ((*ti).alloc).wrapping_mul(::core::mem::size_of::<usize>() as libc::c_ulong),
        (*ti).data as *mut libc::c_void,
    );
    let ref mut fresh1 = (*ti).data;
    *fresh1 = 0 as *mut usize;
    (*ti).count = 0 as libc::c_int as usize;
    (*ti).alloc = 0 as libc::c_int as usize;
}
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn tuple_indices_push(mut ti: *mut call_tuple_indices_t, mut idx: usize) {
    if (*ti).count == (*ti).alloc {
        let mut old_alloc: usize =
            ((*ti).alloc).wrapping_mul(::core::mem::size_of::<usize>() as libc::c_ulong);
        let ref mut fresh2 = (*ti).data;
        *fresh2 = ponyint_pool_realloc_size(
            old_alloc,
            old_alloc.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            (*ti).data as *mut libc::c_void,
        ) as *mut usize;
        let ref mut fresh3 = (*ti).alloc;
        *fresh3 = (*fresh3 as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as usize as usize;
    }
    let ref mut fresh4 = (*ti).count;
    let fresh5 = *fresh4;
    *fresh4 = (*fresh4).wrapping_add(1);
    *((*ti).data).offset(fresh5 as isize) = idx;
}
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn tuple_indices_pop(mut ti: *mut call_tuple_indices_t) -> usize {
    if (*ti).count > 0 {
    } else {
        ponyint_assert_fail(
            b"ti->count > 0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            56 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"tuple_indices_pop\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh6 = (*ti).count;
    *fresh6 = (*fresh6).wrapping_sub(1);
    return *((*ti).data).offset(*fresh6 as isize);
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn ffi_decl_hash(mut d: *mut ffi_decl_t) -> usize {
    return ponyint_hash_ptr((*d).func as *const libc::c_void);
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn ffi_decl_cmp(mut a: *mut ffi_decl_t, mut b: *mut ffi_decl_t) -> bool {
    return (*a).func == (*b).func;
}
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn ffi_decl_free(mut d: *mut ffi_decl_t) {
    ponyint_pool_free(0 as libc::c_int as usize, d as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    return ponyint_hashmap_fill_ratio(map);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_clearindex(mut map: *mut ffi_decls_t, mut index: usize) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_optimize(mut map: *mut ffi_decls_t) {
    let mut cmpf: ffi_decls_cmp_fn =
        Some(ffi_decl_cmp as unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool);
    return ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ffi_decls_cmp_fn, cmp_fn>(cmpf),
    );
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_alloc_size(mut map: *mut ffi_decls_t) -> usize {
    return ponyint_hashmap_alloc_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_init(mut map: *mut ffi_decls_t, mut size: usize) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_mem_size(mut map: *mut ffi_decls_t) -> usize {
    return ponyint_hashmap_mem_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_putindex(
    mut map: *mut ffi_decls_t,
    mut entry: *mut ffi_decl_t,
    mut index: usize,
) {
    let mut cmpf: ffi_decls_cmp_fn =
        Some(ffi_decl_cmp as unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool);
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        ffi_decl_hash(entry),
        ::core::mem::transmute::<ffi_decls_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_destroy(mut map: *mut ffi_decls_t) {
    let mut freef: ffi_decls_free_fn =
        Some(ffi_decl_free as unsafe extern "C" fn(*mut ffi_decl_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<ffi_decls_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_removeindex(mut map: *mut ffi_decls_t, mut index: usize) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn ffi_decls_size(mut map: *mut ffi_decls_t) -> usize {
    return ponyint_hashmap_size(map as *mut hashmap_t);
}
#[no_mangle]
#[c2rust::src_loc = "85:40"]
pub unsafe extern "C" fn ffi_decls_remove(
    mut map: *mut ffi_decls_t,
    mut entry: *mut ffi_decl_t,
) -> *mut ffi_decl_t {
    let mut cmpf: ffi_decls_cmp_fn =
        Some(ffi_decl_cmp as unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool);
    return ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        ffi_decl_hash(entry),
        ::core::mem::transmute::<ffi_decls_cmp_fn, cmp_fn>(cmpf),
    ) as *mut ffi_decl_t;
}
#[no_mangle]
#[c2rust::src_loc = "85:40"]
pub unsafe extern "C" fn ffi_decls_put(
    mut map: *mut ffi_decls_t,
    mut entry: *mut ffi_decl_t,
) -> *mut ffi_decl_t {
    let mut cmpf: ffi_decls_cmp_fn =
        Some(ffi_decl_cmp as unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool);
    return ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        ffi_decl_hash(entry),
        ::core::mem::transmute::<ffi_decls_cmp_fn, cmp_fn>(cmpf),
    ) as *mut ffi_decl_t;
}
#[no_mangle]
#[c2rust::src_loc = "85:40"]
pub unsafe extern "C" fn ffi_decls_next(
    mut map: *mut ffi_decls_t,
    mut i: *mut usize,
) -> *mut ffi_decl_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    return ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut ffi_decl_t;
}
#[no_mangle]
#[c2rust::src_loc = "85:40"]
pub unsafe extern "C" fn ffi_decls_get(
    mut map: *mut ffi_decls_t,
    mut key: *mut ffi_decl_t,
    mut index: *mut usize,
) -> *mut ffi_decl_t {
    let mut cmpf: ffi_decls_cmp_fn =
        Some(ffi_decl_cmp as unsafe extern "C" fn(*mut ffi_decl_t, *mut ffi_decl_t) -> bool);
    return ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        ffi_decl_hash(key),
        ::core::mem::transmute::<ffi_decls_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut ffi_decl_t;
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn invoke_fun(
    mut c: *mut compile_t,
    mut fun: LLVMValueRef,
    mut args: *mut LLVMValueRef,
    mut count: libc::c_int,
    mut ret: *const libc::c_char,
    mut setcc: bool,
) -> LLVMValueRef {
    if fun.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut this_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
    let mut then_block: LLVMBasicBlockRef = LLVMInsertBasicBlockInContext(
        (*c).context,
        this_block,
        b"invoke\0" as *const u8 as *const libc::c_char,
    );
    LLVMMoveBasicBlockAfter(then_block, this_block);
    let mut else_block: LLVMBasicBlockRef = (*(*c).frame).invoke_target;
    let mut invoke: LLVMValueRef = LLVMBuildInvoke_P(
        (*c).builder,
        fun,
        args,
        count as libc::c_uint,
        then_block,
        else_block,
        ret,
    );
    if setcc {
        LLVMSetInstructionCallConv(invoke, (*c).callconv as libc::c_uint);
    }
    LLVMPositionBuilderAtEnd((*c).builder, then_block);
    return invoke;
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn special_case_operator(
    mut c: *mut compile_t,
    mut ast: *mut ast_t,
    mut value: *mut LLVMValueRef,
    mut short_circuit: bool,
    mut native128: bool,
) -> bool {
    let mut postfix: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut postfix,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut left: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut left, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        postfix,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut right: *mut ast_t = ast_child(positional);
    let mut name: *const libc::c_char = ast_name(method);
    let mut special_case: bool = 1 as libc::c_int != 0;
    *value = 0 as LLVMValueRef;
    codegen_debugloc(c, ast);
    if name == (*c).str_add {
        *value = gen_add(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_sub {
        *value = gen_sub(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_mul && native128 as libc::c_int != 0 {
        *value = gen_mul(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_div && native128 as libc::c_int != 0 {
        *value = gen_div(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_rem && native128 as libc::c_int != 0 {
        *value = gen_rem(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_neg {
        *value = gen_neg(c, left, 1 as libc::c_int != 0);
    } else if name == (*c).str_add_unsafe {
        *value = gen_add(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_sub_unsafe {
        *value = gen_sub(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_mul_unsafe && native128 as libc::c_int != 0 {
        *value = gen_mul(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_div_unsafe && native128 as libc::c_int != 0 {
        *value = gen_div(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_rem_unsafe && native128 as libc::c_int != 0 {
        *value = gen_rem(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_neg_unsafe {
        *value = gen_neg(c, left, 0 as libc::c_int != 0);
    } else if name == (*c).str_and && short_circuit as libc::c_int != 0 {
        *value = gen_and_sc(c, left, right);
    } else if name == (*c).str_or && short_circuit as libc::c_int != 0 {
        *value = gen_or_sc(c, left, right);
    } else if name == (*c).str_and && !short_circuit {
        *value = gen_and(c, left, right);
    } else if name == (*c).str_or && !short_circuit {
        *value = gen_or(c, left, right);
    } else if name == (*c).str_xor {
        *value = gen_xor(c, left, right);
    } else if name == (*c).str_not {
        *value = gen_not(c, left);
    } else if name == (*c).str_shl {
        *value = gen_shl(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_shr {
        *value = gen_shr(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_shl_unsafe {
        *value = gen_shl(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_shr_unsafe {
        *value = gen_shr(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_eq {
        *value = gen_eq(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_ne {
        *value = gen_ne(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_lt {
        *value = gen_lt(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_le {
        *value = gen_le(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_ge {
        *value = gen_ge(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_gt {
        *value = gen_gt(c, left, right, 1 as libc::c_int != 0);
    } else if name == (*c).str_eq_unsafe {
        *value = gen_eq(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_ne_unsafe {
        *value = gen_ne(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_lt_unsafe {
        *value = gen_lt(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_le_unsafe {
        *value = gen_le(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_ge_unsafe {
        *value = gen_ge(c, left, right, 0 as libc::c_int != 0);
    } else if name == (*c).str_gt_unsafe {
        *value = gen_gt(c, left, right, 0 as libc::c_int != 0);
    } else {
        special_case = 0 as libc::c_int != 0;
    }
    codegen_debugloc(c, 0 as *mut ast_t);
    return special_case;
}
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn special_case_platform(
    mut c: *mut compile_t,
    mut ast: *mut ast_t,
) -> LLVMValueRef {
    let mut postfix: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut postfix,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        postfix,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut method_name: *const libc::c_char = ast_name(method);
    let mut is_target: bool = false;
    if os_is_target(method_name, (*(*c).opt).release, &mut is_target, (*c).opt) {
        return LLVMConstInt(
            (*c).i1,
            (if is_target as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_ulonglong,
            0 as libc::c_int,
        );
    }
    ast_error(
        (*(*c).opt).check.errors,
        ast,
        b"unknown Platform setting\0" as *const u8 as *const libc::c_char,
    );
    return 0 as LLVMValueRef;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn special_case_call(
    mut c: *mut compile_t,
    mut ast: *mut ast_t,
    mut value: *mut LLVMValueRef,
) -> bool {
    let mut postfix: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut postfix,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    if ast_id(postfix) as libc::c_uint != TK_FUNREF as libc::c_int as libc::c_uint
        || ast_id(named) as libc::c_uint != TK_NONE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        postfix,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut receiver_type: *mut ast_t =
        deferred_reify((*(*c).frame).reify, ast_type(receiver), (*c).opt);
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if ast_id(receiver_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
        let mut package: ast_ptr_t = 0 as *mut ast_t;
        let mut id: ast_ptr_t = 0 as *mut ast_t;
        let mut children_1: [*mut *mut ast_t; 3] = [&mut package, &mut id, 0 as *mut *mut ast_t];
        ast_get_children(
            receiver_type,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1),
            children_1.as_mut_ptr(),
        );
        if ast_name(package) == (*c).str_builtin {
            name = ast_name(id);
        }
    }
    ast_free_unattached(receiver_type);
    if name.is_null() {
        return 0 as libc::c_int != 0;
    }
    if name == (*c).str_Bool {
        return special_case_operator(c, ast, value, 1 as libc::c_int != 0, 1 as libc::c_int != 0);
    }
    if name == (*c).str_I8
        || name == (*c).str_I16
        || name == (*c).str_I32
        || name == (*c).str_I64
        || name == (*c).str_ILong
        || name == (*c).str_ISize
        || name == (*c).str_U8
        || name == (*c).str_U16
        || name == (*c).str_U32
        || name == (*c).str_U64
        || name == (*c).str_ULong
        || name == (*c).str_USize
        || name == (*c).str_F32
        || name == (*c).str_F64
    {
        return special_case_operator(c, ast, value, 0 as libc::c_int != 0, 1 as libc::c_int != 0);
    }
    if name == (*c).str_I128 || name == (*c).str_U128 {
        let mut native128: bool = target_is_native128((*(*c).opt).triple);
        return special_case_operator(c, ast, value, 0 as libc::c_int != 0, native128);
    }
    if name == (*c).str_Platform {
        *value = special_case_platform(c, ast);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "276:1"]
unsafe extern "C" fn dispatch_function(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut l_value: LLVMValueRef,
) -> LLVMValueRef {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    if (*t).bare_method == m {
        return LLVMBuildBitCast(
            (*c).builder,
            l_value,
            LLVMPointerType((*c_m).func_type, 0 as libc::c_int as libc::c_uint),
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    match (*t).underlying as libc::c_uint {
        149 | 56 | 72 | 73 => {
            if ((*t).bare_method).is_null() {
            } else {
                ponyint_assert_fail(
                    b"t->bare_method == NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                        as *const u8 as *const libc::c_char,
                    292 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"dispatch_function\0",
                    ))
                    .as_ptr(),
                );
            };
            let mut func: LLVMValueRef =
                gendesc_vtable(c, gendesc_fetch(c, l_value), (*m).vtable_index as usize);
            return LLVMBuildBitCast(
                (*c).builder,
                func,
                LLVMPointerType((*c_m).func_type, 0 as libc::c_int as libc::c_uint),
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        74 | 75 | 76 | 77 => return (*c_m).func,
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            314 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dispatch_function\0"))
                .as_ptr(),
        );
    };
    return 0 as LLVMValueRef;
}
#[c2rust::src_loc = "318:1"]
unsafe extern "C" fn call_needs_receiver(
    mut postfix: *mut ast_t,
    mut t: *mut reach_type_t,
) -> bool {
    match ast_id(postfix) as libc::c_uint {
        188 | 189 => {
            if !((*((*t).c_type as *mut compile_type_t)).primitive).is_null() {
                return 0 as libc::c_int != 0;
            }
            if is_pointer((*t).ast) as libc::c_int != 0
                || is_nullable_pointer((*t).ast) as libc::c_int != 0
            {
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        _ => return 1 as libc::c_int != 0,
    };
}
#[c2rust::src_loc = "341:1"]
unsafe extern "C" fn set_descriptor(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut value: LLVMValueRef,
) {
    if (*t).underlying as libc::c_uint == TK_STRUCT as libc::c_int as libc::c_uint {
        return;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut desc_ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        value,
        0 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildStore((*c).builder, (*c_t).desc, desc_ptr);
}
#[c2rust::src_loc = "356:1"]
unsafe extern "C" fn make_tuple_indices(
    mut ti: *mut call_tuple_indices_t,
    mut ast: *mut ast_t,
) -> *mut ast_t {
    let mut current: *mut ast_t = ast;
    let mut parent: *mut ast_t = ast_parent(current);
    while !parent.is_null()
        && ast_id(parent) as libc::c_uint != TK_ASSIGN as libc::c_int as libc::c_uint
        && ast_id(parent) as libc::c_uint != TK_CALL as libc::c_int as libc::c_uint
    {
        if ast_id(parent) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
            let mut index: usize = 0;
            let mut child: *mut ast_t = ast_child(parent);
            while current != child {
                index = index.wrapping_add(1);
                child = ast_sibling(child);
            }
            tuple_indices_push(ti, index);
        }
        current = parent;
        parent = ast_parent(current);
    }
    return parent;
}
#[c2rust::src_loc = "381:1"]
unsafe extern "C" fn find_embed_constructor_receiver(mut call: *mut ast_t) -> *mut ast_t {
    let mut tuple_indices: call_tuple_indices_t = {
        let mut init = call_tuple_indices_t {
            data: 0 as *mut usize,
            count: 0 as libc::c_int as usize,
            alloc: 4 as libc::c_int as usize,
        };
        init
    };
    tuple_indices_init(&mut tuple_indices);
    let mut parent: *mut ast_t = make_tuple_indices(&mut tuple_indices, call);
    let mut fieldref: *mut ast_t = 0 as *mut ast_t;
    if !parent.is_null()
        && ast_id(parent) as libc::c_uint == TK_ASSIGN as libc::c_int as libc::c_uint
    {
        let mut current: *mut ast_t = ast_child(parent);
        while ast_id(current) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint
            || ast_id(current) as libc::c_uint == TK_SEQ as libc::c_int as libc::c_uint
        {
            parent = current;
            if ast_id(current) as libc::c_uint == TK_TUPLE as libc::c_int as libc::c_uint {
                if tuple_indices.count == 0 {
                    break;
                }
                let mut index: usize = tuple_indices_pop(&mut tuple_indices);
                current = ast_childidx(parent, index);
            } else {
                current = ast_childlast(parent);
            }
        }
        if ast_id(current) as libc::c_uint == TK_EMBEDREF as libc::c_int as libc::c_uint {
            fieldref = current;
        }
    }
    tuple_indices_destroy(&mut tuple_indices);
    return fieldref;
}
#[c2rust::src_loc = "420:1"]
unsafe extern "C" fn gen_constructor_receiver(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut call: *mut ast_t,
) -> LLVMValueRef {
    let mut fieldref: *mut ast_t = find_embed_constructor_receiver(call);
    if !fieldref.is_null() {
        let mut receiver: LLVMValueRef = gen_fieldptr(c, fieldref);
        set_descriptor(c, t, receiver);
        return receiver;
    } else {
        return gencall_alloc(c, t, call);
    };
}
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn set_method_external_interface(
    mut t: *mut reach_type_t,
    mut name: *const libc::c_char,
    mut vtable_index: u32,
) {
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut sub: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        sub = reach_type_cache_next(&mut (*t).subtypes, &mut i);
        if sub.is_null() {
            break;
        }
        let mut n: *mut reach_method_name_t = reach_method_name(sub, name);
        if n.is_null() {
            continue;
        }
        let mut j: usize = -(1 as libc::c_int) as usize;
        let mut m: *mut reach_method_t = 0 as *mut reach_method_t;
        loop {
            m = reach_mangled_next(&mut (*n).r_mangled, &mut j);
            if m.is_null() {
                break;
            }
            if !((*m).vtable_index == vtable_index) {
                continue;
            }
            let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
            LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
            LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "464:1"]
pub unsafe extern "C" fn gen_funptr(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    if ast_id(ast) as libc::c_uint == TK_FUNREF as libc::c_int as libc::c_uint
        || ast_id(ast) as libc::c_uint == TK_BEREF as libc::c_int as libc::c_uint
    {
    } else {
        ponyint_assert_fail(
            b"(ast_id(ast) == TK_FUNREF) || (ast_id(ast) == TK_BEREF)\0" as *const u8
                as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            466 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"gen_funptr\0")).as_ptr(),
        );
    };
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut typeargs: *mut ast_t = 0 as *mut ast_t;
    match ast_id(receiver) as libc::c_uint {
        190 | 191 => {
            typeargs = method;
            let mut children_0: [*mut *mut ast_t; 3] =
                [&mut receiver, &mut method, 0 as *mut *mut ast_t];
            ast_get_children(
                receiver,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children_0.as_mut_ptr(),
            );
        }
        _ => {}
    }
    let mut value: LLVMValueRef = gen_expr(c, receiver);
    let mut type_0: *mut ast_t = deferred_reify((*(*c).frame).reify, ast_type(receiver), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            488 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"gen_funptr\0")).as_ptr(),
        );
    };
    let mut name: *const libc::c_char = ast_name(method);
    let mut cap: token_id = cap_dispatch(type_0);
    let mut m: *mut reach_method_t = reach_method(t, cap, name, typeargs);
    let mut funptr: LLVMValueRef = dispatch_function(c, t, m, value);
    ast_free_unattached(type_0);
    if (*m).cap as libc::c_uint != TK_AT as libc::c_int as libc::c_uint
        && (*c).linkage as libc::c_uint != LLVMExternalLinkage as libc::c_int as libc::c_uint
    {
        match (*t).underlying as libc::c_uint {
            74 | 75 | 76 | 77 => {
                let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
                LLVMSetFunctionCallConv((*c_m).func, LLVMCCallConv as libc::c_int as libc::c_uint);
                LLVMSetLinkage((*c_m).func, LLVMExternalLinkage);
            }
            149 | 56 | 72 | 73 => {
                set_method_external_interface(t, name, (*m).vtable_index);
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                            as *const u8 as *const libc::c_char,
                        522 as libc::c_int as usize,
                        (*::core::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"gen_funptr\0"))
                            .as_ptr(),
                    );
                };
            }
        }
    }
    return funptr;
}
#[no_mangle]
#[c2rust::src_loc = "530:1"]
pub unsafe extern "C" fn gen_send_message(
    mut c: *mut compile_t,
    mut m: *mut reach_method_t,
    mut args: *mut LLVMValueRef,
    mut args_ast: *mut ast_t,
) {
    let mut c_m: *mut compile_method_t = (*m).c_method as *mut compile_method_t;
    let mut msg_size: usize = LLVMABISizeOfType((*c).target_data, (*c_m).msg_type) as usize;
    let mut msg_type_ptr: LLVMTypeRef =
        LLVMPointerType((*c_m).msg_type, 0 as libc::c_int as libc::c_uint);
    let mut params_buf_size: usize = ((*m).param_count)
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
    let mut param_types: *mut LLVMTypeRef =
        ponyint_pool_alloc_size(params_buf_size) as *mut LLVMTypeRef;
    LLVMGetStructElementTypes((*c_m).msg_type, param_types);
    let mut args_buf_size: usize = ((*m).param_count)
        .wrapping_add(1)
        .wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut cast_args: *mut LLVMValueRef =
        ponyint_pool_alloc_size(args_buf_size) as *mut LLVMValueRef;
    let mut arg_types_buf_size: usize =
        ((*m).param_count).wrapping_mul(::core::mem::size_of::<*mut ast_t>() as libc::c_ulong);
    let mut arg_types: *mut *mut ast_t =
        ponyint_pool_alloc_size(arg_types_buf_size) as *mut *mut ast_t;
    let mut arg_ast: *mut ast_t = ast_child(args_ast);
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    let mut i: usize = 0;
    while i < (*m).param_count {
        let ref mut fresh7 = *arg_types.offset(i as isize);
        *fresh7 = deferred_reify(reify, ast_type(arg_ast), (*c).opt);
        let ref mut fresh8 =
            *cast_args.offset(i.wrapping_add(1) as isize);
        *fresh8 = gen_assign_cast(
            c,
            *param_types.offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize),
            *args.offset(i.wrapping_add(1) as isize),
            *arg_types.offset(i as isize),
        );
        arg_ast = ast_sibling(arg_ast);
        i = i.wrapping_add(1);
    }
    let mut msg_args: [LLVMValueRef; 5] = [0 as *mut LLVMOpaqueValue; 5];
    msg_args[0 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        ponyint_pool_index(msg_size) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    msg_args[1 as libc::c_int as usize] = LLVMConstInt(
        (*c).i32_0,
        (*m).vtable_index as libc::c_ulonglong,
        0 as libc::c_int,
    );
    let mut msg: LLVMValueRef = gencall_runtime(
        c,
        b"pony_alloc_msg\0" as *const u8 as *const libc::c_char,
        msg_args.as_mut_ptr(),
        2 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut md: LLVMValueRef = LLVMMDNodeInContext(
        (*c).context,
        0 as *mut LLVMValueRef,
        0 as libc::c_int as libc::c_uint,
    );
    LLVMSetMetadataStr(
        msg,
        b"pony.msgsend\0" as *const u8 as *const libc::c_char,
        md,
    );
    let mut msg_ptr: LLVMValueRef = LLVMBuildBitCast(
        (*c).builder,
        msg,
        msg_type_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i_0 as libc::c_ulong) < (*m).param_count {
        let mut arg_ptr: LLVMValueRef = LLVMBuildStructGEP_P(
            (*c).builder,
            msg_ptr,
            i_0.wrapping_add(3 as libc::c_int as libc::c_uint),
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMBuildStore(
            (*c).builder,
            *cast_args.offset(i_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
            arg_ptr,
        );
        i_0 = i_0.wrapping_add(1);
    }
    let mut need_trace: bool = 0 as libc::c_int != 0;
    let mut i_1: usize = 0;
    while i_1 < (*m).param_count {
        if gentrace_needed(
            c,
            *arg_types.offset(i_1 as isize),
            (*((*m).params).offset(i_1 as isize)).ast,
        ) {
            need_trace = 1 as libc::c_int != 0;
            break;
        } else {
            i_1 = i_1.wrapping_add(1);
        }
    }
    let mut ctx: LLVMValueRef = codegen_ctx(c);
    if need_trace {
        let mut gc: LLVMValueRef = gencall_runtime(
            c,
            b"pony_gc_send\0" as *const u8 as *const libc::c_char,
            &mut ctx,
            1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMSetMetadataStr(
            gc,
            b"pony.msgsend\0" as *const u8 as *const libc::c_char,
            md,
        );
        let mut i_2: usize = 0;
        while i_2 < (*m).param_count {
            gentrace(
                c,
                ctx,
                *args.offset(i_2.wrapping_add(1) as isize),
                *cast_args.offset(i_2.wrapping_add(1) as isize),
                *arg_types.offset(i_2 as isize),
                (*((*m).params).offset(i_2 as isize)).ast,
            );
            i_2 = i_2.wrapping_add(1);
        }
        gc = gencall_runtime(
            c,
            b"pony_send_done\0" as *const u8 as *const libc::c_char,
            &mut ctx,
            1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMSetMetadataStr(
            gc,
            b"pony.msgsend\0" as *const u8 as *const libc::c_char,
            md,
        );
    }
    msg_args[0 as libc::c_int as usize] = ctx;
    msg_args[1 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        *args.offset(0 as libc::c_int as isize),
        (*c).object_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    msg_args[2 as libc::c_int as usize] = msg;
    msg_args[3 as libc::c_int as usize] = msg;
    msg_args[4 as libc::c_int as usize] = LLVMConstInt(
        (*c).i1,
        1,
        0 as libc::c_int,
    );
    let mut send: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if ast_id((*(*m).fun).ast) as libc::c_uint == TK_NEW as libc::c_int as libc::c_uint {
        send = gencall_runtime(
            c,
            b"pony_sendv_single\0" as *const u8 as *const libc::c_char,
            msg_args.as_mut_ptr(),
            5 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        send = gencall_runtime(
            c,
            b"pony_sendv\0" as *const u8 as *const libc::c_char,
            msg_args.as_mut_ptr(),
            5 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    LLVMSetMetadataStr(
        send,
        b"pony.msgsend\0" as *const u8 as *const libc::c_char,
        md,
    );
    ponyint_pool_free_size(params_buf_size, param_types as *mut libc::c_void);
    ponyint_pool_free_size(args_buf_size, cast_args as *mut libc::c_void);
    let mut i_3: usize = 0;
    while i_3 < (*m).param_count {
        ast_free_unattached(*arg_types.offset(i_3 as isize));
        i_3 = i_3.wrapping_add(1);
    }
    ponyint_pool_free_size(arg_types_buf_size, arg_types as *mut libc::c_void);
}
#[c2rust::src_loc = "628:1"]
unsafe extern "C" fn contains_boxable(mut type_0: *mut ast_t) -> bool {
    match ast_id(type_0) as libc::c_uint {
        150 => return 1 as libc::c_int != 0,
        151 => return is_machine_word(type_0),
        149 | 56 => {
            let mut child: *mut ast_t = ast_child(type_0);
            while !child.is_null() {
                if contains_boxable(type_0) {
                    return 1 as libc::c_int != 0;
                }
                child = ast_sibling(child);
            }
            return 0 as libc::c_int != 0;
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                        as *const u8 as *const libc::c_char,
                    654 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"contains_boxable\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    };
}
#[c2rust::src_loc = "659:1"]
unsafe extern "C" fn can_inline_message_send(
    mut t: *mut reach_type_t,
    mut m: *mut reach_method_t,
    mut method_name: *const libc::c_char,
) -> bool {
    match (*t).underlying as libc::c_uint {
        149 | 56 | 72 | 73 => {}
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                        as *const u8 as *const libc::c_char,
                    671 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"can_inline_message_send\0",
                    ))
                    .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    let mut i: usize = -(1 as libc::c_int) as usize;
    let mut sub: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        sub = reach_type_cache_next(&mut (*t).subtypes, &mut i);
        if sub.is_null() {
            break;
        }
        let mut m_sub: *mut reach_method_t =
            reach_method(sub, (*m).cap, method_name, (*m).typeargs);
        if m_sub.is_null() {
            continue;
        }
        match (*sub).underlying as libc::c_uint {
            76 | 74 => return 0 as libc::c_int != 0,
            77 => {
                if ast_id((*(*m_sub).fun).ast) as libc::c_uint
                    == TK_FUN as libc::c_int as libc::c_uint
                {
                    return 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        if (*m).param_count == (*m_sub).param_count {
        } else {
            ponyint_assert_fail(
                b"m->param_count == m_sub->param_count\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                    as *const u8 as *const libc::c_char,
                698 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"can_inline_message_send\0",
                ))
                .as_ptr(),
            );
        };
        let mut i_0: usize = 0;
        while i_0 < (*m).param_count {
            let mut param: *mut reach_type_t = (*((*m).params).offset(i_0 as isize)).type_0;
            let mut sub_param: *mut reach_type_t = (*((*m_sub).params).offset(i_0 as isize)).type_0;
            if (*param).can_be_boxed {
                if !(*sub_param).can_be_boxed {
                    return 0 as libc::c_int != 0;
                }
                if (*param).underlying as libc::c_uint
                    == TK_TUPLETYPE as libc::c_int as libc::c_uint
                {
                    let mut child: *mut ast_t = ast_child((*param).ast);
                    while !child.is_null() {
                        if contains_boxable(child) {
                            return 0 as libc::c_int != 0;
                        }
                    }
                }
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "729:1"]
pub unsafe extern "C" fn gen_call(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut special: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    if special_case_call(c, ast, &mut special) {
        return special;
    }
    let mut postfix: ast_ptr_t = 0 as *mut ast_t;
    let mut positional: ast_ptr_t = 0 as *mut ast_t;
    let mut named: ast_ptr_t = 0 as *mut ast_t;
    let mut question: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 5] = [
        &mut postfix,
        &mut positional,
        &mut named,
        &mut question,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut receiver: ast_ptr_t = 0 as *mut ast_t;
    let mut method: ast_ptr_t = 0 as *mut ast_t;
    let mut children_0: [*mut *mut ast_t; 3] = [&mut receiver, &mut method, 0 as *mut *mut ast_t];
    ast_get_children(
        postfix,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children_0.as_mut_ptr(),
    );
    let mut typeargs: *mut ast_t = 0 as *mut ast_t;
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    match ast_id(receiver) as libc::c_uint {
        188 | 189 | 190 | 191 | 203 | 204 => {
            typeargs = deferred_reify(reify, method, (*c).opt);
            let mut children_1: [*mut *mut ast_t; 3] =
                [&mut receiver, &mut method, 0 as *mut *mut ast_t];
            ast_get_children(
                receiver,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1),
                children_1.as_mut_ptr(),
            );
        }
        _ => {}
    }
    let mut method_name: *const libc::c_char = ast_name(method);
    let mut type_0: *mut ast_t = deferred_reify(reify, ast_type(receiver), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            763 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"gen_call\0")).as_ptr(),
        );
    };
    let mut cap: token_id = cap_dispatch(type_0);
    let mut m: *mut reach_method_t = reach_method(t, cap, method_name, typeargs);
    ast_free_unattached(type_0);
    ast_free_unattached(typeargs);
    let mut count: usize = ((*m).param_count).wrapping_add(1);
    let mut buf_size: usize =
        count.wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    let mut args: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    let mut arg: *mut ast_t = ast_child(positional);
    let mut i: libc::c_int = 1 as libc::c_int;
    while !arg.is_null() {
        let mut value: LLVMValueRef = gen_expr(c, arg);
        if value.is_null() {
            ponyint_pool_free_size(buf_size, args as *mut libc::c_void);
            return 0 as LLVMValueRef;
        }
        let ref mut fresh9 = *args.offset(i as isize);
        *fresh9 = value;
        arg = ast_sibling(arg);
        i += 1;
    }
    let mut is_new_call: bool = 0 as libc::c_int != 0;
    if call_needs_receiver(postfix, t) {
        match ast_id(postfix) as libc::c_uint {
            188 | 189 => {
                let ref mut fresh10 = *args.offset(0 as libc::c_int as isize);
                *fresh10 = gen_constructor_receiver(c, t, ast);
                is_new_call = 1 as libc::c_int != 0;
            }
            190 | 191 | 203 | 204 => {
                let ref mut fresh11 = *args.offset(0 as libc::c_int as isize);
                *fresh11 = gen_expr(c, receiver);
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    ponyint_assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                            as *const u8 as *const libc::c_char,
                        816 as libc::c_int as usize,
                        (*::core::mem::transmute::<
                            &[u8; 9],
                            &[libc::c_char; 9],
                        >(b"gen_call\0"))
                            .as_ptr(),
                    );
                };
                return 0 as LLVMValueRef;
            }
        }
    } else {
        let ref mut fresh12 = *args.offset(0 as libc::c_int as isize);
        *fresh12 = LLVMConstNull((*((*t).c_type as *mut compile_type_t)).use_type);
    }
    let mut func: LLVMValueRef =
        dispatch_function(c, t, m, *args.offset(0 as libc::c_int as isize));
    let mut is_message: bool = 0 as libc::c_int != 0;
    if ast_id(postfix) as libc::c_uint == TK_NEWBEREF as libc::c_int as libc::c_uint
        || ast_id(postfix) as libc::c_uint == TK_BEREF as libc::c_int as libc::c_uint
        || ast_id(postfix) as libc::c_uint == TK_BECHAIN as libc::c_int as libc::c_uint
    {
        match (*t).underlying as libc::c_uint {
            77 => {
                is_message = 1 as libc::c_int != 0;
            }
            149 | 56 | 72 | 73 => {
                if (*m).cap as libc::c_uint == TK_TAG as libc::c_int as libc::c_uint {
                    is_message = can_inline_message_send(t, m, method_name);
                }
            }
            _ => {}
        }
    }
    let mut bare: bool = (*m).cap as libc::c_uint == TK_AT as libc::c_int as libc::c_uint;
    let mut r: LLVMValueRef = 0 as LLVMValueRef;
    if is_message {
        codegen_debugloc(c, ast);
        gen_send_message(c, m, args, positional);
        codegen_debugloc(c, 0 as *mut ast_t);
        match ast_id(postfix) as libc::c_uint {
            188 | 189 => {
                r = *args.offset(0 as libc::c_int as isize);
            }
            _ => {
                r = (*c).none_instance;
            }
        }
    } else {
        let mut f_type: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf(func));
        let mut params: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
        LLVMGetParamTypes(
            f_type,
            params.offset(
                (if bare as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ),
        );
        if !bare {
            let mut func_receiver_type: LLVMTypeRef = *params.offset(0 as libc::c_int as isize);
            let mut call_receiver_type: LLVMTypeRef =
                LLVMTypeOf(*args.offset(0 as libc::c_int as isize));
            if func_receiver_type != call_receiver_type {
                let ref mut fresh13 = *args.offset(0 as libc::c_int as isize);
                *fresh13 = LLVMBuildBitCast(
                    (*c).builder,
                    *args.offset(0 as libc::c_int as isize),
                    func_receiver_type,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        arg = ast_child(positional);
        i = 1 as libc::c_int;
        while !arg.is_null() {
            let mut arg_type: *mut ast_t = deferred_reify(reify, ast_type(arg), (*c).opt);
            let ref mut fresh14 = *args.offset(i as isize);
            *fresh14 = gen_assign_cast(
                c,
                *params.offset(i as isize),
                *args.offset(i as isize),
                arg_type,
            );
            ast_free_unattached(arg_type);
            arg = ast_sibling(arg);
            i += 1;
        }
        let mut arg_offset: uintptr_t = 0 as libc::c_int as uintptr_t;
        if bare {
            arg_offset = 1 as libc::c_int as uintptr_t;
            i -= 1;
        }
        if !func.is_null() {
            codegen_debugloc(c, ast);
            if ast_canerror(ast) as libc::c_int != 0 && !((*(*c).frame).invoke_target).is_null() {
                r = invoke_fun(
                    c,
                    func,
                    args.offset(arg_offset as isize),
                    i,
                    b"\0" as *const u8 as *const libc::c_char,
                    !bare,
                );
            } else {
                r = codegen_call(c, func, args.offset(arg_offset as isize), i as usize, !bare);
            }
            if is_new_call {
                let mut md: LLVMValueRef = LLVMMDNodeInContext(
                    (*c).context,
                    0 as *mut LLVMValueRef,
                    0 as libc::c_int as libc::c_uint,
                );
                LLVMSetMetadataStr(r, b"pony.newcall\0" as *const u8 as *const libc::c_char, md);
            }
            codegen_debugloc(c, 0 as *mut ast_t);
            ponyint_pool_free_size(buf_size, params as *mut libc::c_void);
        }
    }
    if bare as libc::c_int != 0 && is_none((*(*m).result).ast) as libc::c_int != 0 {
        r = (*c).none_instance;
    }
    if (ast_id(postfix) as libc::c_uint == TK_NEWREF as libc::c_int as libc::c_uint
        || ast_id(postfix) as libc::c_uint == TK_NEWBEREF as libc::c_int as libc::c_uint)
        && (*t).underlying as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint
    {
        r = *args.offset(0 as libc::c_int as isize);
    }
    if ast_id(postfix) as libc::c_uint == TK_BECHAIN as libc::c_int as libc::c_uint
        || ast_id(postfix) as libc::c_uint == TK_FUNCHAIN as libc::c_int as libc::c_uint
    {
        r = *args.offset(0 as libc::c_int as isize);
    }
    ponyint_pool_free_size(buf_size, args as *mut libc::c_void);
    return r;
}
#[no_mangle]
#[c2rust::src_loc = "948:1"]
pub unsafe extern "C" fn gen_pattern_eq(
    mut c: *mut compile_t,
    mut pattern: *mut ast_t,
    mut r_value: LLVMValueRef,
) -> LLVMValueRef {
    let mut pattern_type: *mut ast_t =
        deferred_reify((*(*c).frame).reify, ast_type(pattern), (*c).opt);
    if ast_id(pattern_type) as libc::c_uint == TK_NOMINAL as libc::c_int as libc::c_uint {
        let mut package: ast_ptr_t = 0 as *mut ast_t;
        let mut id: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut package, &mut id, 0 as *mut *mut ast_t];
        ast_get_children(
            pattern_type,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1),
            children.as_mut_ptr(),
        );
        if ast_name(package) == (*c).str_builtin {
            let mut name: *const libc::c_char = ast_name(id);
            if name == (*c).str_Bool
                || name == (*c).str_I8
                || name == (*c).str_I16
                || name == (*c).str_I32
                || name == (*c).str_I64
                || name == (*c).str_I128
                || name == (*c).str_ILong
                || name == (*c).str_ISize
                || name == (*c).str_U8
                || name == (*c).str_U16
                || name == (*c).str_U32
                || name == (*c).str_U64
                || name == (*c).str_U128
                || name == (*c).str_ULong
                || name == (*c).str_USize
                || name == (*c).str_F32
                || name == (*c).str_F64
            {
                ast_free_unattached(pattern_type);
                return gen_eq_rvalue(c, pattern, r_value, 1 as libc::c_int != 0);
            }
        }
    }
    let mut l_value: LLVMValueRef = gen_expr(c, pattern);
    let mut t: *mut reach_type_t = reach_type((*c).reach, pattern_type);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            991 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"gen_pattern_eq\0"))
                .as_ptr(),
        );
    };
    let mut cap: token_id = cap_dispatch(pattern_type);
    let mut m: *mut reach_method_t = reach_method(t, cap, (*c).str_eq, 0 as *mut ast_t);
    let mut func: LLVMValueRef = dispatch_function(c, t, m, l_value);
    ast_free_unattached(pattern_type);
    if func.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] = l_value;
    args[1 as libc::c_int as usize] = r_value;
    codegen_debugloc(c, pattern);
    let mut result: LLVMValueRef = codegen_call(
        c,
        func,
        args.as_mut_ptr(),
        2 as libc::c_int as usize,
        1 as libc::c_int != 0,
    );
    codegen_debugloc(c, 0 as *mut ast_t);
    return result;
}
#[c2rust::src_loc = "1015:1"]
unsafe extern "C" fn ffi_return_type(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut intrinsic: bool,
) -> LLVMTypeRef {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if (*t).underlying as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
        if intrinsic as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"intrinsic\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                    as *const u8 as *const libc::c_char,
                1023 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"ffi_return_type\0"))
                    .as_ptr(),
            );
        };
        let mut count: libc::c_uint = LLVMCountStructElementTypes((*c_t).use_type);
        let mut buf_size: usize = (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
        let mut e_types: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
        LLVMGetStructElementTypes((*c_t).use_type, e_types);
        let mut child: *mut ast_t = ast_child((*t).ast);
        let mut i: usize = 0;
        while !child.is_null() {
            if is_bool(child) {
                let ref mut fresh15 = *e_types.offset(i as isize);
                *fresh15 = (*c).i1;
            }
            child = ast_sibling(child);
            i = i.wrapping_add(1);
        }
        let mut r_type: LLVMTypeRef =
            LLVMStructTypeInContext((*c).context, e_types, count, 0 as libc::c_int);
        ponyint_pool_free_size(buf_size, e_types as *mut libc::c_void);
        return r_type;
    } else if is_none((*t).ast_cap) {
        return (*c).void_type;
    } else {
        return (*c_t).use_type;
    };
}
#[c2rust::src_loc = "1055:1"]
unsafe extern "C" fn declare_ffi(
    mut c: *mut compile_t,
    mut f_name: *const libc::c_char,
    mut t: *mut reach_type_t,
    mut args: *mut ast_t,
    mut intrinsic: bool,
) -> LLVMValueRef {
    let mut is_varargs: bool = 0 as libc::c_int != 0;
    let mut last_arg: *mut ast_t = ast_childlast(args);
    let mut param_count: libc::c_int = ast_childcount(args) as libc::c_int;
    if !last_arg.is_null()
        && ast_id(last_arg) as libc::c_uint == TK_ELLIPSIS as libc::c_int as libc::c_uint
    {
        is_varargs = 1 as libc::c_int != 0;
        param_count -= 1;
    }
    let mut buf_size: usize = 0;
    let mut f_params: *mut LLVMTypeRef = 0 as *mut LLVMTypeRef;
    if param_count != 0 as libc::c_int {
        buf_size = (param_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
        f_params = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
        param_count = 0 as libc::c_int;
        let mut arg: *mut ast_t = ast_child(args);
        let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
        while !arg.is_null()
            && ast_id(arg) as libc::c_uint != TK_ELLIPSIS as libc::c_int as libc::c_uint
        {
            let mut p_type: *mut ast_t = ast_type(arg);
            if p_type.is_null() {
                p_type = ast_childidx(arg, 1 as libc::c_int as usize);
            }
            p_type = deferred_reify(reify, p_type, (*c).opt);
            let mut pt: *mut reach_type_t = reach_type((*c).reach, p_type);
            if !pt.is_null() {
            } else {
                ponyint_assert_fail(
                    b"pt != NULL\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                        as *const u8 as *const libc::c_char,
                    1090 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"declare_ffi\0"))
                        .as_ptr(),
                );
            };
            let fresh16 = param_count;
            param_count = param_count + 1;
            let ref mut fresh17 = *f_params.offset(fresh16 as isize);
            *fresh17 = (*((*pt).c_type as *mut compile_type_t)).use_type;
            ast_free_unattached(p_type);
            arg = ast_sibling(arg);
        }
    }
    let mut r_type: LLVMTypeRef = ffi_return_type(c, t, intrinsic);
    let mut f_type: LLVMTypeRef = LLVMFunctionType(
        r_type,
        f_params,
        param_count as libc::c_uint,
        is_varargs as LLVMBool,
    );
    let mut func: LLVMValueRef = LLVMAddFunction((*c).module, f_name, f_type);
    if !f_params.is_null() {
        ponyint_pool_free_size(buf_size, f_params as *mut libc::c_void);
    }
    return func;
}
#[c2rust::src_loc = "1107:1"]
unsafe extern "C" fn report_ffi_type_err(
    mut c: *mut compile_t,
    mut decl: *mut ffi_decl_t,
    mut ast: *mut ast_t,
    mut name: *const libc::c_char,
) {
    ast_error(
        (*(*c).opt).check.errors,
        ast,
        b"conflicting calls for FFI function: %s have incompatible types\0" as *const u8
            as *const libc::c_char,
        name,
    );
    if !decl.is_null() {
        ast_error_continue(
            (*(*c).opt).check.errors,
            (*decl).decl,
            b"first declaration is here\0" as *const u8 as *const libc::c_char,
        );
        ast_error_continue(
            (*(*c).opt).check.errors,
            (*decl).call,
            b"first call is here\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[c2rust::src_loc = "1122:1"]
unsafe extern "C" fn cast_ffi_arg(
    mut c: *mut compile_t,
    mut decl: *mut ffi_decl_t,
    mut ast: *mut ast_t,
    mut arg: LLVMValueRef,
    mut param: LLVMTypeRef,
    mut name: *const libc::c_char,
) -> LLVMValueRef {
    if arg.is_null() {
        return 0 as LLVMValueRef;
    }
    let mut arg_type: LLVMTypeRef = LLVMTypeOf(arg);
    if param == arg_type {
        return arg;
    }
    if LLVMABISizeOfType((*c).target_data, param) != LLVMABISizeOfType((*c).target_data, arg_type) {
        report_ffi_type_err(c, decl, ast, name);
        return 0 as LLVMValueRef;
    }
    match LLVMGetTypeKind(param) as libc::c_uint {
        12 => {
            if LLVMGetTypeKind(arg_type) as libc::c_uint
                == LLVMIntegerTypeKind as libc::c_int as libc::c_uint
            {
                return LLVMBuildIntToPtr(
                    (*c).builder,
                    arg,
                    param,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            } else {
                return LLVMBuildBitCast(
                    (*c).builder,
                    arg,
                    param,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        8 => {
            if LLVMGetTypeKind(arg_type) as libc::c_uint
                == LLVMPointerTypeKind as libc::c_int as libc::c_uint
            {
                return LLVMBuildPtrToInt(
                    (*c).builder,
                    arg,
                    param,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        10 => {
            if LLVMGetTypeKind(arg_type) as libc::c_uint
                == LLVMStructTypeKind as libc::c_int as libc::c_uint
            {
            } else {
                ponyint_assert_fail(
                    b"LLVMGetTypeKind(arg_type) == LLVMStructTypeKind\0" as *const u8
                        as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                        as *const u8 as *const libc::c_char,
                    1155 as libc::c_int as usize,
                    (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cast_ffi_arg\0"))
                        .as_ptr(),
                );
            };
            return arg;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"false\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            1161 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cast_ffi_arg\0")).as_ptr(),
        );
    };
    return 0 as LLVMValueRef;
}
#[no_mangle]
#[c2rust::src_loc = "1165:1"]
pub unsafe extern "C" fn gen_ffi(mut c: *mut compile_t, mut ast: *mut ast_t) -> LLVMValueRef {
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut typeargs: ast_ptr_t = 0 as *mut ast_t;
    let mut args: ast_ptr_t = 0 as *mut ast_t;
    let mut named_args: ast_ptr_t = 0 as *mut ast_t;
    let mut can_err: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 6] = [
        &mut id,
        &mut typeargs,
        &mut args,
        &mut named_args,
        &mut can_err,
        0 as *mut *mut ast_t,
    ];
    ast_get_children(
        ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1),
        children.as_mut_ptr(),
    );
    let mut err: bool =
        ast_id(can_err) as libc::c_uint == TK_QUESTION as libc::c_int as libc::c_uint;
    let mut f_name: *const libc::c_char = (ast_name(id)).offset(1 as libc::c_int as isize);
    let mut reify: *mut deferred_reification_t = (*(*c).frame).reify;
    let mut type_0: *mut ast_t = deferred_reify(reify, ast_type(ast), (*c).opt);
    let mut t: *mut reach_type_t = reach_type((*c).reach, type_0);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            1178 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"gen_ffi\0")).as_ptr(),
        );
    };
    ast_free_unattached(type_0);
    let mut ffi_decl: *mut ffi_decl_t = 0 as *mut ffi_decl_t;
    let mut is_func: bool = 0 as libc::c_int != 0;
    let mut func: LLVMValueRef = LLVMGetNamedGlobal((*c).module, f_name);
    if func.is_null() {
        func = LLVMGetNamedFunction((*c).module, f_name);
        is_func = 1 as libc::c_int != 0;
    }
    if func.is_null() {
        let mut decl: *mut ast_t = ast_data(ast) as *mut ast_t;
        if !decl.is_null() {
        } else {
            ponyint_assert_fail(
                b"decl != NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                    as *const u8 as *const libc::c_char,
                1197 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"gen_ffi\0")).as_ptr(),
            );
        };
        let mut is_intrinsic: bool = strncmp(
            f_name,
            b"llvm.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                f_name,
                b"internal.\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0;
        let mut decl_id: ast_ptr_t = 0 as *mut ast_t;
        let mut decl_ret: ast_ptr_t = 0 as *mut ast_t;
        let mut decl_params: ast_ptr_t = 0 as *mut ast_t;
        let mut decl_named_params: ast_ptr_t = 0 as *mut ast_t;
        let mut decl_err: ast_ptr_t = 0 as *mut ast_t;
        let mut children_0: [*mut *mut ast_t; 6] = [
            &mut decl_id,
            &mut decl_ret,
            &mut decl_params,
            &mut decl_named_params,
            &mut decl_err,
            0 as *mut *mut ast_t,
        ];
        ast_get_children(
            decl,
            (::core::mem::size_of::<[*mut *mut ast_t; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1),
            children_0.as_mut_ptr(),
        );
        err = ast_id(decl_err) as libc::c_uint == TK_QUESTION as libc::c_int as libc::c_uint;
        func = declare_ffi(c, f_name, t, decl_params, is_intrinsic);
        let mut index: usize = -(1 as libc::c_int) as usize;
        let mut k: ffi_decl_t = ffi_decl_t {
            func: 0 as *mut LLVMOpaqueValue,
            decl: 0 as *mut ast_t,
            call: 0 as *mut ast_t,
        };
        k.func = func;
        ffi_decl = ffi_decls_get(&mut (*c).ffi_decls, &mut k, &mut index);
        if ffi_decl.is_null() {
        } else {
            ponyint_assert_fail(
                b"ffi_decl == NULL\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                    as *const u8 as *const libc::c_char,
                1211 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"gen_ffi\0")).as_ptr(),
            );
        };
        ffi_decl = ponyint_pool_alloc(0 as libc::c_int as usize) as *mut ffi_decl_t;
        let ref mut fresh18 = (*ffi_decl).func;
        *fresh18 = func;
        let ref mut fresh19 = (*ffi_decl).decl;
        *fresh19 = decl;
        let ref mut fresh20 = (*ffi_decl).call;
        *fresh20 = ast;
        ffi_decls_putindex(&mut (*c).ffi_decls, ffi_decl, index);
    } else {
        let mut k_0: ffi_decl_t = ffi_decl_t {
            func: 0 as *mut LLVMOpaqueValue,
            decl: 0 as *mut ast_t,
            call: 0 as *mut ast_t,
        };
        k_0.func = func;
        let mut index_0: usize = -(1 as libc::c_int) as usize;
        ffi_decl = ffi_decls_get(&mut (*c).ffi_decls, &mut k_0, &mut index_0);
        if ffi_decl.is_null()
            && (!is_func
                || LLVMHasMetadataStr(func, b"pony.abi\0" as *const u8 as *const libc::c_char)
                    as libc::c_int
                    != 0)
        {
            ast_error(
                (*(*c).opt).check.errors,
                ast,
                b"cannot use '%s' as an FFI name: name is already in use by the internal ABI\0"
                    as *const u8 as *const libc::c_char,
                f_name,
            );
            return 0 as LLVMValueRef;
        }
        if is_func as libc::c_int != 0 {
        } else {
            ponyint_assert_fail(
                b"is_func\0" as *const u8 as *const libc::c_char,
                b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                    as *const u8 as *const libc::c_char,
                1234 as libc::c_int as usize,
                (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"gen_ffi\0")).as_ptr(),
            );
        };
    }
    let mut count: libc::c_int = ast_childcount(args) as libc::c_int;
    let mut buf_size: usize = (count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMValueRef>() as libc::c_ulong);
    let mut f_args: *mut LLVMValueRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMValueRef;
    let mut f_type: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf(func));
    let mut f_params: *mut LLVMTypeRef = 0 as *mut LLVMTypeRef;
    let mut vararg: bool = LLVMIsFunctionVarArg(f_type) != 0 as libc::c_int;
    if !vararg {
        if count != LLVMCountParamTypes(f_type) as libc::c_int {
            ast_error(
                (*(*c).opt).check.errors,
                ast,
                b"conflicting declarations for FFI function: declarations have an incompatible number of parameters\0"
                    as *const u8 as *const libc::c_char,
            );
            if !ffi_decl.is_null() {
                ast_error_continue(
                    (*(*c).opt).check.errors,
                    (*ffi_decl).decl,
                    b"first declaration is here\0" as *const u8 as *const libc::c_char,
                );
            }
            return 0 as LLVMValueRef;
        }
        f_params = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
        LLVMGetParamTypes(f_type, f_params);
    }
    let mut arg: *mut ast_t = ast_child(args);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let ref mut fresh21 = *f_args.offset(i as isize);
        *fresh21 = gen_expr(c, arg);
        if !vararg {
            let ref mut fresh22 = *f_args.offset(i as isize);
            *fresh22 = cast_ffi_arg(
                c,
                ffi_decl,
                ast,
                *f_args.offset(i as isize),
                *f_params.offset(i as isize),
                b"parameters\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*f_args.offset(i as isize)).is_null() {
            ponyint_pool_free_size(buf_size, f_args as *mut libc::c_void);
            return 0 as LLVMValueRef;
        }
        arg = ast_sibling(arg);
        i += 1;
    }
    let mut result: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    codegen_debugloc(c, ast);
    if err as libc::c_int != 0 && !((*(*c).frame).invoke_target).is_null() {
        result = invoke_fun(
            c,
            func,
            f_args,
            count,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
    } else {
        result = LLVMBuildCall_P(
            (*c).builder,
            func,
            f_args,
            count as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    codegen_debugloc(c, 0 as *mut ast_t);
    ponyint_pool_free_size(buf_size, f_args as *mut libc::c_void);
    if !vararg {
        ponyint_pool_free_size(buf_size, f_params as *mut libc::c_void);
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut isnone: bool = is_none((*t).ast);
    let mut isvoid: bool = LLVMGetReturnType(f_type) == (*c).void_type;
    if isnone as libc::c_int != 0 && isvoid as libc::c_int != 0 {
        result = (*c_t).instance;
    } else if isnone as libc::c_int != isvoid as libc::c_int {
        report_ffi_type_err(
            c,
            ffi_decl,
            ast,
            b"return values\0" as *const u8 as *const libc::c_char,
        );
        return 0 as LLVMValueRef;
    }
    result = cast_ffi_arg(
        c,
        ffi_decl,
        ast,
        result,
        (*c_t).use_type,
        b"return values\0" as *const u8 as *const libc::c_char,
    );
    result = gen_assign_cast(c, (*c_t).use_type, result, (*t).ast_cap);
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "1321:1"]
pub unsafe extern "C" fn gencall_runtime(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
    mut args: *mut LLVMValueRef,
    mut count: libc::c_int,
    mut ret: *const libc::c_char,
) -> LLVMValueRef {
    let mut func: LLVMValueRef = LLVMGetNamedFunction((*c).module, name);
    if !func.is_null() {
    } else {
        ponyint_assert_fail(
            b"func != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.c\0"
                as *const u8 as *const libc::c_char,
            1326 as libc::c_int as usize,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"gencall_runtime\0"))
                .as_ptr(),
        );
    };
    return LLVMBuildCall_P((*c).builder, func, args, count as libc::c_uint, ret);
}
#[no_mangle]
#[c2rust::src_loc = "1331:1"]
pub unsafe extern "C" fn gencall_create(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut call: *mut ast_t,
) -> LLVMValueRef {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut no_inc_rc: bool = !call.is_null() && !is_result_needed(call);
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = codegen_ctx(c);
    args[1 as libc::c_int as usize] = LLVMConstBitCast((*c_t).desc, (*c).descriptor_ptr);
    args[2 as libc::c_int as usize] = LLVMConstInt(
        (*c).i1,
        (if no_inc_rc as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulonglong,
        0 as libc::c_int,
    );
    let mut result: LLVMValueRef = gencall_runtime(
        c,
        b"pony_create\0" as *const u8 as *const libc::c_char,
        args.as_mut_ptr(),
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
    );
    return LLVMBuildBitCast(
        (*c).builder,
        result,
        (*c_t).use_type,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1350:1"]
pub unsafe extern "C" fn gencall_alloc(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut call: *mut ast_t,
) -> LLVMValueRef {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if !((*c_t).primitive).is_null() {
        return 0 as LLVMValueRef;
    }
    if is_pointer((*t).ast) as libc::c_int != 0 || is_nullable_pointer((*t).ast) as libc::c_int != 0
    {
        return 0 as LLVMValueRef;
    }
    if !((*c_t).instance).is_null() {
        return (*c_t).instance;
    }
    if (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
        return gencall_create(c, t, call);
    }
    return gencall_allocstruct(c, t);
}
#[no_mangle]
#[c2rust::src_loc = "1372:1"]
pub unsafe extern "C" fn gencall_allocstruct(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
) -> LLVMValueRef {
    let mut args: [LLVMValueRef; 3] = [0 as *mut LLVMOpaqueValue; 3];
    args[0 as libc::c_int as usize] = codegen_ctx(c);
    let mut result: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut size: usize = (*c_t).abi_size;
    if size == 0 {
        size = 1 as libc::c_int as usize;
    }
    if size <= ((1 as libc::c_int) << 10 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        let mut index: u32 = ponyint_heap_index(size);
        args[1 as libc::c_int as usize] =
            LLVMConstInt((*c).i32_0, index as libc::c_ulonglong, 0 as libc::c_int);
        if ((*c_t).final_fn).is_null() {
            result = gencall_runtime(
                c,
                b"pony_alloc_small\0" as *const u8 as *const libc::c_char,
                args.as_mut_ptr(),
                2 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = gencall_runtime(
                c,
                b"pony_alloc_small_final\0" as *const u8 as *const libc::c_char,
                args.as_mut_ptr(),
                2 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        args[1 as libc::c_int as usize] =
            LLVMConstInt((*c).intptr, size as libc::c_ulonglong, 0 as libc::c_int);
        if ((*c_t).final_fn).is_null() {
            result = gencall_runtime(
                c,
                b"pony_alloc_large\0" as *const u8 as *const libc::c_char,
                args.as_mut_ptr(),
                2 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            result = gencall_runtime(
                c,
                b"pony_alloc_large_final\0" as *const u8 as *const libc::c_char,
                args.as_mut_ptr(),
                2 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    result = LLVMBuildBitCast(
        (*c).builder,
        result,
        (*c_t).structure_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    set_descriptor(c, t, result);
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "1408:1"]
pub unsafe extern "C" fn gencall_error(mut c: *mut compile_t) {
    let mut func: LLVMValueRef = LLVMGetNamedFunction(
        (*c).module,
        b"pony_error\0" as *const u8 as *const libc::c_char,
    );
    if !((*(*c).frame).invoke_target).is_null() {
        invoke_fun(
            c,
            func,
            0 as *mut LLVMValueRef,
            0 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
        );
    } else {
        LLVMBuildCall_P(
            (*c).builder,
            func,
            0 as *mut LLVMValueRef,
            0 as libc::c_int as libc::c_uint,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    LLVMBuildUnreachable((*c).builder);
}
#[no_mangle]
#[c2rust::src_loc = "1420:1"]
pub unsafe extern "C" fn gencall_memcpy(
    mut c: *mut compile_t,
    mut dst: LLVMValueRef,
    mut src: LLVMValueRef,
    mut n: LLVMValueRef,
) {
    let mut func: LLVMValueRef = LLVMMemcpy((*c).module, target_is_ilp32((*(*c).opt).triple));
    let mut args: [LLVMValueRef; 4] = [0 as *mut LLVMOpaqueValue; 4];
    args[0 as libc::c_int as usize] = dst;
    args[1 as libc::c_int as usize] = src;
    args[2 as libc::c_int as usize] = n;
    args[3 as libc::c_int as usize] = LLVMConstInt(
        (*c).i1,
        0,
        0 as libc::c_int,
    );
    LLVMBuildCall_P(
        (*c).builder,
        func,
        args.as_mut_ptr(),
        4 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1433:1"]
pub unsafe extern "C" fn gencall_memmove(
    mut c: *mut compile_t,
    mut dst: LLVMValueRef,
    mut src: LLVMValueRef,
    mut n: LLVMValueRef,
) {
    let mut func: LLVMValueRef = LLVMMemmove((*c).module, target_is_ilp32((*(*c).opt).triple));
    let mut args: [LLVMValueRef; 5] = [0 as *mut LLVMOpaqueValue; 5];
    args[0 as libc::c_int as usize] = dst;
    args[1 as libc::c_int as usize] = src;
    args[2 as libc::c_int as usize] = n;
    args[3 as libc::c_int as usize] = LLVMConstInt(
        (*c).i1,
        0,
        0 as libc::c_int,
    );
    LLVMBuildCall_P(
        (*c).builder,
        func,
        args.as_mut_ptr(),
        4 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1446:1"]
pub unsafe extern "C" fn gencall_lifetime_start(mut c: *mut compile_t, mut ptr: LLVMValueRef) {
    let mut func: LLVMValueRef = LLVMLifetimeStart((*c).module, (*c).void_ptr);
    let mut type_0: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf(ptr));
    let mut size: usize = LLVMABISizeOfType((*c).target_data, type_0) as usize;
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] =
        LLVMConstInt((*c).i64_0, size as libc::c_ulonglong, 0 as libc::c_int);
    args[1 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        (*c).void_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCall_P(
        (*c).builder,
        func,
        args.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1458:1"]
pub unsafe extern "C" fn gencall_lifetime_end(mut c: *mut compile_t, mut ptr: LLVMValueRef) {
    let mut func: LLVMValueRef = LLVMLifetimeEnd((*c).module, (*c).void_ptr);
    let mut type_0: LLVMTypeRef = LLVMGetElementType(LLVMTypeOf(ptr));
    let mut size: usize = LLVMABISizeOfType((*c).target_data, type_0) as usize;
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
    args[0 as libc::c_int as usize] =
        LLVMConstInt((*c).i64_0, size as libc::c_ulonglong, 0 as libc::c_int);
    args[1 as libc::c_int as usize] = LLVMBuildBitCast(
        (*c).builder,
        ptr,
        (*c).void_ptr,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildCall_P(
        (*c).builder,
        func,
        args.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
