use crate::emu;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let info_buf = emu.regs().r8;
    let buf_len = emu.regs().r9;
    let ret_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let needed: u32 = match info_class {
        1 => 44, // TokenUser: TOKEN_USER (SID ptr + attrs)
        2 => 68, // TokenGroups
        3 => 44, // TokenPrivileges
        20 => 4, // TokenElevationType: DWORD
        25 => 4, // TokenIntegrityLevel
        _ => 4,
    };

    if ret_len_ptr != 0 {
        emu.maps.write_dword(ret_len_ptr, needed);
    }

    if info_buf != 0 && buf_len >= needed as u64 {
        match info_class {
            20 => {
                emu.maps.write_dword(info_buf, 2); // TokenElevationTypeFull
            }
            25 => {
                emu.maps.write_dword(info_buf, 0x3000); // SECURITY_MANDATORY_HIGH_RID
            }
            _ => {
                emu.maps.write_dword(info_buf, 1);
            }
        }
    }

    log_red!(
        emu,
        "advapi32!GetTokenInformation token:0x{:x} class:{} needed:{}",
        token_handle,
        info_class,
        needed
    );
    emu.regs_mut().rax = 1;
}
