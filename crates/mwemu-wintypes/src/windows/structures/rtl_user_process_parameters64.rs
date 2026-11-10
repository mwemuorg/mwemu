use super::unicode_string64::UnicodeString64;
use crate::maps::Maps;

/// Matches the leading fields of Windows `RTL_USER_PROCESS_PARAMETERS` (x64).
/// `Flags` bit 0 is `RTL_USER_PROCESS_PARAMETERS_NORMALIZED`: `ImagePathName.Buffer` /
/// `CommandLine.Buffer` are absolute pointers. If this is clear, ntdll treats `Buffer` as an
/// offset from the process-parameters base (`lea rdx,[rax+rcx]` path).
pub const RTL_USER_PROCESS_PARAMETERS_NORMALIZED: u32 = 0x0001;
/// `Flags` bit 0x2000: app manifest is present.
pub const RTL_USER_PROCESS_PARAMETERS_APP_MANIFEST: u32 = 0x2000;
/// `Flags` bit 0x4000: image registry key is missing (no IFEO entry).
/// When set ntdll skips Image File Execution Options lookups and the CSRSS
/// client-connect sequence, preventing `LdrpInitializeCsrss` from reaching
/// `RtlFailFast2(STATUS_STACK_BUFFER_OVERRUN)` via the ALPC error path.
pub const RTL_USER_PROCESS_PARAMETERS_IMAGE_KEY_MISSING: u32 = 0x4000;

#[derive(Debug)]
pub struct RtlUserProcessParameters64 {
    pub maximum_length: u32,
    pub length: u32,
    pub flags: u32,
    pub debug_flags: u32,
    /// ConsoleHandle through CurrentDirectory.Handle (Windows offsets 0x10–0x4F).
    pub reserved2: [u64; 8],
    /// `DllPath` at offset 0x50; ntdll may follow `Buffer` during loader init.
    pub dll_path: UnicodeString64,
    pub image_path_name: UnicodeString64,
    pub command_line: UnicodeString64,
}

impl Default for RtlUserProcessParameters64 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlUserProcessParameters64 {
    pub fn new() -> Self {
        Self {
            maximum_length: Self::size() as u32,
            length: Self::size() as u32,
            flags: RTL_USER_PROCESS_PARAMETERS_NORMALIZED
                | RTL_USER_PROCESS_PARAMETERS_APP_MANIFEST
                | RTL_USER_PROCESS_PARAMETERS_IMAGE_KEY_MISSING,
            debug_flags: 0,
            reserved2: [0; 8],
            dll_path: UnicodeString64::new(),
            image_path_name: UnicodeString64::new(),
            command_line: UnicodeString64::new(),
        }
    }

    pub fn size() -> usize {
        // Real Windows 10+ RTL_USER_PROCESS_PARAMETERS extends to ~0x440+.
        // ntdll accesses fields up to at least +0x428 (HeapPartitionName.Buffer).
        0x480
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.maximum_length);
        maps.write_dword(addr + 4, self.length);
        maps.write_dword(addr + 8, self.flags);
        maps.write_dword(addr + 12, self.debug_flags);
        for (i, val) in self.reserved2.iter().enumerate() {
            maps.write_qword(addr + 16 + (i * 8) as u64, *val);
        }
        self.dll_path.save(addr + 0x50, maps);
        self.image_path_name.save(addr + 0x60, maps);
        self.command_line.save(addr + 0x70, maps)
    }
}
