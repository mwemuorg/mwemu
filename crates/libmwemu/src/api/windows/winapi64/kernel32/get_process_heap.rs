use crate::emu;
use crate::winapi::helper;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    let key = emu.handle_management.get_or_insert_process_heap();
    let addr = emu
        .handle_management
        .heap_base_addr(key)
        .unwrap_or_else(|| {
            let base = emu.heap_mut().base();
            emu.handle_management.set_heap_base_addr(key, base);
            base
        });
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
    log_red!(emu, "kernel32!GetProcessHeap =0x{:x}", addr);
    let _h = helper::handler_create("process_heap");
}
