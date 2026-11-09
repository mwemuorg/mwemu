use crate::emu;
use crate::winapi::helper;

pub fn EventWrite(emu: &mut emu::Emu) {
    let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let event_desc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let user_data_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let user_data = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _activity_id = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!EventWrite handle: 0x{:x} desc: 0x{:x} count: {} data: 0x{:x}",
        reg_handle,
        event_desc,
        user_data_count,
        user_data
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
}
