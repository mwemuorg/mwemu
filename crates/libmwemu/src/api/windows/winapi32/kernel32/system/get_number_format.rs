use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetNumberFormatA(emu: &mut emu::Emu) {
    get_number_format(emu, "GetNumberFormatA", false);
}

fn get_number_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _Locale = WinApiAbi::arg(emu, 0) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 1) as u32;
    let _lpValue = WinApiAbi::arg(emu, 2) as u32;
    let _lpFormat = WinApiAbi::arg(emu, 3) as u32;
    let lpNumberStr = WinApiAbi::arg(emu, 4) as u32;
    let _cchNumber = WinApiAbi::arg(emu, 5) as u32;

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
