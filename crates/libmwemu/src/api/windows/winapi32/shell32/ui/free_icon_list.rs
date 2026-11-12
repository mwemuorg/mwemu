use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn FreeIconList(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    helper::handler_close(handle);
    log_red!(emu, "shell32!FreeIconList handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
