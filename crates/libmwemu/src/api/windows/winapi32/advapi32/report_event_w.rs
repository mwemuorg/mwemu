use crate::emu;

pub fn ReportEventW(emu: &mut emu::Emu) {
    let hevt_log = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let category = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let event_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _user_sid = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let num_strings = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _data_size = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let _strings_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
    let _raw_data = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!ReportEventW log:0x{:x} type:{} cat:{} id:0x{:x} nstr:{}",
        hevt_log,
        wtype,
        category,
        event_id,
        num_strings
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    for _ in 0..9 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
