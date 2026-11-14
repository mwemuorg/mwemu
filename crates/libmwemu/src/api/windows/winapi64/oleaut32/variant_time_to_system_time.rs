use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VariantTimeToSystemTime(emu: &mut Emu) {
    automation::to_system_time(emu, false);
}
