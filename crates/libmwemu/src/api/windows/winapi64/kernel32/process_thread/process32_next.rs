use crate::emu::Emu;
use crate::winapi::common::modules;

pub fn Process32Next(emu: &mut Emu) {
    modules::snapshot_next(emu, "kernel32!Process32Next");
}

pub fn Process32NextW(emu: &mut Emu) {
    modules::snapshot_next(emu, "kernel32!Process32NextW");
}
