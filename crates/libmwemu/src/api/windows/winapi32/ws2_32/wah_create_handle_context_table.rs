use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WahCreateHandleContextTable(emu: &mut emu::Emu) {
    let table_ptr = WinApiAbi::arg(emu, 0);
    log_red!(
        emu,
        "ws2_32!WahCreateHandleContextTable ptr:0x{:x}",
        table_ptr
    );
    let handle = helper::handler_create("wah_handle");
    if table_ptr != 0 {
        emu.maps.write_dword(table_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
