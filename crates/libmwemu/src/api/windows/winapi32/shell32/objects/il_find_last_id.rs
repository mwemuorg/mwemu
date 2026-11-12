use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ILFindLastID(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "shell32!ILFindLastID arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
