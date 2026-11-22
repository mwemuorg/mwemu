use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI8FromR4(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI8FromR4",
        FloatWidth::Single,
        Number::Unsigned(u64::BITS),
    );
}
