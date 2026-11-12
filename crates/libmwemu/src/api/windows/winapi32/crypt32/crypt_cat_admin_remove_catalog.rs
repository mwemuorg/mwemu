use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATAdminRemoveCatalog(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptCATAdminRemoveCatalog arg1:0x{:x}", arg1);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
