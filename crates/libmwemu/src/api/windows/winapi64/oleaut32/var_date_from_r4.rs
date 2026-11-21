use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarDateFromR4(emu: &mut Emu) {
    automation::convert(emu, "VarDateFromR4", FloatWidth::Single, Number::Date);
}
