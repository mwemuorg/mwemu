use crate::emu;

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
