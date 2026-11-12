use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let proc_hndl = WinApiAbi::arg(emu, 0);
    let sec = WinApiAbi::arg(emu, 1);
    let stack_size = WinApiAbi::arg(emu, 2);
    let addr = WinApiAbi::arg(emu, 3);
    let param = WinApiAbi::arg(emu, 4) as u32;
    let flags = WinApiAbi::arg(emu, 5) as u32;
    let out_tid = WinApiAbi::arg(emu, 6);

    log_red!(
        emu,
        "kernel32!CreateRemoteThread hproc: 0x{:x} addr: 0x{:x}",
        proc_hndl,
        addr
    );

    emu.maps.write_dword(out_tid, 0x123);
    emu.regs_mut().rax = helper::handler_create("tid://0x123");

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
