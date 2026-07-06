use crate::emu;
//use crate::endpoint;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

use lazy_static::lazy_static;
use std::sync::Mutex;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "WsaStartup" => WsaStartup(emu),
        "WsaSocketA" => WsaSocketA(emu),
        "socket" => socket(emu),
        "WsaHtons" => WsaHtons(emu),
        "htons" => htons(emu),
        "inet_addr" => inet_addr(emu),
        "connect" => connect(emu),
        "recv" => recv(emu),
        "send" => send(emu),
        "bind" => bind(emu),
        "listen" => listen(emu),
        "accept" => accept(emu),
        "closesocket" => closesocket(emu),
        "setsockopt" => setsockopt(emu),
        "getsockopt" => getsockopt(emu),
        "WsaAccept" => WsaAccept(emu),
        "getaddrinfo" => getaddrinfo(emu),

        /*
        0x774834b5 => sendto(emu),
        0x7748b6dc => recvfrom(emu),
        0x77487089 => WsaRecv(emu),
        0x7748cba6 => WsaRecvFrom(emu),
        0x7748cc3f => WsaConnect(emu),
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
    let node_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!getaddrinfo cannot read node_name_ptr");
    let service_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!getaddrinfo cannot read service_name_ptr");
    let hints_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!getaddrinfo cannot read hints_ptr");
    let result_ptr_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ws2_32!getaddrinfo cannot read result_ptr_ptr");

    let node_name = if node_name_ptr != 0 {
        emu.maps.read_string(node_name_ptr as u64)
    } else {
        "NULL".to_string()
    };

    let service_name = if service_name_ptr != 0 {
        emu.maps.read_string(service_name_ptr as u64)
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
        hints_flags = emu.maps.read_dword(hints_ptr as u64).unwrap_or(0) as i32;
        hints_family = emu.maps.read_dword((hints_ptr + 4) as u64).unwrap_or(0) as i32;
        hints_socktype = emu.maps.read_dword((hints_ptr + 8) as u64).unwrap_or(0) as i32;
        hints_protocol = emu.maps.read_dword((hints_ptr + 12) as u64).unwrap_or(0) as i32;
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
    emu.maps.write_qword(result_ptr_ptr as u64, addrinfo_addr);

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

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

fn WsaSocketA(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!WsaSocketA");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}

fn socket(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!socket");

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}

fn WsaHtons(emu: &mut emu::Emu) {
    let host_port = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!WsaHtons cannot read host_port");
    let out_port = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!WsaHtons cannot read out_port");

    log_red!(emu, "ws2_32!WsaHtons {}", host_port);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    //TODO: implement this

    emu.regs_mut().rax = 0;
}

fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.maps.read_word(emu.regs().get_esp()).unwrap_or_default();

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.stack_pop32(false);
    emu.regs_mut().rax = port.to_be() as u64;
}

fn inet_addr(emu: &mut emu::Emu) {
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!inet_addr: error reading addr");

    log_red!(emu, "ws2_32!inet_addr");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn connect(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!connect: error reading sock") as u64;
    let sockaddr_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!connect: error reading sockaddr ptr") as u64;
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

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

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
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!recv: error reading sock") as u64;
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!recv: error reading buff") as u64;
    let mut len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!recv: error reading len") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!recv: error reading flags") as u64;

    log_red!(emu, "ws2_32!recv   buff: 0x{:x} sz: {}", buff, len);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

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
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!send: error reading sock") as u64;
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!send: error reading buff") as u64;
    let mut len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!send: error reading len") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!send: error reading flags") as u64;

    let bytes = emu.maps.read_string_of_bytes(buff, len as usize);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

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
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!send: error reading sock") as u64;
    let saddr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!send: error reading addr") as u64;
    let len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!send: error reading len") as u64;

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

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tbad socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn listen(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!send: error reading sock") as u64;
    let connections = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!send: error reading num of connections") as u64;

    log_red!(emu, "ws2_32!listen  connections: {}", connections);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn accept(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!accept: error reading sock") as u64;
    let saddr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!accept: error reading sockaddr") as u64;
    let len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!seacceptnd: error reading len") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!accept: error reading flags") as u64;

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(emu, "ws2_32!accept  connections: {}", bytes);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn closesocket(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!send: error reading sock") as u64;

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

fn setsockopt(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!setsockopt: error reading sock") as u64;
    let level = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!setsockopt: error reading level") as u64;
    let optname = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!setsockopt: error reading optname") as u64;
    let optval = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!setsockopt: error reading optval") as u64;
    let optlen = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ws2_32!setsockopt: error reading optlen") as u64;

    let val = emu.maps.read_dword(optval).unwrap_or_default();

    log_red!(
        emu,
        "ws2_32!setsockopt  lvl: {} opt: {} val: {}",
        level,
        optname,
        val
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn getsockopt(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!getsockopt: error reading sock") as u64;
    let level = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!getsockopt: error reading level") as u64;
    let optname = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!getsockopt: error reading optname") as u64;
    let optval = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!getsockopt: error reading optval") as u64;
    let optlen = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ws2_32!getsockopt: error reading optlen") as u64;

    emu.maps.write_dword(optval, 1);

    log_red!(emu, "ws2_32!getsockopt  lvl: {} opt: {}", level, optname);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn WsaAccept(emu: &mut emu::Emu) {
    let sock = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ws2_32!WsaAccept: error reading sock") as u64;
    let saddr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ws2_32!WsaAccept: error reading sockaddr") as u64;
    let len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ws2_32!WsaAccept: error reading len") as u64;
    let cond = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ws2_32!WsaAccept: error reading cond") as u64;
    let callback = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ws2_32!WsaAccept: error reading callback") as u64;

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(
        emu,
        "ws2_32!WsaAccept  connections: {} callback: {}",
        bytes,
        callback
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
