use crate::maps::Maps;

pub mod peb32;
pub mod peb64;

fn map_wide_string(maps: &mut Maps, name: &str, text: &str, capacity: u64) -> u64 {
    let Some(map) = maps.alloc_rw_pages(name, capacity) else {
        log::error!("cannot allocate {capacity} bytes for process parameter {name}");
        return 0;
    };
    let base = map.get_base();
    map.write_wide_string(base, text);
    base
}

/// Build the guest command line from its image path and the configured raw arguments.
pub(crate) fn command_line(image: &str, arguments: &str) -> String {
    let mut line = if image.contains([' ', '\t']) {
        format!("\"{image}\"")
    } else {
        image.to_owned()
    };
    if !arguments.is_empty() && !arguments.starts_with([' ', '\t']) {
        line.push(' ');
    }
    line.push_str(arguments);
    line
}
