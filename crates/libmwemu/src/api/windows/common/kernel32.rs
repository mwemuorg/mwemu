pub(crate) fn is_api_set_contract(module: &str) -> bool {
    let module = module.trim().to_ascii_lowercase();
    module.starts_with("api-ms-win-") || module.starts_with("ext-ms-")
}

pub(crate) fn normalize_library_name(libname: &str, allow_exe_suffix: bool) -> Option<String> {
    let mut dll = libname.trim().to_ascii_lowercase();
    if dll.is_empty() {
        return None;
    }

    if allow_exe_suffix {
        if !dll.ends_with(".dll") && !dll.ends_with(".exe") {
            dll.push_str(".dll");
        }
    } else if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    Some(dll)
}

pub(crate) fn module_name_matches(flink_mod: &str, want: &str) -> bool {
    normalize_module_name(flink_mod) == normalize_module_name(want)
}

fn normalize_module_name(module_name: &str) -> String {
    let module_name = module_name.trim().to_ascii_lowercase();
    let module_name = module_name
        .rsplit_once('\\')
        .map(|(_, name)| name)
        .unwrap_or(&module_name);
    let module_name = module_name
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(module_name);
    module_name
        .strip_suffix(".dll")
        .unwrap_or(module_name)
        .to_string()
}
