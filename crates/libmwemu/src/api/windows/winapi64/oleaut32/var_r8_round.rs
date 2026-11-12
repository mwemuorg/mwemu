use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VarR8Round(emu: &mut Emu) {
    automation::round(emu);
}
