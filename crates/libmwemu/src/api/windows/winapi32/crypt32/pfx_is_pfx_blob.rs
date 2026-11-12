use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn PFXIsPFXBlob(emu: &mut emu::Emu) {
    let blob = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!PFXIsPFXBlob blob:0x{:x}", blob);
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
