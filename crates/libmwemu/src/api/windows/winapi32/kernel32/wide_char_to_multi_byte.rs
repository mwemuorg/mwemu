use crate::emu;

pub fn WideCharToMultiByte(emu: &mut emu::Emu) {
    let codepage = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let flags = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let wstr_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!WideCharToMultiByte error reading param") as u64;
    let wstr_sz = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let mbytestr_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 16)
        .expect("kernel32!WideCharToMultiByte error reading param") as u64;
    let mbytestr_sz = emu
        .maps
        .read_dword(emu.regs().rsp + 20)
        .expect("kernel32!WideCharToMultiByte error reading param");
    let in_default_char =
        emu.maps
            .read_dword(emu.regs().rsp + 24)
            .expect("kernel32!WideCharToMultiByte error reading param") as u64;
    let out_default_char =
        emu.maps
            .read_dword(emu.regs().rsp + 28)
            .expect("kernel32!WideCharToMultiByte error reading param") as u64;

    //log::trace!("default_char_ptr 0x{:x}", in_default_char);
    //let default_char = emu.maps.read_byte(in_default_char)
    //    .expect("kernel32!WideCharToMultiByte error reading default char");

    //emu.maps.write_byte(out_default_char, 0);

    let s = emu.maps.read_wide_string(wstr_ptr);
    if mbytestr_ptr > 0 {
        emu.maps.write_string(mbytestr_ptr, &s);
    }

    log_red!(
        emu,
        "kernel32!WideCharToMultiByte `{}` sz:{}->{} ={}",
        s,
        wstr_sz,
        mbytestr_sz,
        s.len()
    );

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = s.len() as u64 + 2;
}
