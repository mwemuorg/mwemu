use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI8FromR8(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI8FromR8",
        FloatWidth::Double,
        Number::Unsigned(u64::BITS),
    );
}
