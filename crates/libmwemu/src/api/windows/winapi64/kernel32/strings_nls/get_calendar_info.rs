use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn GetCalendarInfoA(emu: &mut Emu) {
    strings::calendar_info(emu, "kernel32!GetCalendarInfoA", false);
}

pub fn GetCalendarInfoW(emu: &mut Emu) {
    strings::calendar_info(emu, "kernel32!GetCalendarInfoW", true);
}
