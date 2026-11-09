use crate::emu::Emu;
use crate::winapi::common::bstr;

pub(super) fn SysStringLen(emu: &mut Emu) {
    bstr::length_api(emu, false);
}
