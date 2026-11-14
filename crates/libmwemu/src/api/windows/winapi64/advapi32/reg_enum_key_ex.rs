use crate::emu;

pub fn RegEnumKeyExA(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExA");
}

fn reg_enum_key_ex(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    log_red!(emu, "advapi32!{api} hkey:0x{:x} idx:{}", hkey, index);
    for _ in 0..8 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExW");
}
