use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetAppContainerNamedObjectPath(emu: &mut emu::Emu) {
    let token = arg(emu, 0) as u64;
    let sid_ptr = arg(emu, 1) as u64;
    let object_path_length = arg(emu, 2);
    let object_path_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetAppContainerNamedObjectPath token:0x{:x} sid:0x{:x}",
        token,
        sid_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
