use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn delete_key(emu: &mut emu::Emu, api: &str, wide: bool, tree: bool) {
    let call = WinApi(Stdcall);
    let slots = if api.starts_with("RegDeleteKeyEx") {
        4
    } else {
        2
    };
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sub = match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
        Ok(value) => value,
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let base = match key_path(emu, hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, slots, api, "", err),
    };
    let err = match sub.as_deref() {
        // RegDeleteTree without a subkey empties the key itself
        None | Some("") if tree => {
            emu.registry.clear_key(&base);
            emu.handle_management.invalidate_registry_key(&base, false);
            ERROR_SUCCESS
        }
        // RegDeleteKey needs a subkey: a key cannot delete itself
        None | Some("") => ERROR_INVALID_PARAMETER,
        Some(sub) => delete_subkey(emu, &store::join(&base, sub), tree)
            .err()
            .unwrap_or(ERROR_SUCCESS),
    };
    let what = store::join(&base, sub.as_deref().unwrap_or(""));
    finish(emu, slots, api, &what, err);
}

pub(super) fn delete_subkey(emu: &mut emu::Emu, path: &str, recursive: bool) -> Result<(), u64> {
    emu.registry
        .delete_key(path, recursive)
        .map_err(|error| match error {
            store::DeleteError::NotFound => ERROR_FILE_NOT_FOUND,
            store::DeleteError::HasSubkeys => ERROR_ACCESS_DENIED,
        })?;
    emu.handle_management.invalidate_registry_key(path, true);
    Ok(())
}
