use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegEnumKeyExA(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExA");
}

fn reg_enum_key_ex(emu: &mut emu::Emu, api: &str) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let index = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "advapi32!{api} hkey:0x{:x} idx:{}", hkey, index);
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExW");
}
