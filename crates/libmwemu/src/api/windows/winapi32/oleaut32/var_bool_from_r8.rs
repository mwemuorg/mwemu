use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarBoolFromR8(emu: &mut Emu) {
    automation::convert(emu, "VarBoolFromR8", FloatWidth::Double, Number::Bool);
}
