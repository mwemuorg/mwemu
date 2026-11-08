use crate::emu;

pub fn GetSidSubAuthorityCount(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    if sid_ptr != 0 && emu.maps.is_mapped(sid_ptr) {
        let _ = emu.maps.read_dword(sid_ptr).unwrap_or(0);
    }
    let result = sid_ptr + 1;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthorityCount sid:0x{:x} => 0x{:x}",
        sid_ptr,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
