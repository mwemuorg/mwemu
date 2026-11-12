use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VariantTimeToDosDateTime(emu: &mut Emu) {
    automation::to_dos_time(emu);
}
