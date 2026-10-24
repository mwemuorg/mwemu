use crate::emu;
use crate::emu::object_handle::FileHandle;
use crate::emu::object_handle::file_handle::INVALID_HANDLE_VALUE;
use log::error;

pub fn CreateFileW(emu: &mut emu::Emu) {
    let lp_file_name_wide = emu.regs().rcx;
    let dw_desired_access = emu.regs().rdx as u32;
    let dw_share_mode = emu.regs().r8 as u32;
    let lp_security_attributes = emu.regs().r9; // LPSECURITY_ATTRIBUTES
    let dw_creation_disposition = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32;
    let dw_flags_and_attributes = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
    let h_template_file = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(
        emu,
        "** {} kernel32!CreateFileW lp_file_name: 0x{:x} dw_desired_access: 0x{:x} dw_share_mode: 0x{:x} lp_security_attributes: 0x{:x} dw_creation_disposition: 0x{:x} dw_flags_and_attributes: 0x{:x}",
        emu.pos,
        lp_file_name_wide,
        dw_desired_access,
        dw_share_mode,
        lp_security_attributes,
        dw_creation_disposition,
        dw_flags_and_attributes
    );

    let name_utf8 = if lp_file_name_wide > 0 {
        emu.maps.read_wide_string(lp_file_name_wide)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "** {} kernel32!CreateFileW name = {} {}",
        emu.pos,
        name_utf8,
        emu.colors.nc
    );

    // Attempt to create or open the file using the FileHandle struct
    match FileHandle::new(
        name_utf8.clone(),
        dw_desired_access,
        dw_creation_disposition,
        dw_flags_and_attributes,
        dw_share_mode, // Pass the parsed struct
    ) {
        Ok(file_handle) => {
            let handle_key = emu.handle_management.insert_file_handle(file_handle);
            emu.regs_mut().rax = handle_key as u64;
            log_red!(
                emu,
                "** {} kernel32!CreateFileW SUCCESS, handle: 0x{:x}",
                emu.pos,
                handle_key
            );
        }
        Err(e) => {
            error!("CreateFileW failed for '{}': {}", name_utf8, e);
            emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
            emu.regs_mut().rax = INVALID_HANDLE_VALUE as u64;
            log_red!(
                emu,
                "** {} kernel32!CreateFileW FAILED, returning INVALID_HANDLE_VALUE",
                emu.pos
            );
        }
    }
}
