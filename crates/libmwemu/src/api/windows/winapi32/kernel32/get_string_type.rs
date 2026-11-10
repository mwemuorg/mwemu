use crate::emu;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!GetStringTypeA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let info_type = emu.maps.read_dword(emu.regs().rsp).unwrap_or(0);
    let str_ptr = emu.maps.read_dword(emu.regs().rsp + 4).unwrap_or(0) as u64;
    let sz = emu.maps.read_dword(emu.regs().rsp + 8).unwrap_or(0);
    let char_type = emu.maps.read_dword(emu.regs().rsp + 12).unwrap_or(0);

    let ustr = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!GetStringTypeW `{}` 0x{}", ustr, sz);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
