use crate::api::dispatch_api;
use crate::api::windows::libraries::shell32::filesystem::{SHGetFolderPathA, SHGetFolderPathW};
use crate::emu::Emu;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "SHGetFolderPathA" => SHGetFolderPathA(emu),
        "SHGetFolderPathW" => SHGetFolderPathW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
