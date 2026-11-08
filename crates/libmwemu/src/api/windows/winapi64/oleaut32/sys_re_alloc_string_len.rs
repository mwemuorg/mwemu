use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::constants;

pub(super) fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let pbstr_ptr = emu.regs().rcx; // Pointer to BSTR*
    let psz = emu.regs().rdx; // Source string (can be NULL)
    let len = emu.regs().r8; // Length in characters

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen pbstr_ptr: 0x{:x} psz: 0x{:x} len: {}",
        pbstr_ptr,
        psz,
        len
    );

    // Check if pbstr_ptr is NULL
    if pbstr_ptr == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read the current BSTR pointer (might be NULL for first allocation)
    let old_bstr = emu.maps.read_qword(pbstr_ptr).unwrap_or(0);

    // Log old content if it exists
    if old_bstr != 0 {
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        if old_len_chars > 0 {
            let old_string = emu
                .maps
                .read_wide_string_n(old_bstr, old_len_chars as usize);
            log_red!(
                emu,
                "Old BSTR content: \"{}\" (length: {} chars)",
                old_string,
                old_len_chars
            );
        }
    }

    // Log new source string if provided
    if psz != 0 && len > 0 {
        let new_string = emu.maps.read_wide_string_n(psz, len as usize);
        log_red!(
            emu,
            "New source string: \"{}\" (length: {} chars)",
            new_string,
            len
        );
    }

    // Calculate allocation size
    let byte_len = len * 2; // Length in bytes (UTF-16)
    let total_alloc_size = 4 + byte_len + 2; // 4-byte prefix + string + null terminator

    // Always allocate new memory (simpler than trying to realloc)
    let new_base = match emu.maps.alloc(total_alloc_size + 100) {
        Some(a) => a,
        None => {
            log::warn!(
                "oleaut32!SysReAllocStringLen: out of memory for {} bytes",
                total_alloc_size + 100
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let name = format!("bstr_{:x}", new_base);
    emu.maps.create_map(
        &name,
        new_base,
        total_alloc_size + 100,
        Permission::READ_WRITE,
    );

    // Write length prefix (in bytes, not including null terminator)
    emu.maps.write_dword(new_base, byte_len as u32);

    // Copy data from source if provided
    if psz != 0 && len > 0 {
        emu.maps.memcpy(new_base + 4, psz, len as usize * 2);
    } else if old_bstr != 0 && len > 0 {
        // No new source provided, preserve existing data (truncated to new length)
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        let copy_len = std::cmp::min(len, old_len_chars as u64);

        if copy_len > 0 {
            emu.maps
                .memcpy(new_base + 4, old_bstr, copy_len as usize * 2);
        }

        // Zero out any remaining space if new length is longer than old length
        for i in copy_len..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    } else {
        // Initialize to zeros (empty string)
        for i in 0..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    }

    // Write null terminator
    emu.maps.write_word(new_base + 4 + byte_len, 0);

    // Update the BSTR pointer to point to the string data (skip the 4-byte length prefix)
    let new_bstr = new_base + 4;
    emu.maps.write_qword(pbstr_ptr, new_bstr);

    // Log the final result
    if len > 0 {
        let final_string = emu.maps.read_wide_string_n(new_bstr, len as usize);
        log_red!(
            emu,
            "Final BSTR content: \"{}\" (length: {} chars)",
            final_string,
            len
        );
    } else {
        log_red!(emu, "Created empty BSTR");
    }

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen allocated new string at 0x{:x} size: {} (base: 0x{:x})",
        new_bstr,
        byte_len,
        new_base
    );

    // Note: In a real implementation, you'd free the old BSTR here
    // but in an emulator, we might want to keep it for debugging

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::TRUE;
}
