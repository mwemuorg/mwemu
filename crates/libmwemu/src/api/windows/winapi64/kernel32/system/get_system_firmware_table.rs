use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemFirmwareTable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_signature = api_try!(emu, call.arg(emu, 0));
    let table_id = api_try!(emu, call.arg(emu, 1));
    let buffer_ptr = api_try!(emu, call.arg(emu, 2));
    let buffer_size = api_try!(emu, call.arg(emu, 3));

    // TODO: Parse FirmwareTableProviderSignature values:
    //   - 'ACPI' = 0x41435049 (ACPI tables)
    //   - 'FIRM' = 0x4649524D (firmware tables)
    //   - 'RSMB' = 0x52534D42 (SMBIOS/DMI tables)

    // TODO: Parse common FirmwareTableID values for ACPI:
    //   - 'RSDT' = 0x54445352 (Root System Description Table)
    //   - 'XSDT' = 0x54445358 (Extended System Description Table)
    //   - 'FACP' = 0x50434146 (Fixed ACPI Description Table)
    //   - 'DSDT' = 0x54445344 (Differentiated System Description Table)

    // TODO: For SMBIOS ('RSMB'), table_id is usually 0
    // TODO: If buffer_ptr is NULL, return required buffer size
    // TODO: If buffer_size is too small, return required size
    // TODO: Fill buffer with fake firmware table data
    // TODO: Common anti-VM check: looks for VMware/VirtualBox signatures in SMBIOS

    log_red!(
        emu,
        "kernel32!GetSystemFirmwareTable provider: 0x{:x} ('{}') table_id: 0x{:x} buffer: 0x{:x} size: {}",
        provider_signature,
        std::str::from_utf8(&provider_signature.to_le_bytes()).unwrap_or("????"),
        table_id,
        buffer_ptr,
        buffer_size
    );

    // TODO: Return actual bytes written/required, or 0 on error
    if buffer_size == 0 {
        emu.set_last_error(0);
        call.set_return(emu, 1024 * 1024); // 1MB
    } else {
        call.set_return(emu, 0); // Return 0 (error) for now
    }
}
