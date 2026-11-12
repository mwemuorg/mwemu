use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI4FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI4FromR4",
        FloatWidth::Single,
        Number::Signed(i32::BITS),
    );
}
