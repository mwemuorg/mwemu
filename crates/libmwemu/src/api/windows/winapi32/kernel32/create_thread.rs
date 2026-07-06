use crate::winapi::helper;
use crate::windows::constants;
use crate::{console, emu};

pub fn CreateThread(emu: &mut emu::Emu) {
    let sec_attr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateThread cannot read sec_attr");
    let stack_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateThread cannot read stack_sz");
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!CreateThread cannot read fptr") as u64;
    let param = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!CreateThread cannot read param");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!CreateThread cannot read flags") as u64;
    let tid_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!CreateThread cannot read tid_ptr") as u64;

    if tid_ptr > 0 {
        emu.maps.write_dword(tid_ptr, 0x123);
    }

    log_red!(emu, "kernel32!CreateThread code: 0x{:x}", code);

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    if flags == constants::CREATE_SUSPENDED {
        log::trace!("\tcreated suspended!");
    }

    // TODO: match winapi64 multi threading

    let con = console::Console::new();
    con.print("Continue emulating the created thread (y/n)? ");
    let line = con.cmd();

    if line == "y" || line == "yes" {
        if emu.maps.is_mapped(code) {
            emu.regs_mut().set_eip(code);
            emu.regs_mut().rax = 0;
            emu.regs_mut().set_ecx(param as u64);
            emu.main_thread_cont = emu.gateway_return;
            emu.stack_push32(param);
            emu.stack_push32(constants::RETURN_THREAD);

            // alloc a stack vs reusing stack.
            return;
        } else {
            log::trace!("cannot emulate the thread, the function pointer is not mapped.");
        }
    }

    emu.regs_mut().rax = helper::handler_create("tid://0x123");
}
