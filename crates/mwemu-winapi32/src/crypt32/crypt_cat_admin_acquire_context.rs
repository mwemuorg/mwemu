use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminAcquireContext(emu: &mut emu::Emu) {
    let ph_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pg_subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATAdminAcquireContext ph_cat_admin:0x{:x} pg_subsystem:0x{:x} dw_flags:0x{:x}",
        ph_cat_admin,
        pg_subsystem,
        dw_flags
    );
    let hndl = helper::handler_create("CryptCATAdminAcquireContext");
    if ph_cat_admin != 0 {
        emu.maps.write_dword(ph_cat_admin, hndl as u32);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
