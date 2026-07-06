use crate::emu;

pub fn GetSystemFirmwareTable(emu: &mut emu::Emu) {
    let provider_signature = emu.regs().rcx;
    let table_id = emu.regs().rdx;
    let buffer_ptr = emu.regs().r8;
    let buffer_size = emu.regs().r9;

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
        emu.regs_mut().rax = 1024 * 1024; // 1MB
    } else {
        emu.regs_mut().rax = 0; // Return 0 (error) for now
    }
}
