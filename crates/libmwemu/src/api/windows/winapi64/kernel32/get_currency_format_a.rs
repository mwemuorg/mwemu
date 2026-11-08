use crate::emu;
use crate::windows::constants;

pub fn GetCurrencyFormatA(emu: &mut emu::Emu) {
    let _Locale = emu.regs().rcx as u32;
    let _dwFlags = emu.regs().rdx as u32;
    let _lpValue = emu.regs().r8 as u32;
    let _lpFormat = emu.regs().r9 as u32;
    let lpCurrencyStr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _cchCurrency = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpCurrencyStr = emu.maps.read_string(lpCurrencyStr);

    log_red!(emu, "kernel32!GetCurrencyFormatA {}", lpCurrencyStr);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
