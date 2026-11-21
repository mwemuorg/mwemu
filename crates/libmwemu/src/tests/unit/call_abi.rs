use crate::api::abi::ArgumentFault;
use crate::api::abi::unix::UnixApi;
use crate::api::abi::unix::VariadicConvention;
use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall, Thiscall},
    WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter},
};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::{emu_aarch64, emu32, emu64};

const STACK: u64 = 0x500003;
const DWORD_BYTES: u64 = size_of::<u32>() as u64;
const QWORD_BYTES: u64 = size_of::<u64>() as u64;
const SHADOW_SPACE: u64 = 4 * QWORD_BYTES;
const VALUE: u64 = 0x1234_5678_90ab_cdef;

fn map_bytes(emu: &mut Emu, address: u64, bytes: &[u8], separate: bool) {
    if separate {
        for (index, byte) in bytes.iter().enumerate() {
            let address = address + index as u64;
            emu.maps
                .create_map(
                    &format!("byte_{address:x}"),
                    address,
                    1,
                    Permission::READ_WRITE,
                )
                .unwrap();
            assert!(emu.maps.write_byte(address, *byte));
        }
    } else {
        emu.maps
            .create_map(
                &format!("arguments_{address:x}"),
                address,
                bytes.len() as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        assert!(emu.maps.write_bytes(address, bytes));
    }
}

#[test]
fn zero_is_a_readable_argument() {
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.set_sp(STACK);
        let (index, offset, bytes) = if is64 {
            (4, SHADOW_SPACE, QWORD_BYTES)
        } else {
            (0, 0, DWORD_BYTES)
        };
        assert_eq!(
            WinApi(Stdcall).arg(&emu, index).unwrap_err().fault,
            ArgumentFault::Unreadable
        );
        map_bytes(&mut emu, STACK + offset, &vec![0; bytes as usize], false);
        assert_eq!(WinApi(Stdcall).arg(&emu, index), Ok(0));
    }
}

#[test]
fn arguments_can_span_many_adjacent_maps() {
    for separate in [false, true] {
        let mut x86 = emu32();
        x86.set_sp(STACK);
        map_bytes(&mut x86, STACK, &VALUE.to_le_bytes(), separate);
        assert_eq!(WinApi(Stdcall).arg(&x86, 0), Ok(VALUE as u32 as u64));
        assert_eq!(WinApi(Stdcall).arg_u64(&x86, 0), Ok(VALUE));
        assert_eq!(
            WinApi(Stdcall).arg_f64(&x86, 0, 0).unwrap().to_bits(),
            VALUE
        );
        assert_eq!(
            WinApi(Stdcall).arg_f32(&x86, 0, 0).unwrap().to_bits(),
            VALUE as u32
        );

        let mut x64 = emu64();
        x64.set_sp(STACK);
        map_bytes(
            &mut x64,
            STACK + SHADOW_SPACE,
            &VALUE.to_le_bytes(),
            separate,
        );
        assert_eq!(WinApi(Stdcall).arg(&x64, 4), Ok(VALUE));
    }
}

#[test]
fn incomplete_and_unreadable_arguments_are_errors() {
    for deny in [false, true] {
        let mut emu = emu32();
        emu.set_sp(STACK);
        map_bytes(&mut emu, STACK, &VALUE.to_le_bytes()[..3], false);
        if deny {
            emu.maps
                .create_map(
                    "unreadable",
                    STACK + 3,
                    5,
                    Permission::from_flags(false, true, false),
                )
                .unwrap();
        }
        for result in [
            WinApi(Stdcall).arg(&emu, 0),
            WinApi(Stdcall).arg_u64(&emu, 0),
        ] {
            let error = result.unwrap_err();
            assert_eq!(error.fault, ArgumentFault::Unreadable);
            assert_eq!(error.address, Some(STACK + 3));
        }
    }
}

#[test]
fn argument_addresses_obey_guest_width_and_checked_arithmetic() {
    let mut x86 = emu32();
    x86.set_sp(u32::MAX as u64 - 1);
    assert_eq!(
        WinApi(Stdcall).arg(&x86, 0).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
    x86.set_sp(u32::MAX as u64 - 5);
    assert_eq!(
        WinApi(Stdcall).arg_u64(&x86, 0).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
    x86.set_sp(STACK);
    assert_eq!(
        WinApi(Stdcall).arg_u64(&x86, usize::MAX).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );

    let mut x64 = emu64();
    x64.set_sp(u64::MAX - 16);
    assert_eq!(
        WinApi(Stdcall).arg(&x64, 4).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
    x64.set_sp(STACK);
    assert_eq!(
        WinApi(Stdcall).arg(&x64, usize::MAX).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
}

#[test]
fn api_and_syscall_keep_distinct_x64_stack_layouts() {
    let mut emu = emu64();
    emu.set_sp(STACK);
    emu.regs_mut().rcx = VALUE;
    emu.regs_mut().rdx = VALUE + 1;
    emu.regs_mut().r8 = VALUE + 2;
    emu.regs_mut().r9 = VALUE + 3;
    let bytes: Vec<_> = [VALUE + 4, VALUE + 5]
        .into_iter()
        .flat_map(u64::to_le_bytes)
        .collect();
    map_bytes(&mut emu, STACK + SHADOW_SPACE, &bytes, false);
    for index in 0..4 {
        assert_eq!(WinApi(Stdcall).arg(&emu, index), Ok(VALUE + index as u64));
        assert_eq!(WinSyscall.arg(&emu, index), Ok(VALUE + index as u64));
    }
    assert_eq!(WinApi(Stdcall).arg(&emu, 4), Ok(VALUE + 4));
    assert_eq!(WinSyscall.arg(&emu, 4), Ok(VALUE + 5));
    assert_eq!(emu.sp(), STACK);
}

#[test]
fn floating_arguments_use_their_type_width_and_register_class() {
    let mut emu = emu64();
    emu.set_sp(STACK);
    emu.regs_mut().rcx = VALUE;
    emu.regs_mut().xmm0 = 1.25f64.to_bits() as u128;
    emu.regs_mut().xmm1 = 2.5f32.to_bits() as u128;
    assert_eq!(WinApi(Stdcall).arg_f64(&emu, 0, 0), Ok(1.25));
    assert_eq!(WinApi(Stdcall).arg_f32(&emu, 1, 2), Ok(2.5));
    assert_eq!(WinApi(Stdcall).arg(&emu, 0), Ok(VALUE));
    map_bytes(
        &mut emu,
        STACK + SHADOW_SPACE,
        &3.75f32.to_bits().to_le_bytes(),
        false,
    );
    assert_eq!(WinApi(Stdcall).arg_f32(&emu, 4, 0), Ok(3.75));
    assert_eq!(WinApi(Stdcall).arg_u32(&emu, 4), Ok(3.75f32.to_bits()));
    assert_eq!(
        WinApi(Stdcall).arg_f64(&emu, 4, 0).unwrap_err().fault,
        ArgumentFault::Unreadable
    );
}

#[test]
fn arm64_integer_arguments_do_not_use_x64_shadow_space() {
    let mut emu = emu_aarch64();
    emu.set_sp(STACK);
    emu.regs_aarch64_mut().x[7] = VALUE;
    assert_eq!(WinApi(Stdcall).arg_u8(&emu, 7), Ok(VALUE as u8));
    assert_eq!(UnixApi.arg_i16(&emu, 7), Ok(VALUE as i16));
    map_bytes(&mut emu, STACK, &(VALUE + 1).to_le_bytes(), false);
    assert_eq!(WinApi(Stdcall).arg(&emu, 7), Ok(VALUE));
    assert_eq!(WinApi(Stdcall).arg(&emu, 8), Ok(VALUE + 1));
    assert_eq!(
        WinApi(Stdcall).arg_f64(&emu, 0, 0).unwrap_err().fault,
        ArgumentFault::UnsupportedConvention
    );
}

#[test]
fn unix_and_windows_hooks_keep_distinct_register_and_stack_layouts() {
    let mut emu = emu64();
    emu.set_sp(STACK);
    emu.regs_mut().rdi = VALUE;
    emu.regs_mut().rsi = VALUE + 1;
    emu.regs_mut().rdx = VALUE + 2;
    emu.regs_mut().rcx = VALUE + 3;
    emu.regs_mut().r8 = VALUE + 4;
    emu.regs_mut().r9 = VALUE + 5;
    map_bytes(&mut emu, STACK, &(VALUE + 6).to_le_bytes(), false);
    for index in 0..7 {
        assert_eq!(UnixApi.arg(&emu, index), Ok(VALUE + index as u64));
    }
    assert_eq!(WinApi(Stdcall).arg(&emu, 0), Ok(VALUE + 3));
    assert_eq!(
        WinApi(Stdcall).arg(&emu, 4).unwrap_err().fault,
        ArgumentFault::Unreadable
    );
    assert_eq!(
        UnixApi.arg(&emu, usize::MAX).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
    assert_eq!(emu.sp(), STACK);
}

#[test]
fn unsupported_conventions_do_not_guess_another_layout() {
    let mut arm = emu_aarch64();
    arm.regs_aarch64_mut().x[7] = VALUE;
    assert_eq!(UnixApi.arg(&arm, 7), Ok(VALUE));
    for result in [
        UnixApi.arg(&arm, 8).map_err(|e| e.fault),
        WinSyscall.arg(&arm, 0).map_err(|e| e.fault),
    ] {
        assert_eq!(result.unwrap_err(), ArgumentFault::UnsupportedConvention);
    }
    let x86 = emu32();
    for fault in [
        UnixApi.arg(&x86, 0).unwrap_err().fault,
        WinSyscall.arg(&x86, 0).unwrap_err().fault,
    ] {
        assert_eq!(fault, ArgumentFault::UnsupportedConvention);
    }
}

#[test]
fn typed_returns_preserve_other_registers_and_the_caller_stack() {
    let mut x86 = emu32();
    x86.set_sp(STACK);
    x86.regs_mut().rbx = VALUE;
    WinApi(Stdcall).set_return_u64(&mut x86, VALUE);
    assert_eq!(x86.regs().rax, VALUE as u32 as u64);
    assert_eq!(x86.regs().rdx, VALUE >> u32::BITS);
    WinApi(Stdcall).set_return_f64(&mut x86, 1.25);
    assert_eq!(x86.fpu_mut().peek_st_logical_f64(0), 1.25);
    assert_eq!(x86.regs().rbx, VALUE);
    assert_eq!(x86.sp(), STACK);

    let mut x64 = emu64();
    x64.set_sp(STACK);
    x64.regs_mut().rdx = VALUE + 1;
    WinApi(Stdcall).set_return_u64(&mut x64, VALUE);
    assert_eq!(x64.regs().rax, VALUE);
    assert_eq!(x64.regs().rdx, VALUE + 1);
    WinApi(Stdcall).set_return_f64(&mut x64, 2.5);
    assert_eq!(x64.regs().xmm0, 2.5f64.to_bits() as u128);
    assert_eq!(x64.sp(), STACK);

    let mut arm = emu_aarch64();
    UnixApi.set_return(&mut arm, VALUE);
    WinApi(Stdcall).set_return_f64(&mut arm, 3.75);
    assert_eq!(arm.regs_aarch64().x[0], VALUE);
    assert_eq!(arm.regs_aarch64().v[0], 3.75f64.to_bits() as u128);
}

#[test]
fn narrow_arguments_keep_the_abi_slot_stride_and_signedness() {
    const NEGATIVE: u64 = 0x7654_3210_ffff_ff80;
    let mut x86 = emu32();
    x86.set_sp(STACK);
    map_bytes(&mut x86, STACK + DWORD_BYTES, &NEGATIVE.to_le_bytes(), true);
    assert_eq!(WinApi(Stdcall).arg_u8(&x86, 1), Ok(128));
    assert_eq!(WinApi(Stdcall).arg_i8(&x86, 1), Ok(-128));
    assert_eq!(WinApi(Stdcall).arg_u16(&x86, 1), Ok(65408));
    assert_eq!(WinApi(Stdcall).arg_i16(&x86, 1), Ok(-128));
    assert_eq!(WinApi(Cdecl).arg_i32(&x86, 1), Ok(-128));
    assert_eq!(WinApi(Cdecl).arg_i64(&x86, 1), Ok(NEGATIVE as i64));
    assert_eq!(x86.sp(), STACK);

    let mut x64 = emu64();
    x64.set_sp(STACK);
    x64.regs_mut().rcx = NEGATIVE;
    x64.regs_mut().rdi = NEGATIVE;
    assert_eq!(WinApi(Stdcall).arg_i8(&x64, 0), Ok(-128));
    assert_eq!(WinApi(Stdcall).arg_i16(&x64, 0), Ok(-128));
    assert_eq!(WinApi(Stdcall).arg_i32(&x64, 0), Ok(-128));
    assert_eq!(UnixApi.arg_i8(&x64, 0), Ok(-128));
    assert_eq!(UnixApi.arg_i16(&x64, 0), Ok(-128));
    assert_eq!(UnixApi.arg_i32(&x64, 0), Ok(-128));
    assert_eq!(UnixApi.arg_i64(&x64, 0), Ok(NEGATIVE as i64));
    map_bytes(&mut x64, STACK, &NEGATIVE.to_le_bytes(), false);
    map_bytes(&mut x64, STACK + 5 * QWORD_BYTES, &[128], false);
    assert_eq!(WinApi(Stdcall).arg_u8(&x64, 5), Ok(128));
    assert_eq!(UnixApi.arg_u8(&x64, 11), Ok(128));
    assert_eq!(UnixApi.arg_u16(&x64, 6), Ok(65408));
    assert_eq!(UnixApi.arg_u32(&x64, 6), Ok(NEGATIVE as u32));
    assert_eq!(UnixApi.arg_u64(&x64, 6), Ok(NEGATIVE));
    assert_eq!(
        WinApi(Stdcall).arg_u16(&x64, 5).unwrap_err().fault,
        ArgumentFault::Unreadable
    );
    assert_eq!(
        UnixApi.arg_u16(&x64, 11).unwrap_err().fault,
        ArgumentFault::Unreadable
    );
    assert_eq!(x64.sp(), STACK);
}

#[test]
fn x86_narrow_arguments_require_the_promoted_dword() {
    let mut emu = emu32();
    emu.set_sp(STACK);
    map_bytes(&mut emu, STACK, &[128, 255], false);
    for fault in [
        WinApi(Stdcall).arg_u8(&emu, 0).unwrap_err(),
        WinApi(Cdecl).arg_i16(&emu, 0).unwrap_err(),
    ] {
        assert_eq!(fault.address, Some(STACK + 2));
        assert_eq!(fault.fault, ArgumentFault::Unreadable);
        assert_eq!(fault.size, DWORD_BYTES as usize);
    }
}

#[test]
fn completion_obeys_convention_and_does_not_read_discarded_slots() {
    let mut x86 = emu32();
    x86.cfg.trace_mem = true;
    for (call, delta) in [
        (WinApi(Stdcall), 2 * DWORD_BYTES),
        (WinApi(Thiscall), 2 * DWORD_BYTES),
        (WinApi(Cdecl), 0),
    ] {
        x86.set_sp(STACK);
        x86.regs_mut().rax = VALUE;
        call.cleanup_args(&mut x86, 2);
        assert_eq!(x86.sp(), STACK + delta, "{call:?}");
        assert_eq!(x86.regs().rax, VALUE);
    }
    x86.set_sp(u32::MAX as u64 - 3);
    WinApi(Stdcall).cleanup_args(&mut x86, 2);
    assert_eq!(x86.sp(), DWORD_BYTES);

    for mut emu in [emu64(), emu_aarch64()] {
        emu.set_sp(STACK);
        for call in [WinApi(Stdcall), WinApi(Cdecl), WinApi(Thiscall), WinSyscall] {
            call.cleanup_args(&mut emu, 10);
            assert_eq!(emu.sp(), STACK, "{call:?}");
        }
    }
}

#[test]
fn thiscall_keeps_the_implicit_object_out_of_x86_stack_slots() {
    let mut x86 = emu32();
    x86.set_sp(STACK);
    x86.regs_mut().rcx = VALUE;
    map_bytes(&mut x86, STACK, &(-2i64).to_le_bytes(), false);
    assert_eq!(WinApi(Thiscall).arg(&x86, 0), Ok(VALUE as u32 as u64));
    assert_eq!(WinApi(Thiscall).arg_i64(&x86, 1), Ok(-2));
    assert_eq!(
        WinApi(Thiscall).arg_u64(&x86, 0).unwrap_err().fault,
        ArgumentFault::UnsupportedConvention
    );
    assert_eq!(
        WinApi(Thiscall).arg_f64(&x86, 0, 0).unwrap_err().fault,
        ArgumentFault::UnsupportedConvention
    );
    WinApi(Thiscall).cleanup_args(&mut x86, 2);
    assert_eq!(x86.sp(), STACK + QWORD_BYTES);

    let mut x64 = emu64();
    x64.regs_mut().rcx = VALUE;
    x64.regs_mut().rdx = (-2i64) as u64;
    assert_eq!(WinApi(Thiscall).arg(&x64, 0), Ok(VALUE));
    assert_eq!(WinApi(Thiscall).arg_i64(&x64, 1), Ok(-2));
}

#[test]
fn unix_integer_varargs_follow_platform_register_and_stack_layouts() {
    let call = UnixApi;
    let mut arm = emu_aarch64();
    arm.set_sp(STACK);
    arm.regs_aarch64_mut().x[1] = VALUE;
    map_bytes(&mut arm, STACK, &0u64.to_le_bytes(), true);
    assert_eq!(
        call.integer_vararg(&arm, 1, 0, VariadicConvention::SystemV)
            .unwrap(),
        VALUE
    );
    assert_eq!(
        call.integer_vararg(&arm, 1, 7, VariadicConvention::SystemV)
            .unwrap(),
        0
    );
    assert_eq!(
        call.integer_vararg(&arm, 1, 0, VariadicConvention::Darwin)
            .unwrap(),
        0
    );
    for convention in [VariadicConvention::SystemV, VariadicConvention::Darwin] {
        let error = call
            .integer_vararg(&arm, usize::MAX, 1, convention)
            .unwrap_err();
        assert_eq!(error.fault, ArgumentFault::AddressOverflow);
        let mut x64 = emu64();
        x64.set_sp(STACK);
        x64.regs_mut().r9 = VALUE;
        map_bytes(&mut x64, STACK, &0u64.to_le_bytes(), true);
        assert_eq!(call.integer_vararg(&x64, 1, 4, convention).unwrap(), VALUE);
        assert_eq!(call.integer_vararg(&x64, 1, 5, convention).unwrap(), 0);
        let error = call.integer_vararg(&x64, 1, 6, convention).unwrap_err();
        assert_eq!(error.fault, ArgumentFault::Unreadable);
        assert_eq!(error.index, 7);
    }
}

#[test]
fn sysenter_arguments_use_edx_and_leave_both_return_addresses() {
    let mut emu = emu32();
    let call = WinSysenter;
    emu.set_sp(STACK + 0x1000);
    emu.regs_mut().rdx = STACK;
    map_bytes(&mut emu, STACK, &[0xcc; 2 * DWORD_BYTES as usize], false);
    map_bytes(
        &mut emu,
        STACK + 2 * DWORD_BYTES,
        &VALUE.to_le_bytes(),
        true,
    );
    assert_eq!(call.arg_u32(&emu, 0), Ok(VALUE as u32));
    assert_eq!(call.arg_u64(&emu, 0), Ok(VALUE));
    let stack = emu.sp();
    call.cleanup_args(&mut emu, 2);
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.maps.read_dword(STACK), Some(0xcccccccc));
    emu.regs_mut().rdx = u32::MAX as u64;
    assert_eq!(
        call.arg(&emu, 0).unwrap_err().fault,
        ArgumentFault::AddressOverflow
    );
    assert_eq!(
        WinInterrupt.arg(&emu, 0).unwrap_err().fault,
        ArgumentFault::UnsupportedConvention
    );
    assert_eq!(
        call.arg(&emu64(), 0).unwrap_err().fault,
        ArgumentFault::UnsupportedConvention
    );
}
