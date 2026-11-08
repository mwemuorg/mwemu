use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetUserNameA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!GetUserNameA '{}' buf size: {}",
        user,
        provided
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_string(buf, &user);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
