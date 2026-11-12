use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VarDecCmpR8(emu: &mut Emu) {
    automation::compare_decimal(emu);
}
