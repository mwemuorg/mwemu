use crate::emu;

pub fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    log_red!(emu, "oleaut32!SysFreeString  0x{:x}", str_ptr);

    if str_ptr == 0 {
        // NULL pointer - nothing to free (this is valid behavior)
        return;
    }

    // BSTR pointer points to string data, but allocation starts 4 bytes earlier (length prefix)
    let alloc_base = str_ptr - 4;

    // Read the length from the prefix to know how much to zero out
    if let Some(length_bytes) = emu.maps.read_dword(alloc_base) {
        let total_size = 4 + length_bytes as u64 + 2; // prefix + string + null terminator
        let string_length = length_bytes / 2; // Convert bytes to characters

        log_red!(
            emu,
            "SysFreeString zeroing {} bytes starting at 0x{:x} (string data was {} bytes, {} chars)",
            total_size,
            // Total allocation size
            alloc_base,
            // Base address
            length_bytes,
            // String data in bytes
            string_length
        );

        // Zero out the entire BSTR allocation
        for i in 0..total_size {
            emu.maps.write_byte(alloc_base + i, 0);
        }
    } else {
        // Guest handed us a BSTR pointer whose length prefix we can't read — a
        // garbage/hostile handle. Warn and no-op instead of crashing.
        log::warn!(
            "{} oleaut32!SysFreeString: unreadable length prefix at 0x{:x}, ignoring",
            emu.pos,
            alloc_base,
        );
    }

    // Optionally, you could also try to free the map by name:
    // emu.maps.free(&format!("alloc_{:x}", alloc_base));
    // or
    // emu.maps.free(&format!("bstr_{:x}", alloc_base));
}
