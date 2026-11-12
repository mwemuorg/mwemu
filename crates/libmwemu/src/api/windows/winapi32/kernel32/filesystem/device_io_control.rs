use crate::emu::Emu;
use crate::winapi::common::files;

pub fn DeviceIoControl(emu: &mut Emu) {
    files::device_io_control(emu, "kernel32!DeviceIoControl");
}
