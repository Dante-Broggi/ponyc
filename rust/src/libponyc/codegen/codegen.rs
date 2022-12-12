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
    #[c2rust::src_loc = "45:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "46:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "47:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "48:1"]
    pub type __int64_t = libc::c_longlong;
    #[c2rust::src_loc = "49:1"]
    pub type __uint64_t = libc::c_ulonglong;
    #[c2rust::src_loc = "94:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[c2rust::src_loc = "122:1"]
    pub type __darwin_time_t = libc::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "55:1"]
    pub type __darwin_blkcnt_t = __int64_t;
    #[c2rust::src_loc = "56:1"]
    pub type __darwin_blksize_t = __int32_t;
    #[c2rust::src_loc = "57:1"]
    pub type __darwin_dev_t = __int32_t;
    #[c2rust::src_loc = "60:1"]
    pub type __darwin_gid_t = __uint32_t;
    #[c2rust::src_loc = "62:1"]
    pub type __darwin_ino64_t = __uint64_t;
    #[c2rust::src_loc = "70:1"]
    pub type __darwin_mode_t = __uint16_t;
    #[c2rust::src_loc = "71:1"]
    pub type __darwin_off_t = __int64_t;
    #[c2rust::src_loc = "75:1"]
    pub type __darwin_uid_t = __uint32_t;
    use super::_types_h::{__int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:1"]
pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: libc::c_long,
    }
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_dev_t.h:1"]
pub mod _dev_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:1"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h:1"]
pub mod _off_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blkcnt_t.h:1"]
pub mod _blkcnt_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blksize_t.h:1"]
pub mod _blksize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:1"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_nlink_t.h:1"]
pub mod _nlink_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type nlink_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:1"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: __uint32_t,
        pub st_gen: __uint32_t,
        pub st_lspare: __int32_t,
        pub st_qspare: [__int64_t; 2],
    }
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {
        #[c2rust::src_loc = "386:1"]
        pub fn stat(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/ErrorHandling.h:1"]
pub mod ErrorHandling_h {
    #[c2rust::src_loc = "27:1"]
    pub type LLVMFatalErrorHandler = Option<unsafe extern "C" fn(*const libc::c_char) -> ()>;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn LLVMInstallFatalErrorHandler(Handler: LLVMFatalErrorHandler);
        #[c2rust::src_loc = "42:1"]
        pub fn LLVMResetFatalErrorHandler();
        #[c2rust::src_loc = "49:1"]
        pub fn LLVMEnablePrettyStackTrace();
    }
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
    #[c2rust::src_loc = "130:1"]
    pub type LLVMPassRegistryRef = *mut LLVMOpaquePassRegistry;
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
        #[c2rust::src_loc = "130:16"]
        pub type LLVMOpaquePassRegistry;
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
        LLVMMetadataRef, LLVMModuleRef, LLVMPassRegistryRef, LLVMTypeRef,
        LLVMValueRef,
    };
    use super::_size_t_h::size_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "469:1"]
        pub fn LLVMInitializeCore(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "474:1"]
        pub fn LLVMShutdown();
        #[c2rust::src_loc = "478:1"]
        pub fn LLVMCreateMessage(Message: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "479:1"]
        pub fn LLVMDisposeMessage(Message: *mut libc::c_char);
        #[c2rust::src_loc = "502:1"]
        pub fn LLVMContextCreate() -> LLVMContextRef;
        #[c2rust::src_loc = "557:1"]
        pub fn LLVMContextDispose(C: LLVMContextRef);
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
        #[c2rust::src_loc = "680:1"]
        pub fn LLVMModuleCreateWithNameInContext(
            ModuleID: *const libc::c_char,
            C: LLVMContextRef,
        ) -> LLVMModuleRef;
        #[c2rust::src_loc = "693:1"]
        pub fn LLVMDisposeModule(M: LLVMModuleRef);
        #[c2rust::src_loc = "753:1"]
        pub fn LLVMSetDataLayout(M: LLVMModuleRef, DataLayoutStr: *const libc::c_char);
        #[c2rust::src_loc = "767:1"]
        pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const libc::c_char);
        #[c2rust::src_loc = "1023:1"]
        pub fn LLVMAddFunction(
            M: LLVMModuleRef,
            Name: *const libc::c_char,
            FunctionTy: LLVMTypeRef,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "1106:1"]
        pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
        #[c2rust::src_loc = "1150:1"]
        pub fn LLVMInt1TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1151:1"]
        pub fn LLVMInt8TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1152:1"]
        pub fn LLVMInt16TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1153:1"]
        pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1154:1"]
        pub fn LLVMInt64TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1156:1"]
        pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: libc::c_uint) -> LLVMTypeRef;
        #[c2rust::src_loc = "1194:1"]
        pub fn LLVMFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1199:1"]
        pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
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
        #[c2rust::src_loc = "1499:1"]
        pub fn LLVMVoidTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1669:1"]
        pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "1934:1"]
        pub fn LLVMConstInt(
            IntTy: LLVMTypeRef,
            N: libc::c_ulonglong,
            SignExtend: LLVMBool,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2025:1"]
        pub fn LLVMConstStringInContext(
            C: LLVMContextRef,
            Str: *const libc::c_char,
            Length: libc::c_uint,
            DontNullTerminate: LLVMBool,
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
        #[c2rust::src_loc = "2683:1"]
        pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2699:1"]
        pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;
        #[c2rust::src_loc = "2900:1"]
        pub fn LLVMMDNodeInContext(
            C: LLVMContextRef,
            Vals: *mut LLVMValueRef,
            Count: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "2970:1"]
        pub fn LLVMCountBasicBlocks(Fn: LLVMValueRef) -> libc::c_uint;
        #[c2rust::src_loc = "3015:1"]
        pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3048:1"]
        pub fn LLVMAppendBasicBlockInContext(
            C: LLVMContextRef,
            Fn: LLVMValueRef,
            Name: *const libc::c_char,
        ) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3120:1"]
        pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
        #[c2rust::src_loc = "3297:1"]
        pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: libc::c_uint);
        #[c2rust::src_loc = "3602:1"]
        pub fn LLVMCreateBuilderInContext(C: LLVMContextRef) -> LLVMBuilderRef;
        #[c2rust::src_loc = "3606:1"]
        pub fn LLVMPositionBuilderBefore(Builder: LLVMBuilderRef, Instr: LLVMValueRef);
        #[c2rust::src_loc = "3607:1"]
        pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
        #[c2rust::src_loc = "3608:1"]
        pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
        #[c2rust::src_loc = "3613:1"]
        pub fn LLVMDisposeBuilder(Builder: LLVMBuilderRef);
        #[c2rust::src_loc = "3631:1"]
        pub fn LLVMSetCurrentDebugLocation2(Builder: LLVMBuilderRef, Loc: LLVMMetadataRef);
        #[c2rust::src_loc = "4134:1"]
        pub fn LLVMGetGlobalPassRegistry() -> LLVMPassRegistryRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Target.h:1"]
pub mod Target_h {
    #[c2rust::src_loc = "37:1"]
    pub type LLVMTargetDataRef = *mut LLVMOpaqueTargetData;
    #[inline]
    #[c2rust::src_loc = "76:1"]
    pub unsafe extern "C" fn LLVMInitializeAllTargetInfos() {
        LLVMInitializeX86TargetInfo();
        LLVMInitializeARMTargetInfo();
        LLVMInitializeAArch64TargetInfo();
        LLVMInitializeWebAssemblyTargetInfo();
        LLVMInitializeRISCVTargetInfo();
    }
    #[inline]
    #[c2rust::src_loc = "85:1"]
    pub unsafe extern "C" fn LLVMInitializeAllTargets() {
        LLVMInitializeX86Target();
        LLVMInitializeARMTarget();
        LLVMInitializeAArch64Target();
        LLVMInitializeWebAssemblyTarget();
        LLVMInitializeRISCVTarget();
    }
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn LLVMInitializeAllTargetMCs() {
        LLVMInitializeX86TargetMC();
        LLVMInitializeARMTargetMC();
        LLVMInitializeAArch64TargetMC();
        LLVMInitializeWebAssemblyTargetMC();
        LLVMInitializeRISCVTargetMC();
    }
    #[inline]
    #[c2rust::src_loc = "103:1"]
    pub unsafe extern "C" fn LLVMInitializeAllAsmPrinters() {
        LLVMInitializeX86AsmPrinter();
        LLVMInitializeARMAsmPrinter();
        LLVMInitializeAArch64AsmPrinter();
        LLVMInitializeWebAssemblyAsmPrinter();
        LLVMInitializeRISCVAsmPrinter();
    }
    #[inline]
    #[c2rust::src_loc = "112:1"]
    pub unsafe extern "C" fn LLVMInitializeAllAsmParsers() {
        LLVMInitializeX86AsmParser();
        LLVMInitializeARMAsmParser();
        LLVMInitializeAArch64AsmParser();
        LLVMInitializeWebAssemblyAsmParser();
        LLVMInitializeRISCVAsmParser();
    }
    #[inline]
    #[c2rust::src_loc = "131:1"]
    pub unsafe extern "C" fn LLVMInitializeNativeTarget() -> LLVMBool {
        LLVMInitializeX86TargetInfo();
        LLVMInitializeX86Target();
        LLVMInitializeX86TargetMC();
        return 0 as libc::c_int;
    }
    use super::AsmParsers_def::{
        LLVMInitializeAArch64AsmParser, LLVMInitializeARMAsmParser, LLVMInitializeRISCVAsmParser,
        LLVMInitializeWebAssemblyAsmParser, LLVMInitializeX86AsmParser,
    };
    use super::AsmPrinters_def::{
        LLVMInitializeAArch64AsmPrinter, LLVMInitializeARMAsmPrinter,
        LLVMInitializeRISCVAsmPrinter, LLVMInitializeWebAssemblyAsmPrinter,
        LLVMInitializeX86AsmPrinter,
    };
    use super::Targets_def::{
        LLVMInitializeAArch64Target, LLVMInitializeAArch64TargetInfo,
        LLVMInitializeAArch64TargetMC, LLVMInitializeARMTarget, LLVMInitializeARMTargetInfo,
        LLVMInitializeARMTargetMC, LLVMInitializeRISCVTarget, LLVMInitializeRISCVTargetInfo,
        LLVMInitializeRISCVTargetMC, LLVMInitializeWebAssemblyTarget,
        LLVMInitializeWebAssemblyTargetInfo, LLVMInitializeWebAssemblyTargetMC,
        LLVMInitializeX86Target, LLVMInitializeX86TargetInfo, LLVMInitializeX86TargetMC,
    };
    use super::Types_h::{
        LLVMBool, LLVMContextRef, LLVMTypeRef,
    };
    extern "C" {
        #[c2rust::src_loc = "37:16"]
        pub type LLVMOpaqueTargetData;
        #[c2rust::src_loc = "201:1"]
        pub fn LLVMDisposeTargetData(TD: LLVMTargetDataRef);
        #[c2rust::src_loc = "212:1"]
        pub fn LLVMCopyStringRepOfTargetData(TD: LLVMTargetDataRef) -> *mut libc::c_char;
        #[c2rust::src_loc = "239:1"]
        pub fn LLVMIntPtrTypeInContext(C: LLVMContextRef, TD: LLVMTargetDataRef) -> LLVMTypeRef;
        #[c2rust::src_loc = "257:1"]
        pub fn LLVMABISizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> libc::c_ulonglong;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/TargetMachine.h:1"]
pub mod TargetMachine_h {
    #[c2rust::src_loc = "34:1"]
    pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;
    #[c2rust::src_loc = "35:1"]
    pub type LLVMTargetRef = *mut LLVMTarget;
    use super::Target_h::LLVMTargetDataRef;
    use super::Types_h::LLVMBool;
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type LLVMOpaqueTargetMachine;
        #[c2rust::src_loc = "35:16"]
        pub type LLVMTarget;
        #[c2rust::src_loc = "108:1"]
        pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);
        #[c2rust::src_loc = "82:1"]
        pub fn LLVMGetTargetFromTriple(
            Triple: *const libc::c_char,
            T: *mut LLVMTargetRef,
            ErrorMessage: *mut *mut libc::c_char,
        ) -> LLVMBool;
        #[c2rust::src_loc = "129:1"]
        pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;
        #[c2rust::src_loc = "152:1"]
        pub fn LLVMNormalizeTargetTriple(triple: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "156:1"]
        pub fn LLVMGetHostCPUName() -> *mut libc::c_char;
        #[c2rust::src_loc = "160:1"]
        pub fn LLVMGetHostCPUFeatures() -> *mut libc::c_char;
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
    #[c2rust::src_loc = "78:9"]
    pub type LLVMDWARFSourceLanguage = libc::c_uint;
    #[c2rust::src_loc = "122:3"]
    pub const LLVMDWARFSourceLanguageBORLAND_Delphi: LLVMDWARFSourceLanguage = 39;
    #[c2rust::src_loc = "121:3"]
    pub const LLVMDWARFSourceLanguageGOOGLE_RenderScript: LLVMDWARFSourceLanguage = 38;
    #[c2rust::src_loc = "120:3"]
    pub const LLVMDWARFSourceLanguageMips_Assembler: LLVMDWARFSourceLanguage = 37;
    #[c2rust::src_loc = "118:3"]
    pub const LLVMDWARFSourceLanguageBLISS: LLVMDWARFSourceLanguage = 36;
    #[c2rust::src_loc = "117:3"]
    pub const LLVMDWARFSourceLanguageRenderScript: LLVMDWARFSourceLanguage = 35;
    #[c2rust::src_loc = "116:3"]
    pub const LLVMDWARFSourceLanguageFortran08: LLVMDWARFSourceLanguage = 34;
    #[c2rust::src_loc = "115:3"]
    pub const LLVMDWARFSourceLanguageFortran03: LLVMDWARFSourceLanguage = 33;
    #[c2rust::src_loc = "114:3"]
    pub const LLVMDWARFSourceLanguageC_plus_plus_14: LLVMDWARFSourceLanguage = 32;
    #[c2rust::src_loc = "113:3"]
    pub const LLVMDWARFSourceLanguageDylan: LLVMDWARFSourceLanguage = 31;
    #[c2rust::src_loc = "112:3"]
    pub const LLVMDWARFSourceLanguageJulia: LLVMDWARFSourceLanguage = 30;
    #[c2rust::src_loc = "111:3"]
    pub const LLVMDWARFSourceLanguageSwift: LLVMDWARFSourceLanguage = 29;
    #[c2rust::src_loc = "110:3"]
    pub const LLVMDWARFSourceLanguageC11: LLVMDWARFSourceLanguage = 28;
    #[c2rust::src_loc = "109:3"]
    pub const LLVMDWARFSourceLanguageRust: LLVMDWARFSourceLanguage = 27;
    #[c2rust::src_loc = "108:3"]
    pub const LLVMDWARFSourceLanguageOCaml: LLVMDWARFSourceLanguage = 26;
    #[c2rust::src_loc = "107:3"]
    pub const LLVMDWARFSourceLanguageC_plus_plus_11: LLVMDWARFSourceLanguage = 25;
    #[c2rust::src_loc = "106:3"]
    pub const LLVMDWARFSourceLanguageC_plus_plus_03: LLVMDWARFSourceLanguage = 24;
    #[c2rust::src_loc = "105:3"]
    pub const LLVMDWARFSourceLanguageHaskell: LLVMDWARFSourceLanguage = 23;
    #[c2rust::src_loc = "104:3"]
    pub const LLVMDWARFSourceLanguageModula3: LLVMDWARFSourceLanguage = 22;
    #[c2rust::src_loc = "103:3"]
    pub const LLVMDWARFSourceLanguageGo: LLVMDWARFSourceLanguage = 21;
    #[c2rust::src_loc = "102:3"]
    pub const LLVMDWARFSourceLanguageOpenCL: LLVMDWARFSourceLanguage = 20;
    #[c2rust::src_loc = "100:3"]
    pub const LLVMDWARFSourceLanguagePython: LLVMDWARFSourceLanguage = 19;
    #[c2rust::src_loc = "98:3"]
    pub const LLVMDWARFSourceLanguageD: LLVMDWARFSourceLanguage = 18;
    #[c2rust::src_loc = "97:3"]
    pub const LLVMDWARFSourceLanguageUPC: LLVMDWARFSourceLanguage = 17;
    #[c2rust::src_loc = "96:3"]
    pub const LLVMDWARFSourceLanguageObjC_plus_plus: LLVMDWARFSourceLanguage = 16;
    #[c2rust::src_loc = "95:3"]
    pub const LLVMDWARFSourceLanguageObjC: LLVMDWARFSourceLanguage = 15;
    #[c2rust::src_loc = "94:3"]
    pub const LLVMDWARFSourceLanguagePLI: LLVMDWARFSourceLanguage = 14;
    #[c2rust::src_loc = "93:3"]
    pub const LLVMDWARFSourceLanguageFortran95: LLVMDWARFSourceLanguage = 13;
    #[c2rust::src_loc = "92:3"]
    pub const LLVMDWARFSourceLanguageAda95: LLVMDWARFSourceLanguage = 12;
    #[c2rust::src_loc = "91:3"]
    pub const LLVMDWARFSourceLanguageC99: LLVMDWARFSourceLanguage = 11;
    #[c2rust::src_loc = "90:3"]
    pub const LLVMDWARFSourceLanguageJava: LLVMDWARFSourceLanguage = 10;
    #[c2rust::src_loc = "88:3"]
    pub const LLVMDWARFSourceLanguageModula2: LLVMDWARFSourceLanguage = 9;
    #[c2rust::src_loc = "87:3"]
    pub const LLVMDWARFSourceLanguagePascal83: LLVMDWARFSourceLanguage = 8;
    #[c2rust::src_loc = "86:3"]
    pub const LLVMDWARFSourceLanguageFortran90: LLVMDWARFSourceLanguage = 7;
    #[c2rust::src_loc = "85:3"]
    pub const LLVMDWARFSourceLanguageFortran77: LLVMDWARFSourceLanguage = 6;
    #[c2rust::src_loc = "84:3"]
    pub const LLVMDWARFSourceLanguageCobol85: LLVMDWARFSourceLanguage = 5;
    #[c2rust::src_loc = "83:3"]
    pub const LLVMDWARFSourceLanguageCobol74: LLVMDWARFSourceLanguage = 4;
    #[c2rust::src_loc = "82:3"]
    pub const LLVMDWARFSourceLanguageC_plus_plus: LLVMDWARFSourceLanguage = 3;
    #[c2rust::src_loc = "81:3"]
    pub const LLVMDWARFSourceLanguageAda83: LLVMDWARFSourceLanguage = 2;
    #[c2rust::src_loc = "80:3"]
    pub const LLVMDWARFSourceLanguageC: LLVMDWARFSourceLanguage = 1;
    #[c2rust::src_loc = "79:3"]
    pub const LLVMDWARFSourceLanguageC89: LLVMDWARFSourceLanguage = 0;
    #[c2rust::src_loc = "128:9"]
    pub type LLVMDWARFEmissionKind = libc::c_uint;
    #[c2rust::src_loc = "131:5"]
    pub const LLVMDWARFEmissionLineTablesOnly: LLVMDWARFEmissionKind = 2;
    #[c2rust::src_loc = "130:5"]
    pub const LLVMDWARFEmissionFull: LLVMDWARFEmissionKind = 1;
    #[c2rust::src_loc = "129:5"]
    pub const LLVMDWARFEmissionNone: LLVMDWARFEmissionKind = 0;
    use super::Types_h::{
        LLVMBool, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
    };
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "275:1"]
        pub fn LLVMDIBuilderCreateCompileUnit(
            Builder: LLVMDIBuilderRef,
            Lang: LLVMDWARFSourceLanguage,
            FileRef: LLVMMetadataRef,
            Producer: *const libc::c_char,
            ProducerLen: size_t,
            isOptimized: LLVMBool,
            Flags: *const libc::c_char,
            FlagsLen: size_t,
            RuntimeVer: libc::c_uint,
            SplitName: *const libc::c_char,
            SplitNameLen: size_t,
            Kind: LLVMDWARFEmissionKind,
            DWOId: libc::c_uint,
            SplitDebugInlining: LLVMBool,
            DebugInfoForProfiling: LLVMBool,
            SysRoot: *const libc::c_char,
            SysRootLen: size_t,
            SDK: *const libc::c_char,
            SDKLen: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "292:1"]
        pub fn LLVMDIBuilderCreateFile(
            Builder: LLVMDIBuilderRef,
            Filename: *const libc::c_char,
            FilenameLen: size_t,
            Directory: *const libc::c_char,
            DirectoryLen: size_t,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "366:1"]
        pub fn LLVMDIBuilderCreateLexicalBlock(
            Builder: LLVMDIBuilderRef,
            Scope: LLVMMetadataRef,
            File: LLVMMetadataRef,
            Line: libc::c_uint,
            Column: libc::c_uint,
        ) -> LLVMMetadataRef;
        #[c2rust::src_loc = "459:1"]
        pub fn LLVMDIBuilderCreateDebugLocation(
            Ctx: LLVMContextRef,
            Line: libc::c_uint,
            Column: libc::c_uint,
            Scope: LLVMMetadataRef,
            InlinedAt: LLVMMetadataRef,
        ) -> LLVMMetadataRef;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/ds/hash.h:1"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct hashmap_t {
        pub count: size_t,
        pub size: size_t,
        pub item_bitmap: *mut bitmap_t,
        pub buckets: *mut hashmap_entry_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:16"]
    pub struct hashmap_entry_t {
        pub ptr: *mut libc::c_void,
        pub hash: size_t,
    }
    #[c2rust::src_loc = "16:1"]
    pub type bitmap_t = size_t;
    use super::_size_t_h::size_t;
    use super::fun_h::{cmp_fn, free_fn};
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn ponyint_hashmap_init(map: *mut hashmap_t, size: size_t);
        #[c2rust::src_loc = "56:1"]
        pub fn ponyint_hashmap_destroy(map: *mut hashmap_t, free_elem: free_fn);
        #[c2rust::src_loc = "60:1"]
        pub fn ponyint_hashmap_optimize(map: *mut hashmap_t, cmp: cmp_fn);
        #[c2rust::src_loc = "66:1"]
        pub fn ponyint_hashmap_get(
            map: *mut hashmap_t,
            key: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
            index: *mut size_t,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "73:1"]
        pub fn ponyint_hashmap_put(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "79:1"]
        pub fn ponyint_hashmap_putindex(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
            index: size_t,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn ponyint_hashmap_remove(
            map: *mut hashmap_t,
            entry: *mut libc::c_void,
            hash: size_t,
            cmp: cmp_fn,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "93:1"]
        pub fn ponyint_hashmap_removeindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "99:1"]
        pub fn ponyint_hashmap_clearindex(map: *mut hashmap_t, index: size_t);
        #[c2rust::src_loc = "103:1"]
        pub fn ponyint_hashmap_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "107:1"]
        pub fn ponyint_hashmap_fill_ratio(map: *mut hashmap_t) -> libc::c_double;
        #[c2rust::src_loc = "111:1"]
        pub fn ponyint_hashmap_mem_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "115:1"]
        pub fn ponyint_hashmap_alloc_size(map: *mut hashmap_t) -> size_t;
        #[c2rust::src_loc = "121:1"]
        pub fn ponyint_hashmap_next(
            i: *mut size_t,
            count: size_t,
            item_bitmap: *mut bitmap_t,
            size: size_t,
            buckets: *mut hashmap_entry_t,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/error.h:1"]
pub mod error_h {
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type errors_t;
        #[c2rust::src_loc = "102:1"]
        pub fn errorf(
            errors: *mut errors_t,
            file: *const libc::c_char,
            fmt: *const libc::c_char,
            _: ...
        );
    }
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
        pub fn ponyint_hash_ptr(p: *const libc::c_void) -> size_t;
    }
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
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "9:23"]
        pub type strlist_t;
        #[c2rust::src_loc = "9:23"]
        pub fn strlist_next(list: *mut strlist_t) -> *mut strlist_t;
        #[c2rust::src_loc = "9:34"]
        pub fn strlist_data(list: *mut strlist_t) -> *const libc::c_char;
        #[c2rust::src_loc = "14:1"]
        pub fn stringtab(string: *const libc::c_char) -> *const libc::c_char;
        #[c2rust::src_loc = "20:1"]
        pub fn stringtab_consume(
            string: *const libc::c_char,
            buf_size: size_t,
        ) -> *const libc::c_char;
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
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn deferred_reify_free(deferred: *mut deferred_reification_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/reach.h:1"]
pub mod reach_h {
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
    use super::_uint32_t_h::uint32_t;
    use super::hash_h::hashmap_t;
    use super::pass_h::pass_opt_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "18:35"]
        pub type reach_method_stack_t;
        #[c2rust::src_loc = "123:1"]
        pub fn reach_new() -> *mut reach_t;
        #[c2rust::src_loc = "126:1"]
        pub fn reach_free(r: *mut reach_t);
        #[c2rust::src_loc = "133:1"]
        pub fn reach(
            r: *mut reach_t,
            type_0: *mut ast_t,
            name: *const libc::c_char,
            typeargs: *mut ast_t,
            opt: *mut pass_opt_t,
        );
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
    use super::TargetMachine_h::{LLVMTargetMachineRef, LLVMTargetRef};
    use super::Target_h::LLVMTargetDataRef;
    use super::Types_h::{
        LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMDIBuilderRef, LLVMMetadataRef,
        LLVMModuleRef, LLVMTypeRef,
        LLVMValueRef,
    };
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "237:1"]
        pub fn codegen_machine(target: LLVMTargetRef, opt: *mut pass_opt_t)
            -> LLVMTargetMachineRef;
        #[c2rust::src_loc = "74:1"]
        pub fn ffi_decls_destroy(map: *mut ffi_decls_t);
        #[c2rust::src_loc = "74:1"]
        pub fn ffi_decls_init(map: *mut ffi_decls_t, size: size_t);
        #[c2rust::src_loc = "68:1"]
        pub fn genned_strings_destroy(map: *mut genned_strings_t);
        #[c2rust::src_loc = "68:1"]
        pub fn genned_strings_init(map: *mut genned_strings_t, size: size_t);
        #[c2rust::src_loc = "43:1"]
        pub fn LLVMBuildCall_P(
            B: LLVMBuilderRef,
            Fn: LLVMValueRef,
            Args: *mut LLVMValueRef,
            NumArgs: libc::c_uint,
            Name: *const libc::c_char,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "37:1"]
        pub fn LLVMConstInBoundsGEP_P(
            ConstantVal: LLVMValueRef,
            ConstantIndices: *mut LLVMValueRef,
            NumIndices: libc::c_uint,
        ) -> LLVMValueRef;
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMSetMetadataStr(val: LLVMValueRef, str: *const libc::c_char, node: LLVMValueRef);
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMParseIRFileInContext(
            ctx: LLVMContextRef,
            file: *const libc::c_char,
        ) -> LLVMModuleRef;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:1"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "40:7"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "42:7"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "43:7"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/common/paths.h:1"]
pub mod paths_h {
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn pony_mkdir(path: *const libc::c_char);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm/Config/Targets.def:1"]
pub mod Targets_def {
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn LLVMInitializeX86TargetInfo();
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMInitializeARMTargetInfo();
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMInitializeAArch64TargetInfo();
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMInitializeWebAssemblyTargetInfo();
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMInitializeRISCVTargetInfo();
        #[c2rust::src_loc = "26:1"]
        pub fn LLVMInitializeX86Target();
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMInitializeARMTarget();
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMInitializeAArch64Target();
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMInitializeWebAssemblyTarget();
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMInitializeRISCVTarget();
        #[c2rust::src_loc = "26:1"]
        pub fn LLVMInitializeX86TargetMC();
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMInitializeARMTargetMC();
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMInitializeAArch64TargetMC();
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMInitializeWebAssemblyTargetMC();
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMInitializeRISCVTargetMC();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm/Config/AsmPrinters.def:1"]
pub mod AsmPrinters_def {
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMInitializeX86AsmPrinter();
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMInitializeARMAsmPrinter();
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMInitializeAArch64AsmPrinter();
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMInitializeWebAssemblyAsmPrinter();
        #[c2rust::src_loc = "31:1"]
        pub fn LLVMInitializeRISCVAsmPrinter();
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm/Config/AsmParsers.def:1"]
pub mod AsmParsers_def {
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn LLVMInitializeX86AsmParser();
        #[c2rust::src_loc = "28:1"]
        pub fn LLVMInitializeARMAsmParser();
        #[c2rust::src_loc = "29:1"]
        pub fn LLVMInitializeAArch64AsmParser();
        #[c2rust::src_loc = "30:1"]
        pub fn LLVMInitializeWebAssemblyAsmParser();
        #[c2rust::src_loc = "31:1"]
        pub fn LLVMInitializeRISCVAsmParser();
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
        #[c2rust::src_loc = "175:6"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "344:6"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendebug.h:1"]
pub mod gendebug_h {
    use super::Types_h::{LLVMDIBuilderRef, LLVMModuleRef};
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn LLVMNewDIBuilder(m: LLVMModuleRef) -> LLVMDIBuilderRef;
        #[c2rust::src_loc = "48:1"]
        pub fn LLVMDIBuilderDestroy(d: LLVMDIBuilderRef);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/ast/ast.h:1"]
pub mod ast_h {
    use super::_size_t_h::size_t;
    use super::symtab_h::{ast_t, sym_status_t};
    extern "C" {
        #[c2rust::src_loc = "74:1"]
        pub fn ast_line(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "75:1"]
        pub fn ast_pos(ast: *mut ast_t) -> size_t;
        #[c2rust::src_loc = "112:1"]
        pub fn ast_child(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "116:1"]
        pub fn ast_sibling(ast: *mut ast_t) -> *mut ast_t;
        #[c2rust::src_loc = "120:1"]
        pub fn ast_get(
            ast: *mut ast_t,
            name: *const libc::c_char,
            status: *mut sym_status_t,
        ) -> *mut ast_t;
        #[c2rust::src_loc = "146:1"]
        pub fn ast_free(ast: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genexe.h:2"]
pub mod genexe_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn genexe(c: *mut compile_t, program: *mut ast_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genprim.h:3"]
pub mod genprim_h {
    use super::codegen_h::compile_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn genprim_reachable_init(c: *mut compile_t, program: *mut ast_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gentype.h:3"]
pub mod gentype_h {
    use super::codegen_h::compile_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn gentypes(c: *mut compile_t) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genname.h:4"]
pub mod genname_h {
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn genname_descriptor(type_0: *const libc::c_char) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gendesc.h:5"]
pub mod gendesc_h {
    use super::codegen_h::compile_t;
    use super::Types_h::{LLVMTypeRef};
    extern "C" {
        #[c2rust::src_loc = "9:1"]
        pub fn gendesc_basetype(c: *mut compile_t, desc_type: LLVMTypeRef);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/gencall.h:6"]
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
        #[c2rust::src_loc = "38:1"]
        pub fn gencall_lifetime_start(c: *mut compile_t, ptr: LLVMValueRef);
        #[c2rust::src_loc = "40:1"]
        pub fn gencall_lifetime_end(c: *mut compile_t, ptr: LLVMValueRef);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/codegen/genopt.h:7"]
pub mod genopt_h {
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn target_is_ilp32(triple: *mut libc::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/pkg/package.h:9"]
pub mod package_h {
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn path_cat(
            part1: *const libc::c_char,
            part2: *const libc::c_char,
            result: *mut libc::c_char,
        );
        #[c2rust::src_loc = "121:1"]
        pub fn package_path(package: *mut ast_t) -> *const libc::c_char;
        #[c2rust::src_loc = "131:1"]
        pub fn package_filename(package: *mut ast_t) -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/reach/paint.h:10"]
pub mod paint_h {
    use super::reach_h::reach_types_t;
    extern "C" {
        #[c2rust::src_loc = "13:1"]
        pub fn paint(types: *mut reach_types_t);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/assemble.h:11"]
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
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyc/type/lookup.h:12"]
pub mod lookup_h {
    use super::pass_h::pass_opt_t;
    use super::reify_h::deferred_reification_t;
    use super::symtab_h::ast_t;
    extern "C" {
        #[c2rust::src_loc = "11:1"]
        pub fn lookup(
            opt: *mut pass_opt_t,
            from: *mut ast_t,
            type_0: *mut ast_t,
            name: *const libc::c_char,
        ) -> *mut deferred_reification_t;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/src/libponyrt/mem/pool.h:13"]
pub mod pool_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn ponyint_pool_free_size(size: size_t, p: *mut libc::c_void);
        #[c2rust::src_loc = "27:22"]
        pub fn ponyint_pool_alloc_size(size: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "24:22"]
        pub fn ponyint_pool_alloc(index: size_t) -> *mut libc::c_void;
        #[c2rust::src_loc = "25:1"]
        pub fn ponyint_pool_free(index: size_t, p: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Initialization.h:20"]
pub mod Initialization_h {
    use super::Types_h::{LLVMPassRegistryRef};
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn LLVMInitializeTransformUtils(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMInitializeScalarOpts(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "36:1"]
        pub fn LLVMInitializeObjCARCOpts(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "37:1"]
        pub fn LLVMInitializeVectorization(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "38:1"]
        pub fn LLVMInitializeInstCombine(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "40:1"]
        pub fn LLVMInitializeIPO(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "41:1"]
        pub fn LLVMInitializeInstrumentation(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "42:1"]
        pub fn LLVMInitializeAnalysis(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "43:1"]
        pub fn LLVMInitializeIPA(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "44:1"]
        pub fn LLVMInitializeCodeGen(R: LLVMPassRegistryRef);
        #[c2rust::src_loc = "45:1"]
        pub fn LLVMInitializeTarget(R: LLVMPassRegistryRef);
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Linker.h:21"]
pub mod Linker_h {
    use super::Types_h::{LLVMBool, LLVMModuleRef};
    extern "C" {
        #[c2rust::src_loc = "41:1"]
        pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;
    }
}
#[c2rust::header_src = "/Users/dantebroggi/Documents/GitHub/ponyc/build/libs/include/llvm-c/Support.h:22"]
pub mod Support_h {
    use super::Types_h::LLVMBool;
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn LLVMLoadLibraryPermanently(Filename: *const libc::c_char) -> LLVMBool;
        #[c2rust::src_loc = "45:1"]
        pub fn LLVMParseCommandLineOptions(
            argc: libc::c_int,
            argv: *const *const libc::c_char,
            Overview: *const libc::c_char,
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:23"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "74:7"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "82:9"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "85:7"]
        pub fn strncpy(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;
        #[c2rust::src_loc = "90:7"]
        pub fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_off_t_h::off_t;
pub use self::_size_t_h::size_t;
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_size_t, __darwin_time_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
use self::assemble_h::type_builtin;
use self::ast_h::{ast_child, ast_free, ast_get, ast_line, ast_pos, ast_sibling};
use self::gencall_h::{gencall_lifetime_end, gencall_lifetime_start, gencall_runtime};
use self::gendebug_h::{LLVMDIBuilderDestroy, LLVMNewDIBuilder};
use self::gendesc_h::gendesc_basetype;
use self::genexe_h::genexe;
use self::genname_h::genname_descriptor;
use self::genopt_h::target_is_ilp32;
use self::genprim_h::genprim_reachable_init;
use self::gentype_h::gentypes;
use self::lookup_h::lookup;
use self::package_h::{package_filename, package_path, path_cat};
use self::paint_h::paint;
use self::pool_h::{
    ponyint_pool_alloc, ponyint_pool_alloc_size, ponyint_pool_free, ponyint_pool_free_size,
};
pub use self::stat_h::stat;
use self::stdio_h::{__stderrp, fprintf, printf, snprintf};
use self::string_h::{memset, strlen, strncpy, strtok};
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t,
};

pub use self::Core_h::{
    C2RustUnnamed, LLVMAMDGPUCSCallConv, LLVMAMDGPUESCallConv, LLVMAMDGPUGSCallConv,
    LLVMAMDGPUHSCallConv, LLVMAMDGPUKERNELCallConv, LLVMAMDGPULSCallConv, LLVMAMDGPUPSCallConv,
    LLVMAMDGPUVSCallConv, LLVMARMAAPCSCallConv, LLVMARMAAPCSVFPCallConv, LLVMARMAPCSCallConv,
    LLVMAVRBUILTINCallConv, LLVMAVRINTRCallConv, LLVMAVRSIGNALCallConv, LLVMAddAttributeAtIndex,
    LLVMAddFunction, LLVMAddGlobal, LLVMAnyRegCallConv, LLVMAppendBasicBlockInContext,
    LLVMAppendingLinkage, LLVMArrayType, LLVMArrayTypeKind, LLVMAttributeFunctionIndex,
    LLVMAttributeIndex, LLVMAttributeReturnIndex, LLVMAvailableExternallyLinkage,
    LLVMBFloatTypeKind, LLVMCCallConv, LLVMCXXFASTTLSCallConv, LLVMCallConv, LLVMColdCallConv,
    LLVMCommonLinkage, LLVMConstInt, LLVMConstStringInContext, LLVMContextCreate,
    LLVMContextDispose, LLVMCountBasicBlocks, LLVMCreateBuilderInContext, LLVMCreateEnumAttribute,
    LLVMCreateMessage, LLVMDLLExportLinkage, LLVMDLLImportLinkage, LLVMDisposeBuilder,
    LLVMDisposeMessage, LLVMDisposeModule, LLVMDoubleTypeInContext, LLVMDoubleTypeKind,
    LLVMExternalLinkage, LLVMExternalWeakLinkage, LLVMFP128TypeKind, LLVMFastCallConv,
    LLVMFloatTypeInContext, LLVMFloatTypeKind, LLVMFunctionType, LLVMFunctionTypeKind,
    LLVMGHCCallConv, LLVMGetElementType, LLVMGetEntryBasicBlock, LLVMGetEnumAttributeKindForName,
    LLVMGetFirstInstruction, LLVMGetFirstParam, LLVMGetGlobalPassRegistry, LLVMGetInsertBlock,
    LLVMGetNextParam, LLVMGetTypeKind, LLVMGhostLinkage, LLVMHHVMCCallConv, LLVMHHVMCallConv,
    LLVMHalfTypeKind, LLVMHiPECallConv, LLVMInitializeCore, LLVMInt16TypeInContext,
    LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext,
    LLVMIntTypeInContext, LLVMIntegerTypeKind, LLVMIntelOCLBICallConv, LLVMInternalLinkage,
    LLVMLabelTypeKind, LLVMLinkOnceAnyLinkage, LLVMLinkOnceODRAutoHideLinkage,
    LLVMLinkOnceODRLinkage, LLVMLinkage, LLVMLinkerPrivateLinkage, LLVMLinkerPrivateWeakLinkage,
    LLVMMDNodeInContext, LLVMMSP430BUILTINCallConv, LLVMMSP430INTRCallConv, LLVMMetadataTypeKind,
    LLVMModuleCreateWithNameInContext, LLVMPPC_FP128TypeKind, LLVMPTXDeviceCallConv,
    LLVMPTXKernelCallConv, LLVMPointerType, LLVMPointerTypeKind, LLVMPositionBuilderAtEnd,
    LLVMPositionBuilderBefore, LLVMPreserveAllCallConv, LLVMPreserveMostCallConv,
    LLVMPrivateLinkage, LLVMSPIRFUNCCallConv, LLVMSPIRKERNELCallConv, LLVMScalableVectorTypeKind,
    LLVMSetCurrentDebugLocation2, LLVMSetDataLayout, LLVMSetFunctionCallConv,
    LLVMSetGlobalConstant, LLVMSetInitializer, LLVMSetInstructionCallConv, LLVMSetLinkage,
    LLVMSetTarget, LLVMSetUnnamedAddr, LLVMShutdown, LLVMStructCreateNamed, LLVMStructSetBody,
    LLVMStructTypeInContext, LLVMStructTypeKind, LLVMSwiftCallConv, LLVMTokenTypeKind,
    LLVMTypeKind, LLVMTypeOf, LLVMVectorTypeKind, LLVMVoidTypeInContext, LLVMVoidTypeKind,
    LLVMWeakAnyLinkage, LLVMWeakODRLinkage, LLVMWebKitJSCallConv, LLVMWin64CallConv,
    LLVMX8664SysVCallConv, LLVMX86FastcallCallConv, LLVMX86INTRCallConv, LLVMX86RegCallCallConv,
    LLVMX86StdcallCallConv, LLVMX86ThisCallCallConv, LLVMX86VectorCallCallConv,
    LLVMX86_AMXTypeKind, LLVMX86_FP80TypeKind, LLVMX86_MMXTypeKind,
};
pub use self::ErrorHandling_h::{
    LLVMEnablePrettyStackTrace, LLVMFatalErrorHandler, LLVMInstallFatalErrorHandler,
    LLVMResetFatalErrorHandler,
};
use self::Initialization_h::{
    LLVMInitializeAnalysis, LLVMInitializeCodeGen, LLVMInitializeIPA, LLVMInitializeIPO,
    LLVMInitializeInstCombine, LLVMInitializeInstrumentation, LLVMInitializeObjCARCOpts,
    LLVMInitializeScalarOpts, LLVMInitializeTarget, LLVMInitializeTransformUtils,
    LLVMInitializeVectorization,
};
use self::Linker_h::LLVMLinkModules2;
use self::Support_h::{LLVMLoadLibraryPermanently, LLVMParseCommandLineOptions};
pub use self::TargetMachine_h::{
    LLVMCreateTargetDataLayout, LLVMDisposeTargetMachine, LLVMGetHostCPUFeatures,
    LLVMGetHostCPUName, LLVMGetTargetFromTriple, LLVMNormalizeTargetTriple,
    LLVMOpaqueTargetMachine, LLVMTarget, LLVMTargetMachineRef, LLVMTargetRef,
};
pub use self::Target_h::{
    LLVMABISizeOfType, LLVMCopyStringRepOfTargetData, LLVMDisposeTargetData,
    LLVMInitializeAllAsmParsers, LLVMInitializeAllAsmPrinters, LLVMInitializeAllTargetInfos,
    LLVMInitializeAllTargetMCs, LLVMInitializeAllTargets, LLVMInitializeNativeTarget,
    LLVMIntPtrTypeInContext, LLVMOpaqueTargetData, LLVMTargetDataRef,
};
pub use self::Types_h::{
    LLVMAttributeRef, LLVMBasicBlockRef, LLVMBool, LLVMBuilderRef, LLVMContextRef,
    LLVMDIBuilderRef, LLVMMetadataRef, LLVMModuleRef, LLVMOpaqueAttributeRef, LLVMOpaqueBasicBlock,
    LLVMOpaqueBuilder, LLVMOpaqueContext, LLVMOpaqueDIBuilder, LLVMOpaqueMetadata,
    LLVMOpaqueModule, LLVMOpaquePassRegistry, LLVMOpaqueType, LLVMOpaqueValue, LLVMPassRegistryRef,
    LLVMTypeRef, LLVMValueRef,
};
use self::_malloc_h::{free, malloc, realloc};
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fpos_t, FILE};
pub use self::codegen_h::{
    codegen_machine, compile_frame_t, compile_locals_t, compile_t, ffi_decls_destroy,
    ffi_decls_init, ffi_decls_t, genned_strings_destroy, genned_strings_init, genned_strings_t,
    LLVMBuildCall_P, LLVMConstInBoundsGEP_P, LLVMParseIRFileInContext, LLVMSetMetadataStr,
};
use self::error_h::{errorf, errors_t};
pub use self::frame_h::{typecheck_frame_t, typecheck_stats_t, typecheck_t};
pub use self::fun_h::{cmp_fn, free_fn, ponyint_hash_ptr};
pub use self::hash_h::{
    bitmap_t, hashmap_entry_t, hashmap_t, ponyint_hashmap_alloc_size, ponyint_hashmap_clearindex,
    ponyint_hashmap_destroy, ponyint_hashmap_fill_ratio, ponyint_hashmap_get, ponyint_hashmap_init,
    ponyint_hashmap_mem_size, ponyint_hashmap_next, ponyint_hashmap_optimize, ponyint_hashmap_put,
    ponyint_hashmap_putindex, ponyint_hashmap_remove, ponyint_hashmap_removeindex,
    ponyint_hashmap_size,
};
pub use self::pass_h::{
    magic_package_t, pass_id, pass_opt_t, plugins_t, verbosity_level, PASS_ALL, PASS_ASM,
    PASS_BITCODE, PASS_COMPLETENESS, PASS_DOCS, PASS_EXPR, PASS_FINALISER, PASS_FLATTEN,
    PASS_IMPORT, PASS_LLVM_IR, PASS_NAME_RESOLUTION, PASS_OBJ, PASS_PAINT, PASS_PARSE, PASS_REACH,
    PASS_REFER, PASS_SCOPE, PASS_SERIALISER, PASS_SUGAR, PASS_SYNTAX, PASS_TRAITS, PASS_VERIFY,
    VERBOSITY_ALL, VERBOSITY_INFO, VERBOSITY_MINIMAL, VERBOSITY_QUIET, VERBOSITY_TOOL_INFO,
};
use self::paths_h::pony_mkdir;
pub use self::reach_h::{
    reach, reach_free, reach_method_stack_t, reach_new, reach_t, reach_types_t,
};
pub use self::reify_h::{deferred_reification_t, deferred_reify_free};
use self::stringtab_h::{stringtab, stringtab_consume, strlist_data, strlist_next, strlist_t};
pub use self::symtab_h::{
    ast_t, sym_status_t, SYM_CONSUMED, SYM_CONSUMED_SAME_EXPR, SYM_DEFINED, SYM_ERROR, SYM_FFIDECL,
    SYM_NOCASE, SYM_NONE, SYM_UNDEFINED,
};

pub use self::DebugInfo_h::{
    LLVMDIBuilderCreateCompileUnit, LLVMDIBuilderCreateDebugLocation, LLVMDIBuilderCreateFile,
    LLVMDIBuilderCreateLexicalBlock, LLVMDWARFEmissionFull, LLVMDWARFEmissionKind,
    LLVMDWARFEmissionLineTablesOnly, LLVMDWARFEmissionNone, LLVMDWARFSourceLanguage,
    LLVMDWARFSourceLanguageAda83, LLVMDWARFSourceLanguageAda95, LLVMDWARFSourceLanguageBLISS,
    LLVMDWARFSourceLanguageBORLAND_Delphi, LLVMDWARFSourceLanguageC, LLVMDWARFSourceLanguageC11,
    LLVMDWARFSourceLanguageC89, LLVMDWARFSourceLanguageC99, LLVMDWARFSourceLanguageC_plus_plus,
    LLVMDWARFSourceLanguageC_plus_plus_03, LLVMDWARFSourceLanguageC_plus_plus_11,
    LLVMDWARFSourceLanguageC_plus_plus_14, LLVMDWARFSourceLanguageCobol74,
    LLVMDWARFSourceLanguageCobol85, LLVMDWARFSourceLanguageD, LLVMDWARFSourceLanguageDylan,
    LLVMDWARFSourceLanguageFortran03, LLVMDWARFSourceLanguageFortran08,
    LLVMDWARFSourceLanguageFortran77, LLVMDWARFSourceLanguageFortran90,
    LLVMDWARFSourceLanguageFortran95, LLVMDWARFSourceLanguageGOOGLE_RenderScript,
    LLVMDWARFSourceLanguageGo, LLVMDWARFSourceLanguageHaskell, LLVMDWARFSourceLanguageJava,
    LLVMDWARFSourceLanguageJulia, LLVMDWARFSourceLanguageMips_Assembler,
    LLVMDWARFSourceLanguageModula2, LLVMDWARFSourceLanguageModula3, LLVMDWARFSourceLanguageOCaml,
    LLVMDWARFSourceLanguageObjC, LLVMDWARFSourceLanguageObjC_plus_plus,
    LLVMDWARFSourceLanguageOpenCL, LLVMDWARFSourceLanguagePLI, LLVMDWARFSourceLanguagePascal83,
    LLVMDWARFSourceLanguagePython, LLVMDWARFSourceLanguageRenderScript,
    LLVMDWARFSourceLanguageRust, LLVMDWARFSourceLanguageSwift, LLVMDWARFSourceLanguageUPC,
};

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "25:8"]
pub struct compile_local_t {
    pub name: *const libc::c_char,
    pub alloca: LLVMValueRef,
    pub alive: bool,
}
#[c2rust::src_loc = "47:1"]
pub type compile_locals_free_fn = Option<unsafe extern "C" fn(*mut compile_local_t) -> ()>;
#[c2rust::src_loc = "47:1"]
pub type compile_locals_cmp_fn =
    Option<unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool>;
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn compile_local_hash(mut p: *mut compile_local_t) -> size_t {
    ponyint_hash_ptr((*p).name as *const libc::c_void)
}
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn compile_local_cmp(
    mut a: *mut compile_local_t,
    mut b: *mut compile_local_t,
) -> bool {
    (*a).name == (*b).name
}
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn compile_local_free(mut p: *mut compile_local_t) {
    ponyint_pool_free(0 as libc::c_int as size_t, p as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_destroy(mut map: *mut compile_locals_t) {
    let mut freef: compile_locals_free_fn =
        Some(compile_local_free as unsafe extern "C" fn(*mut compile_local_t) -> ());
    ponyint_hashmap_destroy(
        map as *mut hashmap_t,
        ::core::mem::transmute::<compile_locals_free_fn, free_fn>(freef),
    );
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_init(mut map: *mut compile_locals_t, mut size: size_t) {
    ponyint_hashmap_init(map as *mut hashmap_t, size);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_removeindex(
    mut map: *mut compile_locals_t,
    mut index: size_t,
) {
    ponyint_hashmap_removeindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_alloc_size(mut map: *mut compile_locals_t) -> size_t {
    ponyint_hashmap_alloc_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_fill_ratio(mut map: *mut hashmap_t) -> libc::c_double {
    ponyint_hashmap_fill_ratio(map)
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_clearindex(
    mut map: *mut compile_locals_t,
    mut index: size_t,
) {
    ponyint_hashmap_clearindex(map as *mut hashmap_t, index);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_putindex(
    mut map: *mut compile_locals_t,
    mut entry: *mut compile_local_t,
    mut index: size_t,
) {
    let mut cmpf: compile_locals_cmp_fn = Some(
        compile_local_cmp
            as unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool,
    );
    ponyint_hashmap_putindex(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        compile_local_hash(entry),
        ::core::mem::transmute::<compile_locals_cmp_fn, cmp_fn>(cmpf),
        index,
    );
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_optimize(mut map: *mut compile_locals_t) {
    let mut cmpf: compile_locals_cmp_fn = Some(
        compile_local_cmp
            as unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool,
    );
    ponyint_hashmap_optimize(
        map as *mut hashmap_t,
        ::core::mem::transmute::<compile_locals_cmp_fn, cmp_fn>(cmpf),
    )
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_mem_size(mut map: *mut compile_locals_t) -> size_t {
    ponyint_hashmap_mem_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn compile_locals_size(mut map: *mut compile_locals_t) -> size_t {
    ponyint_hashmap_size(map as *mut hashmap_t)
}
#[no_mangle]
#[c2rust::src_loc = "47:50"]
pub unsafe extern "C" fn compile_locals_remove(
    mut map: *mut compile_locals_t,
    mut entry: *mut compile_local_t,
) -> *mut compile_local_t {
    let mut cmpf: compile_locals_cmp_fn = Some(
        compile_local_cmp
            as unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool,
    );
    ponyint_hashmap_remove(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        compile_local_hash(entry),
        ::core::mem::transmute::<compile_locals_cmp_fn, cmp_fn>(cmpf),
    ) as *mut compile_local_t
}
#[no_mangle]
#[c2rust::src_loc = "47:50"]
pub unsafe extern "C" fn compile_locals_next(
    mut map: *mut compile_locals_t,
    mut i: *mut size_t,
) -> *mut compile_local_t {
    let mut h: *mut hashmap_t = map as *mut hashmap_t;
    ponyint_hashmap_next(i, (*h).count, (*h).item_bitmap, (*h).size, (*h).buckets)
        as *mut compile_local_t
}
#[no_mangle]
#[c2rust::src_loc = "47:50"]
pub unsafe extern "C" fn compile_locals_put(
    mut map: *mut compile_locals_t,
    mut entry: *mut compile_local_t,
) -> *mut compile_local_t {
    let mut cmpf: compile_locals_cmp_fn = Some(
        compile_local_cmp
            as unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool,
    );
    ponyint_hashmap_put(
        map as *mut hashmap_t,
        entry as *mut libc::c_void,
        compile_local_hash(entry),
        ::core::mem::transmute::<compile_locals_cmp_fn, cmp_fn>(cmpf),
    ) as *mut compile_local_t
}
#[no_mangle]
#[c2rust::src_loc = "47:50"]
pub unsafe extern "C" fn compile_locals_get(
    mut map: *mut compile_locals_t,
    mut key: *mut compile_local_t,
    mut index: *mut size_t,
) -> *mut compile_local_t {
    let mut cmpf: compile_locals_cmp_fn = Some(
        compile_local_cmp
            as unsafe extern "C" fn(*mut compile_local_t, *mut compile_local_t) -> bool,
    );
    ponyint_hashmap_get(
        map as *mut hashmap_t,
        key as *mut libc::c_void,
        compile_local_hash(key),
        ::core::mem::transmute::<compile_locals_cmp_fn, cmp_fn>(cmpf),
        index,
    ) as *mut compile_local_t
}
#[c2rust::src_loc = "50:1"]
unsafe extern "C" fn fatal_error(mut reason: *const libc::c_char) {
    printf(
        b"LLVM fatal error: %s\n\0" as *const u8 as *const libc::c_char,
        reason,
    );
}
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn push_frame(mut c: *mut compile_t) -> *mut compile_frame_t {
    let mut frame: *mut compile_frame_t =
        ponyint_pool_alloc(2 as libc::c_int as size_t) as *mut compile_frame_t;
    memset(
        frame as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<compile_frame_t>() as libc::c_ulong,
    );
    compile_locals_init(&mut (*frame).locals, 0 as libc::c_int as size_t);
    if !((*c).frame).is_null() {
        let ref mut fresh0 = (*frame).prev;
        *fresh0 = (*c).frame;
    }
    let ref mut fresh1 = (*c).frame;
    *fresh1 = frame;
    frame
}
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn pop_frame(mut c: *mut compile_t) {
    let mut frame: *mut compile_frame_t = (*c).frame;
    compile_locals_destroy(&mut (*frame).locals);
    let ref mut fresh2 = (*c).frame;
    *fresh2 = (*frame).prev;
    ponyint_pool_free(2 as libc::c_int as size_t, frame as *mut libc::c_void);
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn make_machine(mut opt: *mut pass_opt_t) -> LLVMTargetMachineRef {
    let mut target: LLVMTargetRef = 0 as *mut LLVMTarget;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    if LLVMGetTargetFromTriple((*opt).triple, &mut target, &mut err) != 0 as libc::c_int {
        errorf(
            (*opt).check.errors,
            0 as *const libc::c_char,
            b"couldn't create target: %s\0" as *const u8 as *const libc::c_char,
            err,
        );
        LLVMDisposeMessage(err);
        return 0 as LLVMTargetMachineRef;
    }
    let mut machine: LLVMTargetMachineRef = codegen_machine(target, opt);
    if machine.is_null() {
        errorf(
            (*opt).check.errors,
            0 as *const libc::c_char,
            b"couldn't create target machine\0" as *const u8 as *const libc::c_char,
        );
        return 0 as LLVMTargetMachineRef;
    }
    machine
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn init_runtime(mut c: *mut compile_t) {
    let ref mut fresh3 = (*c).str_builtin;
    *fresh3 = stringtab(b"$0\0" as *const u8 as *const libc::c_char);
    let ref mut fresh4 = (*c).str_Bool;
    *fresh4 = stringtab(b"Bool\0" as *const u8 as *const libc::c_char);
    let ref mut fresh5 = (*c).str_I8;
    *fresh5 = stringtab(b"I8\0" as *const u8 as *const libc::c_char);
    let ref mut fresh6 = (*c).str_I16;
    *fresh6 = stringtab(b"I16\0" as *const u8 as *const libc::c_char);
    let ref mut fresh7 = (*c).str_I32;
    *fresh7 = stringtab(b"I32\0" as *const u8 as *const libc::c_char);
    let ref mut fresh8 = (*c).str_I64;
    *fresh8 = stringtab(b"I64\0" as *const u8 as *const libc::c_char);
    let ref mut fresh9 = (*c).str_I128;
    *fresh9 = stringtab(b"I128\0" as *const u8 as *const libc::c_char);
    let ref mut fresh10 = (*c).str_ILong;
    *fresh10 = stringtab(b"ILong\0" as *const u8 as *const libc::c_char);
    let ref mut fresh11 = (*c).str_ISize;
    *fresh11 = stringtab(b"ISize\0" as *const u8 as *const libc::c_char);
    let ref mut fresh12 = (*c).str_U8;
    *fresh12 = stringtab(b"U8\0" as *const u8 as *const libc::c_char);
    let ref mut fresh13 = (*c).str_U16;
    *fresh13 = stringtab(b"U16\0" as *const u8 as *const libc::c_char);
    let ref mut fresh14 = (*c).str_U32;
    *fresh14 = stringtab(b"U32\0" as *const u8 as *const libc::c_char);
    let ref mut fresh15 = (*c).str_U64;
    *fresh15 = stringtab(b"U64\0" as *const u8 as *const libc::c_char);
    let ref mut fresh16 = (*c).str_U128;
    *fresh16 = stringtab(b"U128\0" as *const u8 as *const libc::c_char);
    let ref mut fresh17 = (*c).str_ULong;
    *fresh17 = stringtab(b"ULong\0" as *const u8 as *const libc::c_char);
    let ref mut fresh18 = (*c).str_USize;
    *fresh18 = stringtab(b"USize\0" as *const u8 as *const libc::c_char);
    let ref mut fresh19 = (*c).str_F32;
    *fresh19 = stringtab(b"F32\0" as *const u8 as *const libc::c_char);
    let ref mut fresh20 = (*c).str_F64;
    *fresh20 = stringtab(b"F64\0" as *const u8 as *const libc::c_char);
    let ref mut fresh21 = (*c).str_Pointer;
    *fresh21 = stringtab(b"Pointer\0" as *const u8 as *const libc::c_char);
    let ref mut fresh22 = (*c).str_NullablePointer;
    *fresh22 = stringtab(b"NullablePointer\0" as *const u8 as *const libc::c_char);
    let ref mut fresh23 = (*c).str_DoNotOptimise;
    *fresh23 = stringtab(b"DoNotOptimise\0" as *const u8 as *const libc::c_char);
    let ref mut fresh24 = (*c).str_Array;
    *fresh24 = stringtab(b"Array\0" as *const u8 as *const libc::c_char);
    let ref mut fresh25 = (*c).str_String;
    *fresh25 = stringtab(b"String\0" as *const u8 as *const libc::c_char);
    let ref mut fresh26 = (*c).str_Platform;
    *fresh26 = stringtab(b"Platform\0" as *const u8 as *const libc::c_char);
    let ref mut fresh27 = (*c).str_Main;
    *fresh27 = stringtab(b"Main\0" as *const u8 as *const libc::c_char);
    let ref mut fresh28 = (*c).str_Env;
    *fresh28 = stringtab(b"Env\0" as *const u8 as *const libc::c_char);
    let ref mut fresh29 = (*c).str_add;
    *fresh29 = stringtab(b"add\0" as *const u8 as *const libc::c_char);
    let ref mut fresh30 = (*c).str_sub;
    *fresh30 = stringtab(b"sub\0" as *const u8 as *const libc::c_char);
    let ref mut fresh31 = (*c).str_mul;
    *fresh31 = stringtab(b"mul\0" as *const u8 as *const libc::c_char);
    let ref mut fresh32 = (*c).str_div;
    *fresh32 = stringtab(b"div\0" as *const u8 as *const libc::c_char);
    let ref mut fresh33 = (*c).str_rem;
    *fresh33 = stringtab(b"rem\0" as *const u8 as *const libc::c_char);
    let ref mut fresh34 = (*c).str_neg;
    *fresh34 = stringtab(b"neg\0" as *const u8 as *const libc::c_char);
    let ref mut fresh35 = (*c).str_add_unsafe;
    *fresh35 = stringtab(b"add_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh36 = (*c).str_sub_unsafe;
    *fresh36 = stringtab(b"sub_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh37 = (*c).str_mul_unsafe;
    *fresh37 = stringtab(b"mul_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh38 = (*c).str_div_unsafe;
    *fresh38 = stringtab(b"div_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh39 = (*c).str_rem_unsafe;
    *fresh39 = stringtab(b"rem_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh40 = (*c).str_neg_unsafe;
    *fresh40 = stringtab(b"neg_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh41 = (*c).str_and;
    *fresh41 = stringtab(b"op_and\0" as *const u8 as *const libc::c_char);
    let ref mut fresh42 = (*c).str_or;
    *fresh42 = stringtab(b"op_or\0" as *const u8 as *const libc::c_char);
    let ref mut fresh43 = (*c).str_xor;
    *fresh43 = stringtab(b"op_xor\0" as *const u8 as *const libc::c_char);
    let ref mut fresh44 = (*c).str_not;
    *fresh44 = stringtab(b"op_not\0" as *const u8 as *const libc::c_char);
    let ref mut fresh45 = (*c).str_shl;
    *fresh45 = stringtab(b"shl\0" as *const u8 as *const libc::c_char);
    let ref mut fresh46 = (*c).str_shr;
    *fresh46 = stringtab(b"shr\0" as *const u8 as *const libc::c_char);
    let ref mut fresh47 = (*c).str_shl_unsafe;
    *fresh47 = stringtab(b"shl_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh48 = (*c).str_shr_unsafe;
    *fresh48 = stringtab(b"shr_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh49 = (*c).str_eq;
    *fresh49 = stringtab(b"eq\0" as *const u8 as *const libc::c_char);
    let ref mut fresh50 = (*c).str_ne;
    *fresh50 = stringtab(b"ne\0" as *const u8 as *const libc::c_char);
    let ref mut fresh51 = (*c).str_lt;
    *fresh51 = stringtab(b"lt\0" as *const u8 as *const libc::c_char);
    let ref mut fresh52 = (*c).str_le;
    *fresh52 = stringtab(b"le\0" as *const u8 as *const libc::c_char);
    let ref mut fresh53 = (*c).str_ge;
    *fresh53 = stringtab(b"ge\0" as *const u8 as *const libc::c_char);
    let ref mut fresh54 = (*c).str_gt;
    *fresh54 = stringtab(b"gt\0" as *const u8 as *const libc::c_char);
    let ref mut fresh55 = (*c).str_eq_unsafe;
    *fresh55 = stringtab(b"eq_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh56 = (*c).str_ne_unsafe;
    *fresh56 = stringtab(b"ne_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh57 = (*c).str_lt_unsafe;
    *fresh57 = stringtab(b"lt_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh58 = (*c).str_le_unsafe;
    *fresh58 = stringtab(b"le_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh59 = (*c).str_ge_unsafe;
    *fresh59 = stringtab(b"ge_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh60 = (*c).str_gt_unsafe;
    *fresh60 = stringtab(b"gt_unsafe\0" as *const u8 as *const libc::c_char);
    let ref mut fresh61 = (*c).str_this;
    *fresh61 = stringtab(b"this\0" as *const u8 as *const libc::c_char);
    let ref mut fresh62 = (*c).str_create;
    *fresh62 = stringtab(b"create\0" as *const u8 as *const libc::c_char);
    let ref mut fresh63 = (*c).str__create;
    *fresh63 = stringtab(b"_create\0" as *const u8 as *const libc::c_char);
    let ref mut fresh64 = (*c).str__init;
    *fresh64 = stringtab(b"_init\0" as *const u8 as *const libc::c_char);
    let ref mut fresh65 = (*c).str__final;
    *fresh65 = stringtab(b"_final\0" as *const u8 as *const libc::c_char);
    let ref mut fresh66 = (*c).str__event_notify;
    *fresh66 = stringtab(b"_event_notify\0" as *const u8 as *const libc::c_char);
    let ref mut fresh67 = (*c).str__serialise_space;
    *fresh67 = stringtab(b"_serialise_space\0" as *const u8 as *const libc::c_char);
    let ref mut fresh68 = (*c).str__serialise;
    *fresh68 = stringtab(b"_serialise\0" as *const u8 as *const libc::c_char);
    let ref mut fresh69 = (*c).str__deserialise;
    *fresh69 = stringtab(b"_deserialise\0" as *const u8 as *const libc::c_char);
    let mut type_0: LLVMTypeRef = 0 as *mut LLVMOpaqueType;
    let mut params: [LLVMTypeRef; 5] = [0 as *mut LLVMOpaqueType; 5];
    let mut value: LLVMValueRef = 0 as *mut LLVMOpaqueValue;
    let ref mut fresh70 = (*c).void_type;
    *fresh70 = LLVMVoidTypeInContext((*c).context);
    let ref mut fresh71 = (*c).i1;
    *fresh71 = LLVMInt1TypeInContext((*c).context);
    let ref mut fresh72 = (*c).i8_0;
    *fresh72 = LLVMInt8TypeInContext((*c).context);
    let ref mut fresh73 = (*c).i16_0;
    *fresh73 = LLVMInt16TypeInContext((*c).context);
    let ref mut fresh74 = (*c).i32_0;
    *fresh74 = LLVMInt32TypeInContext((*c).context);
    let ref mut fresh75 = (*c).i64_0;
    *fresh75 = LLVMInt64TypeInContext((*c).context);
    let ref mut fresh76 = (*c).i128_0;
    *fresh76 = LLVMIntTypeInContext((*c).context, 128 as libc::c_int as libc::c_uint);
    let ref mut fresh77 = (*c).f32_0;
    *fresh77 = LLVMFloatTypeInContext((*c).context);
    let ref mut fresh78 = (*c).f64_0;
    *fresh78 = LLVMDoubleTypeInContext((*c).context);
    let ref mut fresh79 = (*c).intptr;
    *fresh79 = LLVMIntPtrTypeInContext((*c).context, (*c).target_data);
    let ref mut fresh80 = (*c).void_ptr;
    *fresh80 = LLVMPointerType((*c).i8_0, 0 as libc::c_int as libc::c_uint);
    let ref mut fresh81 = (*c).object_type;
    *fresh81 = LLVMStructCreateNamed(
        (*c).context,
        b"__object\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh82 = (*c).object_ptr;
    *fresh82 = LLVMPointerType((*c).object_type, 0 as libc::c_int as libc::c_uint);
    let ref mut fresh83 = (*c).actor_pad;
    *fresh83 = LLVMArrayType((*c).i8_0, 264 as libc::c_int as libc::c_uint);
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    let ref mut fresh84 = (*c).msg_type;
    *fresh84 = LLVMStructCreateNamed(
        (*c).context,
        b"__message\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh85 = (*c).msg_ptr;
    *fresh85 = LLVMPointerType((*c).msg_type, 0 as libc::c_int as libc::c_uint);
    LLVMStructSetBody(
        (*c).msg_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    let ref mut fresh86 = (*c).trace_type;
    *fresh86 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh87 = (*c).trace_fn;
    *fresh87 = LLVMPointerType((*c).trace_type, 0 as libc::c_int as libc::c_uint);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).void_ptr;
    params[3 as libc::c_int as usize] = (*c).intptr;
    params[4 as libc::c_int as usize] = (*c).i32_0;
    let ref mut fresh88 = (*c).serialise_type;
    *fresh88 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        5 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh89 = (*c).serialise_fn;
    *fresh89 = LLVMPointerType((*c).serialise_type, 0 as libc::c_int as libc::c_uint);
    params[0 as libc::c_int as usize] = (*c).object_ptr;
    let ref mut fresh90 = (*c).custom_serialise_space_fn;
    *fresh90 = LLVMPointerType(
        LLVMFunctionType(
            (*c).i64_0,
            params.as_mut_ptr(),
            1 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        ),
        0 as libc::c_int as libc::c_uint,
    );
    params[0 as libc::c_int as usize] = (*c).object_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    let ref mut fresh91 = (*c).custom_deserialise_fn;
    *fresh91 = LLVMPointerType(
        LLVMFunctionType(
            (*c).void_type,
            params.as_mut_ptr(),
            2 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        ),
        0 as libc::c_int as libc::c_uint,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).msg_ptr;
    let ref mut fresh92 = (*c).dispatch_type;
    *fresh92 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let ref mut fresh93 = (*c).dispatch_fn;
    *fresh93 = LLVMPointerType((*c).dispatch_type, 0 as libc::c_int as libc::c_uint);
    params[0 as libc::c_int as usize] = (*c).object_ptr;
    let ref mut fresh94 = (*c).final_fn;
    *fresh94 = LLVMPointerType(
        LLVMFunctionType(
            (*c).void_type,
            params.as_mut_ptr(),
            1 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        ),
        0 as libc::c_int as libc::c_uint,
    );
    let mut desc_name: *const libc::c_char = genname_descriptor(0 as *const libc::c_char);
    let ref mut fresh95 = (*c).descriptor_type;
    *fresh95 = LLVMStructCreateNamed((*c).context, desc_name);
    let ref mut fresh96 = (*c).descriptor_ptr;
    *fresh96 = LLVMPointerType((*c).descriptor_type, 0 as libc::c_int as libc::c_uint);
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] = (*c).descriptor_ptr;
    let ref mut fresh97 = (*c).field_descriptor;
    *fresh97 = LLVMStructTypeInContext(
        (*c).context,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    gendesc_basetype(c, (*c).descriptor_type);
    params[0 as libc::c_int as usize] = (*c).descriptor_ptr;
    LLVMStructSetBody(
        (*c).object_type,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut align_value: libc::c_uint = (if target_is_ilp32((*(*c).opt).triple) as libc::c_int != 0
    {
        4 as libc::c_int
    } else {
        8 as libc::c_int
    }) as libc::c_uint;
    let mut nounwind_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut nounwind_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"nounwind\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    nounwind_attr =
        LLVMCreateEnumAttribute((*c).context, nounwind_attr_id, 0 as libc::c_int as uint64_t);
    let mut readnone_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut readnone_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"readnone\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    readnone_attr =
        LLVMCreateEnumAttribute((*c).context, readnone_attr_id, 0 as libc::c_int as uint64_t);
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
    let mut noalias_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut noalias_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"noalias\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    noalias_attr =
        LLVMCreateEnumAttribute((*c).context, noalias_attr_id, 0 as libc::c_int as uint64_t);
    let mut noreturn_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut noreturn_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"noreturn\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    noreturn_attr =
        LLVMCreateEnumAttribute((*c).context, noreturn_attr_id, 0 as libc::c_int as uint64_t);
    let mut deref_actor_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_actor_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_actor_attr = LLVMCreateEnumAttribute(
        (*c).context,
        deref_actor_attr_id,
        (264 as libc::c_int as libc::c_uint).wrapping_add(align_value) as uint64_t,
    );
    let mut align_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut align_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"align\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    align_attr = LLVMCreateEnumAttribute((*c).context, align_attr_id, align_value as uint64_t);
    let mut deref_or_null_alloc_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_or_null_alloc_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable_or_null\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_or_null_alloc_attr = LLVMCreateEnumAttribute(
        (*c).context,
        deref_or_null_alloc_attr_id,
        ((1 as libc::c_int) << 5 as libc::c_int) as uint64_t,
    );
    let mut deref_alloc_small_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_alloc_small_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_alloc_small_attr = LLVMCreateEnumAttribute(
        (*c).context,
        deref_alloc_small_attr_id,
        ((1 as libc::c_int) << 5 as libc::c_int) as uint64_t,
    );
    let mut deref_alloc_large_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
    let mut deref_alloc_large_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
        b"dereferenceable\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    deref_alloc_large_attr = LLVMCreateEnumAttribute(
        (*c).context,
        deref_alloc_large_attr_id,
        (((1 as libc::c_int) << 10 as libc::c_int - 1 as libc::c_int) << 1 as libc::c_int)
            as uint64_t,
    );
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_ctx\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        readnone_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).descriptor_ptr;
    params[2 as libc::c_int as usize] = (*c).i1;
    type_0 = LLVMFunctionType(
        (*c).object_ptr,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_create\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_actor_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"ponyint_destroy\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).msg_ptr;
    params[3 as libc::c_int as usize] = (*c).msg_ptr;
    params[4 as libc::c_int as usize] = (*c).i1;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        5 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_sendv\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).msg_ptr;
    params[3 as libc::c_int as usize] = (*c).msg_ptr;
    params[4 as libc::c_int as usize] = (*c).i1;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        5 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_sendv_single\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_or_null_alloc_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_small\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_alloc_small_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_large\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_alloc_large_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    params[3 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        4 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_realloc\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_or_null_alloc_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_final\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_or_null_alloc_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_small_final\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_alloc_small_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_large_final\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        deref_alloc_large_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] = (*c).i32_0;
    type_0 = LLVMFunctionType(
        (*c).msg_ptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_alloc_msg\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        noalias_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeReturnIndex as libc::c_int as LLVMAttributeIndex,
        align_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_trace\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readnone_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).descriptor_ptr;
    params[3 as libc::c_int as usize] = (*c).i32_0;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        4 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_traceknown\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    params[2 as libc::c_int as usize] = (*c).i32_0;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_traceunknown\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_gc_send\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_gc_recv\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_send_done\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_recv_done\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_serialise_reserve\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readnone_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).intptr,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_serialise_offset\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).descriptor_ptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_deserialise_offset\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).intptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).void_ptr,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_deserialise_block\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).i32_0;
    params[1 as libc::c_int as usize] =
        LLVMPointerType((*c).void_ptr, 0 as libc::c_int as libc::c_uint);
    type_0 = LLVMFunctionType(
        (*c).i32_0,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_init\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).object_ptr;
    type_0 = LLVMFunctionType(
        (*c).void_type,
        params.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"ponyint_become\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    params[0 as libc::c_int as usize] = (*c).i1;
    params[1 as libc::c_int as usize] =
        LLVMPointerType((*c).i32_0, 0 as libc::c_int as libc::c_uint);
    params[2 as libc::c_int as usize] =
        LLVMPointerType((*c).i8_0, 0 as libc::c_int as libc::c_uint);
    type_0 = LLVMFunctionType(
        (*c).i1,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_start\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        inacc_or_arg_mem_attr,
    );
    type_0 = LLVMFunctionType(
        (*c).i32_0,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_get_exitcode\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        readonly_attr,
    );
    type_0 = LLVMFunctionType(
        (*c).void_type,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"pony_error\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        noreturn_attr,
    );
    type_0 = LLVMFunctionType(
        (*c).i32_0,
        0 as *mut LLVMTypeRef,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int,
    );
    let ref mut fresh98 = (*c).personality;
    *fresh98 = LLVMAddFunction(
        (*c).module,
        b"ponyint_personality_v0\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    params[1 as libc::c_int as usize] = (*c).void_ptr;
    params[2 as libc::c_int as usize] = (*c).intptr;
    type_0 = LLVMFunctionType(
        (*c).i32_0,
        params.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"memcmp\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        nounwind_attr,
    );
    LLVMAddAttributeAtIndex(
        value,
        LLVMAttributeFunctionIndex as libc::c_int as LLVMAttributeIndex,
        readonly_attr,
    );
    LLVMAddAttributeAtIndex(value, 1 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    LLVMAddAttributeAtIndex(value, 2 as libc::c_int as LLVMAttributeIndex, readonly_attr);
    params[0 as libc::c_int as usize] = (*c).void_ptr;
    type_0 = LLVMFunctionType(
        (*c).i32_0,
        params.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    value = LLVMAddFunction(
        (*c).module,
        b"puts\0" as *const u8 as *const libc::c_char,
        type_0,
    );
}
#[c2rust::src_loc = "626:1"]
unsafe extern "C" fn init_module(
    mut c: *mut compile_t,
    mut program: *mut ast_t,
    mut opt: *mut pass_opt_t,
) -> bool {
    let ref mut fresh99 = (*c).opt;
    *fresh99 = opt;
    let mut package: *mut ast_t = ast_child(program);
    let mut builtin: *mut ast_t = ast_sibling(package);
    if builtin.is_null() {
        builtin = package;
    }
    let ref mut fresh100 = (*c).filename;
    *fresh100 = package_filename(package);
    if target_is_ilp32((*opt).triple) {
        (*c).callconv = LLVMCCallConv;
    } else {
        (*c).callconv = LLVMFastCallConv;
    }
    if !(*(*c).opt).release || (*(*c).opt).extfun as libc::c_int != 0 {
        (*c).linkage = LLVMExternalLinkage;
    } else {
        (*c).linkage = LLVMPrivateLinkage;
    }
    let ref mut fresh101 = (*c).machine;
    *fresh101 = make_machine(opt);
    if ((*c).machine).is_null() {
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh102 = (*c).context;
    *fresh102 = LLVMContextCreate();
    let ref mut fresh103 = (*c).module;
    *fresh103 = LLVMModuleCreateWithNameInContext((*c).filename, (*c).context);
    LLVMSetTarget((*c).module, (*opt).triple);
    let ref mut fresh104 = (*c).target_data;
    *fresh104 = LLVMCreateTargetDataLayout((*c).machine);
    let mut layout: *mut libc::c_char = LLVMCopyStringRepOfTargetData((*c).target_data);
    LLVMSetDataLayout((*c).module, layout);
    LLVMDisposeMessage(layout);
    let ref mut fresh105 = (*c).builder;
    *fresh105 = LLVMCreateBuilderInContext((*c).context);
    let ref mut fresh106 = (*c).di;
    *fresh106 = LLVMNewDIBuilder((*c).module);
    let mut filename: *const libc::c_char = package_filename(package);
    let mut dirname: *const libc::c_char = package_path(package);
    let mut version: *const libc::c_char =
        b"ponyc-0.52.2-34e9713d\0" as *const u8 as *const libc::c_char;
    let mut fileRef: LLVMMetadataRef = LLVMDIBuilderCreateFile(
        (*c).di,
        filename,
        strlen(filename),
        dirname,
        strlen(dirname),
    );
    let ref mut fresh107 = (*c).di_unit;
    *fresh107 = LLVMDIBuilderCreateCompileUnit(
        (*c).di,
        LLVMDWARFSourceLanguageC_plus_plus,
        fileRef,
        version,
        strlen(version),
        (*opt).release as LLVMBool,
        if !((*opt).all_args).is_null() {
            (*opt).all_args
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !((*opt).all_args).is_null() {
            strlen((*opt).all_args)
        } else {
            0 as libc::c_int as libc::c_ulong
        },
        0 as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        LLVMDWARFEmissionFull,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
        0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    let ref mut fresh108 = (*c).frame;
    *fresh108 = 0 as *mut compile_frame_t;
    let ref mut fresh109 = (*c).reach;
    *fresh109 = reach_new();
    let ref mut fresh110 = (*c).none_instance;
    *fresh110 = 0 as LLVMValueRef;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "699:1"]
pub unsafe extern "C" fn codegen_merge_runtime_bitcode(mut c: *mut compile_t) -> bool {
    let mut path: [libc::c_char; 1024] = [0; 1024];
    let mut runtime: LLVMModuleRef = 0 as LLVMModuleRef;
    let mut p: *mut strlist_t = (*(*c).opt).package_search_paths;
    while !p.is_null() && runtime.is_null() {
        path_cat(
            strlist_data(p),
            b"libponyrt.bc\0" as *const u8 as *const libc::c_char,
            path.as_mut_ptr(),
        );
        runtime = LLVMParseIRFileInContext((*c).context, path.as_mut_ptr());
        p = strlist_next(p);
    }
    let mut errors: *mut errors_t = (*(*c).opt).check.errors;
    if runtime.is_null() {
        errorf(
            errors,
            0 as *const libc::c_char,
            b"couldn't find libponyrt.bc\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if (*(*c).opt).verbosity as libc::c_uint >= VERBOSITY_MINIMAL as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"Merging runtime\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if LLVMLinkModules2((*c).module, runtime) != 0 {
        errorf(
            errors,
            0 as *const libc::c_char,
            b"libponyrt.bc contains errors\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[c2rust::src_loc = "734:1"]
unsafe extern "C" fn process_llvm_args(mut opt: *mut pass_opt_t) {
    if ((*opt).llvm_args).is_null() {
        return;
    }
    let mut raw_opt_str_size: size_t =
        (strlen((*opt).llvm_args)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buffer: *mut libc::c_char = malloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(raw_opt_str_size),
    ) as *mut libc::c_char;
    strncpy(buffer, (*opt).llvm_args, raw_opt_str_size);
    let mut argv_buf_size: size_t = 4 as libc::c_int as size_t;
    let mut argv_buffer: *mut *const libc::c_char = malloc(
        (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_mul(argv_buf_size),
    ) as *mut *const libc::c_char;
    let mut token_counter: size_t = 0 as libc::c_int as size_t;
    let fresh111 = token_counter;
    token_counter = token_counter.wrapping_add(1);
    let ref mut fresh112 = *argv_buffer.offset(fresh111 as isize);
    *fresh112 = (*opt).argv0;
    let mut token: *mut libc::c_char = strtok(buffer, b",\0" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        token_counter = token_counter.wrapping_add(1);
        if token_counter > argv_buf_size {
            argv_buf_size <<= 1 as libc::c_int;
            argv_buffer = realloc(
                argv_buffer as *mut libc::c_void,
                (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(argv_buf_size),
            ) as *mut *const libc::c_char;
        }
        let ref mut fresh113 = *argv_buffer
            .offset(token_counter.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *fresh113 = token as *const libc::c_char;
        token = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
    }
    LLVMParseCommandLineOptions(
        token_counter as libc::c_int,
        argv_buffer as *const *const libc::c_char,
        0 as *const libc::c_char,
    );
    free(argv_buffer as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "782:1"]
pub unsafe extern "C" fn codegen_llvm_init() -> bool {
    LLVMLoadLibraryPermanently(0 as *const libc::c_char);
    LLVMInitializeNativeTarget();
    LLVMInitializeAllTargets();
    LLVMInitializeAllTargetMCs();
    LLVMInitializeAllTargetInfos();
    LLVMInitializeAllAsmPrinters();
    LLVMInitializeAllAsmParsers();
    LLVMEnablePrettyStackTrace();
    LLVMInstallFatalErrorHandler(Some(
        fatal_error as unsafe extern "C" fn(*const libc::c_char) -> (),
    ));
    let mut passreg: LLVMPassRegistryRef = LLVMGetGlobalPassRegistry();
    LLVMInitializeCore(passreg);
    LLVMInitializeTransformUtils(passreg);
    LLVMInitializeScalarOpts(passreg);
    LLVMInitializeObjCARCOpts(passreg);
    LLVMInitializeVectorization(passreg);
    LLVMInitializeInstCombine(passreg);
    LLVMInitializeIPO(passreg);
    LLVMInitializeInstrumentation(passreg);
    LLVMInitializeAnalysis(passreg);
    LLVMInitializeIPA(passreg);
    LLVMInitializeCodeGen(passreg);
    LLVMInitializeTarget(passreg);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "811:1"]
pub unsafe extern "C" fn codegen_llvm_shutdown() {
    LLVMResetFatalErrorHandler();
    LLVMShutdown();
}
#[no_mangle]
#[c2rust::src_loc = "817:1"]
pub unsafe extern "C" fn codegen_pass_init(mut opt: *mut pass_opt_t) -> bool {
    process_llvm_args(opt);
    let mut triple: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*opt).triple).is_null() {
        triple = LLVMCreateMessage((*opt).triple);
    } else {
        triple = LLVMCreateMessage(b"x86_64-apple-macosx\0" as *const u8 as *const libc::c_char);
        let mut normalized: *mut libc::c_char = LLVMNormalizeTargetTriple(triple);
        free(triple as *mut libc::c_void);
        triple = normalized;
    }
    if !((*opt).features).is_null() {
        let ref mut fresh114 = (*opt).features;
        *fresh114 = LLVMCreateMessage((*opt).features);
    } else if ((*opt).cpu).is_null() && ((*opt).triple).is_null() {
        let ref mut fresh115 = (*opt).features;
        *fresh115 = LLVMGetHostCPUFeatures();
    } else {
        let ref mut fresh116 = (*opt).features;
        *fresh116 = LLVMCreateMessage(b"\0" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh117 = (*opt).triple;
    *fresh117 = triple;
    if !((*opt).abi).is_null() {
        let ref mut fresh118 = (*opt).abi;
        *fresh118 = LLVMCreateMessage((*opt).abi);
    }
    if !((*opt).cpu).is_null() {
        let ref mut fresh119 = (*opt).cpu;
        *fresh119 = LLVMCreateMessage((*opt).cpu);
    } else {
        let ref mut fresh120 = (*opt).cpu;
        *fresh120 = LLVMGetHostCPUName();
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "865:1"]
pub unsafe extern "C" fn codegen_pass_cleanup(mut opt: *mut pass_opt_t) {
    LLVMDisposeMessage((*opt).triple);
    LLVMDisposeMessage((*opt).cpu);
    LLVMDisposeMessage((*opt).features);
    if !((*opt).abi).is_null() {
        LLVMDisposeMessage((*opt).abi);
        let ref mut fresh121 = (*opt).abi;
        *fresh121 = 0 as *mut libc::c_char;
    }
    let ref mut fresh122 = (*opt).triple;
    *fresh122 = 0 as *mut libc::c_char;
    let ref mut fresh123 = (*opt).cpu;
    *fresh123 = 0 as *mut libc::c_char;
    let ref mut fresh124 = (*opt).features;
    *fresh124 = 0 as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "880:1"]
pub unsafe extern "C" fn codegen(mut program: *mut ast_t, mut opt: *mut pass_opt_t) -> bool {
    if (*opt).verbosity as libc::c_uint >= VERBOSITY_MINIMAL as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"Generating\n\0" as *const u8 as *const libc::c_char,
        );
    }
    pony_mkdir((*opt).output);
    let mut c: compile_t = compile_t {
        opt: 0 as *mut pass_opt_t,
        reach: 0 as *mut reach_t,
        strings: genned_strings_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        ffi_decls: ffi_decls_t {
            contents: hashmap_t {
                count: 0,
                size: 0,
                item_bitmap: 0 as *mut bitmap_t,
                buckets: 0 as *mut hashmap_entry_t,
            },
        },
        filename: 0 as *const libc::c_char,
        str_builtin: 0 as *const libc::c_char,
        str_Bool: 0 as *const libc::c_char,
        str_I8: 0 as *const libc::c_char,
        str_I16: 0 as *const libc::c_char,
        str_I32: 0 as *const libc::c_char,
        str_I64: 0 as *const libc::c_char,
        str_I128: 0 as *const libc::c_char,
        str_ILong: 0 as *const libc::c_char,
        str_ISize: 0 as *const libc::c_char,
        str_U8: 0 as *const libc::c_char,
        str_U16: 0 as *const libc::c_char,
        str_U32: 0 as *const libc::c_char,
        str_U64: 0 as *const libc::c_char,
        str_U128: 0 as *const libc::c_char,
        str_ULong: 0 as *const libc::c_char,
        str_USize: 0 as *const libc::c_char,
        str_F32: 0 as *const libc::c_char,
        str_F64: 0 as *const libc::c_char,
        str_Pointer: 0 as *const libc::c_char,
        str_NullablePointer: 0 as *const libc::c_char,
        str_DoNotOptimise: 0 as *const libc::c_char,
        str_Array: 0 as *const libc::c_char,
        str_String: 0 as *const libc::c_char,
        str_Platform: 0 as *const libc::c_char,
        str_Main: 0 as *const libc::c_char,
        str_Env: 0 as *const libc::c_char,
        str_add: 0 as *const libc::c_char,
        str_sub: 0 as *const libc::c_char,
        str_mul: 0 as *const libc::c_char,
        str_div: 0 as *const libc::c_char,
        str_rem: 0 as *const libc::c_char,
        str_neg: 0 as *const libc::c_char,
        str_add_unsafe: 0 as *const libc::c_char,
        str_sub_unsafe: 0 as *const libc::c_char,
        str_mul_unsafe: 0 as *const libc::c_char,
        str_div_unsafe: 0 as *const libc::c_char,
        str_rem_unsafe: 0 as *const libc::c_char,
        str_neg_unsafe: 0 as *const libc::c_char,
        str_and: 0 as *const libc::c_char,
        str_or: 0 as *const libc::c_char,
        str_xor: 0 as *const libc::c_char,
        str_not: 0 as *const libc::c_char,
        str_shl: 0 as *const libc::c_char,
        str_shr: 0 as *const libc::c_char,
        str_shl_unsafe: 0 as *const libc::c_char,
        str_shr_unsafe: 0 as *const libc::c_char,
        str_eq: 0 as *const libc::c_char,
        str_ne: 0 as *const libc::c_char,
        str_lt: 0 as *const libc::c_char,
        str_le: 0 as *const libc::c_char,
        str_ge: 0 as *const libc::c_char,
        str_gt: 0 as *const libc::c_char,
        str_eq_unsafe: 0 as *const libc::c_char,
        str_ne_unsafe: 0 as *const libc::c_char,
        str_lt_unsafe: 0 as *const libc::c_char,
        str_le_unsafe: 0 as *const libc::c_char,
        str_ge_unsafe: 0 as *const libc::c_char,
        str_gt_unsafe: 0 as *const libc::c_char,
        str_this: 0 as *const libc::c_char,
        str_create: 0 as *const libc::c_char,
        str__create: 0 as *const libc::c_char,
        str__init: 0 as *const libc::c_char,
        str__final: 0 as *const libc::c_char,
        str__event_notify: 0 as *const libc::c_char,
        str__serialise_space: 0 as *const libc::c_char,
        str__serialise: 0 as *const libc::c_char,
        str__deserialise: 0 as *const libc::c_char,
        trait_bitmap_size: 0,
        callconv: LLVMCCallConv,
        linkage: LLVMExternalLinkage,
        context: 0 as *mut LLVMOpaqueContext,
        machine: 0 as *mut LLVMOpaqueTargetMachine,
        target_data: 0 as *mut LLVMOpaqueTargetData,
        module: 0 as *mut LLVMOpaqueModule,
        builder: 0 as *mut LLVMOpaqueBuilder,
        di: 0 as *mut LLVMOpaqueDIBuilder,
        di_unit: 0 as *mut LLVMOpaqueMetadata,
        none_instance: 0 as *mut LLVMOpaqueValue,
        primitives_init: 0 as *mut LLVMOpaqueValue,
        primitives_final: 0 as *mut LLVMOpaqueValue,
        desc_table: 0 as *mut LLVMOpaqueValue,
        numeric_sizes: 0 as *mut LLVMOpaqueValue,
        void_type: 0 as *mut LLVMOpaqueType,
        i1: 0 as *mut LLVMOpaqueType,
        i8_0: 0 as *mut LLVMOpaqueType,
        i16_0: 0 as *mut LLVMOpaqueType,
        i32_0: 0 as *mut LLVMOpaqueType,
        i64_0: 0 as *mut LLVMOpaqueType,
        i128_0: 0 as *mut LLVMOpaqueType,
        f32_0: 0 as *mut LLVMOpaqueType,
        f64_0: 0 as *mut LLVMOpaqueType,
        intptr: 0 as *mut LLVMOpaqueType,
        void_ptr: 0 as *mut LLVMOpaqueType,
        descriptor_type: 0 as *mut LLVMOpaqueType,
        descriptor_ptr: 0 as *mut LLVMOpaqueType,
        field_descriptor: 0 as *mut LLVMOpaqueType,
        object_type: 0 as *mut LLVMOpaqueType,
        object_ptr: 0 as *mut LLVMOpaqueType,
        msg_type: 0 as *mut LLVMOpaqueType,
        msg_ptr: 0 as *mut LLVMOpaqueType,
        actor_pad: 0 as *mut LLVMOpaqueType,
        trace_type: 0 as *mut LLVMOpaqueType,
        trace_fn: 0 as *mut LLVMOpaqueType,
        serialise_type: 0 as *mut LLVMOpaqueType,
        serialise_fn: 0 as *mut LLVMOpaqueType,
        dispatch_type: 0 as *mut LLVMOpaqueType,
        dispatch_fn: 0 as *mut LLVMOpaqueType,
        final_fn: 0 as *mut LLVMOpaqueType,
        custom_serialise_space_fn: 0 as *mut LLVMOpaqueType,
        custom_deserialise_fn: 0 as *mut LLVMOpaqueType,
        personality: 0 as *mut LLVMOpaqueValue,
        frame: 0 as *mut compile_frame_t,
    };
    memset(
        &mut c as *mut compile_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<compile_t>() as libc::c_ulong,
    );
    genned_strings_init(&mut c.strings, 64 as libc::c_int as size_t);
    ffi_decls_init(&mut c.ffi_decls, 64 as libc::c_int as size_t);
    if !init_module(&mut c, program, opt) {
        return 0 as libc::c_int != 0;
    }
    init_runtime(&mut c);
    genprim_reachable_init(&mut c, program);
    let mut ok: bool = genexe(&mut c, program);
    codegen_cleanup(&mut c);
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "905:1"]
pub unsafe extern "C" fn codegen_gen_test(
    mut c: *mut compile_t,
    mut program: *mut ast_t,
    mut opt: *mut pass_opt_t,
    mut last_pass: pass_id,
) -> bool {
    if (last_pass as libc::c_uint) < PASS_REACH as libc::c_int as libc::c_uint {
        memset(
            c as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<compile_t>() as libc::c_ulong,
        );
        genned_strings_init(&mut (*c).strings, 64 as libc::c_int as size_t);
        ffi_decls_init(&mut (*c).ffi_decls, 64 as libc::c_int as size_t);
        if !init_module(c, program, opt) {
            return 0 as libc::c_int != 0;
        }
        init_runtime(c);
        genprim_reachable_init(c, program);
        let mut main_actor: *const libc::c_char = (*c).str_Main;
        let mut env_class: *const libc::c_char = (*c).str_Env;
        let mut package: *mut ast_t = ast_child(program);
        let mut main_def: *mut ast_t = ast_get(package, main_actor, 0 as *mut sym_status_t);
        if main_def.is_null() {
            return 0 as libc::c_int != 0;
        }
        let mut main_ast: *mut ast_t = type_builtin(opt, main_def, main_actor);
        let mut env_ast: *mut ast_t = type_builtin(opt, main_def, env_class);
        let mut main_create: *mut deferred_reification_t =
            lookup(opt, main_ast, main_ast, (*c).str_create);
        if main_create.is_null() {
            ast_free(main_ast);
            ast_free(env_ast);
            return 0 as libc::c_int != 0;
        }
        reach((*c).reach, main_ast, (*c).str_create, 0 as *mut ast_t, opt);
        reach((*c).reach, env_ast, (*c).str__create, 0 as *mut ast_t, opt);
        ast_free(main_ast);
        ast_free(env_ast);
        deferred_reify_free(main_create);
    }
    if (*opt).limit as libc::c_uint == PASS_REACH as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if (last_pass as libc::c_uint) < PASS_PAINT as libc::c_int as libc::c_uint {
        paint(&mut (*(*c).reach).types);
    }
    if (*opt).limit as libc::c_uint == PASS_PAINT as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if !gentypes(c) {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "966:1"]
pub unsafe extern "C" fn codegen_cleanup(mut c: *mut compile_t) {
    while !((*c).frame).is_null() {
        pop_frame(c);
    }
    LLVMDIBuilderDestroy((*c).di);
    LLVMDisposeBuilder((*c).builder);
    LLVMDisposeModule((*c).module);
    LLVMContextDispose((*c).context);
    LLVMDisposeTargetData((*c).target_data);
    LLVMDisposeTargetMachine((*c).machine);
    genned_strings_destroy(&mut (*c).strings);
    ffi_decls_destroy(&mut (*c).ffi_decls);
    reach_free((*c).reach);
}
#[no_mangle]
#[c2rust::src_loc = "982:1"]
pub unsafe extern "C" fn codegen_addfun(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
    mut type_0: LLVMTypeRef,
    mut pony_abi: bool,
) -> LLVMValueRef {
    let mut fun: LLVMValueRef = LLVMAddFunction((*c).module, name, type_0);
    LLVMSetFunctionCallConv(fun, (*c).callconv as libc::c_uint);
    LLVMSetLinkage(fun, (*c).linkage);
    LLVMSetUnnamedAddr(fun, 1 as libc::c_int);
    if pony_abi {
        let mut md: LLVMValueRef = LLVMMDNodeInContext(
            (*c).context,
            0 as *mut LLVMValueRef,
            0 as libc::c_int as libc::c_uint,
        );
        LLVMSetMetadataStr(fun, b"pony.abi\0" as *const u8 as *const libc::c_char, md);
    }
    let mut arg: LLVMValueRef = LLVMGetFirstParam(fun);
    let mut i: uint32_t = 1 as libc::c_int as uint32_t;
    while !arg.is_null() {
        let mut type_1: LLVMTypeRef = LLVMTypeOf(arg);
        if LLVMGetTypeKind(type_1) as libc::c_uint
            == LLVMPointerTypeKind as libc::c_int as libc::c_uint
        {
            let mut elem: LLVMTypeRef = LLVMGetElementType(type_1);
            if LLVMGetTypeKind(elem) as libc::c_uint
                == LLVMStructTypeKind as libc::c_int as libc::c_uint
            {
                let mut size: size_t = LLVMABISizeOfType((*c).target_data, elem) as size_t;
                let mut deref_attr: LLVMAttributeRef = 0 as *mut LLVMOpaqueAttributeRef;
                let mut deref_attr_id: libc::c_uint = LLVMGetEnumAttributeKindForName(
                    b"dereferenceable\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                deref_attr = LLVMCreateEnumAttribute((*c).context, deref_attr_id, size as uint64_t);
                LLVMAddAttributeAtIndex(fun, i, deref_attr);
            }
        }
        arg = LLVMGetNextParam(arg);
        i = i.wrapping_add(1);
    }
    return fun;
}
#[no_mangle]
#[c2rust::src_loc = "1023:1"]
pub unsafe extern "C" fn codegen_startfun(
    mut c: *mut compile_t,
    mut fun: LLVMValueRef,
    mut file: LLVMMetadataRef,
    mut scope: LLVMMetadataRef,
    mut reify: *mut deferred_reification_t,
    mut bare: bool,
) {
    let mut frame: *mut compile_frame_t = push_frame(c);
    let ref mut fresh125 = (*frame).fun;
    *fresh125 = fun;
    (*frame).is_function = 1 as libc::c_int != 0;
    let ref mut fresh126 = (*frame).reify;
    *fresh126 = reify;
    (*frame).bare_function = bare;
    let ref mut fresh127 = (*frame).di_file;
    *fresh127 = file;
    let ref mut fresh128 = (*frame).di_scope;
    *fresh128 = scope;
    if LLVMCountBasicBlocks(fun) == 0 as libc::c_int as libc::c_uint {
        let mut block: LLVMBasicBlockRef =
            codegen_block(c, b"entry\0" as *const u8 as *const libc::c_char);
        LLVMPositionBuilderAtEnd((*c).builder, block);
    }
    LLVMSetCurrentDebugLocation2((*c).builder, 0 as LLVMMetadataRef);
}
#[no_mangle]
#[c2rust::src_loc = "1044:1"]
pub unsafe extern "C" fn codegen_finishfun(mut c: *mut compile_t) {
    pop_frame(c);
}
#[no_mangle]
#[c2rust::src_loc = "1049:1"]
pub unsafe extern "C" fn codegen_pushscope(mut c: *mut compile_t, mut ast: *mut ast_t) {
    let mut frame: *mut compile_frame_t = push_frame(c);
    let ref mut fresh129 = (*frame).fun;
    *fresh129 = (*(*frame).prev).fun;
    let ref mut fresh130 = (*frame).reify;
    *fresh130 = (*(*frame).prev).reify;
    (*frame).bare_function = (*(*frame).prev).bare_function;
    let ref mut fresh131 = (*frame).break_target;
    *fresh131 = (*(*frame).prev).break_target;
    let ref mut fresh132 = (*frame).break_novalue_target;
    *fresh132 = (*(*frame).prev).break_novalue_target;
    let ref mut fresh133 = (*frame).continue_target;
    *fresh133 = (*(*frame).prev).continue_target;
    let ref mut fresh134 = (*frame).invoke_target;
    *fresh134 = (*(*frame).prev).invoke_target;
    let ref mut fresh135 = (*frame).di_file;
    *fresh135 = (*(*frame).prev).di_file;
    if !((*(*frame).prev).di_scope).is_null() {
        let ref mut fresh136 = (*frame).di_scope;
        *fresh136 = LLVMDIBuilderCreateLexicalBlock(
            (*c).di,
            (*(*frame).prev).di_scope,
            (*frame).di_file,
            ast_line(ast) as libc::c_uint,
            ast_pos(ast) as libc::c_uint,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn codegen_popscope(mut c: *mut compile_t) {
    pop_frame(c);
}
#[no_mangle]
#[c2rust::src_loc = "1075:1"]
pub unsafe extern "C" fn codegen_local_lifetime_start(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
) {
    let mut frame: *mut compile_frame_t = (*c).frame;
    let mut k: compile_local_t = compile_local_t {
        name: 0 as *const libc::c_char,
        alloca: 0 as *mut LLVMOpaqueValue,
        alive: false,
    };
    k.name = name;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    while !frame.is_null() {
        let mut p: *mut compile_local_t =
            compile_locals_get(&mut (*frame).locals, &mut k, &mut index);
        if !p.is_null() && !(*p).alive {
            gencall_lifetime_start(c, (*p).alloca);
            (*p).alive = 1 as libc::c_int != 0;
            return;
        }
        if (*frame).is_function {
            return;
        }
        frame = (*frame).prev;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn codegen_local_lifetime_end(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
) {
    let mut frame: *mut compile_frame_t = (*c).frame;
    let mut k: compile_local_t = compile_local_t {
        name: 0 as *const libc::c_char,
        alloca: 0 as *mut LLVMOpaqueValue,
        alive: false,
    };
    k.name = name;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    while !frame.is_null() {
        let mut p: *mut compile_local_t =
            compile_locals_get(&mut (*frame).locals, &mut k, &mut index);
        if !p.is_null() && (*p).alive as libc::c_int != 0 {
            gencall_lifetime_end(c, (*p).alloca);
            (*p).alive = 0 as libc::c_int != 0;
            return;
        }
        if (*frame).is_function {
            return;
        }
        frame = (*frame).prev;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1127:1"]
pub unsafe extern "C" fn codegen_scope_lifetime_end(mut c: *mut compile_t) {
    if !(*(*c).frame).early_termination {
        let mut frame: *mut compile_frame_t = (*c).frame;
        let mut i: size_t = -(1 as libc::c_int) as size_t;
        let mut p: *mut compile_local_t = 0 as *mut compile_local_t;
        loop {
            p = compile_locals_next(&mut (*frame).locals, &mut i);
            if p.is_null() {
                break;
            }
            if (*p).alive {
                gencall_lifetime_end(c, (*p).alloca);
            }
        }
        (*(*c).frame).early_termination = 1 as libc::c_int != 0;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1144:1"]
pub unsafe extern "C" fn codegen_difile(mut c: *mut compile_t) -> LLVMMetadataRef {
    return (*(*c).frame).di_file;
}
#[no_mangle]
#[c2rust::src_loc = "1149:1"]
pub unsafe extern "C" fn codegen_discope(mut c: *mut compile_t) -> LLVMMetadataRef {
    return (*(*c).frame).di_scope;
}
#[no_mangle]
#[c2rust::src_loc = "1154:1"]
pub unsafe extern "C" fn codegen_pushloop(
    mut c: *mut compile_t,
    mut continue_target: LLVMBasicBlockRef,
    mut break_target: LLVMBasicBlockRef,
    mut break_novalue_target: LLVMBasicBlockRef,
) {
    let mut frame: *mut compile_frame_t = push_frame(c);
    let ref mut fresh137 = (*frame).fun;
    *fresh137 = (*(*frame).prev).fun;
    let ref mut fresh138 = (*frame).reify;
    *fresh138 = (*(*frame).prev).reify;
    (*frame).bare_function = (*(*frame).prev).bare_function;
    let ref mut fresh139 = (*frame).break_target;
    *fresh139 = break_target;
    let ref mut fresh140 = (*frame).break_novalue_target;
    *fresh140 = break_novalue_target;
    let ref mut fresh141 = (*frame).continue_target;
    *fresh141 = continue_target;
    let ref mut fresh142 = (*frame).invoke_target;
    *fresh142 = (*(*frame).prev).invoke_target;
    let ref mut fresh143 = (*frame).di_file;
    *fresh143 = (*(*frame).prev).di_file;
    let ref mut fresh144 = (*frame).di_scope;
    *fresh144 = (*(*frame).prev).di_scope;
}
#[no_mangle]
#[c2rust::src_loc = "1170:1"]
pub unsafe extern "C" fn codegen_poploop(mut c: *mut compile_t) {
    pop_frame(c);
}
#[no_mangle]
#[c2rust::src_loc = "1175:1"]
pub unsafe extern "C" fn codegen_pushtry(
    mut c: *mut compile_t,
    mut invoke_target: LLVMBasicBlockRef,
) {
    let mut frame: *mut compile_frame_t = push_frame(c);
    let ref mut fresh145 = (*frame).fun;
    *fresh145 = (*(*frame).prev).fun;
    let ref mut fresh146 = (*frame).reify;
    *fresh146 = (*(*frame).prev).reify;
    (*frame).bare_function = (*(*frame).prev).bare_function;
    let ref mut fresh147 = (*frame).break_target;
    *fresh147 = (*(*frame).prev).break_target;
    let ref mut fresh148 = (*frame).break_novalue_target;
    *fresh148 = (*(*frame).prev).break_novalue_target;
    let ref mut fresh149 = (*frame).continue_target;
    *fresh149 = (*(*frame).prev).continue_target;
    let ref mut fresh150 = (*frame).invoke_target;
    *fresh150 = invoke_target;
    let ref mut fresh151 = (*frame).di_file;
    *fresh151 = (*(*frame).prev).di_file;
    let ref mut fresh152 = (*frame).di_scope;
    *fresh152 = (*(*frame).prev).di_scope;
}
#[no_mangle]
#[c2rust::src_loc = "1190:1"]
pub unsafe extern "C" fn codegen_poptry(mut c: *mut compile_t) {
    pop_frame(c);
}
#[no_mangle]
#[c2rust::src_loc = "1195:1"]
pub unsafe extern "C" fn codegen_debugloc(mut c: *mut compile_t, mut ast: *mut ast_t) {
    if !ast.is_null() && !((*(*c).frame).di_scope).is_null() {
        let mut loc: LLVMMetadataRef = LLVMDIBuilderCreateDebugLocation(
            (*c).context,
            ast_line(ast) as libc::c_uint,
            ast_pos(ast) as libc::c_uint,
            (*(*c).frame).di_scope,
            0 as LLVMMetadataRef,
        );
        LLVMSetCurrentDebugLocation2((*c).builder, loc);
    } else {
        LLVMSetCurrentDebugLocation2((*c).builder, 0 as LLVMMetadataRef);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1208:1"]
pub unsafe extern "C" fn codegen_getlocal(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
) -> LLVMValueRef {
    let mut frame: *mut compile_frame_t = (*c).frame;
    let mut k: compile_local_t = compile_local_t {
        name: 0 as *const libc::c_char,
        alloca: 0 as *mut LLVMOpaqueValue,
        alive: false,
    };
    k.name = name;
    let mut index: size_t = -(1 as libc::c_int) as size_t;
    while !frame.is_null() {
        let mut p: *mut compile_local_t =
            compile_locals_get(&mut (*frame).locals, &mut k, &mut index);
        if !p.is_null() {
            return (*p).alloca;
        }
        if (*frame).is_function {
            return 0 as LLVMValueRef;
        }
        frame = (*frame).prev;
    }
    return 0 as LLVMValueRef;
}
#[no_mangle]
#[c2rust::src_loc = "1232:1"]
pub unsafe extern "C" fn codegen_setlocal(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
    mut alloca: LLVMValueRef,
) {
    let mut p: *mut compile_local_t =
        ponyint_pool_alloc(0 as libc::c_int as size_t) as *mut compile_local_t;
    let ref mut fresh153 = (*p).name;
    *fresh153 = name;
    let ref mut fresh154 = (*p).alloca;
    *fresh154 = alloca;
    (*p).alive = 0 as libc::c_int != 0;
    compile_locals_put(&mut (*(*c).frame).locals, p);
}
#[no_mangle]
#[c2rust::src_loc = "1242:1"]
pub unsafe extern "C" fn codegen_ctx(mut c: *mut compile_t) -> LLVMValueRef {
    let mut frame: *mut compile_frame_t = (*c).frame;
    while !(*frame).is_function {
        frame = (*frame).prev;
    }
    if ((*frame).ctx).is_null() {
        let mut this_block: LLVMBasicBlockRef = LLVMGetInsertBlock((*c).builder);
        let mut entry_block: LLVMBasicBlockRef = LLVMGetEntryBasicBlock((*frame).fun);
        let mut inst: LLVMValueRef = LLVMGetFirstInstruction(entry_block);
        if !inst.is_null() {
            LLVMPositionBuilderBefore((*c).builder, inst);
        } else {
            LLVMPositionBuilderAtEnd((*c).builder, entry_block);
        }
        let ref mut fresh155 = (*frame).ctx;
        *fresh155 = gencall_runtime(
            c,
            b"pony_ctx\0" as *const u8 as *const libc::c_char,
            0 as *mut LLVMValueRef,
            0 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
        LLVMPositionBuilderAtEnd((*c).builder, this_block);
    }
    return (*frame).ctx;
}
#[no_mangle]
#[c2rust::src_loc = "1267:1"]
pub unsafe extern "C" fn codegen_setctx(mut c: *mut compile_t, mut ctx: LLVMValueRef) {
    let mut frame: *mut compile_frame_t = (*c).frame;
    while !(*frame).is_function {
        frame = (*frame).prev;
    }
    let ref mut fresh156 = (*frame).ctx;
    *fresh156 = ctx;
}
#[no_mangle]
#[c2rust::src_loc = "1277:1"]
pub unsafe extern "C" fn codegen_fun(mut c: *mut compile_t) -> LLVMValueRef {
    return (*(*c).frame).fun;
}
#[no_mangle]
#[c2rust::src_loc = "1282:1"]
pub unsafe extern "C" fn codegen_block(
    mut c: *mut compile_t,
    mut name: *const libc::c_char,
) -> LLVMBasicBlockRef {
    return LLVMAppendBasicBlockInContext((*c).context, (*(*c).frame).fun, name);
}
#[no_mangle]
#[c2rust::src_loc = "1287:1"]
pub unsafe extern "C" fn codegen_call(
    mut c: *mut compile_t,
    mut fun: LLVMValueRef,
    mut args: *mut LLVMValueRef,
    mut count: size_t,
    mut setcc: bool,
) -> LLVMValueRef {
    let mut result: LLVMValueRef = LLVMBuildCall_P(
        (*c).builder,
        fun,
        args,
        count as libc::c_int as libc::c_uint,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if setcc {
        LLVMSetInstructionCallConv(result, (*c).callconv as libc::c_uint);
    }
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "1298:1"]
pub unsafe extern "C" fn codegen_string(
    mut c: *mut compile_t,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> LLVMValueRef {
    let mut str_val: LLVMValueRef = LLVMConstStringInContext(
        (*c).context,
        str,
        len as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut str_ty: LLVMTypeRef = LLVMTypeOf(str_val);
    let mut g_str: LLVMValueRef = LLVMAddGlobal(
        (*c).module,
        str_ty,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMSetLinkage(g_str, LLVMPrivateLinkage);
    LLVMSetInitializer(g_str, str_val);
    LLVMSetGlobalConstant(g_str, 1 as libc::c_int);
    LLVMSetUnnamedAddr(g_str, 1 as libc::c_int);
    let mut args: [LLVMValueRef; 2] = [0 as *mut LLVMOpaqueValue; 2];
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
    return LLVMConstInBoundsGEP_P(g_str, args.as_mut_ptr(), 2 as libc::c_int as libc::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "1315:1"]
pub unsafe extern "C" fn suffix_filename(
    mut c: *mut compile_t,
    mut dir: *const libc::c_char,
    mut prefix: *const libc::c_char,
    mut file: *const libc::c_char,
    mut extension: *const libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = (strlen(dir))
        .wrapping_add(strlen(prefix))
        .wrapping_add(strlen(file))
        .wrapping_add(strlen(extension))
        .wrapping_add(4 as libc::c_int as libc::c_ulong);
    let mut filename: *mut libc::c_char = ponyint_pool_alloc_size(len) as *mut libc::c_char;
    snprintf(
        filename,
        len,
        b"%s/%s%s%s\0" as *const u8 as *const libc::c_char,
        dir,
        prefix,
        file,
        extension,
    );
    let mut suffix: libc::c_int = 0 as libc::c_int;
    while suffix < 100 as libc::c_int {
        let mut s: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_birthtimespec: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        let mut err: libc::c_int = stat(filename, &mut s);
        if err == -(1 as libc::c_int)
            || !(s.st_mode as libc::c_int & 0o170000 as libc::c_int == 0o40000 as libc::c_int)
        {
            break;
        }
        suffix += 1;
        snprintf(
            filename,
            len,
            b"%s/%s%s%d%s\0" as *const u8 as *const libc::c_char,
            dir,
            prefix,
            file,
            suffix,
            extension,
        );
    }
    if suffix >= 100 as libc::c_int {
        errorf(
            (*(*c).opt).check.errors,
            0 as *const libc::c_char,
            b"couldn't pick an unused file name\0" as *const u8 as *const libc::c_char,
        );
        ponyint_pool_free_size(len, filename as *mut libc::c_void);
        return 0 as *const libc::c_char;
    }
    return stringtab_consume(filename, len);
}
