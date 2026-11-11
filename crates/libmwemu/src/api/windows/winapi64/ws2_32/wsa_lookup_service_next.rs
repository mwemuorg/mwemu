use crate::emu;

pub fn WSALookupServiceNextA(emu: &mut emu::Emu) {
    let lookup_handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSALookupServiceNextA handle:0x{:x}",
        lookup_handle
    );
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
}

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    let lookup_handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSALookupServiceNextW handle:0x{:x}",
        lookup_handle
    );
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
