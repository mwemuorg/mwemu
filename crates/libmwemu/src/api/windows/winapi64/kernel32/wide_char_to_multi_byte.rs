use crate::winapi::winapi64::kernel32::{LAST_ERROR, clear_last_error};
use crate::{emu, windows::constants};

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let dw_flags = emu.regs().rdx as usize;
    let lp_wide_char_str = emu.regs().r8 as usize;
    let cch_wide_char = emu.regs().r9 as isize;
    let lp_multi_byte_str = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let cb_multi_byte = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let lp_default_char = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let lp_used_default_char = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("kernel32!WideCharToMultiByte error reading param");

    log_red!(
        emu,
        "** {}:{:x} kernel32!WideCharToMultiByte code_page: {} dw_flags: {} lp_wide_char_str: 0x{:x} cch_wide_char: {} lp_multi_byte_str: 0x{:x} cb_multi_byte: {} lp_default_char: 0x{:x} lp_used_default_char: 0x{:x}",
        emu.pos,
        emu.regs().rip,
        code_page,
        dw_flags,
        lp_wide_char_str,
        cch_wide_char,
        lp_multi_byte_str,
        cb_multi_byte,
        lp_default_char,
        lp_used_default_char
    );

    // 1. Input validation
    if lp_wide_char_str == 0 {
        log::warn!("{} kernel32!WideCharToMultiByte invalid parameter", emu.pos);
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INVALID_PARAMETER;
        emu.regs_mut().rax = 0;
        return;
    }

    // 2. Handle special code pages
    if (code_page == constants::CP_UTF7 || code_page == constants::CP_UTF8)
        && (lp_default_char != 0 || lp_used_default_char != 0) {
            // Set last error to ERROR_INVALID_PARAMETER
            log::warn!("{} kernel32!WideCharToMultiByte invalid parameter", emu.pos);
            let mut err = LAST_ERROR.lock().unwrap();
            *err = constants::ERROR_INVALID_PARAMETER;
            emu.regs_mut().rax = 0;
            return;
        }

    // 3. Read input string and get its length
    let s = emu.maps.read_wide_string(lp_wide_char_str as u64);
    let input_len = if cch_wide_char == -1 {
        s.len()
    } else {
        cch_wide_char as usize
    };

    // 4. If this is just a size query
    if cb_multi_byte == 0 {
        emu.regs_mut().rax = input_len as u64;
        clear_last_error(emu);
        return;
    }

    // 5. Check output buffer size
    if cb_multi_byte < input_len as u64 {
        // Set last error to ERROR_INSUFFICIENT_BUFFER
        log::warn!(
            "{} buffer too small for result cb_multi_byte: {} input_len: {}",
            emu.pos,
            cb_multi_byte,
            input_len
        );
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INSUFFICIENT_BUFFER;
        emu.regs_mut().rax = 0;
        return;
    }

    // 6. Perform the actual conversion
    if lp_multi_byte_str > 0 && !s.is_empty() {
        if lp_multi_byte_str < emu.cfg.stack_addr
            || lp_multi_byte_str > emu.cfg.stack_addr + 0x030000
        {
            emu.maps.write_string(lp_multi_byte_str, &s);
        }

        // Set used default char flag if requested
        if lp_used_default_char != 0 {
            emu.maps.write_byte(lp_used_default_char, 0); // For this simple implementation, assume no defaults needed
        }
    }

    // 7. Return number of bytes written
    emu.regs_mut().rax = if cch_wide_char == -1 {
        (s.len() + 1) as u64 // Include null terminator
    } else {
        s.len() as u64
    };

    clear_last_error(emu);

    // LOG THE INPUT WIDE STRING
    if lp_wide_char_str > 0 && !s.is_empty() {
        log_red!(
            emu,
            "Input wide string: \"{}\" (length: {} characters)",
            s.escape_debug(),
            // This will show escape sequences for non-printable chars
            input_len
        );
    }
}
