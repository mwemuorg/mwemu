use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FreeAddrInfoEx(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(handle);
    log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}

pub fn FreeAddrInfoExW(emu: &mut emu::Emu) {
    let info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
    helper::handler_close(info_ptr);
    stdcall_ret(emu, 1);
}
