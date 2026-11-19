use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::sections;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "NtCreateFile" => NtCreateFile(emu),
        "NtQueryInformationFile" => NtQueryInformationFile(emu),
        "NtReadFile" => NtReadFile(emu),
        "NtClose" => NtClose(emu),
        "RtlDosPathNameToNtPathName_U" => RtlDosPathNameToNtPathName_U(emu),
        "NtSetInformationFile" => NtSetInformationFile(emu),
        "NtWriteFile" => NtWriteFile(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn NtWriteFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let buffer = api_try!(emu, call.arg_u32(emu, 5)) as u64;
    let length = api_try!(emu, call.arg_u32(emu, 6));
    log_red!(
        emu,
        "ntdll!NtWriteFile handle:0x{:x} buf:0x{:x} len:{}",
        handle,
        buffer,
        length
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 9);
}

fn RtlDosPathNameToNtPathName_U(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dos_path_name_ptr = api_try!(emu, call.arg(emu, 0));
    let nt_path_name_ptr = api_try!(emu, call.arg(emu, 1));
    let nt_file_name_part_ptr = api_try!(emu, call.arg_u32(emu, 2));
    let curdir_ptr = api_try!(emu, call.arg(emu, 3)); // DirectoryInfo

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

    call.cleanup_args(emu, 4);
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
    let call = WinApi(Stdcall);
    let out_handle_ptr = api_try!(emu, call.arg(emu, 0));
    let access_mask = api_try!(emu, call.arg_u32(emu, 1));
    let oattrib = api_try!(emu, call.arg(emu, 2));
    let iostat = api_try!(emu, call.arg_u32(emu, 3));
    let alloc_sz = api_try!(emu, call.arg_u32(emu, 4));
    let fattrib = api_try!(emu, call.arg_u32(emu, 5));
    let share_access = api_try!(emu, call.arg_u32(emu, 6));
    let create_disp = api_try!(emu, call.arg_u32(emu, 7));
    let create_opt = api_try!(emu, call.arg_u32(emu, 8));
    let ea_buff = api_try!(emu, call.arg_u32(emu, 9));
    let ea_len = api_try!(emu, call.arg_u32(emu, 10));

    let obj_name_ptr = emu.maps.read_dword(oattrib + 8).unwrap_or(0) as u64;
    let filename = emu.maps.read_wide_string(obj_name_ptr);

    log_red!(emu, "ntdll!NtCreateFile {}", filename);

    if out_handle_ptr > 0 {
        emu.maps
            .write_dword(out_handle_ptr, helper::handler_create(&filename) as u32);
    }

    call.cleanup_args(emu, 11);

    call.set_return(emu, STATUS_SUCCESS);
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let base_addr = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(handle);

    call.cleanup_args(emu, 3);

    let name = emu.maps.get_addr_name(base_addr).unwrap_or("").to_string();
    if name.is_empty() {
        if emu.cfg.verbose >= 1 {
            log::trace!("map not allocated, so cannot free it.");
        }
        call.set_return(emu, 0);
        return;
    }

    if name.starts_with("alloc_") {
        emu.maps.dealloc(base_addr);
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", name);
        }
    }
}

fn NtQueryInformationFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let stat = api_try!(emu, call.arg_u32(emu, 1));
    let fileinfo = api_try!(emu, call.arg_u32(emu, 2));
    let len = api_try!(emu, call.arg_u32(emu, 3));
    let fileinfocls = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "ntdll!NtQueryInformationFile");

    call.cleanup_args(emu, 5);

    call.set_return(emu, STATUS_SUCCESS);
}

fn NtReadFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_hndl = api_try!(emu, call.arg(emu, 0));
    let ev_hndl = api_try!(emu, call.arg(emu, 1));
    let apc_rout = api_try!(emu, call.arg_u32(emu, 2));
    let apc_ctx = api_try!(emu, call.arg_u32(emu, 3));
    let stat = api_try!(emu, call.arg_u32(emu, 4));
    let buff = api_try!(emu, call.arg(emu, 5));
    let len = api_try!(emu, call.arg(emu, 6)) as usize;
    let off = api_try!(emu, call.arg_u32(emu, 7));
    let key = api_try!(emu, call.arg_u32(emu, 8));

    let file = helper::handler_get_uri(file_hndl);

    log_red!(
        emu,
        "ntdll!NtReadFile {} buff: 0x{:x} sz: {} off_var: 0x{:x}",
        file,
        buff,
        len,
        off
    );

    call.cleanup_args(emu, 9);

    emu.maps.memset(buff, 0x90, len);

    call.set_return(emu, STATUS_SUCCESS);
}

fn NtClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));

    let uri = helper::handler_get_uri(hndl);
    log_red!(emu, "ntdll!NtClose hndl: 0x{:x} uri: {}", hndl, uri);
    if sections::close(emu, hndl) {
        call.cleanup_args(emu, 1);
        call.set_return(emu, STATUS_SUCCESS);
        return;
    }

    call.cleanup_args(emu, 1);

    if uri.is_empty() {
        call.set_return(emu, STATUS_INVALID_HANDLE);
    } else {
        call.set_return(emu, STATUS_SUCCESS);
    }
}

fn NtSetInformationFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0));
    let length = api_try!(emu, call.arg_u32(emu, 3));
    let info_class = api_try!(emu, call.arg_u32(emu, 4));
    log_red!(
        emu,
        "ntdll!NtSetInformationFile handle: 0x{:x} info_class: {} length: {}",
        handle,
        info_class,
        length
    );
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 5);
}
