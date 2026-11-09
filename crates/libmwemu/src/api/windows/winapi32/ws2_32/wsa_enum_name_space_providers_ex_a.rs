use crate::emu;

pub fn WSAEnumNameSpaceProvidersExA(emu: &mut emu::Emu) {
    let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAEnumNameSpaceProvidersExA");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
