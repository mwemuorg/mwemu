use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn MapVirtualKeyA(emu: &mut emu::Emu) {
    map_virtual_key(emu, "MapVirtualKeyA");
}

fn map_virtual_key(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

pub(super) fn MapVirtualKeyW(emu: &mut emu::Emu) {
    map_virtual_key(emu, "MapVirtualKeyW");
}
