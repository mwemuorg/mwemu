use super::trace_syscall32;
use crate::api::abi::unix::LinuxSyscall;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::*;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::files::external_descriptor;
use crate::syscall::linux::unsupported;
use crate::winapi::helper;

fn trace_socketcall32(emu: &mut emu::Emu, name: &str) {
    trace_syscall32(emu, &format!("socketcall {name}()"));
}

fn format_trace_args(args: &[(&str, String)]) -> String {
    if args.is_empty() {
        return "()".to_string();
    }

    let mut rendered = String::from("(");
    for (idx, (label, value)) in args.iter().enumerate() {
        if idx > 0 {
            rendered.push(' ');
        }
        rendered.push_str(label);
        rendered.push_str(": ");
        rendered.push_str(value);
    }
    rendered.push(')');
    rendered
}

fn unsupported_syscall32_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    unsupported(emu, &format!("{name}{}", format_trace_args(args)));
}

fn trace_socketcall32_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    trace_syscall32(
        emu,
        &format!("socketcall {name}{}", format_trace_args(args)),
    );
}

fn socketcall_args(emu: &emu::Emu, call: u32, address: u32) -> Result<[u32; 6], u64> {
    let count = match call {
        SYS_LISTEN | SYS_SHUTDOWN => 2,
        SYS_SOCKET | SYS_BIND | SYS_CONNECT | SYS_ACCEPT | SYS_GETSOCKNAME | SYS_GETPEERNAME
        | SYS_SENDMSG | SYS_RECVMSG => 3,
        SYS_SOCKETPAIR | SYS_SEND | SYS_RECV | SYS_ACCEPT4 | SYS_SENDMMSG => 4,
        SYS_SETSOCKOPT | SYS_GETSOCKOPT | SYS_RECVMMSG => 5,
        SYS_SENDTO | SYS_RECVFROM => 6,
        _ => return Err(EINVAL),
    };
    let size = count * size_of::<u32>();
    if address.checked_add((size - 1) as u32).is_none()
        || !emu
            .maps
            .can_access_range(address as u64, size, Permission::READ)
    {
        return Err(EFAULT);
    }
    let mut args = [0; 6];
    for (i, arg) in args[..count].iter_mut().enumerate() {
        *arg = emu
            .maps
            .read_dword(address as u64 + (i * size_of::<u32>()) as u64)
            .ok_or(EFAULT)?;
    }
    Ok(args)
}

pub(super) fn dispatch(emu: &mut emu::Emu) {
    match emu.regs().get_eax() {
        NR32_LSEEK => {
            let fd = emu.regs().get_ebx();
            unsupported_syscall32_args(emu, "lseek", &[("fd", fd.to_string())]);
        }

        NR32_KILL => {
            let pid = emu.regs().get_ebx();
            let sig = emu.regs().get_ecx();
            unsupported_syscall32_args(
                emu,
                "kill",
                &[("pid", pid.to_string()), ("sig", sig.to_string())],
            );
        }

        NR32_DUP => {
            let fd = emu.regs().get_ebx();
            unsupported_syscall32_args(emu, "dup", &[("fd", fd.to_string())]);
        }

        NR32_DUP2 => {
            let old_fd = emu.regs().get_ebx();
            let new_fd = emu.regs().get_ecx();
            unsupported_syscall32_args(
                emu,
                "dup2",
                &[
                    ("old_fd", old_fd.to_string()),
                    ("new_fd", new_fd.to_string()),
                ],
            );
        }

        NR32_SOCKETCALL => {
            let [call, address, ..] = LinuxSyscall.args(emu);
            let call = call as u32;
            let args = match socketcall_args(emu, call, address as u32) {
                Ok(args) => args,
                Err(errno) => {
                    LinuxSyscall.set_return(emu, errno);
                    return;
                }
            };
            match call {
                SYS_SOCKET => {
                    let sock = external_descriptor(emu, None);
                    let [fam, typ, proto, ..] = args;

                    trace_socketcall32_args(
                        emu,
                        "socket",
                        &[
                            ("family", fam.to_string()),
                            ("type", typ.to_string()),
                            ("proto", proto.to_string()),
                            ("sock", sock.to_string()),
                        ],
                    );
                    emu.regs_mut().rax = sock;
                }

                SYS_BIND => {
                    let [sock, sockaddr, len, ..] = args;

                    let fam: u16 = emu.maps.read_word(sockaddr as u64).unwrap_or(0);
                    let port: u16 = emu.maps.read_word(sockaddr as u64 + 2).unwrap_or(0).to_be();
                    let ip: u32 = emu.maps.read_dword(sockaddr as u64 + 4).unwrap_or(0);
                    let sip = format!(
                        "{}.{}.{}.{}",
                        ip & 0xff,
                        (ip & 0xff00) >> 8,
                        (ip & 0xff0000) >> 16,
                        (ip & 0xff000000) >> 24
                    );

                    trace_socketcall32_args(
                        emu,
                        "bind",
                        &[
                            ("sock", sock.to_string()),
                            ("family", fam.to_string()),
                            ("addr", format!("{sip}:{port}")),
                            ("addr_len", len.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                SYS_CONNECT => {
                    let [sock, sockaddr, len, ..] = args;

                    let fam: u16 = emu.maps.read_word(sockaddr as u64).unwrap_or(0);
                    let port: u16 = emu.maps.read_word(sockaddr as u64 + 2).unwrap_or(0).to_be();
                    let ip: u32 = emu.maps.read_dword(sockaddr as u64 + 4).unwrap_or(0);
                    let sip = format!(
                        "{}.{}.{}.{}",
                        ip & 0xff,
                        (ip & 0xff00) >> 8,
                        (ip & 0xff0000) >> 16,
                        (ip & 0xff000000) >> 24
                    );

                    trace_socketcall32_args(
                        emu,
                        "connect",
                        &[
                            ("sock", sock.to_string()),
                            ("family", fam.to_string()),
                            ("addr", format!("{sip}:{port}")),
                            ("addr_len", len.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                        return;
                    }

                    /*
                    if emu.cfg.endpoint {
                        if endpoint::sock_connect(sip.as_str(), port) {
                            log::trace!("\tconnected to the endpoint.");
                        } else {
                            log::trace!("\tcannot connect. dont use -e");
                        }
                    }*/

                    emu.regs_mut().rax = 0;
                }

                SYS_LISTEN => {
                    let [sock, conns, ..] = args;

                    trace_socketcall32_args(
                        emu,
                        "listen",
                        &[("sock", sock.to_string()), ("backlog", conns.to_string())],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                SYS_ACCEPT => {
                    let [sock, sockaddr, len, ..] = args;
                    let port: u16 = 8080;
                    let incoming_ip: u32 = 0x11223344;

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        emu.maps.write_word(sockaddr as u64, 0x0002);
                        emu.maps.write_word(sockaddr as u64 + 2, port.to_le()); //TODO: port should be the same than bind()
                        emu.maps.write_dword(sockaddr as u64 + 4, incoming_ip);
                    }

                    trace_socketcall32_args(
                        emu,
                        "accept",
                        &[
                            ("sock", sock.to_string()),
                            ("addr", format!("0x{sockaddr:x}")),
                            ("addr_len", len.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                SYS_GETSOCKNAME => {
                    let [sock, ..] = args;
                    trace_socketcall32_args(emu, "getsockname", &[("sock", sock.to_string())]);
                    log::warn!("getsockname: stub, not implemented");
                }

                SYS_GETPEERNAME => {
                    trace_socketcall32(emu, "getpeername");
                }

                SYS_SOCKETPAIR => {
                    trace_socketcall32(emu, "socketpair");
                }

                SYS_SEND => {
                    let [sock, buf, len, flags, ..] = args;

                    trace_socketcall32_args(
                        emu,
                        "send",
                        &[
                            ("sock", sock.to_string()),
                            ("buf", buf.to_string()),
                            ("len", len.to_string()),
                            ("flags", flags.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                        return;
                    }

                    /*
                    if emu.cfg.endpoint {
                        let buffer = emu.maps.read_buffer(buf as u64, len as usize);
                        let n = endpoint::sock_send(&buffer);
                        log::trace!("\tsent {} bytes.", n);
                        emu.regs_mut().rax = n as u64;
                    } else {
                        emu.regs_mut().rax = len as u64;
                    }*/

                    emu.regs_mut().rax = len as u64;
                }

                SYS_RECV => {
                    let [sock, buf, len, flags, ..] = args;

                    trace_socketcall32_args(
                        emu,
                        "recv",
                        &[
                            ("sock", sock.to_string()),
                            ("buf", buf.to_string()),
                            ("len", len.to_string()),
                            ("flags", flags.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                        return;
                    }

                    /*
                    if emu.cfg.endpoint {
                        let mut rbuff: Vec<u8> = vec![0; len as usize];
                        let n = endpoint::sock_recv(&mut rbuff);
                        emu.maps.write_buffer(buf as u64, &rbuff);
                        log::trace!("\nreceived {} bytes from the endpoint.", n);
                        emu.regs_mut().rax = n as u64;
                    } else {
                        emu.regs_mut().rax = len as u64; //TODO: avoid loops
                    }*/

                    emu.regs_mut().rax = len as u64; //TODO: avoid loops
                }

                SYS_SENDTO => {
                    let [sock, buf, len, flags, sockaddr, addrlen] = args;

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        let fam: u16 = emu.maps.read_word(sockaddr as u64).unwrap_or(0);
                        let port: u16 =
                            emu.maps.read_word(sockaddr as u64 + 2).unwrap_or(0).to_be();
                        let ip: u32 = emu.maps.read_dword(sockaddr as u64 + 4).unwrap_or(0);
                        let sip = format!(
                            "{}.{}.{}.{}",
                            ip & 0xff,
                            (ip & 0xff00) >> 8,
                            (ip & 0xff0000) >> 16,
                            (ip & 0xff000000) >> 24
                        );

                        trace_socketcall32_args(
                            emu,
                            "sendto",
                            &[
                                ("sock", sock.to_string()),
                                ("buf", buf.to_string()),
                                ("len", len.to_string()),
                                ("flags", flags.to_string()),
                                ("family", fam.to_string()),
                                ("addr", format!("{sip}:{port}")),
                                ("addr_len", addrlen.to_string()),
                            ],
                        );
                    } else {
                        trace_socketcall32_args(
                            emu,
                            "sendto",
                            &[
                                ("sock", sock.to_string()),
                                ("buf", buf.to_string()),
                                ("len", len.to_string()),
                                ("flags", flags.to_string()),
                                ("addr", "bad_sockaddr".to_string()),
                                ("addr_len", addrlen.to_string()),
                            ],
                        );
                    }

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = len as u64;
                    }
                }

                SYS_RECVFROM => {
                    let [sock, buf, len, flags, sockaddr, addrlen] = args;

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        let port: u16 = 8080;
                        let incoming_ip: u32 = 0x11223344;

                        emu.maps.write_word(sockaddr as u64, 0x0002);
                        emu.maps.write_word(sockaddr as u64 + 2, port.to_le()); //TODO: port should be the same than bind()
                        emu.maps.write_dword(sockaddr as u64 + 4, incoming_ip);
                    }

                    trace_socketcall32_args(
                        emu,
                        "recvfrom",
                        &[
                            ("sock", sock.to_string()),
                            ("buf", buf.to_string()),
                            ("len", len.to_string()),
                            ("flags", flags.to_string()),
                            ("addr", format!("0x{sockaddr:x}")),
                            ("addr_len", addrlen.to_string()),
                        ],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = len as u64; //TODO: avoid loops
                    }
                }

                SYS_SHUTDOWN => {
                    trace_socketcall32(emu, "shutdown");
                    //endpoint::sock_close();
                }

                SYS_SETSOCKOPT => {
                    trace_socketcall32(emu, "setsockopt");
                }

                SYS_GETSOCKOPT => {
                    trace_socketcall32(emu, "getsockopt");
                }

                SYS_SENDMSG => {
                    trace_socketcall32(emu, "sendmsg");
                }

                SYS_RECVMSG => {
                    trace_socketcall32(emu, "recvmsg");
                }

                SYS_ACCEPT4 => {
                    trace_socketcall32(emu, "accept4");
                }

                SYS_RECVMMSG => {
                    trace_socketcall32(emu, "recvmmsg");
                }

                SYS_SENDMMSG => {
                    trace_socketcall32(emu, "sendmmsg");
                }

                _ => {
                    log::warn!("invalid socket call {}", emu.regs().get_ebx());
                }
            }
        }

        nr => match i386_syscall_name(nr) {
            Some(name) => unsupported(emu, name),
            None => {
                trace_syscall32(emu, &format!("bad_nr(0x{nr:x})"));
                LinuxSyscall.set_return(emu, ENOSYS);
            }
        },
    }
}
