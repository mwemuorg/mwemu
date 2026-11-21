use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarBoolFromDate(emu: &mut Emu) {
    automation::convert(emu, "VarBoolFromDate", FloatWidth::Double, Number::Bool);
}
