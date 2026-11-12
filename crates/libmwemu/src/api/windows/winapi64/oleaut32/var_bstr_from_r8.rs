use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarBstrFromR8(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarBstrFromR8", FloatWidth::Double, true);
}
