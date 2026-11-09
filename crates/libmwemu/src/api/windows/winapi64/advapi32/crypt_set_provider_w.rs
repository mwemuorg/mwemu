use crate::emu;

pub fn CryptSetProviderW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let prov_type = emu.regs().rdx;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(
        emu,
        "advapi32!CryptSetProviderW '{}' type:{}",
        name,
        prov_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
