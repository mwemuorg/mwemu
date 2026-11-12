use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetComputerNameEx2W(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!SetComputerNameEx2W '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
