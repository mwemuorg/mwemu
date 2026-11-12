use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn VirtualLock(emu: &mut emu::Emu) {
    let lp_address = WinApiAbi::arg(emu, 0);
    let dw_size = WinApiAbi::arg(emu, 1);

    log_red!(
        emu,
        "kernel32!VirtualLock addr: 0x{:x} size: 0x{:x}",
        lp_address,
        dw_size
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    // Return TRUE
    emu.regs_mut().rax = 1;
}
