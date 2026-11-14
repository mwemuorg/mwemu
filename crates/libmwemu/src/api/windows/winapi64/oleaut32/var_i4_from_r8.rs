use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI4FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI4FromR8",
        FloatWidth::Double,
        Number::Signed(i32::BITS),
    );
}
