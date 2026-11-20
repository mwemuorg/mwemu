use crate::emu::Emu;
use crate::winapi::common::strings;

mod clipboard;
mod dde;
mod dialogs;
mod graphics;
mod input;
mod menus;
mod messages;
mod strings_format;
mod system;
mod windowing;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    strings::dispatch(api, "user32", emu)
        || clipboard::dispatch(api, emu)
        || dde::dispatch(api, emu)
        || dialogs::dispatch(api, emu)
        || graphics::dispatch(api, emu)
        || input::dispatch(api, emu)
        || menus::dispatch(api, emu)
        || messages::dispatch(api, emu)
        || strings_format::dispatch(api, emu)
        || system::dispatch(api, emu)
        || windowing::dispatch(api, emu)
}
