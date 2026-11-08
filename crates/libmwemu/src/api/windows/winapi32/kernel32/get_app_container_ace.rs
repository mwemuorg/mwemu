use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetAppContainerAce(emu: &mut emu::Emu) {
    let acl_ptr = arg(emu, 0) as u64;
    let starting_index = arg(emu, 1);
    let ace_ptr_ptr = arg(emu, 2) as u64;
    let app_container_ace_index_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetAppContainerAce acl:0x{:x} start:{}",
        acl_ptr,
        starting_index
    );
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
