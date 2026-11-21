use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI1FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI1FromR4",
        FloatWidth::Single,
        Number::Unsigned(u8::BITS),
    );
}
