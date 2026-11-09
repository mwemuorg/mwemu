use crate::emu::Emu;
use crate::winapi::common::bstr;

pub fn SysStringByteLen(emu: &mut Emu) {
    bstr::length_api(emu, true);
}
