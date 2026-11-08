use crate::emu;

pub fn CryptSetProviderA(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "advapi32!CryptSetProviderA '{}' type:{}",
        name,
        prov_type
    );
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
