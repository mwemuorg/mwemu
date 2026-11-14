use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarDecFromR4(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarDecFromR4", FloatWidth::Single, false);
}
