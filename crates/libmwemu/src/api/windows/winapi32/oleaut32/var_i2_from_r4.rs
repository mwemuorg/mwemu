use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI2FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI2FromR4",
        FloatWidth::Single,
        Number::Signed(i16::BITS),
    );
}
