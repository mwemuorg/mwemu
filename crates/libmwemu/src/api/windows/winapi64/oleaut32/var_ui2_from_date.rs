use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI2FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI2FromDate",
        FloatWidth::Double,
        Number::Unsigned(u16::BITS),
    );
}
