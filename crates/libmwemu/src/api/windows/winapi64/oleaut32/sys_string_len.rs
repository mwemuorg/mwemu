use crate::emu::Emu;
use crate::winapi::common::bstr;

pub fn SysStringLen(emu: &mut Emu) {
    bstr::length_api(emu, false);
}
