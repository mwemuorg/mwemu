use crate::winapi::winapi64::kernel32::LAST_ERROR;
use crate::{emu, windows::constants};

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;
    let dw_flags = emu.regs().rdx;
    let utf8_ptr = emu.regs().r8;
    let cb_multi_byte = emu.regs().r9 as i64;
    let wide_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!MultiByteToWideChar cannot read wide_ptr");
    let cch_wide_char =
        emu.maps
            .read_dword(emu.regs().rsp + 0x28) // yes, read only half of the stack item
            .expect("kernel32!MultiByteToWideChar cannot read cchWideChar") as i32;

    let mut utf8: String = String::new();

    // validation 1: output NULL but cch > 0
    if wide_ptr == 0 && cch_wide_char > 0 {
        log::warn!(
            "[ALERT] MultiByteToWideChar: output buffer is NULL but cch_wide_char = {}",
            cch_wide_char
        );
    }

    // validation 2: output NO NULL but cch == 0
    if wide_ptr != 0 && cch_wide_char == 0 {
        log::warn!("[ALERT] MultiByteToWideChar: output buffer is non-NULL but cch_wide_char = 0");
    }

    // validation 3: size too big
    if !(0..=1_000_000).contains(&cch_wide_char) {
        log::warn!(
            "[ALERT] MultiByteToWideChar: cch_wide_char = {} exceeds 1_000_000",
            cch_wide_char,
        );
    }

    // validation 4: if cb_multi_byte is negative or too big
    if !(0..=10_000_000).contains(&cb_multi_byte) {
        log::warn!(
            "[ALERT] MultiByteToWideChar: cb_multi_byte = {} is suspicious",
            cb_multi_byte
        );
    }

    // Read exact number of bytes specified
    if utf8_ptr > 0 {
        let bytes = emu.maps.read_bytes(utf8_ptr, cb_multi_byte as usize);
        utf8 = String::from_utf8_lossy(bytes).to_string();
    }

    log_red!(
        emu,
        ":{:x} kernel32!MultiByteToWideChar code_page: {} dw_flags: {} utf8_ptr: 0x{:x} cb_multi_byte: {} wide_ptr: 0x{:x} cch_wide_char: {}",
        emu.regs().rip,
        code_page,
        dw_flags,
        utf8_ptr,
        cb_multi_byte,
        wide_ptr,
        cch_wide_char
    );

    // LOG THE INPUT STRING
    if utf8_ptr > 0 && !utf8.is_empty() {
        log_red!(
            emu,
            "Input UTF-8 string: \"{}\" (length: {} bytes)",
            utf8.escape_debug(),
            // This will show escape sequences for non-printable chars
            cb_multi_byte
        );
    }

    // Convert to UTF-16 (without null terminator since cb_multi_byte is explicit)
    let wide: Vec<u16> = utf8.encode_utf16().collect();

    if cch_wide_char == 0 {
        // Return required buffer size for UTF-16 conversion
        emu.regs_mut().rax = wide.len() as u64;
    } else if wide_ptr != 0 {
        // Write string if buffer is large enough
        if cch_wide_char >= wide.len() as i32 {
            for (i, wchar) in wide.iter().enumerate() {
                emu.maps.write_word(wide_ptr + (i * 2) as u64, *wchar);
            }
            emu.regs_mut().rax = wide.len() as u64;
        } else {
            let mut err = LAST_ERROR.lock().unwrap();
            *err = constants::ERROR_INSUFFICIENT_BUFFER;
            emu.regs_mut().rax = 0;
        }
    } else {
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INVALID_PARAMETER;
        emu.regs_mut().rax = 0;
    }
}
