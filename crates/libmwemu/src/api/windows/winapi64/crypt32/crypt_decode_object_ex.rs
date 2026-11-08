use crate::emu;

pub(super) fn CryptDecodeObjectEx(emu: &mut emu::Emu) {
    let encoding_type = emu.regs().rcx;
    let struct_type = emu.regs().rdx;
    let encoded = emu.regs().r8;
    let cb_encoded = emu.regs().r9;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let decode_para = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let pv_struct_info = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let pcb_struct_info = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    log_red!(
        emu,
        "crypt32!CryptDecodeObjectEx enc_type: 0x{:x} struct_type: 0x{:x} encoded: 0x{:x} len: {} flags: 0x{:x}",
        encoding_type,
        struct_type,
        encoded,
        cb_encoded,
        flags
    );

    if pcb_struct_info != 0 {
        emu.maps.write_dword(pcb_struct_info, 0);
    }

    emu.regs_mut().rax = 1;
}
