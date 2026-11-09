use crate::emu;

pub fn GetSidSubAuthorityCount(emu: &mut emu::Emu) {
    let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    // Returns pointer to the sub-authority count byte at offset 1
    let result = sid_ptr + 1;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthorityCount sid:0x{:x} => 0x{:x}",
        sid_ptr,
        result
    );
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
