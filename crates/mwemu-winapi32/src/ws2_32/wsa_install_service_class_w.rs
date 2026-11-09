use crate::emu;
use crate::winapi::helper;

pub fn WSAInstallServiceClassW(emu: &mut emu::Emu) {
    let class_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "ws2_32!WSAInstallServiceClassW ptr:0x{:x}",
        class_info_ptr
    );
    let handle = helper::handler_create("WSAInstallServiceClassW");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
