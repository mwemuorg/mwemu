use crate::emu::Emu;
use crate::winapi::common::modules::get_module_handle;

pub fn GetModuleHandleExA(emu: &mut Emu) {
    get_module_handle(emu, "kernel32!GetModuleHandleExA", false, true);
}
