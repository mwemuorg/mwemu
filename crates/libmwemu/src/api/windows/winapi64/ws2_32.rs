use crate::emu;
use crate::serialization;
//use crate::endpoint;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::windows::structures::*;

use crate::maps::mem64::Permission;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "WSAStartup" => WsaStartup(emu),
        "WSASocketA" => WsaSocketA(emu),
        "connect" => connect(emu),
        "recv" => recv(emu),
        "send" => send(emu),
        "socket" => socket(emu),
        "WsaHtons" => WsaHtons(emu),
        "htons" => htons(emu),
        "inet_addr" => inet_addr(emu),
        "bind" => bind(emu),
        "listen" => listen(emu),
        "accept" => accept(emu),
        "closesocket" => closesocket(emu),
        "setsockopt" => setsockopt(emu),
        "getsockopt" => getsockopt(emu),
        "WsaAccept" => WsaAccept(emu),
        "GetSockName" => GetSockName(emu),
        "gethostbyname" => gethostbyname(emu),
        "getaddrinfo" => getaddrinfo(emu),
        /*
        "sendto" => sendto(emu),
        "recvfrom" => recvfrom(emu),
        "WsaRecv" => WsaRecv(emu),
        "WsaRecvFrom" => WsaRecvFrom(emu),
        "WsaConnect" => WsaConnect(emu),
        */
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

lazy_static! {
    static ref COUNT_SEND: Mutex<u32> = Mutex::new(0);
    static ref COUNT_RECV: Mutex<u32> = Mutex::new(0);
}

fn getaddrinfo(emu: &mut emu::Emu) {
    let node_name_ptr = emu.regs().rcx;
    let service_name_ptr = emu.regs().rdx;
    let hints_ptr = emu.regs().r8;
    let result_ptr_ptr = emu.regs().r9;

    let node_name = if node_name_ptr != 0 {
        emu.maps.read_string(node_name_ptr)
    } else {
        "NULL".to_string()
    };

    let service_name = if service_name_ptr != 0 {
        emu.maps.read_string(service_name_ptr)
    } else {
        "NULL".to_string()
    };

    log_red!(
        emu,
        "ws2_32!getaddrinfo node: `{}` service: `{}`",
        node_name,
        service_name
    );

    // Read hints if provided
    let mut hints_flags = 0;
    let mut hints_family = 0;
    let mut hints_socktype = 0;
    let mut hints_protocol = 0;

    if hints_ptr != 0 {
        hints_flags = emu.maps.read_dword(hints_ptr).unwrap_or(0) as i32;
        hints_family = emu.maps.read_dword(hints_ptr + 4).unwrap_or(0) as i32;
        hints_socktype = emu.maps.read_dword(hints_ptr + 8).unwrap_or(0) as i32;
        hints_protocol = emu.maps.read_dword(hints_ptr + 12).unwrap_or(0) as i32;
    }

    // Create a dummy ADDRINFO structure
    let addrinfo_size = 48; // Size of ADDRINFOA structure (approximate)
    let sockaddr_in_size = 16; // Size of sockaddr_in structure

    // Allocate memory for the result
    let heap_management = emu.heap_mut();
    let addrinfo_addr = heap_management
        .allocate((addrinfo_size + sockaddr_in_size + 100) as usize)
        .unwrap();
    let sockaddr_addr = addrinfo_addr + addrinfo_size;
    let canonname_addr = sockaddr_addr + sockaddr_in_size;

    // Create a dummy sockaddr_in (IPv4 address 127.0.0.1, port based on service)
    let ip_addr = 0x0100007f; // 127.0.0.1 in network byte order

    // Determine port based on service name
    let port = if service_name == "http" || service_name == "80" {
        80u16
    } else if service_name == "https" || service_name == "443" {
        443u16
    } else if service_name == "ftp" || service_name == "21" {
        21u16
    } else if service_name == "ssh" || service_name == "22" {
        22u16
    } else if service_name == "smtp" || service_name == "25" {
        25u16
    } else if service_name == "dns" || service_name == "53" {
        53u16
    } else {
        service_name.parse().unwrap_or(80u16)
    };

    // Write sockaddr_in structure
    emu.maps.write_word(sockaddr_addr, 2); // AF_INET = 2
    emu.maps.write_word(sockaddr_addr + 2, port.to_be()); // Port in network byte order
    emu.maps.write_dword(sockaddr_addr + 4, ip_addr); // IP address (127.0.0.1)
    emu.maps.memset(sockaddr_addr + 8, 0, 8); // Zero out the rest

    // Write ADDRINFO structure
    emu.maps.write_dword(addrinfo_addr, hints_flags as u32); // ai_flags
    emu.maps.write_dword(
        addrinfo_addr + 4,
        if hints_family != 0 {
            hints_family as u32
        } else {
            2
        },
    ); // ai_family (AF_INET)
    emu.maps.write_dword(
        addrinfo_addr + 8,
        if hints_socktype != 0 {
            hints_socktype as u32
        } else {
            1
        },
    ); // ai_socktype (SOCK_STREAM)
    emu.maps.write_dword(
        addrinfo_addr + 12,
        if hints_protocol != 0 {
            hints_protocol as u32
        } else {
            6
        },
    ); // ai_protocol (IPPROTO_TCP)
    emu.maps
        .write_qword(addrinfo_addr + 16, sockaddr_in_size); // ai_addrlen
    emu.maps.write_qword(addrinfo_addr + 24, canonname_addr); // ai_canonname
    emu.maps.write_qword(addrinfo_addr + 32, sockaddr_addr); // ai_addr

    // Set ai_canonname to the node name or "localhost"
    let canon_name = if node_name == "NULL" || node_name.is_empty() || node_name == "localhost" {
        "localhost.localdomain".to_string()
    } else if node_name == "127.0.0.1" {
        "localhost".to_string()
    } else {
        format!("{}.localdomain", node_name)
    };
    emu.maps.write_string(canonname_addr, &canon_name);

    // ai_next is NULL (end of list)
    emu.maps.write_qword(addrinfo_addr + 40, 0);

    // Store the result pointer in the ppResult parameter
    emu.maps.write_qword(result_ptr_ptr, addrinfo_addr);

    log::trace!(
        "\tcreated dummy ADDRINFO for {}:{} at 0x{:x}",
        node_name,
        service_name,
        addrinfo_addr
    );
    log::trace!(
        "\tsockaddr at 0x{:x}, canonname at 0x{:x}",
        sockaddr_addr,
        canonname_addr
    );

    // Return 0 for success (WSA error code)
    emu.regs_mut().rax = 0;
}

fn WsaStartup(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!WsaStartup");

    emu.regs_mut().rax = 0;
}

fn WsaSocketA(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!WsaSocketA");

    emu.regs_mut().rax = helper::socket_create();
}

fn socket(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!socket");

    emu.regs_mut().rax = helper::socket_create();
}

fn WsaHtons(emu: &mut emu::Emu) {
    let host_port = emu.regs().rdx;
    let out_port = emu.regs().r8;

    log_red!(emu, "ws2_32!WsaHtons {}", host_port);

    //TODO: implement this
    emu.regs_mut().rax = 0;
}

fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.regs().rcx as u16;

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.regs_mut().rax = port.to_be() as u64;
}

fn inet_addr(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;

    //TODO: derreferece addr

    log_red!(emu, "ws2_32!inet_addr");

    emu.regs_mut().rax = 0;
}

fn connect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let sockaddr_ptr = emu.regs().rdx;
    //let sockaddr = emu.maps.read_bytes(sockaddr_ptr, 8);
    let family: u16 = emu
        .maps
        .read_word(sockaddr_ptr)
        .expect("ws2_32!connect: error reading family");
    let port: u16 = emu
        .maps
        .read_word(sockaddr_ptr + 2)
        .expect("ws2_32!connect: error reading port")
        .to_be();
    let ip: u32 = emu
        .maps
        .read_dword(sockaddr_ptr + 4)
        .expect("ws2_32!connect: error reading ip");

    let sip = format!(
        "{}.{}.{}.{}",
        ip & 0xff,
        (ip & 0xff00) >> 8,
        (ip & 0xff0000) >> 16,
        (ip & 0xff000000) >> 24
    );
    log_red!(emu, "ws2_32!connect  family: {} {}:{}", family, sip, port);

    if emu.cfg.endpoint {
        /*
        if endpoint::sock_connect(sip.as_str(), port) {
            log::trace!("\tconnected to the endpoint.");
        } else {
            log::trace!("\tcannot connect. dont use -e");
        }*/
        emu.regs_mut().rax = 0;
    } else {
        // offline mode

        if !helper::socket_exist(sock) {
            log::trace!("\tinvalid socket.");
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    }
}

fn recv(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let mut len = emu.regs().r8;
    let flags = emu.regs().r9;

    log_red!(emu, "ws2_32!recv   buff: 0x{:x} sz: {}", buff, len);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
        return;
    }

    if emu.cfg.endpoint {
        /*
        let mut rbuff: Vec<u8> = vec![0; len as usize];
        let n = endpoint::sock_recv(&mut rbuff);

        emu.maps.write_buffer(buff, &rbuff);

        log::trace!("\nreceived {} bytes from the endpoint.", n);
        emu.regs_mut().rax = n as u64;
        */
    } else {
        let mut count_recv = COUNT_RECV.lock().unwrap();
        *count_recv += 1;
        if *count_recv > 3 {
            len = 0; // finish the recv loop
        }

        if helper::socket_exist(sock) {
            //emu.maps.write_spaced_bytes(buff, "6c 73 0d 0a".to_string()); // send a ls\r\n
            if len == 4 {
                emu.maps.write_dword(buff, 0x0100); // probably expect a size
            } else {
                if emu.maps.overflow_predicted(buff, len) {
                    if emu.cfg.verbose > 0 {
                        log::trace!(
                            "/!\\ on this asm, the recv overflows the buffer, canceled the write!"
                        );
                    }
                } else {
                    emu.maps.memset(buff, 0x90, len as usize);
                }
            }

            emu.regs_mut().rax = len;
        }
    }
}

fn send(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let mut len = emu.regs().r8;
    let flags = emu.regs().r9;

    let bytes = emu.maps.read_string_of_bytes(buff, len as usize);

    log_red!(emu, "ws2_32!send {{{}}}", bytes);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 0;
        return;
    }

    if emu.cfg.endpoint {
        /*
        let buffer = emu.maps.read_buffer(buff, len as usize);
        let n = endpoint::sock_send(&buffer);
        log::trace!("\tsent {} bytes.", n);
        emu.regs_mut().rax = n as u64;
        */
    } else {
        let mut count_send = COUNT_SEND.lock().unwrap();
        *count_send += 1;
        if *count_send > 3 {
            len = 0; // finish the send loop
        }

        emu.regs_mut().rax = len;
    }
}

fn bind(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let len = emu.regs().r8;

    let family: u16 = emu
        .maps
        .read_word(saddr)
        .expect("ws2_32!connect: error reading family");
    let port: u16 = emu
        .maps
        .read_word(saddr + 2)
        .expect("ws2_32!connect: error reading port");
    let ip: u32 = emu
        .maps
        .read_dword(saddr + 4)
        .expect("ws2_32!connect: error reading ip");

    let sip = format!(
        "{}.{}.{}.{}",
        ip & 0xff,
        (ip & 0xff00) >> 8,
        (ip & 0xff0000) >> 16,
        (ip & 0xff000000) >> 24
    );

    log_red!(
        emu,
        "ws2_32!bind  family: {} {}:{}",
        family,
        sip,
        port.to_be()
    );

    if !helper::socket_exist(sock) {
        log::trace!("\tbad socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn listen(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let connections = emu.regs().rdx;

    log_red!(emu, "ws2_32!listen  connections: {}", connections);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn accept(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let len = emu.regs().r8;
    let flags = emu.regs().r9;

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(emu, "ws2_32!accept  connections: {}", bytes);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn closesocket(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.regs_mut().rax = 0;
}

fn setsockopt(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let level = emu.regs().rdx;
    let optname = emu.regs().r8;
    let optval = emu.regs().r9;
    let optlen = emu
        .maps
        .read_qword(emu.regs().get_esp())
        .expect("ws2_32!setsockopt: error reading optlen");

    let val = emu.maps.read_dword(optval).unwrap_or_default();

    log_red!(
        emu,
        "ws2_32!setsockopt  lvl: {} opt: {} val: {}",
        level,
        optname,
        val
    );

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn getsockopt(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let level = emu.regs().rdx;
    let optname = emu.regs().r8;
    let optval = emu.regs().r9;
    let optlen = emu
        .maps
        .read_qword(emu.regs().get_esp())
        .expect("ws2_32!getsockopt: error reading optlen");

    emu.maps.write_dword(optval, 1);

    log_red!(emu, "ws2_32!getsockopt  lvl: {} opt: {}", level, optname);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn WsaAccept(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let len = emu.regs().r8;
    let cond = emu.regs().r9;
    let callback = emu
        .maps
        .read_qword(emu.regs().get_esp())
        .expect("ws2_32!WsaAccept: error reading callback");

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(
        emu,
        "ws2_32!WsaAccept  connections: {} callback: {}",
        bytes,
        callback
    );

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn GetSockName(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let sockaddr_ptr = emu.regs().rdx;
    let namelen_ptr = emu.regs().r8;

    emu.maps.write_dword(sockaddr_ptr, 0);
    emu.maps.write_dword(namelen_ptr, 4);

    log_red!(emu, "ws2_32!GetSockName sock: {}", sock);

    emu.regs_mut().rax = 0;
}

fn gethostbyname(emu: &mut emu::Emu) {
    let domain_name_ptr = emu.regs().rcx;
    let domain_name = emu.maps.read_string(domain_name_ptr);

    log_red!(emu, "ws2_32!gethostbyname `{}`", domain_name);

    let addr = emu.maps.alloc(1024).expect("low memory");
    let str_addr = addr + 1024 - 100;
    let mem = emu
        .maps
        .create_map("hostent", addr, 1024, Permission::READ_WRITE)
        .expect("cannot create hostent map");
    mem.write_dword(addr, 0x04030201);
    mem.write_qword(addr + 8, addr);
    mem.write_qword(addr + 16, 0);
    mem.write_string(str_addr, &domain_name);

    let mut hostent = Hostent::new();
    hostent.hname = str_addr;
    hostent.alias_list = 0;
    hostent.length = 4;
    hostent.addr_list = addr + 8;
    hostent.save(addr + 30, &mut emu.maps);

    emu.regs_mut().rax = addr + 30;
}
