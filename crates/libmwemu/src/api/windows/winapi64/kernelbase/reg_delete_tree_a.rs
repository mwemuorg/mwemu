use crate::emu;

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegDeleteTreeA hkey:0x{:x} '{}'",
        hkey,
        subkey
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
