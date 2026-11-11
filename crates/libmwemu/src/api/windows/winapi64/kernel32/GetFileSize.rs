use crate::emu::Emu;
use crate::winapi::common::files::get_file_size;

pub fn GetFileSize(emu: &mut Emu) {
    get_file_size(emu, "kernel32!GetFileSize", false);
}
