use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindActCtxSectionGuid(emu: &mut emu::Emu) {
    let flags = arg(emu, 0) as u64 as u32;
    let ext_guid = arg(emu, 1) as u64;
    let section_id = arg(emu, 2) as u64 as u32;
    let guid_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!FindActCtxSectionGuid flags:0x{:x} section:{} guid:0x{:x}",
        flags,
        section_id,
        guid_ptr
    );
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
    stdcall_ret(emu, 4);
}
