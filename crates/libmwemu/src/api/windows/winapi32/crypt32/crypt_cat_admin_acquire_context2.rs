use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminAcquireContext2(emu: &mut emu::Emu) {
    let ph_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pg_subsystem = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pwsz_hash_alg = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p_strong_hash_policy = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
