use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let watch_subtree = WinApiAbi::arg(emu, 1) as u32 != 0;
    let filter = WinApiAbi::arg(emu, 2) as u32;
    let event = WinApiAbi::arg(emu, 3) as u32;
    let async_ = WinApiAbi::arg(emu, 4) as u32 != 0;
    log_red!(
        emu,
        "advapi32!RegNotifyChangeKeyValue hkey:0x{:x} subtree:{} filter:0x{:x}",
        hkey,
        watch_subtree,
        filter
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
