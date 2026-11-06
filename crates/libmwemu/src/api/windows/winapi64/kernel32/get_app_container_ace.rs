use crate::emu;

pub fn GetAppContainerAce(emu: &mut emu::Emu) {
    let acl_ptr = emu.regs().rcx;
    let starting_index = emu.regs().rdx as u32;
    let ace_ptr_ptr = emu.regs().r8;
    let app_container_ace_index_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetAppContainerAce acl:0x{:x} start:{}",
        acl_ptr,
        starting_index
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
