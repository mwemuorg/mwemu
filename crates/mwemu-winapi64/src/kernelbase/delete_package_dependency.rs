use crate::emu;
use crate::winapi::helper;

pub fn DeletePackageDependency(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(
        emu,
        "kernelbase!DeletePackageDependency handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
}
