use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddScopedPolicyIDAce(emu: &mut emu::Emu) {
    let acl = arg(emu, 0) as u64;
    let ace_revision = arg(emu, 1) as u64 as u32;
    let ace_flags = arg(emu, 2) as u64 as u32;
    let access_mask = arg(emu, 3) as u64 as u32;
    log_red!(
        emu,
        "kernel32!AddScopedPolicyIDAce acl:0x{:x} rev:{} flags:0x{:x} mask:0x{:x}",
        acl,
        ace_revision,
        ace_flags,
        access_mask
    );
    let handle = helper::handler_create("AddScopedPolicyIdAce");
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
