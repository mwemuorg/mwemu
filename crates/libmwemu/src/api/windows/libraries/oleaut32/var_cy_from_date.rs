use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarCyFromDate(emu: &mut Emu) {
    automation::convert(emu, "VarCyFromDate", FloatWidth::Double, Number::Currency);
}
