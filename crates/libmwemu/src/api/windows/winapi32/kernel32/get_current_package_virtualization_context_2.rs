use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetCurrentPackageVirtualizationContext(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "kernel32!GetCurrentPackageVirtualizationContext h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
