use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

macro_rules! syscall_name {
    ($name:literal) => {
        $name
    };
    ($name:ident) => {
        stringify!($name)
    };
}

macro_rules! syscall_names {
    ($($name:tt),* $(,)?) => {
        &[$(syscall_name!($name)),*]
    };
}

pub(super) fn dispatch(emu: &mut emu::Emu) {
    dispatch_legacy_syscall32(emu);
}

fn trace_simple_syscall32(emu: &mut emu::Emu, name: &str) {
    super::trace_syscall32(emu, &format!("{name}()"));
}

fn trace_socketcall32(emu: &mut emu::Emu, name: &str) {
    super::trace_syscall32(emu, &format!("socketcall {name}()"));
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

fn trace_syscall32_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    super::trace_syscall32(emu, &format!("{name}{}", format_trace_args(args)));
}

fn trace_socketcall32_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    super::trace_syscall32(
        emu,
        &format!("socketcall {name}{}", format_trace_args(args)),
    );
}

fn trace_legacy_syscall32(emu: &mut emu::Emu, name: &str) {
    trace_simple_syscall32(emu, name);
}

fn trace_bad_syscall32(emu: &mut emu::Emu, nr: u64) {
    super::trace_syscall32(emu, &format!("bad_nr(0x{nr:x})"));
}
fn dispatch_legacy_syscall32(emu: &mut emu::Emu) {
    match emu.regs().get_eax() {
        19 => {
            let fd = emu.regs().rbx;
            trace_syscall32_args(emu, "lseek", &[("fd", fd.to_string())]);
        }

        20 => trace_simple_syscall32(emu, "getpid"),

        21 => trace_simple_syscall32(emu, "mount"),

        22 => trace_simple_syscall32(emu, "umount"),

        23 => trace_simple_syscall32(emu, "setuid"),

        24 => trace_simple_syscall32(emu, "getuid"),

        25 => trace_simple_syscall32(emu, "stime"),

        26 => trace_simple_syscall32(emu, "ptrace"),

        27 => trace_simple_syscall32(emu, "alarm"),

        28 => trace_simple_syscall32(emu, "oldfstat"),

        29 => trace_simple_syscall32(emu, "pause"),

        30 => trace_simple_syscall32(emu, "utime"),

        31 => trace_simple_syscall32(emu, "stty"),

        32 => trace_simple_syscall32(emu, "gtty"),

        33 => trace_simple_syscall32(emu, "access"),

        34 => trace_simple_syscall32(emu, "nice"),

        35 => trace_simple_syscall32(emu, "ftime"),

        36 => trace_simple_syscall32(emu, "sync"),

        37 => {
            let pid = emu.regs().rbx;
            let sig = emu.regs().rcx;
            trace_syscall32_args(
                emu,
                "kill",
                &[("pid", pid.to_string()), ("sig", sig.to_string())],
            );
        }

        38 => trace_simple_syscall32(emu, "rename"),

        39 => trace_simple_syscall32(emu, "mkdir"),

        40 => trace_simple_syscall32(emu, "rmdir"),

        41 => {
            let fd = emu.regs().rbx;
            trace_syscall32_args(emu, "dup", &[("fd", fd.to_string())]);
        }

        42 => trace_simple_syscall32(emu, "pipe"),

        43 => trace_simple_syscall32(emu, "times"),

        44 => trace_simple_syscall32(emu, "prof"),

        45 => trace_simple_syscall32(emu, "brk"),

        46 => trace_simple_syscall32(emu, "setgid"),

        47 => trace_simple_syscall32(emu, "getgid"),

        48 => trace_simple_syscall32(emu, "signal"),

        49 => trace_simple_syscall32(emu, "geteuid"),

        50 => trace_simple_syscall32(emu, "getegid"),

        51 => trace_simple_syscall32(emu, "acct"),

        52 => trace_simple_syscall32(emu, "umount2"),

        53 => trace_simple_syscall32(emu, "lock"),

        54 => trace_simple_syscall32(emu, "ioctl"),

        55 => trace_simple_syscall32(emu, "fcntl"),

        56 => trace_simple_syscall32(emu, "mpx"),

        57 => trace_simple_syscall32(emu, "setpgid"),

        58 => trace_simple_syscall32(emu, "ulimit"),

        59 => trace_simple_syscall32(emu, "oldolduname"),

        60 => trace_simple_syscall32(emu, "umask"),

        61 => trace_simple_syscall32(emu, "chroot"),

        62 => trace_simple_syscall32(emu, "ustat"),

        63 => {
            let old_fd = emu.regs().get_ebx();
            let new_fd = emu.regs().get_ecx();
            trace_syscall32_args(
                emu,
                "dup2",
                &[
                    ("old_fd", old_fd.to_string()),
                    ("new_fd", new_fd.to_string()),
                ],
            );
        }

        64 => trace_simple_syscall32(emu, "getppid"),

        65 => trace_simple_syscall32(emu, "getpgrp"),

        66 => trace_simple_syscall32(emu, "setsid"),

        67 => trace_simple_syscall32(emu, "sigaction"),

        68 => trace_simple_syscall32(emu, "sgetmask"),

        69 => trace_simple_syscall32(emu, "ssetmask"),

        70 => trace_simple_syscall32(emu, "setreuid"),

        71 => trace_simple_syscall32(emu, "setregid"),

        72 => trace_simple_syscall32(emu, "sigsuspend"),

        73 => trace_simple_syscall32(emu, "sigpending"),

        74 => trace_simple_syscall32(emu, "sethostname"),

        75 => trace_simple_syscall32(emu, "setrlimit"),

        76 => trace_simple_syscall32(emu, "getrlimit"),

        77 => trace_simple_syscall32(emu, "getrusage"),

        78 => trace_simple_syscall32(emu, "gettimeofday"),

        79 => trace_simple_syscall32(emu, "settimeofday"),

        80 => trace_simple_syscall32(emu, "getgroups"),

        81 => trace_simple_syscall32(emu, "setgroups"),

        82 => trace_simple_syscall32(emu, "select"),

        83 => trace_simple_syscall32(emu, "symlink"),

        84 => trace_simple_syscall32(emu, "oldlstat"),

        85 => trace_simple_syscall32(emu, "readlink"),

        86 => trace_simple_syscall32(emu, "uselib"),

        87 => trace_simple_syscall32(emu, "swapon"),

        88 => trace_simple_syscall32(emu, "reboot"),

        89 => trace_simple_syscall32(emu, "readdir"),

        90 => trace_simple_syscall32(emu, "mmap"),

        91 => trace_simple_syscall32(emu, "munmap"),

        92 => trace_simple_syscall32(emu, "truncate"),

        93 => trace_simple_syscall32(emu, "ftruncate"),

        94 => trace_simple_syscall32(emu, "fchmod"),

        95 => trace_simple_syscall32(emu, "fchown"),

        96 => trace_simple_syscall32(emu, "getpriority"),

        97 => trace_simple_syscall32(emu, "setpriority"),

        98 => trace_simple_syscall32(emu, "profil"),

        99 => trace_simple_syscall32(emu, "statfs"),

        100 => trace_simple_syscall32(emu, "fstatfs"),

        101 => trace_simple_syscall32(emu, "ioperm"),

        102 => {
            match emu.regs().rbx as u32 {
                constants::SYS_SOCKET => {
                    let sock = helper::socket_create();
                    let fam = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("socket() cannot read family");
                    let typ = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("socket() cannot ready type");
                    let proto = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("socket() cannot read proto");

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

                constants::SYS_BIND => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("bind() cannot read sock");
                    let sockaddr = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("bind() cannot read sockaddr");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("bind() cannot read len");

                    let fam: u16 = emu
                        .maps
                        .read_word(sockaddr as u64)
                        .expect("cannot read family id");
                    let port: u16 = emu
                        .maps
                        .read_word((sockaddr + 2) as u64)
                        .expect("cannot read the port")
                        .to_be();
                    let ip: u32 = emu
                        .maps
                        .read_dword((sockaddr + 4) as u64)
                        .expect("cannot read the ip");
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
                        emu.regs_mut().rax = constants::ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                constants::SYS_CONNECT => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("connect() cannot read sock");
                    let sockaddr = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("connect() cannot read sockaddr");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("connect() cannot read len");

                    let fam: u16 = emu
                        .maps
                        .read_word(sockaddr as u64)
                        .expect("cannot read family id");
                    let port: u16 = emu
                        .maps
                        .read_word((sockaddr + 2) as u64)
                        .expect("cannot read the port")
                        .to_be();
                    let ip: u32 = emu
                        .maps
                        .read_dword((sockaddr + 4) as u64)
                        .expect("cannot read the ip");
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
                        emu.regs_mut().rax = constants::ENOTSOCK;
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

                constants::SYS_LISTEN => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("listen() cannot read sock");
                    let conns = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("listen() cannot read num of conns");

                    trace_socketcall32_args(
                        emu,
                        "listen",
                        &[("sock", sock.to_string()), ("backlog", conns.to_string())],
                    );

                    if !helper::socket_exist(sock as u64) {
                        log::trace!("\tbad socket/");
                        emu.regs_mut().rax = constants::ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                constants::SYS_ACCEPT => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("accept() cannot read sock");
                    let sockaddr = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("accept() cannot read sockaddr");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("accept() cannot read len");
                    let port: u16 = 8080;
                    let incoming_ip: u32 = 0x11223344;

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        emu.maps.write_word(sockaddr as u64, 0x0002);
                        emu.maps.write_word((sockaddr + 2) as u64, port.to_le()); //TODO: port should be the same than bind()
                        emu.maps.write_dword((sockaddr + 4) as u64, incoming_ip);
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
                        emu.regs_mut().rax = constants::ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = 0;
                    }
                }

                constants::SYS_GETSOCKNAME => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("getsockname() cannot read sock");
                    trace_socketcall32_args(emu, "getsockname", &[("sock", sock.to_string())]);
                    todo!("implement this");
                }

                constants::SYS_GETPEERNAME => {
                    trace_socketcall32(emu, "getpeername");
                }

                constants::SYS_SOCKETPAIR => {
                    trace_socketcall32(emu, "socketpair");
                }

                constants::SYS_SEND => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("send() cannot read sock");
                    let buf = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("send() cannot read buff");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("send() cannot read len");
                    let flags = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 12)
                        .expect("send() cannot read flags");

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
                        emu.regs_mut().rax = constants::ENOTSOCK;
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

                constants::SYS_RECV => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("recv() cannot read sock");
                    let buf = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("recv() cannot read buff");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("recv() cannot read len");
                    let flags = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 12)
                        .expect("recv() cannot read flags");

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
                        emu.regs_mut().rax = constants::ENOTSOCK;
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

                constants::SYS_SENDTO => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("sendto() cannot read sock");
                    let buf = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 4)
                        .expect("sendto() cannot read buff");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("sendto() cannot read len");
                    let flags = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 12)
                        .expect("sendto() cannot read flags");
                    let sockaddr = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 16)
                        .expect("sendto() cannot read sockaddr");
                    let addrlen = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 20)
                        .expect("sendto() cannot read addrlen");

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        let fam: u16 = emu
                            .maps
                            .read_word(sockaddr as u64)
                            .expect("cannot read family id");
                        let port: u16 = emu
                            .maps
                            .read_word((sockaddr + 2) as u64)
                            .expect("cannot read the port")
                            .to_be();
                        let ip: u32 = emu
                            .maps
                            .read_dword((sockaddr + 4) as u64)
                            .expect("cannot read the ip");
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
                        emu.regs_mut().rax = constants::ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = len as u64;
                    }
                }

                constants::SYS_RECVFROM => {
                    let sock = emu
                        .maps
                        .read_dword(emu.regs().get_esp())
                        .expect("recvfrom() cannot read sock");
                    let buf = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 8)
                        .expect("recvfrom() cannot read buff");
                    let len = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 12)
                        .expect("recvfrom() cannot read len");
                    let flags = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 16)
                        .expect("recvfrom() cannot read flags");
                    let sockaddr = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 20)
                        .expect("recvfrom() cannot read sockaddr");
                    let addrlen = emu
                        .maps
                        .read_dword(emu.regs().get_esp() + 24)
                        .expect("recvfrom() cannot read sockaddr len");

                    if sockaddr != 0 && emu.maps.is_mapped(sockaddr as u64) {
                        let port: u16 = 8080;
                        let incoming_ip: u32 = 0x11223344;

                        emu.maps.write_word(sockaddr as u64, 0x0002);
                        emu.maps.write_word((sockaddr + 2) as u64, port.to_le()); //TODO: port should be the same than bind()
                        emu.maps.write_dword((sockaddr + 4) as u64, incoming_ip);
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
                        emu.regs_mut().rax = constants::ENOTSOCK;
                    } else {
                        emu.regs_mut().rax = len as u64; //TODO: avoid loops
                    }
                }

                constants::SYS_SHUTDOWN => {
                    trace_socketcall32(emu, "shutdown");
                    //endpoint::sock_close();
                }

                constants::SYS_SETSOCKOPT => {
                    trace_socketcall32(emu, "setsockopt");
                }

                constants::SYS_GETSOCKOPT => {
                    trace_socketcall32(emu, "getsockopt");
                }

                constants::SYS_SENDMSG => {
                    trace_socketcall32(emu, "sendmsg");
                }

                constants::SYS_RECVMSG => {
                    trace_socketcall32(emu, "recvmsg");
                }

                constants::SYS_ACCEPT4 => {
                    trace_socketcall32(emu, "accept4");
                }

                constants::SYS_RECVMMSG => {
                    trace_socketcall32(emu, "recvmmsg");
                }

                constants::SYS_SENDMMSG => {
                    trace_socketcall32(emu, "sendmmsg");
                }

                _ => panic!("invalid socket call {} ", emu.regs().rbx),
            }
        }

        103 => trace_simple_syscall32(emu, "syslog"),

        104 => trace_simple_syscall32(emu, "setitimer"),

        105 => trace_simple_syscall32(emu, "getitimer"),

        106 => trace_simple_syscall32(emu, "stat"),

        107 => trace_simple_syscall32(emu, "lstat"),

        108 => trace_simple_syscall32(emu, "fstat"),

        109 => trace_simple_syscall32(emu, "olduname"),

        110 => trace_simple_syscall32(emu, "iopl"),

        111 => trace_simple_syscall32(emu, "vhanghup"),

        112 => trace_simple_syscall32(emu, "idle"),

        113 => trace_simple_syscall32(emu, "vm86old"),

        114 => trace_simple_syscall32(emu, "wait4"),

        115 => trace_simple_syscall32(emu, "swapoff"),

        116 => trace_simple_syscall32(emu, "sysinfo"),

        117 => trace_simple_syscall32(emu, "ipc"),

        118 => trace_simple_syscall32(emu, "fsync"),

        119 => trace_simple_syscall32(emu, "sigreturn"),

        120 => trace_simple_syscall32(emu, "clone"),

        121 => trace_simple_syscall32(emu, "setdomainname"),

        122 => trace_simple_syscall32(emu, "uname"),

        123 => trace_simple_syscall32(emu, "modify_ltd"),

        124 => trace_simple_syscall32(emu, "adjtimex"),

        125 => trace_simple_syscall32(emu, "mprotect"),

        126 => trace_simple_syscall32(emu, "sigprocmask"),

        127 => trace_simple_syscall32(emu, "create_module"),

        128 => trace_simple_syscall32(emu, "init_module"),

        129 => trace_simple_syscall32(emu, "delete_module"),

        130 => trace_simple_syscall32(emu, "get_kernel_syms"),

        131 => trace_simple_syscall32(emu, "quotactl"),

        132 => trace_simple_syscall32(emu, "getpgid"),

        133 => trace_simple_syscall32(emu, "fchdir"),

        134 => trace_simple_syscall32(emu, "bdflush"),

        135 => trace_simple_syscall32(emu, "sysfs"),

        136 => trace_simple_syscall32(emu, "personality"),

        137 => trace_simple_syscall32(emu, "afs_syscall"),

        138 => trace_simple_syscall32(emu, "setfsuid"),

        139 => trace_simple_syscall32(emu, "setfsgid"),

        140 => trace_simple_syscall32(emu, "_llseek"),

        141 => trace_simple_syscall32(emu, "getdents"),

        142 => trace_simple_syscall32(emu, "_newselect"),

        143 => trace_simple_syscall32(emu, "flock"),

        144 => trace_simple_syscall32(emu, "msync"),

        145 => trace_simple_syscall32(emu, "readv"),

        146 => trace_simple_syscall32(emu, "writev"),

        147 => trace_simple_syscall32(emu, "getsid"),

        148 => trace_simple_syscall32(emu, "fdatasync"),

        149 => trace_simple_syscall32(emu, "_sysctl"),

        150 => trace_simple_syscall32(emu, "mlock"),

        151 => trace_simple_syscall32(emu, "munlock"),

        152 => trace_simple_syscall32(emu, "mlockall"),

        153 => trace_simple_syscall32(emu, "munlockall"),

        154 => trace_simple_syscall32(emu, "sched_setparam"),

        155 => trace_simple_syscall32(emu, "sched_getparam"),

        156 => trace_simple_syscall32(emu, "sched_setscheduler"),

        157 => trace_simple_syscall32(emu, "sched_getscheduler"),

        158 => trace_simple_syscall32(emu, "sched_yield"),

        159 => trace_simple_syscall32(emu, "sched_get_priority_max"),

        160 => trace_simple_syscall32(emu, "sched_get_priority_min"),

        161 => trace_simple_syscall32(emu, "sched_rr_get_inverval"),

        162 => trace_simple_syscall32(emu, "nanosleep"),

        163 => trace_simple_syscall32(emu, "mremap"),

        164 => trace_simple_syscall32(emu, "setresuid"),

        165 => trace_simple_syscall32(emu, "getresuid"),

        166 => trace_simple_syscall32(emu, "vm86"),

        167 => trace_simple_syscall32(emu, "query_module"),

        168 => trace_simple_syscall32(emu, "poll"),

        169 => trace_simple_syscall32(emu, "nfsservctrl"),

        170 => trace_simple_syscall32(emu, "setresgid"),

        171 => trace_simple_syscall32(emu, "getresgid"),

        172 => trace_simple_syscall32(emu, "prctl"),

        173 => trace_simple_syscall32(emu, "rt_sigreturn"),

        174 => trace_simple_syscall32(emu, "rt_sigcation"),

        175 => trace_simple_syscall32(emu, "rt_sigprocmask"),

        176 => trace_simple_syscall32(emu, "rt_sigpending"),

        177 => trace_simple_syscall32(emu, "rt_sigtimedwait"),

        178 => trace_simple_syscall32(emu, "rt_sigqueueinfo"),

        179 => trace_simple_syscall32(emu, "rt_sigsuspend"),

        180 => trace_simple_syscall32(emu, "pread64"),

        181 => trace_simple_syscall32(emu, "pwrite64"),

        182 => trace_simple_syscall32(emu, "chown"),

        183 => trace_simple_syscall32(emu, "getcwd"),

        184 => trace_simple_syscall32(emu, "capget"),

        185 => trace_simple_syscall32(emu, "capset"),

        186 => trace_simple_syscall32(emu, "sigaltstack"),

        187 => trace_simple_syscall32(emu, "sendfile"),

        188 => trace_simple_syscall32(emu, "getpmsg"),

        189 => trace_simple_syscall32(emu, "putpmsg"),

        190 => trace_simple_syscall32(emu, "vfork"),

        191 => trace_simple_syscall32(emu, "ugetrlimit"),

        192 => trace_simple_syscall32(emu, "mmap2"),

        193 => trace_simple_syscall32(emu, "truncate64"),

        194 => trace_simple_syscall32(emu, "ftruncate64"),

        195 => trace_simple_syscall32(emu, "stat64"),

        196 => trace_simple_syscall32(emu, "lstat64"),

        197 => trace_simple_syscall32(emu, "fstat64"),

        198 => trace_simple_syscall32(emu, "lchown32"),

        199 => trace_simple_syscall32(emu, "getuid32"),

        200 => trace_simple_syscall32(emu, "getgid32"),

        201 => trace_simple_syscall32(emu, "geteuid32"),

        202 => trace_simple_syscall32(emu, "getegid32"),

        203 => trace_simple_syscall32(emu, "getreuid32"),

        204 => trace_simple_syscall32(emu, "getregid32"),

        205 => trace_simple_syscall32(emu, "getgrups32"),

        206 => trace_simple_syscall32(emu, "setgroups32"),

        207 => trace_simple_syscall32(emu, "fchown32"),

        208 => trace_simple_syscall32(emu, "setresuid32"),

        209 => trace_simple_syscall32(emu, "getresuid32"),

        210 => trace_simple_syscall32(emu, "setresgid32"),

        211 => trace_simple_syscall32(emu, "getresgid32"),

        212 => trace_simple_syscall32(emu, "chown32"),

        213 => trace_simple_syscall32(emu, "setuid32"),

        214 => trace_simple_syscall32(emu, "setgid32"),

        _ => {
            const SYS32_SYSCALL_NAMES: &[&str] = syscall_names![
                "restart_syscall",
                "exit",
                "fork",
                "read",
                "write",
                "open",
                "close",
                "waitpid",
                "creat",
                "link",
                "unlink",
                "execve",
                "chdir",
                "time",
                "mknod",
                "chmod",
                "lchown",
                "break",
                "oldstat",
                "lseek",
                "getpid",
                "mount",
                "umount",
                "setuid",
                "getuid",
                "stime",
                "ptrace",
                "alarm",
                "oldfstat",
                "pause",
                "utime",
                "stty",
                "gtty",
                "access",
                "nice",
                "ftime",
                "sync",
                "kill",
                "rename",
                "mkdir",
                "rmdir",
                "dup",
                "pipe",
                "times",
                "prof",
                "brk",
                "setgid",
                "getgid",
                "signal",
                "geteuid",
                "getegid",
                "acct",
                "umount2",
                "lock",
                "ioctl",
                "fcntl",
                "mpx",
                "setpgid",
                "ulimit",
                "oldolduname",
                "umask",
                "chroot",
                "ustat",
                "dup2",
                "getppid",
                "getpgrp",
                "setsid",
                "sigaction",
                "sgetmask",
                "ssetmask",
                "setreuid",
                "setregid",
                "sigsuspend",
                "sigpending",
                "sethostname",
                "setrlimit",
                "getrlimit",
                "getrusage",
                "gettimeofday",
                "settimeofday",
                "getgroups",
                "setgroups",
                "select",
                "symlink",
                "oldlstat",
                "readlink",
                "uselib",
                "swapon",
                "reboot",
                "readdir",
                "mmap",
                "munmap",
                "truncate",
                "ftruncate",
                "fchmod",
                "fchown",
                "getpriority",
                "setpriority",
                "profil",
                "statfs",
                "fstatfs",
                "ioperm",
                "socketcall",
                "syslog",
                "setitimer",
                "getitimer",
                "stat",
                "lstat",
                "fstat",
                "olduname",
                "iopl",
                "vhangup",
                "idle",
                "vm86old",
                "wait4",
                "swapoff",
                "sysinfo",
                "ipc",
                "fsync",
                "sigreturn",
                "clone",
                "setdomainname",
                "uname",
                "modify_ldt",
                "adjtimex",
                "mprotect",
                "sigprocmask",
                "create_module",
                "init_module",
                "delete_module",
                "get_kernel_syms",
                "quotactl",
                "getpgid",
                "fchdir",
                "bdflush",
                "sysfs",
                "personality",
                "afs_syscall",
                "setfsuid",
                "setfsgid",
                "_llseek",
                "getdents",
                "_newselect",
                "flock",
                "msync",
                "readv",
                "writev",
                "getsid",
                "fdatasync",
                "_sysctl",
                "mlock",
                "munlock",
                "mlockall",
                "munlockall",
                "sched_setparam",
                "sched_getparam",
                "sched_setscheduler",
                "sched_getscheduler",
                "sched_yield",
                "sched_get_priority_max",
                "sched_get_priority_min",
                "sched_rr_get_interval",
                "nanosleep",
                "mremap",
                "setresuid",
                "getresuid",
                "vm86",
                "query_module",
                "poll",
                "nfsservctl",
                "setresgid",
                "getresgid",
                "prctl",
                "rt_sigreturn",
                "rt_sigaction",
                "rt_sigprocmask",
                "rt_sigpending",
                "rt_sigtimedwait",
                "rt_sigqueueinfo",
                "rt_sigsuspend",
                "pread64",
                "pwrite64",
                "chown",
                "getcwd",
                "capget",
                "capset",
                "sigaltstack",
                "sendfile",
                "getpmsg",
                "putpmsg",
                "vfork",
                "ugetrlimit",
                "mmap2",
                "truncate64",
                "ftruncate64",
                "stat64",
                "lstat64",
                "fstat64",
                "lchown32",
                "getuid32",
                "getgid32",
                "geteuid32",
                "getegid32",
                "setreuid32",
                "setregid32",
                "getgroups32",
                "setgroups32",
                "fchown32",
                "setresuid32",
                "getresuid32",
                "setresgid32",
                "getresgid32",
                "chown32",
                "setuid32",
                "setgid32",
                "setfsuid32",
                "setfsgid32",
                "pivot_root",
                "mincore",
                "madvise",
                "getdents64",
                "fcntl64",
                "gettid",
                "readahead",
                "setxattr",
                "lsetxattr",
                "fsetxattr",
                "getxattr",
                "lgetxattr",
                "fgetxattr",
                "listxattr",
                "llistxattr",
                "flistxattr",
                "removexattr",
                "lremovexattr",
                "fremovexattr",
                "tkill",
                "sendfile64",
                "futex",
                "sched_setaffinity",
                "sched_getaffinity",
                "set_thread_area",
                "get_thread_area",
                "io_setup",
                "io_destroy",
                "io_getevents",
                "io_submit",
                "io_cancel",
                "fadvise64",
                "exit_group",
                "lookup_dcookie",
                "epoll_create",
                "epoll_ctl",
                "epoll_wait",
                "remap_file_pages",
                "set_tid_address",
                "timer_create",
                "timer_settime",
                "timer_gettime",
                "timer_getoverrun",
                "timer_delete",
                "clock_settime",
                "clock_gettime",
                "clock_getres",
                "clock_nanosleep",
                "statfs64",
                "fstatfs64",
                "tgkill",
                "utimes",
                "fadvise64_64",
                "vserver",
                "mbind",
                "get_mempolicy",
                "set_mempolicy",
                "mq_open",
                "mq_unlink",
                "mq_timedsend",
                "mq_timedreceive",
                "mq_notify",
                "mq_getsetattr",
                "kexec_load",
                "waitid",
                "add_key",
                "request_key",
                "keyctl",
                "ioprio_set",
                "ioprio_get",
                "inotify_init",
                "inotify_add_watch",
                "inotify_rm_watch",
                "migrate_pages",
                "openat",
                "mkdirat",
                "mknodat",
                "fchownat",
                "futimesat",
                "fstatat64",
                "unlinkat",
                "renameat",
                "linkat",
                "symlinkat",
                "readlinkat",
                "fchmodat",
                "faccessat",
                "pselect6",
                "ppoll",
                "unshare",
                "set_robust_list",
                "get_robust_list",
                "splice",
                "sync_file_range",
                "tee",
                "vmsplice",
                "move_pages",
                "getcpu",
                "epoll_pwait",
                "utimensat",
                "signalfd",
                "timerfd_create",
                "eventfd",
                "fallocate",
                "timerfd_settime",
                "timerfd_gettime",
                "signalfd4",
                "eventfd2",
                "epoll_create1",
                "dup3",
                "pipe2",
                "inotify_init1",
                "preadv",
                "pwritev",
                "rt_tgsigqueueinfo",
                "perf_event_open",
                "recvmmsg",
                "fanotify_init",
                "fanotify_mark",
                "prlimit64",
                "name_to_handle_at",
                "open_by_handle_at",
                "clock_adjtime",
                "syncfs",
                "sendmmsg",
                "setns",
                "process_vm_readv",
                "process_vm_writev",
                "kcmp",
                "finit_module",
                "sched_setattr",
                "sched_getattr",
                "renameat2",
                "seccomp",
                "getrandom",
                "memfd_create",
                "bpf",
                "execveat",
                "socket",
                "socketpair",
                "bind",
                "connect",
                "listen",
                "accept4",
                "getsockopt",
                "setsockopt",
                "getsockname",
                "getpeername",
                "sendto",
                "sendmsg",
                "recvfrom",
                "recvmsg",
                "shutdown",
                "userfaultfd",
                "membarrier",
                "mlock2",
                "copy_file_range",
                "preadv2",
                "pwritev2",
                "pkey_mprotect",
                "pkey_alloc",
                "pkey_free",
                "statx",
                "arch_prctl",
                "io_pgetevents",
                "rseq",
                "semget",
                "semctl",
                "shmget",
                "shmctl",
                "shmat",
                "shmdt",
                "msgget",
                "msgsnd",
                "msgrcv",
                "msgctl",
                "clock_gettime64",
                "clock_settime64",
                "clock_adjtime64",
                "clock_getres_time64",
                "clock_nanosleep_time64",
                "timer_gettime64",
                "timer_settime64",
                "timerfd_gettime64",
                "timerfd_settime64",
                "utimensat_time64",
                "pselect6_time64",
                "ppoll_time64",
                "io_pgetevents_time64",
                "recvmmsg_time64",
                "mq_timedsend_time64",
                "mq_timedreceive_time64",
                "semtimedop_time64",
                "rt_sigtimedwait_time64",
                "futex_time64",
                "sched_rr_get_interval_time64",
                "pidfd_send_signal",
                "io_uring_setup",
                "io_uring_enter",
                "io_uring_register",
                "open_tree",
                "move_mount",
                "fsopen",
                "fsconfig",
                "fsmount",
                "fspick",
                "pidfd_open",
                "clone3",
                "close_range",
                "openat2",
                "pidfd_getfd",
                "faccessat2",
                "process_madvise",
                "epoll_pwait2",
                "mount_setattr",
                "quotactl_fd",
                "landlock_create_ruleset",
                "landlock_add_rule",
                "landlock_restrict_self",
                "memfd_secret",
                "process_mrelease",
            ];
            if emu.regs().rax >= SYS32_SYSCALL_NAMES.len() as u64 {
                trace_bad_syscall32(emu, emu.regs().rax);
            } else {
                trace_legacy_syscall32(emu, SYS32_SYSCALL_NAMES[emu.regs().rax as usize]);
            }
        }
    }
}
