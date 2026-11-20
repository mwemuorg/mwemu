pub(crate) mod sockets;

use std::net::Ipv4Addr;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_array, writable_pointer, write_pointer};
use crate::winapi::common::{heap, strings, trace};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;
use crate::windows::structures::Hostent;

const WINSOCK_VERSION_2_2: u16 = 0x0202;
const AI_PASSIVE: u32 = 1;
const AI_CANONNAME: u32 = 2;
const AI_NUMERICHOST: u32 = 4;
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

pub(crate) fn startup(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    trace::api(emu, "ws2_32!WSAStartup", "version data", true, |emu| {
        let requested = api_try!(emu, call.arg_u16(emu, 0));
        let address = api_try!(emu, call.arg(emu, 1));
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
        if result == 0 {
            emu.sockets.startups = emu.sockets.startups.saturating_add(1);
        }
        call.set_return(emu, result);
        call.cleanup_args(emu, 2);
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
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg_i32(emu, 1));
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
        "ws2_32!{api} buffer={address:#x} capacity={capacity} result={result:?}"
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, if result.is_ok() { 0 } else { SOCKET_ERROR });
    call.cleanup_args(emu, 2);
}

pub(crate) fn name_info(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let socket_address = api_try!(emu, call.arg(emu, 0));
    let socket_length = api_try!(emu, call.arg_u32(emu, 1));
    let host = api_try!(emu, call.arg(emu, 2));
    let host_capacity = api_try!(emu, call.arg_u32(emu, 3));
    let service = api_try!(emu, call.arg(emu, 4));
    let service_capacity = api_try!(emu, call.arg_u32(emu, 5));
    let flags = api_try!(emu, call.arg_u32(emu, 6));
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
        "ws2_32!{api} address={socket_address:#x} host_buffer={host:#x} service_buffer={service:#x} status={error}"
    );
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    call.set_return(emu, error);
    call.cleanup_args(emu, 7);
}

/// Preserve the emulator's offline IPv4 profile: names resolve to loopback.
/// The single result, sockaddr and optional name share one heap allocation.
pub(crate) fn address_info(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let node_pointer = api_try!(emu, call.arg(emu, 0));
    let service_pointer = api_try!(emu, call.arg(emu, 1));
    let text = |address| {
        if address == 0 {
            Some(String::new())
        } else {
            strings::read_text_checked(emu, address, wide)
        }
    };
    let node = text(node_pointer);
    let service = text(service_pointer);
    let hints = api_try!(emu, call.arg(emu, 2));
    let output = api_try!(emu, call.arg(emu, 3));
    let result = (|| {
        if output == 0 || !writable_pointer(emu, output) {
            return Err(WSAEFAULT);
        }
        if !write_pointer(emu, output, 0) {
            return Err(WSAEFAULT);
        }
        let mut fields = [0; 4];
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
                *field = value;
            }
        }
        if fields[1] == 0 {
            fields[1] = AF_INET;
        }
        if fields[1] != AF_INET {
            return Err(WSAEAFNOSUPPORT);
        }
        let (socket_type, protocol) = match (fields[2], fields[3]) {
            (0 | SOCK_STREAM, 0 | IPPROTO_TCP) => (SOCK_STREAM, IPPROTO_TCP),
            (0 | SOCK_DGRAM, 0 | IPPROTO_UDP) => (SOCK_DGRAM, IPPROTO_UDP),
            _ => return Err(WSAESOCKTNOSUPPORT),
        };
        fields[2] = socket_type;
        fields[3] = protocol;
        if node_pointer == 0 && service_pointer == 0 {
            return Err(WSAHOST_NOT_FOUND);
        }
        let node = node.as_deref().ok_or(WSAEFAULT)?;
        let service = service.as_deref().ok_or(WSAEFAULT)?;
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
        let address = if node_pointer == 0 && fields[0] & AI_PASSIVE != 0 {
            Ipv4Addr::UNSPECIFIED
        } else {
            resolve_local_ipv4(emu, node, fields[0] & AI_NUMERICHOST != 0)?
        };
        let name = if fields[0] & AI_CANONNAME != 0 && node_pointer != 0 {
            strings::encode_text(&format!("{node}\0"), wide)
        } else {
            Vec::new()
        };
        let pointer = pointer_size(emu);
        let structure_size = ADDRINFO_SCALARS_SIZE + 4 * pointer;
        let total = structure_size + SOCKADDR_IN_SIZE as u64 + name.len() as u64;
        let base = heap::heap_allocate(emu, 0, total).ok_or(WSA_NOT_ENOUGH_MEMORY)?;
        let sockaddr = base + structure_size;
        let canonical = if name.is_empty() {
            0
        } else {
            sockaddr + SOCKADDR_IN_SIZE as u64
        };
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
            .copy_from_slice(&address.octets());
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
        "ws2_32!{api} node={node:?} service={service:?} status={error}"
    );
    if error != ERROR_SUCCESS {
        emu.set_last_error(error);
    }
    call.set_return(emu, error);
    call.cleanup_args(emu, 4);
}

pub(crate) fn free_address_info(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!{api} address={address:#x}");
    if address != 0 {
        heap::heap_free(emu, 0, address);
    }
    call.cleanup_args(emu, 1);
}

fn socket_set_contains(emu: &Emu, set: u64, socket: u64) -> Option<bool> {
    let count = u32::from_le_bytes(read_array(emu, set)?);
    if count == 0 {
        return Some(false);
    }
    // fd_count is a DWORD; the SOCKET array is aligned to the guest pointer size.
    let width = pointer_size(emu);
    let array = set.checked_add(width)?;
    let size = count as u64 * width;
    if !emu
        .maps
        .can_access_range(array, usize::try_from(size).ok()?, Permission::READ)
    {
        return None;
    }
    for index in 0..count as u64 {
        let address = array.checked_add(index * width)?;
        let entry = if width == size_of::<u64>() as u64 {
            u64::from_le_bytes(read_array(emu, address)?)
        } else {
            u32::from_le_bytes(read_array(emu, address)?) as u64
        };
        if entry == socket {
            return Some(true);
        }
    }
    Some(false)
}

pub fn __WSAFDIsSet(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let socket = api_try!(emu, call.arg(emu, 0));
    let set = api_try!(emu, call.arg(emu, 1));
    let result = socket_set_contains(emu, set, socket);
    log_red!(
        emu,
        "ws2_32!__WSAFDIsSet socket={socket:#x} set={set:#x} => {result:?}"
    );
    if let Some(found) = result {
        call.set_return(emu, found as u64);
        call.cleanup_args(emu, 2);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

fn store_ipv4_host(emu: &mut Emu, name: &str, ip: Ipv4Addr) -> Option<u64> {
    const DNS_NAME_CAPACITY: u64 = 256;
    if name.len() as u64 >= DNS_NAME_CAPACITY {
        return None;
    }
    let width = pointer_size(emu);
    let address_list_field = if width == 8 { 24 } else { 12 };
    let table = address_list_field + width;
    let octets = table + 3 * width;
    let name_at = octets + 4;
    let key = format!("winsock_hostent_{}", emu.current_thread().id);
    let base = if let Some(map) = emu.maps.get_map_by_name(&key) {
        map.get_base()
    } else {
        let size = name_at + DNS_NAME_CAPACITY;
        let base = emu.maps.alloc(size)?;
        emu.maps
            .create_map(&key, base, size, Permission::READ_WRITE)
            .ok()?;
        base
    };
    Hostent {
        hname: base + name_at,
        alias_list: base + table + 2 * width,
        addr_type: AF_INET as u16,
        length: size_of::<Ipv4Addr>() as u16,
        addr_list: base + table,
    }
    .save(base, &mut emu.maps);
    if write_pointer(emu, base + table, base + octets)
        && write_pointer(emu, base + table + width, 0)
        && write_pointer(emu, base + table + 2 * width, 0)
        && emu.maps.write_bytes(base + octets, &ip.octets())
        && emu.maps.write_bytes(base + name_at, name.as_bytes())
        && emu.maps.write_byte(base + name_at + name.len() as u64, 0)
    {
        Some(base)
    } else {
        None
    }
}

fn resolve_local_ipv4(emu: &Emu, name: &str, numeric_only: bool) -> Result<Ipv4Addr, u64> {
    if let Ok(address) = name.parse() {
        return Ok(address);
    }
    let name = name.trim_end_matches('.');
    if !numeric_only
        && (name.is_empty()
            || name.eq_ignore_ascii_case("localhost")
            || name.eq_ignore_ascii_case(&emu.cfg.host_name))
    {
        Ok(Ipv4Addr::LOCALHOST)
    } else {
        Err(WSAHOST_NOT_FOUND)
    }
}

pub fn gethostbyname(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let result = (|| {
        let name = if source == 0 {
            emu.cfg.host_name.clone()
        } else {
            strings::read_text_checked(emu, source, false).ok_or(WSAEFAULT)?
        };
        let address = resolve_local_ipv4(emu, &name, false)?;
        let name = if name.is_empty() {
            emu.cfg.host_name.clone()
        } else {
            name
        };
        store_ipv4_host(emu, &name, address).ok_or(WSAENOBUFS)
    })();
    log_red!(
        emu,
        "ws2_32!gethostbyname name={name} result={result:?}",
        name = trace::input_string(emu, source, Encoding::Ansi)
    );
    let ret = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, ret);
    call.cleanup_args(emu, 1);
}

pub fn gethostbyaddr(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [address, length, family] = api_try!(emu, call.args(emu));
    let result = (|| {
        if family as u32 != AF_INET {
            return Err(WSAEAFNOSUPPORT);
        }
        if length as i32 != size_of::<Ipv4Addr>() as i32 {
            return Err(WSAEFAULT);
        }
        let ip = Ipv4Addr::from(read_array::<4>(emu, address).ok_or(WSAEFAULT)?);
        if !ip.is_loopback() {
            return Err(WSAHOST_NOT_FOUND);
        }
        store_ipv4_host(emu, "localhost", ip).ok_or(WSAENOBUFS)
    })();
    log_red!(
        emu,
        "ws2_32!gethostbyaddr address={address:#x} length={length} family={family} result={result:?}"
    );
    let ret = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, ret);
    call.cleanup_args(emu, 3);
}

pub fn WSAGetLastError(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let error = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError error={error}");
    call.set_return(emu, error);
}

pub fn WSASetLastError(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let error = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ws2_32!WSASetLastError error={error}");
    emu.set_last_error(error as u64);
    call.cleanup_args(emu, 1);
}
