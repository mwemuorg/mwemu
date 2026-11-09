use crate::emu;
use crate::winapi::helper;

pub fn WSASocketW(emu: &mut emu::Emu) {
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
