use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetAddrInfoExA(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExA", false);
}

fn get_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = WinApiAbi::arg(emu, 0);
    let service_ptr = WinApiAbi::arg(emu, 1);
    let namespace = WinApiAbi::arg(emu, 2) as u32;
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
    stdcall_ret(emu, 10);
}

pub fn GetAddrInfoExW(emu: &mut emu::Emu) {
    get_addr_info_ex(emu, "GetAddrInfoExW", true);
}
