use crate::emu::Emu;
use crate::winapi::common::bstr;

pub(super) fn SysReAllocStringLen(emu: &mut Emu) {
    bstr::realloc_api(emu, true);
}
