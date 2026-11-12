use crate::emu;

pub fn RegEnumKeyA(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyA");
}

fn reg_enum_key(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    log_red!(emu, "advapi32!{api} hkey:0x{:x} index:{}", hkey, index);
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    for _ in 0..8 {}
}

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyW");
}
