use crate::emu;

pub fn GlobalSize_api(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let sz = match emu.maps.get_mem_by_addr(handle) {
        Some(mem) => mem.size() as u64,
        None => 0,
    };
    log_red!(emu, "kernel32!GlobalSize h:0x{:x} => {}", handle, sz);
    emu.set_last_error(0);
    emu.regs_mut().rax = sz;
}
