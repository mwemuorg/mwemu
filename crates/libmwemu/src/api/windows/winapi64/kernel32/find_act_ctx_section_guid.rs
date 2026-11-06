use crate::emu;

pub fn FindActCtxSectionGuid(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let ext_guid = emu.regs().rdx;
    let section_id = emu.regs().r8 as u32;
    let guid_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!FindActCtxSectionGuid flags:0x{:x} section:{} guid:0x{:x}",
        flags,
        section_id,
        guid_ptr
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
}
