use super::trace_syscall32;
use crate::api::abi::unix::LinuxSyscall;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::*;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::unsupported;
use crate::winapi::helper;

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

fn trace_legacy_syscall32(emu: &mut emu::Emu, name: &str) {
    unsupported(emu, name);
}

fn trace_bad_syscall32(emu: &mut emu::Emu, nr: u64) {
    trace_syscall32(emu, &format!("bad_nr(0x{nr:x})"));
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

fn dispatch_legacy_syscall32(emu: &mut emu::Emu) {
    match emu.regs().get_eax() {
        19 => {
            let fd = emu.regs().get_ebx();
            unsupported_syscall32_args(emu, "lseek", &[("fd", fd.to_string())]);
        }

        21 => unsupported(emu, "mount"),

        22 => unsupported(emu, "umount"),

        23 => unsupported(emu, "setuid"),

        25 => unsupported(emu, "stime"),

        26 => unsupported(emu, "ptrace"),

        27 => unsupported(emu, "alarm"),

        28 => unsupported(emu, "oldfstat"),

        29 => unsupported(emu, "pause"),

        30 => unsupported(emu, "utime"),

        31 => unsupported(emu, "stty"),

        32 => unsupported(emu, "gtty"),

        33 => unsupported(emu, "access"),

        34 => unsupported(emu, "nice"),

        35 => unsupported(emu, "ftime"),

        36 => unsupported(emu, "sync"),

        37 => {
            let pid = emu.regs().get_ebx();
            let sig = emu.regs().get_ecx();
            unsupported_syscall32_args(
                emu,
                "kill",
                &[("pid", pid.to_string()), ("sig", sig.to_string())],
            );
        }

        38 => unsupported(emu, "rename"),

        39 => unsupported(emu, "mkdir"),

        40 => unsupported(emu, "rmdir"),

        41 => {
            let fd = emu.regs().get_ebx();
            unsupported_syscall32_args(emu, "dup", &[("fd", fd.to_string())]);
        }

        43 => unsupported(emu, "times"),

        44 => unsupported(emu, "prof"),

        45 => unsupported(emu, "brk"),

        46 => unsupported(emu, "setgid"),

        48 => unsupported(emu, "signal"),

        51 => unsupported(emu, "acct"),

        52 => unsupported(emu, "umount2"),

        53 => unsupported(emu, "lock"),

        54 => unsupported(emu, "ioctl"),

        56 => unsupported(emu, "mpx"),

        57 => unsupported(emu, "setpgid"),

        58 => unsupported(emu, "ulimit"),

        59 => unsupported(emu, "oldolduname"),

        60 => unsupported(emu, "umask"),

        61 => unsupported(emu, "chroot"),

        62 => unsupported(emu, "ustat"),

        63 => {
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

        65 => unsupported(emu, "getpgrp"),

        66 => unsupported(emu, "setsid"),

        67 => unsupported(emu, "sigaction"),

        68 => unsupported(emu, "sgetmask"),

        69 => unsupported(emu, "ssetmask"),

        70 => unsupported(emu, "setreuid"),

        71 => unsupported(emu, "setregid"),

        72 => unsupported(emu, "sigsuspend"),

        73 => unsupported(emu, "sigpending"),

        74 => unsupported(emu, "sethostname"),

        75 => unsupported(emu, "setrlimit"),

        76 => unsupported(emu, "getrlimit"),

        77 => unsupported(emu, "getrusage"),

        78 => unsupported(emu, "gettimeofday"),

        79 => unsupported(emu, "settimeofday"),

        81 => unsupported(emu, "setgroups"),

        82 => unsupported(emu, "select"),

        83 => unsupported(emu, "symlink"),

        84 => unsupported(emu, "oldlstat"),

        85 => unsupported(emu, "readlink"),

        86 => unsupported(emu, "uselib"),

        87 => unsupported(emu, "swapon"),

        88 => unsupported(emu, "reboot"),

        89 => unsupported(emu, "readdir"),

        90 => unsupported(emu, "mmap"),

        91 => unsupported(emu, "munmap"),

        92 => unsupported(emu, "truncate"),

        93 => unsupported(emu, "ftruncate"),

        94 => unsupported(emu, "fchmod"),

        95 => unsupported(emu, "fchown"),

        96 => unsupported(emu, "getpriority"),

        97 => unsupported(emu, "setpriority"),

        98 => unsupported(emu, "profil"),

        99 => unsupported(emu, "statfs"),

        100 => unsupported(emu, "fstatfs"),

        101 => unsupported(emu, "ioperm"),

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
                    let sock = helper::socket_create();
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

        103 => unsupported(emu, "syslog"),

        104 => unsupported(emu, "setitimer"),

        105 => unsupported(emu, "getitimer"),

        106 => unsupported(emu, "stat"),

        107 => unsupported(emu, "lstat"),

        108 => unsupported(emu, "fstat"),

        109 => unsupported(emu, "olduname"),

        110 => unsupported(emu, "iopl"),

        111 => unsupported(emu, "vhanghup"),

        112 => unsupported(emu, "idle"),

        113 => unsupported(emu, "vm86old"),

        114 => unsupported(emu, "wait4"),

        115 => unsupported(emu, "swapoff"),

        116 => unsupported(emu, "sysinfo"),

        117 => unsupported(emu, "ipc"),

        118 => unsupported(emu, "fsync"),

        119 => unsupported(emu, "sigreturn"),

        120 => unsupported(emu, "clone"),

        121 => unsupported(emu, "setdomainname"),

        123 => unsupported(emu, "modify_ltd"),

        124 => unsupported(emu, "adjtimex"),

        125 => unsupported(emu, "mprotect"),

        126 => unsupported(emu, "sigprocmask"),

        127 => unsupported(emu, "create_module"),

        128 => unsupported(emu, "init_module"),

        129 => unsupported(emu, "delete_module"),

        130 => unsupported(emu, "get_kernel_syms"),

        131 => unsupported(emu, "quotactl"),

        132 => unsupported(emu, "getpgid"),

        133 => unsupported(emu, "fchdir"),

        134 => unsupported(emu, "bdflush"),

        135 => unsupported(emu, "sysfs"),

        136 => unsupported(emu, "personality"),

        137 => unsupported(emu, "afs_syscall"),

        138 => unsupported(emu, "setfsuid"),

        139 => unsupported(emu, "setfsgid"),

        140 => unsupported(emu, "_llseek"),

        141 => unsupported(emu, "getdents"),

        142 => unsupported(emu, "_newselect"),

        143 => unsupported(emu, "flock"),

        144 => unsupported(emu, "msync"),

        145 => unsupported(emu, "readv"),

        147 => unsupported(emu, "getsid"),

        148 => unsupported(emu, "fdatasync"),

        149 => unsupported(emu, "_sysctl"),

        150 => unsupported(emu, "mlock"),

        151 => unsupported(emu, "munlock"),

        152 => unsupported(emu, "mlockall"),

        153 => unsupported(emu, "munlockall"),

        154 => unsupported(emu, "sched_setparam"),

        155 => unsupported(emu, "sched_getparam"),

        156 => unsupported(emu, "sched_setscheduler"),

        157 => unsupported(emu, "sched_getscheduler"),

        158 => unsupported(emu, "sched_yield"),

        159 => unsupported(emu, "sched_get_priority_max"),

        160 => unsupported(emu, "sched_get_priority_min"),

        161 => unsupported(emu, "sched_rr_get_inverval"),

        162 => unsupported(emu, "nanosleep"),

        163 => unsupported(emu, "mremap"),

        164 => unsupported(emu, "setresuid"),

        166 => unsupported(emu, "vm86"),

        167 => unsupported(emu, "query_module"),

        168 => unsupported(emu, "poll"),

        169 => unsupported(emu, "nfsservctrl"),

        170 => unsupported(emu, "setresgid"),

        172 => unsupported(emu, "prctl"),

        173 => unsupported(emu, "rt_sigreturn"),

        174 => unsupported(emu, "rt_sigcation"),

        175 => unsupported(emu, "rt_sigprocmask"),

        176 => unsupported(emu, "rt_sigpending"),

        177 => unsupported(emu, "rt_sigtimedwait"),

        178 => unsupported(emu, "rt_sigqueueinfo"),

        179 => unsupported(emu, "rt_sigsuspend"),

        180 => unsupported(emu, "pread64"),

        181 => unsupported(emu, "pwrite64"),

        182 => unsupported(emu, "chown"),

        183 => unsupported(emu, "getcwd"),

        184 => unsupported(emu, "capget"),

        185 => unsupported(emu, "capset"),

        186 => unsupported(emu, "sigaltstack"),

        187 => unsupported(emu, "sendfile"),

        188 => unsupported(emu, "getpmsg"),

        189 => unsupported(emu, "putpmsg"),

        190 => unsupported(emu, "vfork"),

        191 => unsupported(emu, "ugetrlimit"),

        192 => unsupported(emu, "mmap2"),

        193 => unsupported(emu, "truncate64"),

        194 => unsupported(emu, "ftruncate64"),

        195 => unsupported(emu, "stat64"),

        196 => unsupported(emu, "lstat64"),

        197 => unsupported(emu, "fstat64"),

        198 => unsupported(emu, "lchown32"),

        203 => unsupported(emu, "getreuid32"),

        204 => unsupported(emu, "getregid32"),

        206 => unsupported(emu, "setgroups32"),

        207 => unsupported(emu, "fchown32"),

        208 => unsupported(emu, "setresuid32"),

        210 => unsupported(emu, "setresgid32"),

        212 => unsupported(emu, "chown32"),

        213 => unsupported(emu, "setuid32"),

        214 => unsupported(emu, "setgid32"),

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
            if emu.regs().get_eax() >= SYS32_SYSCALL_NAMES.len() as u64 {
                trace_bad_syscall32(emu, emu.regs().get_eax());
            } else {
                trace_legacy_syscall32(emu, SYS32_SYSCALL_NAMES[emu.regs().get_eax() as usize]);
            }
            LinuxSyscall.set_return(emu, ENOSYS);
        }
    }
}
