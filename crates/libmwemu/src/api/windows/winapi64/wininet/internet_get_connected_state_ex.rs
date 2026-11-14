use crate::emu;

pub fn InternetGetConnectedStateEx(emu: &mut emu::Emu) {
    let flags_ptr = emu.regs().rcx;
    log_red!(emu, "wininet!InternetGetConnectedStateEx");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetGetConnectedStateExA(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExA");
}

fn internet_get_connected_state_ex(emu: &mut emu::Emu, api: &str) {
    let flags_ptr = emu.regs().rcx;
    log_red!(emu, "wininet!{api}");
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x20);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetGetConnectedStateExW(emu: &mut emu::Emu) {
    internet_get_connected_state_ex(emu, "InternetGetConnectedStateExW");
}
