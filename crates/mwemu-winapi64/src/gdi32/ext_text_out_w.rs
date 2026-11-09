use crate::emu;

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let options = emu.regs().r9 as u32;
    let _lprect = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let count = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let text = if text_ptr != 0 && count > 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!ExtTextOutW hdc:0x{:x} x:{} y:{} options:0x{:x} count:{} text:'{}'",
        hdc,
        x,
        y,
        options,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}
