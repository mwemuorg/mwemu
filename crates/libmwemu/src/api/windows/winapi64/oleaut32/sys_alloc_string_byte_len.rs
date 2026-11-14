use crate::emu::Emu;
use crate::winapi::common::bstr;

pub(super) fn SysAllocStringByteLen(emu: &mut Emu) {
    bstr::alloc_api(emu, true, true);
}
