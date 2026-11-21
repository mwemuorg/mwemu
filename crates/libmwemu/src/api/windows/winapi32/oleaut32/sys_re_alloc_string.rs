use crate::emu::Emu;
use crate::winapi::common::bstr;

pub fn SysReAllocString(emu: &mut Emu) {
    bstr::realloc_api(emu, false);
}
