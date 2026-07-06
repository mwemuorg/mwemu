use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtCreateFile" => NtCreateFile(emu),
        "NtQueryInformationFile" => NtQueryInformationFile(emu),
        "NtReadFile" => NtReadFile(emu),
        "NtClose" => NtClose(emu),
        "RtlDosPathNameToNtPathName_U" => RtlDosPathNameToNtPathName_U(emu),
        _ => return false,
    }
    true
}

fn RtlDosPathNameToNtPathName_U(emu: &mut emu::Emu) {
    let dos_path_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlDosPathNameToNtPathName_U error reading dos_path_name_ptr param")
        as u64;
    let nt_path_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlDosPathNameToNtPathName_U error reading nt_path_name_ptr param")
        as u64;
    let nt_file_name_part_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlDosPathNameToNtPathName_U error reading nt_file_name_part_ptr param");
    let curdir_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!RtlDosPathNameToNtPathName_U error reading curdir_ptr param")
        as u64; // DirectoryInfo

    let dos_path_name = emu.maps.read_wide_string(dos_path_name_ptr);

    log_red!(emu, "ntdll!RtlDosPathNameToNtPathName_U {}", dos_path_name);

    if curdir_ptr > 0 {
        let dos_path_unicode_ptr = emu
            .maps
            .read_dword(curdir_ptr)
            .expect("ntdll!RtlDosPathNameToNtPathName_U error reading dos_path_unicode_ptr")
            as u64;

        let dst_map_name = emu
            .maps
            .get_addr_name(dos_path_unicode_ptr)
            .expect("ntdll!RtlDosPathNameToNtPathName_U writting on unmapped address");

        if dst_map_name.starts_with("alloc_") {
            emu.maps.memcpy(
                dos_path_unicode_ptr,
                dos_path_name_ptr,
                emu.maps.sizeof_wide(dos_path_name_ptr) * 2,
            );
        } else if emu.cfg.verbose >= 1 {
            log::trace!(
                "/!\\ ntdll!RtlDosPathNameToNtPathName_U denied dest buffer on {} map",
                dst_map_name
            );
            log::trace!(
                "memcpy1 0x{:x} <- 0x{:x}  sz: {}",
                dos_path_unicode_ptr,
                dos_path_name_ptr,
                emu.maps.sizeof_wide(dos_path_name_ptr) * 2
            );
        }
    }

    if nt_path_name_ptr > 0 {
        let dst_map_name = emu
            .maps
            .get_addr_name(nt_path_name_ptr)
            .expect("ntdll!RtlDosPathNameToNtPathName_U writting on unmapped address.");

        if dst_map_name.starts_with("alloc_") {
            emu.maps.memcpy(
                nt_path_name_ptr,
                dos_path_name_ptr,
                emu.maps.sizeof_wide(dos_path_name_ptr) * 2,
            );
        } else {
            match emu.maps.alloc(255) {
                Some(a) => {
                    emu.maps
                        .create_map("nt_alloc", a, 255, Permission::READ_WRITE)
                        .expect("ntdll!RtlDosPathNameToNtPathName_U cannot create map");
                    emu.maps.write_dword(nt_path_name_ptr, a as u32);
                    emu.maps.memcpy(
                        a,
                        dos_path_name_ptr,
                        emu.maps.sizeof_wide(dos_path_name_ptr) * 2,
                    );
                }
                None => {
                    if emu.cfg.verbose >= 1 {
                        log::trace!("/!\\ ntdll!RtlDosPathNameToNtPathName_U low memory");
                    }
                }
            };
        }
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

/*
__kernel_entry NTSTATUS NtCreateFile(
  [out]          PHANDLE            FileHandle,
  [in]           ACCESS_MASK        DesiredAccess,
  [in]           POBJECT_ATTRIBUTES ObjectAttributes,
  [out]          PIO_STATUS_BLOCK   IoStatusBlock,
  [in, optional] PLARGE_INTEGER     AllocationSize,
  [in]           ULONG              FileAttributes,
  [in]           ULONG              ShareAccess,
  [in]           ULONG              CreateDisposition,
  [in]           ULONG              CreateOptions,
  [in]           PVOID              EaBuffer,
  [in]           ULONG              EaLength
);
*/
fn NtCreateFile(emu: &mut emu::Emu) {
    let out_handle_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("ntdll!NtCreateFile error reading out_handle_ptr param") as u64;
    let access_mask = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!NtCreateFile error reading access_mask param");
    let oattrib = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!NtCreateFile error reading oattrib param") as u64;
    let iostat = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!NtCreateFile error reading iostat param");
    let alloc_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ntdll!NtCreateFile error reading alloc_sz param");
    let fattrib = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("ntdll!NtCreateFile error reading fattrib param");
    let share_access = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("ntdll!NtCreateFile error reading share_access param");
    let create_disp = emu
        .maps
        .read_dword(emu.regs().get_esp() + 28)
        .expect("ntdll!NtCreateFile error reading create_disp param");
    let create_opt = emu
        .maps
        .read_dword(emu.regs().get_esp() + 32)
        .expect("ntdll!NtCreateFile error reading create_opt param");
    let ea_buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 36)
        .expect("ntdll!NtCreateFile error reading ea_buff param");
    let ea_len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 40)
        .expect("ntdll!NtCreateFile error reading ea_len param");

    let obj_name_ptr = emu
        .maps
        .read_dword(oattrib + 8)
        .expect("ntdll!NtCreateFile error reading oattrib +8") as u64;
    let filename = emu.maps.read_wide_string(obj_name_ptr);

    log_red!(emu, "ntdll!NtCreateFile {}", filename);

    if out_handle_ptr > 0 {
        emu.maps
            .write_dword(out_handle_ptr, helper::handler_create(&filename) as u32);
    }

    for _ in 0..11 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlFreeHeap error reading handle param") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlFreeHeap error reading flags param");
    let base_addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlFreeHeap error reading base_addr param") as u64;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(handle);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let name = emu.maps.get_addr_name(base_addr).unwrap_or("").to_string();
    if name.is_empty() {
        if emu.cfg.verbose >= 1 {
            log::trace!("map not allocated, so cannot free it.");
        }
        emu.regs_mut().rax = 0;
        return;
    }

    if name.starts_with("alloc_") {
        emu.maps.dealloc(base_addr);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", name);
        }
    }
}

fn NtQueryInformationFile(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtQueryInformationFile error reading handle param") as u64;
    let stat = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!NtQueryInformationFile error reading stat param");
    let fileinfo = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!NtQueryInformationFile error reading fileinfo param");
    let len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!NtQueryInformationFile error reading len param");
    let fileinfocls = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ntdll!NtQueryInformationFile error reading fileinfocls param");

    log_red!(emu, "ntdll!NtQueryInformationFile");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtReadFile error reading file_hndl param") as u64;
    let ev_hndl = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!NtReadFile error reading ev_hndl param") as u64;
    let apc_rout = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!NtReadFile error reading apc_rout param");
    let apc_ctx = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!NtReadFile error reading apc_ctx param");
    let stat = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ntdll!NtReadFile error reading stat param");
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("ntdll!NtReadFile error reading buff param") as u64;
    let len = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("ntdll!NtReadFile error reading len param") as usize;
    let off = emu
        .maps
        .read_dword(emu.regs().get_esp() + 28)
        .expect("ntdll!NtReadFile error reading off param");
    let key = emu
        .maps
        .read_dword(emu.regs().get_esp() + 32)
        .expect("ntdll!NtReadFile error reading key param");

    let file = helper::handler_get_uri(file_hndl);

    log_red!(
        emu,
        "ntdll!NtReadFile {} buff: 0x{:x} sz: {} off_var: 0x{:x}",
        file,
        buff,
        len,
        off
    );

    for _ in 0..9 {
        emu.stack_pop32(false);
    }

    emu.maps.memset(buff, 0x90, len);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtClose(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!NtClose error reading hndl param") as u64;

    let uri = helper::handler_get_uri(hndl);

    log_red!(emu, "ntdll!NtClose hndl: 0x{:x} uri: {}", hndl, uri);

    emu.stack_pop32(false);

    if uri.is_empty() {
        emu.regs_mut().rax = constants::STATUS_INVALID_HANDLE;
    } else {
        emu.regs_mut().rax = constants::STATUS_SUCCESS;
    }
}
