use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarI1FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarI1FromR4",
        FloatWidth::Single,
        Number::Signed(i8::BITS),
    );
}
