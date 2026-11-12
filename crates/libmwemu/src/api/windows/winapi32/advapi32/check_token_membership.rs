use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let token = WinApiAbi::arg(emu, 0);
    let sid_to_check = WinApiAbi::arg(emu, 1);
    let is_member_ptr = WinApiAbi::arg(emu, 2);
    if is_member_ptr != 0 {
        emu.maps.write_dword(is_member_ptr, 1); // TRUE - user is member
    }
    log_red!(
        emu,
        "advapi32!CheckTokenMembership token:0x{:x} sid:0x{:x} => TRUE",
        token,
        sid_to_check
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
