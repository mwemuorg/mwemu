use crate::emu::Emu;
pub mod filesystem;
pub mod launch;
pub mod objects;
pub mod ui;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    launch::dispatch(api, emu)
        || ui::dispatch(api, emu)
        || objects::dispatch(api, emu)
        || filesystem::dispatch(api, emu)
}
