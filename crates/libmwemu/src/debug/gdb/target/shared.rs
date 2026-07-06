use crate::maps::Maps;

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
