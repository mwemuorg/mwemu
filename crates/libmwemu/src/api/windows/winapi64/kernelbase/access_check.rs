use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn AccessCheck(emu: &mut emu::Emu) {
    let p_sd = emu.regs().rcx;
    let client_token = emu.regs().rdx;
    let desired_access = emu.regs().r8;
    let generic_mapping = emu.regs().r9;
    let priv_set = WinApiAbi::arg(emu, 4);
    let priv_set_len = WinApiAbi::arg(emu, 5);
    let granted_access = WinApiAbi::arg(emu, 6);
    let access_status = WinApiAbi::arg(emu, 7);
    log_red!(
        emu,
        "kernelbase!AccessCheck sd:0x{:x} token:0x{:x} desired:0x{:x}",
        p_sd,
        client_token,
        desired_access
    );
    if granted_access != 0 {
        emu.maps.write_dword(granted_access, desired_access as u32);
    }
    if access_status != 0 {
        emu.maps.write_dword(access_status, 1);
    }
    if priv_set_len != 0 {
        emu.maps.write_dword(priv_set_len, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
