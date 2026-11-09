use crate::emu;

pub fn GetComputerNameExW(emu: &mut emu::Emu) {
    let name_type = emu.regs().rcx as u32;
    let buff_ptr = emu.regs().rdx;
    let size_ptr = emu.regs().r8;

    let hostname = &emu.cfg.host_name;

    log_red!(
        emu,
        "kernel32!GetComputerNameExW type:{} => '{}'",
        name_type,
        hostname
    );

    let name = match name_type {
        0 => hostname.clone(),
        1 => format!("{}.localdomain", hostname),
        2 => "localdomain".to_string(),
        3 => format!("{}.localdomain", hostname),
        4 => hostname.clone(),
        5 => format!("{}.localdomain", hostname),
        6 => "localdomain".to_string(),
        7 => format!("{}.localdomain", hostname),
        _ => hostname.clone(),
    };

    let needed = name.chars().count() as u32 + 1;

    if buff_ptr != 0 && size_ptr != 0 {
        let buf_size = emu.maps.read_dword(size_ptr).unwrap_or(0);
        if buf_size < needed {
            emu.maps.write_dword(size_ptr, needed);
            emu.set_last_error(122); // ERROR_INSUFFICIENT_BUFFER
            emu.regs_mut().rax = 0;
            return;
        }
        emu.maps.write_wide_string(buff_ptr, &name);
        emu.maps.write_dword(size_ptr, name.chars().count() as u32);
    }

    emu.regs_mut().rax = 1;
}
