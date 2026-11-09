use crate::emu;

pub fn WSAGetServiceClassInfoA(emu: &mut emu::Emu) {
    let provider_id = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let class_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAGetServiceClassInfoA");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
