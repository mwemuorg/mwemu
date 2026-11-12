use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI2FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI2FromR8",
        FloatWidth::Double,
        Number::Unsigned(u16::BITS),
    );
}
