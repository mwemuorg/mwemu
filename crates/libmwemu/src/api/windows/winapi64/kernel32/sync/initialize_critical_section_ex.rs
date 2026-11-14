use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionEx ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
