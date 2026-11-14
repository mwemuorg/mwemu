use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn EventWriteTransfer(emu: &mut emu::Emu) {
    let reg_handle = WinApiAbi::arg(emu, 0) as u32;
    let event_desc = WinApiAbi::arg(emu, 1) as u32;
    let activity_id = WinApiAbi::arg(emu, 2) as u32;
    let related_id = WinApiAbi::arg(emu, 3) as u32;
    let user_data_count = WinApiAbi::arg(emu, 4) as u32;
    let user_data = WinApiAbi::arg(emu, 5) as u32;
    let _reserved = WinApiAbi::arg(emu, 6) as u32;
    log_red!(
        emu,
        "advapi32!EventWriteTransfer handle: 0x{:x} desc: 0x{:x} activity: 0x{:x} count: {}",
        reg_handle,
        event_desc,
        activity_id,
        user_data_count
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
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
