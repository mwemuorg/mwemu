use crate::emu;

pub fn WSAEnumNameSpaceProvidersExW(emu: &mut emu::Emu) {
    let buf_len_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExW");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
