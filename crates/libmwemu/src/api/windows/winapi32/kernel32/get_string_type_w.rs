use crate::emu;

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let info_type = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!GetStringTypeW error reading info_type param");
    let str_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!GetStringTypeW error reading str_ptr param") as u64;
    let sz = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!GetStringTypeW error reading sz param");
    let char_type = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernel32!GetStringTypeW error reading char_type param");

    let ustr = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!GetStringTypeW `{}` 0x{}", ustr, sz);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
