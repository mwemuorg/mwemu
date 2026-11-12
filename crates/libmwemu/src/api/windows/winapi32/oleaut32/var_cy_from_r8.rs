use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarCyFromR8(emu: &mut Emu) {
    automation::convert(emu, "VarCyFromR8", FloatWidth::Double, Number::Currency);
}
