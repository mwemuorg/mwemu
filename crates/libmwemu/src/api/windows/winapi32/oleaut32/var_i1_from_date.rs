use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI1FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI1FromDate",
        FloatWidth::Double,
        Number::Signed(i8::BITS),
    );
}
