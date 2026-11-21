use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VarCyCmpR8(emu: &mut Emu) {
    automation::compare_currency(emu);
}
