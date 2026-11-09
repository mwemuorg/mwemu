use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn MapViewOfFileEx(emu: &mut Emu) {
    sections::map_view(emu, true);
}
