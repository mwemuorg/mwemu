use super::PE64;
use crate::pe::loader::PeLoader;
use crate::pe::relocation;
use crate::pe::shared::{IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_REL_BASED_DIR64};

const DATA_DIRECTORY_OFFSET: usize = 112;

impl PE64 {
    /// True when the headers contain a nonempty base relocation directory.
    pub fn has_relocations(&self) -> bool {
        self.opt
            .data_directory
            .get(IMAGE_DIRECTORY_ENTRY_BASERELOC)
            .is_some_and(|dir| dir.virtual_address != 0 && dir.size != 0)
    }

    fn visit_relocations(&self, raw: &[u8], apply: impl FnMut(u32, [u8; 8])) -> Option<()> {
        let layout = self.image_layout();
        let blocks = relocation::directory(
            raw,
            layout,
            self.dos.e_lfanew,
            &self.fh,
            DATA_DIRECTORY_OFFSET,
            self.opt.number_of_rva_and_sizes,
            &self.opt.data_directory,
        )?;
        relocation::visit(raw, layout, blocks, IMAGE_REL_BASED_DIR64, apply)
    }

    /// Whether the image can move without losing a base fixup. Position-independent
    /// images can have no fixups and leave the relocation directory empty.
    pub fn can_relocate(&self, raw: &[u8]) -> bool {
        self.visit_relocations(raw, |_, _| {}).is_some()
    }

    /// Rebase initialized image values through the same layout used to map them.
    pub fn apply_relocations<L: PeLoader>(&self, raw: &[u8], loader: &mut L, base_addr: u64) {
        let delta = base_addr.wrapping_sub(self.opt.image_base);
        if delta == 0 {
            return;
        }
        log::trace!("applying PE64 base relocations (delta {delta:#x})");
        let _ = self.visit_relocations(raw, |rva, value| {
            if let Some(address) = base_addr.checked_add(u64::from(rva)) {
                loader.write_qword(address, u64::from_le_bytes(value).wrapping_add(delta));
            }
        });
    }
}
