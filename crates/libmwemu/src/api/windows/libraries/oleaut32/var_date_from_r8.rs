use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarDateFromR8(emu: &mut Emu) {
    automation::convert(emu, "VarDateFromR8", FloatWidth::Double, Number::Date);
}
