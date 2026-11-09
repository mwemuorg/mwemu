use crate::emu;
use crate::winapi::helper;

pub fn GetAddrInfoExOverlappedResult(emu: &mut emu::Emu) {
    let overlapped_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!GetAddrInfoExOverlappedResult overlapped:0x{:x}",
        overlapped_ptr
    );
    let handle = helper::handler_create("GetAddrInfoExOverlappedResult");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
