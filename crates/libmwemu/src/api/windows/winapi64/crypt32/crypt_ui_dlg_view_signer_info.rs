use crate::emu;

pub fn CryptUIDlgViewSignerInfo(emu: &mut emu::Emu) {
    let psi = emu.regs().rcx;
    log_red!(
        emu,
        "crypt32!CryptUIDlgViewSignerInfo pSignerInfo:0x{:x}",
        psi
    );
    if psi != 0 && emu.maps.is_mapped(psi) {
        let _ = emu.maps.read_dword(psi).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
