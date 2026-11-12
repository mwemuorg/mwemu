use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegEnumKeyExA(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExA");
}

fn reg_enum_key_ex(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let name_len_ptr = emu.regs().r9;
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} index:{}", hkey, index);
    let out_ptr = WinApiAbi::arg(emu, 3);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    reg_enum_key_ex(emu, "RegEnumKeyExW");
}
