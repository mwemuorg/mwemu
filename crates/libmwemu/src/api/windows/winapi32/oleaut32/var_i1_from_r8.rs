use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI1FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI1FromR8",
        FloatWidth::Double,
        Number::Signed(i8::BITS),
    );
}
