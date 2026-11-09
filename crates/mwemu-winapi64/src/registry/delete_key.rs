use super::finish;
use super::key_path;
use super::read_name;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn delete_key(emu: &mut emu::Emu, api: &str, wide: bool, tree: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide);
    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, "", err),
    };
    let err = match sub.as_deref() {
        // RegDeleteTree without a subkey empties the key itself
        None | Some("") if tree => {
            store::clear_key(&base);
            ERROR_SUCCESS
        }
        // RegDeleteKey needs a subkey: a key cannot delete itself
        None | Some("") => ERROR_INVALID_PARAMETER,
        Some(sub) => match store::delete_key(&store::join(&base, sub), tree) {
            Ok(()) => ERROR_SUCCESS,
            Err(store::DeleteError::NotFound) => ERROR_FILE_NOT_FOUND,
            Err(store::DeleteError::HasSubkeys) => ERROR_ACCESS_DENIED,
        },
    };
    let what = store::join(&base, sub.as_deref().unwrap_or(""));
    finish(emu, api, &what, err);
}
