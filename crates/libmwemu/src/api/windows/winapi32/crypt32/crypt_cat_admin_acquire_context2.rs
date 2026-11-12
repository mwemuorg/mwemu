use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATAdminAcquireContext2(emu: &mut emu::Emu) {
    let ph_cat_admin = WinApiAbi::arg(emu, 0);
    let pg_subsystem = WinApiAbi::arg(emu, 1);
    let pwsz_hash_alg = WinApiAbi::arg(emu, 2);
    let p_strong_hash_policy = WinApiAbi::arg(emu, 3);
    let dw_flags = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptCATAdminAcquireContext2 0x{:x}",
        ph_cat_admin
    );
    let hndl = helper::handler_create("CryptCATAdminAcquireContext2");
    if ph_cat_admin != 0 {
        emu.maps.write_dword(ph_cat_admin, hndl as u32);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
