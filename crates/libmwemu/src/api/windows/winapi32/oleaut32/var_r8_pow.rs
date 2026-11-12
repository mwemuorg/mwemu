use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VarR8Pow(emu: &mut Emu) {
    automation::power(emu);
}
