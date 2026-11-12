use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetUserNameA(emu: &mut emu::Emu) {
    get_user_name(emu, "GetUserNameA", false);
}

fn get_user_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(emu, "advapi32!{api} '{}' buf size: {}", user, provided);
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        strings::write_string(emu, buf, &user, wide);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}

pub(super) fn GetUserNameW(emu: &mut emu::Emu) {
    get_user_name(emu, "GetUserNameW", true);
}
