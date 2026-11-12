use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn inet_pton(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!inet_pton arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    stdcall_ret(emu, 3);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InetPtonW(emu: &mut emu::Emu, addr: u32) {
    let family = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let src_str = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!InetPtonW family:{} src:'{}'", family, src_str);
    if family == 2 && dst_ptr != 0 {
        let parts: Vec<&str> = src_str.split('.').collect();
        if parts.len() == 4 {
            if let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            ) {
                let addr = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
                emu.maps.write_dword(dst_ptr, addr);
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
