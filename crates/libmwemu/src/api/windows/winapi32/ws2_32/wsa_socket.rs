use crate::emu;
use crate::winapi::helper;

pub fn WSASocketA(emu: &mut emu::Emu) {
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
