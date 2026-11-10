use crate::emu::Emu;
use crate::winapi::common::modules;

pub use modules::main_module_path;

pub fn GetModuleFileNameW(emu: &mut Emu) {
    modules::get_module_file_name(emu, "kernel32!GetModuleFileNameW", true);
}

pub fn module_path(emu: &Emu, handle: u64) -> Option<String> {
    modules::module_path(emu, handle)
}
