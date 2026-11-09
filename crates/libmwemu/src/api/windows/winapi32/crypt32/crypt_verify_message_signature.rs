use crate::emu;

pub fn CryptVerifyMessageSignature(emu: &mut emu::Emu) {
    let p_verify_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pb_signed_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cb_signed_blob = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let ppb_decoded = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pcb_decoded = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let pp_signer_cert = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptVerifyMessageSignature 0x{:x}",
        p_verify_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
