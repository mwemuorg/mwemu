use std::fs::File;
use std::io::{Read as _, Seek as _, SeekFrom};

use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "NtCreateFile" => NtCreateFile(emu),
        "NtQueryInformationFile" => NtQueryInformationFile(emu),
        "NtSetInformationFile" => NtSetInformationFile(emu),
        "NtReadFile" => NtReadFile(emu),
        "NtClose" => NtClose(emu),
        "RtlDosPathNameToNtPathName_U" => RtlDosPathNameToNtPathName_U(emu),
        _ => return false,
    }
    true
}

/*
BOOLEAN
NTAPI
RtlDosPathNameToNtPathName_U(IN PCWSTR DosName,
                             OUT PUNICODE_STRING NtName,
                             OUT PCWSTR *PartName,
                             OUT PRTL_RELATIVE_NAME_U RelativeName)
*/
fn RtlDosPathNameToNtPathName_U(emu: &mut emu::Emu) {
    let dos_path_name_ptr = emu.regs().rcx;
    let nt_path_name_ptr = emu.regs().rdx; // This should point to a UNICODE_STRING structure
    let nt_file_name_part_ptr = emu.regs().r8;
    let curdir_ptr = emu.regs().r9;

    let dos_path_name = emu.maps.read_wide_string(dos_path_name_ptr);

    log_red!(
        emu,
        "ntdll!RtlDosPathNameToNtPathName_U dos_path='{}' dos_path_name_ptr: 0x{dos_path_name_ptr:x} nt_path_name_ptr: 0x{nt_path_name_ptr:x} nt_file_name_part_ptr: 0x{nt_file_name_part_ptr:x} curdir_ptr: 0x{curdir_ptr:x}",
        dos_path_name
    );

    let nt_path = if dos_path_name.starts_with("\\\\?\\") {
        format!("\\??\\{}", &dos_path_name[4..])
    } else if dos_path_name.len() >= 2 && dos_path_name.chars().nth(1) == Some(':') {
        format!("\\??\\{}", dos_path_name)
    } else {
        format!("\\??\\{}", dos_path_name)
    };

    log_red!(
        emu,
        "Converted DOS path '{}' to NT path '{}'",
        dos_path_name,
        nt_path
    );

    if nt_path_name_ptr > 0 {
        let string_length_bytes = nt_path.encode_utf16().count() * 2;

        match emu.maps.alloc((string_length_bytes + 2) as u64) {
            Some(string_buffer_addr) => {
                emu.maps
                    .create_map(
                        &format!("nt_path_string_{:x}", string_buffer_addr),
                        string_buffer_addr,
                        (string_length_bytes + 2) as u64,
                        Permission::READ_WRITE,
                    )
                    .expect("Failed to create nt_path_string map");

                emu.maps.write_wide_string(string_buffer_addr, &nt_path);
                emu.maps
                    .write_word(nt_path_name_ptr, string_length_bytes as u16);
                emu.maps
                    .write_word(nt_path_name_ptr + 2, (string_length_bytes + 2) as u16);
                emu.maps
                    .write_qword(nt_path_name_ptr + 8, string_buffer_addr);

                log_red!(
                    emu,
                    "Created UNICODE_STRING: Length={}, MaxLength={}, Buffer=0x{:x}",
                    string_length_bytes,
                    string_length_bytes + 2,
                    string_buffer_addr
                );

                if nt_file_name_part_ptr > 0 {
                    if let Some(last_backslash_pos) = nt_path.rfind('\\') {
                        let filename_offset = (last_backslash_pos + 1) * 2;
                        let filename_part_addr = string_buffer_addr + filename_offset as u64;
                        emu.maps
                            .write_qword(nt_file_name_part_ptr, filename_part_addr);
                        log_red!(
                            emu,
                            "Set filename part pointer to 0x{:x}",
                            filename_part_addr
                        );
                    } else {
                        emu.maps
                            .write_qword(nt_file_name_part_ptr, string_buffer_addr);
                    }
                }
            }
            None => {
                log_red!(emu, "Failed to allocate memory for NT path string");
                emu.regs_mut().rax = 0;
                return;
            }
        }
    }

    if curdir_ptr > 0 {
        log_red!(emu, "CurDir handling not fully implemented");
    }

    emu.regs_mut().rax = 1;
}

fn NtCreateFile(emu: &mut emu::Emu) {
    let out_hndl_ptr = emu.regs().rcx;
    let access_mask = emu.regs().rdx;
    let oattrib = emu.regs().r8;
    let iostat = emu.regs().r9;
    let alloc_sz = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtCreateFile error reading alloc_sz param");
    let fattrib = emu
        .maps
        .read_dword(emu.regs().rsp + 0x28)
        .expect("ntdll!NtCreateFile error reading fattrib param");
    let share_access = emu
        .maps
        .read_dword(emu.regs().rsp + 0x30)
        .expect("ntdll!NtCreateFile error reading share_access param");
    let create_disp = emu
        .maps
        .read_dword(emu.regs().rsp + 0x38)
        .expect("ntdll!NtCreateFile error reading create_disp param");
    let create_opt = emu
        .maps
        .read_dword(emu.regs().rsp + 0x40)
        .expect("ntdll!NtCreateFile error reading create_opt param");
    let ea_buff = emu
        .maps
        .read_qword(emu.regs().rsp + 0x48)
        .expect("ntdll!NtCreateFile error reading ea_buff param");
    let ea_len = emu
        .maps
        .read_dword(emu.regs().rsp + 0x50)
        .expect("ntdll!NtCreateFile error reading ea_len param");

    log_red!(
        emu,
        "** {} ntdll!NtCreateFile | Handle=0x{:x} Access=0x{:x} ObjAttr=0x{:x} IoStat=0x{:x} AllocSz=0x{:x} FileAttr=0x{:x} ShareAccess=0x{:x} CreateDisp=0x{:x} CreateOpt=0x{:x} EaBuff=0x{:x} EaLen=0x{:x}",
        emu.pos,
        out_hndl_ptr,
        access_mask,
        oattrib,
        iostat,
        alloc_sz,
        fattrib,
        share_access,
        create_disp,
        create_opt,
        ea_buff,
        ea_len
    );

    let filename = if oattrib != 0 {
        log_red!(
            emu,
            "** {} Reading OBJECT_ATTRIBUTES at 0x{:x}",
            emu.pos,
            oattrib
        );

        log_red!(emu, "** {} OBJECT_ATTRIBUTES structure dump:", emu.pos);
        for i in (0..0x30).step_by(8) {
            if let Some(qword_val) = emu.maps.read_qword(oattrib + i) {
                log_red!(emu, "** {}   +0x{:02x}: 0x{:016x}", emu.pos, i, qword_val);
            }
        }

        let root_directory = emu.maps.read_qword(oattrib + 0x08).unwrap_or(0);
        let obj_name_ptr = emu.maps.read_qword(oattrib + 0x10).unwrap_or(0);

        log_red!(
            emu,
            "** {} RootDirectory: 0x{:x}, ObjectName pointer: 0x{:x}",
            emu.pos,
            root_directory,
            obj_name_ptr
        );

        if obj_name_ptr == 0 {
            log_red!(
                emu,
                "** {} ObjectName is NULL - creating unnamed object",
                emu.pos
            );

            if root_directory != 0 {
                String::from("<unnamed_object_with_root>")
            } else {
                String::from("<unnamed_object>")
            }
        } else if !emu.maps.is_mapped(obj_name_ptr) {
            log_red!(
                emu,
                "** {} ObjectName pointer 0x{:x} is not mapped",
                emu.pos,
                obj_name_ptr
            );
            String::from("<invalid_objname_ptr>")
        } else {
            log_red!(
                emu,
                "** {} Reading UNICODE_STRING at 0x{:x}",
                emu.pos,
                obj_name_ptr
            );

            for i in (0..16).step_by(8) {
                if let Some(qword_val) = emu.maps.read_qword(obj_name_ptr + i) {
                    log_red!(
                        emu,
                        "** {} UNICODE_STRING +0x{:02x}: 0x{:016x}",
                        emu.pos,
                        i,
                        qword_val
                    );
                }
            }

            let length = emu.maps.read_word(obj_name_ptr).unwrap_or(0);
            let max_length = emu.maps.read_word(obj_name_ptr + 2).unwrap_or(0);
            let buffer_ptr = emu.maps.read_qword(obj_name_ptr + 8).unwrap_or(0);

            log_red!(
                emu,
                "** {} UNICODE_STRING: Length={} MaxLength={} Buffer=0x{:x}",
                emu.pos,
                length,
                max_length,
                buffer_ptr
            );

            if buffer_ptr == 0 {
                log_red!(emu, "** {} UNICODE_STRING Buffer is NULL", emu.pos);

                if root_directory != 0 {
                    String::from("<null_buffer_with_root>")
                } else {
                    String::from("<null_buffer>")
                }
            } else if length == 0 {
                log_red!(
                    emu,
                    "** {} UNICODE_STRING Length is 0 (empty string)",
                    emu.pos
                );

                if root_directory != 0 {
                    String::from("<empty_string_with_root>")
                } else {
                    String::from("<empty_string>")
                }
            } else if !emu.maps.is_mapped(buffer_ptr) {
                log_red!(
                    emu,
                    "** {} UNICODE_STRING Buffer 0x{:x} is not mapped",
                    emu.pos,
                    buffer_ptr
                );
                String::from("<invalid_buffer_ptr>")
            } else {
                let char_count = (length / 2) as usize;
                let filename_str = emu.maps.read_wide_string_n(buffer_ptr, char_count);

                log_red!(emu, "** {} Filename: '{}'", emu.pos, filename_str);

                if root_directory != 0 {
                    format!("<root_0x{:x}>\\{}", root_directory, filename_str)
                } else {
                    filename_str
                }
            }
        }
    } else {
        log_red!(emu, "** {} OBJECT_ATTRIBUTES pointer is null", emu.pos);
        String::from("<null_oattrib>")
    };

    log_red!(
        emu,
        "** {} ntdll!NtCreateFile resolved filename: '{}'",
        emu.pos,
        filename
    );

    if out_hndl_ptr > 0 {
        emu.maps
            .write_qword(out_hndl_ptr, helper::handler_create(&filename) as u64);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtQueryInformationFile(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let stat = emu.regs().rdx;
    let fileinfo = emu.regs().r8;
    let len = emu.regs().r9;
    let fileinfoctls = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtQueryInformationFile cannot read fileinfoctls param");

    log_red!(emu, "ntdll!NtQueryInformationFile");

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtSetInformationFile(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let io_status_block = emu.regs().rdx;
    let file_information = emu.regs().r8;
    let length = emu.regs().r9;
    let file_information_class = emu
        .maps
        .read_dword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtSetInformationFile cannot read FileInformationClass param");

    log_red!(
        emu,
        "ntdll!NtSetInformationFile handle: 0x{:x} info_class: {} length: {}",
        file_handle,
        file_information_class,
        length
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

/*
NTSTATUS NtReadFile(
  _In_     HANDLE           FileHandle,
  _In_opt_ HANDLE           Event,
  _In_opt_ PIO_APC_ROUTINE  ApcRoutine,
  _In_opt_ PVOID            ApcContext,
  _Out_    PIO_STATUS_BLOCK IoStatusBlock,
  _Out_    PVOID            Buffer,
  _In_     ULONG            Length,
  _In_opt_ PLARGE_INTEGER   ByteOffset,
  _In_opt_ PULONG           Key
);
*/
fn NtReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let ev_hndl = emu.regs().rdx;
    let apc_rout = emu.regs().r8;
    let apc_ctx = emu.regs().r9;
    let stat = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtReadFile error reading stat param");
    let buff = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("ntdll!NtReadFile error reading buff param");
    let len = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("ntdll!NtReadFile error reading len param") as usize;
    let off = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("ntdll!NtReadFile error reading off param");
    let key = emu
        .maps
        .read_qword(emu.regs().rsp + 0x40)
        .expect("ntdll!NtReadFile error reading key param");

    let file_offset = if off != 0 {
        match emu.maps.read_qword(off) {
            Some(offset_value) => offset_value,
            None => {
                log_red!(emu, "Failed to read file offset from 0x{:x}", off);
                emu.regs_mut().rax = constants::STATUS_INVALID_PARAMETER;
                return;
            }
        }
    } else {
        0
    };

    let filename = helper::handler_get_uri(file_hndl);

    log_red!(
        emu,
        "ntdll!NtReadFile {} hndl: 0x{:x} buff: 0x{:x} sz: {} off_var: 0x{:x}",
        filename,
        file_hndl,
        buff,
        len,
        off
    );

    emu.maps.memset(buff, 0x90, len);

    if filename == "\\??\\c:\\cwd" {
        let mut file = File::open(&emu.filename).unwrap();
        file.seek(SeekFrom::Start(file_offset));
        let mut file_buffer = vec![0u8; len];
        let bytes_read = file.read(&mut file_buffer).unwrap();
        for i in 0..bytes_read {
            if let Some(byte_val) = file_buffer.get(i) {
                emu.maps.write_byte(buff + i as u64, *byte_val);
            }
        }

        if bytes_read == len {
            emu.regs_mut().rax = constants::STATUS_SUCCESS;
        } else if bytes_read == 0 {
            emu.regs_mut().rax = constants::STATUS_END_OF_FILE;
        } else {
            emu.regs_mut().rax = constants::STATUS_SUCCESS;
        }
    } else {
        panic!("TODO: read {}", filename);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtClose(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    let uri = helper::handler_get_uri(hndl);

    log_red!(emu, "ntdll!NtClose hndl: 0x{:x} uri: {}", hndl, uri);

    if uri.is_empty() {
        emu.regs_mut().rax = constants::STATUS_INVALID_HANDLE;
    } else {
        emu.regs_mut().rax = constants::STATUS_SUCCESS;
    }
}
