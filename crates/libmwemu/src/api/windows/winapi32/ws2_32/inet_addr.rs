use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn inet_addr(emu: &mut emu::Emu) {
    let addr_ptr = WinApiAbi::arg(emu, 0);
    let ip_str = emu.maps.read_string(addr_ptr);
    log_red!(emu, "ws2_32!inet_addr '{}'", ip_str);
    emu.stack_pop32(false);
    let mut result: u32 = 0xffff_ffff;
    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() == 4
        && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
            parts[0].parse::<u8>(),
            parts[1].parse::<u8>(),
            parts[2].parse::<u8>(),
            parts[3].parse::<u8>(),
        )
    {
        result = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
    }
    emu.regs_mut().rax = result as u64;
}
