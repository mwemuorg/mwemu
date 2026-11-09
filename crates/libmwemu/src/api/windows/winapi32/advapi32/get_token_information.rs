use crate::emu;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let info_class = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let info_buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let buf_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let ret_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    if ret_len_ptr != 0 {
        emu.maps.write_dword(ret_len_ptr, 4);
    }
    if info_buf != 0 && buf_len >= 4 {
        emu.maps.write_dword(info_buf, 1);
    }
    log_red!(
        emu,
        "advapi32!GetTokenInformation token:0x{:x} class:{}",
        token_handle,
        info_class
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
