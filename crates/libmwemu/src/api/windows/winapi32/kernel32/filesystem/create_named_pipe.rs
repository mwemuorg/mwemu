use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    let name_ptr = WinApiAbi::arg(emu, 0);
    let open_mode = WinApiAbi::arg(emu, 1) as u32;
    let pipe_mode = WinApiAbi::arg(emu, 2) as u32;
    let instances = WinApiAbi::arg(emu, 3) as u32;
    let out_buff_sz = WinApiAbi::arg(emu, 4) as u32;
    let in_buff_sz = WinApiAbi::arg(emu, 5) as u32;
    let timeout = WinApiAbi::arg(emu, 6) as u32;
    let security = WinApiAbi::arg(emu, 7) as u32;

    let name = emu.maps.read_string(name_ptr);

    log_red!(
        emu,
        "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
        name,
        in_buff_sz,
        out_buff_sz
    );

    for _ in 0..8 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = helper::handler_create(&name);
}

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    for _ in 0..8u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateNamedPipeW");
    emu.regs_mut().rax = helper::handler_create("CreateNamedPipeW");
}
