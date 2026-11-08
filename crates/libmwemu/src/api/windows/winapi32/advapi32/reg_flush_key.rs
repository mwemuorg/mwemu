use crate::emu;
use crate::winapi::helper;

pub fn RegFlushKey(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!RegFlushKey");
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
