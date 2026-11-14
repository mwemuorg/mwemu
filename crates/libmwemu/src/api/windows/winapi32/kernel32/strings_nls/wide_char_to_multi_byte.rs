use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let codepage = WinApiAbi::arg(emu, 0) as u32;
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let wstr_ptr = WinApiAbi::arg(emu, 2);
    let wstr_sz = WinApiAbi::arg(emu, 3) as u32;
    let mbytestr_ptr = WinApiAbi::arg(emu, 4);
    let mbytestr_sz = WinApiAbi::arg(emu, 5) as u32;
    let in_default_char = WinApiAbi::arg(emu, 6);
    let out_default_char = WinApiAbi::arg(emu, 7);

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
