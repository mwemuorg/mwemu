use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::memory::{pointer_size, read_array, write_pointer};
use crate::winapi::common::security::read_sid;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::security::*;

const ACL_HEADER: usize = 8;
const DWORD_MASK: u32 = size_of::<u32>() as u32 - 1;
const MAX_ACL_SIZE: u32 = (u16::MAX as u32) & !DWORD_MASK;

enum Error {
    Status(u64),
    Memory(u64, usize, MemoryAccess),
}
type AclResult<T> = Result<T, Error>;

struct Acl {
    base: u64,
    revision: u8,
    size: u32,
    count: u16,
}

impl Acl {
    fn read(emu: &Emu, base: u64) -> AclResult<Self> {
        let [revision, _, size_lo, size_hi, count_lo, count_hi, _, _] =
            read_array(emu, base).ok_or(Error::Memory(base, ACL_HEADER, MemoryAccess::Read))?;
        if !(ACL_REVISION..=ACL_REVISION_DS).contains(&u32::from(revision)) {
            return Err(Error::Status(STATUS_INVALID_PARAMETER));
        }
        if u16::from_le_bytes([size_lo, size_hi]) < ACL_HEADER as u16 {
            return Err(Error::Status(STATUS_INVALID_PARAMETER));
        }
        Ok(Self {
            base,
            revision,
            size: u16::from_le_bytes([size_lo, size_hi]) as u32,
            count: u16::from_le_bytes([count_lo, count_hi]),
        })
    }

    fn offset_after(&self, emu: &Emu, count: u16) -> AclResult<u32> {
        let mut offset = ACL_HEADER as u32;
        for _ in 0..count {
            if offset >= self.size {
                return Err(Error::Status(STATUS_INVALID_PARAMETER));
            }
            let size_at = self
                .base
                .checked_add(u64::from(offset) + 2)
                .ok_or(Error::Status(STATUS_INVALID_PARAMETER))?;
            let bytes = read_array(emu, size_at).ok_or(Error::Memory(
                size_at,
                size_of::<u16>(),
                MemoryAccess::Read,
            ))?;
            let size = u32::from(u16::from_le_bytes(bytes));
            if size < 4 || size % 4 != 0 || size > self.size - offset {
                return Err(Error::Status(STATUS_INVALID_PARAMETER));
            }
            offset += size;
        }
        Ok(offset)
    }
}

fn write(emu: &mut Emu, address: u64, bytes: &[u8]) -> AclResult<()> {
    if emu.maps.validate_write_range(address, bytes.len() as u64)
        && emu.maps.write_bytes(address, bytes)
    {
        Ok(())
    } else {
        Err(Error::Memory(address, bytes.len(), MemoryAccess::Write))
    }
}

fn create(emu: &mut Emu, base: u64, size: u32, revision: u32) -> AclResult<()> {
    if size < ACL_HEADER as u32 {
        return Err(Error::Status(STATUS_BUFFER_TOO_SMALL));
    }
    if size > MAX_ACL_SIZE || !(ACL_REVISION..=ACL_REVISION_DS).contains(&revision) {
        return Err(Error::Status(STATUS_INVALID_PARAMETER));
    }
    let [lo, hi] = ((size & !DWORD_MASK) as u16).to_le_bytes();
    write(emu, base, &[revision as u8, 0, lo, hi, 0, 0, 0, 0])
}

fn query(emu: &mut Emu, base: u64, output: u64, capacity: u32, class: u32) -> AclResult<()> {
    let acl = Acl::read(emu, base)?;
    match class {
        ACL_REVISION_INFORMATION if capacity >= size_of::<u32>() as u32 => {
            write(emu, output, &u32::from(acl.revision).to_le_bytes())
        }
        ACL_SIZE_INFORMATION if capacity >= size_of::<[u32; 3]>() as u32 => {
            let used = acl.offset_after(emu, acl.count)?.min(acl.size);
            let mut bytes = [0; size_of::<[u32; 3]>()];
            for (chunk, field) in bytes.as_chunks_mut::<4>().0.iter_mut().zip([
                u32::from(acl.count),
                used,
                acl.size - used,
            ]) {
                *chunk = field.to_le_bytes();
            }
            write(emu, output, &bytes)
        }
        ACL_REVISION_INFORMATION | ACL_SIZE_INFORMATION => {
            Err(Error::Status(STATUS_BUFFER_TOO_SMALL))
        }
        _ => Err(Error::Status(STATUS_INVALID_INFO_CLASS)),
    }
}

fn get_ace(emu: &mut Emu, base: u64, index: u32, output: u64) -> AclResult<()> {
    let acl = Acl::read(emu, base)?;
    if index >= u32::from(acl.count) {
        return Err(Error::Status(STATUS_INVALID_PARAMETER));
    }
    let offset = acl.offset_after(emu, index as u16)?;
    let address = base
        .checked_add(u64::from(offset))
        .ok_or(Error::Status(STATUS_INVALID_PARAMETER))?;
    if offset >= acl.size {
        return Err(Error::Status(STATUS_INVALID_PARAMETER));
    }
    if !write_pointer(emu, output, address) {
        return Err(Error::Memory(
            output,
            pointer_size(emu) as usize,
            MemoryAccess::Write,
        ));
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum ReturnType {
    Win32,
    NtStatus,
}
fn return_result(emu: &mut Emu, words: usize, result: AclResult<()>, kind: ReturnType) {
    let call = WinApi(Stdcall);
    let status = match result {
        Ok(()) => STATUS_SUCCESS,
        Err(Error::Status(code)) => code,
        Err(Error::Memory(address, size, access)) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                size,
                access,
            );
            return;
        }
    };
    let value = match kind {
        ReturnType::NtStatus => status,
        ReturnType::Win32 if status == STATUS_SUCCESS => 1,
        ReturnType::Win32 => {
            emu.set_last_error(if status == STATUS_BUFFER_TOO_SMALL {
                ERROR_INSUFFICIENT_BUFFER
            } else {
                ERROR_INVALID_PARAMETER
            });
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, words);
}
fn create_api(emu: &mut Emu, api: &str, kind: ReturnType) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let length = api_try!(emu, call.arg_u32(emu, 1));
    let revision = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "{api} acl={base:#x} length={length} revision={revision}"
    );
    let result = create(emu, base, length, revision);
    return_result(emu, 3, result, kind);
}
fn query_api(emu: &mut Emu, api: &str, kind: ReturnType) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg_u32(emu, 2));
    let class = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(emu, "{api} acl={base:#x} length={length} class={class}");
    let result = query(emu, base, out, length, class);
    return_result(emu, 4, result, kind);
}
fn ace_api(emu: &mut Emu, api: &str, kind: ReturnType) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "{api} acl={base:#x} index={index}");
    let result = get_ace(emu, base, index, out);
    return_result(emu, 3, result, kind);
}
pub fn InitializeAcl(emu: &mut Emu) {
    create_api(emu, "InitializeAcl", ReturnType::Win32);
}
pub fn RtlCreateAcl(emu: &mut Emu) {
    create_api(emu, "RtlCreateAcl", ReturnType::NtStatus);
}
pub fn GetAclInformation(emu: &mut Emu) {
    query_api(emu, "GetAclInformation", ReturnType::Win32);
}
pub fn RtlQueryInformationAcl(emu: &mut Emu) {
    query_api(emu, "RtlQueryInformationAcl", ReturnType::NtStatus);
}
pub fn FindFirstFreeAce(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [acl_ptr, ace_ptr] = api_try!(emu, call.args(emu));
    let position = Acl::read(emu, acl_ptr).ok().and_then(|acl| {
        let used = acl.offset_after(emu, acl.count).ok()?;
        if used > acl.size {
            return None;
        }
        acl_ptr.checked_add(used.into())
    });
    let written = write_pointer(emu, ace_ptr, position.unwrap_or(0));
    log_red!(
        emu,
        "advapi32!FindFirstFreeAce acl={acl_ptr:#x} ace_ptr={ace_ptr:#x}"
    );
    if !written {
        emu.set_last_error(ERROR_NOACCESS);
    } else if position.is_none() {
        emu.set_last_error(ERROR_INVALID_ACL);
    }
    call.set_return(emu, u64::from(written && position.is_some()));
    call.cleanup_args(emu, 2);
}

pub fn GetAce(emu: &mut Emu) {
    ace_api(emu, "GetAce", ReturnType::Win32);
}
pub fn RtlGetAce(emu: &mut Emu) {
    ace_api(emu, "RtlGetAce", ReturnType::NtStatus);
}

pub(crate) fn add_access_ace(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let revision = api_try!(emu, call.arg_u32(emu, 1));
    let extended = api.ends_with("Ex");
    let flags = if extended {
        api_try!(emu, call.arg_u32(emu, 2))
    } else {
        0
    };
    let mask = api_try!(emu, call.arg_u32(emu, 2 + usize::from(extended)));
    let sid_ptr = api_try!(emu, call.arg(emu, 3 + usize::from(extended)));
    let result = (|| {
        let sid = read_sid(emu, sid_ptr).ok_or(ERROR_INVALID_SID)?;
        let acl = Acl::read(emu, ptr).map_err(|_| ERROR_INVALID_ACL)?;
        if !(ACL_REVISION..=ACL_REVISION_DS).contains(&revision) {
            return Err(ERROR_REVISION_MISMATCH);
        }
        let offset = acl
            .offset_after(emu, acl.count)
            .map_err(|_| ERROR_INVALID_ACL)?;
        let ace_size = ACL_HEADER + sid.len();
        if u64::from(offset) + ace_size as u64 > u64::from(acl.size) {
            return Err(ERROR_ALLOTTED_SPACE_EXCEEDED);
        }
        let start = ptr.checked_add(u64::from(offset)).ok_or(ERROR_NOACCESS)?;
        let count = acl.count.checked_add(1).ok_or(ERROR_INVALID_ACL)?;
        let ty = if api.starts_with("AddAccessDeniedAce") {
            ACCESS_DENIED_ACE_TYPE
        } else {
            ACCESS_ALLOWED_ACE_TYPE
        };
        let mut bytes = vec![
            ty,
            u8::try_from(flags).map_err(|_| ERROR_INVALID_PARAMETER)?,
        ];
        bytes.extend((ace_size as u16).to_le_bytes());
        bytes.extend(mask.to_le_bytes());
        bytes.extend(sid);
        if !emu.maps.validate_write_range(ptr, ACL_HEADER as u64)
            || !emu.maps.validate_write_range(start, ace_size as u64)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_bytes(start, &bytes);
        emu.maps.write_word(ptr + 4, count);
        emu.maps.write_byte(ptr, acl.revision.max(revision as u8));
        Ok(())
    })();
    log_red!(emu, "{api} acl={ptr:#x} mask={mask:#x}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4 + usize::from(extended));
}

pub fn AddAccessAllowedAceEx(emu: &mut Emu) {
    add_access_ace(emu, "AddAccessAllowedAceEx");
}
pub fn AddAccessDeniedAceEx(emu: &mut Emu) {
    add_access_ace(emu, "AddAccessDeniedAceEx");
}
pub fn IsValidAcl(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let valid = Acl::read(emu, base).is_ok_and(|acl| acl.offset_after(emu, acl.count).is_ok());
    log_red!(emu, "advapi32!IsValidAcl acl={base:#x}");
    call.set_return(emu, u64::from(valid));
    call.cleanup_args(emu, 1);
}
pub fn DeleteAce(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let base = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "advapi32!DeleteAce acl={base:#x} index={index}");
    let result = (|| {
        let acl = Acl::read(emu, base).map_err(|_| ERROR_INVALID_ACL)?;
        if index >= u32::from(acl.count) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let start = acl
            .offset_after(emu, index as u16)
            .map_err(|_| ERROR_INVALID_ACL)?;
        let next = acl
            .offset_after(emu, index as u16 + 1)
            .map_err(|_| ERROR_INVALID_ACL)?;
        let used = acl
            .offset_after(emu, acl.count)
            .map_err(|_| ERROR_INVALID_ACL)?;
        let from = base.checked_add(next.into()).ok_or(ERROR_NOACCESS)?;
        let to = base.checked_add(start.into()).ok_or(ERROR_NOACCESS)?;
        let length = used - next;
        if !emu.maps.validate_write_range(base, ACL_HEADER as u64)
            || (length > 0
                && (!emu.maps.validate_write_range(to, u64::from(length))
                    || !emu.maps.copy_range(to, from, length as usize)))
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_word(base + 4, acl.count - 1);
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}
