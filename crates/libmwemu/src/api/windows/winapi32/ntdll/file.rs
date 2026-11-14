use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::sections;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtCreateFile" => NtCreateFile(emu),
        "NtQueryInformationFile" => NtQueryInformationFile(emu),
        "NtReadFile" => NtReadFile(emu),
        "NtClose" => NtClose(emu),
        "RtlDosPathNameToNtPathName_U" => RtlDosPathNameToNtPathName_U(emu),
        "NtSetInformationFile" => NtSetInformationFile(emu),
        "NtWriteFile" => {
            let handle = arg(emu, 0);
            let buffer = arg(emu, 5) as u64;
            let length = arg(emu, 6);
            log_red!(
                emu,
                "ntdll!NtWriteFile handle:0x{:x} buf:0x{:x} len:{}",
                handle,
                buffer,
                length
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
            stdcall_ret(emu, 9);
        }
        _ => return false,
    }
    true
}

fn RtlDosPathNameToNtPathName_U(emu: &mut emu::Emu) {
    let dos_path_name_ptr = WinApiAbi::arg(emu, 0);
    let nt_path_name_ptr = WinApiAbi::arg(emu, 1);
    let nt_file_name_part_ptr = WinApiAbi::arg(emu, 2) as u32;
    let curdir_ptr = WinApiAbi::arg(emu, 3); // DirectoryInfo

    let dos_path_name = emu.maps.read_wide_string(dos_path_name_ptr);

    log_red!(emu, "ntdll!RtlDosPathNameToNtPathName_U {}", dos_path_name);

    if curdir_ptr > 0 {
        let dos_path_unicode_ptr = emu.maps.read_dword(curdir_ptr).unwrap_or(0) as u64;

        let dst_map_name = emu
            .maps
            .get_addr_name(dos_path_unicode_ptr)
            .unwrap_or_else(|| {
                log::warn!(
                    "ntdll!RtlDosPathNameToNtPathName_U writing on unmapped address 0x{:x}",
                    dos_path_unicode_ptr
                );
                ""
            });

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
        let dst_map_name = emu.maps.get_addr_name(nt_path_name_ptr).unwrap_or_else(|| {
            log::warn!(
                "ntdll!RtlDosPathNameToNtPathName_U writing on unmapped address 0x{:x}",
                nt_path_name_ptr
            );
            ""
        });

        if dst_map_name.starts_with("alloc_") {
            emu.maps.memcpy(
                nt_path_name_ptr,
                dos_path_name_ptr,
                emu.maps.sizeof_wide(dos_path_name_ptr) * 2,
            );
        } else {
            match emu.maps.alloc(255) {
                Some(a) => {
                    if emu
                        .maps
                        .create_map("nt_alloc", a, 255, Permission::READ_WRITE)
                        .is_err()
                    {
                        log::error!("ntdll!RtlDosPathNameToNtPathName_U cannot create map");
                    }
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
    let out_handle_ptr = WinApiAbi::arg(emu, 0);
    let access_mask = WinApiAbi::arg(emu, 1) as u32;
    let oattrib = WinApiAbi::arg(emu, 2);
    let iostat = WinApiAbi::arg(emu, 3) as u32;
    let alloc_sz = WinApiAbi::arg(emu, 4) as u32;
    let fattrib = WinApiAbi::arg(emu, 5) as u32;
    let share_access = WinApiAbi::arg(emu, 6) as u32;
    let create_disp = WinApiAbi::arg(emu, 7) as u32;
    let create_opt = WinApiAbi::arg(emu, 8) as u32;
    let ea_buff = WinApiAbi::arg(emu, 9) as u32;
    let ea_len = WinApiAbi::arg(emu, 10) as u32;

    let obj_name_ptr = emu.maps.read_dword(oattrib + 8).unwrap_or(0) as u64;
    let filename = emu.maps.read_wide_string(obj_name_ptr);

    log_red!(emu, "ntdll!NtCreateFile {}", filename);

    if out_handle_ptr > 0 {
        emu.maps
            .write_dword(out_handle_ptr, helper::handler_create(&filename) as u32);
    }

    for _ in 0..11 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let base_addr = WinApiAbi::arg(emu, 2);

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
    let handle = WinApiAbi::arg(emu, 0);
    let stat = WinApiAbi::arg(emu, 1) as u32;
    let fileinfo = WinApiAbi::arg(emu, 2) as u32;
    let len = WinApiAbi::arg(emu, 3) as u32;
    let fileinfocls = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "ntdll!NtQueryInformationFile");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

fn NtReadFile(emu: &mut emu::Emu) {
    let file_hndl = WinApiAbi::arg(emu, 0);
    let ev_hndl = WinApiAbi::arg(emu, 1);
    let apc_rout = WinApiAbi::arg(emu, 2) as u32;
    let apc_ctx = WinApiAbi::arg(emu, 3) as u32;
    let stat = WinApiAbi::arg(emu, 4) as u32;
    let buff = WinApiAbi::arg(emu, 5);
    let len = WinApiAbi::arg(emu, 6) as usize;
    let off = WinApiAbi::arg(emu, 7) as u32;
    let key = WinApiAbi::arg(emu, 8) as u32;

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

    emu.regs_mut().rax = STATUS_SUCCESS;
}

fn NtClose(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);

    let uri = helper::handler_get_uri(hndl);
    log_red!(emu, "ntdll!NtClose hndl: 0x{:x} uri: {}", hndl, uri);
    if sections::close(emu, hndl) {
        emu.stack_pop32(false);
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.stack_pop32(false);

    if uri.is_empty() {
        emu.regs_mut().rax = STATUS_INVALID_HANDLE;
    } else {
        emu.regs_mut().rax = STATUS_SUCCESS;
    }
}

fn NtSetInformationFile(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let length = arg(emu, 3);
    let info_class = arg(emu, 4);
    log_red!(
        emu,
        "ntdll!NtSetInformationFile handle: 0x{:x} info_class: {} length: {}",
        handle,
        info_class,
        length
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 5);
}
