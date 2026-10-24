use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingA(emu: &mut emu::Emu) {
    let hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let attr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let protect = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let maxsz_high = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let maxsz_low = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

    let mut name: String = String::new();

    if name_ptr > 0 {
        name = emu.maps.read_string(name_ptr);
    }

    emu.regs_mut().rax = helper::handler_create(&name);

    log_red!(
        emu,
        "kernel32!CreateFileMappingA {} '{}' ={}",
        name_ptr,
        name,
        emu.regs().get_eax()
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
