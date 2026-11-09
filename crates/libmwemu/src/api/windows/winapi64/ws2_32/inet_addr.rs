use crate::emu;

pub fn inet_addr(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let ip_str = emu.maps.read_string(addr);

    log_red!(emu, "ws2_32!inet_addr '{}'", ip_str);

    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() == 4
        && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
            parts[0].parse::<u8>(),
            parts[1].parse::<u8>(),
            parts[2].parse::<u8>(),
            parts[3].parse::<u8>(),
        )
    {
        let result = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
        emu.regs_mut().rax = result as u64;
        return;
    }
    emu.regs_mut().rax = 0xFFFFFFFF; // INADDR_NONE
}
