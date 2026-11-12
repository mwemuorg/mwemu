use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegEnumKeyA(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyA");
}

fn reg_enum_key(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!{api} hkey:0x{:x} index:{}", hkey, index);
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
    stdcall_ret(emu, 4);
}

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyW");
}
