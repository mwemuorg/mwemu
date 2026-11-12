use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarDecFromR8(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarDecFromR8", FloatWidth::Double, false);
}
