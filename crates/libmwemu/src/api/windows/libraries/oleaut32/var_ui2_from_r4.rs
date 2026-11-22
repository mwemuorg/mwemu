use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI2FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI2FromR4",
        FloatWidth::Single,
        Number::Unsigned(u16::BITS),
    );
}
