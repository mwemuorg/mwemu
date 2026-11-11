use crate::emu;

pub fn WSAGetServiceClassNameByClassIdA(emu: &mut emu::Emu) {
    let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdA");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSAGetServiceClassNameByClassIdW(emu: &mut emu::Emu) {
    let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAGetServiceClassNameByClassIdW");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
