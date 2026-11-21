use crate::api::windows::policy;
use crate::emu::Emu;

/// Compatibility adapter for gateways that still return an empty string on success.
pub(crate) fn unimplemented(addr: impl Into<u64>, api: &str, emu: &mut Emu) -> String {
    let addr = addr.into();
    let api = api.rsplit('!').next().unwrap_or(api);
    let library = emu
        .maps
        .get_addr_name(addr)
        .unwrap_or("unknown")
        .split('.')
        .next()
        .unwrap_or("unknown")
        .to_string();
    policy::unsupported(emu, &library, api, addr);
    if emu.api_policy.stopped() {
        api.to_ascii_lowercase()
    } else {
        String::new()
    }
}
