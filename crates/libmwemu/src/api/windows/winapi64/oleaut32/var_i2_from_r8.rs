use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI2FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI2FromR8",
        FloatWidth::Double,
        Number::Signed(i16::BITS),
    );
}
