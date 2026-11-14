use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CloseTrace(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    helper::handler_close(handle);
    log_red!(emu, "advapi32!CloseTrace handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
