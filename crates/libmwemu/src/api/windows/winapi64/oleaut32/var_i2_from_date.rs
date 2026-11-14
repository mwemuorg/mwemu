use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI2FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI2FromDate",
        FloatWidth::Double,
        Number::Signed(i16::BITS),
    );
}
