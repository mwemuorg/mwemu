use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarR8FromDate(emu: &mut Emu) {
    automation::convert(emu, "VarR8FromDate", FloatWidth::Double, Number::Double);
}
