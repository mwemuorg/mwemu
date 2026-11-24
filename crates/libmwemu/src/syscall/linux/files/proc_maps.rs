use std::collections::BTreeMap;
use std::io::Write;

use super::LinuxFiles;

use crate::emu::Emu;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::syscall::linux::constants::elf::PT_LOAD;

const MAX_PROC_MAPS_SIZE: usize = 1024 * 1024;

fn image_offset(emu: &Emu, name: &str, base: u64) -> Option<u64> {
    if let Some(elf) = emu.elf64.as_ref() {
        if let Some(index) = name.strip_prefix("elf64.seg") {
            let segment = elf.elf_phdr.get(index.parse::<usize>().ok()?)?;
            return Some(segment.p_offset & !(PAGE_SIZE - 1));
        }
        if name == "code" || name.starts_with("elf64.") {
            return elf
                .elf_shdr
                .iter()
                .find(|sh| sh.sh_addr == base)
                .map(|sh| sh.sh_offset);
        }
    }
    let index = name.strip_prefix("elf32_seg")?.parse::<usize>().ok()?;
    let segment = emu
        .elf32
        .as_ref()?
        .elf_phdr
        .iter()
        .filter(|ph| ph.p_type == PT_LOAD)
        .nth(index)?;
    Some(segment.p_offset as u64)
}

pub(super) fn proc_maps(emu: &Emu, fs: &LinuxFiles) -> Vec<u8> {
    let mut bytes = Vec::new();
    let shared_files: BTreeMap<_, _> = fs
        .mapping_stores
        .iter()
        .filter_map(|(&inode, &id)| Some((id, (inode, fs.nodes.get(&inode)?.name.as_str()))))
        .collect();
    for (&base, &index) in &emu.maps.maps {
        let map = &emu.maps.mem_slab[index];
        let name = map.get_name();
        let view = map.shared_view();
        let mode = if view.is_some() { 's' } else { 'p' };
        let (offset, inode, label) = if let Some((entry, &(inode, path))) =
            view.and_then(|entry| shared_files.get(&entry.storage).map(|file| (entry, file)))
        {
            (Some(entry.offset as u64), inode, path)
        } else if let Some(offset) = image_offset(emu, name, base) {
            (Some(offset), fs.executable_inode, fs.executable.as_str())
        } else {
            let label = match name {
                "stack" => "[stack]",
                "heap" | ".heap" => "[heap]",
                _ => "",
            };
            (None, 0, label)
        };
        let device = if inode == 0 { "00:00" } else { "00:01" };
        let label = label.replace('\n', "\\012");
        for (start, len, protection) in map.permission_runs(base, map.size() as u64) {
            let read = if protection.can_read() { 'r' } else { '-' };
            let write = if protection.can_write() { 'w' } else { '-' };
            let exec = if protection.can_execute() { 'x' } else { '-' };
            let end = start + len;
            let offset = offset.map_or(0, |file| file + (start - base));
            let previous = bytes.len();
            let _ = writeln!(
                bytes,
                "{start:08x}-{end:08x} {read}{write}{exec}{mode} {offset:08x} {device} {inode} {label}"
            );
            if bytes.len() > MAX_PROC_MAPS_SIZE {
                bytes.truncate(previous);
                return bytes;
            }
        }
    }
    bytes
}
