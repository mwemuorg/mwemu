use crate::emu;
use crate::winapi::helper;

pub fn EventWriteTransfer(emu: &mut emu::Emu) {
    let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let event_desc = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let activity_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let related_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let user_data_count = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let user_data = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!EventWriteTransfer handle: 0x{:x} desc: 0x{:x} activity: 0x{:x} count: {}",
        reg_handle,
        event_desc,
        activity_id,
        user_data_count
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
