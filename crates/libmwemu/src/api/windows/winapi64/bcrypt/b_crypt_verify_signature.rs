use crate::emu;
use crate::windows::constants;

pub fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    let _padding_info = emu.regs().rdx;
    let pb_hash = emu.regs().r8;
    let cb_hash = emu.regs().r9 as u32;
    let pb_signature = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let cb_signature = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;

    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) => valid",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature
    );
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
