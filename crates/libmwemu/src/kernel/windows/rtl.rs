//! The runtime library: counted strings, security descriptors, version
//! queries, registry helpers, bitmaps and image helpers.
//!
//! The security-descriptor group matters more than it looks: `wdmsec.lib`'s
//! `IoCreateDeviceSecure` is statically linked into most vendor drivers and
//! builds a DACL from `SeExports` SIDs through exactly these calls before it
//! reaches `IoCreateDevice`. If they returned garbage, DriverEntry would fail
//! before the device the exploit talks to ever existed.

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::policy::api_try;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

use super::structs::*;
use super::*;
use crate::kernel::heap::Region;

fn unicode_len(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_word(us).unwrap_or(0) as u64
}

fn unicode_max(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_word(us + 2).unwrap_or(0) as u64
}

fn unicode_buf(emu: &Emu, us: u64) -> u64 {
    emu.maps.read_qword(us + 8).unwrap_or(0)
}

/// Append `bytes` (UTF-16LE) to the counted string at `dst`.
fn unicode_append(emu: &mut Emu, dst: u64, bytes: &[u8]) -> u64 {
    let len = unicode_len(emu, dst);
    let max = unicode_max(emu, dst);
    let buf = unicode_buf(emu, dst);
    if buf == 0 || len + bytes.len() as u64 > max {
        return STATUS_BUFFER_TOO_SMALL;
    }
    let rip = emu.pc();
    emu.kernel_guard_access(rip, buf + len, bytes.len() as u32, true);
    emu.maps.write_bytes(buf + len, bytes);
    let new_len = len + bytes.len() as u64;
    emu.maps.write_word(dst, new_len as u16);
    if new_len + 2 <= max {
        emu.maps.write_word(buf + new_len, 0);
    }
    STATUS_SUCCESS
}

fn utf16_bytes(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
}

fn compare_strings(a: &str, b: &str, case_insensitive: bool) -> i64 {
    let (x, y) = if case_insensitive {
        (a.to_lowercase(), b.to_lowercase())
    } else {
        (a.to_string(), b.to_string())
    };
    match x.cmp(&y) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// `RtlLengthSid`: `8 + 4 * SubAuthorityCount`.
fn sid_length(emu: &Emu, sid: u64) -> u64 {
    if sid == 0 || !emu.maps.is_mapped(sid) {
        return 0;
    }
    8 + 4 * emu.maps.read_byte(sid + 1).unwrap_or(0) as u64
}

/// Append an ACE of `ace_type` for `sid` to the ACL at `acl`.
fn add_ace(emu: &mut Emu, acl: u64, ace_type: u8, mask: u32, sid: u64) -> u64 {
    let acl_size = emu.maps.read_word(acl + ACL_SIZE).unwrap_or(0) as u64;
    let count = emu.maps.read_word(acl + ACL_ACE_COUNT).unwrap_or(0);
    let sid_len = sid_length(emu, sid);
    // Walk existing ACEs to find the end.
    let mut off = ACL_HEADER_SIZE;
    for _ in 0..count {
        let sz = emu.maps.read_word(acl + off + 2).unwrap_or(0) as u64;
        if sz == 0 {
            break;
        }
        off += sz;
    }
    let ace_size = ACE_HEADER_SIZE + sid_len;
    if off + ace_size > acl_size {
        return STATUS_BUFFER_TOO_SMALL;
    }
    let ace = acl + off;
    emu.maps.write_byte(ace, ace_type);
    emu.maps.write_byte(ace + 1, 0);
    emu.maps.write_word(ace + 2, ace_size as u16);
    emu.maps.write_dword(ace + 4, mask);
    if sid_len > 0 {
        emu.maps.memcpy(ace + 8, sid, sid_len as usize);
    }
    emu.maps.write_word(acl + ACL_ACE_COUNT, count + 1);
    STATUS_SUCCESS
}

/// Total size of a self-relative image of the absolute SD at `sd`.
fn sd_self_relative_size(emu: &Emu, sd: u64) -> u64 {
    let mut total = SD_ABSOLUTE_SIZE;
    for part in [SD_OWNER, SD_GROUP] {
        let p = emu.maps.read_qword(sd + part).unwrap_or(0);
        total += sid_length(emu, p);
    }
    for part in [SD_SACL, SD_DACL] {
        let p = emu.maps.read_qword(sd + part).unwrap_or(0);
        if p != 0 && emu.maps.is_mapped(p) {
            total += emu.maps.read_word(p + ACL_SIZE).unwrap_or(0) as u64;
        }
    }
    total
}

/// RTL_OSVERSIONINFOW / EXW: Windows 10 22H2 workstation.
fn fill_version_info(emu: &mut Emu, info: u64) {
    let size = emu.maps.read_dword(info).unwrap_or(0x114);
    emu.maps.write_dword(info + 4, OS_MAJOR);
    emu.maps.write_dword(info + 8, OS_MINOR);
    emu.maps.write_dword(info + 0xc, NT_BUILD_NUMBER as u32);
    emu.maps.write_dword(info + 0x10, 2); // VER_PLATFORM_WIN32_NT
    emu.maps.write_word(info + 0x14, 0); // szCSDVersion = L""
    if size >= 0x11c {
        emu.maps.write_word(info + 0x114, 0); // wServicePackMajor
        emu.maps.write_word(info + 0x116, 0);
        emu.maps.write_word(info + 0x118, 0x100); // wSuiteMask: VER_SUITE_SINGLEUSERTS
        emu.maps.write_byte(info + 0x11a, 1); // VER_NT_WORKSTATION
    }
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- memory helpers (guarded) ------------------------------------------
        "RtlCopyMemory" | "RtlMoveMemory" | "RtlCopyMemoryNonTemporal" => RtlCopyMemory,
        "RtlZeroMemory" | "RtlSecureZeroMemory" => RtlZeroMemory,
        "RtlFillMemory" => RtlFillMemory,
        "RtlCompareMemory" => RtlCompareMemory,
        "RtlEqualMemory" => RtlEqualMemory,

        // --- counted strings ------------------------------------------------------
        "RtlInitUnicodeString" | "RtlInitUnicodeStringEx" => RtlInitUnicodeString,
        "RtlInitAnsiString" | "RtlInitString" | "RtlInitAnsiStringEx" | "RtlInitUTF8String" => RtlInitAnsiString,
        "RtlCopyUnicodeString" | "RtlCopyString" => RtlCopyUnicodeString,
        "RtlAppendUnicodeStringToString" => RtlAppendUnicodeStringToString,
        "RtlAppendUnicodeToString" => RtlAppendUnicodeToString,
        "RtlAppendStringToString" => RtlAppendStringToString,
        "RtlCompareUnicodeString" | "RtlCompareString" => RtlCompareUnicodeString,
        "RtlEqualUnicodeString" | "RtlEqualString" => RtlEqualUnicodeString,
        "RtlPrefixUnicodeString" | "RtlPrefixString" => RtlPrefixUnicodeString,
        "RtlUpcaseUnicodeString" | "RtlDowncaseUnicodeString" => RtlUpcaseUnicodeString,
        "RtlUpcaseUnicodeChar" | "RtlDowncaseUnicodeChar" => RtlUpcaseUnicodeChar,
        "RtlAnsiCharToUnicodeChar" => RtlAnsiCharToUnicodeChar,
        // RtlAnsiStringToUnicodeString(Dest, Source, AllocateDestinationString)
        "RtlAnsiStringToUnicodeString" | "RtlUTF8StringToUnicodeString" => RtlAnsiStringToUnicodeString,
        "RtlUnicodeStringToAnsiString" | "RtlUnicodeStringToUTF8String" => RtlUnicodeStringToAnsiString,
        "RtlFreeUnicodeString" | "RtlFreeAnsiString" | "RtlFreeUTF8String" | "RtlFreeOemString" => RtlFreeUnicodeString,
        "RtlUnicodeStringToInteger" | "RtlCharToInteger" => RtlUnicodeStringToInteger,
        "RtlIntegerToUnicodeString" | "RtlInt64ToUnicodeString" => RtlIntegerToUnicodeString,
        "RtlUnicodeToMultiByteN"
        | "RtlUpcaseUnicodeToMultiByteN"
        | "RtlUnicodeToOemN"
        | "RtlUnicodeToUTF8N" => RtlUnicodeToMultiByteN,
        "RtlMultiByteToUnicodeN" | "RtlOemToUnicodeN" | "RtlUTF8ToUnicodeN" => RtlMultiByteToUnicodeN,
        "RtlUnicodeToMultiByteSize"
        | "RtlxUnicodeStringToAnsiSize"
        | "RtlMultiByteToUnicodeSize"
        | "RtlxAnsiStringToUnicodeSize" => RtlUnicodeToMultiByteSize,
        "RtlHashUnicodeString" => RtlHashUnicodeString,
        "RtlStringFromGUID" => RtlStringFromGUID,
        "RtlGUIDFromString" => RtlGUIDFromString,

        // --- version ------------------------------------------------------------------
        "RtlGetVersion" => RtlGetVersion,
        "RtlVerifyVersionInfo" => return_zero,
        "RtlIsNtDdiVersionAvailable" | "RtlIsServicePackVersionInstalled" => return_one,
        "RtlGetNtVersionNumbers" => RtlGetNtVersionNumbers,
        "RtlGetNtProductType" => RtlGetNtProductType,

        // --- registry helpers ---------------------------------------------------------
        // Nothing is stored under the emulated registry, so queries report the
        // value as absent and the driver falls back to its defaults.
        "RtlQueryRegistryValues" | "RtlQueryRegistryValuesEx" => registry_query_stub,
        "RtlWriteRegistryValue"
        | "RtlCreateRegistryKey"
        | "RtlDeleteRegistryValue"
        | "RtlCheckRegistryKey" => registry_update_stub,
        "RtlFormatCurrentUserKeyPath" => RtlFormatCurrentUserKeyPath,

        // --- security descriptors -----------------------------------------------------
        "RtlCreateSecurityDescriptor" => RtlCreateSecurityDescriptor,
        "RtlSetDaclSecurityDescriptor" | "RtlSetSaclSecurityDescriptor" => RtlSetDaclSecurityDescriptor,
        "RtlSetOwnerSecurityDescriptor" | "RtlSetGroupSecurityDescriptor" => RtlSetOwnerSecurityDescriptor,
        "RtlGetDaclSecurityDescriptor" | "RtlGetSaclSecurityDescriptor" => RtlGetDaclSecurityDescriptor,
        "RtlGetOwnerSecurityDescriptor" | "RtlGetGroupSecurityDescriptor" => RtlGetOwnerSecurityDescriptor,
        "RtlLengthSecurityDescriptor" => RtlLengthSecurityDescriptor,
        "RtlValidSecurityDescriptor"
        | "RtlValidRelativeSecurityDescriptor"
        | "RtlValidSid"
        | "RtlValidAcl" => return_one,
        // RtlAbsoluteToSelfRelativeSD(Absolute, SelfRelative, *BufferLength)
        "RtlAbsoluteToSelfRelativeSD" | "RtlMakeSelfRelativeSD" => RtlAbsoluteToSelfRelativeSD,
        "RtlSelfRelativeToAbsoluteSD" | "RtlSelfRelativeToAbsoluteSD2" => return_not_supported,
        "RtlCreateAcl" => RtlCreateAcl,
        // RtlAddAccessAllowedAce(Acl, AceRevision, AccessMask, Sid)
        "RtlAddAccessAllowedAce"
        | "RtlAddAccessAllowedAceEx"
        | "RtlAddAccessDeniedAce"
        | "RtlAddAccessDeniedAceEx"
        | "RtlAddMandatoryAce" => RtlAddAccessAllowedAce,
        "RtlGetAce" => RtlGetAce,
        "RtlLengthSid" => RtlLengthSid,
        "RtlLengthRequiredSid" => RtlLengthRequiredSid,
        "RtlInitializeSid" | "RtlInitializeSidEx" => RtlInitializeSid,
        "RtlSubAuthoritySid" => RtlSubAuthoritySid,
        "RtlSubAuthorityCountSid" => RtlSubAuthorityCountSid,
        "RtlCopySid" => RtlCopySid,
        "RtlEqualSid" | "RtlEqualPrefixSid" => RtlEqualSid,
        "RtlConvertSidToUnicodeString" => RtlConvertSidToUnicodeString,

        // --- image helpers -------------------------------------------------------------
        "RtlImageNtHeader" | "RtlImageNtHeaderEx" => RtlImageNtHeader,
        // RtlImageDirectoryEntryToData(Base, MappedAsImage, Directory, *Size)
        "RtlImageDirectoryEntryToData" => RtlImageDirectoryEntryToData,
        "RtlLookupFunctionEntry" | "RtlLookupFunctionTable" | "RtlPcToFileHeader" => image_lookup_stub,
        "RtlVirtualUnwind"
        | "RtlWalkFrameChain"
        | "RtlCaptureStackBackTrace"
        | "RtlCaptureContext"
        | "RtlRestoreContext"
        | "RtlAddFunctionTable"
        | "RtlDeleteFunctionTable"
        | "RtlInstallFunctionTableCallback"
        | "RtlUnwind" => return_zero,
        "RtlUnwindEx" | "RtlRaiseException" | "RtlRaiseStatus" => exception_abort,
        "RtlAssert" => RtlAssert,

        // --- bitmaps --------------------------------------------------------------------
        "RtlInitializeBitMap" => RtlInitializeBitMap,
        "RtlClearAllBits" | "RtlSetAllBits" => RtlClearAllBits,
        "RtlSetBit" | "RtlClearBit" | "RtlTestBit" | "RtlCheckBit" => RtlSetBit,
        "RtlSetBits" | "RtlClearBits" => RtlSetBits,
        "RtlFindClearBits"
        | "RtlFindClearBitsAndSet"
        | "RtlFindSetBits"
        | "RtlFindSetBitsAndClear"
        | "RtlFindNextForwardRunClear"
        | "RtlFindFirstRunClear"
        | "RtlFindLongestRunClear" => RtlFindClearBits,
        "RtlNumberOfSetBits" | "RtlNumberOfClearBits" => RtlNumberOfSetBits,

        // --- time, random, misc -----------------------------------------------------------
        "RtlTimeToTimeFields" => RtlTimeToTimeFields,
        "RtlTimeFieldsToTime" => RtlTimeFieldsToTime,
        "RtlRandom" | "RtlRandomEx" => RtlRandom,
        "RtlQueryFeatureConfiguration"
        | "RtlQueryFeatureConfigurationChangeStamp"
        | "RtlRegisterFeatureConfigurationChangeNotification"
        | "RtlUnregisterFeatureConfigurationChangeNotification"
        | "RtlNotifyFeatureUsage" => feature_configuration_stub,
        "RtlIsGenericTableEmptyAvl" | "RtlIsGenericTableEmpty" => return_one,
        "RtlInitializeGenericTableAvl" | "RtlInitializeGenericTable" => table_initialize_stub,
        // RtlInsertElementGenericTableAvl(Table, Buffer, BufferSize, *NewElement)
        "RtlInsertElementGenericTableAvl" | "RtlInsertElementGenericTable" => table_insert_stub,
        "RtlLookupElementGenericTableAvl"
        | "RtlLookupElementGenericTable"
        | "RtlEnumerateGenericTableAvl"
        | "RtlEnumerateGenericTable"
        | "RtlEnumerateGenericTableWithoutSplayingAvl"
        | "RtlGetElementGenericTableAvl"
        | "RtlLookupFirstMatchingElementGenericTableAvl" => return_zero,
        "RtlDeleteElementGenericTableAvl" | "RtlDeleteElementGenericTable" => return_one,
        "RtlNumberGenericTableElementsAvl" | "RtlNumberGenericTableElements" => table_count_stub,
        "RtlIsNameLegalDOS8Dot3" => return_one,
        "RtlSetBitMapEx" | "RtlCopyLuid" => return_zero,

        // --- FsRtl --------------------------------------------------------------------
        // FsRtlIsNameInExpression(Expression, Name, IgnoreCase, UpcaseTable)
        "FsRtlIsNameInExpression" | "FsRtlIsNameInExpressionPrivate" => FsRtlIsNameInExpression,
        "FsRtlIsDbcsInExpression" => FsRtlIsDbcsInExpression,
        "FsRtlNotifyVolumeEvent"
        | "FsRtlNotifyVolumeEventEx"
        | "FsRtlNotifyCleanup"
        | "FsRtlNotifyCleanupAll"
        | "FsRtlNotifyFilterChangeDirectory"
        | "FsRtlNotifyFilterReportChange"
        | "FsRtlNotifyFullChangeDirectory"
        | "FsRtlNotifyFullReportChange"
        | "FsRtlNotifyInitializeSync"
        | "FsRtlNotifyUninitializeSync"
        | "FsRtlRegisterFileSystemFilterCallbacks"
        | "FsRtlRegisterUncProvider"
        | "FsRtlRegisterUncProviderEx"
        | "FsRtlDeregisterUncProvider"
        | "FsRtlCancellableWaitForSingleObject"
        | "FsRtlCancellableWaitForMultipleObjects"
        | "FsRtlFreeExtraCreateParameter"
        | "FsRtlFreeExtraCreateParameterList"
        | "FsRtlTeardownPerStreamContexts"
        | "FsRtlTeardownPerFileContexts"
        | "FsRtlRemovePerStreamContext"
        | "FsRtlInsertPerStreamContext"
        | "FsRtlAcquireFileExclusive"
        | "FsRtlReleaseFile"
        | "FsRtlPostPagingFileStackOverflow"
        | "FsRtlPostStackOverflow"
        | "FsRtlMdlReadDev"
        | "FsRtlMdlReadCompleteDev"
        | "FsRtlPrepareMdlWriteDev"
        | "FsRtlMdlWriteCompleteDev" => return_zero,
        "FsRtlAllocateExtraCreateParameterList"
        | "FsRtlAllocateExtraCreateParameter"
        | "FsRtlAllocateExtraCreateParameterFromLookasideList"
        | "FsRtlAllocatePoolWithTag"
        | "FsRtlAllocateResource" => filesystem_allocation_stub,
        "FsRtlIsNtstatusExpected" => return_one,
        "FsRtlIsPagingFile" | "FsRtlIsSystemPagingFile" => return_zero,
        "FsRtlAreNamesEqual" => FsRtlAreNamesEqual,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn RtlCopyMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    crt::checked_copy(emu, dst, src, len);
    call.set_return(emu, dst);
}

fn RtlZeroMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    crt::checked_fill(emu, dst, 0, len);
    call.set_return(emu, dst);
}

fn RtlFillMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let byte = api_try!(emu, call.arg_u8(emu, 2));
    crt::checked_fill(emu, dst, byte, len);
    call.set_return(emu, dst);
}

fn RtlCompareMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, a, len as u32, false);
    emu.kernel_guard_access(rip, b, len as u32, false);
    let mut n = 0;
    while n < len && emu.maps.read_byte(a + n) == emu.maps.read_byte(b + n) {
        n += 1;
    }
    call.set_return(emu, n);
}

fn RtlEqualMemory(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let eq = (0..len).all(|i| emu.maps.read_byte(a + i) == emu.maps.read_byte(b + i));
    call.set_return(emu, eq as u64);
}

fn RtlInitUnicodeString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    if src == 0 {
        write_unicode_header(emu, dst, 0, 0, 0);
    } else {
        let n = emu.maps.read_wide_string(src).encode_utf16().count() as u64 * 2;
        write_unicode_header(emu, dst, src, n, n + 2);
    }
    call.set_return(emu, 0);
}

fn RtlInitAnsiString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    if src == 0 {
        write_unicode_header(emu, dst, 0, 0, 0);
    } else {
        let n = emu.maps.read_string(src).len() as u64;
        write_unicode_header(emu, dst, src, n, n + 1);
    }
    call.set_return(emu, 0);
}

fn RtlCopyUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let max = unicode_max(emu, dst);
    let dbuf = unicode_buf(emu, dst);
    if src == 0 {
        emu.maps.write_word(dst, 0);
    } else {
        let n = unicode_len(emu, src).min(max);
        let sbuf = unicode_buf(emu, src);
        if n > 0 && dbuf != 0 && sbuf != 0 {
            crt::checked_copy(emu, dbuf, sbuf, n);
        }
        emu.maps.write_word(dst, n as u16);
        let cw = if symbol == "RtlCopyString" { 1 } else { 2 };
        if n + cw <= max && dbuf != 0 {
            if cw == 2 {
                emu.maps.write_word(dbuf + n, 0);
            } else {
                emu.maps.write_byte(dbuf + n, 0);
            }
        }
    }
    call.set_return(emu, 0);
}

fn RtlAppendUnicodeStringToString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = unicode_len(emu, src) as usize;
    let sbuf = unicode_buf(emu, src);
    let bytes: Vec<u8> = (0..n)
        .map(|i| emu.maps.read_byte(sbuf + i as u64).unwrap_or(0))
        .collect();
    let st = unicode_append(emu, dst, &bytes);
    call.set_return(emu, st);
}

fn RtlAppendUnicodeToString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let s = if src == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(src)
    };
    let st = unicode_append(emu, dst, &utf16_bytes(&s));
    call.set_return(emu, st);
}

fn RtlAppendStringToString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let s = read_ansi_string(emu, src);
    let st = unicode_append(emu, dst, s.as_bytes());
    call.set_return(emu, st);
}

fn RtlCompareUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ci = api_try!(emu, call.arg(emu, 2)) != 0;
    let (a, b) = if symbol == "RtlCompareString" {
        (
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    } else {
        (
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    };
    call.set_return(emu, compare_strings(&a, &b, ci) as u64);
}

fn RtlEqualUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ci = api_try!(emu, call.arg(emu, 2)) != 0;
    let (a, b) = if symbol == "RtlEqualString" {
        (
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    } else {
        (
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    };
    call.set_return(emu, (compare_strings(&a, &b, ci) == 0) as u64);
}

fn RtlPrefixUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ci = api_try!(emu, call.arg(emu, 2)) != 0;
    let (p, s) = if symbol == "RtlPrefixString" {
        (
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_ansi_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    } else {
        (
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 0))),
            read_unicode_string(emu, api_try!(emu, call.arg(emu, 1))),
        )
    };
    let ok = if ci {
        s.to_lowercase().starts_with(&p.to_lowercase())
    } else {
        s.starts_with(&p)
    };
    call.set_return(emu, ok as u64);
}

fn RtlUpcaseUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let alloc = api_try!(emu, call.arg(emu, 2)) != 0;
    let s = read_unicode_string(emu, src);
    let s = if symbol == "RtlUpcaseUnicodeString" {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    };
    if alloc {
        let (buf, len) = alloc_wide(emu, &s, symbol);
        write_unicode_header(emu, dst, buf, len, len + 2);
    } else {
        let bytes = utf16_bytes(&s);
        emu.maps.write_word(dst, 0);
        unicode_append(emu, dst, &bytes);
    }
    call.set_return(emu, 0);
}

fn RtlUpcaseUnicodeChar(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let c = api_try!(emu, call.arg_u16(emu, 0));
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let r = if symbol == "RtlUpcaseUnicodeChar" {
        ch.to_uppercase().next().unwrap_or(ch)
    } else {
        ch.to_lowercase().next().unwrap_or(ch)
    };
    call.set_return(emu, r as u64);
}

fn RtlAnsiCharToUnicodeChar(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let pp = api_try!(emu, call.arg(emu, 0));
    let p = emu.maps.read_qword(pp).unwrap_or(0);
    let c = emu.maps.read_byte(p).unwrap_or(0);
    emu.maps.write_qword(pp, p + 1);
    call.set_return(emu, c as u64);
}

fn RtlAnsiStringToUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let alloc = api_try!(emu, call.arg(emu, 2)) != 0;
    let s = read_ansi_string(emu, src);
    if alloc {
        let (buf, len) = alloc_wide(emu, &s, symbol);
        write_unicode_header(emu, dst, buf, len, len + 2);
        call.set_return(emu, if buf != 0 { 0 } else { STATUS_NO_MEMORY });
    } else {
        emu.maps.write_word(dst, 0);
        let st = unicode_append(emu, dst, &utf16_bytes(&s));
        call.set_return(emu, st);
    }
}

fn RtlUnicodeStringToAnsiString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let alloc = api_try!(emu, call.arg(emu, 2)) != 0;
    let s = read_unicode_string(emu, src);
    if alloc {
        let buf = emu.kernel_alloc(
            Region::Slab,
            s.len() as u64 + 1,
            "ansi-string",
            symbol,
            true,
        );
        if buf != 0 {
            emu.maps.write_string(buf, &s);
        }
        write_unicode_header(emu, dst, buf, s.len() as u64, s.len() as u64 + 1);
        call.set_return(emu, if buf != 0 { 0 } else { STATUS_NO_MEMORY });
    } else {
        emu.maps.write_word(dst, 0);
        let st = unicode_append(emu, dst, s.as_bytes());
        call.set_return(emu, st);
    }
}

fn RtlFreeUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let us = api_try!(emu, call.arg(emu, 0));
    let buf = unicode_buf(emu, us);
    if buf != 0 {
        emu.kernel_free(buf, symbol);
        write_unicode_header(emu, us, 0, 0, 0);
    }
    call.set_return(emu, 0);
}

fn RtlUnicodeStringToInteger(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg_u32(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    let s = if symbol == "RtlCharToInteger" {
        emu.maps.read_string(api_try!(emu, call.arg(emu, 0)))
    } else {
        read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)))
    };
    let t = s.trim();
    let (digits, radix) = if base == 0 {
        if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            (h, 16)
        } else if let Some(o) = t.strip_prefix("0o") {
            (o, 8)
        } else if let Some(b) = t.strip_prefix("0b") {
            (b, 2)
        } else {
            (t, 10)
        }
    } else {
        (t, base)
    };
    let end = digits
        .find(|c: char| !c.is_digit(radix))
        .unwrap_or(digits.len());
    let v = u32::from_str_radix(&digits[..end], radix).unwrap_or(0);
    write_out_dword(emu, out, v);
    call.set_return(emu, 0);
}

fn RtlIntegerToUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let v = if symbol == "RtlIntegerToUnicodeString" {
        api_try!(emu, call.arg_u32(emu, 0)) as u64
    } else {
        api_try!(emu, call.arg(emu, 0))
    };
    let base = api_try!(emu, call.arg_u32(emu, 1));
    let dst = api_try!(emu, call.arg(emu, 2));
    let s = match base {
        16 => format!("{:x}", v),
        8 => format!("{:o}", v),
        2 => format!("{:b}", v),
        _ => v.to_string(),
    };
    emu.maps.write_word(dst, 0);
    let st = unicode_append(emu, dst, &utf16_bytes(&s));
    call.set_return(emu, st);
}

fn RtlUnicodeToMultiByteN(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // (MultiByteString, MaxBytes, *BytesInMultiByte, UnicodeString, BytesInUnicode)
    let dst = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    let src = api_try!(emu, call.arg(emu, 3));
    let n = (api_try!(emu, call.arg_u32(emu, 4)) as u64) / 2;
    let s = emu.maps.read_wide_string_n(src, n as usize);
    let s = if symbol.contains("Upcase") {
        s.to_uppercase()
    } else {
        s
    };
    let bytes = s.as_bytes();
    let n = (bytes.len() as u64).min(max);
    emu.maps.write_bytes(dst, &bytes[..n as usize]);
    write_out_dword(emu, out, n as u32);
    call.set_return(emu, 0);
}

fn RtlMultiByteToUnicodeN(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // (UnicodeString, MaxBytes, *BytesInUnicode, MultiByteString, BytesInMultiByte)
    let dst = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    let src = api_try!(emu, call.arg(emu, 3));
    let n = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let bytes: Vec<u8> = (0..n)
        .map(|i| emu.maps.read_byte(src + i).unwrap_or(0))
        .collect();
    let s = String::from_utf8_lossy(&bytes).to_string();
    let wide = utf16_bytes(&s);
    let n = (wide.len() as u64).min(max);
    emu.maps.write_bytes(dst, &wide[..n as usize]);
    write_out_dword(emu, out, n as u32);
    call.set_return(emu, 0);
}

fn RtlUnicodeToMultiByteSize(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let v = match symbol {
        "RtlUnicodeToMultiByteSize" => {
            let out = api_try!(emu, call.arg(emu, 0));
            let n = api_try!(emu, call.arg(emu, 2)) / 2;
            write_out_dword(emu, out, n as u32);
            0
        }
        "RtlMultiByteToUnicodeSize" => {
            let out = api_try!(emu, call.arg(emu, 0));
            let n = api_try!(emu, call.arg(emu, 2)) * 2;
            write_out_dword(emu, out, n as u32);
            0
        }
        "RtlxUnicodeStringToAnsiSize" => unicode_len(emu, api_try!(emu, call.arg(emu, 0))) / 2 + 1,
        _ => unicode_len(emu, api_try!(emu, call.arg(emu, 0))) * 2 + 2,
    };
    call.set_return(emu, v);
}

fn RtlHashUnicodeString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let s = read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)));
    let ci = api_try!(emu, call.arg(emu, 1)) != 0;
    let out = api_try!(emu, call.arg(emu, 3));
    let s = if ci { s.to_uppercase() } else { s };
    let mut h: u32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(65599).wrapping_add(c as u32);
    }
    write_out_dword(emu, out, h);
    call.set_return(emu, 0);
}

fn RtlStringFromGUID(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let guid = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    let d1 = emu.maps.read_dword(guid).unwrap_or(0);
    let d2 = emu.maps.read_word(guid + 4).unwrap_or(0);
    let d3 = emu.maps.read_word(guid + 6).unwrap_or(0);
    let b: Vec<u8> = (0..8)
        .map(|i| emu.maps.read_byte(guid + 8 + i).unwrap_or(0))
        .collect();
    let s = format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        d1, d2, d3, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]
    );
    let (buf, len) = alloc_wide(emu, &s, symbol);
    write_unicode_header(emu, dst, buf, len, len + 2);
    call.set_return(emu, 0);
}

fn RtlGUIDFromString(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let s = read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)));
    let out = api_try!(emu, call.arg(emu, 1));
    let hex: Vec<u8> = s
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<String>()
        .as_bytes()
        .chunks(2)
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap_or("0"), 16).unwrap_or(0))
        .collect();
    if hex.len() == 16 {
        emu.maps
            .write_dword(out, u32::from_be_bytes([hex[0], hex[1], hex[2], hex[3]]));
        emu.maps
            .write_word(out + 4, u16::from_be_bytes([hex[4], hex[5]]));
        emu.maps
            .write_word(out + 6, u16::from_be_bytes([hex[6], hex[7]]));
        emu.maps.write_bytes(out + 8, &hex[8..16]);
        call.set_return(emu, 0);
    } else {
        call.set_return(emu, STATUS_INVALID_PARAMETER);
    }
}

fn RtlGetVersion(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let info = api_try!(emu, call.arg(emu, 0));
    fill_version_info(emu, info);
    call.set_return(emu, 0);
}

fn RtlGetNtVersionNumbers(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let (a, b, c) = (
        api_try!(emu, call.arg(emu, 0)),
        api_try!(emu, call.arg(emu, 1)),
        api_try!(emu, call.arg(emu, 2)),
    );
    write_out_dword(emu, a, OS_MAJOR);
    write_out_dword(emu, b, OS_MINOR);
    write_out_dword(emu, c, NT_BUILD_NUMBER as u32 | 0xF000_0000);
    call.set_return(emu, 0);
}

fn RtlGetNtProductType(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    write_out_dword(emu, out, 1);
    call.set_return(emu, 1);
}

fn registry_query_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let path = api_try!(emu, call.arg(emu, 1));
    let name = if path != 0 {
        emu.maps.read_wide_string(path)
    } else {
        String::new()
    };
    log::info!("{}: '{}'", symbol, name);
    call.set_return(emu, STATUS_OBJECT_NAME_NOT_FOUND);
}

fn registry_update_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let path = api_try!(emu, call.arg(emu, 1));
    let name = if path != 0 {
        emu.maps.read_wide_string(path)
    } else {
        String::new()
    };
    log::info!("{}: '{}'", symbol, name);
    call.set_return(emu, 0);
}

fn RtlFormatCurrentUserKeyPath(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let (buf, len) = alloc_wide(emu, "\\Registry\\User\\.DEFAULT", symbol);
    write_unicode_header(emu, dst, buf, len, len + 2);
    call.set_return(emu, 0);
}

fn RtlCreateSecurityDescriptor(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    let rev = api_try!(emu, call.arg_u8(emu, 1));
    emu.maps.write_bytes(sd, &[0u8; SD_ABSOLUTE_SIZE as usize]);
    emu.maps.write_byte(sd + SD_REVISION, rev);
    call.set_return(emu, 0);
}

fn RtlSetDaclSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    let present = api_try!(emu, call.arg(emu, 1)) != 0;
    let acl = api_try!(emu, call.arg(emu, 2));
    let defaulted = api_try!(emu, call.arg(emu, 3)) != 0;
    let (off, present_bit, def_bit) = if symbol == "RtlSetDaclSecurityDescriptor" {
        (SD_DACL, SE_DACL_PRESENT, SE_DACL_DEFAULTED)
    } else {
        (SD_SACL, SE_SACL_PRESENT, 0x20)
    };
    let mut ctl = emu.maps.read_word(sd + SD_CONTROL).unwrap_or(0) & !(present_bit | def_bit);
    if present {
        ctl |= present_bit;
        emu.maps.write_qword(sd + off, acl);
        if defaulted {
            ctl |= def_bit;
        }
    } else {
        emu.maps.write_qword(sd + off, 0);
    }
    emu.maps.write_word(sd + SD_CONTROL, ctl);
    call.set_return(emu, 0);
}

fn RtlSetOwnerSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    let sid = api_try!(emu, call.arg(emu, 1));
    let off = if symbol == "RtlSetOwnerSecurityDescriptor" {
        SD_OWNER
    } else {
        SD_GROUP
    };
    emu.maps.write_qword(sd + off, sid);
    call.set_return(emu, 0);
}

fn RtlGetDaclSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // (SD, *Present, *Acl, *Defaulted)
    let sd = api_try!(emu, call.arg(emu, 0));
    let present_out = api_try!(emu, call.arg(emu, 1));
    let acl_out = api_try!(emu, call.arg(emu, 2));
    let def_out = api_try!(emu, call.arg(emu, 3));
    let (off, bit) = if symbol == "RtlGetDaclSecurityDescriptor" {
        (SD_DACL, SE_DACL_PRESENT)
    } else {
        (SD_SACL, SE_SACL_PRESENT)
    };
    let ctl = emu.maps.read_word(sd + SD_CONTROL).unwrap_or(0);
    let present = ctl & bit != 0;
    let acl = if present {
        emu.maps.read_qword(sd + off).unwrap_or(0)
    } else {
        0
    };
    if present_out != 0 {
        emu.maps.write_byte(present_out, present as u8);
    }
    write_out_qword(emu, acl_out, acl);
    if def_out != 0 {
        emu.maps.write_byte(def_out, 0);
    }
    call.set_return(emu, 0);
}

fn RtlGetOwnerSecurityDescriptor(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    let sid_out = api_try!(emu, call.arg(emu, 1));
    let def_out = api_try!(emu, call.arg(emu, 2));
    let off = if symbol == "RtlGetOwnerSecurityDescriptor" {
        SD_OWNER
    } else {
        SD_GROUP
    };
    let sid = emu.maps.read_qword(sd + off).unwrap_or(0);
    write_out_qword(emu, sid_out, sid);
    if def_out != 0 {
        emu.maps.write_byte(def_out, 0);
    }
    call.set_return(emu, 0);
}

fn RtlLengthSecurityDescriptor(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    let n = sd_self_relative_size(emu, sd);
    call.set_return(emu, n);
}

fn RtlAbsoluteToSelfRelativeSD(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let abs = api_try!(emu, call.arg(emu, 0));
    let rel = api_try!(emu, call.arg(emu, 1));
    let len_ptr = api_try!(emu, call.arg(emu, 2));
    let need = sd_self_relative_size(emu, abs);
    let have = emu.maps.read_dword(len_ptr).unwrap_or(0) as u64;
    if have < need || rel == 0 {
        write_out_dword(emu, len_ptr, need as u32);
        call.set_return(emu, STATUS_BUFFER_TOO_SMALL);
        return;
    }
    // Self-relative header is 0x14 bytes: Revision, Sbz1, Control,
    // then four u32 offsets (Owner, Group, Sacl, Dacl).
    emu.maps.write_bytes(rel, &vec![0u8; need as usize]);
    emu.maps
        .write_byte(rel, emu.maps.read_byte(abs).unwrap_or(1));
    let ctl = emu.maps.read_word(abs + SD_CONTROL).unwrap_or(0) | SE_SELF_RELATIVE;
    emu.maps.write_word(rel + 2, ctl);
    let mut cursor = 0x14u64;
    for (i, part) in [SD_OWNER, SD_GROUP, SD_SACL, SD_DACL].iter().enumerate() {
        let p = emu.maps.read_qword(abs + part).unwrap_or(0);
        if p == 0 || !emu.maps.is_mapped(p) {
            continue;
        }
        let n = if i < 2 {
            sid_length(emu, p)
        } else {
            emu.maps.read_word(p + ACL_SIZE).unwrap_or(0) as u64
        };
        emu.maps.memcpy(rel + cursor, p, n as usize);
        emu.maps.write_dword(rel + 4 + i as u64 * 4, cursor as u32);
        cursor += n;
    }
    write_out_dword(emu, len_ptr, need as u32);
    call.set_return(emu, 0);
}

fn return_not_supported(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_NOT_SUPPORTED);
}

fn RtlCreateAcl(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let acl = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let rev = api_try!(emu, call.arg_u8(emu, 2));
    emu.maps
        .write_bytes(acl, &vec![0u8; len.min(0x10000) as usize]);
    emu.maps.write_byte(acl, rev);
    emu.maps.write_word(acl + ACL_SIZE, len as u16);
    emu.maps.write_word(acl + ACL_ACE_COUNT, 0);
    call.set_return(emu, 0);
}

fn RtlAddAccessAllowedAce(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let acl = api_try!(emu, call.arg(emu, 0));
    let ex = symbol.ends_with("Ex") || symbol == "RtlAddMandatoryAce";
    let (mask, sid) = if ex {
        (
            api_try!(emu, call.arg_u32(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
        )
    } else {
        (
            api_try!(emu, call.arg_u32(emu, 2)),
            api_try!(emu, call.arg(emu, 3)),
        )
    };
    let ace_type = if symbol.contains("Denied") {
        1
    } else if symbol.contains("Mandatory") {
        0x11
    } else {
        0
    };
    let st = add_ace(emu, acl, ace_type, mask, sid);
    call.set_return(emu, st);
}

fn RtlGetAce(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let acl = api_try!(emu, call.arg(emu, 0));
    let idx = api_try!(emu, call.arg(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    let count = emu.maps.read_word(acl + ACL_ACE_COUNT).unwrap_or(0) as u64;
    if idx >= count {
        call.set_return(emu, STATUS_INVALID_PARAMETER);
        return;
    }
    let mut off = ACL_HEADER_SIZE;
    for _ in 0..idx {
        off += emu.maps.read_word(acl + off + 2).unwrap_or(0) as u64;
    }
    write_out_qword(emu, out, acl + off);
    call.set_return(emu, 0);
}

fn RtlLengthSid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sid = api_try!(emu, call.arg(emu, 0));
    let n = sid_length(emu, sid);
    call.set_return(emu, n);
}

fn RtlLengthRequiredSid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let n = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, 8 + 4 * n);
}

fn RtlInitializeSid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sid = api_try!(emu, call.arg(emu, 0));
    let auth = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u8(emu, 2));
    emu.maps.write_byte(sid, 1);
    emu.maps.write_byte(sid + 1, count);
    if auth != 0 {
        emu.maps.memcpy(sid + 2, auth, 6);
    }
    call.set_return(emu, 0);
}

fn RtlSubAuthoritySid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sid = api_try!(emu, call.arg(emu, 0));
    let i = api_try!(emu, call.arg(emu, 1));
    call.set_return(emu, sid + 8 + 4 * i);
}

fn RtlSubAuthorityCountSid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let sid = api_try!(emu, call.arg(emu, 0));
    call.set_return(emu, sid + 1);
}

fn RtlCopySid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let len = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    let src = api_try!(emu, call.arg(emu, 2));
    let n = sid_length(emu, src);
    if n > len {
        call.set_return(emu, STATUS_BUFFER_TOO_SMALL);
    } else {
        emu.maps.memcpy(dst, src, n as usize);
        call.set_return(emu, 0);
    }
}

fn RtlEqualSid(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let a = api_try!(emu, call.arg(emu, 0));
    let b = api_try!(emu, call.arg(emu, 1));
    let n = sid_length(emu, a);
    let eq = n == sid_length(emu, b)
        && (0..n).all(|i| emu.maps.read_byte(a + i) == emu.maps.read_byte(b + i));
    call.set_return(emu, eq as u64);
}

fn RtlConvertSidToUnicodeString(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let sid = api_try!(emu, call.arg(emu, 1));
    let alloc = api_try!(emu, call.arg(emu, 2)) != 0;
    let count = emu.maps.read_byte(sid + 1).unwrap_or(0) as u64;
    let auth = emu.maps.read_byte(sid + 7).unwrap_or(0);
    let mut s = format!("S-1-{}", auth);
    for i in 0..count {
        s.push_str(&format!(
            "-{}",
            emu.maps.read_dword(sid + 8 + 4 * i).unwrap_or(0)
        ));
    }
    if alloc {
        let (buf, len) = alloc_wide(emu, &s, symbol);
        write_unicode_header(emu, dst, buf, len, len + 2);
    } else {
        emu.maps.write_word(dst, 0);
        unicode_append(emu, dst, &utf16_bytes(&s));
    }
    call.set_return(emu, 0);
}

fn RtlImageNtHeader(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let base = if symbol == "RtlImageNtHeader" {
        api_try!(emu, call.arg(emu, 0))
    } else {
        api_try!(emu, call.arg(emu, 1))
    };
    let out = api_try!(emu, call.arg(emu, 3));
    let nt = if emu.maps.read_word(base) == Some(0x5a4d) {
        base + emu.maps.read_dword(base + 0x3c).unwrap_or(0) as u64
    } else {
        0
    };
    if symbol == "RtlImageNtHeader" {
        call.set_return(emu, nt);
    } else {
        write_out_qword(emu, out, nt);
        call.set_return(emu, if nt != 0 { 0 } else { 0xC000007B });
    }
}

fn RtlImageDirectoryEntryToData(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let dir = api_try!(emu, call.arg(emu, 2));
    let size_out = api_try!(emu, call.arg(emu, 3));
    let mut result = 0;
    if emu.maps.read_word(base) == Some(0x5a4d) {
        let nt = base + emu.maps.read_dword(base + 0x3c).unwrap_or(0) as u64;
        let magic = emu.maps.read_word(nt + 0x18).unwrap_or(0);
        let dirs = if magic == 0x20b {
            nt + 0x18 + 0x70
        } else {
            nt + 0x18 + 0x60
        };
        let rva = emu.maps.read_dword(dirs + dir * 8).unwrap_or(0) as u64;
        let size = emu.maps.read_dword(dirs + dir * 8 + 4).unwrap_or(0);
        if rva != 0 {
            result = base + rva;
            write_out_dword(emu, size_out, size);
        }
    }
    call.set_return(emu, result);
}

fn image_lookup_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    if symbol == "RtlPcToFileHeader" {
        let out = api_try!(emu, call.arg(emu, 1));
        let base = emu.kernel.as_ref().map(|k| k.module.base).unwrap_or(0);
        write_out_qword(emu, out, base);
        call.set_return(emu, base);
    } else {
        call.set_return(emu, 0);
    }
}

fn exception_abort(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let what = if symbol == "RtlRaiseStatus" {
        format!("status 0x{:x}", api_try!(emu, call.arg(emu, 0)))
    } else {
        "exception record".to_string()
    };
    emu.kernel_abort(format!(
        "{} ({}) — no SEH dispatch in kernel mode",
        symbol, what
    ));
}

fn RtlAssert(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let msg = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let file = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let line = api_try!(emu, call.arg(emu, 2));
    emu.kernel_log_line(format!("ASSERT {} at {}:{}", msg, file, line));
    call.set_return(emu, 0);
}

fn RtlInitializeBitMap(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let bits = api_try!(emu, call.arg_u32(emu, 2));
    emu.maps.write_dword(bm, bits);
    emu.maps.write_qword(bm + 8, buf);
    call.set_return(emu, 0);
}

fn RtlClearAllBits(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
    let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
    let v = if symbol == "RtlSetAllBits" { 0xff } else { 0 };
    emu.maps
        .write_bytes(buf, &vec![v; bits.div_ceil(8) as usize]);
    call.set_return(emu, 0);
}

fn RtlSetBit(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let bit = api_try!(emu, call.arg(emu, 1));
    let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
    let byte = buf + bit / 8;
    let mut v = emu.maps.read_byte(byte).unwrap_or(0);
    let mask = 1u8 << (bit % 8);
    match symbol {
        "RtlSetBit" => {
            v |= mask;
            emu.maps.write_byte(byte, v);
            call.set_return(emu, 0);
        }
        "RtlClearBit" => {
            v &= !mask;
            emu.maps.write_byte(byte, v);
            call.set_return(emu, 0);
        }
        _ => call.set_return(emu, (v & mask != 0) as u64),
    }
}

fn RtlSetBits(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let start = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2));
    let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
    for bit in start..start + n {
        let byte = buf + bit / 8;
        let mut v = emu.maps.read_byte(byte).unwrap_or(0);
        if symbol == "RtlSetBits" {
            v |= 1 << (bit % 8);
        } else {
            v &= !(1 << (bit % 8));
        }
        emu.maps.write_byte(byte, v);
    }
    call.set_return(emu, 0);
}

fn RtlFindClearBits(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let n = api_try!(emu, call.arg(emu, 1)).max(1);
    let want_set = symbol.contains("SetBits");
    let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
    let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
    let get = |emu: &Emu, bit: u64| {
        emu.maps.read_byte(buf + bit / 8).unwrap_or(0) & (1 << (bit % 8)) != 0
    };
    let mut found = 0xffff_ffffu64;
    let mut run = 0;
    for bit in 0..bits {
        if get(emu, bit) == want_set {
            run += 1;
            if run == n {
                found = bit + 1 - n;
                break;
            }
        } else {
            run = 0;
        }
    }
    if found != 0xffff_ffff && (symbol.ends_with("AndSet") || symbol.ends_with("AndClear")) {
        for bit in found..found + n {
            let byte = buf + bit / 8;
            let mut v = emu.maps.read_byte(byte).unwrap_or(0);
            if symbol.ends_with("AndSet") {
                v |= 1 << (bit % 8);
            } else {
                v &= !(1 << (bit % 8));
            }
            emu.maps.write_byte(byte, v);
        }
    }
    call.set_return(emu, found);
}

fn RtlNumberOfSetBits(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let bm = api_try!(emu, call.arg(emu, 0));
    let bits = emu.maps.read_dword(bm).unwrap_or(0) as u64;
    let buf = emu.maps.read_qword(bm + 8).unwrap_or(0);
    let set = (0..bits)
        .filter(|bit| emu.maps.read_byte(buf + bit / 8).unwrap_or(0) & (1 << (bit % 8)) != 0)
        .count() as u64;
    call.set_return(
        emu,
        if symbol == "RtlNumberOfSetBits" {
            set
        } else {
            bits - set
        },
    );
}

fn RtlTimeToTimeFields(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let t = emu
        .maps
        .read_qword(api_try!(emu, call.arg(emu, 0)))
        .unwrap_or(0);
    let out = api_try!(emu, call.arg(emu, 1));
    // Only the fields a log line needs: seconds since 1601 split up.
    let secs = t / 10_000_000;
    let days = secs / 86400;
    let (y, m, d) = civil_from_days(days as i64 - 134774); // 1601 -> 1970 offset
    emu.maps.write_word(out, y as u16);
    emu.maps.write_word(out + 2, m as u16);
    emu.maps.write_word(out + 4, d as u16);
    emu.maps.write_word(out + 6, ((secs / 3600) % 24) as u16);
    emu.maps.write_word(out + 8, ((secs / 60) % 60) as u16);
    emu.maps.write_word(out + 10, (secs % 60) as u16);
    emu.maps.write_word(out + 12, ((t / 10_000) % 1000) as u16);
    emu.maps.write_word(out + 14, ((days + 1) % 7) as u16);
    call.set_return(emu, 0);
}

fn RtlTimeFieldsToTime(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 1));
    emu.maps.write_qword(out, system_time_now());
    call.set_return(emu, 1);
}

fn RtlRandom(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let seed_ptr = api_try!(emu, call.arg(emu, 0));
    let seed = emu.maps.read_dword(seed_ptr).unwrap_or(1);
    let next = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
    emu.maps.write_dword(seed_ptr, next);
    call.set_return(emu, (next >> 1) as u64);
}

fn feature_configuration_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(
        emu,
        if symbol == "RtlQueryFeatureConfiguration" {
            STATUS_NOT_FOUND
        } else {
            0
        },
    );
}

fn table_initialize_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let t = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_bytes(t, &[0u8; 0x40]);
    call.set_return(emu, 0);
}

fn table_insert_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    let new_out = api_try!(emu, call.arg(emu, 3));
    let e = emu.kernel_alloc(Region::Slab, size.max(1), "generic-table", symbol, true);
    if e != 0 {
        emu.maps.memcpy(e, buf, size as usize);
    }
    if new_out != 0 {
        emu.maps.write_byte(new_out, 1);
    }
    call.set_return(emu, e);
}

fn table_count_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 0);
}

fn FsRtlIsNameInExpression(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let expr_us = api_try!(emu, call.arg(emu, 0));
    let name_us = api_try!(emu, call.arg(emu, 1));
    let ignore_case = api_try!(emu, call.arg(emu, 2)) != 0;
    let expr = read_unicode_string(emu, expr_us);
    let name = read_unicode_string(emu, name_us);
    let result = fsrtl_name_match(&expr, &name, ignore_case);
    log::info!(
        "FsRtlIsNameInExpression: '{}' vs '{}' case_insensitive={} -> {}",
        expr,
        name,
        ignore_case,
        result
    );
    call.set_return(emu, result as u64);
}

fn FsRtlIsDbcsInExpression(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let expr = read_ansi_string(emu, api_try!(emu, call.arg(emu, 0)));
    let name = read_ansi_string(emu, api_try!(emu, call.arg(emu, 1)));
    let result = fsrtl_name_match(&expr, &name, false);
    call.set_return(emu, result as u64);
}

fn filesystem_allocation_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = match symbol {
        "FsRtlAllocatePoolWithTag" => api_try!(emu, call.arg(emu, 1)).max(1),
        _ => 0x100,
    };
    let p = emu.kernel_alloc(Region::Slab, size, "fsrtl", symbol, true);
    call.set_return(emu, p);
}

fn FsRtlAreNamesEqual(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let a = read_unicode_string(emu, api_try!(emu, call.arg(emu, 0)));
    let b = read_unicode_string(emu, api_try!(emu, call.arg(emu, 1)));
    let ignore_case = api_try!(emu, call.arg(emu, 2)) != 0;
    let eq = if ignore_case {
        a.eq_ignore_ascii_case(&b)
    } else {
        a == b
    };
    call.set_return(emu, eq as u64);
}

/// NTFS/FsRtl wildcard matching: `*` matches any sequence, `?` matches
/// exactly one character, `"` matches any single character including `.`,
/// and the DOS_DOT (`"`) / DOS_STAR (`<`) / DOS_QM (`>`) semantics from
/// the Windows kernel.
fn fsrtl_name_match(expression: &str, name: &str, ignore_case: bool) -> bool {
    let expr: Vec<char> = expression.chars().collect();
    let name: Vec<char> = if ignore_case {
        name.to_uppercase().chars().collect()
    } else {
        name.chars().collect()
    };
    let expr: Vec<char> = if ignore_case {
        expression.to_uppercase().chars().collect()
    } else {
        expr
    };
    fsrtl_match_recursive(&expr, &name, 0, 0)
}

fn fsrtl_match_recursive(expr: &[char], name: &[char], mut ei: usize, mut ni: usize) -> bool {
    while ei < expr.len() {
        match expr[ei] {
            '*' => {
                ei += 1;
                if ei == expr.len() {
                    return true;
                }
                for ni2 in ni..=name.len() {
                    if fsrtl_match_recursive(expr, name, ei, ni2) {
                        return true;
                    }
                }
                return false;
            }
            '?' => {
                if ni >= name.len() {
                    return false;
                }
                ei += 1;
                ni += 1;
            }
            // DOS_DOT: matches `.` or the end-of-name when the name has no
            // trailing dot.
            '"' => {
                if ni < name.len() && name[ni] == '.' {
                    ni += 1;
                }
                ei += 1;
            }
            // DOS_STAR `<`: matches zero or more characters until the last `.`.
            '<' => {
                ei += 1;
                if let Some(last_dot) = name[ni..].iter().rposition(|c| *c == '.') {
                    ni += last_dot;
                } else {
                    ni = name.len();
                }
            }
            // DOS_QM `>`: matches any single character, or nothing at end.
            '>' => {
                if ni < name.len() && name[ni] != '.' {
                    ni += 1;
                }
                ei += 1;
            }
            c => {
                if ni >= name.len() || name[ni] != c {
                    return false;
                }
                ei += 1;
                ni += 1;
            }
        }
    }
    ni == name.len()
}

/// Howard Hinnant's days-to-civil, for `RtlTimeToTimeFields`.
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}
