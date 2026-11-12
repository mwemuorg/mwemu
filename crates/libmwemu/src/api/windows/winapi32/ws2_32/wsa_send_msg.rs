use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSASendMsg(emu: &mut emu::Emu) {
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
    stdcall_ret(emu, 6);
}
