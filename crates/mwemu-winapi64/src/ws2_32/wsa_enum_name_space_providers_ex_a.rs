use crate::emu;

pub fn WSAEnumNameSpaceProvidersExA(emu: &mut emu::Emu) {
    let buf_len_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExA");
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
