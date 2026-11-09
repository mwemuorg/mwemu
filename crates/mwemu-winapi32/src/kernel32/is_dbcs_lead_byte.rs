use crate::emu::Emu;
use crate::winapi::common::strings::dbcs_lead_byte;

pub fn IsDBCSLeadByte(emu: &mut Emu) {
    dbcs_lead_byte(emu, "kernel32!IsDBCSLeadByte", false);
}
