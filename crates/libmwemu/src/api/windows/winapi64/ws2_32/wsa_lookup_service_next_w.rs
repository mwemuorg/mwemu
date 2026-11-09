use crate::emu;

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    let lookup_handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSALookupServiceNextW handle:0x{:x}",
        lookup_handle
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
