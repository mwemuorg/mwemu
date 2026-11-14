use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_len = WinApiAbi::arg(emu, 5);
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if reg_type == 1 || reg_type == 2 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        format!("0x{:x}", data_ptr)
    };
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' type:{} value:'{}' len:{}",
        hkey,
        name,
        reg_type,
        value,
        data_len
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
