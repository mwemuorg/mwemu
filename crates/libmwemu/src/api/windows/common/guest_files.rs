//! Read-only NT file operations provided by guest code. Only an export that redirects
//! into guest executable memory is eligible. Ordinary ntdll stubs use the usual file provider.
use std::mem::offset_of;

use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind, Register};

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_guest_bytes, read_pointer, write_pointer};
use crate::winapi::common::{file_names, handles, sections};
use crate::winapi::policy;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::memory::{PAGE_READONLY, SEC_COMMIT, SECTION_MAP_READ, VIEW_UNMAP};
use crate::windows::constants::process::{NT_CURRENT_PROCESS, OBJ_CASE_INSENSITIVE};
use crate::windows::constants::security::SYNCHRONIZE;
use crate::windows::structures::FileStandardInformation;

const READ_CHUNK_SIZE: usize = 64 * 1024;
const IO_FRAME: &str = "guest_nt_file_request";
const IO_BUFFER: &str = "guest_nt_file_data";

pub(crate) enum GuestFileRead {
    NotIntercepted,
    File(Vec<u8>),
    Failed(u64),
    Stopped,
}

#[derive(Clone, Copy, Debug)]
enum Failure {
    Status(u64),
    UnsupportedSection(u64),
    Stopped,
}

impl Failure {
    fn section_operation(self) -> Self {
        match self {
            Self::Status(
                status @ (STATUS_NOT_SUPPORTED
                | STATUS_NOT_IMPLEMENTED
                | STATUS_INVALID_DEVICE_REQUEST),
            ) => Self::UnsupportedSection(status),
            other => other,
        }
    }
}

pub(crate) fn win32_error(status: u64) -> u64 {
    match status {
        STATUS_OBJECT_NAME_NOT_FOUND | STATUS_NO_SUCH_FILE => ERROR_FILE_NOT_FOUND,
        STATUS_OBJECT_PATH_NOT_FOUND => ERROR_PATH_NOT_FOUND,
        STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
        STATUS_INVALID_HANDLE => ERROR_INVALID_HANDLE,
        STATUS_INVALID_PARAMETER => ERROR_INVALID_PARAMETER,
        STATUS_NO_MEMORY => ERROR_NOT_ENOUGH_MEMORY,
        STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
        STATUS_INVALID_IMAGE_FORMAT => ERROR_BAD_EXE_FORMAT,
        STATUS_NOT_SUPPORTED | STATUS_NOT_IMPLEMENTED | STATUS_INVALID_DEVICE_REQUEST => {
            ERROR_NOT_SUPPORTED
        }
        STATUS_NAME_TOO_LONG => ERROR_FILENAME_EXCED_RANGE,
        _ => ERROR_GEN_FAILURE,
    }
}

#[repr(C)]
struct ObjectAttributes<P> {
    length: u32,
    root: P,
    name: P,
    attributes: u32,
    descriptor: P,
    qos: P,
}

#[repr(C)]
struct UnicodeString<P> {
    length: u16,
    maximum: u16,
    buffer: P,
}

#[repr(C)]
struct Request<P> {
    object: ObjectAttributes<P>,
    name: UnicodeString<P>,
    io: [P; 2],
    file: P,
    standard: FileStandardInformation,
    offset: i64,
    section: P,
    view: P,
    size: P,
}

struct Frame {
    base: u64,
    io: u64,
    file: u64,
    standard: u64,
    offset: u64,
    section: u64,
    view: u64,
    size: u64,
}

impl Frame {
    fn allocate<P>(emu: &mut Emu, path: &[u16]) -> Result<Self, Failure> {
        let length = path
            .len()
            .checked_mul(size_of::<u16>())
            .and_then(|n| u16::try_from(n).ok())
            .ok_or(Failure::Status(STATUS_NAME_TOO_LONG))?;
        let maximum = length
            .checked_add(2)
            .ok_or(Failure::Status(STATUS_NAME_TOO_LONG))?;
        let name_offset = size_of::<Request<P>>();
        let base = emu
            .maps
            .alloc_rw_pages(IO_FRAME, (name_offset + usize::from(maximum)) as u64)
            .ok_or(Failure::Status(STATUS_NO_MEMORY))?
            .get_base();
        let at = |offset: usize| base + offset as u64;
        let descriptor = at(offset_of!(Request<P>, name));
        let buffer = at(name_offset);
        let bytes: Vec<_> = path.iter().flat_map(|v| v.to_le_bytes()).collect();
        let ok = emu
            .maps
            .write_dword(base, size_of::<ObjectAttributes<P>>() as u32)
            && write_pointer(emu, at(offset_of!(ObjectAttributes<P>, name)), descriptor)
            && emu.maps.write_dword(
                at(offset_of!(ObjectAttributes<P>, attributes)),
                OBJ_CASE_INSENSITIVE,
            )
            && emu.maps.write_word(descriptor, length)
            && emu.maps.write_word(descriptor + 2, maximum)
            && write_pointer(
                emu,
                descriptor + offset_of!(UnicodeString<P>, buffer) as u64,
                buffer,
            )
            && emu.maps.write_bytes(buffer, &bytes);
        if !ok {
            emu.maps.dealloc(base);
            return Err(Failure::Status(STATUS_ACCESS_VIOLATION));
        }
        Ok(Self {
            base,
            io: at(offset_of!(Request<P>, io)),
            file: at(offset_of!(Request<P>, file)),
            standard: at(offset_of!(Request<P>, standard)),
            offset: at(offset_of!(Request<P>, offset)),
            section: at(offset_of!(Request<P>, section)),
            view: at(offset_of!(Request<P>, view)),
            size: at(offset_of!(Request<P>, size)),
        })
    }
}

fn nt_path(emu: &Emu, path: &str) -> Result<Vec<u16>, Failure> {
    if path.starts_with(r"\??\") || path.starts_with(r"\Device\") {
        return Ok(path.encode_utf16().collect());
    }
    file_names::native_path(&emu.cfg.cwd_path, path.encode_utf16().collect())
        .map(|(name, _)| name)
        .map_err(|error| {
            Failure::Status(if error == ERROR_PATH_NOT_FOUND {
                STATUS_OBJECT_PATH_NOT_FOUND
            } else {
                STATUS_OBJECT_PATH_SYNTAX_BAD
            })
        })
}

fn redirect(emu: &Emu, api: &str) -> Option<u64> {
    let entry = emu.export_indexes.resolve_name_in_module("ntdll.dll", api);
    let source = emu.maps.get_mem_by_addr(entry)?;
    let code = source.read_code(entry, 15);
    let bits = if emu.cfg.is_x64() { 64 } else { 32 };
    let instruction = Decoder::with_ip(bits, code.as_ref(), entry, DecoderOptions::NONE).decode();
    if instruction.mnemonic() != Mnemonic::Jmp {
        return None;
    }
    let target = match instruction.op0_kind() {
        OpKind::NearBranch32 | OpKind::NearBranch64 => instruction.near_branch_target(),
        OpKind::Memory if instruction.is_ip_rel_memory_operand() => {
            read_pointer(emu, instruction.ip_rel_memory_address())?
        }
        OpKind::Memory
            if instruction.memory_base() == Register::None
                && instruction.memory_index() == Register::None =>
        {
            read_pointer(emu, instruction.memory_displacement64())?
        }
        _ => return None,
    };
    let system_target = match emu.maps.find_pe_image_info(target) {
        Some((base, _)) => base != emu.base && !emu.guest_dlls.contains_key(&base),
        None => emu.export_indexes.resolve_address(target).is_some(),
    };
    if source.inside(target)
        || system_target
        || !emu.maps.can_access_range(target, 1, Permission::EXECUTE)
    {
        return None;
    }
    // Enter the actual export so breakpoints and instruction hooks observe its redirect.
    Some(entry)
}

fn invoke<const N: usize>(
    emu: &mut Emu,
    api: &str,
    callback: u64,
    args: [u64; N],
) -> Result<(), Failure> {
    let regs = *emu.regs();
    let flags = *emu.flags_ref();
    let result =
        policy::invoke_callback(emu, "ntdll", api, callback, args).ok_or(Failure::Stopped)?;
    *emu.regs_mut() = regs;
    *emu.flags_overwrite_mut() = flags;
    let status = u64::from(result as u32);
    if status == STATUS_PENDING {
        policy::callback_failed(emu, "ntdll", api);
        return Err(Failure::Stopped);
    }
    if (status as i32) < 0 {
        Err(Failure::Status(status))
    } else {
        Ok(())
    }
}

fn open(
    emu: &mut Emu,
    frame: &Frame,
    open: Option<u64>,
    create: Option<u64>,
    share: u32,
) -> Result<u64, Failure> {
    let access = u64::from(SYNCHRONIZE | FILE_READ_DATA | FILE_READ_ATTRIBUTES);
    let share = u64::from(share);
    let options = u64::from(FILE_SYNCHRONOUS_IO_NONALERT | FILE_NON_DIRECTORY_FILE);
    if let Some(target) = open {
        invoke(
            emu,
            "NtOpenFile",
            target,
            [frame.file, access, frame.base, frame.io, share, options],
        )?;
    } else if let Some(target) = create {
        invoke(
            emu,
            "NtCreateFile",
            target,
            [
                frame.file,
                access,
                frame.base,
                frame.io,
                0,
                0,
                share,
                u64::from(FILE_OPEN),
                options,
                0,
                0,
            ],
        )?;
    } else {
        return Err(Failure::Status(STATUS_NOT_SUPPORTED));
    }
    read_pointer(emu, frame.file)
        .filter(|handle| *handle != 0)
        .ok_or(Failure::Status(STATUS_INVALID_HANDLE))
}

fn close(emu: &mut Emu, handle: u64) -> Result<(), Failure> {
    if let Some(target) = redirect(emu, "NtClose") {
        invoke(emu, "NtClose", target, [handle])
    } else {
        handles::close_object(emu, handle)
            .then_some(())
            .ok_or(Failure::Status(STATUS_INVALID_HANDLE))
    }
}

fn file_size(emu: &mut Emu, frame: &Frame, handle: u64, target: u64) -> Result<usize, Failure> {
    let eof = frame.standard + offset_of!(FileStandardInformation, end_of_file) as u64;
    if !emu.maps.write_qword(eof, u64::MAX) {
        return Err(Failure::Status(STATUS_ACCESS_VIOLATION));
    }
    invoke(
        emu,
        "NtQueryInformationFile",
        target,
        [
            handle,
            frame.io,
            frame.standard,
            size_of::<FileStandardInformation>() as u64,
            u64::from(FILE_STANDARD_INFORMATION),
        ],
    )?;
    let length = emu
        .maps
        .read_qword(eof)
        .ok_or(Failure::Status(STATUS_ACCESS_VIOLATION))?;
    if length > i64::MAX as u64 {
        return Err(Failure::Status(STATUS_FILE_INVALID));
    }
    if length > emu.maps.max_alloc_size {
        return Err(Failure::Status(STATUS_NO_MEMORY));
    }
    usize::try_from(length).map_err(|_| Failure::Status(STATUS_NO_MEMORY))
}

fn read_bytes(
    emu: &mut Emu,
    frame: &Frame,
    handle: u64,
    target: u64,
    length: usize,
) -> Result<Vec<u8>, Failure> {
    if length == 0 {
        return Ok(Vec::new());
    }
    let chunk = length.min(READ_CHUNK_SIZE);
    let buffer = emu
        .maps
        .alloc_rw_pages(IO_BUFFER, chunk as u64)
        .ok_or(Failure::Status(STATUS_NO_MEMORY))?
        .get_base();
    let result = (|| {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| Failure::Status(STATUS_NO_MEMORY))?;
        let information = frame.io + pointer_size(emu);
        while bytes.len() < length {
            let wanted = (length - bytes.len()).min(chunk);
            if !write_pointer(emu, information, 0)
                || !emu.maps.write_qword(frame.offset, bytes.len() as u64)
            {
                return Err(Failure::Status(STATUS_ACCESS_VIOLATION));
            }
            invoke(
                emu,
                "NtReadFile",
                target,
                [
                    handle,
                    0,
                    0,
                    0,
                    frame.io,
                    buffer,
                    wanted as u64,
                    frame.offset,
                    0,
                ],
            )?;
            let count = read_pointer(emu, information)
                .filter(|n| *n != 0 && *n <= wanted as u64)
                .ok_or(Failure::Status(STATUS_FILE_INVALID))?;
            let data = read_guest_bytes(emu, buffer, count as usize)
                .ok_or(Failure::Status(STATUS_ACCESS_VIOLATION))?;
            bytes.extend_from_slice(&data);
        }
        Ok(bytes)
    })();
    if !matches!(result, Err(Failure::Stopped)) {
        emu.maps.dealloc(buffer);
    }
    result
}

fn with_cleanup<T>(
    result: Result<T, Failure>,
    cleanup: impl FnOnce() -> Result<(), Failure>,
) -> Result<T, Failure> {
    if matches!(result, Err(Failure::Stopped)) {
        return result;
    }
    match cleanup() {
        Err(Failure::Stopped) => Err(Failure::Stopped),
        Err(error) if result.is_ok() || matches!(result, Err(Failure::UnsupportedSection(_))) => {
            Err(error)
        }
        _ => result,
    }
}

fn read_section(
    emu: &mut Emu,
    frame: &Frame,
    file: u64,
    create: u64,
    map: u64,
    length: usize,
) -> Result<Vec<u8>, Failure> {
    invoke(
        emu,
        "NtCreateSection",
        create,
        [
            frame.section,
            SECTION_MAP_READ as u64,
            0,
            0,
            u64::from(PAGE_READONLY),
            u64::from(SEC_COMMIT),
            file,
        ],
    )
    .map_err(Failure::section_operation)?;
    let section = read_pointer(emu, frame.section)
        .filter(|value| *value != 0)
        .ok_or(Failure::Status(STATUS_INVALID_HANDLE))?;
    let result = (|| {
        invoke(
            emu,
            "NtMapViewOfSection",
            map,
            [
                section,
                NT_CURRENT_PROCESS,
                frame.view,
                0,
                0,
                0,
                frame.size,
                VIEW_UNMAP as u64,
                0,
                u64::from(PAGE_READONLY),
            ],
        )
        .map_err(Failure::section_operation)?;
        let view = read_pointer(emu, frame.view)
            .filter(|v| *v != 0)
            .ok_or(Failure::Status(STATUS_FILE_INVALID))?;
        let bytes = if read_pointer(emu, frame.size).is_some_and(|span| span >= length as u64) {
            read_guest_bytes(emu, view, length).ok_or(Failure::Status(STATUS_ACCESS_VIOLATION))
        } else {
            Err(Failure::Status(STATUS_FILE_INVALID))
        };
        with_cleanup(bytes, || {
            if let Some(target) = redirect(emu, "NtUnmapViewOfSection") {
                invoke(
                    emu,
                    "NtUnmapViewOfSection",
                    target,
                    [NT_CURRENT_PROCESS, view],
                )
            } else {
                sections::unmap(emu, view).map_err(Failure::Status)
            }
        })
    })();
    with_cleanup(result, || close(emu, section))
}

fn read(emu: &mut Emu, path: &str, share: u32, prefer_section: bool) -> GuestFileRead {
    if emu.cfg.arch.is_aarch64() || emu.maps.get_map_by_name(IO_FRAME).is_some() {
        return GuestFileRead::NotIntercepted;
    }
    let open_file = redirect(emu, "NtOpenFile");
    let create_file = redirect(emu, "NtCreateFile");
    if open_file.is_none() && create_file.is_none() {
        return GuestFileRead::NotIntercepted;
    }
    let Some(query) = redirect(emu, "NtQueryInformationFile") else {
        return GuestFileRead::NotIntercepted;
    };
    let read_file = redirect(emu, "NtReadFile");
    let section = redirect(emu, "NtCreateSection").zip(redirect(emu, "NtMapViewOfSection"));
    if read_file.is_none() && section.is_none() {
        return GuestFileRead::NotIntercepted;
    }

    let result = (|| {
        let path = nt_path(emu, path)?;
        let frame = if emu.cfg.is_x64() {
            Frame::allocate::<u64>(emu, &path)?
        } else {
            Frame::allocate::<u32>(emu, &path)?
        };
        let bytes = (|| {
            let handle = open(emu, &frame, open_file, create_file, share)?;
            let result = (|| {
                let length = file_size(emu, &frame, handle, query)?;
                if length == 0 {
                    return Ok(Vec::new());
                }
                if (prefer_section || read_file.is_none())
                    && let Some((create, map)) = section
                {
                    match read_section(emu, &frame, handle, create, map, length) {
                        Err(Failure::UnsupportedSection(_)) if read_file.is_some() => {}
                        result => return result,
                    }
                }
                let target = read_file.ok_or(Failure::Status(STATUS_NOT_SUPPORTED))?;
                read_bytes(emu, &frame, handle, target, length)
            })();
            with_cleanup(result, || close(emu, handle))
        })();
        if !matches!(bytes, Err(Failure::Stopped)) {
            emu.maps.dealloc(frame.base);
        }
        bytes
    })();
    match result {
        Ok(bytes) => GuestFileRead::File(bytes),
        Err(Failure::Status(status) | Failure::UnsupportedSection(status)) => {
            GuestFileRead::Failed(status)
        }
        Err(Failure::Stopped) => GuestFileRead::Stopped,
    }
}

pub(crate) fn read_file(emu: &mut Emu, path: &str, share: u32) -> GuestFileRead {
    read(emu, path, share, false)
}

pub(crate) fn read_image_file(emu: &mut Emu, path: &str, share: u32) -> GuestFileRead {
    read(emu, path, share, true)
}
