use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarR4FromR8(emu: &mut Emu) {
    automation::convert(emu, "VarR4FromR8", FloatWidth::Double, Number::Single);
}
