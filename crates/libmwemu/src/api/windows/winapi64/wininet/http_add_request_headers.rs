use crate::emu;

pub fn HttpAddRequestHeadersA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let headers_ptr = emu.regs().rdx;
    let headers_len = emu.regs().r8 as i32;
    let modifiers = emu.regs().r9 as u32;
    let hdrs = if headers_ptr != 0 {
        emu.maps.read_string(headers_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!HttpAddRequestHeadersA handle:0x{:x} hdrs:'{}' mod:0x{:x}",
        handle,
        hdrs,
        modifiers
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn HttpAddRequestHeadersW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let headers_ptr = emu.regs().rdx;
    let headers_len = emu.regs().r8 as i32;
    let modifiers = emu.regs().r9 as u32;
    let hdrs = if headers_ptr != 0 {
        emu.maps.read_wide_string(headers_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!HttpAddRequestHeadersW handle:0x{:x} hdrs:'{}' mod:0x{:x}",
        handle,
        hdrs,
        modifiers
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
