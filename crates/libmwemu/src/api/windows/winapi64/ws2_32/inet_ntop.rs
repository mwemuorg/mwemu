use crate::emu;

pub fn inet_ntop(emu: &mut emu::Emu) {
    let family = emu.regs().rcx as u32;
    let addr_ptr = emu.regs().rdx;
    let str_buf = emu.regs().r8;
    let str_len = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!inet_ntop family: {} addr: 0x{:x} buf: 0x{:x}",
        family,
        addr_ptr,
        str_buf
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if family == 2 && addr_ptr != 0 && str_buf != 0 {
        // AF_INET
        let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
        let a = ip & 0xFF;
        let b = (ip >> 8) & 0xFF;
        let c = (ip >> 16) & 0xFF;
        let d = (ip >> 24) & 0xFF;
        let s = format!("{}.{}.{}.{}", a, b, c, d);
        if (s.len() as u32) < str_len {
            emu.maps.write_string(str_buf, &s);
            emu.regs_mut().rax = str_buf;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn InetNtopW(emu: &mut emu::Emu) {
    let family = emu.regs().rcx as u32;
    let addr_ptr = emu.regs().rdx;
    let str_buf = emu.regs().r8;
    let str_len = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!InetNtopW family:{} addr:0x{:x} buf:0x{:x}",
        family,
        addr_ptr,
        str_buf
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if family == 2 && addr_ptr != 0 && str_buf != 0 {
        let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
        let a = ip & 0xFF;
        let b = (ip >> 8) & 0xFF;
        let c = (ip >> 16) & 0xFF;
        let d = (ip >> 24) & 0xFF;
        let s = format!("{}.{}.{}.{}", a, b, c, d);
        if (s.len() as u32) < str_len {
            emu.maps.write_wide_string(str_buf, &s);
            emu.regs_mut().rax = str_buf;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
}
