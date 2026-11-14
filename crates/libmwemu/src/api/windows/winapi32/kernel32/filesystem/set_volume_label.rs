use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetVolumeLabelA(emu: &mut emu::Emu) {
    set_volume_label(emu, "SetVolumeLabelA", false);
}

fn set_volume_label(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpRootPathName = WinApiAbi::arg(emu, 0) as u32;
    let lpVolumeName = WinApiAbi::arg(emu, 1) as u32;

    let lpRootPathName = strings::read_text(emu, lpRootPathName as u64, wide);
    let lpVolumeName = strings::read_text(emu, lpVolumeName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpRootPathName, lpVolumeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SetVolumeLabelW(emu: &mut emu::Emu) {
    set_volume_label(emu, "SetVolumeLabelW", true);
}
