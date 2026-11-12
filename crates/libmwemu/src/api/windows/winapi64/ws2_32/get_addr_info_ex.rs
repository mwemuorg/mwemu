use crate::emu;
use crate::winapi::common::strings;

pub fn GetAddrInfoExA(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExA", false);
}

fn get_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let service_ptr = emu.regs().rdx;
    let namespace = emu.regs().r8 as u32;
    let hints_ptr = emu.regs().r9;
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        strings::read_text(emu, service_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!{api} name:'{}' service:'{}' ns:{}",
        name,
        service,
        namespace
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn GetAddrInfoExW(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExW", true);
}
