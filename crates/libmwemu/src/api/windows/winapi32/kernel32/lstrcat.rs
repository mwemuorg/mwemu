use crate::emu;

pub fn lstrcat(emu: &mut emu::Emu) {
    let str1_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!lstrcat cannot read str1 param") as u64;
    let str2_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!lstrcat cannot read str2 param") as u64;

    let mut str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "kernel32!lstrcat '{}'+'{}'", str1, str2);

    str1.push_str(&str2);

    emu.maps.write_string(str1_ptr, &str1);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
