use crate::emu;

pub fn GetTextMetricsA(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
        emu.maps.write_dword(tm + 28, 400); // tmWeight (FW_NORMAL)
        emu.maps.write_dword(tm + 32, 0); // tmOverhang
        emu.maps.write_dword(tm + 36, 96); // tmDigitizedAspectX
        emu.maps.write_dword(tm + 40, 96); // tmDigitizedAspectY
        emu.maps.write_byte(tm + 44, 0x20); // tmFirstChar
        emu.maps.write_byte(tm + 45, 0xff); // tmLastChar
        emu.maps.write_byte(tm + 46, 0x3f); // tmDefaultChar ('?')
        emu.maps.write_byte(tm + 47, 0x20); // tmBreakChar (' ')
        emu.maps.write_byte(tm + 48, 0); // tmItalic
        emu.maps.write_byte(tm + 49, 0); // tmUnderlined
        emu.maps.write_byte(tm + 50, 0); // tmStruckOut
        emu.maps.write_byte(tm + 51, 0); // tmPitchAndFamily
        emu.maps.write_byte(tm + 52, 0); // tmCharSet (ANSI_CHARSET)
    }
    log_red!(emu, "gdi32!GetTextMetricsA hdc:0x{:x} tm:0x{:x}", hdc, tm);
    emu.regs_mut().rax = 1;
}

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let tm = emu.regs().rdx;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
        emu.maps.write_dword(tm + 28, 400); // tmWeight (FW_NORMAL)
        emu.maps.write_dword(tm + 32, 0); // tmOverhang
        emu.maps.write_dword(tm + 36, 96); // tmDigitizedAspectX
        emu.maps.write_dword(tm + 40, 96); // tmDigitizedAspectY
        emu.maps.write_word(tm + 44, 0x0020); // tmFirstChar (WCHAR)
        emu.maps.write_word(tm + 46, 0xffff); // tmLastChar
        emu.maps.write_word(tm + 48, 0x003f); // tmDefaultChar ('?')
        emu.maps.write_word(tm + 50, 0x0020); // tmBreakChar (' ')
        emu.maps.write_byte(tm + 52, 0); // tmItalic
        emu.maps.write_byte(tm + 53, 0); // tmUnderlined
        emu.maps.write_byte(tm + 54, 0); // tmStruckOut
        emu.maps.write_byte(tm + 55, 0); // tmPitchAndFamily
        emu.maps.write_byte(tm + 56, 0); // tmCharSet (ANSI_CHARSET)
    }
    log_red!(emu, "gdi32!GetTextMetricsW hdc:0x{:x} tm:0x{:x}", hdc, tm);
    emu.regs_mut().rax = 1;
}
