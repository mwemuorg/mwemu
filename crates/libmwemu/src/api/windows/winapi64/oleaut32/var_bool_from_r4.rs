use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarBoolFromR4(emu: &mut Emu) {
    automation::convert(emu, "VarBoolFromR4", FloatWidth::Single, Number::Bool);
}
