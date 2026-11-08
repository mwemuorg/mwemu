use crate::emu;

pub fn DllGetActivationFactory(emu: &mut emu::Emu) {
    let class_id_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let iid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let factory_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(emu, "shell32!DllGetActivationFactory");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80004001u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
