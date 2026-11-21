use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::memory::read_guest_bytes;
use crate::api::windows::common::ntdll::read_unicode_string;
use crate::api::windows::common::sections;
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::PAGE_READWRITE;

const PORT_MESSAGE_SIZE: usize = 0x28;
const LPC_REPLY: u16 = 2;
const ALPC_MSGFLG_SYNC_REQUEST: u32 = 0x0002_0000;
const CSR_CAPTURE_BUFFER: usize = PORT_MESSAGE_SIZE;
const CSR_API_NUMBER: usize = CSR_CAPTURE_BUFFER + size_of::<u64>();
const CSR_RETURN_VALUE: usize = CSR_API_NUMBER + size_of::<u32>();
const CSR_CLIENT_CONNECT: u32 = 0;
const CSR_CONNECT_REQUEST: usize = 0x40;
const CSR_CONNECT_SIZE: usize = CSR_CONNECT_REQUEST + 24;

fn csr_port(name: &str) -> bool {
    if name.eq_ignore_ascii_case(r"\Windows\ApiPort") {
        return true;
    }
    let mut parts = name.split('\\');
    matches!(
        (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next()),
        (Some(""), Some(sessions), Some(session), Some(windows), Some(port), None)
        if sessions.eq_ignore_ascii_case("Sessions")
            && session.parse::<u32>().is_ok() && session.bytes().all(|b| b.is_ascii_digit())
            && windows.eq_ignore_ascii_case("Windows") && port.eq_ignore_ascii_case("ApiPort")
    )
}

pub(super) fn nt_alpc_connect_port(emu: &mut Emu) {
    let call = WinSyscall;
    let [out, name] = api_try!(emu, call.args(emu));
    let name = match read_unicode_string(emu, name) {
        Ok(name) => name,
        Err(status) => return call.set_return(emu, status),
    };
    if !csr_port(&name) {
        return policy::unsupported_with_abi(emu, "ntdll", "NtAlpcConnectPort", emu.pc(), call);
    }
    let [
        _,
        _,
        object,
        attributes,
        flags,
        sid,
        message,
        length,
        sent,
        received,
        timeout,
    ] = api_try!(emu, call.args(emu));
    if [object, attributes, flags, sid, sent, received, timeout]
        .iter()
        .any(|value| *value != 0)
    {
        return policy::unsupported_with_abi(emu, "ntdll", "NtAlpcConnectPort", emu.pc(), call);
    }
    if !emu.maps.validate_write_range(out, 8) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    if message != 0 || length != 0 {
        if message == 0 || length == 0 {
            return call.set_return(emu, STATUS_INVALID_PARAMETER);
        }
        let Some(capacity) = emu.maps.read_qword(length) else {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        };
        if capacity < PORT_MESSAGE_SIZE as u64 {
            return call.set_return(emu, STATUS_BUFFER_TOO_SMALL);
        }
        if !emu
            .maps
            .can_access_range(message, PORT_MESSAGE_SIZE, Permission::READ)
            || !emu.maps.validate_write_range(length, 8)
        {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        }
        if emu.maps.read_word(message) != Some(0)
            || emu.maps.read_word(message + 2) != Some(PORT_MESSAGE_SIZE as u16)
        {
            return policy::unsupported_with_abi(emu, "ntdll", "NtAlpcConnectPort", emu.pc(), call);
        }
        emu.maps.write_qword(length, PORT_MESSAGE_SIZE as u64);
    }
    let handle = emu
        .handle_management
        .insert_native_object(NativeObject::CsrPort);
    emu.maps.write_qword(out, handle);
    log_orange!(emu, "NtAlpcConnectPort port={name:?} handle={handle:#x}");
    call.set_return(emu, STATUS_SUCCESS);
}

pub(super) fn nt_alpc_send_wait_receive_port(emu: &mut Emu) {
    let call = WinSyscall;
    let [
        handle,
        flags,
        send,
        out_attributes,
        receive,
        length,
        in_attributes,
        timeout,
    ] = api_try!(emu, call.args(emu));
    if emu.handle_management.native_object(handle) != Some(NativeObject::CsrPort)
        || flags as u32 & !ALPC_MSGFLG_SYNC_REQUEST != 0
        || [out_attributes, in_attributes, timeout]
            .iter()
            .any(|value| *value != 0)
    {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtAlpcSendWaitReceivePort",
            emu.pc(),
            call,
        );
    }
    if !emu
        .maps
        .can_access_range(send, PORT_MESSAGE_SIZE, Permission::READ)
    {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let Some(data_len) = emu.maps.read_word(send) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    let total = PORT_MESSAGE_SIZE + usize::from(data_len);
    if total < CSR_RETURN_VALUE + size_of::<u32>()
        || emu.maps.read_word(send + 2).map(usize::from) != Some(total)
    {
        return call.set_return(emu, STATUS_INVALID_PARAMETER);
    }
    if !emu.maps.can_access_range(send, total, Permission::READ) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    if emu.maps.read_qword(send + CSR_CAPTURE_BUFFER as u64) != Some(0)
        || emu.maps.read_dword(send + CSR_API_NUMBER as u64) != Some(CSR_CLIENT_CONNECT)
        || emu.maps.read_word(send + 6) != Some(0)
        || total != CSR_CONNECT_SIZE
        || emu.maps.read_dword(send + CSR_CONNECT_REQUEST as u64) != Some(0)
        || emu.maps.read_qword(send + CSR_CONNECT_REQUEST as u64 + 8) != Some(0)
        || emu.maps.read_dword(send + CSR_CONNECT_REQUEST as u64 + 16) != Some(0)
    {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtAlpcSendWaitReceivePort",
            emu.pc(),
            call,
        );
    }
    let Some(capacity) = emu.maps.read_qword(length) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    if !emu.maps.validate_write_range(length, 8) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    if capacity < total as u64 {
        emu.maps.write_qword(length, total as u64);
        return call.set_return(emu, STATUS_BUFFER_TOO_SMALL);
    }
    if !emu.maps.validate_write_range(receive, total as u64) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let Some(mut reply) = read_guest_bytes(emu, send, total) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    reply[4..6].copy_from_slice(&LPC_REPLY.to_le_bytes());
    reply[CSR_RETURN_VALUE..CSR_RETURN_VALUE + 4]
        .copy_from_slice(&(STATUS_SUCCESS as u32).to_le_bytes());
    emu.maps.write_bytes(receive, &reply);
    emu.maps.write_qword(length, total as u64);
    call.set_return(emu, STATUS_SUCCESS);
}

pub(super) fn nt_connect_port(emu: &mut Emu) {
    const SECURITY_QOS_SIZE: usize = 12;
    const PORT_VIEW_SIZE: usize = 48;
    const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 512;
    let call = WinSyscall;
    let [out, name, quality, client, server, maximum, data, length] = api_try!(emu, call.args(emu));
    let name = match read_unicode_string(emu, name) {
        Ok(name) => name,
        Err(status) => return call.set_return(emu, status),
    };
    if !csr_port(&name) || server != 0 || data != 0 {
        return policy::unsupported_with_abi(emu, "ntdll", "NtConnectPort", emu.pc(), call);
    }
    if !emu
        .maps
        .can_access_range(quality, SECURITY_QOS_SIZE, Permission::READ)
        || !emu.maps.validate_write_range(out, 8)
        || (maximum != 0 && !emu.maps.validate_write_range(maximum, 4))
        || (length != 0 && !emu.maps.validate_write_range(length, 4))
    {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    if emu.maps.read_dword(quality) != Some(SECURITY_QOS_SIZE as u32)
        || !matches!(emu.maps.read_dword(quality + 4), Some(0..=3))
        || !matches!(emu.maps.read_byte(quality + 8), Some(0..=1))
        || !matches!(emu.maps.read_byte(quality + 9), Some(0..=1))
        || (length != 0 && emu.maps.read_dword(length) != Some(0))
    {
        return call.set_return(emu, STATUS_INVALID_PARAMETER);
    }
    if client != 0 {
        if !emu
            .maps
            .can_access_range(client, PORT_VIEW_SIZE, Permission::READ)
            || !emu.maps.validate_write_range(client, PORT_VIEW_SIZE as u64)
        {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        }
        if emu.maps.read_dword(client) != Some(PORT_VIEW_SIZE as u32) {
            return call.set_return(emu, STATUS_INVALID_PARAMETER);
        }
        let (Some(section), Some(offset), Some(size)) = (
            emu.maps.read_qword(client + 8),
            emu.maps.read_dword(client + 16),
            emu.maps.read_qword(client + 24),
        ) else {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        };
        match sections::map(emu, section, 0, u64::from(offset), size, PAGE_READWRITE) {
            Ok((base, mapped)) => {
                emu.maps.write_qword(client + 24, mapped);
                emu.maps.write_qword(client + 32, base);
                emu.maps.write_qword(client + 40, base);
            }
            Err(status) => return call.set_return(emu, status),
        }
    }
    let handle = emu
        .handle_management
        .insert_native_object(NativeObject::CsrPort);
    emu.maps.write_qword(out, handle);
    if maximum != 0 {
        emu.maps.write_dword(maximum, PORT_MAXIMUM_MESSAGE_LENGTH);
    }
    call.set_return(emu, STATUS_SUCCESS);
}
