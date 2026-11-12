use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CloseTrace(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(handle);
    log_red!(emu, "advapi32!CloseTrace handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
