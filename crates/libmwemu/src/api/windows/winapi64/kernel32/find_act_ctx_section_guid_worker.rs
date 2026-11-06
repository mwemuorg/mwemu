use crate::emu;
use crate::windows::constants;

pub fn FindActCtxSectionGuidWorker(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _extguid = emu.regs().rdx as u32;
    let _section_kind = emu.regs().r8 as u32;
    let _guid = emu.regs().r9 as u32;
    let _a5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!FindActCtxSectionGuidWorker");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
