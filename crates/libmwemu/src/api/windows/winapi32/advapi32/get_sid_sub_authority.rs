use crate::emu;

pub fn GetSidSubAuthority(emu: &mut emu::Emu) {
    let sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    // Returns pointer to the sub-authority DWORD at offset 8 + index*4
    let result = sid_ptr + 8 + index * 4;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthority sid:0x{:x} idx:{} => 0x{:x}",
        sid_ptr,
        index,
        result
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
