use crate::emu;
use crate::maps::mem64::Permission;

pub fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let char_count = emu.regs().rdx;

    log_red!(
        emu,
        ":{:x} oleaut32!SysAllocStringLen str_ptr: 0x{:x} size: {}",
        emu.regs().rip,
        str_ptr,
        char_count
    );

    // Calculate exact sizes like the real API
    let string_bytes = char_count * 2; // Requested characters in bytes
    let total_alloc_size = 4 + string_bytes + 2 + 16; // Length prefix + string + null terminator
    // the extra 16 byes are not allocated on the
    // real winapi, but it's needed to allo some
    // optimizations

    // Allocate memory (no extra padding needed)
    let bstr = match emu.maps.alloc(total_alloc_size) {
        Some(a) => a,
        None => {
            // A hostile length can request an absurd allocation; degrade to a
            // null BSTR instead of aborting the whole run.
            log::warn!(
                "oleaut32!SysAllocStringLen: out of memory for {} bytes",
                total_alloc_size
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_alloc_{:x}", bstr);
    emu.maps
        .create_map(&name, bstr, total_alloc_size, Permission::READ_WRITE);

    // Write length prefix (byte count of string data, excluding null terminator)
    emu.maps.write_dword(bstr, string_bytes as u32);

    if str_ptr == 0 {
        // Handle null input - zero out the string area
        for i in 0..char_count {
            emu.maps.write_word(bstr + 4 + (i * 2), 0);
        }
    } else {
        // Copy exactly char_count characters from input. A hostile/short source
        // pointer can leave a char unmapped: truncate there (like a real fault)
        // instead of panicking and killing the whole analysis.
        for i in 0..char_count {
            let char_addr = str_ptr + (i * 2);
            let wide_char = match emu.maps.read_word(char_addr) {
                Some(w) => w,
                None => {
                    log::warn!(
                        "oleaut32!SysAllocStringLen: unreadable source at 0x{:x}, truncating",
                        char_addr
                    );
                    break;
                }
            };
            emu.maps.write_word(bstr + 4 + (i * 2), wide_char);
        }
    }

    // Always write null terminator after the copied characters
    emu.maps.write_word(bstr + 4 + (char_count * 2), 0);

    let return_ptr = bstr + 4; // Return pointer to string data (after length prefix)

    log_red!(
        emu,
        "SysAllocStringLen returning: 0x{:x} (base: 0x{:x}, length_prefix: {})",
        return_ptr,
        bstr,
        string_bytes
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = return_ptr;
}
