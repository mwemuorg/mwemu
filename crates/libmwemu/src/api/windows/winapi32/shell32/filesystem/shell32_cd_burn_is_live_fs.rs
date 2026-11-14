use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHELL32_CDBurn_IsLiveFS(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "shell32!SHELL32_CDBurn_IsLiveFS arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;
}
