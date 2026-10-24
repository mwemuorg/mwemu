use crate::emu;
use crate::winapi::helper;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let open_mode = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let pipe_mode = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let instances = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let out_buff_sz = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let in_buff_sz = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let timeout = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let security = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);

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
