use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegEnumValueA(emu: &mut emu::Emu) {
    reg_enum_value(emu, "RegEnumValueA");
}

fn reg_enum_value(emu: &mut emu::Emu, api: &str) {
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

pub fn RegEnumValueW(emu: &mut emu::Emu) {
    reg_enum_value(emu, "RegEnumValueW");
}
