use crate::emu;

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
