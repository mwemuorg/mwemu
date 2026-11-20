use super::{absolute_guest_path, file, io_error, seek, transfer};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::{Emu, object_handle::FileHandle};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, files::*, security::*};
use std::io;

const HFILE_ERROR: u64 = u32::MAX as u64;
const OF_WRITE: u32 = 1;
const OF_READWRITE: u32 = 2;
const OF_ACCESS_MASK: u32 = 3;
const OF_SHARE_MASK: u32 = 0x70;
const OF_SHARE_EXCLUSIVE: u32 = 0x10;
const OF_SHARE_DENY_WRITE: u32 = 0x20;
const OF_SHARE_DENY_READ: u32 = 0x30;
const OF_SHARE_DENY_NONE: u32 = 0x40;

fn open(emu: &mut Emu, library: &str, api: &str, create: bool) {
    let call = WinApi(Stdcall);
    let pointer = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg_u32(emu, 1));
    if create && mode != 0 {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let result = (|| {
        let access = if create {
            GENERIC_READ | GENERIC_WRITE
        } else {
            match mode & OF_ACCESS_MASK {
                0 => GENERIC_READ,
                OF_WRITE => GENERIC_WRITE,
                OF_READWRITE => GENERIC_READ | GENERIC_WRITE,
                _ => return Err(ERROR_INVALID_PARAMETER),
            }
        };
        let share = match mode & OF_SHARE_MASK {
            OF_SHARE_EXCLUSIVE => 0,
            OF_SHARE_DENY_WRITE => FILE_SHARE_READ,
            OF_SHARE_DENY_READ => FILE_SHARE_WRITE,
            0 | OF_SHARE_DENY_NONE => FILE_SHARE_READ | FILE_SHARE_WRITE,
            _ => return Err(ERROR_INVALID_PARAMETER),
        };
        let name = absolute_guest_path(emu, pointer, false)?;
        let disposition = if create { CREATE_ALWAYS } else { OPEN_EXISTING };
        let file = FileHandle::new(
            name,
            access,
            disposition as u32,
            FILE_ATTRIBUTE_NORMAL as u32,
            share,
        )
        .map_err(|e| {
            e.downcast_ref::<io::Error>()
                .map_or(ERROR_INVALID_PARAMETER, |e| {
                    io_error(e, ERROR_ACCESS_DENIED)
                })
        })?;
        Ok(u64::from(emu.handle_management.insert_file_handle(file)))
    })();
    log_red!(emu, "{library}!{api} => {result:?}");
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        HFILE_ERROR
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, 2);
}

fn operation(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = u64::from(api_try!(emu, call.arg_u32(emu, 0)));
    let result = match api {
        "_lclose" => {
            if file(emu, handle).is_ok() && emu.handle_management.close_handle(handle) {
                Ok(0)
            } else {
                Err(ERROR_INVALID_HANDLE)
            }
        }
        "_llseek" => {
            let distance = api_try!(emu, call.arg_i32(emu, 1));
            let method = api_try!(emu, call.arg_u32(emu, 2));
            seek(emu, handle, i64::from(distance), method, true)
        }
        _ => {
            let buffer = api_try!(emu, call.arg(emu, 1));
            let count = api_try!(emu, call.arg_u32(emu, 2));
            let write = api.ends_with("write");
            if write && count == 0 {
                file(emu, handle)
                    .and_then(|f| {
                        f.set_end_of_file()
                            .map_err(|e| io_error(&e, ERROR_WRITE_FAULT))
                    })
                    .map(|()| 0)
            } else if api.starts_with("_h") && count > i32::MAX as u32 {
                Err(ERROR_INVALID_PARAMETER)
            } else {
                transfer(emu, handle, buffer, count as usize, write).map(|n| n as u64)
            }
        }
    };
    log_red!(emu, "{library}!{api} handle={handle:#x} => {result:?}");
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        HFILE_ERROR
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, if api == "_lclose" { 1 } else { 3 });
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    if library != "kernel32" {
        return false;
    }
    match api {
        "_lcreat" => open(emu, library, api, true),
        "_lopen" => open(emu, library, api, false),
        "_lclose" | "_llseek" | "_lread" | "_lwrite" | "_hread" | "_hwrite" => {
            operation(emu, library, api)
        }
        _ => return false,
    }
    true
}
