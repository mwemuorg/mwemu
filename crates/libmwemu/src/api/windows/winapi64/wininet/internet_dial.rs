use crate::emu;

pub fn InternetDial(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let connect_ptr = emu.regs().rdx;
    let conn_id_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let connect = if connect_ptr != 0 {
        emu.maps.read_string(connect_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetDial connect:'{}' flags:0x{:x}",
        connect,
        flags
    );
    if conn_id_ptr != 0 {
        emu.maps.write_dword(conn_id_ptr, 1);
    }
    emu.regs_mut().rax = 0;
}

pub fn InternetDialA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let connect_ptr = emu.regs().rdx;
    let conn_id_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let connect = if connect_ptr != 0 {
        emu.maps.read_string(connect_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetDialA connect:'{}' flags:0x{:x}",
        connect,
        flags
    );
    if conn_id_ptr != 0 {
        emu.maps.write_dword(conn_id_ptr, 1);
    }
    emu.regs_mut().rax = 0;
}

pub fn InternetDialW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let connect_ptr = emu.regs().rdx;
    let conn_id_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let connect = if connect_ptr != 0 {
        emu.maps.read_wide_string(connect_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetDialW connect:'{}' flags:0x{:x}",
        connect,
        flags
    );
    if conn_id_ptr != 0 {
        emu.maps.write_dword(conn_id_ptr, 1);
    }
    emu.regs_mut().rax = 0;
}
