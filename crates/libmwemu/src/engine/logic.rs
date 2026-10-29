use crate::emu::Emu;
use crate::exception::types;
use iced_x86::{Code, Instruction, OpKind};

pub fn rol(emu: &mut Emu, val: u64, rot2: u64, bits: u32) -> u64 {
    let mut ret: u64 = val;

    let rot = if bits == 64 {
        rot2 & 0b111111
    } else {
        rot2 & 0b11111
    };

    for _ in 0..rot {
        let last_bit = get_bit!(ret, bits - 1);
        //log::trace!("last bit: {}", last_bit);
        let mut ret2: u64 = ret;

        //  For the ROL and ROR instructions, the original value of the CF flag is not a part of the result, but the CF flag receives a copy of the bit that was shifted from one end to the other.
        emu.flags_mut().f_cf = last_bit == 1;

        for j in 0..bits - 1 {
            let bit = get_bit!(ret, j);
            set_bit!(ret2, j + 1, bit);
        }

        set_bit!(ret2, 0, last_bit);
        ret = ret2;
        //log::trace!("{:b}", ret);
    }

    ret
}

pub fn rcl_bit_based(emu: &Emu, val: u64, rot2: u64, bits: u32) -> u64 {
    let mut ret: u128 = val as u128;

    let rot = if bits == 64 {
        rot2 & 0b111111
    } else {
        rot2 & 0b11111
    };

    if emu.flag_cf() {
        set_bit!(ret, bits, 1);
    } else {
        set_bit!(ret, bits, 0);
    }

    for _ in 0..rot {
        let last_bit = get_bit!(ret, bits);
        //log::trace!("last bit: {}", last_bit);
        let mut ret2: u128 = ret;

        for j in 0..bits {
            let bit = get_bit!(ret, j);
            set_bit!(ret2, j + 1, bit);
        }

        set_bit!(ret2, 0, last_bit);
        ret = ret2;
        //log::trace!("{:b}", ret);
    }

    let a: u128 = 2;
    (ret & (a.pow(bits) - 1)) as u64
}

pub fn ror(emu: &mut Emu, val: u64, rot2: u64, bits: u32) -> u64 {
    let mut ret: u64 = val;

    let rot = if bits == 64 {
        rot2 & 0b111111
    } else {
        rot2 & 0b11111
    };

    for _ in 0..rot {
        let first_bit = get_bit!(ret, 0);
        let mut ret2: u64 = ret;

        //  For the ROL and ROR instructions, the original value of the CF flag is not a part of the result, but the CF flag receives a copy of the bit that was shifted from one end to the other.
        emu.flags_mut().f_cf = first_bit == 1;

        for j in (1..bits).rev() {
            let bit = get_bit!(ret, j);
            set_bit!(ret2, j - 1, bit);
        }

        set_bit!(ret2, bits - 1, first_bit);
        ret = ret2;
    }

    ret
}

pub fn rcr(emu: &mut Emu, val: u64, rot2: u64, bits: u32) -> u64 {
    let mut ret: u128 = val as u128;

    let rot = if bits == 64 {
        rot2 & 0b111111
    } else {
        rot2 & 0b11111
    };

    if emu.flag_cf() {
        set_bit!(ret, bits, 1);
    } else {
        set_bit!(ret, bits, 0);
    }

    for _ in 0..rot {
        let first_bit = get_bit!(ret, 0);
        let mut ret2: u128 = ret;

        for j in (1..=bits).rev() {
            let bit = get_bit!(ret, j);
            set_bit!(ret2, j - 1, bit);
        }

        set_bit!(ret2, bits, first_bit);
        ret = ret2;
    }

    let cnt = rot2 % (bits + 1) as u64;
    if cnt == 1 {
        emu.flags_mut().f_cf = (val & 0x1) == 1;
    } else {
        emu.flags_mut().f_cf = ((val >> (cnt - 1)) & 0x1) == 1;
    }

    let a: u128 = 2;
    (ret & (a.pow(bits) - 1)) as u64
}

pub fn mul64(emu: &mut Emu, value0: u64) {
    let value1: u64 = emu.regs().rax;
    let value2: u64 = value0;
    let res: u128 = value1 as u128 * value2 as u128;
    emu.regs_mut().rdx = ((res & 0xffffffffffffffff0000000000000000) >> 64) as u64;
    emu.regs_mut().rax = (res & 0xffffffffffffffff) as u64;
    emu.flags_mut().calc_pf(res as u8);
    emu.flags_mut().f_of = emu.regs().rdx != 0;
    emu.flags_mut().f_cf = emu.regs().rdx != 0;
}

pub fn mul32(emu: &mut Emu, value0: u64) {
    let value1: u32 = to32!(emu.regs().get_eax());
    let value2: u32 = value0 as u32;
    let res: u64 = value1 as u64 * value2 as u64;
    emu.regs_mut().set_edx((res & 0xffffffff00000000) >> 32);
    emu.regs_mut().set_eax(res & 0x00000000ffffffff);
    emu.flags_mut().calc_pf(res as u8);
    emu.flags_mut().f_of = emu.regs().get_edx() != 0;
    emu.flags_mut().f_cf = emu.regs().get_edx() != 0;
}

pub fn mul16(emu: &mut Emu, value0: u64) {
    let value1: u32 = to32!(emu.regs().get_ax());
    let value2: u32 = value0 as u32;
    let res: u32 = value1 * value2;
    emu.regs_mut().set_dx(((res & 0xffff0000) >> 16).into());
    emu.regs_mut().set_ax((res & 0xffff).into());
    emu.flags_mut().calc_pf(res as u8);
    emu.flags_mut().f_of = emu.regs().get_dx() != 0;
    emu.flags_mut().f_cf = emu.regs().get_dx() != 0;
}

pub fn mul8(emu: &mut Emu, value0: u64) {
    let value1: u32 = emu.regs().get_al() as u32;
    let value2: u32 = value0 as u32;
    let res: u32 = value1 * value2;
    emu.regs_mut().set_ax((res & 0xffff).into());
    emu.flags_mut().calc_pf(res as u8);
    emu.flags_mut().f_of = emu.regs().get_ah() != 0;
    emu.flags_mut().f_cf = emu.regs().get_ah() != 0;
}

pub fn imul64p1(emu: &mut Emu, value0: u64) {
    let value1: i64 = emu.regs().rax as i64;
    let value2: i64 = value0 as i64;
    let res: i128 = value1 as i128 * value2 as i128;
    let ures: u128 = res as u128;
    emu.regs_mut().rdx = (ures >> 64) as u64;
    emu.regs_mut().rax = (ures & 0xffffffffffffffff) as u64;
    let rax = emu.regs().rax as i64;
    let rdx = emu.regs().rdx as i64;
    emu.flags_mut().f_of = !((rax < 0 && rdx == -1) || (rax >= 0 && rdx == 0));
    emu.flags_mut().f_cf = emu.flag_of();
}

pub fn imul32p1(emu: &mut Emu, value0: u64) {
    let value1: i32 = emu.regs().get_eax() as i32;
    let value2: i32 = value0 as i32;
    let res: i64 = value1 as i64 * value2 as i64;
    let ures: u64 = res as u64;
    emu.regs_mut().set_edx((ures & 0xffffffff00000000) >> 32);
    emu.regs_mut().set_eax(ures & 0x00000000ffffffff);
    let eax = emu.regs().get_eax() as i32;
    let edx = emu.regs().get_edx() as i32;
    emu.flags_mut().f_of = !((eax < 0 && edx == -1) || (eax >= 0 && edx == 0));
    emu.flags_mut().f_cf = emu.flag_of();
}

pub fn imul16p1(emu: &mut Emu, value0: u64) {
    // The operands arrive as u64: go through u16/i16 so they keep their sign.
    let value1: i32 = emu.regs().get_ax() as u16 as i16 as i32;
    let value2: i32 = value0 as u16 as i16 as i32;
    let res: i32 = value1 * value2;
    let ures: u32 = res as u32;
    emu.regs_mut().set_dx(((ures & 0xffff0000) >> 16).into());
    emu.regs_mut().set_ax((ures & 0xffff).into());
    let ax = emu.regs().get_ax() as i16;
    let dx = emu.regs().get_dx() as i16;
    emu.flags_mut().f_of = !((ax < 0 && dx == -1) || (ax >= 0 && dx == 0));
    emu.flags_mut().f_cf = emu.flag_of();
}

pub fn imul8p1(emu: &mut Emu, value0: u64) {
    let value1: i8 = emu.regs().get_al() as i8;
    let value2: i8 = value0 as i8;
    let res: i16 = value1 as i16 * value2 as i16;
    emu.regs_mut().set_ax(res as u16 as u64);

    let ax = emu.regs().get_ax() as u16;
    let al = (ax & 0x00FF) as i8 as i16;
    let ah = ((ax >> 8) & 0x00FF) as i8 as i16;

    emu.flags_mut().f_of = !((al < 0 && ah == -1) || (al >= 0 && ah == 0));
    emu.flags_mut().f_cf = emu.flag_of();
}

/// #DE for a DIV whose quotient does not fit the destination register. Like the
/// division by zero it is raised before any register is written.
fn divide_overflow(emu: &mut Emu) {
    log::trace!("/!\\ quotient overflow on division");
    emu.exception(types::ExceptionType::Div0);
    emu.force_break = true;
}

pub fn div64(emu: &mut Emu, value0: u64) {
    let mut value1: u128 = emu.regs().rdx as u128;
    value1 <<= 64;
    value1 += emu.regs().rax as u128;
    let value2: u128 = value0 as u128;

    if value2 == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let resq: u128 = value1 / value2;
    let resr: u128 = value1 % value2;
    if resq > u64::MAX as u128 {
        divide_overflow(emu);
        return;
    }
    emu.regs_mut().rax = resq as u64;
    emu.regs_mut().rdx = resr as u64;
    emu.flags_mut().calc_pf(resq as u8);
}

pub fn div32(emu: &mut Emu, value0: u64) {
    let mut value1: u64 = emu.regs().get_edx();
    value1 <<= 32;
    value1 += emu.regs().get_eax();
    let value2: u64 = value0;

    if value2 == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let resq: u64 = value1 / value2;
    let resr: u64 = value1 % value2;
    if resq > u32::MAX as u64 {
        divide_overflow(emu);
        return;
    }
    emu.regs_mut().set_eax(resq);
    emu.regs_mut().set_edx(resr);
    emu.flags_mut().calc_pf(resq as u8);
}

pub fn div16(emu: &mut Emu, value0: u64) {
    let value1: u32 = to32!((emu.regs().get_dx() << 16) + emu.regs().get_ax());
    let value2: u32 = value0 as u32;

    if value2 == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let resq: u32 = value1 / value2;
    let resr: u32 = value1 % value2;
    if resq > u16::MAX as u32 {
        divide_overflow(emu);
        return;
    }
    emu.regs_mut().set_ax(resq.into());
    emu.regs_mut().set_dx(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_tf = false;
}

pub fn div8(emu: &mut Emu, value0: u64) {
    let value1: u32 = emu.regs().get_ax() as u32;
    let value2: u32 = value0 as u32;
    if value2 == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let resq: u32 = value1 / value2;
    let resr: u32 = value1 % value2;
    if resq > u8::MAX as u32 {
        divide_overflow(emu);
        return;
    }
    emu.regs_mut().set_al(resq.into());
    emu.regs_mut().set_ah(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_tf = false;
}

pub fn idiv64(emu: &mut Emu, value0: u64) {
    // IDIV is signed: RDX:RAX / src64 -> RAX = quotient, RDX = remainder.
    let divisor = (value0 as i64) as i128;
    if divisor == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let dividend = (((emu.regs().rdx as u128) << 64) | (emu.regs().rax as u128)) as i128;
    // checked_div also traps i128::MIN / -1, which would otherwise panic.
    let (resq, resr) = match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
        (Some(q), Some(r)) if q <= i64::MAX as i128 && q >= i64::MIN as i128 => (q, r),
        _ => {
            log::trace!("/!\\ sign change exception on division");
            emu.exception(types::ExceptionType::SignChangeOnDivision);
            emu.force_break = true;
            return;
        }
    };

    emu.regs_mut().rax = resq as u64;
    emu.regs_mut().rdx = resr as u64;
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_tf = false;
}

pub fn idiv32(emu: &mut Emu, value0: u64) {
    // IDIV is signed: EDX:EAX / src32 -> EAX = quotient, EDX = remainder.
    let divisor = ((value0 as u32) as i32) as i128;
    if divisor == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let hi = emu.regs().get_edx() & 0xffffffff;
    let lo = emu.regs().get_eax() & 0xffffffff;
    // Widen to i128 so EDX:EAX = i64::MIN divided by -1 cannot overflow.
    let dividend = (((hi << 32) | lo) as i64) as i128;
    let resq = dividend / divisor;
    let resr = dividend % divisor;
    emu.flags_mut().f_tf = false;

    if resq > i32::MAX as i128 || resq < i32::MIN as i128 {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
        return;
    }

    emu.regs_mut().set_eax((resq as u32) as u64);
    emu.regs_mut().set_edx((resr as u32) as u64);
    emu.flags_mut().calc_pf(resq as u8);
}

pub fn idiv16(emu: &mut Emu, value0: u64) {
    // IDIV is signed: DX:AX / src16 -> AX = quotient, DX = remainder.
    let divisor = ((value0 as u16) as i16) as i64;
    if divisor == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    let hi = (emu.regs().get_dx() as u16) as u32;
    let lo = (emu.regs().get_ax() as u16) as u32;
    // Widen to i64 so DX:AX = i32::MIN divided by -1 cannot overflow.
    let dividend = (((hi << 16) | lo) as i32) as i64;
    let resq = dividend / divisor;
    let resr = dividend % divisor;
    emu.flags_mut().f_tf = false;

    if resq > i16::MAX as i64 || resq < i16::MIN as i64 {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
        return;
    }

    emu.regs_mut().set_ax((resq as u16) as u64);
    emu.regs_mut().set_dx((resr as u16) as u64);
    emu.flags_mut().calc_pf(resq as u8);
}

pub fn idiv8(emu: &mut Emu, value0: u64) {
    // IDIV is signed: AX / src8 -> AL = quotient, AH = remainder.
    let divisor = ((value0 as u8) as i8) as i32;
    if divisor == 0 {
        emu.flags_mut().f_tf = true;
        log::trace!("/!\\ division by 0 exception");
        emu.exception(types::ExceptionType::Div0);
        emu.force_break = true;
        return;
    }

    // Widen to i32 so AX=i16::MIN divided by -1 cannot overflow the division.
    let dividend = ((emu.regs().get_ax() as u16) as i16) as i32;
    let resq = dividend / divisor;
    let resr = dividend % divisor;
    emu.flags_mut().f_tf = false;

    // #DE when the signed quotient does not fit in a byte.
    if resq > i8::MAX as i32 || resq < i8::MIN as i32 {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
        return;
    }

    emu.regs_mut().set_al((resq as u8) as u64);
    emu.regs_mut().set_ah((resr as u8) as u64);
    emu.flags_mut().calc_pf(resq as u8);
}

pub fn shrd(emu: &mut Emu, value0: u64, value1: u64, pcounter: u64, size: u32) -> (u64, bool) {
    let mut storage0: u64 = value0;
    let mut counter: u64 = pcounter;

    /*if size == 64 {
        counter = counter % 64;
    } else {
        counter = counter % 32;
    }*/

    match size {
        64 => counter %= 64,
        32 => counter %= 32,
        _ => {}
    }

    if counter == 0 {
        return (storage0, false);
    }

    if counter >= size as u64 {
        if emu.cfg.verbose >= 1 {
            log::trace!(
                "/!\\ SHRD undefined behaviour value0 = 0x{:x} value1 = 0x{:x} pcounter = 0x{:x} counter = 0x{:x} size = 0x{:x}",
                value0,
                value1,
                pcounter,
                counter,
                size
            );
        }
        let result = 0; //inline::shrd(value0, value1, pcounter, size);
        emu.flags_overwrite_mut().calc_flags(result, size);
        return (result, true);
    }

    emu.flags_mut().f_cf = get_bit!(value0, counter - 1) == 1;

    let mut to = size as u64 - 1 - counter;
    if to > 64 {
        // log::trace!("to: {}", to);
        to = 64;
    }

    for i in 0..=to {
        let bit = get_bit!(storage0, i as u32 + counter as u32);
        set_bit!(storage0, i as u32, bit);
    }

    let from = size as u64 - counter;

    //log::trace!("from: {}", from);

    for i in from..size as u64 {
        let bit = get_bit!(value1, i as u32 + counter as u32 - size);
        set_bit!(storage0, i as u32, bit);
    }

    /*
    for i in 0..=(size as u64 -1 -counter) {
       let bit = get_bit!(storage0, i+counter);
       set_bit!(storage0, i, bit);
    }
    for i in (size as u64 -counter)..(size as u64) {
        let bit = get_bit!(storage0, i+counter-size as u64);
        set_bit!(storage0, i, bit);
    }*/

    emu.flags_overwrite_mut().calc_flags(storage0, size);
    (storage0, false)
}

pub fn shld(emu: &mut Emu, value0: u64, value1: u64, pcounter: u64, size: u32) -> (u64, bool) {
    let mut storage0: u64 = value0;
    let mut counter: u64 = pcounter;

    if size == 64 {
        counter %= 64;
    } else {
        counter %= 32;
    }

    if counter == 0 {
        return (value0, false);
    }

    /*
    if counter >= size as u64 {
        counter = size as u64 -1;
    }*/

    if counter > size as u64 {
        if emu.cfg.verbose >= 1 {
            log::trace!("/!\\ undefined behaviour on shld");
        }

        let result = 0;
        //let result = inline::shld(value0, value1, pcounter, size);
        emu.flags_overwrite_mut().calc_flags(result, size);

        return (result, true);
        //counter = pcounter - size as u64;
    }

    emu.flags_mut().f_cf = get_bit!(value0, size as u64 - counter) == 1;
    /*
    if counter < size as u64 && size - (counter as u8) < 64 {
        emu.flags_mut().f_cf = get_bit!(value0, size - counter as u8) == 1;
    }*/

    for i in (counter..=((size as u64) - 1)).rev() {
        let bit = get_bit!(storage0, i - counter);
        set_bit!(storage0, i, bit);
    }

    for i in (0..counter).rev() {
        let bit = get_bit!(value1, i + (size as u64) - counter);
        set_bit!(storage0, i, bit);
    }

    emu.flags_overwrite_mut().calc_flags(storage0, size);

    (storage0, false)
}

/// Width of the LOOP/LOOPcc counter. The address-size prefix selects CX, ECX
/// or RCX, and iced bakes that choice into the `Code` variant.
pub fn loop_counter_bits(ins: &Instruction) -> u32 {
    use Code::*;
    match ins.code() {
        Loop_rel8_16_CX | Loop_rel8_32_CX | Loope_rel8_16_CX | Loope_rel8_32_CX
        | Loopne_rel8_16_CX | Loopne_rel8_32_CX => 16,
        Loop_rel8_16_ECX | Loop_rel8_32_ECX | Loop_rel8_64_ECX | Loope_rel8_16_ECX
        | Loope_rel8_32_ECX | Loope_rel8_64_ECX | Loopne_rel8_16_ECX | Loopne_rel8_32_ECX
        | Loopne_rel8_64_ECX => 32,
        _ => 64,
    }
}

/// Decrement the LOOP counter at `bits` width and return the new value.
pub fn loop_decrement(emu: &mut Emu, bits: u32) -> u64 {
    let regs = emu.regs_mut();
    match bits {
        16 => {
            let v = (regs.get_cx() as u16).wrapping_sub(1) as u64;
            regs.set_cx(v);
            v
        }
        32 => {
            let v = (regs.get_ecx() as u32).wrapping_sub(1) as u64;
            regs.set_ecx(v);
            v
        }
        _ => {
            regs.rcx = regs.rcx.wrapping_sub(1);
            regs.rcx
        }
    }
}

/// Take a LOOP/LOOPcc branch.
pub fn loop_jump(emu: &mut Emu, addr: u64) -> bool {
    if emu.cfg.is_x64() {
        emu.set_rip_with_check(addr, false)
    } else {
        emu.set_eip(addr, false)
    }
}

/// Target of a BT/BTS/BTR/BTC: the operand-sized element that holds the
/// selected bit, the bit index inside it and, for a memory base, its address.
pub struct BitTarget {
    pub value: u64,
    pub bit: u32,
    pub addr: Option<u64>,
    pub size: u32,
}

impl BitTarget {
    pub fn selected_bit(&self) -> bool {
        (self.value >> self.bit) & 1 == 1
    }
}

/// Resolve the bit addressed by operand 0 (bit base) and operand 1 (bit offset).
/// A register offset on a memory base is a signed element index: the element at
/// `base + (offset div size) * bytes` is accessed (SDM "Bit(BitBase, BitOffset)"),
/// so offsets outside the first element reach the neighbouring memory. Register
/// bases and immediate offsets use the offset modulo the operand size.
pub fn bit_target(emu: &mut Emu, ins: &Instruction) -> Option<BitTarget> {
    let size = emu.get_operand_sz(ins, 0);
    let offset = emu.get_operand_value(ins, 1, true)?;
    let bit = (offset % size as u64) as u32;
    if ins.op_kind(0) != OpKind::Memory || ins.op_kind(1) != OpKind::Register {
        let value = emu.get_operand_value(ins, 0, true)?;
        return Some(BitTarget {
            value,
            bit,
            addr: None,
            size,
        });
    }
    let base = emu.get_operand_value(ins, 0, false)?;
    let element = sign_extend(offset, size) >> size.trailing_zeros();
    let addr = base.wrapping_add((element * (size / 8) as i64) as u64);
    let value = read_sized(emu, addr, size)?;
    Some(BitTarget {
        value,
        bit,
        addr: Some(addr),
        size,
    })
}

/// Write a BTS/BTR/BTC result back to wherever `bit_target` read it from.
pub fn bit_target_store(emu: &mut Emu, ins: &Instruction, target: &BitTarget, value: u64) -> bool {
    match target.addr {
        Some(addr) => write_sized(emu, addr, target.size, value),
        None => emu.set_operand_value(ins, 0, value),
    }
}

fn sign_extend(value: u64, bits: u32) -> i64 {
    match bits {
        16 => value as u16 as i16 as i64,
        32 => value as u32 as i32 as i64,
        _ => value as i64,
    }
}

fn read_sized(emu: &Emu, addr: u64, bits: u32) -> Option<u64> {
    match bits {
        16 => emu.maps.read_word(addr).map(u64::from),
        32 => emu.maps.read_dword(addr).map(u64::from),
        _ => emu.maps.read_qword(addr),
    }
}

fn write_sized(emu: &mut Emu, addr: u64, bits: u32, value: u64) -> bool {
    match bits {
        16 => emu.maps.write_word(addr, value as u16),
        32 => emu.maps.write_dword(addr, value as u32),
        _ => emu.maps.write_qword(addr, value),
    }
}
