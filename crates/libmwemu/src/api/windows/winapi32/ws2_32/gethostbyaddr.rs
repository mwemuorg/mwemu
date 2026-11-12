use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn gethostbyaddr(emu: &mut emu::Emu) {
    let addr_ptr = WinApiAbi::arg(emu, 0);
    let len = WinApiAbi::arg(emu, 1) as u32;
    let addr_type = WinApiAbi::arg(emu, 2) as u32;
    log_red!(
        emu,
        "ws2_32!gethostbyaddr ptr:0x{:x} len:{} type:{}",
        addr_ptr,
        len,
        addr_type
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
