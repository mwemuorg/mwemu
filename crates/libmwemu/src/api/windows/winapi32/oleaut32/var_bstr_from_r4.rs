use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarBstrFromR4(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarBstrFromR4", FloatWidth::Single, true);
}
