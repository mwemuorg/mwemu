use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    create_named_pipe(emu, "CreateNamedPipeA", false);
}

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    create_named_pipe(emu, "CreateNamedPipeW", true);
}

fn create_named_pipe(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let open_mode = emu.regs().rcx;
    let pipe_mode = emu.regs().r8;
    let instances = emu.regs().r9;
    let out_buff_sz = WinApiAbi::arg(emu, 4);
    let in_buff_sz = WinApiAbi::arg(emu, 5);
    let timeout = WinApiAbi::arg(emu, 6);
    let security = WinApiAbi::arg(emu, 7);
    let name = strings::read_text(emu, name_ptr, wide);
    if wide {
        log_red!(
            emu,
            "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
            name,
            in_buff_sz,
            out_buff_sz
        );
    } else {
        log_red!(
            emu,
            "kernel32!{api}  name:{} in: 0x{:x} out: 0x{:x}",
            name,
            in_buff_sz,
            out_buff_sz
        );
    }

    emu.regs_mut().rax = helper::handler_create(&name);
}
