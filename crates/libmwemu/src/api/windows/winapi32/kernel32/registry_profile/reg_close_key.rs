use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hKey = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!RegCloseKey hkey: 0x{:x}", hKey);
    emu.stack_pop32(false);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
