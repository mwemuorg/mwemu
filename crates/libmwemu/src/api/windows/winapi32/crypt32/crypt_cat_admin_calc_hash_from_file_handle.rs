use crate::emu;

pub fn CryptCATAdminCalcHashFromFileHandle(emu: &mut emu::Emu) {
    let h_file = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pcb_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pb_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle 0x{:x}",
        h_file
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
