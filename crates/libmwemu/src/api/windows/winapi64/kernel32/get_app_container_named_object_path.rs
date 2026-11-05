use crate::emu;

pub fn GetAppContainerNamedObjectPath(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let sid_ptr = emu.regs().rdx;
    let object_path_length = emu.regs().r8 as u32;
    let object_path_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetAppContainerNamedObjectPath token:0x{:x} sid:0x{:x}",
        token,
        sid_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
