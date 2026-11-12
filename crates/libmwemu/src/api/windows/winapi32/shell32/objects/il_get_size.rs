use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ILGetSize(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "shell32!ILGetSize arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
