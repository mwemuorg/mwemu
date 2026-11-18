use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let key = emu.handle_management.get_or_insert_process_heap();
    let addr = emu
        .handle_management
        .heap_base_addr(key)
        .unwrap_or_else(|| {
            let base = emu.heap_mut().base();
            emu.handle_management.set_heap_base_addr(key, base);
            base
        });
    call.set_return(emu, addr);
    log_red!(emu, "kernel32!GetProcessHeap =0x{:x}", addr as u32);
}
