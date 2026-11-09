use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetKeyboardType(emu: &mut emu::Emu) {
    let ntype = arg(emu, 0) as i32;
    let result: u32 = match ntype {
        0 => 4,
        1 => 0,
        2 => 12,
        _ => 0,
    };
    log_red!(emu, "user32!GetKeyboardType type:{} = {}", ntype, result);
    emu.regs_mut().rax = result as u64;
    stdcall_ret(emu, 1);
}
