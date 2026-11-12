use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth, Number};

pub fn VarUI8FromDate(emu: &mut Emu) {
    automation::convert(
        emu,
        "VarUI8FromDate",
        FloatWidth::Double,
        Number::Unsigned(u64::BITS),
    );
}
