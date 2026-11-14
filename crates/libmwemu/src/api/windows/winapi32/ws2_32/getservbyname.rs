use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn getservbyname(emu: &mut emu::Emu) {
    let name_ptr = WinApiAbi::arg(emu, 0);
    let proto_ptr = WinApiAbi::arg(emu, 1);
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    let proto = if proto_ptr != 0 {
        emu.maps.read_string(proto_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!getservbyname name:'{}' proto:'{}'",
        name,
        proto
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
