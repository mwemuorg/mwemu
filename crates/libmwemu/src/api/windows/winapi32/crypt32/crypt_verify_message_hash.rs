use crate::emu;

pub fn CryptVerifyMessageHash(emu: &mut emu::Emu) {
    let p_hash_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_hashed_blob = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_hashed_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pb_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pcb_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptVerifyMessageHash 0x{:x}", p_hash_para);
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
