use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarDecFromDate(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarDecFromDate", FloatWidth::Double, false);
}
