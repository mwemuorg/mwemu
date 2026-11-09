use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn OleCreateLinkToFile(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    let _a1 = arg(emu, 1);
    log_red!(
        emu,
        "ole32!OleCreateLinkToFile a0:0x{:x} a1:0x{:x}",
        _a0,
        _a1
    );
    let hndl = helper::handler_create("ole32://OleCreateLinkToFile");
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, hndl as u32);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
