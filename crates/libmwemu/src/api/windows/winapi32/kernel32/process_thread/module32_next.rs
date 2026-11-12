use crate::emu::Emu;
use crate::winapi::common::modules;

pub fn Module32Next(emu: &mut Emu) {
    modules::snapshot_next(emu, "kernel32!Module32Next");
}

pub fn Module32NextW(emu: &mut Emu) {
    modules::snapshot_next(emu, "kernel32!Module32NextW");
}
