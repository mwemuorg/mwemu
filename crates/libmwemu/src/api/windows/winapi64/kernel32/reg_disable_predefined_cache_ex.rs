
use crate::emu;
use crate::winapi::helper;
pub fn RegDisablePredefinedCacheEx(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!RegDisablePredefinedCacheEx hkey:0x{:x} param2:0x{:x} param3:0x{:x}",
        hkey,
        param2,
        param3
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
