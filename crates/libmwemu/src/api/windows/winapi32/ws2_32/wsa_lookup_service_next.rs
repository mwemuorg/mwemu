use crate::emu;

pub fn WSALookupServiceNextA(emu: &mut emu::Emu) {
    let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WSALookupServiceNextA handle:0x{:x}",
        lookup_handle
    );
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WSALookupServiceNextW handle:0x{:x}",
        lookup_handle
    );
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
