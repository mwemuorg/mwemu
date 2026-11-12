use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarR4FromDate(emu: &mut Emu) {
    automation::convert(emu, "VarR4FromDate", FloatWidth::Double, Number::Single);
}
