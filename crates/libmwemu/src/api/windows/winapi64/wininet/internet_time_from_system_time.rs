use crate::emu;

pub fn InternetTimeFromSystemTime(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetTimeFromSystemTime arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetTimeFromSystemTimeA(emu: &mut emu::Emu) {
    let time_ptr = emu.regs().rcx;
    let format = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    log_red!(emu, "wininet!InternetTimeFromSystemTimeA format:{}", format);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buf_ptr != 0 {
        emu.maps
            .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetTimeFromSystemTimeW(emu: &mut emu::Emu) {
    let time_ptr = emu.regs().rcx;
    let format = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    log_red!(emu, "wininet!InternetTimeFromSystemTimeW format:{}", format);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buf_ptr != 0 {
        emu.maps
            .write_string(buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
