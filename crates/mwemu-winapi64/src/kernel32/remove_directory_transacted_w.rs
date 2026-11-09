use crate::emu;

pub fn RemoveDirectoryTransactedW(emu: &mut emu::Emu) {
    let lpPathName = emu.regs().rcx as u32;
    let _hTransaction = emu.regs().rdx as u32;

    let lpPathName = emu.maps.read_wide_string(lpPathName as u64);

    log_red!(emu, "kernel32!RemoveDirectoryTransactedW {}", lpPathName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
