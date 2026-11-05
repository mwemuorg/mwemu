use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::structures::Hostent;
//use crate::endpoint;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;

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
        "gethostbyname" => gethostbyname(emu),
        "sendto" => sendto(emu),
        "recvfrom" => recvfrom(emu),
        "WsaConnect" => WsaConnect(emu),
        "WsaRecv" => WsaRecv(emu),
        "WsaRecvFrom" => WsaRecvFrom(emu),
        "GetSockName" => GetSockName(emu),
        "htonl" => htonl(emu),
        "ntohl" => ntohl(emu),
        "ntohs" => ntohs(emu),
        "select" => select(emu),
        "ioctlsocket" => ioctlsocket(emu),
        "shutdown" => shutdown(emu),
        "getpeername" => getpeername(emu),
        "WSAGetLastError" => WsaGetLastError(emu),
        "WSASetLastError" => WsaSetLastError(emu),
        "FreeAddrInfoEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeAddrInfoExW" => {
            let info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
            helper::handler_close(info_ptr);
        }
        "FreeAddrInfoW" => {
            let info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!FreeAddrInfoW ptr:0x{:x}", info_ptr);
            if info_ptr != 0 {
                let map_name = format!("addrinfo_{:x}", info_ptr);
                emu.maps.free(&map_name);
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAddrInfoExA" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let namespace = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            let service = if service_ptr != 0 {
                emu.maps.read_string(service_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!GetAddrInfoExA name:'{}' service:'{}' ns:{}",
                name,
                service,
                namespace
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAddrInfoExCancel" => {
            let cancel_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!GetAddrInfoExCancel ptr:0x{:x}", cancel_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAddrInfoExOverlappedResult" => {
            let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!GetAddrInfoExOverlappedResult overlapped:0x{:x}",
                overlapped_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAddrInfoExW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let namespace = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let name = if name_ptr != 0 {
                emu.maps.read_wide_string(name_ptr)
            } else {
                String::new()
            };
            let service = if service_ptr != 0 {
                emu.maps.read_wide_string(service_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!GetAddrInfoExW name:'{}' service:'{}' ns:{}",
                name,
                service,
                namespace
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAddrInfoW" => {
            let node_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hints_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let result_ptr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let node = if node_ptr != 0 {
                emu.maps.read_wide_string(node_ptr)
            } else {
                String::new()
            };
            let service = if service_ptr != 0 {
                emu.maps.read_wide_string(service_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!GetAddrInfoW node:'{}' service:'{}'",
                node,
                service
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "GetHostNameW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let namelen = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!GetHostNameW buf:0x{:x} len:{}",
                name_ptr,
                namelen
            );
            if name_ptr != 0 && namelen > 8 {
                emu.maps.write_wide_string(name_ptr, "DESKTOP");
            }
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "GetNameInfoW" => {
            let sa_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let salen = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let host_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let hostlen = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!GetNameInfoW sa:0x{:x} host:0x{:x} hostlen:{}",
                sa_ptr,
                host_ptr,
                hostlen
            );
            if host_ptr != 0 && hostlen > 10 {
                emu.maps.write_wide_string(host_ptr, "localhost");
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InetNtopW" => {
            let family = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let str_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let str_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!InetNtopW family:{} addr:0x{:x} buf:0x{:x}",
                family,
                addr_ptr,
                str_buf
            );
            if family == 2 && addr_ptr != 0 && str_buf != 0 {
                let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
                let a = ip & 0xFF;
                let b = (ip >> 8) & 0xFF;
                let c = (ip >> 16) & 0xFF;
                let d = (ip >> 24) & 0xFF;
                let s = format!("{}.{}.{}.{}", a, b, c, d);
                if (s.len() as u32) < str_len {
                    emu.maps.write_wide_string(str_buf, &s);
                    emu.regs_mut().rax = str_buf;
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "InetPtonW" => {
            let family = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let src_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let src_str = if src_ptr != 0 {
                emu.maps.read_wide_string(src_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!InetPtonW family:{} src:'{}'", family, src_str);
            if family == 2 && dst_ptr != 0 {
                let parts: Vec<&str> = src_str.split('.').collect();
                if parts.len() == 4 {
                    if let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                        parts[0].parse::<u8>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                        parts[3].parse::<u8>(),
                    ) {
                        let addr = (a as u32)
                            | ((b as u32) << 8)
                            | ((c as u32) << 16)
                            | ((d as u32) << 24);
                        emu.maps.write_dword(dst_ptr, addr);
                        emu.regs_mut().rax = 1;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "ProcessSocketNotifications" => {
            let completion_port = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let count = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
                completion_port,
                count
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "SetAddrInfoExA" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!SetAddrInfoExA name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetAddrInfoExW" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_wide_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!SetAddrInfoExW name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WEP" => {
            let reason = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ws2_32!WEP reason:{}", reason);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WPUCompleteOverlappedRequest" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let error = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let bytes = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
                sock,
                error,
                bytes
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WPUGetProviderPathEx" => {
            let provider_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let path_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let path_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(emu, "ws2_32!WPUGetProviderPathEx flags:{}", flags);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAccept" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let addrlen_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let condition_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAAccept sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                let new_sock = helper::socket_create();
                if addr_ptr != 0 {
                    emu.maps.write_word(addr_ptr, 2);
                    emu.maps.write_word(addr_ptr + 2, 0x5000);
                    emu.maps.write_dword(addr_ptr + 4, 0x0100007f);
                }
                if addrlen_ptr != 0 {
                    emu.maps.write_dword(addrlen_ptr, 16);
                }
                emu.regs_mut().rax = new_sock;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAddressToStringA" => {
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAAddressToStringA addr:0x{:x}", addr_ptr);
            if addr_ptr != 0 && str_ptr != 0 {
                let ip = emu.maps.read_dword(addr_ptr + 4).unwrap_or(0x0100007f);
                let port = emu.maps.read_word(addr_ptr + 2).unwrap_or(0).to_be();
                let s = format!(
                    "{}.{}.{}.{}:{}",
                    ip & 0xFF,
                    (ip >> 8) & 0xFF,
                    (ip >> 16) & 0xFF,
                    (ip >> 24) & 0xFF,
                    port
                );
                emu.maps.write_string(str_ptr, &s);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAddressToStringW" => {
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAAddressToStringW addr:0x{:x}", addr_ptr);
            if addr_ptr != 0 && str_ptr != 0 {
                let ip = emu.maps.read_dword(addr_ptr + 4).unwrap_or(0x0100007f);
                let port = emu.maps.read_word(addr_ptr + 2).unwrap_or(0).to_be();
                let s = format!(
                    "{}.{}.{}.{}:{}",
                    ip & 0xFF,
                    (ip >> 8) & 0xFF,
                    (ip >> 16) & 0xFF,
                    (ip >> 24) & 0xFF,
                    port
                );
                emu.maps.write_wide_string(str_ptr, &s);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAdvertiseProvider" => {
            let provider_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let routine_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAAdvertiseProvider id:0x{:x} routine:0x{:x}",
                provider_id_ptr,
                routine_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetHostByAddr" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetHostByAddr hwnd:0x{:x} msg:{}",
                hwnd,
                msg
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetHostByName" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetHostByName hwnd:0x{:x} name:'{}'",
                hwnd,
                name
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetProtoByName" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetProtoByName hwnd:0x{:x} name:'{}'",
                hwnd,
                name
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetProtoByNumber" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let number = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetProtoByNumber hwnd:0x{:x} number:{}",
                hwnd,
                number
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetServByName" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetServByName hwnd:0x{:x} name:'{}'",
                hwnd,
                name
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncGetServByPort" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let port = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetServByPort hwnd:0x{:x} port:{}",
                hwnd,
                port
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAAsyncSelect" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hwnd = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let events = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAAsyncSelect sock:0x{:x} hwnd:0x{:x} msg:{} events:0x{:x}",
                sock,
                hwnd,
                msg,
                events
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSACancelAsyncRequest" => {
            let async_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSACancelAsyncRequest handle:0x{:x}",
                async_handle
            );
            if helper::handler_exist(async_handle) {
                helper::handler_close(async_handle);
                emu.regs_mut().rax = 0;
            } else {
                emu.set_last_error(10044);
                emu.regs_mut().rax = 0xFFFFFFFF;
            }
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSACancelBlockingCall" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ws2_32!WSACancelBlockingCall 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSACleanup" => {
            log_red!(emu, "ws2_32!WSACleanup");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSACloseEvent" => {
            let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSACloseEvent event:0x{:x}", event);
            if event != 0 {
                helper::handler_close(event);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAConnect" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let namelen = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(emu, "ws2_32!WSAConnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAConnectByList" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let addr_list_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAConnectByList sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 1;
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAConnectByNameA" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let node_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let node = if node_ptr != 0 {
                emu.maps.read_string(node_ptr)
            } else {
                String::new()
            };
            let service = if service_ptr != 0 {
                emu.maps.read_string(service_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAConnectByNameA sock:0x{:x} node:'{}' service:'{}'",
                sock,
                node,
                service
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 1;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAConnectByNameW" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let node_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let node = if node_ptr != 0 {
                emu.maps.read_wide_string(node_ptr)
            } else {
                String::new()
            };
            let service = if service_ptr != 0 {
                emu.maps.read_wide_string(service_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAConnectByNameW sock:0x{:x} node:'{}' service:'{}'",
                sock,
                node,
                service
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 1;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSACreateEvent" => {
            let event = helper::handler_create("wsa_event");
            log_red!(emu, "ws2_32!WSACreateEvent -> 0x{:x}", event);
            emu.regs_mut().rax = event;
        }
        "WSADuplicateSocketA" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let process_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSADuplicateSocketA sock:0x{:x} pid:{}",
                sock,
                process_id
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSADuplicateSocketW" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let process_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSADuplicateSocketW sock:0x{:x} pid:{}",
                sock,
                process_id
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumNameSpaceProvidersA" => {
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersA");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumNameSpaceProvidersExA" => {
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExA");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumNameSpaceProvidersExW" => {
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExW");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumNameSpaceProvidersW" => {
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersW");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumNetworkEvents" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let event = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let net_events_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAEnumNetworkEvents sock:0x{:x} event:0x{:x}",
                sock,
                event
            );
            if net_events_ptr != 0 {
                for i in 0..44u64 {
                    emu.maps.write_byte(net_events_ptr + i, 0);
                }
                emu.maps.write_dword(net_events_ptr, 0x01 | 0x02 | 0x10);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumProtocolsA" => {
            let protocols_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumProtocolsA");
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEnumProtocolsW" => {
            let protocols_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAEnumProtocolsW");
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAEventSelect" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let event = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let events = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAEventSelect sock:0x{:x} event:0x{:x} events:0x{:x}",
                sock,
                event,
                events
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetOverlappedResult" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let transfer_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let wait = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAGetOverlappedResult sock:0x{:x} wait:{}",
                sock,
                wait
            );
            if transfer_ptr != 0 {
                emu.maps.write_dword(transfer_ptr, 0);
            }
            emu.regs_mut().rax = 1;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetQOSByName" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let qos_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetServiceClassInfoA" => {
            let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAGetServiceClassInfoA");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetServiceClassInfoW" => {
            let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAGetServiceClassInfoW");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetServiceClassNameByClassIdA" => {
            let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdA");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAGetServiceClassNameByClassIdW" => {
            let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdW");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAHtonl" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hostlong = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAHtonl sock:0x{:x} val:{}", sock, hostlong);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if result_ptr != 0 {
                    emu.maps.write_dword(result_ptr, hostlong.to_be());
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAHtons" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hostshort = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u16;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAHtons sock:0x{:x} val:{}", sock, hostshort);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if result_ptr != 0 {
                    emu.maps.write_word(result_ptr, hostshort.to_be());
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAInstallServiceClassA" => {
            let class_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAInstallServiceClassA ptr:0x{:x}",
                class_info_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAInstallServiceClassW" => {
            let class_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAInstallServiceClassW ptr:0x{:x}",
                class_info_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAIoctl" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let ioctl_code = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let in_buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let in_buf_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAIoctl sock:0x{:x} code:0x{:x}",
                sock,
                ioctl_code
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAIsBlocking" => {
            log_red!(emu, "ws2_32!WSAIsBlocking");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAJoinLeaf" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let namelen = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = sock;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSALookupServiceBeginA" => {
            let restrictions_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSALookupServiceBeginA flags:0x{:x}", flags);
            let handle = helper::handler_create("wsa_lookup");
            if handle_ptr != 0 {
                emu.maps.write_dword(handle_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSALookupServiceBeginW" => {
            let restrictions_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSALookupServiceBeginW flags:0x{:x}", flags);
            let handle = helper::handler_create("wsa_lookup");
            if handle_ptr != 0 {
                emu.maps.write_dword(handle_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSALookupServiceEnd" => {
            let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSALookupServiceEnd handle:0x{:x}",
                lookup_handle
            );
            if helper::handler_exist(lookup_handle) {
                helper::handler_close(lookup_handle);
                emu.regs_mut().rax = 0;
            } else {
                emu.set_last_error(6);
                emu.regs_mut().rax = 0xFFFFFFFF;
            }
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSALookupServiceNextA" => {
            let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSALookupServiceNextA handle:0x{:x}",
                lookup_handle
            );
            emu.set_last_error(10110);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSALookupServiceNextW" => {
            let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSALookupServiceNextW handle:0x{:x}",
                lookup_handle
            );
            emu.set_last_error(10110);
            emu.regs_mut().rax = 0xFFFFFFFF;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSANSPIoctl" => {
            let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let control_code = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSANSPIoctl handle:0x{:x} code:0x{:x}",
                lookup_handle,
                control_code
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSANtohl" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let netlong = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSANtohl sock:0x{:x} val:0x{:x}", sock, netlong);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if result_ptr != 0 {
                    emu.maps.write_dword(result_ptr, u32::from_be(netlong));
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSANtohs" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let netshort = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u16;
            let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSANtohs sock:0x{:x} val:0x{:x}",
                sock,
                netshort
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if result_ptr != 0 {
                    emu.maps.write_word(result_ptr, u16::from_be(netshort));
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAPoll" => {
            let fds_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let nfds = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let timeout = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            log_red!(emu, "ws2_32!WSAPoll nfds:{} timeout:{}", nfds, timeout);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAProviderCompleteAsyncCall" => {
            let id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let result = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAProviderCompleteAsyncCall id:0x{:x} result:{}",
                id,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAProviderConfigChange" => {
            let handle_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let overlapped_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAProviderConfigChange");
            if handle_ptr != 0 {
                let handle = helper::handler_create("wsa_provider_change");
                emu.maps.write_dword(handle_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSARecv" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSARecv arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSARecvDisconnect" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSARecvDisconnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSARecvFrom" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buffers_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buffer_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let bytes_recvd_ptr =
                emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSARecvFrom sock:0x{:x} buffers:{}",
                sock,
                buffer_count
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if bytes_recvd_ptr != 0 {
                    emu.maps.write_dword(bytes_recvd_ptr, 0);
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSARemoveServiceClass" => {
            let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSARemoveServiceClass ptr:0x{:x}", class_id_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAResetEvent" => {
            let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAResetEvent event:0x{:x}", event);
            if helper::handler_exist(event) {
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(6);
                emu.regs_mut().rax = 0;
            }
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASend" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSASend arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSASendDisconnect" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSASendDisconnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASendMsg" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let bytes_sent_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSASendMsg sock:0x{:x} flags:0x{:x}",
                sock,
                flags
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if bytes_sent_ptr != 0 {
                    emu.maps.write_dword(bytes_sent_ptr, 0);
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASendTo" => {
            let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buffers_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let buffer_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let bytes_sent_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSASendTo sock:0x{:x} buffers:{}",
                sock,
                buffer_count
            );
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if buffers_ptr != 0 && buffer_count > 0 {
                    let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0);
                    if bytes_sent_ptr != 0 {
                        emu.maps.write_dword(bytes_sent_ptr, buf_len);
                    }
                }
                emu.regs_mut().rax = 0;
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASetBlockingHook" => {
            let hook_fn = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASetEvent" => {
            let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSASetEvent event:0x{:x}", event);
            if helper::handler_exist(event) {
                emu.regs_mut().rax = 1;
            } else {
                emu.set_last_error(6);
                emu.regs_mut().rax = 0;
            }
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASetServiceA" => {
            let query_set_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSASetServiceA op:{} flags:0x{:x}",
                operation,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASetServiceW" => {
            let query_set_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let operation = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSASetServiceW op:{} flags:0x{:x}",
                operation,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSASocketA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "ws2_32!WSASocketA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WSASocketW" => {
            let af = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let sock_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSASocketW af:{} type:{} proto:{}",
                af,
                sock_type,
                proto
            );
            let sock = helper::socket_create();
            emu.set_last_error(0);
            emu.regs_mut().rax = sock;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAStartup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSAStartup arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSAStringToAddressA" => {
            let addr_str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let family = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sockaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let addr_str = if addr_str_ptr != 0 {
                emu.maps.read_string(addr_str_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAStringToAddressA addr:'{}' family:{}",
                addr_str,
                family
            );
            if sockaddr_ptr != 0 && family == 2 {
                emu.maps.write_word(sockaddr_ptr, 2);
                emu.maps.write_word(sockaddr_ptr + 2, 0);
                emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAStringToAddressW" => {
            let addr_str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let family = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let sockaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let addr_str = if addr_str_ptr != 0 {
                emu.maps.read_wide_string(addr_str_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!WSAStringToAddressW addr:'{}' family:{}",
                addr_str,
                family
            );
            if sockaddr_ptr != 0 && family == 2 {
                emu.maps.write_word(sockaddr_ptr, 2);
                emu.maps.write_word(sockaddr_ptr + 2, 0);
                emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAUnadvertiseProvider" => {
            let provider_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSAUnadvertiseProvider id:0x{:x}",
                provider_id_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSAUnhookBlockingHook" => {
            log_red!(emu, "ws2_32!WSAUnhookBlockingHook");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAWaitForMultipleEvents" => {
            let count = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let events_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let wait_all = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let timeout = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WSAWaitForMultipleEvents count:{} wait_all:{} timeout:{}",
                count,
                wait_all,
                timeout
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
        }
        "WSApSetPostRoutine" => {
            let routine_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSApSetPostRoutine routine:0x{:x}", routine_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCDeinstallProvider" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCDeinstallProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCDeinstallProvider32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCDeinstallProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCDeinstallProviderEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCDeinstallProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnableNSProvider" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCEnableNSProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnableNSProvider32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCEnableNSProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnumNameSpaceProviders32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCEnumNameSpaceProviders32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnumNameSpaceProvidersEx32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCEnumNameSpaceProvidersEx32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnumProtocols" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCEnumProtocols ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnumProtocols32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCEnumProtocols32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCEnumProtocolsEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCEnumProtocolsEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetApplicationCategory" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCGetApplicationCategory ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetApplicationCategoryEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCGetApplicationCategoryEx ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetProviderInfo" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCGetProviderInfo ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetProviderInfo32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCGetProviderInfo32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetProviderPath" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCGetProviderPath ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCGetProviderPath32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCGetProviderPath32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallNameSpace" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallNameSpace ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallNameSpace32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallNameSpace32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallNameSpaceEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallNameSpaceEx2" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx2 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallNameSpaceEx32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallProvider" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallProvider64_32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallProvider64_32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallProviderAndChains" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCInstallProviderAndChains ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallProviderAndChains64_32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCInstallProviderAndChains64_32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCInstallProviderEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCInstallProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCSetApplicationCategory" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCSetApplicationCategory ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCSetApplicationCategoryEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WSCSetApplicationCategoryEx ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCSetProviderInfo" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCSetProviderInfo ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCSetProviderInfo32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCSetProviderInfo32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUnInstallNameSpace" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpace ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUnInstallNameSpace32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpace32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUnInstallNameSpaceEx2" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpaceEx2 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUpdateProvider" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUpdateProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUpdateProvider32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUpdateProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCUpdateProviderEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCUpdateProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCWriteNameSpaceOrder" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCWriteNameSpaceOrder ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCWriteNameSpaceOrder32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCWriteNameSpaceOrder32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCWriteProviderOrder" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCWriteProviderOrder ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCWriteProviderOrder32" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCWriteProviderOrder32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WSCWriteProviderOrderEx" => {
            let err_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WSCWriteProviderOrderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCloseApcHelper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahCloseApcHelper handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCloseHandleHelper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahCloseHandleHelper handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCloseNotificationHandleHelper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WahCloseNotificationHandleHelper handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCloseSocketHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahCloseSocketHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCloseThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahCloseThread handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCompleteRequest" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahCompleteRequest arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCreateHandleContextTable" => {
            let table_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WahCreateHandleContextTable ptr:0x{:x}",
                table_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_dword(table_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCreateNotificationHandle" => {
            let table_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WahCreateNotificationHandle ptr:0x{:x}",
                table_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_dword(table_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahCreateSocketHandle" => {
            let table_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahCreateSocketHandle ptr:0x{:x}", table_ptr);
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_dword(table_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahDestroyHandleContextTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahDestroyHandleContextTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahDisableNonIFSHandleSupport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahDisableNonIFSHandleSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahEnableNonIFSHandleSupport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahEnableNonIFSHandleSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahEnumerateHandleContexts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahEnumerateHandleContexts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahInsertHandleContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahInsertHandleContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahNotifyAllProcesses" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahNotifyAllProcesses arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahOpenApcHelper" => {
            let helper_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahOpenApcHelper ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_dword(helper_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahOpenCurrentThread" => {
            let helper_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahOpenCurrentThread ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_dword(helper_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahOpenHandleHelper" => {
            let helper_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahOpenHandleHelper ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_dword(helper_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahOpenNotificationHandleHelper" => {
            let helper_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!WahOpenNotificationHandleHelper ptr:0x{:x}",
                helper_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_dword(helper_ptr, handle as u32);
            }
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahQueueUserApc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahQueueUserApc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahReferenceContextByHandle" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahReferenceContextByHandle arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahRemoveHandleContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!WahRemoveHandleContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "WahWaitForNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahWaitForNotification handle:0x{:x}", handle);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WahWriteLSPEvent" => {
            let event = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!WahWriteLSPEvent event:0x{:x}", event);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "__WSAFDIsSet" => {
            let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let set_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!__WSAFDIsSet fd:0x{:x} set:0x{:x}", fd, set_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "freeaddrinfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "ws2_32!freeaddrinfo handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "gethostbyaddr" => {
            let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let addr_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "ws2_32!gethostbyaddr ptr:0x{:x} len:{} type:{}",
                addr_ptr,
                len,
                addr_type
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
        }
        "gethostname" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!gethostname arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "getnameinfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!getnameinfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "getprotobyname" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!getprotobyname name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "getprotobynumber" => {
            let number = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "ws2_32!getprotobynumber number:{}", number);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "getservbyname" => {
            let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            let proto = if proto_ptr != 0 {
                emu.maps.read_string(proto_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!getservbyname name:'{}' proto:'{}'",
                name,
                proto
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "getservbyport" => {
            let port = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let proto = if proto_ptr != 0 {
                emu.maps.read_string(proto_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!getservbyport port:{} proto:'{}'", port, proto);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
        }
        "getsockname" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "ws2_32!getsockname arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "inet_ntoa" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!inet_ntoa arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "inet_ntop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!inet_ntop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "inet_pton" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "ws2_32!inet_pton arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit
                && let Some(ref fname) = emu.cfg.dump_filename
            {
                serialization::Serialization::dump(emu, fname);
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
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

fn getaddrinfo(emu: &mut emu::Emu) {
    let node_name_ptr = arg(emu, 0) as u64;
    let service_name_ptr = arg(emu, 1) as u64;
    let hints_ptr = arg(emu, 2) as u64;
    let result_ptr_ptr = arg(emu, 3) as u64;

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
    let mut hints_flags = 0i32;
    let mut hints_family = 0i32;
    let mut hints_socktype = 0i32;
    let mut hints_protocol = 0i32;

    if hints_ptr != 0 {
        hints_flags = emu.maps.read_dword(hints_ptr).unwrap_or(0) as i32;
        hints_family = emu.maps.read_dword(hints_ptr + 4).unwrap_or(0) as i32;
        hints_socktype = emu.maps.read_dword(hints_ptr + 8).unwrap_or(0) as i32;
        hints_protocol = emu.maps.read_dword(hints_ptr + 12).unwrap_or(0) as i32;
    }

    // 32-bit ADDRINFOA: 32 bytes (8 dwords)
    let addrinfo_size: u64 = 32;
    let sockaddr_in_size: u64 = 16;

    // Allocate memory for the result
    let heap_management = emu.heap_mut();
    let addrinfo_addr =
        match heap_management.allocate((addrinfo_size + sockaddr_in_size + 100) as usize) {
            Some(addr) => addr,
            None => {
                log::warn!("ws2_32!getaddrinfo: heap allocation failed");
                emu.regs_mut().rax = 1; // EAI_FAIL
                stdcall_ret(emu, 4);
                return;
            }
        };
    let sockaddr_addr = addrinfo_addr + addrinfo_size;
    let canonname_addr = sockaddr_addr + sockaddr_in_size;

    // Create a dummy sockaddr_in (IPv4 address 127.0.0.1, port based on service)
    let ip_addr: u32 = 0x0100007f; // 127.0.0.1 in network byte order

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

    // Write 32-bit ADDRINFOA structure (all fields are dwords)
    emu.maps.write_dword(addrinfo_addr, hints_flags as u32); // +0  ai_flags
    emu.maps.write_dword(
        addrinfo_addr + 4,
        if hints_family != 0 {
            hints_family as u32
        } else {
            2 // AF_INET
        },
    );
    emu.maps.write_dword(
        addrinfo_addr + 8,
        if hints_socktype != 0 {
            hints_socktype as u32
        } else {
            1 // SOCK_STREAM
        },
    );
    emu.maps.write_dword(
        addrinfo_addr + 12,
        if hints_protocol != 0 {
            hints_protocol as u32
        } else {
            6 // IPPROTO_TCP
        },
    );
    emu.maps
        .write_dword(addrinfo_addr + 16, sockaddr_in_size as u32); // +16 ai_addrlen
    emu.maps
        .write_dword(addrinfo_addr + 20, canonname_addr as u32); // +20 ai_canonname
    emu.maps
        .write_dword(addrinfo_addr + 24, sockaddr_addr as u32); // +24 ai_addr
    emu.maps.write_dword(addrinfo_addr + 28, 0); // +28 ai_next = NULL

    // Set ai_canonname to the node name or "localhost"
    let canon_name = if node_name == "NULL" || node_name.is_empty() || node_name == "localhost" {
        "localhost.localdomain".to_string()
    } else if node_name == "127.0.0.1" {
        "localhost".to_string()
    } else {
        format!("{}.localdomain", node_name)
    };
    emu.maps.write_string(canonname_addr, &canon_name);

    // Store the result pointer in the ppResult parameter (32-bit pointer)
    if result_ptr_ptr != 0 {
        emu.maps.write_dword(result_ptr_ptr, addrinfo_addr as u32);
    }

    log::trace!(
        "\tcreated 32-bit ADDRINFO for {}:{} at 0x{:x}",
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
    stdcall_ret(emu, 4);
}

pub fn WsaStartup(emu: &mut emu::Emu) {
    let version = arg(emu, 0);
    let wsadata_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "ws2_32!WsaStartup version:0x{:x} wsadata:0x{:x}",
        version,
        wsadata_ptr
    );
    if wsadata_ptr != 0 {
        emu.maps.write_word(wsadata_ptr, version as u16);
        emu.maps.write_word(wsadata_ptr + 2, version as u16);
        emu.maps.write_string(wsadata_ptr + 4, "WinSock 2.0");
        emu.maps.write_string(wsadata_ptr + 4 + 257, "Running");
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129, 512);
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129 + 2, 0);
    }
    stdcall_ret(emu, 2);
    emu.regs_mut().rax = 0;
}

fn WsaSocketA(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!WsaSocketA");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}

pub fn socket(emu: &mut emu::Emu) {
    log_red!(emu, "ws2_32!socket");

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::socket_create();
}

fn WsaHtons(emu: &mut emu::Emu) {
    let _sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let host_port = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u16;
    let out_port_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    log_red!(emu, "ws2_32!WsaHtons port:{}", host_port);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if out_port_ptr != 0 {
        emu.maps.write_word(out_port_ptr, host_port.to_be());
    }
    emu.regs_mut().rax = 0;
}

pub fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.maps.read_word(emu.regs().get_esp()).unwrap_or_default();

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.stack_pop32(false);
    emu.regs_mut().rax = port.to_be() as u64;
}

pub fn inet_addr(emu: &mut emu::Emu) {
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let ip_str = emu.maps.read_string(addr_ptr);
    log_red!(emu, "ws2_32!inet_addr '{}'", ip_str);
    emu.stack_pop32(false);
    let mut result: u32 = 0xffff_ffff;
    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() == 4
        && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
            parts[0].parse::<u8>(),
            parts[1].parse::<u8>(),
            parts[2].parse::<u8>(),
            parts[3].parse::<u8>(),
        )
    {
        result = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
    }
    emu.regs_mut().rax = result as u64;
}

pub fn connect(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sockaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    //let sockaddr = emu.maps.read_bytes(sockaddr_ptr, 8);
    let family: u16 = emu.maps.read_word(sockaddr_ptr).unwrap_or(0);
    let port: u16 = emu.maps.read_word(sockaddr_ptr + 2).unwrap_or(0).to_be();
    let ip: u32 = emu.maps.read_dword(sockaddr_ptr + 4).unwrap_or(0);

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

pub fn recv(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let mut len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

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
        emu.count_recv += 1;
        if emu.count_recv > 3 {
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

pub fn send(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let mut len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

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
        emu.count_send += 1;
        if emu.count_send > 3 {
            len = 0; // finish the send loop
        }

        emu.regs_mut().rax = len;
    }
}

fn bind(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let saddr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let family: u16 = emu.maps.read_word(saddr).unwrap_or(0);
    let port: u16 = emu.maps.read_word(saddr + 2).unwrap_or(0);
    let ip: u32 = emu.maps.read_dword(saddr + 4).unwrap_or(0);

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
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let connections = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

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
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let saddr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

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

pub fn closesocket(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

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
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let optname = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let optval = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let optlen = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let optname = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let optval = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let optlen = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let saddr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cond = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let callback = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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

pub fn gethostbyname(emu: &mut emu::Emu) {
    let name = emu.maps.read_string(arg(emu, 0) as u64);
    log_red!(emu, "ws2_32!gethostbyname `{}`", name);
    // One block holds the hostent, the address and the name string; the
    // resolved address is a fixed 1.2.3.4 so callers get a well-formed result.
    let block = match emu.maps.alloc(1024) {
        Some(a) => a,
        None => {
            log::error!("ws2_32!gethostbyname: out of memory");
            emu.stack_pop32(false);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    if emu
        .maps
        .create_map("hostent", block, 1024, Permission::READ_WRITE)
        .is_err()
    {
        log::error!("ws2_32!gethostbyname: cannot create hostent map");
        emu.stack_pop32(false);
        emu.regs_mut().rax = 0;
        return;
    }
    let name_addr = block + 1024 - 100;
    emu.maps.write_dword(block, 0x04030201); // the IP
    emu.maps.write_dword(block + 8, block as u32); // addr_list[0]
    emu.maps.write_dword(block + 12, 0); // addr_list NULL terminator
    emu.maps.write_string(name_addr, &name);
    // 32-bit hostent: h_name, h_aliases, h_addrtype, h_length, h_addr_list
    let hostent = block + 30;
    emu.maps.write_dword(hostent, name_addr as u32);
    emu.maps.write_dword(hostent + 4, 0);
    emu.maps.write_word(hostent + 8, 2); // AF_INET
    emu.maps.write_word(hostent + 10, 4);
    emu.maps.write_dword(hostent + 12, (block + 8) as u32);
    let _ = Hostent::size();
    emu.regs_mut().rax = hostent;
    stdcall_ret(emu, 1);
}
fn GetSockName(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let sockaddr_ptr = arg(emu, 1) as u64;
    let namelen_ptr = arg(emu, 2) as u64;
    log_red!(emu, "ws2_32!GetSockName sock: {}", sock);
    if sockaddr_ptr != 0 {
        emu.maps.write_dword(sockaddr_ptr, 0);
    }
    if namelen_ptr != 0 {
        emu.maps.write_dword(namelen_ptr, 16); // sizeof(sockaddr_in)
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
fn sendto(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let len = arg(emu, 2);
    log_red!(emu, "ws2_32!sendto sock: {} len: {}", sock, len);
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
    }
    emu.regs_mut().rax = len as u64; // bytes sent
    stdcall_ret(emu, 6);
}
fn recvfrom(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let buff = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64;
    log_red!(
        emu,
        "ws2_32!recvfrom sock: {} buff: 0x{:x} len: {}",
        sock,
        buff,
        len
    );
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 0xffff_ffff; // SOCKET_ERROR
        return;
    }
    // No real peer: report an orderly connection close (0 bytes).
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}
fn WsaConnect(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let name_ptr = arg(emu, 1) as u64;
    log_red!(emu, "ws2_32!WSAConnect sock:{} name:0x{:x}", sock, name_ptr);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xffff_ffff_u64;
    } else {
        if name_ptr != 0 {
            let port = emu.maps.read_word(name_ptr + 2).unwrap_or(0).to_be();
            let ip0 = emu.maps.read_byte(name_ptr + 4).unwrap_or(0);
            let ip1 = emu.maps.read_byte(name_ptr + 5).unwrap_or(0);
            let ip2 = emu.maps.read_byte(name_ptr + 6).unwrap_or(0);
            let ip3 = emu.maps.read_byte(name_ptr + 7).unwrap_or(0);
            log_red!(
                emu,
                "\tconnecting to {}.{}.{}.{}:{}",
                ip0,
                ip1,
                ip2,
                ip3,
                port
            );
        }
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 7);
}
fn WsaRecv(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let bytes_recv_ptr = arg(emu, 3) as u64;
    log_red!(emu, "ws2_32!WSARecv sock: {}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0); // orderly close
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 7);
}
fn WsaRecvFrom(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let bytes_recv_ptr = arg(emu, 3) as u64;
    log_red!(emu, "ws2_32!WSARecvFrom sock: {}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 9);
}

fn htonl(emu: &mut emu::Emu) {
    let val = arg(emu, 0);
    log_red!(emu, "ws2_32!htonl 0x{:x}", val);
    emu.regs_mut().rax = val.to_be() as u64;
    stdcall_ret(emu, 1);
}

fn ntohl(emu: &mut emu::Emu) {
    let val = arg(emu, 0);
    log_red!(emu, "ws2_32!ntohl 0x{:x}", val);
    emu.regs_mut().rax = u32::from_be(val) as u64;
    stdcall_ret(emu, 1);
}

fn ntohs(emu: &mut emu::Emu) {
    let val = arg(emu, 0) as u16;
    log_red!(emu, "ws2_32!ntohs {}", val);
    emu.regs_mut().rax = u16::from_be(val) as u64;
    stdcall_ret(emu, 1);
}

fn select(emu: &mut emu::Emu) {
    let nfds = arg(emu, 0);
    let readfds = arg(emu, 1) as u64;
    let writefds = arg(emu, 2) as u64;
    let exceptfds = arg(emu, 3) as u64;
    let timeout_ptr = arg(emu, 4) as u64;
    let timeout_str = if timeout_ptr == 0 {
        "infinite".to_string()
    } else {
        let tv_sec = emu.maps.read_dword(timeout_ptr).unwrap_or(0);
        let tv_usec = emu.maps.read_dword(timeout_ptr + 4).unwrap_or(0);
        format!("{}.{:06}s", tv_sec, tv_usec)
    };
    log_red!(
        emu,
        "ws2_32!select nfds:{} read:0x{:x} write:0x{:x} except:0x{:x} timeout:{}",
        nfds,
        readfds,
        writefds,
        exceptfds,
        timeout_str
    );
    // Return 0 = timeout, no sockets ready
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

fn ioctlsocket(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let cmd = arg(emu, 1);
    let argp = arg(emu, 2) as u64;
    let cmd_name = match cmd {
        0x8004667E => "FIONBIO",
        0x4004667F => "FIONREAD",
        0x80000001 => "SIOCATMARK",
        _ => "unknown",
    };
    log_red!(
        emu,
        "ws2_32!ioctlsocket sock:0x{:x} cmd:{}(0x{:x}) argp:0x{:x}",
        sock,
        cmd_name,
        cmd,
        argp
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038); // WSAENOTSOCK
        emu.regs_mut().rax = 0xFFFFFFFF_u64; // SOCKET_ERROR
    } else {
        if cmd == 0x4004667F && argp != 0 {
            // FIONREAD: report 0 bytes available
            emu.maps.write_dword(argp, 0);
        }
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 3);
}

fn shutdown(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let how = arg(emu, 1);
    log_red!(emu, "ws2_32!shutdown sock:{} how:{}", sock, how);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xffff_ffff_u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 2);
}

fn getpeername(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    log_red!(emu, "ws2_32!getpeername sock: {}", sock);
    emu.regs_mut().rax = 0xffff_ffff_u64; // SOCKET_ERROR
    stdcall_ret(emu, 3);
}

fn WsaGetLastError(emu: &mut emu::Emu) {
    let err = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError => {}", err);
    emu.regs_mut().rax = err;
}

fn WsaSetLastError(emu: &mut emu::Emu) {
    let err = arg(emu, 0);
    log_red!(emu, "ws2_32!WSASetLastError err: {}", err);
    emu.set_last_error(err as u64);
    stdcall_ret(emu, 1);
}
