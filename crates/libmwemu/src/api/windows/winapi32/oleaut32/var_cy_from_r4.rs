use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarCyFromR4(emu: &mut Emu) {
    automation::convert(emu, "VarCyFromR4", FloatWidth::Single, Number::Currency);
}
