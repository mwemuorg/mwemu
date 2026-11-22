use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI4FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI4FromR4",
        FloatWidth::Single,
        Number::Unsigned(u32::BITS),
    );
}
