use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminReleaseContext(emu: &mut emu::Emu) {
    let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseContext h_cat_admin:0x{:x} dw_flags:0x{:x}",
        h_cat_admin,
        dw_flags
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
