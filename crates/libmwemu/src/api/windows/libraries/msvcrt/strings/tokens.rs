use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::{
    crt::{Runtime, record_errno},
    memory::{pointer_size, read_pointer, write_pointer},
};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::{EINVAL, ENOMEM};

pub(super) fn tokenize(emu: &mut Emu, api: &str, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let text = api_try!(emu, call.arg(emu, 0));
    let delimiters = api_try!(emu, call.arg(emu, 1));
    let context = if api == "strtok_s" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        let name = format!("{}_strtok_{}", runtime.library(), emu.current_thread().id);
        if let Some(map) = emu.maps.get_map_by_name(&name) {
            map.get_base()
        } else {
            let width = pointer_size(emu);
            let Some(cell) = emu.maps.alloc(width).filter(|&address| {
                emu.maps
                    .create_map(&name, address, width, Permission::READ_WRITE)
                    .is_ok()
            }) else {
                if record_errno(emu, runtime, ENOMEM) {
                    call.set_return(emu, 0);
                }
                return;
            };
            cell
        }
    };
    log_red!(emu, "{}!{api}", runtime.library());
    if delimiters == 0 || context == 0 {
        if record_errno(emu, runtime, EINVAL) {
            call.set_return(emu, 0);
        }
        return;
    }
    let cursor = if text != 0 {
        Some(text)
    } else {
        read_pointer(emu, context)
    };
    let Some(cursor) = cursor else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if cursor == 0 {
        if api == "strtok_s" && !record_errno(emu, runtime, EINVAL) {
            return;
        }
        call.set_return(emu, 0);
        return;
    }
    let mut run = || -> Option<u64> {
        let mut set = [false; 256];
        let mut source = delimiters;
        loop {
            let byte = emu.maps.read_byte(source)?;
            if byte == 0 {
                break;
            }
            set[usize::from(byte)] = true;
            source = source.checked_add(1)?;
        }
        let mut pos = cursor;
        let mut start = None;
        loop {
            let byte = emu.maps.read_byte(pos)?;
            if byte == 0 {
                return write_pointer(emu, context, pos).then_some(start.unwrap_or(0));
            }
            if set[usize::from(byte)] {
                if let Some(start) = start {
                    let next = pos.checked_add(1)?;
                    if !emu.maps.validate_write_range(context, pointer_size(emu))
                        || !emu.maps.write_byte(pos, 0)
                        || !write_pointer(emu, context, next)
                    {
                        return None;
                    }
                    return Some(start);
                }
            } else if start.is_none() {
                start = Some(pos);
            }
            pos = pos.checked_add(1)?;
        }
    };
    if let Some(token) = run() {
        call.set_return(emu, token);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    if !matches!(api, "strtok" | "strtok_s") {
        return false;
    }
    tokenize(emu, api, runtime);
    true
}

msvcrt_exports!(strtok, strtok_s);
