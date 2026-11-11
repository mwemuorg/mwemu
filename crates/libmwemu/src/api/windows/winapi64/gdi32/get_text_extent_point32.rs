use crate::emu;

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    let text = if text_ptr != 0 && len > 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32); // cx
        emu.maps.write_dword(size + 4, 16); // cy
    }
    log_red!(
        emu,
        "gdi32!GetTextExtentPoint32A hdc:0x{:x} text:'{}' len:{} => {}x16",
        hdc,
        text,
        len,
        len * 7
    );
    emu.regs_mut().rax = 1;
}

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let len = emu.regs().r8;
    let size = emu.regs().r9;
    let text = if text_ptr != 0 && len > 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32);
        emu.maps.write_dword(size + 4, 16);
    }
    log_red!(
        emu,
        "gdi32!GetTextExtentPoint32W hdc:0x{:x} text:'{}' len:{} => {}x16",
        hdc,
        text,
        len,
        len * 7
    );
    emu.regs_mut().rax = 1;
}
