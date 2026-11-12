use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextColor(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);

    let arg1 = WinApiAbi::arg(emu, 0);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "gdi32!GetTextColor arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // black
    stdcall_ret(emu, 1);
}
