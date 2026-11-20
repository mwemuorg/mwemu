use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI8FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI8FromDate",
        FloatWidth::Double,
        Number::Signed(i64::BITS),
    );
}
