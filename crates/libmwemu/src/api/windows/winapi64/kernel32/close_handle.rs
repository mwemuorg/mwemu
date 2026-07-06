use crate::emu;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;

    log_red!(emu, "kernel32!CloseHandle 0x{:X}", handle);
    emu.handle_management.remove_file_handle(handle as u32);

    emu.regs_mut().rax = 1;
}
