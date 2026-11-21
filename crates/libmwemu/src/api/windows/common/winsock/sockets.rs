use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6};

use serde::{Deserialize, Serialize};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::read_array;
use crate::winapi::common::strings;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

const FIRST_SOCKET: u64 = 0x100;
const FIRST_DYNAMIC_PORT: u16 = 49152;
const DEFAULT_BUFFER_SIZE: u32 = 65536;
const SOCKADDR_IN_SIZE: usize = 16;
const SOCKADDR_IN6_SIZE: usize = 28;
const PORT_OFFSET: usize = 2;
const IPV4_MAX_UDP_PAYLOAD: u32 = 65507;
const IPV6_MAX_UDP_PAYLOAD: u32 = 65527;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Socket {
    family: u32,
    kind: u32,
    local: Option<Vec<u8>>,
    peer: Option<Vec<u8>>,
    listening: bool,
    nonblocking: bool,
    send_shutdown: bool,
    receive_shutdown: bool,
    options: BTreeMap<(u32, u32), u32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct SocketState {
    sockets: BTreeMap<u64, Socket>,
    next_socket: u64,
    next_port: u16,
    pub(super) startups: u32,
}

impl SocketState {
    pub(crate) fn contains(&self, handle: u64) -> bool {
        self.sockets.contains_key(&handle)
    }

    fn get(&self, handle: u64) -> Result<&Socket, u64> {
        self.sockets.get(&handle).ok_or(WSAENOTSOCK)
    }

    fn get_mut(&mut self, handle: u64) -> Result<&mut Socket, u64> {
        self.sockets.get_mut(&handle).ok_or(WSAENOTSOCK)
    }

    fn create(&mut self, family: u32, kind: u32, protocol: u32) -> Result<u64, u64> {
        if !matches!(family, AF_INET | AF_INET6) {
            return Err(WSAEAFNOSUPPORT);
        }
        let expected = match kind {
            SOCK_STREAM => IPPROTO_TCP,
            SOCK_DGRAM => IPPROTO_UDP,
            _ => return Err(WSAESOCKTNOSUPPORT),
        };
        if protocol != 0 && protocol != expected {
            return Err(WSAEPROTONOSUPPORT);
        }
        let handle = self.next_socket.max(FIRST_SOCKET);
        if handle >= u32::MAX as u64 {
            return Err(WSAEMFILE);
        }
        self.next_socket = handle + 1;
        self.sockets.insert(
            handle,
            Socket {
                family,
                kind,
                local: None,
                peer: None,
                listening: false,
                nonblocking: false,
                send_shutdown: false,
                receive_shutdown: false,
                options: BTreeMap::new(),
            },
        );
        Ok(handle)
    }

    fn bind(&mut self, handle: u64, mut address: Vec<u8>) -> Result<(), u64> {
        let socket = self.get(handle)?;
        if socket.local.is_some() {
            return Err(WSAEINVAL);
        }
        if address[PORT_OFFSET..PORT_OFFSET + size_of::<u16>()] == [0, 0] {
            let start = self.next_port.max(FIRST_DYNAMIC_PORT);
            let mut port = start;
            loop {
                address[PORT_OFFSET..PORT_OFFSET + size_of::<u16>()]
                    .copy_from_slice(&port.to_be_bytes());
                if !self.address_used(socket.family, socket.kind, &address) {
                    self.next_port = port.checked_add(1).unwrap_or(FIRST_DYNAMIC_PORT);
                    break;
                }
                port = port.checked_add(1).unwrap_or(FIRST_DYNAMIC_PORT);
                if port == start {
                    return Err(WSAEADDRINUSE);
                }
            }
        } else if self.address_used(socket.family, socket.kind, &address) {
            return Err(WSAEADDRINUSE);
        }
        self.get_mut(handle)?.local = Some(address);
        Ok(())
    }

    fn address_used(&self, family: u32, kind: u32, address: &[u8]) -> bool {
        // This offline profile reserves a port across local addresses of one family.
        self.sockets.values().any(|socket| {
            socket.family == family
                && socket.kind == kind
                && socket.local.as_ref().is_some_and(|local| {
                    local[PORT_OFFSET..PORT_OFFSET + size_of::<u16>()]
                        == address[PORT_OFFSET..PORT_OFFSET + size_of::<u16>()]
                })
        })
    }

    fn autobind(&mut self, handle: u64) -> Result<(), u64> {
        let socket = self.get(handle)?;
        if socket.local.is_none() {
            let mut address = vec![0; address_size(socket.family)];
            address[..size_of::<u16>()].copy_from_slice(&(socket.family as u16).to_le_bytes());
            self.bind(handle, address)?;
        }
        Ok(())
    }
}

fn address_size(family: u32) -> usize {
    if family == AF_INET6 {
        SOCKADDR_IN6_SIZE
    } else {
        SOCKADDR_IN_SIZE
    }
}

fn read_address(emu: &Emu, address: u64, length: i32, family: u32) -> Result<Vec<u8>, u64> {
    let size = address_size(family);
    if address == 0
        || length < size as i32
        || !emu.maps.can_access_range(address, size, Permission::READ)
    {
        return Err(WSAEFAULT);
    }
    let data = if family == AF_INET6 {
        read_array::<SOCKADDR_IN6_SIZE>(emu, address)
            .ok_or(WSAEFAULT)?
            .to_vec()
    } else {
        read_array::<SOCKADDR_IN_SIZE>(emu, address)
            .ok_or(WSAEFAULT)?
            .to_vec()
    };
    if u16::from_le_bytes([data[0], data[1]]) as u32 != family {
        return Err(WSAEAFNOSUPPORT);
    }
    Ok(data)
}

fn output_capacity(emu: &Emu, output: u64, length: u64, needed: usize) -> Result<(), u64> {
    let capacity = i32::from_le_bytes(read_array(emu, length).ok_or(WSAEFAULT)?);
    if output == 0
        || length == 0
        || capacity < needed as i32
        || !emu
            .maps
            .validate_write_range(length, size_of::<i32>() as u64)
        || !emu.maps.validate_write_range(output, needed as u64)
    {
        return Err(WSAEFAULT);
    }
    Ok(())
}

fn return_result(emu: &mut Emu, slots: usize, result: Result<u64, u64>, socket_result: bool) {
    let call = WinApi(Stdcall);
    let value = match result {
        Ok(value) => value,
        Err(error) => {
            emu.set_last_error(error);
            if socket_result && emu.cfg.arch.is_64bits() {
                INVALID_SOCKET
            } else {
                SOCKET_ERROR
            }
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

pub fn socket(emu: &mut Emu) {
    create_socket(emu, "socket", false);
}

pub(crate) fn create_socket(emu: &mut Emu, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let kind = api_try!(emu, call.arg_u32(emu, 1));
    let protocol = api_try!(emu, call.arg_u32(emu, 2));
    if extended {
        let info = api_try!(emu, call.arg(emu, 3));
        let group = api_try!(emu, call.arg_u32(emu, 4));
        let flags = api_try!(emu, call.arg_u32(emu, 5));
        if info != 0
            || group != 0
            || flags & !(WSA_FLAG_OVERLAPPED | WSA_FLAG_NO_HANDLE_INHERIT) != 0
        {
            policy::unsupported(emu, "ws2_32", api, emu.pc());
            return;
        }
    }
    log_red!(
        emu,
        "ws2_32!{api} family={family} type={kind} protocol={protocol}"
    );
    let result = emu.sockets.create(family, kind, protocol);
    return_result(emu, if extended { 6 } else { 3 }, result, true);
}

pub fn closesocket(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!closesocket socket={handle:#x}");
    let result = emu
        .sockets
        .sockets
        .remove(&handle)
        .map(|_| 0)
        .ok_or(WSAENOTSOCK);
    return_result(emu, 1, result, false);
}

pub fn bind(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(
        emu,
        "ws2_32!bind socket={handle:#x} address={address:#x} length={length}"
    );
    let result = (|| {
        let family = emu.sockets.get(handle)?.family;
        let address = read_address(emu, address, length, family)?;
        emu.sockets.bind(handle, address)?;
        Ok(0)
    })();
    return_result(emu, 3, result, false);
}

pub fn listen(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let backlog = api_try!(emu, call.arg_i32(emu, 1));
    log_red!(emu, "ws2_32!listen socket={handle:#x} backlog={backlog}");
    let result = (|| {
        let socket = emu.sockets.get_mut(handle)?;
        if socket.kind != SOCK_STREAM {
            return Err(WSAEOPNOTSUPP);
        }
        if socket.local.is_none() {
            return Err(WSAEINVAL);
        }
        socket.listening = true;
        Ok(0)
    })();
    return_result(emu, 2, result, false);
}

pub fn accept(emu: &mut Emu) {
    accept_socket(emu, false);
}

pub fn WSAAccept(emu: &mut Emu) {
    accept_socket(emu, true);
}

fn accept_socket(emu: &mut Emu, extended: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg(emu, 2));
    if extended {
        api_try!(emu, call.arg(emu, 3));
        api_try!(emu, call.arg(emu, 4));
    }
    let api = if extended { "WSAAccept" } else { "accept" };
    log_red!(
        emu,
        "ws2_32!{api} socket={handle:#x} address={address:#x} length={length:#x}"
    );
    let result = (|| {
        let socket = emu.sockets.get(handle)?;
        if socket.kind != SOCK_STREAM {
            return Err(WSAEOPNOTSUPP);
        }
        if !socket.listening {
            return Err(WSAEINVAL);
        }
        if address != 0 && length != 0 {
            output_capacity(emu, address, length, address_size(socket.family))?;
        }
        if socket.nonblocking {
            return Err(WSAEWOULDBLOCK);
        }
        Ok(0)
    })();
    if result.is_ok() {
        // No incoming network or blocking-wait model exists.
        policy::unsupported(emu, "ws2_32", api, emu.pc());
    } else {
        return_result(emu, if extended { 5 } else { 3 }, result, true);
    }
}

pub fn getsockname(emu: &mut Emu) {
    socket_name(emu, false);
}

pub fn getpeername(emu: &mut Emu) {
    socket_name(emu, true);
}

fn socket_name(emu: &mut Emu, peer: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg(emu, 2));
    let api = if peer { "getpeername" } else { "getsockname" };
    log_red!(
        emu,
        "ws2_32!{api} socket={handle:#x} output={output:#x} length={length:#x}"
    );
    let result = (|| {
        let socket = emu.sockets.get(handle)?;
        let bytes = if peer {
            socket.peer.as_ref().ok_or(WSAENOTCONN)?
        } else {
            socket.local.as_ref().ok_or(WSAEINVAL)?
        }
        .clone();
        output_capacity(emu, output, length, bytes.len())?;
        if !emu.maps.write_bytes(output, &bytes)
            || !emu
                .maps
                .write_bytes(length, &(bytes.len() as i32).to_le_bytes())
        {
            return Err(WSAEFAULT);
        }
        Ok(0)
    })();
    return_result(emu, 3, result, false);
}

pub fn connect(emu: &mut Emu) {
    connect_socket(emu, false);
}

pub fn WSAConnect(emu: &mut Emu) {
    connect_socket(emu, true);
}

fn connect_socket(emu: &mut Emu, extended: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg_i32(emu, 2));
    if extended {
        let caller_data = api_try!(emu, call.arg(emu, 3));
        let callee_data = api_try!(emu, call.arg(emu, 4));
        let send_qos = api_try!(emu, call.arg(emu, 5));
        let receive_qos = api_try!(emu, call.arg(emu, 6));
        if caller_data != 0 || callee_data != 0 || send_qos != 0 || receive_qos != 0 {
            policy::unsupported(emu, "ws2_32", "WSAConnect", emu.pc());
            return;
        }
    }
    let api = if extended { "WSAConnect" } else { "connect" };
    log_red!(
        emu,
        "ws2_32!{api} socket={handle:#x} address={address:#x} length={length}"
    );
    let result = (|| {
        let socket = emu.sockets.get(handle)?;
        let address = read_address(emu, address, length, socket.family)?;
        if socket.kind == SOCK_STREAM {
            // The offline profile has no remote listeners.
            return Err(WSAECONNREFUSED);
        }
        emu.sockets.autobind(handle)?;
        emu.sockets.get_mut(handle)?.peer = Some(address);
        Ok(0)
    })();
    return_result(emu, if extended { 7 } else { 3 }, result, false);
}

pub fn shutdown(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let how = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "ws2_32!shutdown socket={handle:#x} how={how}");
    let result = (|| {
        let socket = emu.sockets.get_mut(handle)?;
        if !matches!(how, SD_RECEIVE | SD_SEND | SD_BOTH) {
            return Err(WSAEINVAL);
        }
        if socket.peer.is_none() {
            return Err(WSAENOTCONN);
        }
        socket.receive_shutdown |= matches!(how, SD_RECEIVE | SD_BOTH);
        socket.send_shutdown |= matches!(how, SD_SEND | SD_BOTH);
        Ok(0)
    })();
    return_result(emu, 2, result, false);
}

fn option_kind(socket: &Socket, level: u32, option: u32, write: bool) -> Result<bool, u64> {
    match (level, option) {
        (SOL_SOCKET, SO_TYPE | SO_ERROR | SO_ACCEPTCONN) if !write => Ok(false),
        (SOL_SOCKET, SO_KEEPALIVE | SO_BROADCAST | SO_OOBINLINE) => Ok(true),
        (SOL_SOCKET, SO_SNDBUF | SO_RCVBUF | SO_SNDTIMEO | SO_RCVTIMEO | SO_LINGER) => Ok(false),
        (IPPROTO_TCP, TCP_NODELAY) if socket.kind == SOCK_STREAM => Ok(true),
        _ => Err(WSAENOPROTOOPT),
    }
}

pub fn getsockopt(emu: &mut Emu) {
    socket_option(emu, false);
}

pub fn setsockopt(emu: &mut Emu) {
    socket_option(emu, true);
}

fn socket_option(emu: &mut Emu, write: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let level = api_try!(emu, call.arg_u32(emu, 1));
    let option = api_try!(emu, call.arg_u32(emu, 2));
    let value = api_try!(emu, call.arg(emu, 3));
    let length = api_try!(emu, call.arg(emu, 4));
    let api = if write { "setsockopt" } else { "getsockopt" };
    log_red!(
        emu,
        "ws2_32!{api} socket={handle:#x} level={level:#x} option={option:#x}"
    );
    let result = (|| {
        let socket = emu.sockets.get(handle)?;
        let boolean = option_kind(socket, level, option, write)?;
        if write {
            if (length as u32 as i32) < size_of::<u32>() as i32 || value == 0 {
                return Err(WSAEFAULT);
            }
            let raw = u32::from_le_bytes(read_array(emu, value).ok_or(WSAEFAULT)?);
            let raw = if boolean { (raw != 0) as u32 } else { raw };
            emu.sockets
                .get_mut(handle)?
                .options
                .insert((level, option), raw);
        } else {
            let raw = match (level, option) {
                (SOL_SOCKET, SO_TYPE) => socket.kind,
                (SOL_SOCKET, SO_ERROR) => 0,
                (SOL_SOCKET, SO_ACCEPTCONN) => socket.listening as u32,
                _ => *socket
                    .options
                    .get(&(level, option))
                    .unwrap_or(&match option {
                        SO_SNDBUF | SO_RCVBUF => DEFAULT_BUFFER_SIZE,
                        _ => 0,
                    }),
            };
            output_capacity(emu, value, length, size_of::<u32>())?;
            if !emu.maps.write_bytes(value, &raw.to_le_bytes())
                || !emu
                    .maps
                    .write_bytes(length, &(size_of::<u32>() as u32).to_le_bytes())
            {
                return Err(WSAEFAULT);
            }
        }
        Ok(0)
    })();
    return_result(emu, 5, result, false);
}

pub fn WSAEventSelect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let socket = api_try!(emu, call.arg(emu, 0));
    let event = api_try!(emu, call.arg(emu, 1));
    let network_events = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(
        emu,
        "ws2_32!WSAEventSelect socket={socket:#x} event={event:#x} network_events={network_events:#x}"
    );
    let result = match emu.sockets.get_mut(socket) {
        Err(error) => Err(error),
        Ok(socket) if network_events == 0 => {
            socket.nonblocking = true;
            Ok(0)
        }
        Ok(_) => {
            policy::unsupported(emu, "ws2_32", "WSAEventSelect", emu.pc());
            return;
        }
    };
    return_result(emu, 3, result, false);
}

pub fn ioctlsocket(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let command = api_try!(emu, call.arg_u32(emu, 1));
    let argument = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ws2_32!ioctlsocket socket={handle:#x} command={command:#x} argument={argument:#x}"
    );
    let result = (|| {
        emu.sockets.get(handle)?;
        if argument == 0 {
            return Err(WSAEFAULT);
        }
        match command {
            FIONBIO => {
                let value = u32::from_le_bytes(read_array(emu, argument).ok_or(WSAEFAULT)?);
                emu.sockets.get_mut(handle)?.nonblocking = value != 0;
            }
            FIONREAD => {
                if !emu.maps.write_bytes(argument, &0u32.to_le_bytes()) {
                    return Err(WSAEFAULT);
                }
            }
            _ => return Err(WSAEINVAL),
        }
        Ok(0)
    })();
    return_result(emu, 3, result, false);
}

pub fn send(emu: &mut Emu) {
    transfer(emu, false);
}
pub fn recv(emu: &mut Emu) {
    transfer(emu, true);
}

fn transfer(emu: &mut Emu, receive: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let length = api_try!(emu, call.arg_i32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let api = if receive { "recv" } else { "send" };
    log_red!(
        emu,
        "ws2_32!{api} socket={handle:#x} buffer={buffer:#x} length={length} flags={flags:#x}"
    );
    let result = (|| {
        let socket = emu.sockets.get(handle)?;
        if length < 0 {
            return Err(WSAEINVAL);
        }
        if socket.peer.is_none() {
            return Err(WSAENOTCONN);
        }
        if if receive {
            socket.receive_shutdown
        } else {
            socket.send_shutdown
        } {
            return Err(WSAESHUTDOWN);
        }
        if length != 0
            && (buffer == 0
                || !emu.maps.can_access_range(
                    buffer,
                    length as usize,
                    if receive {
                        Permission::WRITE
                    } else {
                        Permission::READ
                    },
                ))
        {
            return Err(WSAEFAULT);
        }
        if flags != 0 {
            return Err(WSAEOPNOTSUPP);
        }
        if receive {
            if socket.nonblocking {
                return Err(WSAEWOULDBLOCK);
            }
            return Ok(None);
        }
        // UDP output is discarded by the offline profile; no guest bytes are fabricated.
        if length as u32
            > if socket.family == AF_INET6 {
                IPV6_MAX_UDP_PAYLOAD
            } else {
                IPV4_MAX_UDP_PAYLOAD
            }
        {
            return Err(WSAEMSGSIZE);
        }
        Ok(Some(length as u64))
    })();
    match result {
        Ok(None) => policy::unsupported(emu, "ws2_32", api, emu.pc()),
        Ok(Some(value)) => return_result(emu, 4, Ok(value), false),
        Err(error) => return_result(emu, 4, Err(error), false),
    }
}

fn convert_byte_order(emu: &mut Emu, api: &str, short: bool) {
    let call = WinApi(Stdcall);
    let socket = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u32(emu, 1));
    let result = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ws2_32!{api} socket={socket:#x} value={value:#x} result_ptr={result:#x}"
    );
    let status = if !emu.sockets.contains(socket) {
        Err(WSAENOTSOCK)
    } else if result == 0 {
        Err(WSAEFAULT)
    } else {
        let written = if short {
            emu.maps.write_word(result, (value as u16).swap_bytes())
        } else {
            emu.maps.write_dword(result, value.swap_bytes())
        };
        written.then_some(0).ok_or(WSAEFAULT)
    };
    return_result(emu, 3, status, false);
}

pub fn WSAHtons(emu: &mut Emu) {
    convert_byte_order(emu, "WSAHtons", true);
}

pub fn WSANtohs(emu: &mut Emu) {
    convert_byte_order(emu, "WSANtohs", true);
}

pub fn WSAHtonl(emu: &mut Emu) {
    convert_byte_order(emu, "WSAHtonl", false);
}

pub fn WSANtohl(emu: &mut Emu) {
    convert_byte_order(emu, "WSANtohl", false);
}

fn datagram(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [socket, buffer, length, flags, address, address_length] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "ws2_32!{api} socket={socket:#x} buffer={buffer:#x} length={length} flags={flags:#x} address={address:#x} address_length={address_length:#x}",
        length = length as i32
    );
    if !emu.sockets.contains(socket) {
        return_result(emu, 6, Err(WSAENOTSOCK), false);
    } else if (length as i32) < 0 {
        return_result(emu, 6, Err(WSAEINVAL), false);
    } else {
        policy::unsupported(emu, "ws2_32", api, emu.pc());
    }
}

pub fn sendto(emu: &mut Emu) {
    datagram(emu, "sendto");
}

pub fn recvfrom(emu: &mut Emu) {
    datagram(emu, "recvfrom");
}

fn wsa_endpoint(emu: &mut Emu, api: &str, wide: bool, parse: bool) {
    let call = WinApi(Stdcall);
    let [source, size_or_family, provider, output, length] = api_try!(emu, call.args(emu));
    if provider != 0 {
        policy::unsupported(emu, "ws2_32", api, emu.pc());
        return;
    }
    let result = (|| {
        let (bytes, units) = if parse {
            let text = strings::read_text_checked(emu, source, wide).ok_or(WSAEFAULT)?;
            let endpoint = text
                .parse::<SocketAddr>()
                .or_else(|_| text.parse::<IpAddr>().map(|ip| SocketAddr::new(ip, 0)))
                .map_err(|_| WSAEINVAL)?;
            let family = size_or_family as u32;
            let size = match (family, endpoint) {
                (AF_INET, SocketAddr::V4(_)) => SOCKADDR_IN_SIZE,
                (AF_INET6, SocketAddr::V6(_)) => SOCKADDR_IN6_SIZE,
                (AF_INET | AF_INET6, _) => return Err(WSAEINVAL),
                _ => return Err(WSAEAFNOSUPPORT),
            };
            let mut bytes = vec![0; size];
            bytes[..2].copy_from_slice(&(family as u16).to_le_bytes());
            bytes[2..4].copy_from_slice(&endpoint.port().to_be_bytes());
            match endpoint {
                SocketAddr::V4(v4) => bytes[4..8].copy_from_slice(&v4.ip().octets()),
                SocketAddr::V6(v6) => {
                    bytes[8..24].copy_from_slice(&v6.ip().octets());
                    bytes[24..28].copy_from_slice(&v6.scope_id().to_le_bytes());
                }
            }
            (bytes, size)
        } else {
            let family = emu.maps.read_word(source).ok_or(WSAEFAULT)? as u32;
            if !matches!(family, AF_INET | AF_INET6) {
                return Err(WSAEAFNOSUPPORT);
            }
            let bytes = read_address(emu, source, size_or_family as i32, family)?;
            let port = u16::from_be_bytes([bytes[2], bytes[3]]);
            let text = if family == AF_INET {
                let ip = Ipv4Addr::from(<[u8; 4]>::try_from(&bytes[4..8]).map_err(|_| WSAEFAULT)?);
                if port == 0 {
                    ip.to_string()
                } else {
                    SocketAddr::new(ip.into(), port).to_string()
                }
            } else {
                let ip =
                    Ipv6Addr::from(<[u8; 16]>::try_from(&bytes[8..24]).map_err(|_| WSAEFAULT)?);
                let scope = u32::from_le_bytes(bytes[24..28].try_into().map_err(|_| WSAEFAULT)?);
                let addr = SocketAddrV6::new(ip, port, 0, scope);
                if port != 0 {
                    addr.to_string()
                } else if scope == 0 {
                    ip.to_string()
                } else {
                    format!("{ip}%{scope}")
                }
            };
            let units = text.len() + 1;
            (strings::encode_text(&format!("{text}\0"), wide), units)
        };
        let capacity = emu.maps.read_dword(length).ok_or(WSAEFAULT)?;
        if !emu.maps.validate_write_range(length, 4) {
            return Err(WSAEFAULT);
        }
        if output == 0 || (capacity as usize) < units {
            emu.maps.write_dword(length, units as u32);
            return Err(WSAEFAULT);
        }
        if !emu.maps.write_bytes(output, &bytes) {
            return Err(WSAEFAULT);
        }
        emu.maps.write_dword(length, units as u32);
        Ok(0)
    })();
    log_red!(
        emu,
        "ws2_32!{api} input={source:#x} family_or_length={size_or_family} output={output:#x} output_length={length:#x} result={result:?}"
    );
    return_result(emu, 5, result, false);
}

pub fn WSAAddressToStringA(emu: &mut Emu) {
    wsa_endpoint(emu, "WSAAddressToStringA", false, false);
}
pub fn WSAAddressToStringW(emu: &mut Emu) {
    wsa_endpoint(emu, "WSAAddressToStringW", true, false);
}
pub fn WSAStringToAddressA(emu: &mut Emu) {
    wsa_endpoint(emu, "WSAStringToAddressA", false, true);
}
pub fn WSAStringToAddressW(emu: &mut Emu) {
    wsa_endpoint(emu, "WSAStringToAddressW", true, true);
}

pub fn WSACleanup(emu: &mut Emu) {
    let result = if emu.sockets.startups == 0 {
        Err(WSANOTINITIALISED)
    } else {
        emu.sockets.startups -= 1;
        if emu.sockets.startups == 0 {
            emu.sockets.sockets.clear();
        }
        Ok(0)
    };
    log_red!(emu, "ws2_32!WSACleanup result={result:?}");
    return_result(emu, 0, result, false);
}
