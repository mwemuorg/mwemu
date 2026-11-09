use crate::emu;

pub fn GetHostNameW(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let namelen = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!GetHostNameW buf:0x{:x} len:{}",
        name_ptr,
        namelen
    );
    if name_ptr != 0 && namelen > 8 {
        emu.maps.write_wide_string(name_ptr, "DESKTOP");
    }
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
