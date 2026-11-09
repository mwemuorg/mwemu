use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetObjectW(emu: &mut emu::Emu) {
    let obj = arg(emu, 0);
    let size = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    log_red!(
        emu,
        "gdi32!GetObjectW obj:0x{:x} size:{} buf:0x{:x}",
        obj,
        size,
        buffer
    );
    if buffer != 0 && size > 0 {
        let fill = size.min(64) as usize;
        for i in 0..fill {
            emu.maps.write_byte(buffer + i as u64, 0);
        }
        emu.regs_mut().rax = fill as u64;
    } else {
        helper::advance_tick(emu, 1);
        emu.regs_mut().rax = 24;
    }
    stdcall_ret(emu, 3);
}
