use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn getservbyport(emu: &mut emu::Emu) {
    let port = WinApiAbi::arg(emu, 0) as u32;
    let proto_ptr = WinApiAbi::arg(emu, 1);
    let proto = if proto_ptr != 0 {
        emu.maps.read_string(proto_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!getservbyport port:{} proto:'{}'", port, proto);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
