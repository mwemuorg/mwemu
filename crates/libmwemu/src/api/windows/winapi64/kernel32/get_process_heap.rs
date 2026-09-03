use crate::emu;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    emu.regs_mut().rax = emu.handle_management.get_or_insert_process_heap() as u64;
    log_red!(emu, "kernel32!GetProcessHeap ={}", emu.regs().rax);
}
