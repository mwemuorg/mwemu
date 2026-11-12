use crate::emu;
use crate::winapi::helper;

pub fn RegDisablePredefinedCache(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!RegDisablePredefinedCache arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
