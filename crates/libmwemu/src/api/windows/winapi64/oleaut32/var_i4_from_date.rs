use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI4FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI4FromDate",
        FloatWidth::Double,
        Number::Signed(i32::BITS),
    );
}
