use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = WinApiAbi::arg(emu, 0) as u32;
    let info_class = WinApiAbi::arg(emu, 1) as u32;
    let info_buf = WinApiAbi::arg(emu, 2);
    let buf_len = WinApiAbi::arg(emu, 3) as u32;
    let ret_len_ptr = WinApiAbi::arg(emu, 4);
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
