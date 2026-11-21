use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI8FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI8FromR4",
        FloatWidth::Single,
        Number::Signed(i64::BITS),
    );
}
