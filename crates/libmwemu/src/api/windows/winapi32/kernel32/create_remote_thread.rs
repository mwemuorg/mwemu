use crate::emu;
use crate::winapi::helper;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let proc_hndl =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!CreateRemoteThread cannot read the proc handle") as u64;
    let sec = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateRemoteThread cannot read the proc security thread attributs")
        as u64;
    let stack_size =
        emu.maps
            .read_dword(emu.regs().get_esp() + 8)
            .expect("kernel32!CreateRemoteThread cannot read the stack size") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!CreateRemoteThread cannot read the addr") as u64;
    let param = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!CreateRemoteThread cannot read the param");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!CreateRemoteThread cannot read the flags");
    let out_tid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("kernel32!CreateRemoteThread cannot read the tid") as u64;

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
