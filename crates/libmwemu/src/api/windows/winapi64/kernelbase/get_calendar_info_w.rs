use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn GetCalendarInfoW(emu: &mut Emu) {
    strings::calendar_info(emu, "kernelbase!GetCalendarInfoW", true);
}
