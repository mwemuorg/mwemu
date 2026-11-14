use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn ioctlsocket(emu: &mut emu::Emu) {
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
