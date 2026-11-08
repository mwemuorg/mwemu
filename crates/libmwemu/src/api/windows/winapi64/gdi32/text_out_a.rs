use crate::emu;

pub fn TextOutA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let text_ptr = emu.regs().r9;
    let count = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let text = if text_ptr != 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!TextOutA hdc:0x{:x} x:{} y:{} count:{} text:'{}'",
        hdc,
        x,
        y,
        count,
        text
    );
    emu.regs_mut().rax = 1;
}
