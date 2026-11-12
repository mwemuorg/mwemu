use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetDateFormatA(emu: &mut Emu) {
    get_date_format(emu, "GetDateFormatA", false);
}

pub fn GetDateFormatW(emu: &mut Emu) {
    get_date_format(emu, "GetDateFormatW", true);
}

fn get_date_format(emu: &mut Emu, api: &str, wide: bool) {
    let buffer = WinApiAbi::arg(emu, 4);
    let capacity = WinApiAbi::arg(emu, 5) as i32;
    let text = "11/01/2026";
    let result = strings::write_text(emu, buffer, capacity, text, wide);
    log_red!(emu, "kernelbase!{api} => '{}' result:{}", text, result);
    WinApiAbi::set_ret(emu, result);
}
