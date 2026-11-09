use crate::emu;
use crate::winapi::helper;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let proc_hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let stack_size = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let addr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let out_tid = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;

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
