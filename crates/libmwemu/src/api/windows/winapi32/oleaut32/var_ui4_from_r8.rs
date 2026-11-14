use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI4FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI4FromR8",
        FloatWidth::Double,
        Number::Unsigned(u32::BITS),
    );
}
