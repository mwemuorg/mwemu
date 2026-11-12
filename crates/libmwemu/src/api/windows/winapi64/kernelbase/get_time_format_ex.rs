use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetTimeFormatEx(emu: &mut Emu) {
    let buffer = WinApiAbi::arg(emu, 4);
    let capacity = WinApiAbi::arg(emu, 5) as i32;
    let text = "12:00:00";
    let result = strings::write_text(emu, buffer, capacity, text, true);
    log_red!(
        emu,
        "kernelbase!GetTimeFormatEx => '{}' result:{}",
        text,
        result
    );
    WinApiAbi::set_ret(emu, result);
}
