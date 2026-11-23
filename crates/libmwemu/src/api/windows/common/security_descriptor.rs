use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{
    pointer_size, read_array, read_pointer, writable_pointer, write_pointer,
};
use crate::winapi::common::security::{SID_HEADER_SIZE, read_sid};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::security::*;

fn address(base: u64, offset: u64) -> Result<u64, u64> {
    base.checked_add(offset).ok_or(ERROR_NOACCESS)
}
fn writable(emu: &Emu, ptr: u64, size: u64) -> bool {
    ptr != 0 && emu.maps.validate_write_range(ptr, size)
}
fn control(emu: &Emu, sd: u64) -> Result<u16, u64> {
    let head = read_array::<4>(emu, sd).ok_or(ERROR_NOACCESS)?;
    if head[0] != SECURITY_DESCRIPTOR_REVISION {
        return Err(ERROR_UNKNOWN_REVISION);
    }
    Ok(u16::from_le_bytes([head[2], head[3]]))
}
#[derive(Clone, Copy)]
enum Field {
    Owner,
    Group,
    Sacl,
    Dacl,
}
impl Field {
    fn index(self) -> u64 {
        match self {
            Self::Owner => 0,
            Self::Group => 1,
            Self::Sacl => 2,
            Self::Dacl => 3,
        }
    }
    fn defaulted(self) -> u16 {
        match self {
            Self::Owner => SE_OWNER_DEFAULTED,
            Self::Group => SE_GROUP_DEFAULTED,
            Self::Sacl => SE_SACL_DEFAULTED,
            Self::Dacl => SE_DACL_DEFAULTED,
        }
    }
    fn present(self) -> u16 {
        match self {
            Self::Sacl => SE_SACL_PRESENT,
            Self::Dacl => SE_DACL_PRESENT,
            _ => 0,
        }
    }
    fn slot(self, emu: &Emu, sd: u64, relative: bool) -> Result<u64, u64> {
        let width = if relative { 4 } else { pointer_size(emu) };
        address(sd, (self.index() + 1) * width)
    }
    fn read(self, emu: &Emu, sd: u64, flags: u16) -> Result<u64, u64> {
        let relative = flags & SE_SELF_RELATIVE != 0;
        let at = self.slot(emu, sd, relative)?;
        if relative {
            match emu.maps.read_dword(at).ok_or(ERROR_NOACCESS)? {
                0 => Ok(0),
                rva => address(sd, u64::from(rva)),
            }
        } else {
            read_pointer(emu, at).ok_or(ERROR_NOACCESS)
        }
    }
}
/// Recognize the one-ACE descriptor built by RtlCheckTokenMembershipEx.
pub(super) fn membership_sid(emu: &Emu, sd: u64, desired: u32) -> Result<Option<Vec<u8>>, u64> {
    const ACL_HEADER_SIZE: u64 = 8;
    const ACE_SID_OFFSET: u64 = 8;
    let flags = control(emu, sd).map_err(|_| STATUS_ACCESS_VIOLATION)?;
    if flags & !SE_SELF_RELATIVE != SE_DACL_PRESENT {
        return Ok(None);
    }
    let field = |f: Field| f.read(emu, sd, flags).map_err(|_| STATUS_ACCESS_VIOLATION);
    let acl = field(Field::Dacl)?;
    if acl == 0 {
        return Ok(None);
    }
    let header: [u8; 8] = read_array(emu, acl).ok_or(STATUS_ACCESS_VIOLATION)?;
    let size = u16::from_le_bytes([header[2], header[3]]) as u64;
    if header[0] as u32 != ACL_REVISION
        || u16::from_le_bytes([header[4], header[5]]) != 1
        || size < ACL_HEADER_SIZE + ACE_SID_OFFSET
    {
        return Ok(None);
    }
    let ace = acl
        .checked_add(ACL_HEADER_SIZE)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    let header: [u8; 8] = read_array(emu, ace).ok_or(STATUS_ACCESS_VIOLATION)?;
    let ace_size = u16::from_le_bytes([header[2], header[3]]) as u64;
    if header[0] != ACCESS_ALLOWED_ACE_TYPE
        || header[1] != 0
        || u32::from_le_bytes([header[4], header[5], header[6], header[7]]) != desired
        || ace_size < ACE_SID_OFFSET + SID_HEADER_SIZE as u64
        || ace_size > size - ACL_HEADER_SIZE
    {
        return Ok(None);
    }
    let address = ace
        .checked_add(ACE_SID_OFFSET)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    let count = emu
        .maps
        .read_byte(address.checked_add(1).ok_or(STATUS_ACCESS_VIOLATION)?)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    if ace_size != ACE_SID_OFFSET + SID_HEADER_SIZE as u64 + count as u64 * size_of::<u32>() as u64
    {
        return Ok(None);
    }
    let sid = read_sid(emu, address).ok_or(STATUS_ACCESS_VIOLATION)?;
    for slot in [Field::Owner, Field::Group] {
        if read_sid(emu, field(slot)?).as_deref() != Some(&sid) {
            return Ok(None);
        }
    }
    Ok(Some(sid))
}

fn component(
    emu: &mut Emu,
    sd: u64,
    field: Field,
    set: bool,
    present: u64,
    ptr: u64,
    defaulted: u64,
) -> Result<(), u64> {
    let flags = control(emu, sd)?;
    let presence = field.present();
    let required = presence == 0
        || (if set {
            present != 0
        } else {
            flags & presence != 0
        });
    if set {
        if flags & SE_SELF_RELATIVE != 0 {
            return Err(ERROR_BAD_DESCRIPTOR_FORMAT);
        }
        let at = field.slot(emu, sd, false)?;
        if !writable(emu, address(sd, 2)?, 2) || (required && !writable_pointer(emu, at)) {
            return Err(ERROR_NOACCESS);
        }
        let mut new = flags & !(presence | field.defaulted());
        if required {
            write_pointer(emu, at, ptr);
            new |= presence;
            if defaulted != 0 {
                new |= field.defaulted();
            }
        }
        emu.maps.write_word(sd + 2, new);
        return Ok(());
    }
    if presence != 0 && !writable(emu, present, 4) {
        return Err(ERROR_NOACCESS);
    }
    if required {
        let value = field.read(emu, sd, flags)?;
        let has_default = presence != 0 || value != 0;
        if !writable_pointer(emu, ptr) || (has_default && !writable(emu, defaulted, 4)) {
            return Err(ERROR_NOACCESS);
        }
        write_pointer(emu, ptr, value);
        if has_default {
            emu.maps
                .write_dword(defaulted, u32::from(flags & field.defaulted() != 0));
        }
    }
    if presence != 0 {
        emu.maps.write_dword(present, u32::from(required));
    }
    Ok(())
}

pub(crate) fn descriptor(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let sd = api_try!(emu, call.arg(emu, 0));
    if api == "GetSecurityDescriptorLength" {
        let size = control(emu, sd).and_then(|flags| {
            let mut length = if flags & SE_SELF_RELATIVE != 0 {
                20
            } else {
                5 * pointer_size(emu)
            };
            for field in [Field::Owner, Field::Group, Field::Sacl, Field::Dacl] {
                if field.present() != 0 && flags & field.present() == 0 {
                    continue;
                }
                let address = field.read(emu, sd, flags)?;
                if address == 0 {
                    continue;
                }
                let size = if field.present() == 0 {
                    read_sid(emu, address).ok_or(ERROR_INVALID_SID)?.len() as u64
                } else {
                    u64::from(
                        emu.maps
                            .read_word(address.checked_add(2).ok_or(ERROR_NOACCESS)?)
                            .ok_or(ERROR_NOACCESS)?,
                    )
                };
                length += (size + 3) & !3;
            }
            Ok(length)
        });
        log_red!(emu, "{api} sd={sd:#x}");
        call.set_return(emu, size.unwrap_or_default());
        call.cleanup_args(emu, 1);
        return;
    }
    let (result, slots) = match api {
        "InitializeSecurityDescriptor" => {
            let rev = api_try!(emu, call.arg_u32(emu, 1));
            let len = 5 * pointer_size(emu);
            let res = if rev != u32::from(SECURITY_DESCRIPTOR_REVISION) {
                Err(ERROR_UNKNOWN_REVISION)
            } else if !writable(emu, sd, len) {
                Err(ERROR_NOACCESS)
            } else {
                let mut bytes = vec![0; len as usize];
                bytes[0] = SECURITY_DESCRIPTOR_REVISION;
                emu.maps.write_bytes(sd, &bytes);
                Ok(())
            };
            (res, 2)
        }
        "GetSecurityDescriptorControl" => {
            let out = api_try!(emu, call.arg(emu, 1));
            let rev = api_try!(emu, call.arg(emu, 2));
            (
                control(emu, sd).and_then(|v| {
                    if !writable(emu, out, 2) || !writable(emu, rev, 4) {
                        return Err(ERROR_NOACCESS);
                    }
                    emu.maps.write_word(out, v);
                    emu.maps
                        .write_dword(rev, u32::from(SECURITY_DESCRIPTOR_REVISION));
                    Ok(())
                }),
                3,
            )
        }
        _ => {
            let field = if api.ends_with("Owner") {
                Field::Owner
            } else if api.ends_with("Group") {
                Field::Group
            } else if api.ends_with("Dacl") {
                Field::Dacl
            } else {
                Field::Sacl
            };
            let acl = field.present() != 0;
            let idx = 1 + usize::from(acl);
            let present = if acl {
                api_try!(emu, call.arg(emu, 1))
            } else {
                0
            };
            let ptr = api_try!(emu, call.arg(emu, idx));
            let defaulted = api_try!(emu, call.arg(emu, idx + 1));
            (
                component(
                    emu,
                    sd,
                    field,
                    api.starts_with("Set"),
                    present,
                    ptr,
                    defaulted,
                ),
                idx + 2,
            )
        }
    };
    log_red!(emu, "{api} descriptor={sd:#x}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, slots);
}
