use crate::emu;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let prov_type = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let flags = emu.regs().r9;
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "advapi32!CryptSetProviderExA '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
