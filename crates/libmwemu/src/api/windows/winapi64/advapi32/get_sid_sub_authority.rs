use crate::emu;

pub fn GetSidSubAuthority(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    let index = emu.regs().rdx;
    // Returns pointer to the sub-authority DWORD at offset 8 + index*4
    let result = sid_ptr + 8 + index * 4;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthority sid:0x{:x} idx:{} => 0x{:x}",
        sid_ptr,
        index,
        result
    );
    for _ in 0..2 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
