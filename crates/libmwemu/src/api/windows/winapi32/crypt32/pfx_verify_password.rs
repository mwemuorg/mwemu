use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn PFXVerifyPassword(emu: &mut emu::Emu) {
    let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!PFXVerifyPassword blob:0x{:x}", blob);
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.regs_mut().rax = 1;

    stdcall_ret(emu, 3);
}
