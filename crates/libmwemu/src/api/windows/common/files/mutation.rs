use super::{absolute_guest_path, io_error};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::{Emu, object_handle::file_handle::guest_operation_path};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, files::*};
use std::{fs, io, path::Path};

#[derive(Clone, Copy)]
enum Operation {
    CreateDirectory,
    RemoveDirectory,
    DeleteFile,
    Copy,
    Move,
}

fn copy(source: &Path, dest: &Path, replace: bool) -> io::Result<()> {
    let input = fs::File::open(source)?;
    let metadata = input.metadata()?;
    if !metadata.is_file() {
        return Err(io::ErrorKind::PermissionDenied.into());
    }
    if let Ok(existing) = fs::symlink_metadata(dest) {
        if !replace {
            return Err(io::ErrorKind::AlreadyExists.into());
        }
        if !existing.is_file() || existing.permissions().readonly() || source == dest {
            return Err(io::ErrorKind::PermissionDenied.into());
        }
    }
    let mut temp =
        tempfile::NamedTempFile::new_in(dest.parent().ok_or(io::ErrorKind::NotADirectory)?)?;
    io::copy(&mut &input, temp.as_file_mut())?;
    temp.as_file().set_permissions(metadata.permissions())?;
    if replace {
        temp.persist(dest)
    } else {
        temp.persist_noclobber(dest)
    }
    .map_err(|e| e.error)?;
    Ok(())
}

fn mutate(op: Operation, from: &str, to: Option<&str>, replace: bool) -> Result<(), u64> {
    let path = guest_operation_path(from, !matches!(op, Operation::Copy))
        .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
    let result = match op {
        Operation::CreateDirectory => fs::create_dir(path),
        Operation::RemoveDirectory => fs::remove_dir(path),
        Operation::DeleteFile => fs::remove_file(path),
        Operation::Copy | Operation::Move => {
            let dest = guest_operation_path(to.ok_or(ERROR_INVALID_PARAMETER)?, true)
                .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
            if matches!(op, Operation::Copy) {
                copy(&path, &dest, replace)
            } else if !replace
                && dest
                    .try_exists()
                    .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?
            {
                return Err(ERROR_ALREADY_EXISTS);
            } else {
                fs::rename(path, dest)
            }
        }
    };
    result.map_err(|e| match (op, e.kind()) {
        (Operation::CreateDirectory, io::ErrorKind::AlreadyExists) => ERROR_ALREADY_EXISTS,
        (Operation::RemoveDirectory, io::ErrorKind::NotFound) => ERROR_PATH_NOT_FOUND,
        (Operation::RemoveDirectory, io::ErrorKind::NotADirectory) => ERROR_DIRECTORY,
        _ => io_error(&e, ERROR_ACCESS_DENIED),
    })
}

fn run(emu: &mut Emu, library: &str, api: &str, op: Operation) {
    let call = WinApi(Stdcall);
    let wide = api.ends_with('W');
    let from = api_try!(emu, call.arg(emu, 0));
    let mut to = None;
    let mut replace = false;
    let slots = match op {
        Operation::CreateDirectory => 2,
        Operation::DeleteFile | Operation::RemoveDirectory => 1,
        Operation::Copy | Operation::Move => {
            let dest = api_try!(emu, call.arg(emu, 1));
            to = Some(absolute_guest_path(emu, dest, wide));
            if api.starts_with("CopyFileEx") {
                let callback = api_try!(emu, call.arg(emu, 2));
                let cancel = api_try!(emu, call.arg(emu, 4));
                let flags = api_try!(emu, call.arg_u32(emu, 5));
                if callback != 0 || cancel != 0 || flags & !COPY_FILE_FAIL_IF_EXISTS != 0 {
                    policy::unsupported(emu, library, api, emu.pc());
                    return;
                }
                replace = flags & COPY_FILE_FAIL_IF_EXISTS == 0;
                6
            } else if matches!(op, Operation::Copy) {
                replace = api_try!(emu, call.arg_u32(emu, 2)) == 0;
                3
            } else if api.starts_with("MoveFileEx") {
                let flags = api_try!(emu, call.arg_u32(emu, 2));
                if flags & !MOVEFILE_REPLACE_EXISTING != 0 {
                    policy::unsupported(emu, library, api, emu.pc());
                    return;
                }
                replace = flags & MOVEFILE_REPLACE_EXISTING != 0;
                3
            } else {
                2
            }
        }
    };
    let result = absolute_guest_path(emu, from, wide)
        .and_then(|from| mutate(op, &from, to.transpose()?.as_deref(), replace));
    log_red!(emu, "{library}!{api} => {result:?}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, slots);
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    let op = match api {
        "CreateDirectoryA" | "CreateDirectoryW" => Operation::CreateDirectory,
        "RemoveDirectoryA" | "RemoveDirectoryW" => Operation::RemoveDirectory,
        "DeleteFileA" | "DeleteFileW" => Operation::DeleteFile,
        "CopyFileA" | "CopyFileW" | "CopyFileExA" | "CopyFileExW" => Operation::Copy,
        "MoveFileA" | "MoveFileW" | "MoveFileExA" | "MoveFileExW" => Operation::Move,
        "CreateDirectoryFromAppW" if library == "kernelbase" => Operation::CreateDirectory,
        "RemoveDirectoryFromAppW" if library == "kernelbase" => Operation::RemoveDirectory,
        "DeleteFileFromAppW" if library == "kernelbase" => Operation::DeleteFile,
        "CopyFileFromAppW" if library == "kernelbase" => Operation::Copy,
        "MoveFileFromAppW" if library == "kernelbase" => Operation::Move,
        _ => return false,
    };
    run(emu, library, api, op);
    true
}
