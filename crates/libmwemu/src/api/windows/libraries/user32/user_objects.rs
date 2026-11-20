use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{memory::read_array, resources::numeric_payload, strings};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;

const WINDOW_STATION: u64 = 0x0001_0002;
const UOI_FLAGS: u32 = 1;
const UOI_NAME: u32 = 2;
const UOI_TYPE: u32 = 3;
const WSF_VISIBLE: u32 = 1;
const IMAGE_BITMAP: u32 = 0;
const IMAGE_ICON: u32 = 1;
const IMAGE_CURSOR: u32 = 2;
const LR_SHARED: u32 = 0x8000;
const LR_DEFAULTSIZE: u32 = 0x0040;
const RT_GROUP_CURSOR: u32 = 12;
const RT_GROUP_ICON: u32 = 14;
const RT_ICON: u32 = 3;
const RT_CURSOR: u32 = 1;
const ERROR_INVALID_MENU_HANDLE: u64 = 1401;

fn allocate(emu: &mut Emu, kind: String, shared: bool) -> u64 {
    let state = &mut emu.guest_windows.user;
    if shared
        && let Some((&handle, _)) = state
            .objects
            .iter()
            .find(|(_, stored)| stored.1 && stored.0 == kind)
    {
        return handle;
    }
    if let Some(handle) = state.allocate() {
        state.objects.insert(handle, (kind, shared));
        handle
    } else {
        emu.set_last_error(ERROR_NOT_ENOUGH_MEMORY);
        0
    }
}

fn system_object(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [object, index, output, capacity, needed] = api_try!(emu, call.args(emu));
    let index = index as u32;
    let capacity = capacity as u32;
    log_red!(
        emu,
        "user32!{api} user_object={object:#x} index={index} output={output:#x} capacity={capacity} needed={needed:#x}"
    );
    let kind = if object == WINDOW_STATION {
        Some(("WindowStation", "WinSta0"))
    } else {
        emu.guest_windows
            .user
            .objects
            .get(&object)
            .filter(|(name, _)| name == "desktop")
            .map(|_| ("Desktop", "Default"))
    };
    let result = (|| {
        let (kind, name) = kind.ok_or(ERROR_INVALID_HANDLE)?;
        let data = match index {
            UOI_FLAGS => [
                0u32,
                0,
                if object == WINDOW_STATION {
                    WSF_VISIBLE
                } else {
                    0
                },
            ]
            .map(u32::to_le_bytes)
            .as_flattened()
            .to_vec(),
            UOI_NAME | UOI_TYPE => strings::encode_text(
                &format!("{}\0", if index == UOI_TYPE { kind } else { name }),
                wide,
            ),
            _ => return Err(ERROR_INVALID_PARAMETER),
        };
        if needed != 0 && !emu.maps.write_dword(needed, data.len() as u32) {
            return Err(ERROR_NOACCESS);
        }
        if output == 0 || (capacity as usize) < data.len() {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if !emu.maps.validate_write_range(output, data.len() as u64)
            || !emu.maps.write_bytes(output, &data)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, result.is_ok().into());
    call.cleanup_args(emu, 5);
}

fn close_object(emu: &mut Emu, api: &str, prefix: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let entry = emu.guest_windows.user.objects.get(&handle);
    let valid = entry.is_some_and(|(k, _)| {
        k == prefix || k.strip_prefix(prefix).is_some_and(|s| s.starts_with(':'))
    });
    let owned = entry.is_some_and(|(_, shared)| !*shared);
    if valid && owned {
        emu.guest_windows.user.objects.remove(&handle);
    } else if !valid {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    log_red!(
        emu,
        "user32!{api} handle={handle:#x} owned={owned} valid={valid}"
    );
    call.set_return(emu, valid.into());
    call.cleanup_args(emu, 1);
}

fn stock_image(image_type: u32, id: u64) -> bool {
    match image_type {
        IMAGE_ICON => matches!(id, 32512..=32518),
        IMAGE_CURSOR => matches!(id,32512..=32516|32640..=32646|32648..=32651|32671|32672),
        _ => false,
    }
}

fn image(emu: &mut Emu, api: &str, image_type: Option<u32>) {
    let call = WinApi(Stdcall);
    let [module, name] = api_try!(emu, call.args(emu));
    let general = image_type.is_none();
    let (kind, width, height, flags) = if let Some(kind) = image_type {
        (kind, 0, 0, LR_SHARED)
    } else {
        (
            api_try!(emu, call.arg_u32(emu, 2)),
            api_try!(emu, call.arg_i32(emu, 3)),
            api_try!(emu, call.arg_i32(emu, 4)),
            api_try!(emu, call.arg_u32(emu, 5)),
        )
    };
    log_red!(
        emu,
        "user32!{api} instance={module:#x} resource={name:#x} image_type={kind} width={width} height={height} flags={flags:#x}"
    );
    if kind == IMAGE_BITMAP
        || flags & !(LR_SHARED | LR_DEFAULTSIZE) != 0
        || name > u64::from(u16::MAX)
    {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    let class = match kind {
        IMAGE_ICON => "icon",
        IMAGE_CURSOR => "cursor",
        _ => "",
    };
    let found = if module == 0 {
        stock_image(kind, name)
    } else if matches!(kind, IMAGE_ICON | IMAGE_CURSOR) {
        let group = if kind == IMAGE_ICON {
            RT_GROUP_ICON
        } else {
            RT_GROUP_CURSOR
        };
        let payload = if kind == IMAGE_ICON {
            RT_ICON
        } else {
            RT_CURSOR
        };
        numeric_payload(emu, module, group, name as u32)
            .ok()
            .filter(|(_, size)| *size >= 20)
            .and_then(|(at, _)| {
                let header = read_array::<20>(emu, at)?;
                let u16_at = |i: usize| u16::from_le_bytes([header[i], header[i + 1]]);
                (u16_at(0) == 0
                    && u32::from(u16_at(2)) == if kind == IMAGE_ICON { 1 } else { 2 }
                    && u16_at(4) != 0)
                    .then(|| u16_at(18))
            })
            .is_some_and(|id| {
                numeric_payload(emu, module, payload, id.into()).is_ok_and(|(_, size)| size > 0)
            })
    } else {
        false
    };
    let result = if found {
        allocate(
            emu,
            format!("{class}:{module:x}:{name}:{width}:{height}"),
            flags & LR_SHARED != 0,
        )
    } else {
        emu.set_last_error(ERROR_RESOURCE_NAME_NOT_FOUND);
        0
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, if general { 6 } else { 2 });
}

fn new_menu(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let id = emu.guest_windows.user.allocate().unwrap_or(0);
    if id != 0 {
        emu.guest_windows.user.menus.insert(id);
    } else {
        emu.set_last_error(ERROR_NOT_ENOUGH_MEMORY);
    }
    log_red!(emu, "user32!{api} menu={id:#x}");
    call.set_return(emu, id);
}
pub fn GetUserObjectInformationA(emu: &mut Emu) {
    system_object(emu, "GetUserObjectInformationA", false);
}

pub fn GetUserObjectInformationW(emu: &mut Emu) {
    system_object(emu, "GetUserObjectInformationW", true);
}

pub fn GetProcessWindowStation(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetProcessWindowStation");
    call.set_return(emu, WINDOW_STATION);
    call.cleanup_args(emu, 0);
}

pub fn OpenInputDesktop(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [flags, inherit, access] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!OpenInputDesktop flags={flags:#x} inherit={inherit} desired_access={access:#x}"
    );
    if flags != 0 {
        policy::unsupported(emu, "user32", "OpenInputDesktop", emu.pc());
        return;
    }
    let desktop = allocate(emu, "desktop".into(), false);
    call.set_return(emu, desktop);
    call.cleanup_args(emu, 3);
}

pub fn CloseDesktop(emu: &mut Emu) {
    close_object(emu, "CloseDesktop", "desktop");
}

pub fn DestroyCursor(emu: &mut Emu) {
    close_object(emu, "DestroyCursor", "cursor");
}

pub fn DestroyIcon(emu: &mut Emu) {
    close_object(emu, "DestroyIcon", "icon");
}

pub fn LoadIconA(emu: &mut Emu) {
    image(emu, "LoadIconA", Some(IMAGE_ICON));
}

pub fn LoadIconW(emu: &mut Emu) {
    image(emu, "LoadIconW", Some(IMAGE_ICON));
}

pub fn LoadCursorA(emu: &mut Emu) {
    image(emu, "LoadCursorA", Some(IMAGE_CURSOR));
}

pub fn LoadCursorW(emu: &mut Emu) {
    image(emu, "LoadCursorW", Some(IMAGE_CURSOR));
}

pub fn LoadImageA(emu: &mut Emu) {
    image(emu, "LoadImageA", None);
}

pub fn LoadImageW(emu: &mut Emu) {
    image(emu, "LoadImageW", None);
}

pub fn CopyIcon(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let key = emu
        .guest_windows
        .user
        .objects
        .get(&source)
        .map(|o| o.0.clone())
        .filter(|s| s.starts_with("icon:") || s.starts_with("cursor:"));
    let result = if let Some(kind) = key {
        allocate(emu, kind, false)
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        0
    };
    log_red!(emu, "user32!CopyIcon icon={source:#x} copy={result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn GetCursor(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let cursor = emu.guest_windows.user.cursor;
    log_red!(emu, "user32!GetCursor cursor={cursor:#x}");
    call.set_return(emu, cursor);
}

pub fn SetCursor(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let cursor = api_try!(emu, call.arg(emu, 0));
    let valid = cursor == 0
        || emu
            .guest_windows
            .user
            .objects
            .get(&cursor)
            .is_some_and(|(k, _)| k.starts_with("cursor:"));
    let previous = if valid {
        std::mem::replace(&mut emu.guest_windows.user.cursor, cursor)
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        0
    };
    log_red!(
        emu,
        "user32!SetCursor cursor={cursor:#x} previous={previous:#x}"
    );
    call.set_return(emu, previous);
    call.cleanup_args(emu, 1);
}

pub fn SetCursorPos(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let x = api_try!(emu, call.arg_i32(emu, 0));
    let y = api_try!(emu, call.arg_i32(emu, 1));
    emu.guest_windows.user.cursor_position = Some([x, y]);
    log_red!(emu, "user32!SetCursorPos x={x} y={y}");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub fn ShowCursor(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let show = api_try!(emu, call.arg_i32(emu, 0)) != 0;
    let thread = emu.current_thread().id;
    let count = emu
        .guest_windows
        .user
        .cursor_count
        .entry(thread)
        .or_default();
    *count = count.saturating_add(if show { 1 } else { -1 });
    let count = *count;
    log_red!(emu, "user32!ShowCursor show={show} count={count}");
    call.set_return(emu, u64::from(count as u32));
    call.cleanup_args(emu, 1);
}

pub fn CreateMenu(emu: &mut Emu) {
    new_menu(emu, "CreateMenu");
}

pub fn CreatePopupMenu(emu: &mut Emu) {
    new_menu(emu, "CreatePopupMenu");
}

pub fn GetMenu(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let menu = emu.guest_windows.get(hwnd).map_or(0, |w| w.menu);
    log_red!(emu, "user32!GetMenu hwnd={hwnd:#x} menu={menu:#x}");
    call.set_return(emu, menu);
    call.cleanup_args(emu, 1);
}

pub fn DestroyMenu(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let menu = api_try!(emu, call.arg(emu, 0));
    let ok = emu.guest_windows.user.menus.remove(&menu);
    if !ok {
        emu.set_last_error(ERROR_INVALID_MENU_HANDLE);
    }
    log_red!(emu, "user32!DestroyMenu menu={menu:#x} success={ok}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn GetMenuItemCount(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let menu = api_try!(emu, call.arg(emu, 0));
    let count = if emu.guest_windows.user.menus.contains(&menu) {
        0
    } else {
        emu.set_last_error(ERROR_INVALID_MENU_HANDLE);
        u32::MAX
    };
    log_red!(emu, "user32!GetMenuItemCount menu={menu:#x} count={count}");
    call.set_return(emu, count.into());
    call.cleanup_args(emu, 1);
}

pub fn GetSubMenu(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [menu, position] = api_try!(emu, call.args(emu));
    log_red!(emu, "user32!GetSubMenu menu={menu:#x} position={position}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

pub fn DeleteMenu(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [menu, position, flags] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!DeleteMenu menu={menu:#x} item={position} flags={flags:#x}"
    );
    if !emu.guest_windows.user.menus.contains(&menu) {
        emu.set_last_error(ERROR_INVALID_MENU_HANDLE);
    }
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
