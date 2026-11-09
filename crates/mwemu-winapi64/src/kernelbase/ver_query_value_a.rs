use crate::emu;

pub(super) fn VerQueryValueA(emu: &mut emu::Emu) {
    let p_block = emu.regs().rcx as usize;
    let lp_sub_block = emu.regs().rdx;
    let lplp_buffer = emu.regs().r8 as usize;
    let pu_len = emu.regs().r9 as usize;

    let sub_block = if lp_sub_block > 0 {
        emu.maps.read_string(lp_sub_block)
    } else {
        "\\".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!VerQueryValueA p_block: 0x{:x} lp_sub_block: {} lplp_buffer: 0x{:x} pu_len: 0x{:x}",
        emu.pos,
        p_block,
        sub_block,
        lplp_buffer,
        pu_len
    );

    if sub_block == "\\" {
        // Root query returns pointer to VS_FIXEDFILEINFO
        // The VS_FIXEDFILEINFO starts at offset 0x28 in the version block
        let fixed_info_ptr = (p_block + 0x28) as u64;
        emu.maps.write_qword(lplp_buffer as u64, fixed_info_ptr);
        emu.maps.write_dword(pu_len as u64, 52); // Size of VS_FIXEDFILEINFO
        emu.regs_mut().rax = 1;
    } else if sub_block.starts_with("\\StringFileInfo\\") {
        // String queries - allocate and return string data
        let string_data = match sub_block.as_str() {
            "\\StringFileInfo\\040904B0\\CompanyName" => "Microsoft Corporation\0",
            "\\StringFileInfo\\040904B0\\FileDescription" => "User Experience Controls Library\0",
            "\\StringFileInfo\\040904B0\\FileVersion" => "6.10 (WinBuild.160101.0800)\0",
            "\\StringFileInfo\\040904B0\\InternalName" => "comctl32\0",
            "\\StringFileInfo\\040904B0\\LegalCopyright" => {
                "© Microsoft Corporation. All rights reserved.\0"
            }
            "\\StringFileInfo\\040904B0\\OriginalFilename" => "comctl32.DLL\0",
            "\\StringFileInfo\\040904B0\\ProductName" => "Microsoft® Windows® Operating System\0",
            "\\StringFileInfo\\040904B0\\ProductVersion" => "10.0.22621.4541\0",
            _ => "\0",
        };

        let string_addr = match emu.maps.alloc(string_data.len() as u64) {
            Some(a) => a,
            None => {
                log::error!("kernelbase!VerQueryValueA: out of memory");
                emu.regs_mut().rax = 0;
                return;
            }
        };
        emu.maps.write_string(string_addr, string_data);
        emu.maps.write_qword(lplp_buffer as u64, string_addr);
        emu.maps
            .write_dword(pu_len as u64, string_data.len() as u32);
        emu.regs_mut().rax = 1;
    } else if sub_block == "\\VarFileInfo\\Translation" {
        // Translation array
        let trans_addr = match emu.maps.alloc(4) {
            Some(a) => a,
            None => {
                log::error!("kernelbase!VerQueryValueA: out of memory (translation)");
                emu.regs_mut().rax = 0;
                return;
            }
        };
        emu.maps.write_dword(trans_addr, 0x04B00409); // Language and codepage
        emu.maps.write_qword(lplp_buffer as u64, trans_addr);
        emu.maps.write_dword(pu_len as u64, 4);
        emu.regs_mut().rax = 1;
    } else {
        log::trace!("VerQueryValueA: Unknown sub_block: {}", sub_block);
        emu.regs_mut().rax = 0; // Failure
    }
}
