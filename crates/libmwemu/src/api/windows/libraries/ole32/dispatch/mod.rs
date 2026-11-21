use crate::emu::Emu;

mod marshaling;
mod objects;
mod runtime;
mod storage;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    objects::dispatch(api, emu)
        || runtime::dispatch(api, emu)
        || marshaling::dispatch(api, emu)
        || storage::dispatch(api, emu)
}
