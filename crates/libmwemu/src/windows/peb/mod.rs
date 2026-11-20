pub mod peb32;
pub mod peb64;

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
