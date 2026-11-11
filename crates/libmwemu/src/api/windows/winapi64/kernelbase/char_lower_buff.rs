use crate::emu;

pub fn CharLowerBuffA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as usize;
    log_red!(emu, "kernelbase!CharLowerBuffA buf:0x{:x} len:{}", buf, len);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    if buf != 0 && len > 0 {
        let count = len.min(0x10000);
        for i in 0..count {
            let b = emu.maps.read_byte(buf + i as u64).unwrap_or(0);
            if b.is_ascii_uppercase() {
                emu.maps.write_byte(buf + i as u64, b + 32);
            }
        }
        emu.regs_mut().rax = count as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn CharLowerBuffW(emu: &mut emu::Emu) {
    let lpsz = emu.regs().rcx; // Buffer pointer (LPWSTR)
    let cch_length = emu.regs().rdx; // Length in characters (DWORD)

    log_red!(
        emu,
        "kernelbase!CharLowerBuffW lpsz: 0x{:x} cchLength: {}",
        lpsz,
        cch_length
    );

    if lpsz == 0 || cch_length == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let mut processed_count = 0;

    // Process each character in the buffer
    for i in 0..cch_length {
        let char_addr = lpsz + (i * 2); // Each wide character is 2 bytes

        if let Some(wide_char) = emu.maps.read_word(char_addr) {
            // Convert UTF-16 code unit to char for processing
            if let Some(unicode_char) = char::from_u32(wide_char as u32) {
                // Convert to lowercase
                let lowercase_char = unicode_char.to_lowercase().next().unwrap_or(unicode_char);

                // Convert back to UTF-16 code unit
                let mut utf16_buf = [0u16; 2];
                let utf16_encoded = lowercase_char.encode_utf16(&mut utf16_buf);

                // Write the lowercase character back (take first code unit for BMP characters)
                let lowercase_code_unit = utf16_encoded[0];
                emu.maps.write_word(char_addr, lowercase_code_unit);

                processed_count += 1;
            } else {
                // Invalid Unicode, but still count as processed
                processed_count += 1;
            }
        } else {
            // Couldn't read memory, break early
            break;
        }
    }

    log_red!(
        emu,
        "CharLowerBuffW processed {} characters",
        processed_count
    );

    // Return the number of characters processed
    emu.regs_mut().rax = processed_count;
}
