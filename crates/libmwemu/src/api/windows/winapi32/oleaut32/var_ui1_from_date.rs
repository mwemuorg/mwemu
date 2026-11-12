use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI1FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI1FromDate",
        FloatWidth::Double,
        Number::Unsigned(u8::BITS),
    );
}
