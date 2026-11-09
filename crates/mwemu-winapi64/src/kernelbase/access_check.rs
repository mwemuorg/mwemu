use crate::emu;

pub fn AccessCheck(emu: &mut emu::Emu) {
    let p_sd = emu.regs().rcx;
    let client_token = emu.regs().rdx;
    let desired_access = emu.regs().r8;
    let generic_mapping = emu.regs().r9;
    let priv_set = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let priv_set_len = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let granted_access = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let access_status = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
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
