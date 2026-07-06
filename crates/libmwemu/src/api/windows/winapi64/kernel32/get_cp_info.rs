use crate::{emu, windows::structures};

pub fn GetCPInfo(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx as usize;
    let lp_cp_info = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetCPInfo code_page: {} lp_cp_info: 0x{:x}",
        code_page,
        lp_cp_info
    );

    // Create and initialize a CpInfo structure
    let cp_info = match code_page {
        // CP_ACP (0) - ANSI code page
        0 => structures::CpInfo {
            max_char_size: 2,
            default_char: [0x3F, 0], // '?'
            lead_byte: [0; 12],
        },
        // CP_OEMCP (1) - OEM code page
        1 => structures::CpInfo {
            max_char_size: 1,
            default_char: [0x3F, 0], // '?'
            lead_byte: [0; 12],
        },
        // For other code pages, use default values
        _ => structures::CpInfo::new(),
    };

    // Save the CpInfo structure to the provided memory location
    cp_info.save(lp_cp_info, &mut emu.maps);

    // Return TRUE to indicate success
    emu.regs_mut().rax = 1;
}
