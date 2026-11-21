use crate::emu::Emu;
use crate::winapi::common::automation::{self, FloatWidth};

pub fn VarUdateFromDate(emu: &mut Emu) {
    automation::unsupported_conversion(emu, "VarUdateFromDate", FloatWidth::Double, false);
}
