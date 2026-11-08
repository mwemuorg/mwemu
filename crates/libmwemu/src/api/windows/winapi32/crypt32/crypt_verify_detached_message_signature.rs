use crate::emu;

pub fn CryptVerifyDetachedMessageSignature(emu: &mut emu::Emu) {
    let p_verify_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pb_detached_sign_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cb_detached_sign_blob = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let c_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let rgb_pb_to_be_signed = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptVerifyDetachedMessageSignature 0x{:x}",
        p_verify_para
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
