use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    let key = emu.handle_management.get_or_insert_process_heap();
    emu.regs_mut().rax = heap_engine::heap_handle_address(emu, key);
    log_red!(emu, "kernel32!GetProcessHeap =0x{:x}", emu.regs().rax);
}
