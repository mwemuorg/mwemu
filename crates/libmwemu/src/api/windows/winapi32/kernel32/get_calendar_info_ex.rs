use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCalendarInfoEx(emu: &mut emu::Emu) {
    let locale_ptr = arg(emu, 0) as u64;
    let calendar = arg(emu, 1);
    let cal_type = arg(emu, 2);
    let data_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetCalendarInfoEx locale:0x{:x} cal:{} type:{} data:0x{:x}",
        locale_ptr,
        calendar,
        cal_type,
        data_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
