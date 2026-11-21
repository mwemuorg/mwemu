use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarR8FromR4(emu: &mut Emu) {
    automation::convert(emu, "VarR8FromR4", FloatWidth::Single, Number::Double);
}
