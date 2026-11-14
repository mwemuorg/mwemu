use crate::emu;

pub fn IIDFromString(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let iid_ptr = emu.regs().rdx;
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let cleaned: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    if cleaned.len() == 32 && iid_ptr != 0 {
        let d1 = u32::from_str_radix(&cleaned[0..8], 16).unwrap_or(0);
        let d2 = u16::from_str_radix(&cleaned[8..12], 16).unwrap_or(0);
        let d3 = u16::from_str_radix(&cleaned[12..16], 16).unwrap_or(0);
        emu.maps.write_dword(iid_ptr, d1);
        emu.maps.write_word(iid_ptr + 4, d2);
        emu.maps.write_word(iid_ptr + 6, d3);
        for i in 0..8usize {
            let b = u8::from_str_radix(&cleaned[16 + i * 2..18 + i * 2], 16).unwrap_or(0);
            emu.maps.write_byte(iid_ptr + 8 + i as u64, b);
        }
        emu.regs_mut().rax = 0;
    } else if iid_ptr != 0 {
        emu.maps.write_dword(iid_ptr, 0);
        emu.maps.write_dword(iid_ptr + 4, 0);
        emu.maps.write_dword(iid_ptr + 8, 0);
        emu.maps.write_dword(iid_ptr + 12, 0);
        emu.regs_mut().rax = 0x800401F3; // CO_E_CLASSSTRING
    } else {
        emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
    }
    log_red!(emu, "ole32!IIDFromString '{}'", s);
}
