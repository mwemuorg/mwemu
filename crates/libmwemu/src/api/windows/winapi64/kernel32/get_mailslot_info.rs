use crate::emu;
use crate::windows::constants;

pub fn GetMailslotInfo(emu: &mut emu::Emu) {
    let _hMailslot = emu.regs().rcx as u32;
    let _lpMaxMessageSize = emu.regs().rdx as u32;
    let _lpNextSize = emu.regs().r8 as u32;
    let _lpMessageCount = emu.regs().r9 as u32;
    let _lpReadTimeout = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(emu, "kernel32!GetMailslotInfo");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
