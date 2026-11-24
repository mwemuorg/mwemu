use super::proc::handle_syscall64_exit;
use super::trace_syscall64;
use crate::api::abi::set_integer_return;
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux;
use crate::syscall::linux::constants::errors::*;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::constants::system::*;

use crate::winapi::helper; // TODO: why not winapi64 helper?

use std::fs as stdfs;
use std::path::Path;

use crate::windows::structures;

/// Unix file-metadata fields a Linux guest's `stat`/`statx` expects. We extract
/// them cross-platform so the Linux-syscall code also *compiles* on non-Unix
/// hosts (e.g. a Windows CI runner). On a non-Unix host the inode identity isn't
/// available, so those fields are zeroed — acceptable, since emulating Linux
/// binaries that rely on real host inodes from Windows is out of scope.
struct UnixMeta {
    dev: u64,
    ino: u64,
    nlink: u64,
    mode: u32,
    size: u64,
    blksize: u64,
    blocks: u64,
    atime: i64,
    ctime: i64,
    mtime: i64,
    rdev: u64,
}

#[cfg(unix)]
fn unix_meta(m: &stdfs::Metadata) -> UnixMeta {
    use std::os::unix::fs::MetadataExt;
    UnixMeta {
        dev: m.dev(),
        ino: m.ino(),
        nlink: m.nlink(),
        mode: m.mode(),
        size: m.size(),
        blksize: m.blksize(),
        blocks: m.blocks(),
        atime: m.atime(),
        ctime: m.ctime(),
        mtime: m.mtime(),
        rdev: m.rdev(),
    }
}

#[cfg(not(unix))]
fn unix_meta(m: &stdfs::Metadata) -> UnixMeta {
    UnixMeta {
        dev: 0,
        ino: 0,
        nlink: 1,
        mode: 0o100644,
        size: m.len(),
        blksize: 4096,
        blocks: m.len().div_ceil(512),
        atime: 0,
        ctime: 0,
        mtime: 0,
        rdev: 0,
    }
}

fn trace_simple_syscall64(emu: &mut emu::Emu, name: &str) {
    trace_syscall64(emu, &format!("{name}()"));
}

fn trace_socketcall64(emu: &mut emu::Emu, name: &str) {
    trace_syscall64(emu, &format!("socketcall {name}()"));
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

pub(super) fn trace_syscall64_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    trace_syscall64(emu, &format!("{name}{}", format_trace_args(args)));
}

fn trace_socketcall64_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    trace_syscall64(
        emu,
        &format!("socketcall {name}{}", format_trace_args(args)),
    );
}

pub(super) fn dispatch(emu: &mut emu::Emu) {
    match emu.regs().rax {
        NR64_EXIT_GROUP => {
            handle_syscall64_exit(emu);
        }

        // getrandom(buf, buflen, flags): fill the requested bytes and return
        // their count so the result stays within the caller's buffer.
        NR64_GETRANDOM => linux::getrandom(emu, Arch::X86_64),

        NR64_BIND => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            if sockaddr > 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).unwrap_or(0);
                let port: u16 = emu.maps.read_word(sockaddr + 2).unwrap_or(0).to_be();
                let ip: u32 = emu.maps.read_dword(sockaddr + 4).unwrap_or(0);
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
                    emu,
                    "bind",
                    &[
                        ("sock", sock.to_string()),
                        ("family", fam.to_string()),
                        ("addr", format!("{sip}:{port}")),
                        ("addr_len", len.to_string()),
                    ],
                );

                if !helper::socket_exist(sock) {
                    log::trace!("\tbad socket/");
                    emu.regs_mut().rax = ENOTSOCK;
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                trace_socketcall64_args(
                    emu,
                    "bind",
                    &[
                        ("sock", sock.to_string()),
                        ("addr", "bad_sockaddr".to_string()),
                        ("addr_len", len.to_string()),
                    ],
                );
                emu.regs_mut().rax = EINVAL;
            }
        }

        NR64_CONNECT => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            if sockaddr > 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).unwrap_or(0);
                let port: u16 = emu.maps.read_word(sockaddr + 2).unwrap_or(0).to_be();
                let ip: u32 = emu.maps.read_dword(sockaddr + 4).unwrap_or(0);
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
                    emu,
                    "connect",
                    &[
                        ("sock", sock.to_string()),
                        ("family", fam.to_string()),
                        ("addr", format!("{sip}:{port}")),
                        ("addr_len", len.to_string()),
                    ],
                );

                if !helper::socket_exist(sock) {
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
            } else {
                trace_socketcall64_args(
                    emu,
                    "connect",
                    &[
                        ("sock", sock.to_string()),
                        ("addr", "bad_sockaddr".to_string()),
                        ("addr_len", len.to_string()),
                    ],
                );
                emu.regs_mut().rax = EINVAL;
            }
        }

        NR64_LISTEN => {
            let sock = emu.regs().rdi;
            let conns = emu.regs().rsi;

            trace_socketcall64_args(
                emu,
                "listen",
                &[("sock", sock.to_string()), ("backlog", conns.to_string())],
            );

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = ENOTSOCK;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        NR64_ACCEPT => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            let port: u16 = 8080;
            let incoming_ip: u32 = 0x11223344;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                emu.maps.write_word(sockaddr, 0x0002);
                emu.maps.write_word(sockaddr + 2, port.to_le()); //TODO: port should be the same than bind()
                emu.maps.write_dword(sockaddr + 4, incoming_ip);
            }

            trace_socketcall64_args(
                emu,
                "accept",
                &[
                    ("sock", sock.to_string()),
                    ("addr", format!("0x{sockaddr:x}")),
                    ("addr_len", len.to_string()),
                ],
            );

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = ENOTSOCK;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        NR64_GETSOCKNAME => {
            let sock = emu.regs().rdi;
            trace_socketcall64_args(emu, "getsockname", &[("sock", sock.to_string())]);
            emu.regs_mut().rax = 0;
        }

        NR64_GETPEERNAME => {
            trace_socketcall64(emu, "getpeername");
            emu.regs_mut().rax = 0;
        }

        NR64_SOCKETPAIR => {
            trace_socketcall64(emu, "socketpair");
            emu.regs_mut().rax = 0;
        }

        /*constants::NR64_SEND => {
            let sock = emu.maps.read_dword(emu.regs().rsp).expect("send() cannot read sock");
            let buf = emu.maps.read_dword(emu.regs().rsp+4).expect("send() cannot read buff");
            let len = emu.maps.read_dword(emu.regs().rsp+8).expect("send() cannot read len");
            let flags = emu.maps.read_dword(emu.regs().rsp+12).expect("send() cannot read flags");

            log::trace!("{}** {} syscall socketcall send() sock: {} buff: {} len: {} {}", emu.colors.light_red, emu.pos, sock, buf, len, emu.colors.nc);

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
                return;
            }

            if emu.cfg.endpoint {
                let buffer = emu.maps.read_buffer(buf, len as usize);
                let n = endpoint::sock_send(&buffer);
                log::trace!("\tsent {} bytes.", n);
                emu.regs_mut().rax = n;
            } else {
                emu.regs_mut().rax = len;
            }
        }

        constants::NR64_RECV => {
            let sock = emu.maps.read_dword(emu.regs().rsp).expect("recv() cannot read sock");
            let buf = emu.maps.read_dword(emu.regs().rsp+4).expect("recv() cannot read buff");
            let len = emu.maps.read_dword(emu.regs().rsp+8).expect("recv() cannot read len");
            let flags = emu.maps.read_dword(emu.regs().rsp+12).expect("recv() cannot read flags");

            log::trace!("{}** {} syscall socketcall recv() sock: {} buff: {} len: {}  {}", emu.colors.light_red, emu.pos, sock, buf, len, emu.colors.nc);

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
                return;
            }

            if emu.cfg.endpoint {

                let mut rbuff:Vec<u8> = vec![0;len as usize];
                let n = endpoint::sock_recv(&mut rbuff);
                emu.maps.write_buffer(buf, &rbuff);
                log::trace!("\nreceived {} bytes from the endpoint.", n);
                emu.regs_mut().rax = n;

            } else {
                emu.regs_mut().rax = len; //TODO: avoid loops
            }
        }*/
        NR64_SENDTO => {
            let sock = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let len = emu.regs().rdx;
            let flags = emu.regs().r10;
            let sockaddr = emu.regs().r8;
            let addrlen = emu.regs().r9;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).unwrap_or(0);
                let port: u16 = emu.maps.read_word(sockaddr + 2).unwrap_or(0).to_be();
                let ip: u32 = emu.maps.read_dword(sockaddr + 4).unwrap_or(0);
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
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
                trace_socketcall64_args(
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

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = ENOTSOCK;
            } else {
                emu.regs_mut().rax = len;
            }
        }

        NR64_RECVFROM => {
            let sock = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let len = emu.regs().rdx;
            let flags = emu.regs().r10;
            let sockaddr = emu.regs().r8;
            let addrlen = emu.regs().r9;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                let port: u16 = 8080;
                let incoming_ip: u32 = 0x11223344;

                emu.maps.write_word(sockaddr, 0x0002);
                emu.maps.write_word(sockaddr + 2, port.to_le()); //TODO: port should be the same than bind()
                emu.maps.write_dword(sockaddr + 4, incoming_ip);
            }

            trace_socketcall64_args(
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

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = ENOTSOCK;
            } else {
                emu.regs_mut().rax = len; //TODO: avoid loops
            }
        }

        NR64_SHUTDOWN => {
            trace_socketcall64(emu, "shutdown");
            //endpoint::sock_close();
        }

        NR64_SETSOCKOPT => {
            trace_socketcall64(emu, "setsockopt");
        }

        NR64_GETSOCKOPT => {
            trace_socketcall64(emu, "getsockopt");
        }

        NR64_SENDMSG => {
            trace_socketcall64(emu, "sendmsg");
        }

        NR64_RECVMSG => {
            trace_socketcall64(emu, "recvmsg");
        }

        NR64_ACCEPT4 => {
            trace_socketcall64(emu, "accept4");
        }

        NR64_RECVMMSG => {
            trace_socketcall64(emu, "recvmmsg");
        }

        NR64_SENDMMSG => {
            trace_socketcall64(emu, "sendmmsg");
        }

        NR64_ARCH_PRCTL => {
            let mode = emu.regs().rdi;
            let ptr = emu.regs().rsi;
            emu.regs_mut().rax = 0;
            let mut op: String = "unimplemented operation".to_string();

            match mode {
                ARCH_SET_GS => {
                    op = "set gs".to_string();
                    emu.regs_mut().gs = ptr;
                }
                ARCH_SET_FS => {
                    op = "set fs".to_string();
                    // ARCH_SET_FS takes the new FS base *directly* in the second
                    // arg — it must NOT be dereferenced.
                    emu.regs_mut().fs = ptr;
                }
                ARCH_GET_FS => {
                    op = "get fs".to_string();
                    emu.maps.write_qword(ptr, emu.regs().fs);
                }
                ARCH_GET_GS => {
                    op = "get gs".to_string();
                    emu.maps.write_qword(ptr, emu.regs().gs);
                }
                _ => {}
            }

            trace_syscall64_args(emu, "arch_prctl", &[("op", op)]);
        }

        NR64_UNAME => linux::uname(emu, Arch::X86_64),

        NR64_ACCESS => {
            let filename = emu.maps.read_string(emu.regs().rdi);

            trace_syscall64_args(emu, "access", &[("path", filename.clone())]);

            if filename == "/etc/ld.so.preload" {
                emu.regs_mut().rax = ENOENT;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        NR64_MSYNC => linux::mappings::msync(emu, Arch::X86_64),
        NR64_MUNMAP => linux::mappings::munmap(emu, Arch::X86_64),
        NR64_MMAP => linux::mappings::mmap(emu, Arch::X86_64),

        NR64_FSTAT => {
            let fd = emu.regs().rdi;
            let stat_ptr = emu.regs().rsi;
            let mut stat = structures::Stat::fake();

            if helper::handler_exist(fd) {
                let filepath = helper::handler_get_uri(fd);
                let path = Path::new(&filepath);
                let Ok(metadata) = stdfs::metadata(path) else {
                    log::warn!("fstat: cannot stat file {:?}", path);
                    stat.save(stat_ptr, &mut emu.maps);
                    emu.regs_mut().rax = 0;
                    return;
                };
                // Real dev/ino are essential: ld.so identifies already-loaded
                // shared objects by (st_dev, st_ino); reusing constants would
                // make it treat every library as the same object.
                let um = unix_meta(&metadata);
                stat.dev = um.dev;
                stat.ino = um.ino;
                stat.nlink = um.nlink;
                stat.mode = um.mode;
                stat.size = um.size as i64;
                stat.blksize = um.blksize as i64;
                stat.blocks = um.blocks as i64;
            }

            if stat_ptr > 0 && emu.maps.is_mapped(stat_ptr) {
                println!("saving stat at 0x{:x}", stat_ptr);
                stat.save(stat_ptr, &mut emu.maps);
            }

            trace_syscall64_args(
                emu,
                "fstat",
                &[("fd", fd.to_string()), ("stat", format!("0x{stat_ptr:x}"))],
            );

            emu.regs_mut().rax = 0;
        }

        NR64_STAT => {
            let filename_ptr = emu.regs().rdi;
            let stat_ptr = emu.regs().rsi;
            let filename = emu.maps.read_string(filename_ptr);

            let stat = structures::Stat::fake();
            //let path = Path::new(&filename);
            //let metadata =
            //    stdfs::metadata(path).expect("this file should exist because was opened by kernel64");
            //let file_size = metadata.len();
            //stat.size = file_size as i64;
            if stat_ptr > 0 && emu.maps.is_mapped(stat_ptr) {
                stat.save(stat_ptr, &mut emu.maps);
            }

            trace_syscall64_args(
                emu,
                "stat",
                &[("path", filename), ("stat", format!("0x{stat_ptr:x}"))],
            );

            emu.regs_mut().rax = 0;
        }

        NR64_STATX => {
            let dirfd = emu.regs().rdi;
            let path_ptr = emu.regs().rsi;
            let _flags = emu.regs().r10;
            let buf = emu.regs().r8;
            let pathname = emu.maps.read_string(path_ptr);

            // Resolve the target path: absolute as-is; otherwise relative to the
            // directory fd opened earlier (e.g. ls's "." handle), or to the cwd.
            let fullpath = if pathname.starts_with('/') {
                pathname.clone()
            } else if helper::handler_exist(dirfd) {
                let dir = helper::handler_get_uri(dirfd);
                if pathname.is_empty() {
                    dir
                } else {
                    format!("{}/{}", dir.trim_end_matches('/'), pathname)
                }
            } else {
                pathname.clone()
            };

            let mut rax: u64 = 0;
            if buf > 0 && emu.maps.is_mapped(buf) {
                match stdfs::symlink_metadata(Path::new(&fullpath)) {
                    Ok(m) => {
                        let um = unix_meta(&m);
                        emu.maps.write_dword(buf, 0x000007ff); // stx_mask: all basic fields
                        emu.maps.write_dword(buf + 0x04, um.blksize as u32);
                        emu.maps.write_qword(buf + 0x08, 0); // stx_attributes
                        emu.maps.write_dword(buf + 0x10, um.nlink as u32);
                        // Report the emulated user (uid/gid 1000); resolved to a
                        // name via /etc/passwd "files" (the systemd userdb path is
                        // blocked at openat).
                        emu.maps.write_dword(buf + 0x14, 1000);
                        emu.maps.write_dword(buf + 0x18, 1000);
                        emu.maps.write_word(buf + 0x1c, um.mode as u16);
                        emu.maps.write_qword(buf + 0x20, um.ino);
                        emu.maps.write_qword(buf + 0x28, um.size);
                        emu.maps.write_qword(buf + 0x30, um.blocks);
                        emu.maps.write_qword(buf + 0x38, 0); // stx_attributes_mask
                        // atime / btime / ctime / mtime (tv_sec @ +0, tv_nsec @ +8)
                        emu.maps.write_qword(buf + 0x40, um.atime as u64);
                        emu.maps.write_qword(buf + 0x60, um.ctime as u64);
                        emu.maps.write_qword(buf + 0x70, um.mtime as u64);
                        let rdev = um.rdev;
                        emu.maps
                            .write_dword(buf + 0x80, ((rdev >> 8) & 0xfff) as u32);
                        emu.maps.write_dword(buf + 0x84, (rdev & 0xff) as u32);
                        let dev = um.dev;
                        emu.maps
                            .write_dword(buf + 0x88, ((dev >> 8) & 0xfff) as u32);
                        emu.maps.write_dword(buf + 0x8c, (dev & 0xff) as u32);
                    }
                    Err(_) => {
                        rax = (-2i64) as u64; // -ENOENT
                    }
                }
            }
            emu.regs_mut().rax = rax;
            trace_syscall64_args(
                emu,
                "statx",
                &[("path", fullpath), ("buf", format!("0x{buf:x}"))],
            );
        }

        NR64_READLINK => {
            let link_ptr = emu.regs().rdi;
            let buff = emu.regs().rsi;
            let buffsz = emu.regs().rdx;

            let link = emu.maps.read_string(link_ptr);
            trace_syscall64_args(
                emu,
                "readlink",
                &[
                    ("path", link.clone()),
                    ("buf", format!("0x{buff:x}")),
                    ("len", buffsz.to_string()),
                ],
            );
            let sym_link_dest = match stdfs::read_link(&link) {
                Ok(link) => link,
                Err(_) => {
                    emu.regs_mut().rax = 0xffffffffffffffff;
                    return;
                }
            };

            emu.maps.write_string(buff, sym_link_dest.to_str().unwrap());

            emu.regs_mut().rax = sym_link_dest.as_os_str().len() as u64;
        }

        // Partial process models for single-threaded emulation.
        NR64_TGKILL | NR64_TKILL => {
            emu.regs_mut().rax = 0;
            trace_simple_syscall64(emu, "tgkill");
        }
        NR64_RT_SIGPROCMASK | NR64_RT_SIGACTION => {
            emu.regs_mut().rax = 0;
            trace_simple_syscall64(emu, "rt_sig*");
        }
        NR64_SET_ROBUST_LIST | NR64_RSEQ | NR64_SCHED_GETAFFINITY | NR64_PRLIMIT64 => {
            emu.regs_mut().rax = 0;
            trace_simple_syscall64(emu, "ok0");
        }

        NR64_MPROTECT => linux::mappings::mprotect(emu, Arch::X86_64),

        NR64_GETUID | NR64_GETEUID | NR64_GETGID | NR64_GETEGID => linux::get_identity(emu),
        NR64_GETRESUID | NR64_GETRESGID => linux::get_res_ids(emu, Arch::X86_64, false),
        NR64_GETGROUPS => linux::get_groups(emu, Arch::X86_64, false),

        nr => {
            match x86_64_syscall_name(nr) {
                Some(name) => trace_simple_syscall64(emu, name),
                None => trace_syscall64(emu, &format!("bad_nr(0x{nr:x})")),
            }
            set_integer_return(emu, ENOSYS);
        }
    }
}
