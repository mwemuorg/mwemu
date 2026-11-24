use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::memory::read_guest_bytes;
use crate::api::windows::common::ntdll::read_unicode_string;
use crate::api::windows::common::sections;
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::PAGE_READWRITE;
use crate::windows::structures::Peb64;

pub(super) fn session_object(emu: &Emu, name: &str, object: &str) -> bool {
    let Some(session) = emu.maps.get_map_by_name("peb").and_then(|peb| {
        emu.maps
            .read_dword(peb.get_base() + Peb64::SESSION_ID_OFFSET)
    }) else {
        return false;
    };
    if session == 0 {
        return name.eq_ignore_ascii_case(&format!(r"\Windows\{object}"));
    }
    name.eq_ignore_ascii_case(&format!(r"\Sessions\{session}\Windows\{object}"))
}

const PORT_MESSAGE_SIZE: usize = 0x28;
const LPC_REPLY: u16 = 2;
const ALPC_MSGFLG_SYNC_REQUEST: u32 = 0x0002_0000;
const CSR_CAPTURE_BUFFER: usize = PORT_MESSAGE_SIZE;
const CSR_API_NUMBER: usize = CSR_CAPTURE_BUFFER + size_of::<u64>();
const CSR_RETURN_VALUE: usize = CSR_API_NUMBER + size_of::<u32>();
const CSR_CLIENT_CONNECT: u32 = 0;
const CSR_CONNECT_REQUEST: usize = 0x40;
const CSR_CONNECT_SIZE: usize = CSR_CONNECT_REQUEST + 24;
const CSR_CONNECT_DATA: usize = CSR_CONNECT_REQUEST + 8;
const CSR_CONNECT_DATA_LENGTH: usize = CSR_CONNECT_DATA + size_of::<u64>();
const USERSRV_SERVERDLL_INDEX: u32 = 3;
const USERCONNECT_SIZE: u64 = 0x248;
const USERCONNECT_SHARED_INFO: u64 = 8;
const CAPTURE_POINTER_COUNT: u64 = 16;
const CAPTURE_POINTER_OFFSETS: u64 = 32;
const CAPTURE_HEADER_SIZE: u64 = CAPTURE_POINTER_OFFSETS + size_of::<u64>() as u64;
const USER_SERVER_INFO_MAP: &str = "win32_user_server_info";

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

// Win11 USERCONNECT contains one captured SHAREDINFO. Its psi points into the
// read-only USER server data even when no display capabilities are enabled.
fn user_connect_data(
    emu: &Emu,
    view_base: u64,
    storage: u64,
    capture: u64,
    data: u64,
) -> Result<u64, u64> {
    let view = emu
        .maps
        .allocation_at(capture)
        .filter(|view| view.base == view_base)
        .ok_or(STATUS_INVALID_PARAMETER)?;
    if !view.contains(capture, CAPTURE_HEADER_SIZE)
        || emu
            .maps
            .get_mem_by_addr(capture)
            .and_then(|mem| mem.shared_view())
            .is_none_or(|view| view.storage != storage)
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if !emu
        .maps
        .can_access_range(capture, CAPTURE_HEADER_SIZE as usize, Permission::READ)
    {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let length = emu
        .maps
        .read_dword(capture)
        .ok_or(STATUS_ACCESS_VIOLATION)? as u64;
    if !view.contains(capture, length)
        || emu.maps.read_dword(capture + CAPTURE_POINTER_COUNT) != Some(1)
        || emu.maps.read_qword(capture + CAPTURE_POINTER_OFFSETS) != Some(CSR_CONNECT_DATA as u64)
        || data < capture + CAPTURE_HEADER_SIZE
        || data
            .checked_add(USERCONNECT_SIZE)
            .is_none_or(|end| end > capture + length)
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if !emu.maps.validate_write_range(data, USERCONNECT_SIZE) {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    Ok(data)
}

fn write_user_connect(emu: &mut Emu, data: u64) -> Result<(), u64> {
    let server = match emu.maps.get_map_by_name(USER_SERVER_INFO_MAP) {
        Some(server) => server.get_base(),
        None => {
            let address = emu
                .maps
                .map(USER_SERVER_INFO_MAP, PAGE_SIZE, Permission::READ);
            if address == 0 {
                return Err(STATUS_NO_MEMORY);
            }
            address
        }
    };
    let mut info = [0; USERCONNECT_SIZE as usize - USERCONNECT_SHARED_INFO as usize];
    info[..size_of::<u64>()].copy_from_slice(&server.to_le_bytes());
    emu.maps.write_bytes(data + USERCONNECT_SHARED_INFO, &info);
    Ok(())
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
    let port = emu.handle_management.native_object(handle);
    if !matches!(
        port,
        Some(NativeObject::CsrPort | NativeObject::CsrClientPort { .. })
    ) || flags as u32 & !ALPC_MSGFLG_SYNC_REQUEST != 0
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
    if emu.maps.read_dword(send + CSR_API_NUMBER as u64) != Some(CSR_CLIENT_CONNECT)
        || emu.maps.read_word(send + 6) != Some(0)
        || total != CSR_CONNECT_SIZE
    {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtAlpcSendWaitReceivePort",
            emu.pc(),
            call,
        );
    }
    let user_data = match (
        emu.maps.read_dword(send + CSR_CONNECT_REQUEST as u64),
        emu.maps.read_qword(send + CSR_CAPTURE_BUFFER as u64),
        emu.maps.read_qword(send + CSR_CONNECT_DATA as u64),
        emu.maps.read_dword(send + CSR_CONNECT_DATA_LENGTH as u64),
        port,
    ) {
        (Some(0), Some(0), Some(0), Some(0), _) => None,
        (
            Some(USERSRV_SERVERDLL_INDEX),
            Some(capture),
            Some(data),
            Some(size),
            Some(NativeObject::CsrClientPort { view_base, storage }),
        ) if size as u64 == USERCONNECT_SIZE => {
            match user_connect_data(emu, view_base, storage, capture, data) {
                Ok(data) => Some(data),
                Err(status) => return call.set_return(emu, status),
            }
        }
        _ => {
            return policy::unsupported_with_abi(
                emu,
                "ntdll",
                "NtAlpcSendWaitReceivePort",
                emu.pc(),
                call,
            );
        }
    };
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
    if let Some(data) = user_data
        && let Err(status) = write_user_connect(emu, data)
    {
        return call.set_return(emu, status);
    }
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
    const CSR_CONNECTION_SIZE: u32 = 48;
    const REMOTE_PORT_VIEW_SIZE: u32 = 24;
    let attach = data != 0;
    if !csr_port(&name)
        || (attach && !session_object(emu, &name, "ApiPort"))
        || (!attach && server != 0)
    {
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
    // Win11 initializes the fixed-size QoS fields but leaves Length zero on CSR attach.
    let qos_length = emu.maps.read_dword(quality);
    if !(qos_length == Some(SECURITY_QOS_SIZE as u32) || (attach && qos_length == Some(0)))
        || !matches!(emu.maps.read_dword(quality + 4), Some(0..=3))
        || !matches!(emu.maps.read_byte(quality + 8), Some(0..=1))
        || !matches!(emu.maps.read_byte(quality + 9), Some(0..=1))
        || (!attach && length != 0 && emu.maps.read_dword(length) != Some(0))
    {
        return call.set_return(emu, STATUS_INVALID_PARAMETER);
    }
    let server_base = if attach {
        if client == 0 || length == 0 {
            return call.set_return(emu, STATUS_INVALID_PARAMETER);
        }
        if emu.maps.read_dword(length) != Some(CSR_CONNECTION_SIZE) {
            return policy::unsupported_with_abi(emu, "ntdll", "NtConnectPort", emu.pc(), call);
        }
        if !emu
            .maps
            .validate_write_range(data, CSR_CONNECTION_SIZE as u64)
            || (server != 0
                && !emu
                    .maps
                    .validate_write_range(server, REMOTE_PORT_VIEW_SIZE as u64))
        {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        }
        if server != 0 && emu.maps.read_dword(server) != Some(REMOTE_PORT_VIEW_SIZE) {
            return call.set_return(emu, STATUS_INVALID_PARAMETER);
        }
        let Some(map) = emu.maps.get_map_by_name("peb_ro_shared") else {
            return call.set_return(emu, STATUS_OBJECT_NAME_NOT_FOUND);
        };
        Some(map.get_base())
    } else {
        None
    };
    let port = if client != 0 {
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
        let Some(storage) = emu
            .handle_management
            .section_handle(section)
            .map(|section| section.storage)
        else {
            return call.set_return(emu, STATUS_INVALID_HANDLE);
        };
        match sections::map(emu, section, 0, u64::from(offset), size, PAGE_READWRITE) {
            Ok((base, mapped)) => {
                emu.maps.write_qword(client + 24, mapped);
                emu.maps.write_qword(client + 32, base);
                emu.maps.write_qword(client + 40, base);
                NativeObject::CsrClientPort {
                    view_base: base,
                    storage,
                }
            }
            Err(status) => return call.set_return(emu, status),
        }
    } else {
        NativeObject::CsrPort
    };
    if let Some(base) = server_base {
        emu.maps.write_qword(data, base);
        emu.maps
            .write_qword(data + 8, base + Peb64::CSR_STATIC_TABLE_OFFSET);
        if server != 0 {
            // The Base server does not offer a second port view.
            emu.maps.write_qword(server + 8, 0);
            emu.maps.write_qword(server + 16, 0);
        }
    }
    let handle = emu.handle_management.insert_native_object(port);
    emu.maps.write_qword(out, handle);
    if maximum != 0 {
        emu.maps.write_dword(maximum, PORT_MAXIMUM_MESSAGE_LENGTH);
    }
    call.set_return(emu, STATUS_SUCCESS);
}
