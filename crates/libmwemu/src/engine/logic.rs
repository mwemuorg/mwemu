use crate::emu::Emu;
use crate::exception::types;

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
    let value1: i32 = emu.regs().get_ax() as i32;
    let value2: i32 = value0 as i32;
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
    emu.regs_mut().rax = resq as u64;
    emu.regs_mut().rdx = resr as u64;
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_of = resq > 0xffffffffffffffff;
    if emu.flag_of() {
        log::trace!("/!\\ int overflow on division");
    }
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
    emu.regs_mut().set_eax(resq);
    emu.regs_mut().set_edx(resr);
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_of = resq > 0xffffffff;
    if emu.flag_of() {
        log::trace!("/!\\ int overflow on division");
    }
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
    emu.regs_mut().set_ax(resq.into());
    emu.regs_mut().set_dx(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_of = resq > 0xffff;
    emu.flags_mut().f_tf = false;
    if emu.flag_of() {
        log::trace!("/!\\ int overflow on division");
    }
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
    emu.regs_mut().set_al(resq.into());
    emu.regs_mut().set_ah(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_of = resq > 0xff;
    emu.flags_mut().f_tf = false;
    if emu.flag_of() {
        log::trace!("/!\\ int overflow");
    }
}

pub fn idiv64(emu: &mut Emu, value0: u64) {
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
    emu.regs_mut().rax = resq as u64;
    emu.regs_mut().rdx = resr as u64;
    emu.flags_mut().calc_pf(resq as u8);
    if resq > 0xffffffffffffffff {
        log::trace!("/!\\ int overflow exception on division");
        if emu.break_on_alert {
            panic!();
        }
    } else if ((value1 as i128) > 0 && (resq as i64) < 0)
        || ((value1 as i128) < 0 && (resq as i64) > 0)
    {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
    }
}

pub fn idiv32(emu: &mut Emu, value0: u64) {
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
    emu.regs_mut().set_eax(resq);
    emu.regs_mut().set_edx(resr);
    emu.flags_mut().calc_pf(resq as u8);
    if resq > 0xffffffff {
        log::trace!("/!\\ int overflow exception on division");
        if emu.break_on_alert {
            panic!();
        }
    } else if ((value1 as i64) > 0 && (resq as i32) < 0)
        || ((value1 as i64) < 0 && (resq as i32) > 0)
    {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
    }
}

pub fn idiv16(emu: &mut Emu, value0: u64) {
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
    emu.regs_mut().set_ax(resq.into());
    emu.regs_mut().set_dx(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_tf = false;
    if resq > 0xffff {
        log::trace!("/!\\ int overflow exception on division");
        if emu.break_on_alert {
            panic!();
        }
    } else if ((value1 as i32) > 0 && (resq as i16) < 0)
        || ((value1 as i32) < 0 && (resq as i16) > 0)
    {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
    }
}

pub fn idiv8(emu: &mut Emu, value0: u64) {
    let value1: u32 = to32!(emu.regs().get_ax());
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
    emu.regs_mut().set_al(resq.into());
    emu.regs_mut().set_ah(resr.into());
    emu.flags_mut().calc_pf(resq as u8);
    emu.flags_mut().f_tf = false;
    if resq > 0xff {
        log::trace!("/!\\ int overflow exception on division");
        if emu.break_on_alert {
            panic!();
        }
    } else if ((value1 as i16) > 0 && (resq as i8) < 0) || ((value1 as i16) < 0 && (resq as i8) > 0)
    {
        log::trace!("/!\\ sign change exception on division");
        emu.exception(types::ExceptionType::SignChangeOnDivision);
        emu.force_break = true;
    }
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
