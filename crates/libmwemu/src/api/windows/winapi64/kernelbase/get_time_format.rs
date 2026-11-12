use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetTimeFormatA(emu: &mut Emu) {
    get_time_format(emu, "GetTimeFormatA", false);
}

pub fn GetTimeFormatW(emu: &mut Emu) {
    get_time_format(emu, "GetTimeFormatW", true);
}

fn get_time_format(emu: &mut Emu, api: &str, wide: bool) {
    let buffer = WinApiAbi::arg(emu, 4);
    let capacity = WinApiAbi::arg(emu, 5) as i32;
    let text = "12:00:00";
    let result = strings::write_text(emu, buffer, capacity, text, wide);
    log_red!(emu, "kernelbase!{api} => '{}' result:{}", text, result);
    WinApiAbi::set_ret(emu, result);
}
