use crate::emu;
use crate::windows::constants;

pub fn GetCalendarInfoA(emu: &mut emu::Emu) {
    let _Locale = emu.regs().rcx as u32;
    let _Calendar = emu.regs().rdx as u32;
    let _CalType = emu.regs().r8 as u32;
    let _lpCalData = emu.regs().r9 as u32;
    let _cchData = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _lpValue = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!GetCalendarInfoA");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
