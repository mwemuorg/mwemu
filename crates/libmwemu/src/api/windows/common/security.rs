use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::security_descriptor::descriptor;
use crate::winapi::common::trace;
use crate::winapi::common::{buffers, heap, memory, process};
use crate::winapi::environment_store::Encoding;
use crate::winapi::libraries::advapi32::add_access_ace;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::{
    PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION,
};
use crate::windows::constants::security::*;
use std::fmt::Write;

const SID_REVISION: u8 = 1;
const SID_MAX_SUB_AUTHORITIES: u8 = 15;
const SID_HEADER_SIZE: usize = 8;
const PROCESS_USER_SUBAUTHORITIES: [u32; 5] = [21, 1234567890, 1234567890, 1234567890, 1001];
const NT_AUTHORITY: u8 = 5;

pub(crate) fn process_user_sid() -> Vec<u8> {
    let mut sid = vec![
        SID_REVISION,
        PROCESS_USER_SUBAUTHORITIES.len() as u8,
        0,
        0,
        0,
        0,
        0,
        NT_AUTHORITY,
    ];
    sid.extend(
        PROCESS_USER_SUBAUTHORITIES
            .iter()
            .flat_map(|value| value.to_le_bytes()),
    );
    sid
}

pub(crate) fn read_sid(emu: &Emu, address: u64) -> Option<Vec<u8>> {
    let header: [u8; SID_HEADER_SIZE] = memory::read_array(emu, address)?;
    if header[0] != SID_REVISION || header[1] > SID_MAX_SUB_AUTHORITIES {
        return None;
    }
    let length = sid_size(header[1]);
    (0..length)
        .map(|offset| emu.maps.read_byte(address.checked_add(offset as u64)?))
        .collect()
}

pub(crate) fn sid_string(sid: &[u8]) -> String {
    let authority = sid[2..SID_HEADER_SIZE]
        .iter()
        .fold(0u64, |value, &byte| (value << 8) | u64::from(byte));
    let mut text = if authority > u32::MAX as u64 {
        format!("S-1-0x{authority:012x}")
    } else {
        format!("S-1-{authority}")
    };
    for sub in sid[SID_HEADER_SIZE..].chunks_exact(4) {
        let _ = write!(
            text,
            "-{}",
            u32::from_le_bytes([sub[0], sub[1], sub[2], sub[3]])
        );
    }
    text
}

pub fn ConvertSidToStringSidA(emu: &mut Emu) {
    convert_sid_to_string(emu, false);
}
pub fn ConvertSidToStringSidW(emu: &mut Emu) {
    convert_sid_to_string(emu, true);
}

fn convert_sid_to_string(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = (|| {
        if output == 0 || !memory::writable_pointer(emu, output) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let text = sid_string(&read_sid(emu, address).ok_or(ERROR_INVALID_SID)?);
        let bytes: Vec<u8> = if wide {
            text.encode_utf16()
                .chain(Some(0))
                .flat_map(u16::to_le_bytes)
                .collect()
        } else {
            text.bytes().chain(Some(0)).collect()
        };
        let buffer =
            heap::allocate(emu, 0, bytes.len() as u64, 0).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        if !emu.maps.write_bytes(buffer, &bytes) || !memory::write_pointer(emu, output, buffer) {
            heap::heap_free(emu, 0, buffer);
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    log_red!(
        emu,
        "ConvertSidToStringSid{} sid=0x{:x}",
        if wide { "W" } else { "A" },
        address
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

pub fn OpenProcessToken(emu: &mut Emu) {
    open_process_token(emu, WinApi(Stdcall), false);
}

pub(crate) fn native_open_process_token(emu: &mut Emu, call: WindowsAbi) {
    open_process_token(emu, call, true);
}

fn open_process_token(emu: &mut Emu, call: WindowsAbi, native: bool) {
    let [process_handle, rights, output] = api_try!(emu, call.args(emu));
    let api = if native {
        "ntdll!NtOpenProcessToken"
    } else {
        "advapi32!OpenProcessToken"
    };
    log_red!(
        emu,
        "{api} process={process_handle:#x} desired_access={rights:#x} token_handle={output:#x}"
    );
    let status = if !memory::writable_pointer(emu, output) {
        STATUS_ACCESS_VIOLATION
    } else if !process::query_current_process(emu, process_handle) {
        if emu.handle_management.contains_handle(process_handle) {
            STATUS_OBJECT_TYPE_MISMATCH
        } else {
            STATUS_INVALID_HANDLE
        }
    } else if emu
        .handle_management
        .process_access(process_handle)
        .is_some_and(|a| {
            a & (PROCESS_QUERY_LIMITED_INFORMATION
                | PROCESS_QUERY_INFORMATION
                | GENERIC_READ
                | GENERIC_ALL
                | MAXIMUM_ALLOWED)
                == 0
        })
    {
        STATUS_ACCESS_DENIED
    } else {
        let token = emu.handle_management.open_process_token(rights as u32);
        memory::write_pointer(emu, output, u64::from(token));
        STATUS_SUCCESS
    };
    if native {
        call.set_return(emu, status);
    } else {
        if status != STATUS_SUCCESS {
            emu.set_last_error(match status {
                STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
                STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
                _ => ERROR_INVALID_HANDLE,
            });
        }
        call.set_return(emu, u64::from(status == STATUS_SUCCESS));
    }
    call.cleanup_args(emu, 3);
}

fn token_rights(emu: &Emu, handle: u64) -> Option<u32> {
    let pseudo = if memory::pointer_size(emu) == 8 {
        CURRENT_PROCESS_TOKEN as i64 as u64
    } else {
        u64::from(CURRENT_PROCESS_TOKEN as u32)
    };
    if handle == pseudo {
        Some(TOKEN_QUERY)
    } else {
        emu.handle_management.process_token_rights(handle)
    }
}

pub fn AdjustTokenPrivileges(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [token, disable_all, new_state, length, previous, returned] = api_try!(emu, call.args(emu));
    let error = match token_rights(emu, token) {
        None => ERROR_INVALID_HANDLE,
        Some(rights)
            if rights
                & (TOKEN_ADJUST_PRIVILEGES | GENERIC_WRITE | GENERIC_ALL | MAXIMUM_ALLOWED)
                == 0
                || (previous != 0
                    && rights & (TOKEN_QUERY | GENERIC_READ | GENERIC_ALL | MAXIMUM_ALLOWED)
                        == 0) =>
        {
            ERROR_ACCESS_DENIED
        }
        Some(_) if disable_all as u32 != 0 => {
            policy::unsupported(emu, "advapi32", "AdjustTokenPrivileges", emu.pc());
            return;
        }
        Some(_) => match emu.maps.read_dword(new_state) {
            None if new_state == 0 => ERROR_INVALID_PARAMETER,
            None => ERROR_NOACCESS,
            Some(0) => empty_previous_privileges(emu, previous, length as u32, returned),
            Some(_) => {
                policy::unsupported(emu, "advapi32", "AdjustTokenPrivileges", emu.pc());
                return;
            }
        },
    };
    trace::export(emu, call, "advapi32", "AdjustTokenPrivileges");
    emu.set_last_error(error);
    call.set_return(emu, u64::from(error == ERROR_SUCCESS));
    call.cleanup_args(emu, 6);
}

fn empty_previous_privileges(emu: &mut Emu, previous: u64, length: u32, returned: u64) -> u64 {
    // Windows uses sizeof(TOKEN_PRIVILEGES), including one LUID_AND_ATTRIBUTES slot, for an empty result.
    const TOKEN_PRIVILEGES_SIZE: usize = size_of::<[u32; 4]>();
    if previous == 0 {
        return ERROR_SUCCESS;
    }
    if returned == 0
        || !emu
            .maps
            .validate_write_range(returned, size_of::<u32>() as u64)
    {
        return ERROR_NOACCESS;
    }
    if length < TOKEN_PRIVILEGES_SIZE as u32 {
        emu.maps.write_dword(returned, TOKEN_PRIVILEGES_SIZE as u32);
        return ERROR_INSUFFICIENT_BUFFER;
    }
    if !emu
        .maps
        .validate_write_range(previous, TOKEN_PRIVILEGES_SIZE as u64)
        || !emu.maps.write_bytes(previous, &[0; TOKEN_PRIVILEGES_SIZE])
    {
        return ERROR_NOACCESS;
    }
    emu.maps.write_dword(returned, TOKEN_PRIVILEGES_SIZE as u32);
    ERROR_SUCCESS
}

pub fn GetTokenInformation(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let class = api_try!(emu, call.arg_u32(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let size = api_try!(emu, call.arg_u32(emu, 3));
    let returned = api_try!(emu, call.arg(emu, 4));
    if class != TOKEN_USER {
        policy::unsupported(emu, "advapi32", "GetTokenInformation", emu.pc());
        return;
    }
    let width = memory::pointer_size(emu);
    let sid = process_user_sid();
    let header = 2 * width;
    let required = (header + sid.len() as u64) as u32;
    let rights = token_rights(emu, handle);
    let error = match rights {
        None => ERROR_INVALID_HANDLE,
        Some(mask) if mask & (TOKEN_QUERY | GENERIC_READ | GENERIC_ALL | MAXIMUM_ALLOWED) == 0 => {
            ERROR_ACCESS_DENIED
        }
        Some(_) if !emu.maps.write_dword(returned, required) => ERROR_NOACCESS,
        Some(_) if size < required => ERROR_INSUFFICIENT_BUFFER,
        Some(_) if !emu.maps.validate_write_range(output, u64::from(required)) || output == 0 => {
            ERROR_NOACCESS
        }
        Some(_) => {
            let mut bytes = vec![0; required as usize];
            bytes[..width as usize]
                .copy_from_slice(&(output + header).to_le_bytes()[..width as usize]);
            bytes[header as usize..].copy_from_slice(&sid);
            if emu.maps.write_bytes(output, &bytes) {
                ERROR_SUCCESS
            } else {
                ERROR_NOACCESS
            }
        }
    };
    log_red!(
        emu,
        "GetTokenInformation token=0x{:x} class={}",
        handle,
        class
    );
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(error == ERROR_SUCCESS));
    call.cleanup_args(emu, 5);
}

fn sid_size(count: u8) -> usize {
    SID_HEADER_SIZE + usize::from(count) * size_of::<u32>()
}

pub(crate) fn sid_api(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let (result, slots) = match api {
        "IsValidSid" => (Ok(u64::from(read_sid(emu, first).is_some())), 1),
        "GetLengthSid" => (Ok(read_sid(emu, first).map_or(0, |v| v.len() as u64)), 1),
        "GetSidLengthRequired" => (Ok(sid_size(first as u8) as u64), 1),
        "GetSidIdentifierAuthority" => (Ok(first.wrapping_add(2)), 1),
        "GetSidSubAuthorityCount" => (Ok(first.wrapping_add(1)), 1),
        "GetSidSubAuthority" => {
            let index = api_try!(emu, call.arg_u32(emu, 1));
            (
                Ok(first
                    .wrapping_add(SID_HEADER_SIZE as u64)
                    .wrapping_add(u64::from(index) * size_of::<u32>() as u64)),
                2,
            )
        }
        "EqualSid" | "EqualPrefixSid" => {
            let second = api_try!(emu, call.arg(emu, 1));
            let equal =
                read_sid(emu, first)
                    .zip(read_sid(emu, second))
                    .is_some_and(|(left, right)| {
                        if api == "EqualSid" {
                            left == right
                        } else {
                            left.len() == right.len()
                                && left.len() > SID_HEADER_SIZE
                                && left[..left.len() - 4] == right[..right.len() - 4]
                        }
                    });
            (Ok(u64::from(equal)), 2)
        }
        "CopySid" => {
            let dest = api_try!(emu, call.arg(emu, 1));
            let src = api_try!(emu, call.arg(emu, 2));
            let result = read_sid(emu, src).ok_or(ERROR_INVALID_SID).and_then(|sid| {
                if u64::from(first as u32) < sid.len() as u64 {
                    return Err(ERROR_INSUFFICIENT_BUFFER);
                }
                if dest == 0
                    || !emu.maps.validate_write_range(dest, sid.len() as u64)
                    || !emu.maps.write_bytes(dest, &sid)
                {
                    return Err(ERROR_NOACCESS);
                }
                Ok(1)
            });
            (result, 3)
        }
        "InitializeSid" => {
            let authority = api_try!(emu, call.arg(emu, 1));
            let count = api_try!(emu, call.arg_u8(emu, 2));
            let result = (|| {
                if count > SID_MAX_SUB_AUTHORITIES {
                    return Err(ERROR_INVALID_PARAMETER);
                }
                let auth = memory::read_array::<6>(emu, authority).ok_or(ERROR_NOACCESS)?;
                let mut sid = vec![0; sid_size(count)];
                sid[0] = SID_REVISION;
                sid[1] = count;
                sid[2..SID_HEADER_SIZE].copy_from_slice(&auth);
                if first == 0
                    || !emu.maps.validate_write_range(first, sid.len() as u64)
                    || !emu.maps.write_bytes(first, &sid)
                {
                    return Err(ERROR_NOACCESS);
                }
                Ok(1)
            })();
            (result, 3)
        }
        _ => return,
    };
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    log_red!(emu, "{api} => {value:#x}");
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

pub(crate) fn sid_storage(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    if api == "FreeSid" {
        let sid = api_try!(emu, call.arg(emu, 0));
        let freed = buffers::free(emu, sid, BufferKind::Sid);
        if !freed {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
        }
        log_red!(emu, "FreeSid sid={sid:#x}");
        call.set_return(emu, if freed { 0 } else { sid });
        call.cleanup_args(emu, 1);
        return;
    }
    let authority = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg_u8(emu, 1));
    let mut subs = [0u32; 8];
    for (i, sub) in subs.iter_mut().enumerate().take(usize::from(count)) {
        *sub = api_try!(emu, call.arg_u32(emu, i + 2));
    }
    let output = api_try!(emu, call.arg(emu, 10));
    let result = (|| {
        if usize::from(count) > subs.len() {
            return Err(ERROR_INVALID_SID);
        }
        let authority = memory::read_array::<6>(emu, authority).ok_or(ERROR_NOACCESS)?;
        if !memory::writable_pointer(emu, output) {
            return Err(ERROR_NOACCESS);
        }
        let mut bytes = vec![SID_REVISION, count];
        bytes.extend(authority);
        for sub in &subs[..usize::from(count)] {
            bytes.extend(sub.to_le_bytes());
        }
        let ptr = buffers::allocate(emu, bytes.len() as u64, 4, 0, BufferKind::Sid)
            .ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        if !emu.maps.write_bytes(ptr, &bytes) || !memory::write_pointer(emu, output, ptr) {
            buffers::free(emu, ptr, BufferKind::Sid);
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    log_red!(emu, "{api} subauthorities={count}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 11);
}

fn well_known_sid(kind: u32) -> Option<Vec<u8>> {
    let (authority, subs): (u8, &[u32]) = match kind {
        WIN_NULL_SID => (0, &[0]),
        WIN_WORLD_SID => (1, &[0]),
        WIN_LOCAL_SID => (2, &[0]),
        WIN_LOCAL_SYSTEM_SID => (NT_AUTHORITY, &[18]),
        WIN_LOCAL_SERVICE_SID => (NT_AUTHORITY, &[19]),
        WIN_NETWORK_SERVICE_SID => (NT_AUTHORITY, &[20]),
        WIN_BUILTIN_DOMAIN_SID => (NT_AUTHORITY, &[32]),
        WIN_BUILTIN_ADMINISTRATORS_SID => (NT_AUTHORITY, &[32, 544]),
        WIN_BUILTIN_USERS_SID => (NT_AUTHORITY, &[32, 545]),
        WIN_BUILTIN_GUESTS_SID => (NT_AUTHORITY, &[32, 546]),
        _ => return None,
    };
    let mut bytes = vec![SID_REVISION, subs.len() as u8, 0, 0, 0, 0, 0, authority];
    for sub in subs {
        bytes.extend(sub.to_le_bytes());
    }
    Some(bytes)
}

pub(crate) fn well_known(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let create = api == "CreateWellKnownSid";
    let kind = api_try!(emu, call.arg_u32(emu, usize::from(!create)));
    let Some(sid) = well_known_sid(kind) else {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    };
    let result = if create {
        let dest = api_try!(emu, call.arg(emu, 2));
        let size = api_try!(emu, call.arg(emu, 3));
        (|| {
            let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)?;
            if !emu.maps.validate_write_range(size, 4) {
                return Err(ERROR_NOACCESS);
            }
            emu.maps.write_dword(size, sid.len() as u32);
            if dest == 0 || (capacity as usize) < sid.len() {
                return Err(ERROR_INSUFFICIENT_BUFFER);
            }
            if !emu.maps.validate_write_range(dest, sid.len() as u64)
                || !emu.maps.write_bytes(dest, &sid)
            {
                return Err(ERROR_NOACCESS);
            }
            Ok(1)
        })()
    } else {
        let address = api_try!(emu, call.arg(emu, 0));
        Ok(u64::from(read_sid(emu, address).as_ref() == Some(&sid)))
    };
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    log_red!(emu, "{api} kind={kind}");
    call.set_return(emu, value);
    call.cleanup_args(emu, if create { 4 } else { 2 });
}

pub(crate) fn dispatch_sid(api: &str, library: &str, emu: &mut Emu) -> bool {
    match api {
        "AllocateLocallyUniqueId" => allocate_luid(emu),
        "IsValidSid"
        | "GetLengthSid"
        | "GetSidLengthRequired"
        | "GetSidIdentifierAuthority"
        | "GetSidSubAuthorityCount"
        | "GetSidSubAuthority"
        | "EqualSid"
        | "EqualPrefixSid"
        | "CopySid"
        | "InitializeSid" => sid_api(emu, api),
        "AllocateAndInitializeSid" | "FreeSid" => sid_storage(emu, api),
        "CreateWellKnownSid" | "IsWellKnownSid" => well_known(emu, library, api),
        "AddAccessAllowedAce" | "AddAccessDeniedAce" => add_access_ace(emu, api),
        "GetSecurityDescriptorLength"
        | "InitializeSecurityDescriptor"
        | "GetSecurityDescriptorControl"
        | "GetSecurityDescriptorGroup"
        | "GetSecurityDescriptorOwner"
        | "GetSecurityDescriptorSacl"
        | "GetSecurityDescriptorDacl"
        | "SetSecurityDescriptorGroup"
        | "SetSecurityDescriptorOwner"
        | "SetSecurityDescriptorSacl"
        | "SetSecurityDescriptorDacl" => descriptor(emu, api),
        _ => return false,
    }
    true
}

fn allocate_luid(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let next = emu.last_luid.checked_add(1);
    let ok = next.is_some_and(|id| {
        ptr != 0 && emu.maps.validate_write_range(ptr, 8) && emu.maps.write_qword(ptr, id)
    });
    if ok {
        emu.last_luid += 1;
    } else {
        emu.set_last_error(if next.is_none() {
            ERROR_NOT_ENOUGH_MEMORY
        } else {
            ERROR_NOACCESS
        });
    }
    log_red!(emu, "AllocateLocallyUniqueId output={ptr:#x}");
    call.set_return(emu, u64::from(ok));
    call.cleanup_args(emu, 1);
}

pub fn AllocateLocallyUniqueId(emu: &mut Emu) {
    allocate_luid(emu);
}

pub fn InitializeSid(emu: &mut Emu) {
    sid_api(emu, "InitializeSid");
}

pub fn GetSecurityDescriptorDacl(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorDacl");
}

pub fn IsValidSid(emu: &mut Emu) {
    sid_api(emu, "IsValidSid");
}

pub fn GetLengthSid(emu: &mut Emu) {
    sid_api(emu, "GetLengthSid");
}

pub fn GetSidLengthRequired(emu: &mut Emu) {
    sid_api(emu, "GetSidLengthRequired");
}

pub fn GetSidIdentifierAuthority(emu: &mut Emu) {
    sid_api(emu, "GetSidIdentifierAuthority");
}

pub fn GetSidSubAuthorityCount(emu: &mut Emu) {
    sid_api(emu, "GetSidSubAuthorityCount");
}

pub fn GetSidSubAuthority(emu: &mut Emu) {
    sid_api(emu, "GetSidSubAuthority");
}

pub fn EqualSid(emu: &mut Emu) {
    sid_api(emu, "EqualSid");
}

pub fn EqualPrefixSid(emu: &mut Emu) {
    sid_api(emu, "EqualPrefixSid");
}

pub fn CopySid(emu: &mut Emu) {
    sid_api(emu, "CopySid");
}

fn parse_sid(text: &str) -> Option<Vec<u8>> {
    let mut pieces = text.split('-');
    if !pieces.next()?.eq_ignore_ascii_case("S") || pieces.next()? != "1" {
        return None;
    }
    let authority = pieces.next()?;
    let authority = if let Some(hex) = authority
        .strip_prefix("0x")
        .or_else(|| authority.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()?
    } else {
        authority.parse::<u64>().ok()?
    };
    let authority = authority.to_be_bytes();
    if authority[..2] != [0, 0] {
        return None;
    }
    let mut bytes = vec![SID_REVISION, 0];
    bytes.extend_from_slice(&authority[2..]);
    for part in pieces {
        if bytes[1] == SID_MAX_SUB_AUTHORITIES {
            return None;
        }
        let sub = part.parse::<u32>().ok()?;
        bytes.extend_from_slice(&sub.to_le_bytes());
        bytes[1] += 1;
    }
    Some(bytes)
}

fn convert_string_to_sid(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [input, output] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    let result = (|| {
        if input == 0 || output == 0 || !memory::writable_pointer(emu, output) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let text = String::from_utf16_lossy(&encoding.read(emu, input)?);
        let sid = match text.as_str() {
            "WD" => well_known_sid(WIN_WORLD_SID),
            "SY" => well_known_sid(WIN_LOCAL_SYSTEM_SID),
            "LS" => well_known_sid(WIN_LOCAL_SERVICE_SID),
            "NS" => well_known_sid(WIN_NETWORK_SERVICE_SID),
            "BA" => well_known_sid(WIN_BUILTIN_ADMINISTRATORS_SID),
            "BU" => well_known_sid(WIN_BUILTIN_USERS_SID),
            "BG" => well_known_sid(WIN_BUILTIN_GUESTS_SID),
            _ => parse_sid(&text),
        }
        .ok_or(ERROR_INVALID_SID)?;
        let buffer = heap::allocate(emu, 0, sid.len() as u64, 0).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        if !emu.maps.write_bytes(buffer, &sid) || !memory::write_pointer(emu, output, buffer) {
            heap::heap_free(emu, 0, buffer);
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}
pub fn ConvertStringSidToSidA(emu: &mut Emu) {
    convert_string_to_sid(emu, "ConvertStringSidToSidA", Encoding::Ansi);
}
pub fn ConvertStringSidToSidW(emu: &mut Emu) {
    convert_string_to_sid(emu, "ConvertStringSidToSidW", Encoding::Wide);
}
pub fn AllocateAndInitializeSid(emu: &mut Emu) {
    sid_storage(emu, "AllocateAndInitializeSid");
}
pub fn CreateWellKnownSid(emu: &mut Emu) {
    well_known(emu, "advapi32", "CreateWellKnownSid");
}
pub fn InitializeSecurityDescriptor(emu: &mut Emu) {
    descriptor(emu, "InitializeSecurityDescriptor");
}

pub fn GetSecurityDescriptorControl(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorControl");
}

pub fn GetSecurityDescriptorGroup(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorGroup");
}

pub fn GetSecurityDescriptorLength(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorLength");
}

pub fn GetSecurityDescriptorOwner(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorOwner");
}

pub fn GetSecurityDescriptorSacl(emu: &mut Emu) {
    descriptor(emu, "GetSecurityDescriptorSacl");
}

pub fn SetSecurityDescriptorGroup(emu: &mut Emu) {
    descriptor(emu, "SetSecurityDescriptorGroup");
}

pub fn SetSecurityDescriptorOwner(emu: &mut Emu) {
    descriptor(emu, "SetSecurityDescriptorOwner");
}

pub fn SetSecurityDescriptorSacl(emu: &mut Emu) {
    descriptor(emu, "SetSecurityDescriptorSacl");
}

pub fn SetSecurityDescriptorDacl(emu: &mut Emu) {
    descriptor(emu, "SetSecurityDescriptorDacl");
}

pub fn AddAccessAllowedAce(emu: &mut Emu) {
    add_access_ace(emu, "AddAccessAllowedAce");
}

pub fn AddAccessDeniedAce(emu: &mut Emu) {
    add_access_ace(emu, "AddAccessDeniedAce");
}

pub fn IsWellKnownSid(emu: &mut Emu) {
    well_known(emu, "advapi32", "IsWellKnownSid");
}

pub fn FreeSid(emu: &mut Emu) {
    sid_storage(emu, "FreeSid");
}
