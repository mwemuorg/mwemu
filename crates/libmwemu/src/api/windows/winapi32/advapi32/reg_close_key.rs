use crate::emu;
use crate::winapi::helper;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(hkey);
    log_red!(emu, "advapi32!RegCloseKey 0x{:x}", hkey);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
