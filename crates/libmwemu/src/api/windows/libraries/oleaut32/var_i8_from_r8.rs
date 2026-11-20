use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI8FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI8FromR8",
        FloatWidth::Double,
        Number::Signed(i64::BITS),
    );
}
