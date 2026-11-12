use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetAddrInfoExA(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExA", false);
}

fn set_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!{api} name:'{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 12);
}

pub fn SetAddrInfoExW(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExW", true);
}
