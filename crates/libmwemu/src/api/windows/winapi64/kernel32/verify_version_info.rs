use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    verify_version_info(emu, "VerifyVersionInfoA");
}

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    verify_version_info(emu, "VerifyVersionInfoW");
}

fn verify_version_info(emu: &mut emu::Emu, api: &str) {
    let version_info = WinApiAbi::arg(emu, 0);
    let type_mask = WinApiAbi::arg(emu, 1);
    let condition_mask = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "kernel32!{} info:0x{:x} type_mask:0x{:x} cond_mask:0x{:x} -> TRUE",
        api,
        version_info,
        type_mask,
        condition_mask
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
