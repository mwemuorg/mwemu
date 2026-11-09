use crate::emu;
use crate::winapi::helper;

pub fn WSAHtons(emu: &mut emu::Emu) {
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
