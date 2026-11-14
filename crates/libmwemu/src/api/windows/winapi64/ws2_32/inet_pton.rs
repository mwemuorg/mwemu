use crate::emu;

pub fn inet_pton(emu: &mut emu::Emu, addr: u64) {
    let family = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let dst_ptr = emu.regs().r8;
    let src_str = if src_ptr != 0 {
        emu.maps.read_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!inet_pton family: {} src: '{}'",
        family,
        src_str
    );
    if family == 2 && dst_ptr != 0 {
        // AF_INET
        let parts: Vec<&str> = src_str.split('.').collect();
        if parts.len() == 4
            && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            )
        {
            let addr = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
            emu.maps.write_dword(dst_ptr, addr);
            emu.regs_mut().rax = 1; // success
        } else {
            emu.regs_mut().rax = 0; // invalid
        }
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn InetPtonW(emu: &mut emu::Emu, addr: u64) {
    let family = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let dst_ptr = emu.regs().r8;
    let src_str = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!InetPtonW family:{} src:'{}'", family, src_str);
    if family == 2 && dst_ptr != 0 {
        let parts: Vec<&str> = src_str.split('.').collect();
        if parts.len() == 4
            && let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            )
        {
            let addr = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
            emu.maps.write_dword(dst_ptr, addr);
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
}
