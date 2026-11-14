use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WEP(emu: &mut emu::Emu) {
    let reason = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "ws2_32!WEP reason:{}", reason);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
