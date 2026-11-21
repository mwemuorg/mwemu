use crate::emu::Emu;
use crate::winapi::common::automation;

pub fn VarR4CmpR8(emu: &mut Emu) {
    automation::compare_single(emu);
}
