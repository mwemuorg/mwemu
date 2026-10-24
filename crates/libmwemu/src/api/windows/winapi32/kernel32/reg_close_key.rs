use crate::emu;
use crate::windows::constants;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hKey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!RegCloseKey hkey: 0x{:x}", hKey);
    emu.stack_pop32(false);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
