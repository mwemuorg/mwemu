use crate::maps::Maps;
use crate::maps::mem64::Permission;
use gdbstub::target::TargetError;

use super::MwemuGdbError;

pub(super) fn read_memory(
    maps: &Maps,
    address: u64,
    data: &mut [u8],
) -> Result<usize, TargetError<MwemuGdbError>> {
    address
        .checked_add(data.len() as u64)
        .ok_or(TargetError::NonFatal)?;
    let length = maps
        .first_inaccessible_address(address, data.len(), Permission::READ)
        .map_or(data.len(), |end| (end - address) as usize);
    if length == 0 && !data.is_empty() {
        return Err(TargetError::NonFatal);
    }
    let bytes = maps
        .try_read_bytes(address, length)
        .ok_or(TargetError::NonFatal)?;
    data[..length].copy_from_slice(&bytes);
    Ok(length)
}

pub(super) fn write_memory(
    maps: &mut Maps,
    address: u64,
    data: &[u8],
) -> Result<(), TargetError<MwemuGdbError>> {
    // Validate the entire range before writing, and retain code-cache invalidation.
    if maps.write_bytes(address, data) {
        Ok(())
    } else {
        Err(TargetError::NonFatal)
    }
}

pub(crate) fn copy_range(source: &[u8], offset: u64, length: usize, buf: &mut [u8]) -> usize {
    let offset = offset as usize;

    if offset >= source.len() {
        return 0;
    }

    let remaining = &source[offset..];
    let to_copy = remaining.len().min(length).min(buf.len());
    buf[..to_copy].copy_from_slice(&remaining[..to_copy]);
    to_copy
}

pub(crate) fn generate_library_list_xml(maps: &Maps) -> String {
    let mut xml = String::from(
        r#"<library-list>
"#,
    );

    for (_, mem) in maps.mem_slab.iter() {
        let name = mem.get_name();
        if name.ends_with(".pe") || name.ends_with(".dll") || name.ends_with(".exe") {
            let base = mem.get_base();
            xml.push_str(&format!(
                r#"  <library name="{}">
    <segment address="0x{:x}"/>
  </library>
"#,
                escape_xml(name),
                base + 0x1000
            ));
        }
    }

    xml.push_str("</library-list>\n");
    xml
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
