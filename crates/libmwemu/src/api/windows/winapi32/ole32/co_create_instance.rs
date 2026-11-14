use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn CoCreateInstance(emu: &mut emu::Emu) {
    let ppv = arg(emu, 4) as u64;
    log_red!(emu, "ole32!CoCreateInstance ppv: 0x{:x}", ppv);
    if ppv != 0 {
        emu.maps.write_dword(ppv, 0);
    }
    emu.regs_mut().rax = REGDB_E_CLASSNOTREG;
    stdcall_ret(emu, 5);
}
