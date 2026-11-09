use crate::emu;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let codepage = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0);
    let wstr_ptr = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0) as u64;
    let wstr_sz = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);
    let mbytestr_ptr = emu.maps.read_dword(emu.regs().rsp + 16).unwrap_or(0) as u64;
    let mbytestr_sz = emu.maps.read_dword(emu.regs().rsp + 20).unwrap_or(0);
    let in_default_char = emu.maps.read_dword(emu.regs().rsp + 24).unwrap_or(0) as u64;
    let out_default_char = emu.maps.read_dword(emu.regs().rsp + 28).unwrap_or(0) as u64;

    //log::trace!("default_char_ptr 0x{:x}", in_default_char);
    //let default_char = emu.maps.read_byte(in_default_char)
    //    .unwrap_or(0);

    //emu.maps.write_byte(out_default_char, 0);

    let wchar_count = if wstr_sz as i32 == -1 {
        let s = emu.maps.read_wide_string(wstr_ptr);
        s.len() + 1
    } else {
        wstr_sz as usize
    };

    if mbytestr_ptr == 0 || mbytestr_sz == 0 {
        log_red!(
            emu,
            "kernel32!WideCharToMultiByte sz:{}->{} ={}",
            wstr_sz,
            mbytestr_sz,
            wchar_count
        );
        for _ in 0..8 {
            emu.stack_pop32(false);
        }
        emu.regs_mut().rax = wchar_count as u64;
        return;
    }

    let write_count = wchar_count.min(mbytestr_sz as usize);
    for i in 0..write_count {
        let wc = emu.maps.read_word(wstr_ptr + (i as u64) * 2).unwrap_or(0);
        let byte = if wc <= 0xFF { wc as u8 } else { b'?' };
        emu.maps.write_byte(mbytestr_ptr + i as u64, byte);
    }

    log_red!(
        emu,
        "kernel32!WideCharToMultiByte sz:{}->{} ={}",
        wstr_sz,
        mbytestr_sz,
        write_count
    );

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = write_count as u64;
}
