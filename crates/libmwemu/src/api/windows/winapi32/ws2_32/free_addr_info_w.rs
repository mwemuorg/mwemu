use crate::emu;
use crate::winapi::helper;

pub fn FreeAddrInfoW(emu: &mut emu::Emu) {
    let info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!FreeAddrInfoW ptr:0x{:x}", info_ptr);
    let handle = helper::handler_create("FreeAddrInfoW");
    if info_ptr != 0 {
        let map_name = format!("addrinfo_{:x}", info_ptr);
        emu.maps.free(&map_name);
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
