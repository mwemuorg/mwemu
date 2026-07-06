use crate::emu;
use crate::maps::mem64::Permission;
use bitfield::bitfield;

const USER_KUSER_SHARED_ADDR: u64 = 0x7FFE0000;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum AlternativeArchitectureType {
    StandardDesign,
    Nec98x86,
    EndAlternatives,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum NtProductType {
    WinNt = 1,
    LanManNt,
    Server,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KsystemTime {
    pub LowPart: u32,
    pub High1Time: i32,
    pub High2Time: i32,
}

bitfield! {
    /// Represents the `KusdMitigationPoliciesUnion` from C.
    /// Backed by a single `u8` with 4 x 2-bit fields.
    #[derive(Clone, Copy)]
    pub struct KuserSharedData00(u8);
    u8;

    /// Bits 0-1: `NXSupportPolicy`
    nx_support_policy, set_nx_support_policy: 1, 0;

    /// Bits 2-3: `SEHValidationPolicy`
    seh_validation_policy, set_seh_validation_policy: 3, 2;

    /// Bits 4-5: `CurDirDevicesSkippedForDlls`
    cur_dir_devices_skipped, set_cur_dir_devices_skipped: 5, 4;

    /// Bits 6-7: Reserved
    reserved, set_reserved: 7, 6;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdMitigationPoliciesUnion {
    pub MitigationPolicies: u8,
    pub Anonymous: KuserSharedData00,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdVirtualizationFlagsUnion {
    pub VirtualizationFlags: u8,
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct KusdSharedDataFlagsBits(u32);
    u32;

    // Bit fields (total 32 bits)
    pub dbg_error_port_present, set_dbg_error_port_present: 0;
    pub dbg_elevation_enabled, set_dbg_elevation_enabled: 1;
    pub dbg_virt_enabled, set_dbg_virt_enabled: 2;
    pub dbg_installer_detect_enabled, set_dbg_installer_detect_enabled: 3;
    pub dbg_lkg_enabled, set_dbg_lkg_enabled: 4;
    pub dbg_dyn_processor_enabled, set_dbg_dyn_processor_enabled: 5;
    pub dbg_console_broker_enabled, set_dbg_console_broker_enabled: 6;
    pub dbg_secure_boot_enabled, set_dbg_secure_boot_enabled: 7;
    pub dbg_multi_session_sku, set_dbg_multi_session_sku: 8;
    pub dbg_multi_users_in_session_sku, set_dbg_multi_users_in_session_sku: 9;
    pub dbg_state_separation_enabled, set_dbg_state_separation_enabled: 10;
    pub dbg_split_token_enabled, set_dbg_split_token_enabled: 11;
    pub dbg_shadow_admin_enabled, set_dbg_shadow_admin_enabled: 12;
    pub spare_bits, set_spare_bits: 31, 13; // Bits 13..=31 (19 bits)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdSharedDataFlagsUnion {
    pub SharedDataFlags: u32,
    pub bits: KusdSharedDataFlagsBits,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OverlayStruct {
    pub ReservedTickCountOverlay: [u32; 3],
    pub TickCountPad: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdTickCountUnion {
    pub TickCount: KsystemTime,
    pub TickCountQuad: u64,
    pub Overlay: OverlayStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdQpcDataUnion {
    pub QpcData: u16,
    pub anonymous: KusdQpcDataAnon,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KusdQpcDataAnon {
    pub QpcBypassEnabled: u8,
    pub QpcReserved: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XstateFeature {
    pub Offset: u32,
    pub Size: u32,
}

// Bitfield helper for ControlFlags
bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct ControlFlagsBitfield(u32);
    u32;

    pub OptimizedSave, set_OptimizedSave: 0, 0;
    pub CompactionEnabled, set_CompactionEnabled: 1, 1;
    pub Reserved1, set_Reserved1: 31, 2;
}

// Anonymous union replacement using a wrapper
#[repr(C)]
#[derive(Clone, Copy)]
pub union ControlFlagsUnion {
    pub raw: u32,
    pub bits: ControlFlagsBitfield,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XstateConfiguration {
    pub EnabledFeatures: u64,
    pub EnabledVolatileFeatures: u64,
    pub Size: u32,
    pub Anonymous: ControlFlagsUnion,
    pub Features: [XstateFeature; 64],
    pub EnabledSupervisorFeatures: u64,
    pub AlignedFeatures: u64,
    pub AllFeatureSize: u32,
    pub AllFeatures: [u32; 64],
    pub EnabledUserVisibleSupervisorFeatures: u64,
    pub ExtendedFeatureDisableFeatures: u64,
    pub AllNonLargeFeatureSize: u32,
    pub MaxSveVectorLength: u16,
    pub Spare: u16,
}

/*
The KuserSharedData is getting from windows 24h2 from  vergilusproject
https://www.vergiliusproject.com/kernels/x64/windows-11/24h2/_KUSER_SHARED_DATA
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KuserSharedData {
    pub TickCountLowDeprecated: u32,
    pub TickCountMultiplier: u32,
    pub InterruptTime: KsystemTime,
    pub SystemTime: KsystemTime,
    pub TimeZoneBias: KsystemTime,
    pub ImageNumberLow: u16,
    pub ImageNumberHigh: u16,
    pub NtSystemRoot: [u16; 260],
    pub MaxStackTraceDepth: u32,
    pub CryptoExponent: u32,
    pub TimeZoneId: u32,
    pub LargePageMinimum: u32,
    pub AitSamplingValue: u32,
    pub AppCompatFlag: u32,
    pub RNGSeedVersion: u64,
    pub GlobalValidationRunlevel: u32,
    pub TimeZoneBiasStamp: i32,
    pub NtBuildNumber: u32,
    pub NtProductType: NtProductType,
    pub ProductTypeIsValid: bool,
    pub Reserved0: [bool; 1],
    pub NativeProcessorArchitecture: u16,
    pub NtMajorVersion: u32,
    pub NtMinorVersion: u32,
    pub ProcessorFeatures: [bool; 64],
    pub Reserved1: u32,
    pub Reserved3: u32,
    pub TimeSlip: u32,
    pub AlternativeArchitecture: AlternativeArchitectureType,
    pub BootId: u32,
    pub SystemExpirationDate: i64,
    pub SuiteMask: u32,
    pub KdDebuggerEnabled: bool,
    pub MitigationPolicies: KusdMitigationPoliciesUnion,
    pub CyclesPerYield: u16,
    pub ActiveConsoleId: u32,
    pub DismountCount: u32,
    pub ComPlusPackage: u32,
    pub LastSystemRITEventTickCount: u32,
    pub NumberOfPhysicalPages: u32,
    pub SafeBootMode: bool,
    pub VirtualizationFlags: KusdVirtualizationFlagsUnion,
    pub Reserved12: [u8; 2],
    pub SharedDataFlags: KusdSharedDataFlagsUnion,
    pub DataFlagsPad: [u32; 1],
    pub TestRetInstruction: u64,
    pub QpcFrequency: i64,
    pub SystemCall: u32,
    pub Reserved2: u32,
    pub SystemCallPad: [u64; 2],
    pub TickCount: KusdTickCountUnion,
    pub Cookie: u32,
    pub CookiePad: [u32; 1],
    pub ConsoleSessionForegroundProcessId: i64,
    pub TimeUpdateLock: u64,
    pub BaselineSystemTimeQpc: u64,
    pub BaselineInterruptTimeQpc: u64,
    pub QpcSystemTimeIncrement: u64,
    pub QpcInterruptTimeIncrement: u64,
    pub QpcSystemTimeIncrementShift: u8,
    pub QpcInterruptTimeIncrementShift: u8,
    pub UnparkedProcessorCount: u16,
    pub EnclaveFeatureMask: [u32; 4],
    pub TelemetryCoverageRound: u32,
    pub UserModeGlobalLogger: [u16; 16],
    pub ImageFileExecutionOptions: u32,
    pub LangGenerationCount: u32,
    pub Reserved4: u64,
    pub InterruptTimeBias: u64,
    pub QpcBias: u64,
    pub ActiveProcessorCount: u32,
    pub ActiveGroupCount: u8,
    pub Reserved9: u8,
    pub QpcData: KusdQpcDataUnion,
    pub TimeZoneBiasEffectiveStart: i64,
    pub TimeZoneBiasEffectiveEnd: i64,
    pub XState: XstateConfiguration,
    pub FeatureConfigurationChangeStamp: KsystemTime,
    pub Spare: u32,
    pub UserPointerAuthMask: u64,
    pub Reserved10: [u32; 210],
}

pub fn init_kuser_shared_data(emu: &mut emu::Emu) -> u64 {
    emu.maps
        .create_map(
            "KuserSharedData",
            USER_KUSER_SHARED_ADDR,
            0x1000,
            Permission::READ_WRITE,
        )
        .expect("cannot create KuserSharedData map");

    // KUSER_SHARED_DATA layout from Windows 11 24H2 (vergiliusproject) / sogen.
    //
    // Written field-by-field directly into the guest map — the same safe "save"
    // idiom the structures in structs.rs use. The region is zeroed, then each
    // non-zero field is written at its `#[repr(C)]` offset (resolved by
    // `offset_of!`). No `unsafe`: no zeroed-enum, no union access, no raw byte
    // reinterpretation. Fields left at zero keep their zero-initialized value.
    use std::mem::offset_of;
    let base = USER_KUSER_SHARED_ADDR;
    emu.maps
        .write_bytes(base, &vec![0u8; std::mem::size_of::<KuserSharedData>()]);

    macro_rules! wr8 {
        ($($f:tt).+, $v:expr) => {
            emu.maps
                .write_byte(base + offset_of!(KuserSharedData, $($f).+) as u64, $v);
        };
    }
    macro_rules! wr16 {
        ($($f:tt).+, $v:expr) => {
            emu.maps
                .write_word(base + offset_of!(KuserSharedData, $($f).+) as u64, $v);
        };
    }
    macro_rules! wr32 {
        ($($f:tt).+, $v:expr) => {
            emu.maps
                .write_dword(base + offset_of!(KuserSharedData, $($f).+) as u64, $v);
        };
    }
    macro_rules! wr64 {
        ($($f:tt).+, $v:expr) => {
            emu.maps
                .write_qword(base + offset_of!(KuserSharedData, $($f).+) as u64, $v);
        };
    }

    wr32!(TickCountMultiplier, 0x0fa00000);
    wr32!(InterruptTime.LowPart, 0x17bd9547);
    wr32!(InterruptTime.High1Time, 0x0000004b);
    wr32!(InterruptTime.High2Time, 0x0000004b);
    wr32!(SystemTime.LowPart, 0x7af9da99);
    wr32!(SystemTime.High1Time, 0x01db27b9);
    wr32!(SystemTime.High2Time, 0x01db27b9);
    wr32!(TimeZoneBias.LowPart, 0x3c773000);
    wr32!(TimeZoneBias.High1Time, (-17i32) as u32);
    wr32!(TimeZoneBias.High2Time, (-17i32) as u32);
    wr32!(TimeZoneId, 0x00000002);
    wr32!(LargePageMinimum, 0x00200000);
    wr64!(RNGSeedVersion, 0x0000000000000013);
    wr32!(TimeZoneBiasStamp, 0x00000004);
    wr32!(NtBuildNumber, 0x00006c51);
    wr32!(NtProductType, NtProductType::WinNt as u32);
    wr8!(ProductTypeIsValid, 1);
    wr16!(NativeProcessorArchitecture, 0x0009);
    wr32!(NtMajorVersion, 0x0000000a);
    wr32!(BootId, 0x0000000b);
    wr64!(SystemExpirationDate, 0x01dc26860a9ff300);
    wr32!(SuiteMask, 0x00000110);
    // MitigationPolicies (u8) = 0x0a already encodes NXSupportPolicy=2 (bits 0-1)
    // and SEHValidationPolicy=2 (bits 2-3).
    wr8!(MitigationPolicies, 0x0a);
    wr16!(CyclesPerYield, 0x0064);
    wr32!(DismountCount, 0x00000006);
    wr32!(ComPlusPackage, 0x00000001);
    wr32!(LastSystemRITEventTickCount, 0x01ec1fd3);
    wr32!(NumberOfPhysicalPages, 0x00bf0958);
    // TickCount union: write the 64-bit quad (covers LowPart; high parts stay 0).
    wr64!(TickCount, 0x00000000001f7f05);
    wr32!(Cookie, 0x1c3471da);
    wr64!(ConsoleSessionForegroundProcessId, 0x00000000000028f4);
    wr64!(TimeUpdateLock, 0x0000000002b28586);
    wr64!(BaselineSystemTimeQpc, 0x0000004b17cd596c);
    wr64!(BaselineInterruptTimeQpc, 0x0000004b17cd596c);
    wr64!(QpcSystemTimeIncrement, 0x8000000000000000);
    wr64!(QpcInterruptTimeIncrement, 0x8000000000000000);
    wr8!(QpcSystemTimeIncrementShift, 0x01);
    wr8!(QpcInterruptTimeIncrementShift, 0x01);
    wr16!(UnparkedProcessorCount, 0x000c);
    wr32!(TelemetryCoverageRound, 0x00000001);
    wr32!(LangGenerationCount, 0x00000003);
    wr64!(InterruptTimeBias, 0x00000015a5d56406);
    wr32!(ActiveProcessorCount, 0x0000000c);
    wr8!(ActiveGroupCount, 0x01);
    wr64!(TimeZoneBiasEffectiveStart, 0x01db276e654cb2ff);
    wr64!(TimeZoneBiasEffectiveEnd, 0x01db280b8c3b2800);
    wr64!(XState.EnabledFeatures, 0x000000000000001f);
    wr64!(XState.EnabledVolatileFeatures, 0x000000000000000f);
    wr32!(XState.Size, 0x000003c0);
    // QpcData union (u16): 0x0083 sets the low byte (QpcBypassEnabled).
    wr16!(QpcData, 0x0083);
    wr64!(QpcBias, 0x000000159530c4af);

    // RtlAllocateHeap checks [0x7ffe0380]. If 0, it falls back to STATUS_NO_MEMORY (error).
    emu.maps.write_byte(base + 0x380, 1);

    USER_KUSER_SHARED_ADDR
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::offset_of;

    // Guards the field-by-field "save" rewrite (no more unsafe struct→bytes copy):
    // verifies representative fields land at their `#[repr(C)]` offsets with the
    // right width/value, so a wrong write width or a dropped field is caught.
    #[test]
    fn writes_expected_fields() {
        let mut emu = crate::emu64();
        let base = init_kuser_shared_data(&mut emu);
        assert_eq!(base, USER_KUSER_SHARED_ADDR);

        let rd = |o: usize| -> u64 { emu.maps.read_qword(base + o as u64).unwrap() };
        let rd32 = |o: usize| -> u32 { emu.maps.read_dword(base + o as u64).unwrap() };

        assert_eq!(
            rd32(offset_of!(KuserSharedData, TickCountMultiplier)),
            0x0fa00000
        );
        assert_eq!(
            rd32(offset_of!(KuserSharedData, InterruptTime.LowPart)),
            0x17bd9547
        );
        assert_eq!(rd32(offset_of!(KuserSharedData, NtBuildNumber)), 0x6c51);
        assert_eq!(rd32(offset_of!(KuserSharedData, NtProductType)), 1); // WinNt
        assert_eq!(rd32(offset_of!(KuserSharedData, NtMajorVersion)), 0xa);
        assert_eq!(rd32(offset_of!(KuserSharedData, Cookie)), 0x1c3471da);
        assert_eq!(
            emu.maps
                .read_byte(base + offset_of!(KuserSharedData, MitigationPolicies) as u64)
                .unwrap(),
            0x0a
        );
        assert_eq!(rd(offset_of!(KuserSharedData, TickCount)), 0x1f7f05);
        assert_eq!(rd(offset_of!(KuserSharedData, QpcBias)), 0x000000159530c4af);
        assert_eq!(
            rd(offset_of!(KuserSharedData, XState.EnabledFeatures)),
            0x1f
        );
        // RtlAllocateHeap probe byte.
        assert_eq!(emu.maps.read_byte(base + 0x380).unwrap(), 1);
        // A field left at zero stays zero.
        assert_eq!(rd32(offset_of!(KuserSharedData, TickCountLowDeprecated)), 0);
    }
}
