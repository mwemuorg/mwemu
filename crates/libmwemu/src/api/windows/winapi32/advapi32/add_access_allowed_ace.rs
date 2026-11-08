use crate::emu;
use crate::winapi::helper;

pub fn AddAccessAllowedAce(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!AddAccessAllowedAce p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let handle = helper::handler_create("AddAccessAllowedAce");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
    }
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
