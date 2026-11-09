use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminAcquireContext2(emu: &mut emu::Emu) {
    let cat_admin_ptr = emu.regs().rcx;
    let subsystem_ptr = emu.regs().rdx;
    log_red!(emu, "crypt32!CryptCATAdminAcquireContext2");
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatadminacquirecontext2");
    if cat_admin_ptr != 0 {
        emu.maps.write_qword(cat_admin_ptr, handle);
    }
    emu.regs_mut().rax = 1;
}
