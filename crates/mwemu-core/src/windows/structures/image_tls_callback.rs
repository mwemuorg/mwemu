#[derive(Debug)]
pub struct ImageTlsCallback {
    // every tls callback has this structure
    dll_handle: u32,
    reason: u32,
    reserved: u32,
}
