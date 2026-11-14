use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetDIBits(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    let arg3 = WinApiAbi::arg(emu, 2) as u32;
    let arg4 = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "gdi32!SetDIBits arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    stdcall_ret(emu, 7);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
