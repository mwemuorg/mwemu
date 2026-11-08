use crate::emu;

pub fn freeaddrinfo(emu: &mut emu::Emu) {
    let pai = emu.regs().rcx;
    log_red!(emu, "ws2_32!freeaddrinfo ptr:0x{:x}", pai);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if pai != 0 {
        let map_name = format!("addrinfo_{:x}", pai);
        emu.maps.free(&map_name);
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
