use crate::emu;
use crate::winapi::helper;

pub fn WSALookupServiceBeginA(emu: &mut emu::Emu) {
    let restrictions_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSALookupServiceBeginA flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSALookupServiceBeginW(emu: &mut emu::Emu) {
    let restrictions_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSALookupServiceBeginW flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
