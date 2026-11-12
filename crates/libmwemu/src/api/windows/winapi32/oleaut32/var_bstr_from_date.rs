use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarBstrFromDate(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarBstrFromDate", FloatWidth::Double, true);
}
