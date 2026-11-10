use crate::emu::Emu;
use crate::winapi::common::files::get_disk_free_space;

pub fn GetDiskFreeSpaceW(emu: &mut Emu) {
    get_disk_free_space(emu, "kernel32!GetDiskFreeSpaceW", true);
}
