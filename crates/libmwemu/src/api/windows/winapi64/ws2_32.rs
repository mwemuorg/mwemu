use crate::emu;
use crate::serialization;
//use crate::endpoint;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::windows::structures::*;

use crate::maps::mem64::Permission;

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
        "sendto" => sendto(emu),
        "recvfrom" => recvfrom(emu),
        "WsaRecv" => WsaRecv(emu),
        "WsaRecvFrom" => WsaRecvFrom(emu),
        "WsaConnect" => WsaConnect(emu),
        "WsaSend" => WsaSend(emu),
        "FreeAddrInfoEx" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
            helper::handler_close(handle);
        }
        "FreeAddrInfoExW" => {
            let info_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
            helper::handler_close(info_ptr);
        }
        "FreeAddrInfoW" => {
            let info_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!FreeAddrInfoW ptr:0x{:x}", info_ptr);
            if info_ptr != 0 {
                let map_name = format!("addrinfo_{:x}", info_ptr);
                emu.maps.free(&map_name);
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAddrInfoExA" => {
            let name_ptr = emu.regs().rcx;
            let service_ptr = emu.regs().rdx;
            let namespace = emu.regs().r8 as u32;
            let hints_ptr = emu.regs().r9;
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
        }
        "GetAddrInfoExCancel" => {
            let cancel_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!GetAddrInfoExCancel ptr:0x{:x}", cancel_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetAddrInfoExOverlappedResult" => {
            let overlapped_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!GetAddrInfoExOverlappedResult overlapped:0x{:x}",
                overlapped_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetAddrInfoExW" => {
            let name_ptr = emu.regs().rcx;
            let service_ptr = emu.regs().rdx;
            let namespace = emu.regs().r8 as u32;
            let hints_ptr = emu.regs().r9;
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
        }
        "GetAddrInfoW" => {
            let node_ptr = emu.regs().rcx;
            let service_ptr = emu.regs().rdx;
            let hints_ptr = emu.regs().r8;
            let result_ptr_ptr = emu.regs().r9;
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
        }
        "GetHostNameW" => {
            let name_ptr = emu.regs().rcx;
            let namelen = emu.regs().rdx as u32;
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
        }
        "GetNameInfoW" => {
            let sa_ptr = emu.regs().rcx;
            let salen = emu.regs().rdx as u32;
            let host_ptr = emu.regs().r8;
            let hostlen = emu.regs().r9 as u32;
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
        }
        "InetNtopW" => {
            let family = emu.regs().rcx as u32;
            let addr_ptr = emu.regs().rdx;
            let str_buf = emu.regs().r8;
            let str_len = emu.regs().r9 as u32;
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
        }
        "InetPtonW" => {
            let family = emu.regs().rcx as u32;
            let src_ptr = emu.regs().rdx;
            let dst_ptr = emu.regs().r8;
            let src_str = if src_ptr != 0 {
                emu.maps.read_wide_string(src_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!InetPtonW family:{} src:'{}'", family, src_str);
            if family == 2 && dst_ptr != 0 {
                let parts: Vec<&str> = src_str.split('.').collect();
                if parts.len() == 4
                    && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                        parts[0].parse::<u8>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                        parts[3].parse::<u8>(),
                    )
                {
                    let addr =
                        (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
                    emu.maps.write_dword(dst_ptr, addr);
                    emu.regs_mut().rax = 1;
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ProcessSocketNotifications" => {
            let completion_port = emu.regs().rcx;
            let count = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
                completion_port,
                count
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SetAddrInfoExA" => {
            let name_ptr = emu.regs().rcx;
            let service_ptr = emu.regs().rdx;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!SetAddrInfoExA name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SetAddrInfoExW" => {
            let name_ptr = emu.regs().rcx;
            let service_ptr = emu.regs().rdx;
            let name = if name_ptr != 0 {
                emu.maps.read_wide_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!SetAddrInfoExW name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WEP" => {
            let reason = emu.regs().rcx;
            log_red!(emu, "ws2_32!WEP reason:{}", reason);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WPUCompleteOverlappedRequest" => {
            let sock = emu.regs().rcx;
            let overlapped_ptr = emu.regs().rdx;
            let error = emu.regs().r8;
            let bytes = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
                sock,
                error,
                bytes
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WPUGetProviderPathEx" => {
            let provider_id_ptr = emu.regs().rcx;
            let path_ptr = emu.regs().rdx;
            let path_len_ptr = emu.regs().r8;
            let flags = emu.regs().r9;
            log_red!(emu, "ws2_32!WPUGetProviderPathEx flags:{}", flags);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAAccept" => {
            let sock = emu.regs().rcx;
            let addr_ptr = emu.regs().rdx;
            let addrlen_ptr = emu.regs().r8;
            let condition_ptr = emu.regs().r9;
            log_red!(emu, "ws2_32!WSAAccept sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
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
        }
        "WSAAddressToStringA" => {
            let addr_ptr = emu.regs().rcx;
            let addr_len = emu.regs().rdx as u32;
            let proto_info_ptr = emu.regs().r8;
            let str_ptr = emu.regs().r9;
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
        }
        "WSAAddressToStringW" => {
            let addr_ptr = emu.regs().rcx;
            let addr_len = emu.regs().rdx as u32;
            let proto_info_ptr = emu.regs().r8;
            let str_ptr = emu.regs().r9;
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
        }
        "WSAAdvertiseProvider" => {
            let provider_id_ptr = emu.regs().rcx;
            let routine_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WSAAdvertiseProvider id:0x{:x} routine:0x{:x}",
                provider_id_ptr,
                routine_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAAsyncGetHostByAddr" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let addr_ptr = emu.regs().r8;
            let addr_len = emu.regs().r9 as u32;
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetHostByAddr hwnd:0x{:x} msg:{}",
                hwnd,
                msg
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
        }
        "WSAAsyncGetHostByName" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let name_ptr = emu.regs().r8;
            let buf_ptr = emu.regs().r9;
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
        }
        "WSAAsyncGetProtoByName" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let name_ptr = emu.regs().r8;
            let buf_ptr = emu.regs().r9;
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
        }
        "WSAAsyncGetProtoByNumber" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let number = emu.regs().r8 as u32;
            let buf_ptr = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetProtoByNumber hwnd:0x{:x} number:{}",
                hwnd,
                number
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
        }
        "WSAAsyncGetServByName" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let name_ptr = emu.regs().r8;
            let proto_ptr = emu.regs().r9;
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
        }
        "WSAAsyncGetServByPort" => {
            let hwnd = emu.regs().rcx;
            let msg = emu.regs().rdx as u32;
            let port = emu.regs().r8 as u32;
            let proto_ptr = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!WSAAsyncGetServByPort hwnd:0x{:x} port:{}",
                hwnd,
                port
            );
            let handle = helper::handler_create("wsa_async_task");
            emu.regs_mut().rax = handle;
        }
        "WSAAsyncSelect" => {
            let sock = emu.regs().rcx;
            let hwnd = emu.regs().rdx;
            let msg = emu.regs().r8;
            let events = emu.regs().r9;
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
        }
        "WSACancelAsyncRequest" => {
            let async_handle = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSACancelAsyncRequest handle:0x{:x}",
                async_handle
            );
            if helper::handler_exist(async_handle) {
                helper::handler_close(async_handle);
                emu.regs_mut().rax = 0;
            } else {
                emu.set_last_error(10044); // WSAEINVAL
                emu.regs_mut().rax = 0xFFFFFFFF;
            }
        }
        "WSACancelBlockingCall" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSACancelBlockingCall 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSACleanup" => {
            log_red!(emu, "ws2_32!WSACleanup");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSACloseEvent" => {
            let event = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSACloseEvent event: 0x{:x}", event);
            if event != 0 {
                helper::handler_close(event);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "WSAConnect" => {
            let sock = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let namelen = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAConnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "WSAConnectByList" => {
            let sock = emu.regs().rcx;
            let addr_list_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAConnectByList sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 1;
            }
        }
        "WSAConnectByNameA" => {
            let sock = emu.regs().rcx;
            let node_ptr = emu.regs().rdx;
            let service_ptr = emu.regs().r8;
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
        }
        "WSAConnectByNameW" => {
            let sock = emu.regs().rcx;
            let node_ptr = emu.regs().rdx;
            let service_ptr = emu.regs().r8;
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
        }
        "WSACreateEvent" => {
            let event = helper::handler_create("wsa_event");
            log_red!(emu, "ws2_32!WSACreateEvent -> 0x{:x}", event);
            emu.regs_mut().rax = event;
        }
        "WSADuplicateSocketA" => {
            let sock = emu.regs().rcx;
            let process_id = emu.regs().rdx;
            let proto_info_ptr = emu.regs().r8;
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
        }
        "WSADuplicateSocketW" => {
            let sock = emu.regs().rcx;
            let process_id = emu.regs().rdx;
            let proto_info_ptr = emu.regs().r8;
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
        }
        "WSAEnumNameSpaceProvidersA" => {
            let buf_len_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersA");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAEnumNameSpaceProvidersExA" => {
            let buf_len_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExA");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAEnumNameSpaceProvidersExW" => {
            let buf_len_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExW");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAEnumNameSpaceProvidersW" => {
            let buf_len_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersW");
            emu.set_last_error(10014);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAEnumNetworkEvents" => {
            let sock = emu.regs().rcx;
            let event = emu.regs().rdx;
            let net_events_ptr = emu.regs().r8;
            log_red!(
                emu,
                "ws2_32!WSAEnumNetworkEvents sock: 0x{:x} event: 0x{:x}",
                sock,
                event
            );
            if net_events_ptr != 0 {
                // Zero out WSANETWORKEVENTS structure (4 + 10*4 = 44 bytes)
                for i in 0..44u64 {
                    emu.maps.write_byte(net_events_ptr + i, 0);
                }
                // Set FD_READ | FD_WRITE | FD_CONNECT in lNetworkEvents
                emu.maps.write_dword(net_events_ptr, 0x01 | 0x02 | 0x10);
            }
            emu.regs_mut().rax = 0;
        }
        "WSAEnumProtocolsA" => {
            let protocols_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAEnumProtocolsA");
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "WSAEnumProtocolsW" => {
            let protocols_ptr = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let buf_len_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAEnumProtocolsW");
            if buf_len_ptr != 0 {
                emu.maps.write_dword(buf_len_ptr, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "WSAEventSelect" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSAEventSelect 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSAGetLastError" => {
            let err = emu.get_last_error();
            log_red!(emu, "ws2_32!WSAGetLastError = {}", err);
            emu.set_last_error(0);
            emu.regs_mut().rax = err;
        }
        "WSAGetOverlappedResult" => {
            let sock = emu.regs().rcx;
            let overlapped_ptr = emu.regs().rdx;
            let transfer_ptr = emu.regs().r8;
            let wait = emu.regs().r9 as u32;
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
        }
        "WSAGetQOSByName" => {
            let sock = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let qos_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAGetServiceClassInfoA" => {
            let provider_id = emu.regs().rcx;
            let class_id = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAGetServiceClassInfoA");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAGetServiceClassInfoW" => {
            let provider_id = emu.regs().rcx;
            let class_id = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSAGetServiceClassInfoW");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAGetServiceClassNameByClassIdA" => {
            let class_id_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdA");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAGetServiceClassNameByClassIdW" => {
            let class_id_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdW");
            emu.set_last_error(10109);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSAHtonl" => {
            let sock = emu.regs().rcx;
            let hostlong = emu.regs().rdx as u32;
            let result_ptr = emu.regs().r8;
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
        }
        "WSAHtons" => {
            let sock = emu.regs().rcx;
            let hostshort = emu.regs().rdx as u16;
            let result_ptr = emu.regs().r8;
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
        }
        "WSAInstallServiceClassA" => {
            let class_info_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSAInstallServiceClassA ptr:0x{:x}",
                class_info_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAInstallServiceClassW" => {
            let class_info_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSAInstallServiceClassW ptr:0x{:x}",
                class_info_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAIoctl" => {
            let sock = emu.regs().rcx;
            let ioctl_code = emu.regs().rdx;
            let in_buf_ptr = emu.regs().r8;
            let in_buf_len = emu.regs().r9;
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
        }
        "WSAIsBlocking" => {
            log_red!(emu, "ws2_32!WSAIsBlocking");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAJoinLeaf" => {
            let sock = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let namelen = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(0);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = sock;
            }
        }
        "WSALookupServiceBeginA" => {
            let restrictions_ptr = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let handle_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSALookupServiceBeginA flags:0x{:x}", flags);
            let handle = helper::handler_create("wsa_lookup");
            if handle_ptr != 0 {
                emu.maps.write_qword(handle_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WSALookupServiceBeginW" => {
            let restrictions_ptr = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let handle_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSALookupServiceBeginW flags:0x{:x}", flags);
            let handle = helper::handler_create("wsa_lookup");
            if handle_ptr != 0 {
                emu.maps.write_qword(handle_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WSALookupServiceEnd" => {
            let lookup_handle = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSALookupServiceEnd handle:0x{:x}",
                lookup_handle
            );
            if helper::handler_exist(lookup_handle) {
                helper::handler_close(lookup_handle);
                emu.regs_mut().rax = 0;
            } else {
                emu.set_last_error(6); // ERROR_INVALID_HANDLE
                emu.regs_mut().rax = 0xFFFFFFFF;
            }
        }
        "WSALookupServiceNextA" => {
            let lookup_handle = emu.regs().rcx;
            let flags = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WSALookupServiceNextA handle:0x{:x}",
                lookup_handle
            );
            emu.set_last_error(10110);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSALookupServiceNextW" => {
            let lookup_handle = emu.regs().rcx;
            let flags = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WSALookupServiceNextW handle:0x{:x}",
                lookup_handle
            );
            emu.set_last_error(10110);
            emu.regs_mut().rax = 0xFFFFFFFF;
        }
        "WSANSPIoctl" => {
            let lookup_handle = emu.regs().rcx;
            let control_code = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WSANSPIoctl handle:0x{:x} code:0x{:x}",
                lookup_handle,
                control_code
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSANtohl" => {
            let sock = emu.regs().rcx;
            let netlong = emu.regs().rdx as u32;
            let result_ptr = emu.regs().r8;
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
        }
        "WSANtohs" => {
            let sock = emu.regs().rcx;
            let netshort = emu.regs().rdx as u16;
            let result_ptr = emu.regs().r8;
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
        }
        "WSAPoll" => {
            let fds_ptr = emu.regs().rcx;
            let nfds = emu.regs().rdx as u32;
            let timeout = emu.regs().r8 as i32;
            log_red!(emu, "ws2_32!WSAPoll nfds:{} timeout:{}", nfds, timeout);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAProviderCompleteAsyncCall" => {
            let id = emu.regs().rcx;
            let result = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WSAProviderCompleteAsyncCall id:0x{:x} result:{}",
                id,
                result
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAProviderConfigChange" => {
            let handle_ptr = emu.regs().rcx;
            let overlapped_ptr = emu.regs().rdx;
            let completion_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!WSAProviderConfigChange");
            if handle_ptr != 0 {
                let handle = helper::handler_create("wsa_provider_change");
                emu.maps.write_qword(handle_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WSARecv" => {
            let sock = emu.regs().rcx;
            let buffers_ptr = emu.regs().rdx;
            let buffer_count = emu.regs().r8 as u32;
            let bytes_recvd_ptr = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!WSARecv sock: 0x{:x} buffers: {} bytes_recvd: 0x{:x}",
                sock,
                buffer_count,
                bytes_recvd_ptr
            );
            if !helper::socket_exist(sock) {
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                // Fill first WSABUF with 0x90 (nop sled, same as recv)
                if buffers_ptr != 0 && buffer_count > 0 {
                    let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0) as u64;
                    let buf_ptr = emu.maps.read_qword(buffers_ptr + 8).unwrap_or(0);
                    if buf_ptr != 0 {
                        let fill_len = buf_len.min(1024);
                        for i in 0..fill_len {
                            emu.maps.write_byte(buf_ptr + i, 0x90);
                        }
                        if bytes_recvd_ptr != 0 {
                            emu.maps.write_dword(bytes_recvd_ptr, fill_len as u32);
                        }
                    }
                }
                emu.regs_mut().rax = 0;
            }
        }
        "WSARecvDisconnect" => {
            let sock = emu.regs().rcx;
            let data_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSARecvDisconnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "WSARecvFrom" => {
            let sock = emu.regs().rcx;
            let buffers_ptr = emu.regs().rdx;
            let buffer_count = emu.regs().r8 as u32;
            let bytes_recvd_ptr = emu.regs().r9;
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
        }
        "WSARemoveServiceClass" => {
            let class_id_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSARemoveServiceClass ptr:0x{:x}", class_id_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAResetEvent" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSAResetEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSASend" => {
            let sock = emu.regs().rcx;
            let buffers_ptr = emu.regs().rdx;
            let buffer_count = emu.regs().r8 as u32;
            let bytes_sent_ptr = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!WSASend sock: 0x{:x} buffers: {}",
                sock,
                buffer_count
            );
            if !helper::socket_exist(sock) {
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
        }
        "WSASendDisconnect" => {
            let sock = emu.regs().rcx;
            let data_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!WSASendDisconnect sock:0x{:x}", sock);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "WSASendMsg" => {
            let sock = emu.regs().rcx;
            let msg_ptr = emu.regs().rdx;
            let flags = emu.regs().r8 as u32;
            let bytes_sent_ptr = emu.regs().r9;
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
        }
        "WSASendTo" => {
            let sock = emu.regs().rcx;
            let buffers_ptr = emu.regs().rdx;
            let buffer_count = emu.regs().r8 as u32;
            let bytes_sent_ptr = emu.regs().r9;
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
        }
        "WSASetBlockingHook" => {
            let hook_fn = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSASetBlockingHook hook:0x{:x}", hook_fn);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSASetEvent" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSASetEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSASetLastError" => {
            let error_code = emu.regs().rcx as u32;
            emu.set_last_error(error_code as u64);
            log_red!(emu, "ws2_32!WSASetLastError error: {}", error_code);
        }
        "WSASetServiceA" => {
            let query_set_ptr = emu.regs().rcx;
            let operation = emu.regs().rdx;
            let flags = emu.regs().r8;
            log_red!(
                emu,
                "ws2_32!WSASetServiceA op:{} flags:0x{:x}",
                operation,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSASetServiceW" => {
            let query_set_ptr = emu.regs().rcx;
            let operation = emu.regs().rdx;
            let flags = emu.regs().r8;
            log_red!(
                emu,
                "ws2_32!WSASetServiceW op:{} flags:0x{:x}",
                operation,
                flags
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSASocketW" => {
            let af = emu.regs().rcx as u32;
            let sock_type = emu.regs().rdx as u32;
            let proto = emu.regs().r8 as u32;
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
        }
        "WSAStringToAddressA" => {
            let addr_str_ptr = emu.regs().rcx;
            let family = emu.regs().rdx as u32;
            let proto_info_ptr = emu.regs().r8;
            let sockaddr_ptr = emu.regs().r9;
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
        }
        "WSAStringToAddressW" => {
            let addr_str_ptr = emu.regs().rcx;
            let family = emu.regs().rdx as u32;
            let proto_info_ptr = emu.regs().r8;
            let sockaddr_ptr = emu.regs().r9;
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
        }
        "WSAUnadvertiseProvider" => {
            let provider_id_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSAUnadvertiseProvider id:0x{:x}",
                provider_id_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSAUnhookBlockingHook" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSAUnhookBlockingHook 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WSAWaitForMultipleEvents" => {
            let count = emu.regs().rcx as u32;
            let events_ptr = emu.regs().rdx;
            let wait_all = emu.regs().r8 as u32;
            let timeout = emu.regs().r9 as u32;
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
        }
        "WSApSetPostRoutine" => {
            let routine_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSApSetPostRoutine routine:0x{:x}", routine_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCDeinstallProvider" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCDeinstallProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCDeinstallProvider32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCDeinstallProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCDeinstallProviderEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCDeinstallProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnableNSProvider" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCEnableNSProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnableNSProvider32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCEnableNSProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnumNameSpaceProviders32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCEnumNameSpaceProviders32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnumNameSpaceProvidersEx32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCEnumNameSpaceProvidersEx32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnumProtocols" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCEnumProtocols ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnumProtocols32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCEnumProtocols32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCEnumProtocolsEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCEnumProtocolsEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetApplicationCategory" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCGetApplicationCategory ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetApplicationCategoryEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCGetApplicationCategoryEx ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetProviderInfo" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCGetProviderInfo ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetProviderInfo32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCGetProviderInfo32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetProviderPath" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCGetProviderPath ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCGetProviderPath32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCGetProviderPath32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallNameSpace" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallNameSpace ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallNameSpace32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallNameSpace32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallNameSpaceEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallNameSpaceEx2" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx2 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallNameSpaceEx32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallNameSpaceEx32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallProvider" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallProvider64_32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallProvider64_32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallProviderAndChains64_32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCInstallProviderAndChains64_32 ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCInstallProviderEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCInstallProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCSetApplicationCategory" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCSetApplicationCategory ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCSetApplicationCategoryEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCSetApplicationCategoryEx ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCSetProviderInfo" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCSetProviderInfo ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCSetProviderInfo32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCSetProviderInfo32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUnInstallNameSpace" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpace ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUnInstallNameSpace32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpace32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUnInstallNameSpaceEx2" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUnInstallNameSpaceEx2 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUpdateProvider" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUpdateProvider ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUpdateProvider32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUpdateProvider32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCUpdateProviderEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCUpdateProviderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCWriteNameSpaceOrder" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCWriteNameSpaceOrder ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCWriteNameSpaceOrder32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCWriteNameSpaceOrder32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCWriteProviderOrder" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCWriteProviderOrder ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCWriteProviderOrder32" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCWriteProviderOrder32 ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WSCWriteProviderOrderEx" => {
            let err_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WSCWriteProviderOrderEx ptr:0x{:x}", err_ptr);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahCloseApcHelper" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahCloseApcHelper handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
        }
        "WahCloseHandleHelper" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahCloseHandleHelper handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
        }
        "WahCloseNotificationHandleHelper" => {
            let handle = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WahCloseNotificationHandleHelper handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
        }
        "WahCloseSocketHandle" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahCloseSocketHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
        }
        "WahCloseThread" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahCloseThread handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 0;
        }
        "WahCompleteRequest" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahCompleteRequest arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahCreateHandleContextTable" => {
            let table_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WahCreateHandleContextTable ptr:0x{:x}",
                table_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_qword(table_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahCreateNotificationHandle" => {
            let table_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WahCreateNotificationHandle ptr:0x{:x}",
                table_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_qword(table_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahCreateSocketHandle" => {
            let table_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahCreateSocketHandle ptr:0x{:x}", table_ptr);
            let handle = helper::handler_create("wah_handle");
            if table_ptr != 0 {
                emu.maps.write_qword(table_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahDestroyHandleContextTable" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahDestroyHandleContextTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahDisableNonIFSHandleSupport" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahDisableNonIFSHandleSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahEnableNonIFSHandleSupport" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahEnableNonIFSHandleSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahEnumerateHandleContexts" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahEnumerateHandleContexts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahInsertHandleContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahInsertHandleContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahNotifyAllProcesses" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahNotifyAllProcesses arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahOpenApcHelper" => {
            let helper_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahOpenApcHelper ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_qword(helper_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahOpenCurrentThread" => {
            let helper_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahOpenCurrentThread ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_qword(helper_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahOpenHandleHelper" => {
            let helper_ptr = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahOpenHandleHelper ptr:0x{:x}", helper_ptr);
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_qword(helper_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahOpenNotificationHandleHelper" => {
            let helper_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WahOpenNotificationHandleHelper ptr:0x{:x}",
                helper_ptr
            );
            let handle = helper::handler_create("wah_handle");
            if helper_ptr != 0 {
                emu.maps.write_qword(helper_ptr, handle);
            }
            emu.regs_mut().rax = 0;
        }
        "WahQueueUserApc" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahQueueUserApc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahReferenceContextByHandle" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahReferenceContextByHandle arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahRemoveHandleContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WahRemoveHandleContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WahWaitForNotification" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahWaitForNotification 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WahWriteLSPEvent" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "ws2_32!WahWriteLSPEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__WSAFDIsSet" => {
            let sock = emu.regs().rcx;
            let fd_set_ptr = emu.regs().rdx;
            log_red!(emu, "ws2_32!__WSAFDIsSet sock:0x{:x}", sock);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "freeaddrinfo" => {
            let pai = emu.regs().rcx;
            log_red!(emu, "ws2_32!freeaddrinfo ptr:0x{:x}", pai);
            if pai != 0 {
                let map_name = format!("addrinfo_{:x}", pai);
                emu.maps.free(&map_name);
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "gethostbyaddr" => {
            let addr_ptr = emu.regs().rcx;
            let len = emu.regs().rdx as u32;
            let typ = emu.regs().r8 as u32;
            log_red!(
                emu,
                "ws2_32!gethostbyaddr addr:0x{:x} len:{} type:{}",
                addr_ptr,
                len,
                typ
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "gethostname" => {
            let name_ptr = emu.regs().rcx;
            let namelen = emu.regs().rdx as u32;
            log_red!(
                emu,
                "ws2_32!gethostname buf:0x{:x} len:{}",
                name_ptr,
                namelen
            );
            if name_ptr != 0 && namelen > 7 {
                emu.maps.write_string(name_ptr, "DESKTOP");
            }
            emu.regs_mut().rax = 0;
        }
        "getnameinfo" => {
            let sa = emu.regs().rcx;
            let salen = emu.regs().rdx as u32;
            let host = emu.regs().r8;
            let hostlen = emu.regs().r9 as u32;
            log_red!(
                emu,
                "ws2_32!getnameinfo sa:0x{:x} host:0x{:x} hostlen:{}",
                sa,
                host,
                hostlen
            );
            if host != 0 && hostlen > 9 {
                emu.maps.write_string(host, "localhost");
            }
            emu.regs_mut().rax = 0;
        }
        "getpeername" => {
            let sock = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let namelen_ptr = emu.regs().r8;
            log_red!(
                emu,
                "ws2_32!getpeername sock: 0x{:x} name: 0x{:x}",
                sock,
                name_ptr
            );
            if !helper::socket_exist(sock) {
                log_red!(emu, "  invalid socket");
                emu.regs_mut().rax = 0xFFFFFFFF; // SOCKET_ERROR
            } else {
                if name_ptr != 0 {
                    // Write dummy sockaddr_in: AF_INET, port 80, 127.0.0.1
                    emu.maps.write_word(name_ptr, 2); // AF_INET
                    emu.maps.write_word(name_ptr + 2, 80u16.to_be()); // port
                    emu.maps.write_dword(name_ptr + 4, 0x0100007f); // 127.0.0.1
                    for i in 0..8u64 {
                        emu.maps.write_byte(name_ptr + 8 + i, 0); // padding
                    }
                }
                if namelen_ptr != 0 {
                    emu.maps.write_dword(namelen_ptr, 16);
                }
                emu.regs_mut().rax = 0; // success
            }
        }
        "getprotobyname" => {
            let name_ptr = emu.regs().rcx;
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!getprotobyname name:'{}'", name);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "getprotobynumber" => {
            let proto = emu.regs().rcx as u32;
            log_red!(emu, "ws2_32!getprotobynumber proto:{}", proto);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "getservbyname" => {
            let name_ptr = emu.regs().rcx;
            let proto_ptr = emu.regs().rdx;
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
        }
        "getservbyport" => {
            let port = emu.regs().rcx as u32;
            let proto_ptr = emu.regs().rdx;
            let proto = if proto_ptr != 0 {
                emu.maps.read_string(proto_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "ws2_32!getservbyport port:{} proto:'{}'", port, proto);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "getsockname" => {
            let sock = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let namelen_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!getsockname sock:0x{:x}", sock);
            if name_ptr != 0 {
                emu.maps.write_word(name_ptr, 2); // AF_INET
                emu.maps.write_word(name_ptr + 2, 0); // port 0
                emu.maps.write_dword(name_ptr + 4, 0); // INADDR_ANY
                for i in 0..8u64 {
                    emu.maps.write_byte(name_ptr + 8 + i, 0);
                }
            }
            if namelen_ptr != 0 {
                emu.maps.write_dword(namelen_ptr, 16);
            }
            emu.regs_mut().rax = 0;
        }
        "htonl" => {
            let val = emu.regs().rcx as u32;
            log_red!(emu, "ws2_32!htonl val:0x{:x}", val);
            emu.set_last_error(0);
            emu.regs_mut().rax = val.to_be() as u64;
        }
        "inet_ntoa" => {
            let addr_val = emu.regs().rcx as u32;
            let a = addr_val & 0xFF;
            let b = (addr_val >> 8) & 0xFF;
            let c = (addr_val >> 16) & 0xFF;
            let d = (addr_val >> 24) & 0xFF;
            let ip_str = format!("{}.{}.{}.{}", a, b, c, d);
            log_red!(emu, "ws2_32!inet_ntoa -> {}", ip_str);
            let alloc_sz = 32u64;
            let buf = match emu.maps.alloc(alloc_sz) {
                Some(a) => a,
                None => {
                    emu.regs_mut().rax = 0;
                    return String::new();
                }
            };
            emu.maps.create_map(
                "inet_ntoa_buf",
                buf,
                alloc_sz,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_string(buf, &ip_str);
            emu.regs_mut().rax = buf;
        }
        "inet_ntop" => {
            let family = emu.regs().rcx as u32;
            let addr_ptr = emu.regs().rdx;
            let str_buf = emu.regs().r8;
            let str_len = emu.regs().r9 as u32;
            log_red!(
                emu,
                "ws2_32!inet_ntop family: {} addr: 0x{:x} buf: 0x{:x}",
                family,
                addr_ptr,
                str_buf
            );
            if family == 2 && addr_ptr != 0 && str_buf != 0 {
                // AF_INET
                let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
                let a = ip & 0xFF;
                let b = (ip >> 8) & 0xFF;
                let c = (ip >> 16) & 0xFF;
                let d = (ip >> 24) & 0xFF;
                let s = format!("{}.{}.{}.{}", a, b, c, d);
                if (s.len() as u32) < str_len {
                    emu.maps.write_string(str_buf, &s);
                    emu.regs_mut().rax = str_buf;
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "inet_pton" => {
            let family = emu.regs().rcx as u32;
            let src_ptr = emu.regs().rdx;
            let dst_ptr = emu.regs().r8;
            let src_str = if src_ptr != 0 {
                emu.maps.read_string(src_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "ws2_32!inet_pton family: {} src: '{}'",
                family,
                src_str
            );
            if family == 2 && dst_ptr != 0 {
                // AF_INET
                let parts: Vec<&str> = src_str.split('.').collect();
                if parts.len() == 4
                    && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                        parts[0].parse::<u8>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                        parts[3].parse::<u8>(),
                    )
                {
                    let addr =
                        (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
                    emu.maps.write_dword(dst_ptr, addr);
                    emu.regs_mut().rax = 1; // success
                } else {
                    emu.regs_mut().rax = 0; // invalid
                }
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ioctlsocket" => {
            let sock = emu.regs().rcx;
            let cmd = emu.regs().rdx as u32;
            let argp_ptr = emu.regs().r8;
            log_red!(emu, "ws2_32!ioctlsocket sock:0x{:x} cmd:0x{:x}", sock, cmd);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                if argp_ptr != 0 && cmd == 0x4004667f {
                    emu.maps.write_dword(argp_ptr, 0);
                }
                emu.regs_mut().rax = 0;
            }
        }
        "ntohl" => {
            let val = emu.regs().rcx as u32;
            log_red!(emu, "ws2_32!ntohl val:0x{:x}", val);
            emu.set_last_error(0);
            emu.regs_mut().rax = u32::from_be(val) as u64;
        }
        "ntohs" => {
            let val = emu.regs().rcx as u16;
            log_red!(emu, "ws2_32!ntohs val:0x{:x}", val);
            emu.set_last_error(0);
            emu.regs_mut().rax = u16::from_be(val) as u64;
        }
        "select" => {
            let nfds = emu.regs().rcx as u32;
            let readfds = emu.regs().rdx;
            let writefds = emu.regs().r8;
            let exceptfds = emu.regs().r9;
            log_red!(
                emu,
                "ws2_32!select nfds:{} read:0x{:x} write:0x{:x}",
                nfds,
                readfds,
                writefds
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "shutdown" => {
            let sock = emu.regs().rcx;
            let how = emu.regs().rdx as u32;
            log_red!(emu, "ws2_32!shutdown sock:0x{:x} how:{}", sock, how);
            if !helper::socket_exist(sock) {
                emu.set_last_error(10038);
                emu.regs_mut().rax = 0xFFFFFFFF;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "WSCInstallProviderAndChains" => {
            let err_ptr = emu.regs().rcx;
            log_red!(
                emu,
                "ws2_32!WSCInstallProviderAndChains ptr:0x{:x}",
                err_ptr
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "WsaSocketA" => {
            let af = emu.regs().rcx as u32;
            let sock_type = emu.regs().rdx as u32;
            let protocol = emu.regs().r8 as u32;
            let sock = helper::socket_create();
            log_red!(
                emu,
                "ws2_32!WsaSocketA af:{} type:{} proto:{} => 0x{:x}",
                af,
                sock_type,
                protocol,
                sock
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = sock;
        }
        "WsaStartup" => {
            let version = emu.regs().rcx as u32;
            let wsa_data_ptr = emu.regs().rdx;
            log_red!(
                emu,
                "ws2_32!WsaStartup ver:0x{:x} data:0x{:x}",
                version,
                wsa_data_ptr
            );
            if wsa_data_ptr != 0 {
                emu.maps.write_word(wsa_data_ptr, version as u16);
                emu.maps.write_word(wsa_data_ptr + 2, version as u16);
            }
            emu.regs_mut().rax = 0;
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
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

pub fn getaddrinfo(emu: &mut emu::Emu) {
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
    let addrinfo_addr =
        match heap_management.allocate((addrinfo_size + sockaddr_in_size + 100) as usize) {
            Some(addr) => addr,
            None => {
                log::warn!("ws2_32!getaddrinfo: heap allocation failed");
                emu.regs_mut().rax = 1;
                return;
            }
        };
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
    emu.maps.write_qword(addrinfo_addr + 16, sockaddr_in_size); // ai_addrlen
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

pub fn WsaStartup(emu: &mut emu::Emu) {
    let version = emu.regs().rcx as u16;
    let wsadata_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSAStartup version:0x{:x} data:0x{:x}",
        version,
        wsadata_ptr
    );
    if wsadata_ptr != 0 {
        emu.maps.write_word(wsadata_ptr, version);
        emu.maps.write_word(wsadata_ptr + 2, version);
        emu.maps.write_string(wsadata_ptr + 4, "WinSock 2.0");
        emu.maps.write_string(wsadata_ptr + 4 + 257, "Running");
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129, 0x0200);
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129 + 2, 0x0101);
    }
    emu.regs_mut().rax = 0;
}

fn WsaSocketA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WsaSocketA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );

    emu.regs_mut().rax = helper::socket_create();
}

pub fn socket(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!socket arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );

    emu.regs_mut().rax = helper::socket_create();
}

fn WsaHtons(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let host_port = emu.regs().rdx as u16;
    let out_port = emu.regs().r8;

    let net_port = host_port.to_be();
    log_red!(emu, "ws2_32!WsaHtons {} -> 0x{:x}", host_port, net_port);

    if out_port != 0 {
        emu.maps.write_word(out_port, net_port);
    }
    emu.regs_mut().rax = 0;
}

pub fn htons(emu: &mut emu::Emu) {
    let port: u16 = emu.regs().rcx as u16;

    log_red!(emu, "ws2_32!htons port: {}", port);

    emu.regs_mut().rax = port.to_be() as u64;
}

pub fn inet_addr(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let ip_str = emu.maps.read_string(addr);

    log_red!(emu, "ws2_32!inet_addr '{}'", ip_str);

    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() == 4
        && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
            parts[0].parse::<u8>(),
            parts[1].parse::<u8>(),
            parts[2].parse::<u8>(),
            parts[3].parse::<u8>(),
        )
    {
        let result = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
        emu.regs_mut().rax = result as u64;
        return;
    }
    emu.regs_mut().rax = 0xFFFFFFFF; // INADDR_NONE
}

pub fn connect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let sockaddr_ptr = emu.regs().rdx;
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
        emu.count_send += 1;
        if emu.count_send > 3 {
            len = 0; // finish the send loop
        }

        emu.regs_mut().rax = len;
    }
}

pub fn bind(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let len = emu.regs().r8;

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

    if !helper::socket_exist(sock) {
        log::trace!("\tbad socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn listen(emu: &mut emu::Emu) {
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

pub fn accept(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let addrlen = emu.regs().r8;

    log_red!(emu, "ws2_32!accept  sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = !0u64; // INVALID_SOCKET
    } else {
        let new_sock = helper::socket_create();
        if saddr != 0 && addrlen != 0 {
            // Write a dummy sockaddr_in for the accepted connection
            emu.maps.write_word(saddr, 2); // AF_INET
            emu.maps.write_word(saddr + 2, 0x5000); // port 80 in network order
            emu.maps.write_dword(saddr + 4, 0x0100007f); // 127.0.0.1
            emu.maps.write_dword(addrlen, 16);
        }
        emu.regs_mut().rax = new_sock;
    }
}

pub fn closesocket(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.regs_mut().rax = 0;
}

pub fn setsockopt(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let level = emu.regs().rdx;
    let optname = emu.regs().r8;
    let optval = emu.regs().r9;
    let optlen = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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

pub fn getsockopt(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let level = emu.regs().rdx;
    let optname = emu.regs().r8;
    let optval = emu.regs().r9;
    let optlen = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
    let addrlen = emu.regs().r8;

    log_red!(emu, "ws2_32!WsaAccept  sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = !0u64; // INVALID_SOCKET
    } else {
        let new_sock = helper::socket_create();
        if saddr != 0 && addrlen != 0 {
            emu.maps.write_word(saddr, 2); // AF_INET
            emu.maps.write_word(saddr + 2, 0x5000); // port 80 network order
            emu.maps.write_dword(saddr + 4, 0x0100007f); // 127.0.0.1
        }
        emu.regs_mut().rax = new_sock;
    }
}

fn GetSockName(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let sockaddr_ptr = emu.regs().rdx;
    let namelen_ptr = emu.regs().r8;

    log_red!(emu, "ws2_32!GetSockName sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        emu.regs_mut().rax = !0u64; // SOCKET_ERROR
        return;
    }

    if sockaddr_ptr != 0 {
        emu.maps.write_word(sockaddr_ptr, 2); // AF_INET
        emu.maps.write_word(sockaddr_ptr + 2, 0); // port 0
        emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f); // 127.0.0.1
    }
    if namelen_ptr != 0 {
        emu.maps.write_dword(namelen_ptr, 16); // sizeof(sockaddr_in)
    }
    emu.regs_mut().rax = 0;
}

pub fn gethostbyname(emu: &mut emu::Emu) {
    let domain_name_ptr = emu.regs().rcx;
    let domain_name = emu.maps.read_string(domain_name_ptr);

    log_red!(emu, "ws2_32!gethostbyname `{}`", domain_name);

    let addr = match emu.maps.alloc(1024) {
        Some(a) => a,
        None => {
            log::error!("ws2_32!gethostbyname: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let str_addr = addr + 1024 - 100;
    let mem = match emu
        .maps
        .create_map("hostent", addr, 1024, Permission::READ_WRITE)
    {
        Ok(m) => m,
        Err(e) => {
            log::error!("ws2_32!gethostbyname: cannot create hostent map: {}", e);
            emu.regs_mut().rax = 0;
            return;
        }
    };
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

fn sendto(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buf = emu.regs().rdx;
    let len = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    log_red!(emu, "ws2_32!sendto sock:0x{:x} len:{}", sock, len);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = len;
    }
}

fn recvfrom(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buf = emu.regs().rdx;
    let _len = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    log_red!(emu, "ws2_32!recvfrom sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn WsaRecv(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buffers = emu.regs().rdx;
    let _count = emu.regs().r8;
    let bytes_recv_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSARecv sock:0x{:x}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}

fn WsaRecvFrom(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buffers = emu.regs().rdx;
    let _count = emu.regs().r8;
    let bytes_recv_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSARecvFrom sock:0x{:x}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}

fn WsaConnect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _name = emu.regs().rdx;
    let _namelen = emu.regs().r8 as u32;
    log_red!(emu, "ws2_32!WSAConnect sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn WsaSend(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buffers = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let bytes_sent_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSASend sock:0x{:x} bufs:{} ", sock, count);
    let total: u32 = if buffers != 0 && count > 0 {
        let mut sum = 0u32;
        for i in 0..count as u64 {
            let len = emu.maps.read_dword(buffers + i * 16).unwrap_or(0);
            sum += len;
        }
        sum
    } else {
        0
    };
    if bytes_sent_ptr != 0 {
        emu.maps.write_dword(bytes_sent_ptr, total);
    }
    emu.regs_mut().rax = 0;
}
