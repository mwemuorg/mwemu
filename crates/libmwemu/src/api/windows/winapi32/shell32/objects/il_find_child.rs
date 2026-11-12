use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ILFindChild(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "shell32!ILFindChild arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
