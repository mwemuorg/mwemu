use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingW(emu: &mut emu::Emu) {
    let hFile = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateFileMappingW cannot read hFile param");
    let attr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateFileMappingW cannot read attr param");
    let protect = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!CreateFileMappingW cannot read protect");
    let maxsz_high = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!CreateFileMappingW cannot read max size high");
    let maxsz_low = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!CreateFileMappingW cannot read max size low");
    let name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!CreateFileMappingW cannot read name ptr") as u64;

    let mut name: String = String::new();

    if name_ptr > 0 {
        name = emu.maps.read_wide_string(name_ptr);
    }

    emu.regs_mut().rax = helper::handler_create(&name);

    log_red!(
        emu,
        "kernel32!CreateFileMappingW '{}' ={}",
        name,
        emu.regs().get_eax()
    );

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
