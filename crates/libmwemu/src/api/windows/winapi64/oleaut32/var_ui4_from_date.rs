use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI4FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI4FromDate",
        FloatWidth::Double,
        Number::Unsigned(u32::BITS),
    );
}
