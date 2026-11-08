use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;

pub fn WSAConnect(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let name_ptr = arg(emu, 1) as u64;
    let namelen = arg(emu, 2);
    let caller_data_ptr = arg(emu, 3) as u64;
    let callee_data_ptr = arg(emu, 4) as u64;
    let sqos_ptr = arg(emu, 5) as u64;
    let gqos_ptr = arg(emu, 6) as u64;
    log_red!(
        emu,
        "ws2_32!WSAConnect sock:0x{:x} name:0x{:x} namelen:{} callerdata:0x{:x} calleedata:0x{:x} sqos:0x{:x} gqos:0x{:x}",
        sock,
        name_ptr,
        namelen,
        caller_data_ptr,
        callee_data_ptr,
        sqos_ptr,
        gqos_ptr
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if callee_data_ptr != 0 {
            emu.maps.write_dword(callee_data_ptr, 0);
            emu.maps.write_dword(callee_data_ptr + 4, 0);
        }
        emu.regs_mut().rax = 0;
    }
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
}
