use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CreateWindowExA(emu: &mut emu::Emu) {
    let class = emu.maps.read_string(arg(emu, 1) as u64);
    let title = emu.maps.read_string(arg(emu, 2) as u64);
    log_red!(
        emu,
        "user32!CreateWindowExA class:'{}' title:'{}'",
        class,
        title
    );
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 12);
}
