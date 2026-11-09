use crate::emu;

pub(super) fn CryptBinaryToStringA(emu: &mut emu::Emu) {
    let data_ptr = emu.regs().rcx;
    let data_len = emu.regs().rdx;
    let flags = emu.regs().r8;
    let str_ptr = emu.regs().r9;
    let str_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_a(
        emu,
        data_ptr,
        data_len,
        flags,
        str_ptr,
        str_len_ptr,
    );
}
