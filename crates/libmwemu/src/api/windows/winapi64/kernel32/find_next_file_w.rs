use crate::emu;

pub fn FindNextFileW(emu: &mut emu::Emu) {
    let h_find = emu.regs().rcx;
    let find_data_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!FindNextFileW h:0x{:x} data:0x{:x} => FALSE (no more files)",
        h_find,
        find_data_ptr
    );
    emu.regs_mut().rax = 0;
    emu.set_last_error(18);
}
