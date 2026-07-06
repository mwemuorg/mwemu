use crate::emu;
use crate::emu::object_handle::FileHandle;
use crate::emu::object_handle::file_handle::{FILE_SYSTEM, INVALID_HANDLE_VALUE};
use log::error;

pub fn CreateFileA(emu: &mut emu::Emu) {
    let lp_file_name = emu.regs().rcx;
    let dw_desired_access = emu.regs().rdx as u32;
    let dw_share_mode = emu.regs().r8 as u32;
    let lp_security_attributes = emu.regs().r9;
    let dw_creation_disposition =
        emu.maps
            .read_qword(emu.regs().rsp + 0x20)
            .expect("kernel32!CreateFileW cannot read dw_creation_disposition") as u32;
    let dw_flags_and_attributes =
        emu.maps
            .read_qword(emu.regs().rsp + 0x28)
            .expect("kernel32!CreateFileW cannot read dw_flags_and_attributes") as u32;
    let h_template_file = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("kernel32!CreateFileW cannot read h_template_file");

    log_red!(
        emu,
        "** {} kernel32!CreateFileA lp_file_name: 0x{:x} dw_desired_access: 0x{:x} dw_share_mode: 0x{:x} lp_security_attributes: 0x{:x} dw_creation_disposition: 0x{:x} dw_flags_and_attributes: 0x{:x}",
        emu.pos,
        lp_file_name,
        dw_desired_access,
        dw_share_mode,
        lp_security_attributes,
        dw_creation_disposition,
        dw_flags_and_attributes
    );

    let mut name_utf8 = String::new();
    if lp_file_name > 0 {
        name_utf8 = emu.maps.read_string(lp_file_name);
    }

    log_red!(
        emu,
        "** {} kernel32!CreateFileA name = {} {}",
        emu.pos,
        name_utf8,
        emu.colors.nc
    );

    // Map the Windows path to the emulator's path
    let emu_path = FILE_SYSTEM
        .get()
        .and_then(|file_system| file_system.local_to_windows_path(&name_utf8).ok());
    let emu_path_str = emu_path.unwrap().to_string();

    // Attempt to create or open the file using the FileHandle struct
    match FileHandle::new(
        emu_path_str.clone(), // Use the mapped path string
        dw_desired_access,
        dw_creation_disposition,
        dw_flags_and_attributes,
        dw_share_mode,
    ) {
        Ok(file_handle) => {
            // Use the global HANDLE_MANGEMENT to create and store the handle
            let handle_key = emu.handle_management.insert_file_handle(file_handle);
            emu.regs_mut().rax = handle_key as u64; // Handle is the slab key
            log_red!(
                emu,
                "** {} kernel32!CreateFileA SUCCESS, handle: 0x{:x}",
                emu.pos,
                handle_key
            );
        }
        Err(e) => {
            error!("CreateFileA failed for '{}': {}", name_utf8, e);
            emu.regs_mut().rax = INVALID_HANDLE_VALUE as u64;
            log_red!(
                emu,
                "** {} kernel32!CreateFileA FAILED, returning INVALID_HANDLE_VALUE",
                emu.pos
            );
        }
    }
}
