use crate::emu;

pub fn CryptCATAdminCalcHashFromFileHandle(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let hash_size_ptr = emu.regs().rdx;
    let hash_ptr = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle file:0x{:x}",
        file_handle
    );
    if hash_size_ptr != 0 {
        emu.maps.write_dword(hash_size_ptr, 20);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
