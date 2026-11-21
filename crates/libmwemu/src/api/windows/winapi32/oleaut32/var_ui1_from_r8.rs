use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI1FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI1FromR8",
        FloatWidth::Double,
        Number::Unsigned(u8::BITS),
    );
}
