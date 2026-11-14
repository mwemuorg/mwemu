use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::system::*;

// The emulated machine has one fixed NTFS volume, C:.

const VOLUME_LABEL: &str = "Windows";
const VOLUME_SERIAL: u32 = 0x1234_5678;
const FILE_SYSTEM: &str = "NTFS";
const MAX_COMPONENT_LENGTH: u32 = 255;
// case sensitive/preserved names, unicode, ACLs, compression, quotas,
// sparse files, reparse points, object ids, encryption, named streams
const FILE_SYSTEM_FLAGS: u32 = 0x0007_00ff;

const ERROR_PATH_NOT_FOUND: u64 = 3;
const ERROR_BAD_LENGTH: u64 = 24;
const ERROR_FILENAME_EXCED_RANGE: u64 = 206;

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> String {
    strings::read_text(emu, ptr, wide)
}

/// Write `s` to a buffer of `size` characters; false when it does not fit.
fn write_text(emu: &mut emu::Emu, ptr: u64, size: usize, s: &str, wide: bool) -> bool {
    let len = if wide {
        s.encode_utf16().count()
    } else {
        s.len()
    };
    if len + 1 > size {
        return false;
    }
    strings::write_string(emu, ptr, s, wide);
    true
}

/// Root of the volume holding `path`: `X:\` or `\\server\share\`; relative
/// paths are on the current drive.
fn volume_root(path: &str) -> String {
    let path = path.replace('/', "\\");
    let path = path.strip_prefix("\\\\?\\").unwrap_or(&path);
    if let Some(unc) = path.strip_prefix("\\\\") {
        let mut parts = unc.split('\\').filter(|p| !p.is_empty());
        if let (Some(server), Some(share)) = (parts.next(), parts.next()) {
            return format!("\\\\{}\\{}\\", server, share);
        }
    }
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        return format!("{}:\\", (bytes[0] as char).to_ascii_uppercase());
    }
    "C:\\".to_string()
}

/*
BOOL GetVolumePathNameW(
  [in]  LPCWSTR lpszFileName,
  [out] LPWSTR  lpszVolumePathName,
  [in]  DWORD   cchBufferLength
);
*/
pub fn GetVolumePathNameA(emu: &mut emu::Emu) {
    volume_path_name(emu, "GetVolumePathNameA", false);
}

pub fn GetVolumePathNameW(emu: &mut emu::Emu) {
    volume_path_name(emu, "GetVolumePathNameW", true);
}

fn volume_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let file = read_text(emu, emu.regs().rcx, wide);
    let out = emu.regs().rdx;
    let size = emu.regs().r8 as u32 as usize;
    let root = volume_root(&file);
    log_red!(emu, "kernel32!{} `{}` = `{}`", api, file, root);
    if out == 0 || !write_text(emu, out, size, &root, wide) {
        emu.set_last_error(ERROR_FILENAME_EXCED_RANGE);
        emu.regs_mut().rax = FALSE;
        return;
    }
    emu.regs_mut().rax = TRUE;
}

/*
BOOL GetVolumeInformationW(
  [in, optional]  LPCWSTR lpRootPathName,
  [out, optional] LPWSTR  lpVolumeNameBuffer,
  [in]            DWORD   nVolumeNameSize,
  [out, optional] LPDWORD lpVolumeSerialNumber,
  [out, optional] LPDWORD lpMaximumComponentLength,
  [out, optional] LPDWORD lpFileSystemFlags,
  [out, optional] LPWSTR  lpFileSystemNameBuffer,
  [in]            DWORD   nFileSystemNameSize
);
*/
pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    volume_information(emu, "GetVolumeInformationA", false);
}

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    volume_information(emu, "GetVolumeInformationW", true);
}

fn volume_information(emu: &mut emu::Emu, api: &str, wide: bool) {
    let root_ptr = emu.regs().rcx;
    let label_buf = emu.regs().rdx;
    let label_size = emu.regs().r8 as u32 as usize;
    let serial_ptr = emu.regs().r9;
    let max_component_ptr = WinApiAbi::arg(emu, 4);
    let flags_ptr = WinApiAbi::arg(emu, 5);
    let fs_buf = WinApiAbi::arg(emu, 6);
    let fs_size = WinApiAbi::arg(emu, 7) as u32 as usize;

    let root = if root_ptr == 0 {
        "C:\\".to_string()
    } else {
        read_text(emu, root_ptr, wide)
    };
    log_red!(emu, "kernel32!{} `{}`", api, root);

    // the root must name a volume, with its trailing backslash
    if !root.eq_ignore_ascii_case("C:\\") {
        emu.set_last_error(ERROR_PATH_NOT_FOUND);
        emu.regs_mut().rax = FALSE;
        return;
    }
    if (label_buf != 0 && !write_text(emu, label_buf, label_size, VOLUME_LABEL, wide))
        || (fs_buf != 0 && !write_text(emu, fs_buf, fs_size, FILE_SYSTEM, wide))
    {
        emu.set_last_error(ERROR_BAD_LENGTH);
        emu.regs_mut().rax = FALSE;
        return;
    }
    for (ptr, value) in [
        (serial_ptr, VOLUME_SERIAL),
        (max_component_ptr, MAX_COMPONENT_LENGTH),
        (flags_ptr, FILE_SYSTEM_FLAGS),
    ] {
        if ptr != 0 {
            emu.maps.write_dword(ptr, value);
        }
    }
    emu.regs_mut().rax = TRUE;
}
