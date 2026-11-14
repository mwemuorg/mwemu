use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetCurrencyFormatA(emu: &mut emu::Emu) {
    get_currency_format(emu, "GetCurrencyFormatA", false);
}

fn get_currency_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _Locale = WinApiAbi::arg(emu, 0) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 1) as u32;
    let _lpValue = WinApiAbi::arg(emu, 2) as u32;
    let _lpFormat = WinApiAbi::arg(emu, 3) as u32;
    let lpCurrencyStr = WinApiAbi::arg(emu, 4) as u32;
    let _cchCurrency = WinApiAbi::arg(emu, 5) as u32;

    let lpCurrencyStr = strings::read_text(emu, lpCurrencyStr as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpCurrencyStr);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn GetCurrencyFormatW(emu: &mut emu::Emu) {
    get_currency_format(emu, "GetCurrencyFormatW", true);
}
