use crate::emu;

pub fn CLSIDFromString(emu: &mut emu::Emu) {
    let lpsz = emu.regs().rcx;
    let pclsid = emu.regs().rdx;

    let guid_text = if lpsz != 0 {
        emu.maps.read_wide_string(lpsz)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "ole32!CLSIDFromString '{}' -> 0x{:x}",
        guid_text,
        pclsid
    );

    // Parse "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}"
    let cleaned: String = guid_text
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();

    if cleaned.len() != 32 || pclsid == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0x800401F3; // CO_E_CLASSSTRING
        return;
    }

    let d1 = u32::from_str_radix(&cleaned[0..8], 16).unwrap_or(0);
    let d2 = u16::from_str_radix(&cleaned[8..12], 16).unwrap_or(0);
    let d3 = u16::from_str_radix(&cleaned[12..16], 16).unwrap_or(0);

    emu.maps.write_dword(pclsid, d1);
    emu.maps.write_word(pclsid + 4, d2);
    emu.maps.write_word(pclsid + 6, d3);

    for i in 0..8usize {
        let byte_val = u8::from_str_radix(&cleaned[16 + i * 2..18 + i * 2], 16).unwrap_or(0);
        emu.maps.write_byte(pclsid + 8 + i as u64, byte_val);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK (NOERROR)
}
