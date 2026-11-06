use crate::emu;
use crate::windows::constants;

const DOS_DEVICES: &[(&str, &str)] = &[("C:", constants::SYSTEM_VOLUME_DEVICE)];

pub fn QueryDosDeviceW(emu: &mut emu::Emu) {
    let device_ptr = emu.regs().rcx;
    let target_ptr = emu.regs().rdx;
    let max_chars = emu.regs().r8 & 0xffff_ffff;

    let device = if device_ptr == 0 {
        None
    } else {
        Some(emu.maps.read_wide_string(device_ptr))
    };

    log_red!(
        emu,
        "kernel32!QueryDosDeviceW device: {:?} lpTargetPath: 0x{:x} ucchMax: {}",
        device,
        target_ptr,
        max_chars
    );

    let strings: Vec<&str> = match device.as_deref() {
        None => DOS_DEVICES.iter().map(|(name, _)| *name).collect(),
        Some(name) => match DOS_DEVICES
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
        {
            Some((_, target)) => vec![*target],
            None => {
                emu.set_last_error(constants::ERROR_FILE_NOT_FOUND);
                emu.regs_mut().rax = 0;
                return;
            }
        },
    };
    let mut chars: Vec<u16> = Vec::new();
    for s in strings {
        chars.extend(s.encode_utf16());
        chars.push(0);
    }
    chars.push(0);

    if target_ptr == 0 || (chars.len() as u64) > max_chars {
        emu.set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = 0;
        return;
    }

    let bytes: Vec<u8> = chars.iter().flat_map(|c| c.to_le_bytes()).collect();
    emu.maps.write_bytes(target_ptr, &bytes);
    emu.regs_mut().rax = chars.len() as u64;
}
