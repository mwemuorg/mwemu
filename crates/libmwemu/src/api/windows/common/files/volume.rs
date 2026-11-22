use super::{
    BYTES_PER_SECTOR, FREE_CLUSTERS, SECTORS_PER_CLUSTER, TOTAL_CLUSTERS, absolute_guest_path,
    file_names, guest_file_metadata,
};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

const ROOT: &str = "C:\\";
const DRIVE_FIXED: u64 = 3;
const DRIVE_NO_ROOT_DIR: u64 = 1;
const DRIVE_C_BIT: u64 = 1 << (b'C' - b'A');
const VOLUME_SERIAL: u32 = 0x1234_5678;
const MAX_COMPONENT_LENGTH: u32 = 255;
const FILE_SYSTEM_FLAGS: u32 = 0x0007_00ff;

fn root_or_current(emu: &Emu, address: u64, wide: bool) -> Result<String, u64> {
    if address == 0 {
        Ok(ROOT.to_owned())
    } else {
        file_names::read_path(emu, address, wide)
    }
}

fn volume_type(root: &str) -> u64 {
    if root.replace('/', "\\").eq_ignore_ascii_case(ROOT) {
        DRIVE_FIXED
    } else {
        DRIVE_NO_ROOT_DIR
    }
}

pub(crate) fn drive_number_type(index: i32) -> u64 {
    let Some(letter) = u8::try_from(index)
        .ok()
        .filter(|index| *index <= b'Z' - b'A')
    else {
        return DRIVE_NO_ROOT_DIR;
    };
    volume_type(&format!("{}:\\", (b'A' + letter) as char))
}

fn drive_type(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let root = root_or_current(emu, address, api.ends_with('W'));
    let value = root
        .as_deref()
        .map(volume_type)
        .unwrap_or(DRIVE_NO_ROOT_DIR);
    log_red!(emu, "{api} => {value}");
    call.set_return(emu, value);
    call.cleanup_args(emu, 1);
}

fn logical_strings(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let capacity = api_try!(emu, call.arg_u32(emu, 0)) as usize;
    let output = api_try!(emu, call.arg(emu, 1));
    let codec = file_names::encoding(emu, api.ends_with('W'));
    let bytes = codec.terminated(&"C:\\\0".encode_utf16().collect::<Vec<_>>());
    let needed = bytes.len() / codec.width();
    let value = if capacity < needed {
        needed
    } else if output != 0
        && emu.maps.validate_write_range(output, bytes.len() as u64)
        && emu.maps.write_bytes(output, &bytes)
    {
        needed - 1
    } else {
        emu.set_last_error(ERROR_NOACCESS);
        0
    };
    log_red!(emu, "{api} capacity={capacity} => {value}");
    call.set_return(emu, value as u64);
    call.cleanup_args(emu, 2);
}

pub(super) fn check_disk_directory(emu: &Emu, address: u64, wide: bool) -> Result<(), u64> {
    if address != 0 {
        let path = absolute_guest_path(emu, address, wide)?;
        if !path.get(..3).is_some_and(|s| s.eq_ignore_ascii_case(ROOT)) {
            return Err(ERROR_PATH_NOT_FOUND);
        }
        if !path.eq_ignore_ascii_case(ROOT)
            && !guest_file_metadata(&path).is_ok_and(|meta| meta.is_dir())
        {
            return Err(ERROR_PATH_NOT_FOUND);
        }
    }
    Ok(())
}

fn free_space(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let cluster = u64::from(BYTES_PER_SECTOR) * u64::from(SECTORS_PER_CLUSTER);
    let free = u64::from(FREE_CLUSTERS) * cluster;
    let total = u64::from(TOTAL_CLUSTERS) * cluster;
    let outputs = [
        (api_try!(emu, call.arg(emu, 1)), free),
        (api_try!(emu, call.arg(emu, 2)), total),
        (api_try!(emu, call.arg(emu, 3)), free),
    ];
    let result = (|| {
        check_disk_directory(emu, address, api.ends_with('W'))?;
        if outputs
            .iter()
            .any(|(p, _)| *p != 0 && !emu.maps.validate_write_range(*p, size_of::<u64>() as u64))
        {
            return Err(ERROR_NOACCESS);
        }
        for (p, value) in outputs {
            if p != 0 {
                emu.maps.write_qword(p, value);
            }
        }
        Ok::<_, u64>(())
    })();
    log_red!(emu, "{api} => {result:?}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4);
}

fn path_name(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let wide = api.ends_with('W');
    let codec = file_names::encoding(emu, wide);
    let result = absolute_guest_path(emu, address, wide).and_then(|path| {
        // A drive or UNC share is the root in the guest's single-mount model.
        let path = path.strip_prefix(r"\\?\").unwrap_or(&path);
        let root = if let Some(unc) = path.strip_prefix(r"\\") {
            let mut parts = unc.split('\\');
            format!(
                r"\\{}\{}\",
                parts.next().ok_or(ERROR_PATH_NOT_FOUND)?,
                parts.next().ok_or(ERROR_PATH_NOT_FOUND)?
            )
        } else {
            let (drive, _) = path.split_once(':').ok_or(ERROR_PATH_NOT_FOUND)?;
            format!(r"{}:\", drive.to_uppercase())
        };
        let bytes = codec.terminated(&root.encode_utf16().collect::<Vec<_>>());
        if capacity < bytes.len() / codec.width() {
            return Err(ERROR_FILENAME_EXCED_RANGE);
        }
        if out == 0
            || !emu.maps.validate_write_range(out, bytes.len() as u64)
            || !emu.maps.write_bytes(out, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    });
    log_red!(emu, "{api} => {result:?}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 3);
}

fn information(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let root = api_try!(emu, call.arg(emu, 0));
    let label = api_try!(emu, call.arg(emu, 1));
    let label_len = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let outputs = [
        (api_try!(emu, call.arg(emu, 3)), VOLUME_SERIAL),
        (api_try!(emu, call.arg(emu, 4)), MAX_COMPONENT_LENGTH),
        (api_try!(emu, call.arg(emu, 5)), FILE_SYSTEM_FLAGS),
    ];
    let fs = api_try!(emu, call.arg(emu, 6));
    let fs_len = api_try!(emu, call.arg_u32(emu, 7)) as usize;
    let wide = api.ends_with('W');
    let codec = file_names::encoding(emu, wide);
    let texts = [(label, label_len, "Windows"), (fs, fs_len, "NTFS")].map(|(p, n, s)| {
        (
            p,
            n,
            codec.terminated(&s.encode_utf16().collect::<Vec<_>>()),
        )
    });
    let result = (|| {
        if api.ends_with("GetVolumeInformationByHandleW") {
            super::file(emu, root)?;
        } else if !root_or_current(emu, root, wide)?.eq_ignore_ascii_case(ROOT) {
            return Err(ERROR_PATH_NOT_FOUND);
        }
        if texts
            .iter()
            .any(|(p, n, b)| *p != 0 && *n < b.len() / codec.width())
        {
            return Err(ERROR_BAD_LENGTH);
        }
        if texts
            .iter()
            .any(|(p, _, b)| *p != 0 && !emu.maps.validate_write_range(*p, b.len() as u64))
            || outputs.iter().any(|(p, _)| {
                *p != 0 && !emu.maps.validate_write_range(*p, size_of::<u32>() as u64)
            })
        {
            return Err(ERROR_NOACCESS);
        }
        for (p, _, bytes) in texts {
            if p != 0 {
                emu.maps.write_bytes(p, &bytes);
            }
        }
        for (p, value) in outputs {
            if p != 0 {
                emu.maps.write_dword(p, value);
            }
        }
        Ok(())
    })();
    log_red!(emu, "{api} => {result:?}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 8);
}

fn dos_device(emu: &mut Emu, api: &str) {
    use crate::windows::constants::files::SYSTEM_VOLUME_DEVICE;
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let wide = api.ends_with('W');
    let codec = file_names::encoding(emu, wide);
    let result = (|| {
        let name = if input == 0 {
            "C:"
        } else if file_names::read_path(emu, input, wide)?.eq_ignore_ascii_case("C:") {
            SYSTEM_VOLUME_DEVICE
        } else {
            return Err(ERROR_FILE_NOT_FOUND);
        };
        let bytes = codec.terminated(&format!("{name}\0").encode_utf16().collect::<Vec<_>>());
        let count = bytes.len() / codec.width();
        if capacity < count || out == 0 {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if !emu.maps.validate_write_range(out, bytes.len() as u64)
            || !emu.maps.write_bytes(out, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(count as u64)
    })();
    log_red!(emu, "{api} => {result:?}");
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, 3);
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    let handler: fn(&mut Emu, &str) = match api {
        "GetLogicalDrives" => |emu, name| {
            let call = WinApi(Stdcall);
            log_red!(emu, "{name}");
            call.set_return(emu, DRIVE_C_BIT);
        },
        "QueryDosDeviceA" | "QueryDosDeviceW" => dos_device,
        "GetLogicalDriveStringsA" | "GetLogicalDriveStringsW" => logical_strings,
        "GetDriveTypeA" | "GetDriveTypeW" => drive_type,
        "GetDiskFreeSpaceExA" | "GetDiskFreeSpaceExW" => free_space,
        "GetVolumePathNameA" | "GetVolumePathNameW" => path_name,
        "GetVolumeInformationA" | "GetVolumeInformationW" | "GetVolumeInformationByHandleW" => {
            information
        }
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}
