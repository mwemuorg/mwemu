use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetNumberFormatA(emu: &mut emu::Emu) {
    get_number_format(emu, "GetNumberFormatA", false);
}

fn get_number_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _Locale = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpValue = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpFormat = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let lpNumberStr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _cchNumber = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let lpNumberStr = strings::read_text(emu, lpNumberStr as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpNumberStr);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn GetNumberFormatW(emu: &mut emu::Emu) {
    get_number_format(emu, "GetNumberFormatW", true);
}
