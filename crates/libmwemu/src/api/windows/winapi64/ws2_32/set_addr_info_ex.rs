use crate::emu;
use crate::winapi::common::strings;

pub fn SetAddrInfoExA(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExA", false);
}

fn set_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let service_ptr = emu.regs().rdx;
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!{api} name:'{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn SetAddrInfoExW(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExW", true);
}
