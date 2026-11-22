use crate::emu::Emu;
use crate::winapi::common::files::{FileQuery, query_path};

pub fn PathFileExistsA(emu: &mut Emu) {
    query_path(emu, "shlwapi!PathFileExistsA", false, FileQuery::Exists);
}

pub fn PathFileExistsW(emu: &mut Emu) {
    query_path(emu, "shlwapi!PathFileExistsW", true, FileQuery::Exists);
}
