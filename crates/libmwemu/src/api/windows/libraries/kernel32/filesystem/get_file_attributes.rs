use crate::emu::Emu;
use crate::winapi::common::files::{FileQuery, query_path};

pub fn GetFileAttributesA(emu: &mut Emu) {
    query_path(
        emu,
        "kernel32!GetFileAttributesA",
        false,
        FileQuery::Attributes,
    );
}

pub fn GetFileAttributesW(emu: &mut Emu) {
    query_path(
        emu,
        "kernel32!GetFileAttributesW",
        true,
        FileQuery::Attributes,
    );
}

pub fn GetFileAttributesExA(emu: &mut Emu) {
    query_path(
        emu,
        "kernel32!GetFileAttributesExA",
        false,
        FileQuery::Extended,
    );
}

pub fn GetFileAttributesExW(emu: &mut Emu) {
    query_path(
        emu,
        "kernel32!GetFileAttributesExW",
        true,
        FileQuery::Extended,
    );
}
