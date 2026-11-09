use crate::emu;

pub(super) fn CryptStringToBinaryW(emu: &mut emu::Emu) {
    let string = emu.regs().rcx;
    let num_chars = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ptr = emu.regs().r9;
    let inout_sz = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_w(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
}
