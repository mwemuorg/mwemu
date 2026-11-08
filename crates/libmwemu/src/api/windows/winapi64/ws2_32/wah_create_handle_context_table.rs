use crate::emu;
use crate::winapi::helper;

pub fn WahCreateHandleContextTable(emu: &mut emu::Emu) {
    let table_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WahCreateHandleContextTable ptr:0x{:x}",
        table_ptr
    );
    let handle = helper::handler_create("wah_handle");
    if table_ptr != 0 {
        emu.maps.write_qword(table_ptr, handle);
    }
    emu.regs_mut().rax = 0;
}
