use crate::emu;

pub fn RegCopyTreeW(emu: &mut emu::Emu) {
    let hkey_src = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let hkey_dst = emu.regs().r8;
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_wide_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegCopyTreeW src:0x{:x} '{}' dst:0x{:x}",
        hkey_src,
        subkey,
        hkey_dst
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
