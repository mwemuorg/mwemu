use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAUnadvertiseProvider(emu: &mut emu::Emu) {
    let provider_id_ptr = WinApiAbi::arg(emu, 0);
    log_red!(
        emu,
        "ws2_32!WSAUnadvertiseProvider id:0x{:x}",
        provider_id_ptr
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
