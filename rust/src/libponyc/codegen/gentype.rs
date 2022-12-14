use ::libc;
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
    pub type __darwin_off_t = i64;
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
        #[c2rust::src_loc = "1316:1"]
        pub fn LLVMStructCreateNamed(C: LLVMContextRef, Name: *const libc::c_char) -> LLVMTypeRef;
        #[c2rust::src_loc = "1330:1"]
        pub fn LLVMStructSetBody(
            StructTy: LLVMTypeRef,
            ElementTypes: *mut LLVMTypeRef,
            ElementCount: libc::c_uint,
            Packed: LLVMBool,
        );
        #[c2rust::src_loc = "1443:1"]
        pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: libc::c_uint)
            -> LLVMTypeRef;
        #[c2rust::src_loc = "2087:1"]
        pub fn LLVMConstNamedStruct(
            StructTy: LLVMTypeRef,
            ConstantVals: *mut LLVMValueRef,
            Count: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2236:1"]
        pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
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
        #[c2rust::src_loc = "2665:1"]
        pub fn LLVMGetParam(Fn: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3681:1"]
        pub fn LLVMBuildRetVoid(_: LLVMBuilderRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3688:1"]
        pub fn LLVMBuildSwitch(
            _: LLVMBuilderRef,
            V: LLVMValueRef,
            Else: LLVMBasicBlockRef,
            NumCases: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "3702:1"]
        pub fn LLVMBuildUnreachable(_: LLVMBuilderRef) -> LLVMValueRef;
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
        #[c2rust::src_loc = "261:1"]
        pub fn LLVMABIAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> libc::c_uint;
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
    #[c2rust::src_loc = "179:1"]
    pub type LLVMDWARFTypeEncoding = libc::c_uint;
    use super::Types_h::{LLVMDIBuilderRef, LLVMMetadataRef};
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "292:1"]
        pub fn LLVMDIBuilderCreateFile(
            Builder: LLVMDIBuilderRef,
            Filename: *const libc::c_char,
            FilenameLen: size_t,
            Directory: *const libc::c_char,
            DirectoryLen: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "698:1"]
        pub fn LLVMDIBuilderCreateBasicType(
            Builder: LLVMDIBuilderRef,
            Name: *const libc::c_char,
            NameLen: size_t,
            SizeInBits: u64,
            Encoding: LLVMDWARFTypeEncoding,
            Flags: LLVMDIFlags,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "714:1"]
        pub fn LLVMDIBuilderCreatePointerType(
            Builder: LLVMDIBuilderRef,
            PointeeTy: LLVMMetadataRef,
            SizeInBits: u64,
            AlignInBits: u32,
            AddressSpace: libc::c_uint,
            Name: *const libc::c_char,
            NameLen: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "737:1"]
        pub fn LLVMDIBuilderCreateStructType(
            Builder: LLVMDIBuilderRef,
            Scope: LLVMMetadataRef,
            Name: *const libc::c_char,
            NameLen: size_t,
            File: LLVMMetadataRef,
            LineNumber: libc::c_uint,
            SizeInBits: u64,
            AlignInBits: u32,
            Flags: LLVMDIFlags,
            DerivedFrom: LLVMMetadataRef,
            Elements: *mut LLVMMetadataRef,
            NumElements: libc::c_uint,
            RunTimeLang: libc::c_uint,
            VTableHolder: LLVMMetadataRef,
            UniqueId: *const libc::c_char,
            UniqueIdLen: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "759:1"]
        pub fn LLVMDIBuilderCreateMemberType(
            Builder: LLVMDIBuilderRef,
            Scope: LLVMMetadataRef,
            Name: *const libc::c_char,
            NameLen: size_t,
            File: LLVMMetadataRef,
            LineNo: libc::c_uint,
            SizeInBits: u64,
            AlignInBits: u32,
            OffsetInBits: u64,
            Flags: LLVMDIFlags,
            Ty: LLVMMetadataRef,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "1208:1"]
        pub fn LLVMMetadataReplaceAllUsesWith(
            TempTargetMetadata: LLVMMetadataRef,
            Replacement: LLVMMetadataRef,
        );
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendebug.h:1"]
pub mod gendebug_h {
    #[c2rust::src_loc = "10:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "15:3"]
    pub const DW_ATE_unsigned: C2RustUnnamed = 7;
    #[c2rust::src_loc = "14:3"]
    pub const DW_ATE_signed: C2RustUnnamed = 5;
    #[c2rust::src_loc = "13:3"]
    pub const DW_ATE_float: C2RustUnnamed = 4;
    #[c2rust::src_loc = "12:3"]
    pub const DW_ATE_boolean: C2RustUnnamed = 2;
    use super::Types_h::{LLVMDIBuilderRef, LLVMMetadataRef};
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn LLVMDIBuilderCreateReplaceableStruct(
            d: LLVMDIBuilderRef,
            name: *const libc::c_char,
            scope: LLVMMetadataRef,
            file: LLVMMetadataRef,
            line: libc::c_uint,
        ) -> LLVMMetadataRef;
    }
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
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    #[c2rust::src_loc = "187:1"]
    pub type ast_ptr_t = *mut ast_t;
    use super::_size_t_h::size_t;
    use super::source_h::source_t;
    use super::symtab_h::{ast_t, sym_status_t};
    use super::token_h::token_id;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn ast_id(ast: *mut ast_t) -> token_id;
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "76:1"]
        pub fn ast_source(ast: *mut ast_t) -> *mut source_t;
        #[c2rust::src_loc = "78:1"]
        pub fn ast_data(ast: *mut ast_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "94:1"]
        pub fn ast_name(ast: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "105:1"]
        pub fn ast_has_annotation(ast: *mut ast_t, name: *const libc::c_char) -> bool;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "113:1"]
        pub fn ast_childidx(ast: *mut ast_t, idx: size_t) -> *mut ast_t;
        #[c2rust::src_loc = "118:1"]
        pub fn ast_index(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
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
        #[c2rust::src_loc = "23:55"]
        pub fn reach_types_next(map: *mut reach_types_t, i: *mut size_t) -> *mut reach_type_t;
        #[c2rust::src_loc = "138:1"]
        pub fn reach_type_name(r: *mut reach_t, name: *const libc::c_char) -> *mut reach_type_t;
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
    extern "C" {
        #[c2rust::src_loc = "289:1"]
        pub fn codegen_block(c: *mut compile_t, name: *const libc::c_char) -> LLVMBasicBlockRef;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::_stdio_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "69:14"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "155:6"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "344:6"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.h:3"]
pub mod gendesc_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn gendesc_type(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "13:1"]
        pub fn gendesc_init(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "15:1"]
        pub fn gendesc_table(c: *mut compile_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexpr.h:4"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genfun.h:5"]
pub mod genfun_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn genfun_allocate_compile_methods(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "27:1"]
        pub fn genfun_method_sigs(c: *mut compile_t, t: *mut reach_type_t) -> bool;
        #[c2rust::src_loc = "29:1"]
        pub fn genfun_method_bodies(c: *mut compile_t, t: *mut reach_type_t) -> bool;
        #[c2rust::src_loc = "31:1"]
        pub fn genfun_primitive_calls(c: *mut compile_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genident.h:6"]
pub mod genident_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn gen_is_tuple_fun(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "15:1"]
        pub fn gen_numeric_size_table(c: *mut compile_t) -> LLVMValueRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:7"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn genname_dispatch(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "31:1"]
        pub fn genname_instance(type_0: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "37:1"]
        pub fn genname_box(name: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:8"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "15:1"]
        pub fn target_is_macosx(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "21:1"]
        pub fn target_is_ppc(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn target_is_lp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "24:1"]
        pub fn target_is_llp64(triple: *mut libc::c_char) -> bool;
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genprim.h:9"]
pub mod genprim_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "10:1"]
        pub fn genprim_pointer_methods(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "12:1"]
        pub fn genprim_nullable_pointer_methods(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "14:1"]
        pub fn genprim_donotoptimise_methods(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "16:1"]
        pub fn genprim_array_trace(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "30:1"]
        pub fn genprim_platform_methods(c: *mut compile_t, t: *mut reach_type_t);
        #[c2rust::src_loc = "32:1"]
        pub fn genprim_signature(c: *mut compile_t);
        #[c2rust::src_loc = "34:1"]
        pub fn genprim_builtins(c: *mut compile_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genreference.h:10"]
pub mod genreference_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn gen_digestof_fun(c: *mut compile_t, t: *mut reach_type_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genserialise.h:11"]
pub mod genserialise_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn genserialise(c: *mut compile_t, t: *mut reach_type_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentrace.h:12"]
pub mod gentrace_h {
    use super::codegen_h::compile_t;
    use super::reach_h::reach_type_t;
    use super::symtab_h::ast_t;
    use super::Types_h::LLVMValueRef;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn gentrace_prototype(c: *mut compile_t, t: *mut reach_type_t);
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/id.h:13"]
pub mod id_h {
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn is_name_private(name: *const libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/subtype.h:17"]
pub mod subtype_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn is_bool(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "44:1"]
        pub fn is_float(type_0: *mut ast_t) -> bool;
        #[c2rust::src_loc = "50:1"]
        pub fn is_signed(type_0: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:18"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/ponyassert.h:19"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::_types_h::{__darwin_size_t, __int64_t};
pub use self::ast_h::{
    ast_child, ast_childidx, ast_data, ast_get, ast_get_children, ast_has_annotation, ast_id,
    ast_index, ast_line, ast_name, ast_ptr_t, ast_source,
};
pub use self::codegen_h::{
    codegen_addfun, codegen_block, codegen_finishfun, codegen_startfun, compile_frame_t,
    compile_locals_t, compile_t, ffi_decls_t, genned_strings_t, LLVMBuildCall_P, LLVMBuildLoad_P,
    LLVMBuildStructGEP_P,
};

pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::gendebug_h::{
    C2RustUnnamed, DW_ATE_boolean, DW_ATE_float, DW_ATE_signed, DW_ATE_unsigned,
    LLVMDIBuilderCreateReplaceableStruct,
};
use self::gendesc_h::{gendesc_init, gendesc_table, gendesc_type};
use self::genexpr_h::gen_assign_cast;
use self::genfun_h::{
    genfun_allocate_compile_methods, genfun_method_bodies, genfun_method_sigs,
    genfun_primitive_calls,
};
use self::genident_h::{gen_is_tuple_fun, gen_numeric_size_table};
use self::genname_h::{genname_box, genname_dispatch, genname_instance};
use self::genopt_h::{
    target_is_ilp32, target_is_llp64, target_is_lp64, target_is_macosx, target_is_ppc,
};
use self::genprim_h::{
    genprim_array_trace, genprim_builtins, genprim_donotoptimise_methods,
    genprim_nullable_pointer_methods, genprim_platform_methods, genprim_pointer_methods,
    genprim_signature,
};
use self::genreference_h::gen_digestof_fun;
use self::genserialise_h::genserialise;
use self::gentrace_h::{gentrace, gentrace_prototype};
pub use self::gentype_h::compile_type_t;
pub use self::hash_h::{bitmap_t, hashmap_entry_t, hashmap_t};
use self::id_h::is_name_private;
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
    compile_opaque_free_fn, compile_opaque_t, reach_field_t, reach_method_names_t,
    reach_method_stack_t, reach_method_t, reach_param_t, reach_t, reach_type_cache_t,
    reach_type_name, reach_type_t, reach_types_next, reach_types_t,
};
pub use self::reify_h::deferred_reification_t;
pub use self::source_h::source_t;
use self::stdio_h::{__stderrp, fprintf, snprintf};
use self::string_h::{memset, strlen};

use self::subtype_h::{is_bool, is_float, is_signed};
pub use self::symtab_h::{
    ast_t, sym_status_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL,
    SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};
pub use self::sys__types_h::__darwin_off_t;
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
    LLVMAppendingLinkage, LLVMAvailableExternallyLinkage, LLVMBuildBitCast, LLVMBuildRetVoid,
    LLVMBuildSwitch, LLVMBuildUnreachable, LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv,
    LLVMColdCallConv, LLVMCommonLinkage, LLVMConstNamedStruct, LLVMDLLExportLinkage,
    LLVMDLLImportLinkage, LLVMExternalLinkage, LLVMExternalWeakLinkage, LLVMFastCallConv,
    LLVMGHCCallConv, LLVMGetParam, LLVMGhostLinkage, LLVMHHVMCCallConv, LLVMHHVMCallConv,
    LLVMHiPECallConv, LLVMIntelOCLBICallConv, LLVMInternalLinkage, LLVMLinkOnceAnyLinkage,
    LLVMLinkOnceODRAutoHideLinkage, LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage,
    LLVMLinkerPrivateWeakLinkage, LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv,
    LLVMPTXDeviceCallConv, LLVMPTXKernelCallConv, LLVMPointerType, LLVMPositionBuilderAtEnd,
    LLVMPreserveAllCallConv, LLVMPreserveMostCallConv, LLVMPrivateLinkage, LLVMSPIRFUNCCallConv,
    LLVMSPIRKERNELCallConv, LLVMSetFunctionCallConv, LLVMSetGlobalConstant, LLVMSetInitializer,
    LLVMSetLinkage, LLVMStructCreateNamed, LLVMStructSetBody, LLVMSwiftCallConv,
    LLVMWeakAnyLinkage, LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv,
    LLVMX8664SysVCallConv, LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv,
    LLVMX86StdcallCallConv, LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv,
};
pub use self::DebugInfo_h::{
    LLVMDIBuilderCreateBasicType, LLVMDIBuilderCreateFile, LLVMDIBuilderCreateMemberType,
    LLVMDIBuilderCreatePointerType, LLVMDIBuilderCreateStructType, LLVMDIFlagAccessibility,
    LLVMDIFlagAppleBlock, LLVMDIFlagArtificial, LLVMDIFlagBigEndian, LLVMDIFlagBitField,
    LLVMDIFlagEnumClass, LLVMDIFlagExplicit, LLVMDIFlagFixedEnum, LLVMDIFlagFwdDecl,
    LLVMDIFlagIndirectVirtualBase, LLVMDIFlagIntroducedVirtual, LLVMDIFlagLValueReference,
    LLVMDIFlagLittleEndian, LLVMDIFlagMultipleInheritance, LLVMDIFlagNoReturn,
    LLVMDIFlagNonTrivial, LLVMDIFlagObjcClassComplete, LLVMDIFlagObjectPointer, LLVMDIFlagPrivate,
    LLVMDIFlagProtected, LLVMDIFlagPrototyped, LLVMDIFlagPtrToMemberRep, LLVMDIFlagPublic,
    LLVMDIFlagRValueReference, LLVMDIFlagReserved, LLVMDIFlagReservedBit4,
    LLVMDIFlagSingleInheritance, LLVMDIFlagStaticMember, LLVMDIFlagThunk,
    LLVMDIFlagTypePassByReference, LLVMDIFlagTypePassByValue, LLVMDIFlagVector, LLVMDIFlagVirtual,
    LLVMDIFlagVirtualInheritance, LLVMDIFlagZero, LLVMDIFlags, LLVMDWARFTypeEncoding,
    LLVMMetadataReplaceAllUsesWith,
};
pub use self::TargetMachine_h::{LLVMOpaqueTargetMachine, LLVMTargetMachineRef};
pub use self::Target_h::{
    LLVMABIAlignmentOfType, LLVMABISizeOfType, LLVMOffsetOfElement, LLVMOpaqueTargetData,
    LLVMTargetDataRef,
};
pub use self::Types_h::{
    LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    LLVMModuleRef, LLVMOpaqueBasicBlock, LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder,
    LLVMOpaqueMetadata, LLVMOpaqueModule, LLVMOpaqueType, LLVMOpaqueValue, LLVMTypeRef,
    LLVMValueRef,
};
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
#[no_mangle]
#[c2rust::src_loc = "24:1"]
pub unsafe extern "C" fn get_fieldinfo(
    mut l_type: *mut ast_t,
    mut right: *mut ast_t,
    mut l_def: *mut *mut ast_t,
    mut field: *mut *mut ast_t,
    mut index: *mut u32,
) {
    let mut d: *mut ast_t = ast_data(l_type) as *mut ast_t;
    let mut f: *mut ast_t = ast_get(d, ast_name(right), 0 as *mut sym_status_t);
    let mut i: u32 = ast_index(f) as u32;
    *l_def = d;
    *field = f;
    *index = i;
}
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn compile_type_free(mut p: *mut libc::c_void) {
    ponyint_pool_free(3 as libc::c_int as size_t, p);
}
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn allocate_compile_types(mut c: *mut compile_t) {
    let mut i: size_t = -(1 as libc::c_int) as size_t;
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        let mut c_t: *mut compile_type_t =
            ponyint_pool_alloc(3 as libc::c_int as size_t) as *mut compile_type_t;
        memset(
            c_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<compile_type_t>() as libc::c_ulong,
        );
        let ref mut fresh0 = (*c_t).free_fn;
        *fresh0 = Some(compile_type_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        let ref mut fresh1 = (*t).c_type;
        *fresh1 = c_t as *mut compile_opaque_t;
        genfun_allocate_compile_methods(c, t);
    }
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn make_opaque_struct(mut c: *mut compile_t, mut t: *mut reach_type_t) -> bool {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    match ast_id((*t).ast) as libc::c_uint {
        151 => {
            match (*t).underlying as libc::c_uint {
                72 | 73 => {
                    let ref mut fresh2 = (*c_t).use_type;
                    *fresh2 = (*c).object_ptr;
                    let ref mut fresh3 = (*c_t).mem_type;
                    *fresh3 = (*c).object_ptr;
                    return 1 as libc::c_int != 0;
                }
                _ => {}
            }
            let mut pkg: ast_ptr_t = 0 as *mut ast_t;
            let mut id: ast_ptr_t = 0 as *mut ast_t;
            let mut children: [*mut *mut ast_t; 3] = [&mut pkg, &mut id, 0 as *mut *mut ast_t];
            ast_get_children(
                (*t).ast,
                (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                children.as_mut_ptr(),
            );
            let mut package: *const libc::c_char = ast_name(pkg);
            let mut name: *const libc::c_char = ast_name(id);
            let mut triple: *mut libc::c_char = (*(*c).opt).triple;
            let mut ilp32: bool = target_is_ilp32(triple);
            let mut llp64: bool = target_is_llp64(triple);
            let mut lp64: bool = target_is_lp64(triple);
            if package == (*c).str_builtin {
                if name == (*c).str_I8 {
                    let ref mut fresh4 = (*c_t).primitive;
                    *fresh4 = (*c).i8_0;
                } else if name == (*c).str_U8 {
                    let ref mut fresh5 = (*c_t).primitive;
                    *fresh5 = (*c).i8_0;
                } else if name == (*c).str_I16 {
                    let ref mut fresh6 = (*c_t).primitive;
                    *fresh6 = (*c).i16_0;
                } else if name == (*c).str_U16 {
                    let ref mut fresh7 = (*c_t).primitive;
                    *fresh7 = (*c).i16_0;
                } else if name == (*c).str_I32 {
                    let ref mut fresh8 = (*c_t).primitive;
                    *fresh8 = (*c).i32_0;
                } else if name == (*c).str_U32 {
                    let ref mut fresh9 = (*c_t).primitive;
                    *fresh9 = (*c).i32_0;
                } else if name == (*c).str_I64 {
                    let ref mut fresh10 = (*c_t).primitive;
                    *fresh10 = (*c).i64_0;
                } else if name == (*c).str_U64 {
                    let ref mut fresh11 = (*c_t).primitive;
                    *fresh11 = (*c).i64_0;
                } else if name == (*c).str_I128 {
                    let ref mut fresh12 = (*c_t).primitive;
                    *fresh12 = (*c).i128_0;
                } else if name == (*c).str_U128 {
                    let ref mut fresh13 = (*c_t).primitive;
                    *fresh13 = (*c).i128_0;
                } else if ilp32 as libc::c_int != 0 && name == (*c).str_ILong {
                    let ref mut fresh14 = (*c_t).primitive;
                    *fresh14 = (*c).i32_0;
                } else if ilp32 as libc::c_int != 0 && name == (*c).str_ULong {
                    let ref mut fresh15 = (*c_t).primitive;
                    *fresh15 = (*c).i32_0;
                } else if ilp32 as libc::c_int != 0 && name == (*c).str_ISize {
                    let ref mut fresh16 = (*c_t).primitive;
                    *fresh16 = (*c).i32_0;
                } else if ilp32 as libc::c_int != 0 && name == (*c).str_USize {
                    let ref mut fresh17 = (*c_t).primitive;
                    *fresh17 = (*c).i32_0;
                } else if lp64 as libc::c_int != 0 && name == (*c).str_ILong {
                    let ref mut fresh18 = (*c_t).primitive;
                    *fresh18 = (*c).i64_0;
                } else if lp64 as libc::c_int != 0 && name == (*c).str_ULong {
                    let ref mut fresh19 = (*c_t).primitive;
                    *fresh19 = (*c).i64_0;
                } else if lp64 as libc::c_int != 0 && name == (*c).str_ISize {
                    let ref mut fresh20 = (*c_t).primitive;
                    *fresh20 = (*c).i64_0;
                } else if lp64 as libc::c_int != 0 && name == (*c).str_USize {
                    let ref mut fresh21 = (*c_t).primitive;
                    *fresh21 = (*c).i64_0;
                } else if llp64 as libc::c_int != 0 && name == (*c).str_ILong {
                    let ref mut fresh22 = (*c_t).primitive;
                    *fresh22 = (*c).i32_0;
                } else if llp64 as libc::c_int != 0 && name == (*c).str_ULong {
                    let ref mut fresh23 = (*c_t).primitive;
                    *fresh23 = (*c).i32_0;
                } else if llp64 as libc::c_int != 0 && name == (*c).str_ISize {
                    let ref mut fresh24 = (*c_t).primitive;
                    *fresh24 = (*c).i64_0;
                } else if llp64 as libc::c_int != 0 && name == (*c).str_USize {
                    let ref mut fresh25 = (*c_t).primitive;
                    *fresh25 = (*c).i64_0;
                } else if name == (*c).str_F32 {
                    let ref mut fresh26 = (*c_t).primitive;
                    *fresh26 = (*c).f32_0;
                } else if name == (*c).str_F64 {
                    let ref mut fresh27 = (*c_t).primitive;
                    *fresh27 = (*c).f64_0;
                } else if name == (*c).str_Bool {
                    let ref mut fresh28 = (*c_t).primitive;
                    *fresh28 = (*c).i1;
                    let ref mut fresh29 = (*c_t).use_type;
                    *fresh29 = (*c).i1;
                    if target_is_ppc(triple) as libc::c_int != 0
                        && ilp32 as libc::c_int != 0
                        && target_is_macosx(triple) as libc::c_int != 0
                    {
                        let ref mut fresh30 = (*c_t).mem_type;
                        *fresh30 = (*c).i32_0;
                    } else {
                        let ref mut fresh31 = (*c_t).mem_type;
                        *fresh31 = (*c).i8_0;
                    }
                    return 1 as libc::c_int != 0;
                } else {
                    if name == (*c).str_Pointer {
                        let ref mut fresh32 = (*c_t).use_type;
                        *fresh32 = (*c).void_ptr;
                        let ref mut fresh33 = (*c_t).mem_type;
                        *fresh33 = (*c).void_ptr;
                        return 1 as libc::c_int != 0;
                    } else {
                        if name == (*c).str_NullablePointer {
                            let ref mut fresh34 = (*c_t).use_type;
                            *fresh34 = (*c).void_ptr;
                            let ref mut fresh35 = (*c_t).mem_type;
                            *fresh35 = (*c).void_ptr;
                            return 1 as libc::c_int != 0;
                        }
                    }
                }
            }
            if ((*t).bare_method).is_null() {
                let ref mut fresh36 = (*c_t).structure;
                *fresh36 = LLVMStructCreateNamed((*c).context, (*t).name);
                let ref mut fresh37 = (*c_t).structure_ptr;
                *fresh37 = LLVMPointerType((*c_t).structure, 0 as libc::c_int as libc::c_uint);
                if !((*c_t).primitive).is_null() {
                    let ref mut fresh38 = (*c_t).use_type;
                    *fresh38 = (*c_t).primitive;
                } else {
                    let ref mut fresh39 = (*c_t).use_type;
                    *fresh39 = (*c_t).structure_ptr;
                }
                let ref mut fresh40 = (*c_t).mem_type;
                *fresh40 = (*c_t).use_type;
            } else {
                let ref mut fresh41 = (*c_t).structure;
                *fresh41 = (*c).void_ptr;
                let ref mut fresh42 = (*c_t).structure_ptr;
                *fresh42 = (*c).void_ptr;
                let ref mut fresh43 = (*c_t).use_type;
                *fresh43 = (*c).void_ptr;
                let ref mut fresh44 = (*c_t).mem_type;
                *fresh44 = (*c).void_ptr;
            }
            return 1 as libc::c_int != 0;
        }
        150 => {
            let ref mut fresh45 = (*c_t).primitive;
            *fresh45 = LLVMStructCreateNamed((*c).context, (*t).name);
            let ref mut fresh46 = (*c_t).use_type;
            *fresh46 = (*c_t).primitive;
            let ref mut fresh47 = (*c_t).mem_type;
            *fresh47 = (*c_t).primitive;
            return 1 as libc::c_int != 0;
        }
        149 | 56 => {
            let ref mut fresh48 = (*c_t).use_type;
            *fresh48 = (*c).object_ptr;
            let ref mut fresh49 = (*c_t).mem_type;
            *fresh49 = (*c).object_ptr;
            return 1 as libc::c_int != 0;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                as *const u8 as *const libc::c_char,
            201 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"make_opaque_struct\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int != 0;
}
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn make_debug_basic(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut size: u64 = LLVMABISizeOfType((*c).target_data, (*c_t).primitive);
    let mut _align: u64 = LLVMABIAlignmentOfType((*c).target_data, (*c_t).primitive) as u64;
    let mut encoding: libc::c_uint = 0;
    if is_bool((*t).ast) {
        encoding = DW_ATE_boolean as libc::c_int as libc::c_uint;
    } else if is_float((*t).ast) {
        encoding = DW_ATE_float as libc::c_int as libc::c_uint;
    } else if is_signed((*t).ast) {
        encoding = DW_ATE_signed as libc::c_int as libc::c_uint;
    } else {
        encoding = DW_ATE_unsigned as libc::c_int as libc::c_uint;
    }
    let ref mut fresh50 = (*c_t).di_type;
    *fresh50 = LLVMDIBuilderCreateBasicType(
        (*c).di,
        (*t).name,
        strlen((*t).name),
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(size),
        encoding,
        LLVMDIFlagZero,
    );
}
#[c2rust::src_loc = "228:1"]
unsafe extern "C" fn make_debug_prototype(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh51 = (*c_t).di_type;
    *fresh51 = LLVMDIBuilderCreateReplaceableStruct(
        (*c).di,
        (*t).name,
        (*c).di_unit,
        (*c_t).di_file,
        ast_line((*t).ast) as libc::c_uint,
    );
    if (*t).underlying as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint {
        let ref mut fresh52 = (*c_t).di_type_embed;
        *fresh52 = (*c_t).di_type;
        let mut size_bytes: u64 = LLVMABISizeOfType((*c).target_data, (*c_t).mem_type);
        let mut align_bytes: u32 = LLVMABIAlignmentOfType((*c).target_data, (*c_t).mem_type);
        let ref mut fresh53 = (*c_t).di_type;
        *fresh53 = LLVMDIBuilderCreatePointerType(
            (*c).di,
            (*c_t).di_type_embed,
            size_bytes.wrapping_mul(8 as libc::c_int as libc::c_ulonglong),
            align_bytes.wrapping_mul(8 as libc::c_int as libc::c_uint),
            0 as libc::c_int as libc::c_uint,
            0 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
    }
}
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn make_debug_info(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
    let mut source: *mut source_t = 0 as *mut source_t;
    if !def.is_null() {
        source = ast_source(def);
    } else {
        source = ast_source((*t).ast);
    }
    let mut file: *const libc::c_char = (*source).file;
    if file.is_null() {
        file = b"\0" as *const u8 as *const libc::c_char;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh54 = (*c_t).di_file;
    *fresh54 = LLVMDIBuilderCreateFile(
        (*c).di,
        file,
        strlen(file),
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    match (*t).underlying as libc::c_uint {
        150 | 75 => {
            make_debug_prototype(c, t);
            return;
        }
        74 => {
            if !((*c_t).primitive).is_null() {
                make_debug_basic(c, t);
            } else {
                make_debug_prototype(c, t);
            }
            return;
        }
        149 | 56 | 72 | 73 | 76 | 77 => {
            make_debug_prototype(c, t);
            return;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                as *const u8 as *const libc::c_char,
            290 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"make_debug_info\0"))
                .as_ptr(),
        );
    };
}
#[c2rust::src_loc = "293:1"]
unsafe extern "C" fn make_box_type(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if ((*c_t).primitive).is_null() {
        return;
    }
    let mut box_name: *const libc::c_char = genname_box((*t).name);
    let ref mut fresh55 = (*c_t).structure;
    *fresh55 = LLVMStructCreateNamed((*c).context, box_name);
    let mut elements: [LLVMTypeRef; 2] = [0 as *mut LLVMOpaqueType; 2];
    elements[0 as libc::c_int as usize] =
        LLVMPointerType((*c_t).desc_type, 0 as libc::c_int as libc::c_uint);
    elements[1 as libc::c_int as usize] = (*c_t).mem_type;
    LLVMStructSetBody(
        (*c_t).structure,
        elements.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh56 = (*c_t).structure_ptr;
    *fresh56 = LLVMPointerType((*c_t).structure, 0 as libc::c_int as libc::c_uint);
}
#[c2rust::src_loc = "311:1"]
unsafe extern "C" fn make_global_instance(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    if (*t).underlying as libc::c_uint != TK_PRIMITIVE as libc::c_int as libc::c_uint {
        return;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if !((*c_t).primitive).is_null() {
        return;
    }
    if !((*t).bare_method).is_null() {
        return;
    }
    let mut inst_name: *const libc::c_char = genname_instance((*t).name);
    let mut value: LLVMValueRef = LLVMConstNamedStruct(
        (*c_t).structure,
        &mut (*c_t).desc,
        1 as libc::c_int as libc::c_uint,
    );
    let ref mut fresh57 = (*c_t).instance;
    *fresh57 = LLVMAddGlobal((*c).module, (*c_t).structure, inst_name);
    LLVMSetInitializer((*c_t).instance, value);
    LLVMSetGlobalConstant((*c_t).instance, 1 as libc::c_int);
    LLVMSetLinkage((*c_t).instance, LLVMPrivateLinkage);
}
#[c2rust::src_loc = "335:1"]
unsafe extern "C" fn make_dispatch(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    if (*t).underlying as libc::c_uint != TK_ACTOR as libc::c_int as libc::c_uint {
        return;
    }
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut dispatch_name: *const libc::c_char = genname_dispatch((*t).name);
    let ref mut fresh58 = (*c_t).dispatch_fn;
    *fresh58 = codegen_addfun(c, dispatch_name, (*c).dispatch_type, 1 as libc::c_int != 0);
    LLVMSetFunctionCallConv(
        (*c_t).dispatch_fn,
        LLVMCCallConv as libc::c_int as libc::c_uint,
    );
    LLVMSetLinkage((*c_t).dispatch_fn, LLVMExternalLinkage);
    codegen_startfun(
        c,
        (*c_t).dispatch_fn,
        0 as LLVMMetadataRef,
        0 as LLVMMetadataRef,
        0 as *mut deferred_reification_t,
        0 as libc::c_int != 0,
    );
    let mut unreachable: LLVMBasicBlockRef =
        codegen_block(c, b"unreachable\0" as *const u8 as *const libc::c_char);
    let mut msg: LLVMValueRef = LLVMGetParam((*c_t).dispatch_fn, 2 as libc::c_int as libc::c_uint);
    let mut id_ptr: LLVMValueRef = LLVMBuildStructGEP_P(
        (*c).builder,
        msg,
        1 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut id: LLVMValueRef = LLVMBuildLoad_P(
        (*c).builder,
        id_ptr,
        b"id\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh59 = (*c_t).dispatch_switch;
    *fresh59 = LLVMBuildSwitch(
        (*c).builder,
        id,
        unreachable,
        0 as libc::c_int as libc::c_uint,
    );
    LLVMPositionBuilderAtEnd((*c).builder, unreachable);
    LLVMBuildUnreachable((*c).builder);
    codegen_finishfun(c);
}
#[c2rust::src_loc = "367:1"]
unsafe extern "C" fn make_struct(mut c: *mut compile_t, mut t: *mut reach_type_t) -> bool {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let mut type_0: LLVMTypeRef = 0 as *mut LLVMOpaqueType;
    let mut extra: libc::c_int = 0 as libc::c_int;
    let mut packed: bool = 0 as libc::c_int != 0;
    if !((*t).bare_method).is_null() {
        return 1 as libc::c_int != 0;
    }
    match (*t).underlying as libc::c_uint {
        149 | 56 | 72 | 73 => return 1 as libc::c_int != 0,
        150 => {
            type_0 = (*c_t).primitive;
        }
        75 => {
            if ((*c_t).structure).is_null() {
                return 1 as libc::c_int != 0;
            }
            type_0 = (*c_t).structure;
            let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
            if ast_has_annotation(def, b"packed\0" as *const u8 as *const libc::c_char) {
                packed = 1 as libc::c_int != 0;
            }
        }
        74 => {
            if !((*c_t).primitive).is_null() {
                (*c_t).abi_size = LLVMABISizeOfType((*c).target_data, (*c_t).structure) as size_t;
                return 1 as libc::c_int != 0;
            }
            extra = 1 as libc::c_int;
            type_0 = (*c_t).structure;
        }
        76 => {
            extra = 1 as libc::c_int;
            type_0 = (*c_t).structure;
        }
        77 => {
            extra = 2 as libc::c_int;
            type_0 = (*c_t).structure;
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                        as *const u8 as *const libc::c_char,
                    428 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"make_struct\0"))
                        .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
    }
    let mut buf_size: size_t = (((*t).field_count).wrapping_add(extra as libc::c_uint)
        as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<LLVMTypeRef>() as libc::c_ulong);
    let mut elements: *mut LLVMTypeRef = ponyint_pool_alloc_size(buf_size) as *mut LLVMTypeRef;
    if extra > 0 as libc::c_int {
        let ref mut fresh60 = *elements.offset(0 as libc::c_int as isize);
        *fresh60 = LLVMPointerType((*c_t).desc_type, 0 as libc::c_int as libc::c_uint);
    }
    if extra > 1 as libc::c_int {
        let ref mut fresh61 = *elements.offset(1 as libc::c_int as isize);
        *fresh61 = (*c).actor_pad;
    }
    let mut i: u32 = 0 as libc::c_int as u32;
    while i < (*t).field_count {
        let mut f_c_t: *mut compile_type_t =
            (*(*((*t).fields).offset(i as isize)).type_0).c_type as *mut compile_type_t;
        if (*((*t).fields).offset(i as isize)).embed {
            let ref mut fresh62 = *elements.offset(i.wrapping_add(extra as libc::c_uint) as isize);
            *fresh62 = (*f_c_t).structure;
        } else {
            let ref mut fresh63 = *elements.offset(i.wrapping_add(extra as libc::c_uint) as isize);
            *fresh63 = (*f_c_t).mem_type;
        }
        if (*elements.offset(i.wrapping_add(extra as libc::c_uint) as isize)).is_null() {
            if 0 as libc::c_int != 0 {
            } else {
                ponyint_assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                        as *const u8 as *const libc::c_char,
                    454 as libc::c_int as size_t,
                    (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"make_struct\0"))
                        .as_ptr(),
                );
            };
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    LLVMStructSetBody(
        type_0,
        elements,
        ((*t).field_count).wrapping_add(extra as libc::c_uint),
        packed as LLVMBool,
    );
    ponyint_pool_free_size(buf_size, elements as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn make_debug_field(
    mut c: *mut compile_t,
    mut t: *mut reach_type_t,
    mut i: u32,
) -> LLVMMetadataRef {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut flags: LLVMDIFlags = LLVMDIFlagZero;
    let mut offset: u64 = 0 as libc::c_int as u64;
    let mut ast: *mut ast_t = 0 as *mut ast_t;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if (*t).underlying as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint {
        let mut def: *mut ast_t = ast_data((*t).ast) as *mut ast_t;
        let mut members: *mut ast_t = ast_childidx(def, 4 as libc::c_int as size_t);
        ast = ast_childidx(members, i as size_t);
        name = ast_name(ast_child(ast));
        if is_name_private(name) {
            flags = (flags as libc::c_uint | LLVMDIFlagPrivate as libc::c_int as libc::c_uint)
                as LLVMDIFlags;
        }
        let mut extra: u32 = 0 as libc::c_int as u32;
        if (*t).underlying as libc::c_uint != TK_STRUCT as libc::c_int as libc::c_uint {
            extra = extra.wrapping_add(1);
        }
        if (*t).underlying as libc::c_uint == TK_ACTOR as libc::c_int as libc::c_uint {
            extra = extra.wrapping_add(1);
        }
        offset = LLVMOffsetOfElement((*c).target_data, (*c_t).structure, i.wrapping_add(extra));
    } else {
        snprintf(
            buf.as_mut_ptr(),
            32 as libc::c_int as libc::c_ulong,
            b"_%d\0" as *const u8 as *const libc::c_char,
            i.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        name = buf.as_mut_ptr();
        ast = (*t).ast;
        offset = LLVMOffsetOfElement((*c).target_data, (*c_t).primitive, i);
    }
    let mut type_0: LLVMTypeRef = 0 as *mut LLVMOpaqueType;
    let mut di_type: LLVMMetadataRef = 0 as *mut LLVMOpaqueMetadata;
    let mut f_c_t: *mut compile_type_t =
        (*(*((*t).fields).offset(i as isize)).type_0).c_type as *mut compile_type_t;
    if (*((*t).fields).offset(i as isize)).embed {
        type_0 = (*f_c_t).structure;
        di_type = (*f_c_t).di_type_embed;
    } else {
        type_0 = (*f_c_t).mem_type;
        di_type = (*f_c_t).di_type;
    }
    let mut size: u64 = LLVMABISizeOfType((*c).target_data, type_0);
    let mut align: u64 = LLVMABIAlignmentOfType((*c).target_data, type_0) as u64;
    return LLVMDIBuilderCreateMemberType(
        (*c).di,
        (*c).di_unit,
        name,
        strlen(name),
        (*c_t).di_file,
        ast_line(ast) as libc::c_uint,
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(size),
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(align) as u32,
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(offset),
        flags,
        di_type,
    );
}
#[c2rust::src_loc = "521:1"]
unsafe extern "C" fn make_debug_fields(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    let mut fields: *mut LLVMMetadataRef = 0 as *mut LLVMMetadataRef;
    let mut fields_buf_size: size_t = 0;
    if (*t).field_count > 0 as libc::c_int as libc::c_uint {
        fields_buf_size = ((*t).field_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LLVMMetadataRef>() as libc::c_ulong);
        fields = ponyint_pool_alloc_size(fields_buf_size) as *mut LLVMMetadataRef;
        let mut i: u32 = 0 as libc::c_int as u32;
        while i < (*t).field_count {
            let ref mut fresh64 = *fields.offset(i as isize);
            *fresh64 = make_debug_field(c, t, i);
            i = i.wrapping_add(1);
        }
    }
    let mut type_0: LLVMTypeRef = 0 as *mut LLVMOpaqueType;
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if (*t).underlying as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint {
        type_0 = (*c_t).structure;
    } else {
        type_0 = (*c_t).primitive;
    }
    let mut size: u64 = 0 as libc::c_int as u64;
    let mut align: u64 = 0 as libc::c_int as u64;
    if !type_0.is_null() {
        size = LLVMABISizeOfType((*c).target_data, type_0);
        align = LLVMABIAlignmentOfType((*c).target_data, type_0) as u64;
    }
    let mut di_type: LLVMMetadataRef = LLVMDIBuilderCreateStructType(
        (*c).di,
        (*c).di_unit,
        (*t).name,
        strlen((*t).name),
        (*c_t).di_file,
        ast_line((*t).ast) as libc::c_uint,
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(size),
        (8 as libc::c_int as libc::c_ulonglong).wrapping_mul(align) as u32,
        LLVMDIFlagZero,
        0 as LLVMMetadataRef,
        fields,
        (*t).field_count,
        0 as libc::c_int as libc::c_uint,
        0 as LLVMMetadataRef,
        (*t).name,
        strlen((*t).name),
    );
    if !fields.is_null() {
        ponyint_pool_free_size(fields_buf_size, fields as *mut libc::c_void);
    }
    if (*t).underlying as libc::c_uint != TK_TUPLETYPE as libc::c_int as libc::c_uint {
        LLVMMetadataReplaceAllUsesWith((*c_t).di_type_embed, di_type);
        let ref mut fresh65 = (*c_t).di_type_embed;
        *fresh65 = di_type;
    } else {
        LLVMMetadataReplaceAllUsesWith((*c_t).di_type, di_type);
        let ref mut fresh66 = (*c_t).di_type;
        *fresh66 = di_type;
    };
}
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn make_debug_final(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    match (*t).underlying as libc::c_uint {
        149 | 56 | 150 | 72 | 73 | 75 | 76 | 77 => {
            make_debug_fields(c, t);
            return;
        }
        74 => {
            let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
            if ((*c_t).primitive).is_null() {
                make_debug_fields(c, t);
            }
            return;
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        ponyint_assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                as *const u8 as *const libc::c_char,
            596 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"make_debug_final\0"))
                .as_ptr(),
        );
    };
}
#[c2rust::src_loc = "599:1"]
unsafe extern "C" fn make_intrinsic_methods(mut c: *mut compile_t, mut t: *mut reach_type_t) {
    if (*t).can_be_boxed {
        gen_digestof_fun(c, t);
        if ast_id((*t).ast) as libc::c_uint == TK_TUPLETYPE as libc::c_int as libc::c_uint {
            gen_is_tuple_fun(c, t);
        }
    }
    if ast_id((*t).ast) as libc::c_uint != TK_NOMINAL as libc::c_int as libc::c_uint {
        return;
    }
    let mut pkg: ast_ptr_t = 0 as *mut ast_t;
    let mut id: ast_ptr_t = 0 as *mut ast_t;
    let mut children: [*mut *mut ast_t; 3] = [&mut pkg, &mut id, 0 as *mut *mut ast_t];
    ast_get_children(
        (*t).ast,
        (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        children.as_mut_ptr(),
    );
    let mut package: *const libc::c_char = ast_name(pkg);
    let mut name: *const libc::c_char = ast_name(id);
    if package == (*c).str_builtin {
        if name == (*c).str_Pointer {
            genprim_pointer_methods(c, t);
        } else if name == (*c).str_NullablePointer {
            genprim_nullable_pointer_methods(c, t);
        } else if name == (*c).str_DoNotOptimise {
            genprim_donotoptimise_methods(c, t);
        } else if name == (*c).str_Platform {
            genprim_platform_methods(c, t);
        }
    }
}
#[c2rust::src_loc = "629:1"]
unsafe extern "C" fn make_trace(mut c: *mut compile_t, mut t: *mut reach_type_t) -> bool {
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    if ((*c_t).trace_fn).is_null() {
        return 1 as libc::c_int != 0;
    }
    if (*t).underlying as libc::c_uint == TK_CLASS as libc::c_int as libc::c_uint {
        let mut pkg: ast_ptr_t = 0 as *mut ast_t;
        let mut id: ast_ptr_t = 0 as *mut ast_t;
        let mut children: [*mut *mut ast_t; 3] = [&mut pkg, &mut id, 0 as *mut *mut ast_t];
        ast_get_children(
            (*t).ast,
            (::core::mem::size_of::<[*mut *mut ast_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut *mut ast_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            children.as_mut_ptr(),
        );
        let mut package: *const libc::c_char = ast_name(pkg);
        let mut name: *const libc::c_char = ast_name(id);
        if package == (*c).str_builtin && name == (*c).str_Array {
            genprim_array_trace(c, t);
            return 1 as libc::c_int != 0;
        }
    }
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
        (*c_t).structure_ptr,
        b"object\0" as *const u8 as *const libc::c_char,
    );
    let mut extra: libc::c_int = 0 as libc::c_int;
    match (*t).underlying as libc::c_uint {
        76 => {
            extra = 1 as libc::c_int;
        }
        77 => {
            extra = 2 as libc::c_int;
        }
        150 => {
            object = LLVMBuildStructGEP_P(
                (*c).builder,
                object,
                1 as libc::c_int as libc::c_uint,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    let mut i: u32 = 0 as libc::c_int as u32;
    while i < (*t).field_count {
        let mut f: *mut reach_field_t =
            &mut *((*t).fields).offset(i as isize) as *mut reach_field_t;
        let mut f_c_t: *mut compile_type_t = (*(*f).type_0).c_type as *mut compile_type_t;
        let mut field: LLVMValueRef = LLVMBuildStructGEP_P(
            (*c).builder,
            object,
            i.wrapping_add(extra as libc::c_uint),
            b"\0" as *const u8 as *const libc::c_char,
        );
        if !(*f).embed {
            field = LLVMBuildLoad_P(
                (*c).builder,
                field,
                b"\0" as *const u8 as *const libc::c_char,
            );
            let mut field_type: *mut ast_t = (*f).ast;
            field = gen_assign_cast(c, (*f_c_t).use_type, field, field_type);
            gentrace(c, ctx, field, field, field_type, field_type);
        } else {
            let mut trace_fn: LLVMValueRef = (*f_c_t).trace_fn;
            if !trace_fn.is_null() {
                let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
                args[0 as libc::c_int as usize] = ctx;
                args[1 as libc::c_int as usize] = LLVMBuildBitCast(
                    (*c).builder,
                    field,
                    (*c).object_ptr,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                LLVMBuildCall_P(
                    (*c).builder,
                    trace_fn,
                    args.as_mut_ptr(),
                    2 as libc::c_int as libc::c_uint,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    LLVMBuildRetVoid((*c).builder);
    codegen_finishfun(c);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "714:1"]
pub unsafe extern "C" fn gentypes(mut c: *mut compile_t) -> bool {
    let mut t: *mut reach_type_t = 0 as *mut reach_type_t;
    let mut i: size_t = 0;
    if target_is_ilp32((*(*c).opt).triple) {
        (*c).trait_bitmap_size = (((*(*c).reach).trait_type_count)
            .wrapping_add(31 as libc::c_int as libc::c_uint)
            & !(31 as libc::c_int) as libc::c_uint)
            >> 5 as libc::c_int;
    } else {
        (*c).trait_bitmap_size = (((*(*c).reach).trait_type_count)
            .wrapping_add(63 as libc::c_int as libc::c_uint)
            & !(63 as libc::c_int) as libc::c_uint)
            >> 6 as libc::c_int;
    }
    allocate_compile_types(c);
    genprim_builtins(c);
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b" Data prototypes\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = -(1 as libc::c_int) as size_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if !make_opaque_struct(c, t) {
            return 0 as libc::c_int != 0;
        }
        gendesc_type(c, t);
        make_debug_info(c, t);
        make_box_type(c, t);
        make_dispatch(c, t);
        gentrace_prototype(c, t);
    }
    gendesc_table(c);
    let ref mut fresh67 = (*c).numeric_sizes;
    *fresh67 = gen_numeric_size_table(c);
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b" Data types\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = -(1 as libc::c_int) as size_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if !make_struct(c, t) {
            return 0 as libc::c_int != 0;
        }
        make_global_instance(c, t);
    }
    genprim_signature(c);
    t = reach_type_name((*c).reach, b"None\0" as *const u8 as *const libc::c_char);
    if !t.is_null() {
    } else {
        ponyint_assert_fail(
            b"t != NULL\0" as *const u8 as *const libc::c_char,
            b"/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.c\0"
                as *const u8 as *const libc::c_char,
            766 as libc::c_int as size_t,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"gentypes\0")).as_ptr(),
        );
    };
    let mut c_t: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
    let ref mut fresh68 = (*c).none_instance;
    *fresh68 = (*c_t).instance;
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b" Function prototypes\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = -(1 as libc::c_int) as size_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        let mut c_t_0: *mut compile_type_t = (*t).c_type as *mut compile_type_t;
        if !((*c_t_0).structure).is_null() {
            (*c_t_0).abi_size = LLVMABISizeOfType((*c).target_data, (*c_t_0).structure) as size_t;
        }
        make_debug_final(c, t);
        make_intrinsic_methods(c, t);
        if !genfun_method_sigs(c, t) {
            return 0 as libc::c_int != 0;
        }
    }
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b" Functions\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = -(1 as libc::c_int) as size_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if !genfun_method_bodies(c, t) {
            return 0 as libc::c_int != 0;
        }
    }
    genfun_primitive_calls(c);
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_INFO as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b" Descriptors\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = -(1 as libc::c_int) as size_t;
    loop {
        t = reach_types_next(&mut (*(*c).reach).types, &mut i);
        if t.is_null() {
            break;
        }
        if !make_trace(c, t) {
            return 0 as libc::c_int != 0;
        }
        if !genserialise(c, t) {
            return 0 as libc::c_int != 0;
        }
        gendesc_init(c, t);
    }
    return 1 as libc::c_int != 0;
}
