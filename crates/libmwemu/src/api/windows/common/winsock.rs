use std::net::Ipv4Addr;

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{Call, pointer_size, writable_pointer, write_pointer};
use crate::winapi::common::{heap, strings, trace};
use crate::winapi::helper::{self, WinApiAbi};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

const WINSOCK_VERSION_2_2: u16 = 0x0202;
const SOCKADDR_IN_SIZE: usize = 16;

const SOCKADDR_PORT_OFFSET: u64 = 2;
const SOCKADDR_ADDRESS_OFFSET: u64 = 4;
const ADDRINFO_SCALARS_SIZE: u64 = 4 * size_of::<u32>() as u64;
const SERVICE_PORTS: &[(&str, u16)] = &[
    ("http", 80),
    ("https", 443),
    ("ftp", 21),
    ("ssh", 22),
    ("smtp", 25),
    ("domain", 53),
];

struct WsaDataLayout {
    size: usize,
    description: usize,
    status: usize,
}

const WSADATA32: WsaDataLayout = WsaDataLayout {
    size: 400,
    description: 4,
    status: 4 + WSADESCRIPTION_LEN + 1,
};
const WSADATA64: WsaDataLayout = WsaDataLayout {
    size: 408,
    description: 16,
    status: 16 + WSADESCRIPTION_LEN + 1,
};

pub(crate) fn socket(emu: &mut Emu, api: &str) {
    trace::api(
        emu,
        &format!("ws2_32!{api}"),
        "family type protocol info group flags",
        true,
        |emu| {
            let socket = helper::socket_create();
            emu.set_last_error(ERROR_SUCCESS);
            Call::Api.finish(emu, socket, 6);
        },
    );
}

pub(crate) fn startup(emu: &mut Emu) {
    trace::api(emu, "ws2_32!WSAStartup", "version data", true, |emu| {
        let requested = Call::Api.arg(emu, 0) as u16;
        let address = Call::Api.arg(emu, 1);
        let major = requested as u8;
        let minor = (requested >> u8::BITS) as u8;
        let negotiated = match major {
            0 => None,
            1 => Some(1 | ((minor.min(1) as u16) << u8::BITS)),
            2 => Some(2 | ((minor.min(2) as u16) << u8::BITS)),
            _ => Some(WINSOCK_VERSION_2_2),
        };
        let result = if let Some(version) = negotiated {
            write_wsa_data(emu, address, version)
        } else {
            WSAVERNOTSUPPORTED
        };
        Call::Api.finish(emu, result, 2);
    });
}

fn write_wsa_data(emu: &mut Emu, address: u64, version: u16) -> u64 {
    let layout = if emu.cfg.arch.is_64bits() {
        &WSADATA64
    } else {
        &WSADATA32
    };
    if address == 0 || !emu.maps.validate_write_range(address, layout.size as u64) {
        return WSAEFAULT;
    }
    let mut data = vec![0; layout.size];
    data[..size_of::<u16>()].copy_from_slice(&version.to_le_bytes());
    data[size_of::<u16>()..2 * size_of::<u16>()]
        .copy_from_slice(&WINSOCK_VERSION_2_2.to_le_bytes());
    let description = b"WinSock 2.0";
    let status = b"Running";
    data[layout.description..layout.description + description.len()].copy_from_slice(description);
    data[layout.status..layout.status + status.len().min(WSASYS_STATUS_LEN)]
        .copy_from_slice(status);
    if emu.maps.write_bytes(address, &data) {
        ERROR_SUCCESS
    } else {
        WSAEFAULT
    }
}

pub(crate) fn socket_name(emu: &mut Emu) {
    trace::api(
        emu,
        "ws2_32!getsockname",
        "socket address length",
        true,
        |emu| {
            let socket = Call::Api.arg(emu, 0);
            let address = Call::Api.arg(emu, 1);
            let length = Call::Api.arg(emu, 2);
            let error = if !helper::socket_exist(socket) {
                WSAENOTSOCK
            } else if address == 0
                || length == 0
                || !emu
                    .maps
                    .can_access_range(length, size_of::<u32>(), Permission::READ_WRITE)
                || emu
                    .maps
                    .read_dword(length)
                    .is_none_or(|n| (n as i32) < SOCKADDR_IN_SIZE as i32)
                || !emu
                    .maps
                    .validate_write_range(address, SOCKADDR_IN_SIZE as u64)
            {
                WSAEFAULT
            } else {
                let mut data = [0; SOCKADDR_IN_SIZE];
                data[..size_of::<u16>()].copy_from_slice(&(AF_INET as u16).to_le_bytes());
                if emu.maps.write_bytes(address, &data)
                    && emu.maps.write_dword(length, SOCKADDR_IN_SIZE as u32)
                {
                    ERROR_SUCCESS
                } else {
                    WSAEFAULT
                }
            };
            emu.set_last_error(error);
            Call::Api.finish(
                emu,
                if error == ERROR_SUCCESS {
                    0
                } else {
                    SOCKET_ERROR
                },
                3,
            );
        },
    );
}

fn text_output(
    emu: &Emu,
    address: u64,
    capacity: u32,
    text: &str,
    wide: bool,
) -> Result<Vec<u8>, u64> {
    let bytes = strings::encode_text(&format!("{text}\0"), wide);
    let unit_size = if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    };
    if address == 0
        || bytes.len() / unit_size > capacity as usize
        || !emu.maps.validate_write_range(address, bytes.len() as u64)
    {
        return Err(WSAEFAULT);
    }
    Ok(bytes)
}

pub(crate) fn host_name(emu: &mut Emu, api: &str, wide: bool) {
    let address = Call::Api.arg(emu, 0);
    let capacity = Call::Api.arg(emu, 1) as i32;
    let result = if capacity <= 0 {
        Err(WSAEFAULT)
    } else {
        text_output(emu, address, capacity as u32, &emu.cfg.host_name, wide).and_then(|bytes| {
            emu.maps
                .write_bytes(address, &bytes)
                .then_some(())
                .ok_or(WSAEFAULT)
        })
    };
    log_red!(
        emu,
        "ws2_32!{api} buffer:0x{:x} capacity:{} => {:?}",
        address,
        capacity,
        result
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    Call::Api.finish(emu, if result.is_ok() { 0 } else { SOCKET_ERROR }, 2);
}

pub(crate) fn name_info(emu: &mut Emu, api: &str, wide: bool) {
    let socket_address = Call::Api.arg(emu, 0);
    let socket_length = Call::Api.arg(emu, 1) as u32;
    let host = Call::Api.arg(emu, 2);
    let host_capacity = Call::Api.arg(emu, 3) as u32;
    let service = Call::Api.arg(emu, 4);
    let service_capacity = Call::Api.arg(emu, 5) as u32;
    let flags = Call::Api.arg(emu, 6) as u32;
    let result = (|| {
        if host == 0 && service == 0 {
            return Err(WSAHOST_NOT_FOUND);
        }
        if (host != 0 && host_capacity == 0)
            || (service != 0 && service_capacity == 0)
            || flags & !(NI_NOFQDN | NI_NUMERICHOST | NI_NAMEREQD | NI_NUMERICSERV | NI_DGRAM) != 0
        {
            return Err(WSAEINVAL);
        }
        if socket_address == 0
            || socket_length < SOCKADDR_IN_SIZE as u32
            || !emu
                .maps
                .can_access_range(socket_address, SOCKADDR_IN_SIZE, Permission::READ)
        {
            return Err(WSAEFAULT);
        }
        if emu.maps.read_word(socket_address) != Some(AF_INET as u16) {
            return Err(WSAEAFNOSUPPORT);
        }
        let port = u16::from_be(
            emu.maps
                .read_word(socket_address + SOCKADDR_PORT_OFFSET)
                .ok_or(WSAEFAULT)?,
        );
        let ip = Ipv4Addr::from(
            emu.maps
                .read_dword(socket_address + SOCKADDR_ADDRESS_OFFSET)
                .ok_or(WSAEFAULT)?
                .to_le_bytes(),
        );
        if host != 0
            && ip != Ipv4Addr::LOCALHOST
            && flags & NI_NAMEREQD != 0
            && flags & NI_NUMERICHOST == 0
        {
            return Err(WSAHOST_NOT_FOUND);
        }
        let host_name = if ip == Ipv4Addr::LOCALHOST && flags & NI_NUMERICHOST == 0 {
            "localhost".to_string()
        } else {
            ip.to_string()
        };
        let service_name = if flags & NI_NUMERICSERV == 0 {
            SERVICE_PORTS
                .iter()
                .find(|(name, p)| *p == port && (flags & NI_DGRAM == 0 || *name == "domain"))
                .map(|(name, _)| name.to_string())
                .unwrap_or_else(|| port.to_string())
        } else {
            port.to_string()
        };
        let host_bytes = if host == 0 {
            None
        } else {
            Some(text_output(emu, host, host_capacity, &host_name, wide)?)
        };
        let service_bytes = if service == 0 {
            None
        } else {
            Some(text_output(
                emu,
                service,
                service_capacity,
                &service_name,
                wide,
            )?)
        };
        // Validate both outputs before modifying either one.
        if let Some(bytes) = host_bytes
            && !emu.maps.write_bytes(host, &bytes)
        {
            return Err(WSAEFAULT);
        }
        if let Some(bytes) = service_bytes
            && !emu.maps.write_bytes(service, &bytes)
        {
            return Err(WSAEFAULT);
        }
        Ok(())
    })();
    let error = result.err().unwrap_or(ERROR_SUCCESS);
    log_red!(
        emu,
        "ws2_32!{api} address:0x{:x} host:0x{:x} service:0x{:x} => {}",
        socket_address,
        host,
        service,
        error
    );
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    Call::Api.finish(emu, error, 7);
}

/// Preserve the emulator's offline IPv4 profile: names resolve to loopback.
/// The single result, sockaddr and optional name share one heap allocation.
pub(crate) fn address_info(emu: &mut Emu, api: &str, wide: bool) {
    let node_pointer = Call::Api.arg(emu, 0);
    let service_pointer = Call::Api.arg(emu, 1);
    let node = strings::read_text(emu, node_pointer, wide);
    let service = strings::read_text(emu, service_pointer, wide);
    let hints = Call::Api.arg(emu, 2);
    let output = Call::Api.arg(emu, 3);
    let result = (|| {
        if output == 0 || !writable_pointer(emu, output) {
            return Err(WSAEFAULT);
        }
        if !write_pointer(emu, output, 0) {
            return Err(WSAEFAULT);
        }
        let mut fields = [0, AF_INET, SOCK_STREAM, IPPROTO_TCP];
        if hints != 0 {
            if !emu
                .maps
                .can_access_range(hints, ADDRINFO_SCALARS_SIZE as usize, Permission::READ)
            {
                return Err(WSAEFAULT);
            }
            for (index, field) in fields.iter_mut().enumerate() {
                let value = emu
                    .maps
                    .read_dword(hints + (index * size_of::<u32>()) as u64)
                    .ok_or(WSAEFAULT)?;
                if index == 0 || value != 0 {
                    *field = value;
                }
            }
        }
        if fields[1] != AF_INET {
            return Err(WSAEAFNOSUPPORT);
        }
        if node_pointer == 0 && service_pointer == 0 {
            return Err(WSAHOST_NOT_FOUND);
        }
        let port = if service.is_empty() {
            0
        } else {
            SERVICE_PORTS
                .iter()
                .find(|(name, _)| *name == service || (service == "dns" && *name == "domain"))
                .map(|(_, port)| *port)
                .or_else(|| service.parse::<u16>().ok())
                .ok_or(WSATYPE_NOT_FOUND)?
        };
        let canonical = if node.is_empty() || node == "localhost" {
            "localhost.localdomain".to_string()
        } else if node == "127.0.0.1" {
            "localhost".to_string()
        } else {
            format!("{node}.localdomain")
        };
        let name = strings::encode_text(&format!("{canonical}\0"), wide);
        let pointer = pointer_size(emu);
        let structure_size = ADDRINFO_SCALARS_SIZE + 4 * pointer;
        let total = structure_size + SOCKADDR_IN_SIZE as u64 + name.len() as u64;
        let base = heap::heap_allocate(emu, 0, total).ok_or(WSA_NOT_ENOUGH_MEMORY)?;
        let sockaddr = base + structure_size;
        let canonical = sockaddr + SOCKADDR_IN_SIZE as u64;
        let mut data = vec![0; total as usize];
        for (index, value) in fields.into_iter().enumerate() {
            let offset = index * size_of::<u32>();
            data[offset..offset + size_of::<u32>()].copy_from_slice(&value.to_le_bytes());
        }
        for (index, value) in [SOCKADDR_IN_SIZE as u64, canonical, sockaddr, 0]
            .into_iter()
            .enumerate()
        {
            let offset = ADDRINFO_SCALARS_SIZE as usize + index * pointer as usize;
            data[offset..offset + pointer as usize]
                .copy_from_slice(&value.to_le_bytes()[..pointer as usize]);
        }
        let offset = structure_size as usize;
        data[offset..offset + size_of::<u16>()].copy_from_slice(&(AF_INET as u16).to_le_bytes());
        data[offset + SOCKADDR_PORT_OFFSET as usize..offset + SOCKADDR_ADDRESS_OFFSET as usize]
            .copy_from_slice(&port.to_be_bytes());
        data[offset + SOCKADDR_ADDRESS_OFFSET as usize
            ..offset + SOCKADDR_ADDRESS_OFFSET as usize + size_of::<Ipv4Addr>()]
            .copy_from_slice(&Ipv4Addr::LOCALHOST.octets());
        data[offset + SOCKADDR_IN_SIZE..].copy_from_slice(&name);
        if !emu.maps.write_bytes(base, &data) || !write_pointer(emu, output, base) {
            heap::heap_free(emu, 0, base);
            return Err(WSAEFAULT);
        }
        Ok(())
    })();
    let error = result.err().unwrap_or(ERROR_SUCCESS);
    log_red!(
        emu,
        "ws2_32!{api} node:'{}' service:'{}' => {}",
        node,
        service,
        error
    );
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    Call::Api.finish(emu, error, 4);
}

pub(crate) fn free_address_info(emu: &mut Emu, api: &str) {
    let address = Call::Api.arg(emu, 0);
    log_red!(emu, "ws2_32!{api} address:0x{:x}", address);
    if address != 0 {
        heap::heap_free(emu, 0, address);
    }
    WinApiAbi::stdcall_ret(emu, 1);
}
