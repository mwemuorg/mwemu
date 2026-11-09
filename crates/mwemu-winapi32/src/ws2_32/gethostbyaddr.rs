use crate::emu;

pub fn gethostbyaddr(emu: &mut emu::Emu) {
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let addr_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!gethostbyaddr ptr:0x{:x} len:{} type:{}",
        addr_ptr,
        len,
        addr_type
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
