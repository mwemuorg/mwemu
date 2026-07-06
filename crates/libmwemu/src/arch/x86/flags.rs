use serde::{Deserialize, Serialize};

pub const MIN_I8: i8 = -128;
pub const MAX_I8: i8 = 0x7f;
pub const MIN_U8: u8 = 0;
pub const MAX_U8: u8 = 0xff;

pub const MIN_I16: i16 = -32768;
pub const MAX_I16: i16 = 0x7fff;
pub const MIN_U16: u16 = 0;
pub const MAX_U16: u16 = 0xffff;

pub const MIN_I32: i32 = -2147483648;
pub const MAX_I32: i32 = 0x7fffffff;
pub const MIN_U32: u32 = 0;
pub const MAX_U32: u32 = 0xffffffff;

pub const MIN_I64: i64 = -9223372036854775808;
pub const MAX_I64: i64 = 0x7fffffffffffffff;
pub const MIN_U64: u64 = 0;
pub const MAX_U64: u64 = 0xffffffffffffffff;

// instead of table we generate the table at compile time to make sure it is correct
// the parity table calculate true if the number of zero-bit in the lsb 8-bit is even and false otherwise.
const fn build_parity_table() -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = i.count_ones() % 2 == 0;
        i += 1;
    }
    table
}

pub const PARITY_LOOKUP_TABLE: [bool; 256] = build_parity_table();

macro_rules! get_bit {
    ($val:expr, $count:expr) => {
        ($val & (1 << $count)) >> $count
    };
}

macro_rules! set_bit {
    ($val:expr, $count:expr, $bit:expr) => {
        if $bit == 1 {
            $val |= 1 << $count;
        } else {
            $val &= !(1 << $count);
        }
    };
}

pub const F_CF: u32 = 0;
pub const F_PF: u32 = 2;
pub const F_AF: u32 = 4;
pub const F_ZF: u32 = 6;
pub const F_SF: u32 = 7;
pub const F_TF: u32 = 8;
pub const F_IF: u32 = 9;
pub const F_DF: u32 = 10;
pub const F_OF: u32 = 11;
pub const F_IOPL1: u32 = 12;
pub const F_IOPL2: u32 = 13;
pub const F_NT: u32 = 14;
pub const F_RF: u32 = 16;
pub const F_VM: u32 = 17;
pub const F_AC: u32 = 18;
pub const F_VIF: u32 = 19;
pub const F_VIP: u32 = 20;
pub const F_ID: u32 = 21;

// auxbits bit layout (all bits within a single u32):
//   Bit 31 (LF_BIT_CF):    carry flag = carry_out from MSB of the operation
//   Bit 30 (LF_BIT_PO):    CF xor OF  = carry_out from bit (width-2)
//   Bits 15:8 (LF_BIT_PDB): PF delta byte for parity correction
//   Bit 3  (LF_BIT_AF):    auxiliary carry = carry_out from bit 3
//   Bit 0  (LF_BIT_SD):    sign delta for SF correction
const LF_BIT_SD: u32 = 0;
const LF_BIT_AF: u32 = 3;
const LF_BIT_PDB: u32 = 8;
const LF_BIT_PO: u32 = 30;
const LF_BIT_CF: u32 = 31;

const LF_MASK_SD: u32 = 1 << LF_BIT_SD;
const LF_MASK_AF: u32 = 1 << LF_BIT_AF;
const LF_MASK_PDB: u32 = 0xff << LF_BIT_PDB;
const LF_MASK_CF: u32 = 1 << LF_BIT_CF;
const LF_MASK_PO: u32 = 1 << LF_BIT_PO;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum LazyFlagsKind {
    None,
    Oszapc,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct LazyFlags {
    kind: LazyFlagsKind,
    result: u64,
    auxbits: u32,
}

impl Default for LazyFlags {
    fn default() -> Self {
        Self {
            kind: LazyFlagsKind::None,
            result: 0,
            auxbits: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Flags {
    pub f_cf: bool,
    pub f_pf: bool,
    pub f_af: bool,
    pub f_zf: bool,
    pub f_sf: bool,
    pub f_tf: bool,
    pub f_if: bool,
    pub f_df: bool,
    pub f_of: bool,
    pub f_iopl1: bool,
    pub f_iopl2: bool,
    pub f_nt: bool,
    pub f_rf: bool,
    pub f_vm: bool,
    pub f_ac: bool,
    pub f_vif: bool,
    pub f_vip: bool,
    pub f_id: bool,
    lazy: LazyFlags,
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            f_cf: false,
            f_pf: false,
            f_af: false,
            f_zf: false,
            f_sf: false,
            f_tf: false,
            f_if: false,
            f_df: false,
            f_of: false,
            f_iopl1: false,
            f_iopl2: false,
            f_nt: false,
            f_rf: false,
            f_vm: false,
            f_ac: false,
            f_vif: false,
            f_vip: false,
            f_id: false,
            lazy: LazyFlags::default(),
        }
    }

    #[inline]
    pub fn clear_lazy(&mut self) {
        self.lazy = LazyFlags::default();
    }

    /// Sign-extends an N-bit result to 64 bits so that SF can be read
    /// from bit 63 with a simple shift.
    #[inline(always)]
    fn sign_extend_result(result: u64, bits: u32) -> u64 {
        match bits {
            64 => result,
            32 => result as u32 as i32 as i64 as u64,
            16 => result as u16 as i16 as i64 as u64,
            8 => result as u8 as i8 as i64 as u64,
            _ => unreachable!("weird size"),
        }
    }

    #[inline(always)]
    fn add_cout_vec(op1: u64, op2: u64, result: u64) -> u64 {
        (op1 & op2) | ((op1 | op2) & !result)
    }

    #[inline(always)]
    fn sub_cout_vec(op1: u64, op2: u64, result: u64) -> u64 {
        (!op1 & op2) | ((!op1 ^ op2) & result)
    }

    #[inline(always)]
    fn set_lazy_oszapc(&mut self, result: u64, carries: u64, bits: u32) {
        match bits {
            64 => self.set_lazy_oszapc64(result, carries),
            32 => self.set_lazy_oszapc32(result as u32, carries as u32),
            16 => self.set_lazy_oszapc16(result as u16, carries as u32),
            8 => self.set_lazy_oszapc8(result as u8, carries as u32),
            _ => unreachable!("weird size"),
        }
    }

    #[inline(always)]
    fn set_lazy_oszapc64(&mut self, result: u64, carries: u64) {
        let auxbits = ((carries & LF_MASK_AF as u64) | ((carries >> 62) << LF_BIT_PO)) as u32;
        self.cache_oszapc_fields(result, auxbits);
        self.lazy = LazyFlags {
            kind: LazyFlagsKind::Oszapc,
            result,
            auxbits,
        };
    }

    #[inline(always)]
    fn set_lazy_oszapc32(&mut self, result: u32, carries: u32) {
        let auxbits = carries & !(LF_MASK_PDB | LF_MASK_SD);
        let result = result as i32 as i64 as u64;
        self.cache_oszapc_fields(result, auxbits);
        self.lazy = LazyFlags {
            kind: LazyFlagsKind::Oszapc,
            result,
            auxbits,
        };
    }

    #[inline(always)]
    fn set_lazy_oszapc16(&mut self, result: u16, carries: u32) {
        let auxbits = (carries & LF_MASK_AF) | (carries << 16);
        let result = result as i16 as i64 as u64;
        self.cache_oszapc_fields(result, auxbits);
        self.lazy = LazyFlags {
            kind: LazyFlagsKind::Oszapc,
            result,
            auxbits,
        };
    }

    #[inline(always)]
    fn set_lazy_oszapc8(&mut self, result: u8, carries: u32) {
        let auxbits = (carries & LF_MASK_AF) | (carries << 24);
        let result = result as i8 as i64 as u64;
        self.cache_oszapc_fields(result, auxbits);
        self.lazy = LazyFlags {
            kind: LazyFlagsKind::Oszapc,
            result,
            auxbits,
        };
    }

    #[inline(always)]
    fn set_lazy_logic(&mut self, result: u64, bits: u32) {
        match bits {
            64 => self.set_lazy_oszapc64(result, 0),
            32 => self.set_lazy_oszapc32(result as u32, 0),
            16 => self.set_lazy_oszapc16(result as u16, 0),
            8 => self.set_lazy_oszapc8(result as u8, 0),
            _ => unreachable!("weird size"),
        }
    }

    #[inline]
    fn set_lazy_from_flags(&mut self, result: u64, cf: bool, of: bool, af: bool, bits: u32) {
        self.set_lazy_from_encoded_flags(result, cf, of, af, bits);
    }

    #[inline]
    fn set_lazy_from_encoded_flags(
        &mut self,
        result: u64,
        cf: bool,
        of: bool,
        af: bool,
        bits: u32,
    ) {
        let mut auxbits = 0_u32;
        if af {
            auxbits |= LF_MASK_AF;
        }
        if cf {
            auxbits |= LF_MASK_CF;
        }
        if cf ^ of {
            auxbits |= LF_MASK_PO;
        }

        self.lazy = LazyFlags {
            kind: LazyFlagsKind::Oszapc,
            result: Self::sign_extend_result(result, bits),
            auxbits,
        };
        self.cache_oszapc_fields(self.lazy.result, auxbits);
    }

    /// Eagerly resolves CF, ZF, SF, OF from the sign-extended result and the
    /// packed auxbits. PF and AF remain lazy (computed on-demand from auxbits).
    ///
    /// OF = CF xor (carry into MSB) = auxbits[31] xor auxbits[30].
    /// Adding LF_MASK_PO (1<<30) to auxbits and extracting bit 31 yields
    /// exactly this XOR: the add creates a carry from position 30 to 31
    /// iff bit 30 was set, flipping bit 31.
    #[inline(always)]
    fn cache_oszapc_fields(&mut self, result: u64, auxbits: u32) {
        self.f_cf = (auxbits & LF_MASK_CF) != 0;
        self.f_zf = result == 0;
        self.f_sf = (((result >> 63) as u32 ^ (auxbits >> LF_BIT_SD)) & 1) != 0;
        self.f_of = ((auxbits.wrapping_add(LF_MASK_PO) >> LF_BIT_CF) & 1) != 0;
    }

    #[inline]
    fn mask(bits: u32) -> u64 {
        match bits {
            64 => u64::MAX,
            32 => 0xffff_ffff,
            16 => 0xffff,
            8 => 0xff,
            _ => unreachable!("weird size"),
        }
    }

    #[inline]
    fn calc_pf_value(final_value: u8) -> bool {
        PARITY_LOOKUP_TABLE[final_value as usize]
    }

    /// Branchless parity check using a 16-bit lookup constant.
    /// `0x9669` has bit N set iff popcount(N) is even.
    /// `(value ^ (value >> 4)) & 0x0f` collapses the byte into a nibble
    /// whose popcount parity equals the original byte's popcount parity.
    #[inline]
    fn calc_lazy_pf_value(final_value: u8) -> bool {
        let temp = (final_value ^ (final_value >> 4)) & 0x0f;
        ((0x9669_u16 >> temp) & 1) != 0
    }

    #[inline]
    fn calc_af_value(value1: u64, value2: u64, result: u64) -> bool {
        ((value1 ^ value2 ^ result) & 0x10) != 0
    }

    #[inline(always)]
    fn sign_mask(bits: u32) -> u64 {
        1u64 << (bits - 1)
    }

    #[inline(always)]
    pub fn cf(&self) -> bool {
        self.f_cf
    }

    #[inline(always)]
    pub fn pf(&self) -> bool {
        match self.lazy.kind {
            LazyFlagsKind::None => self.f_pf,
            LazyFlagsKind::Oszapc => Self::calc_lazy_pf_value(
                ((self.lazy.result as u32 ^ (self.lazy.auxbits >> LF_BIT_PDB)) & 0xff) as u8,
            ),
        }
    }

    #[inline(always)]
    pub fn af(&self) -> bool {
        match self.lazy.kind {
            LazyFlagsKind::None => self.f_af,
            LazyFlagsKind::Oszapc => (self.lazy.auxbits & LF_MASK_AF) != 0,
        }
    }

    #[inline(always)]
    pub fn zf(&self) -> bool {
        self.f_zf
    }

    #[inline(always)]
    pub fn sf(&self) -> bool {
        self.f_sf
    }

    #[inline(always)]
    pub fn of(&self) -> bool {
        self.f_of
    }

    #[inline]
    pub fn materialize_lazy(&mut self) {
        if matches!(self.lazy.kind, LazyFlagsKind::None) {
            return;
        }
        self.f_cf = self.cf();
        self.f_pf = self.pf();
        self.f_af = self.af();
        self.f_zf = self.zf();
        self.f_sf = self.sf();
        self.f_of = self.of();
        self.clear_lazy();
    }

    pub fn clear(&mut self) {
        self.clear_lazy();
        self.f_cf = false;
        self.f_pf = false;
        self.f_af = false;
        self.f_zf = false;
        self.f_sf = false;
        self.f_tf = false;
        self.f_if = false;
        self.f_df = false;
        self.f_of = false;
        self.f_iopl1 = false;
        self.f_iopl2 = false;
        self.f_nt = false;
        self.f_rf = false;
        self.f_vm = false;
        self.f_ac = false;
        self.f_vif = false;
        self.f_vip = false;
        self.f_id = false;
    }

    pub fn set(&mut self) {
        self.clear_lazy();
        self.f_cf = true;
        self.f_pf = true;
        self.f_af = true;
        self.f_zf = true;
        self.f_sf = true;
        self.f_tf = true;
        self.f_if = true;
        self.f_df = true;
        self.f_of = true;
        self.f_iopl1 = true;
        self.f_iopl2 = true;
        self.f_nt = true;
        self.f_rf = true;
        self.f_vm = true;
        self.f_ac = true;
        self.f_vif = true;
        self.f_vip = true;
        self.f_id = true;
    }

    pub fn print_trace(&self, pos: u64) {
        let mut flags = *self;
        flags.materialize_lazy();
        let mut fs = String::new();
        fs.push_str("[ ");
        if flags.f_cf {
            fs.push_str("CF ");
        }
        if flags.f_pf {
            fs.push_str("PF ");
        }
        if flags.f_af {
            fs.push_str("AF ");
        }
        if flags.f_zf {
            fs.push_str("ZF ");
        }
        if flags.f_sf {
            fs.push_str("SF ");
        }
        if flags.f_tf {
            fs.push_str("TF ");
        }
        if flags.f_if {
            fs.push_str("IF ");
        }
        if flags.f_df {
            fs.push_str("DF ");
        }
        if flags.f_of {
            fs.push_str("OF ");
        }
        fs.push(']');
        log::trace!("\t{} flags: 0x{:x} {}", pos, flags.dump(), fs);
    }

    pub fn print(&self) {
        let mut flags = *self;
        flags.materialize_lazy();
        log::trace!("--- flags ---");
        log::trace!("0x{:x}", flags.dump());
        log::trace!("cf: {}", flags.f_cf);
        log::trace!("pf: {}", flags.f_pf);
        log::trace!("af: {}", flags.f_af);
        log::trace!("zf: {}", flags.f_zf);
        log::trace!("sf: {}", flags.f_sf);
        log::trace!("tf: {}", flags.f_tf);
        log::trace!("if: {}", flags.f_if);
        log::trace!("df: {}", flags.f_df);
        log::trace!("of: {}", flags.f_of);
        log::trace!("iopl1: {}", flags.f_iopl1);
        log::trace!("iopl2: {}", flags.f_iopl2);
        log::trace!("nt: {}", flags.f_nt);
        log::trace!("rf: {}", flags.f_rf);
        log::trace!("vm: {}", flags.f_vm);
        log::trace!("ac: {}", flags.f_ac);
        log::trace!("vif: {}", flags.f_vif);
        log::trace!("vip: {}", flags.f_vip);
        log::trace!("id: {}", flags.f_id);
        log::trace!("---");
    }

    pub fn diff(a: &Flags, b: &Flags) -> String {
        let mut a = *a;
        let mut b = *b;
        a.materialize_lazy();
        b.materialize_lazy();
        let mut output = String::new();
        // f_cf
        if a.f_cf != b.f_cf {
            output = format!("{}{}: {} -> {} ", output, "cf", a.f_cf, b.f_cf);
        }
        // f_pf
        if a.f_pf != b.f_pf {
            output = format!("{}{}: {} -> {} ", output, "pf", a.f_pf, b.f_pf);
        }
        // f_af
        if a.f_af != b.f_af {
            output = format!("{}{}: {} -> {} ", output, "af", a.f_af, b.f_af);
        }
        // f_zf
        if a.f_zf != b.f_zf {
            output = format!("{}{}: {} -> {} ", output, "zf", a.f_zf, b.f_zf);
        }
        // f_sf
        if a.f_sf != b.f_sf {
            output = format!("{}{}: {} -> {} ", output, "sf", a.f_sf, b.f_sf);
        }
        // f_tf
        if a.f_tf != b.f_tf {
            output = format!("{}{}: {} -> {} ", output, "tf", a.f_tf, b.f_tf);
        }
        // f_if
        if a.f_if != b.f_if {
            output = format!("{}{}: {} -> {} ", output, "if", a.f_if, b.f_if);
        }
        // f_df
        if a.f_df != b.f_df {
            output = format!("{}{}: {} -> {} ", output, "df", a.f_df, b.f_df);
        }
        // f_of
        if a.f_of != b.f_of {
            output = format!("{}{}: {} -> {} ", output, "of", a.f_of, b.f_of);
        }
        // f_iopl1
        if a.f_iopl1 != b.f_iopl1 {
            output = format!("{}{}: {} -> {} ", output, "iopl1", a.f_iopl1, b.f_iopl1);
        }
        // f_iopl2
        if a.f_iopl2 != b.f_iopl2 {
            output = format!("{}{}: {} -> {} ", output, "iopl2", a.f_iopl2, b.f_iopl2);
        }
        // f_nt
        if a.f_nt != b.f_nt {
            output = format!("{}{}: {} -> {} ", output, "nt", a.f_nt, b.f_nt);
        }
        // f_rf
        if a.f_rf != b.f_rf {
            output = format!("{}{}: {} -> {} ", output, "rf", a.f_rf, b.f_rf);
        }
        // f_vm
        if a.f_vm != b.f_vm {
            output = format!("{}{}: {} -> {} ", output, "vm", a.f_vm, b.f_vm);
        }
        // f_ac
        if a.f_ac != b.f_ac {
            output = format!("{}{}: {} -> {} ", output, "ac", a.f_ac, b.f_ac);
        }
        // f_vif
        if a.f_vif != b.f_vif {
            output = format!("{}{}: {} -> {} ", output, "vif", a.f_vif, b.f_vif);
        }
        // f_vip
        if a.f_vip != b.f_vip {
            output = format!("{}{}: {} -> {} ", output, "vip", a.f_vip, b.f_vip);
        }
        // f_id
        if a.f_id != b.f_id {
            output = format!("{}{}: {} -> {} ", output, "id", a.f_id, b.f_id);
        }
        output
    }

    pub fn dump(mut self) -> u32 {
        self.materialize_lazy();
        let mut flags: u32 = 0;

        if self.f_cf {
            set_bit!(flags, 0, 1);
        }
        set_bit!(flags, 1, 1); // always 1 in EFLAGS
        if self.f_pf {
            set_bit!(flags, 2, 1);
        }
        // 3 is reserved
        if self.f_af {
            set_bit!(flags, 4, 1);
        }
        // 5 is reserved
        if self.f_zf {
            set_bit!(flags, 6, 1);
        }
        if self.f_sf {
            set_bit!(flags, 7, 1);
        }
        if self.f_tf {
            set_bit!(flags, 8, 1);
        }
        if self.f_if {
            set_bit!(flags, 9, 1);
        }
        if self.f_df {
            set_bit!(flags, 10, 1);
        }
        if self.f_of {
            set_bit!(flags, 11, 1);
        }

        if self.f_iopl1 {
            set_bit!(flags, 12, 1);
        }
        if self.f_iopl2 {
            set_bit!(flags, 13, 1);
        }

        if self.f_nt {
            set_bit!(flags, 14, 1);
        }
        set_bit!(flags, 15, 0);
        if self.f_rf {
            set_bit!(flags, 16, 1);
        }
        if self.f_vm {
            set_bit!(flags, 17, 1);
        }
        if self.f_ac {
            set_bit!(flags, 18, 1);
        }
        if self.f_vif {
            set_bit!(flags, 19, 1);
        }
        if self.f_vip {
            set_bit!(flags, 20, 1);
        }
        if self.f_id {
            set_bit!(flags, 21, 1);
        }

        flags
    }

    pub fn load(&mut self, flags: u32) {
        self.clear_lazy();
        self.f_cf = get_bit!(flags, 0) == 1;
        self.f_pf = get_bit!(flags, 2) == 1;
        self.f_af = get_bit!(flags, 4) == 1;
        self.f_zf = get_bit!(flags, 6) == 1;
        self.f_sf = get_bit!(flags, 7) == 1;
        self.f_tf = get_bit!(flags, 8) == 1;
        self.f_if = get_bit!(flags, 9) == 1;
        self.f_df = get_bit!(flags, 10) == 1;
        self.f_of = get_bit!(flags, 11) == 1;
        self.f_iopl1 = get_bit!(flags, 12) == 1;
        self.f_iopl2 = get_bit!(flags, 13) == 1;
        self.f_nt = get_bit!(flags, 14) == 1;
        self.f_rf = get_bit!(flags, 16) == 1;
        self.f_vm = get_bit!(flags, 17) == 1;
        self.f_ac = get_bit!(flags, 18) == 1;
        self.f_vif = get_bit!(flags, 19) == 1;
        self.f_vip = get_bit!(flags, 20) == 1;
        self.f_id = get_bit!(flags, 21) == 1;
    }

    pub fn calc_flags(&mut self, final_value: u64, bits: u32) {
        self.f_tf = false;
        self.set_lazy_from_flags(final_value, self.f_cf, self.f_of, self.f_af, bits);
    }

    pub fn calc_logic_flags_lazy(&mut self, final_value: u64, bits: u32) {
        self.f_tf = false;
        self.f_cf = false;
        self.f_af = false;
        self.f_of = false;
        self.set_lazy_logic(final_value, bits);
    }

    #[inline]
    pub fn calc_pf(&mut self, final_value: u8) {
        self.materialize_lazy();
        self.f_pf = Self::calc_pf_value(final_value);
    }

    #[inline]
    pub fn calc_af(&mut self, value1: u64, value2: u64, result: u64, _bits: u64) {
        self.materialize_lazy();
        self.f_af = Self::calc_af_value(value1, value2, result);
        //self.f_af = (value1 & 0x0f) + (value2 & 0x0f) > 0x09;
    }

    #[inline(always)]
    pub fn add64(&mut self, value1: u64, value2: u64, cf: bool, include_carry: bool) -> u64 {
        let v1 = value1;
        let v2 = value2;
        let c = if include_carry { cf as u64 } else { 0 };
        let result = v1.wrapping_add(v2).wrapping_add(c);
        self.set_lazy_oszapc64(result, Self::add_cout_vec(v1, v2, result));
        result
    }

    #[inline(always)]
    pub fn add32(&mut self, value1: u32, value2: u32, cf: bool, include_carry: bool) -> u64 {
        let c = if include_carry { cf as u32 } else { 0 };
        let result = value1.wrapping_add(value2).wrapping_add(c);
        let carries = Self::add_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc32(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn add16(&mut self, value1: u16, value2: u16, cf: bool, include_carry: bool) -> u64 {
        let c = if include_carry { cf as u16 } else { 0 };
        let result = value1.wrapping_add(value2).wrapping_add(c);
        let carries = Self::add_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc16(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn add8(&mut self, value1: u8, value2: u8, cf: bool, include_carry: bool) -> u64 {
        let c = if include_carry { cf as u8 } else { 0 };
        let result = value1.wrapping_add(value2).wrapping_add(c);
        let carries = Self::add_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc8(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn sub64_borrow(&mut self, value1: u64, value2: u64, borrow: bool) -> u64 {
        let borrow = borrow as u64;
        let result = value1.wrapping_sub(value2).wrapping_sub(borrow);
        self.set_lazy_oszapc64(result, Self::sub_cout_vec(value1, value2, result));
        result
    }

    #[inline(always)]
    pub fn sub32_borrow(&mut self, value1: u64, value2: u64, borrow: bool) -> u64 {
        let value1 = value1 as u32;
        let value2 = value2 as u32;
        let borrow = borrow as u32;
        let result = value1.wrapping_sub(value2).wrapping_sub(borrow);
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc32(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn sub16_borrow(&mut self, value1: u64, value2: u64, borrow: bool) -> u64 {
        let value1 = value1 as u16;
        let value2 = value2 as u16;
        let borrow = borrow as u16;
        let result = value1.wrapping_sub(value2).wrapping_sub(borrow);
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc16(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn sub8_borrow(&mut self, value1: u64, value2: u64, borrow: bool) -> u64 {
        let value1 = value1 as u8;
        let value2 = value2 as u8;
        let borrow = borrow as u8;
        let result = value1.wrapping_sub(value2).wrapping_sub(borrow);
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, result as u64);
        self.set_lazy_oszapc8(result, carries as u32);
        result as u64
    }

    #[inline(always)]
    pub fn sub64(&mut self, value1: u64, value2: u64) -> u64 {
        let r = value1.wrapping_sub(value2);
        self.set_lazy_oszapc64(r, Self::sub_cout_vec(value1, value2, r));
        r
    }

    #[inline(always)]
    pub fn sub32(&mut self, value1: u64, value2: u64) -> u64 {
        let r = (value1 as u32).wrapping_sub(value2 as u32);

        let value1 = value1 as u32;
        let value2 = value2 as u32;
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, r as u64);
        self.set_lazy_oszapc32(r, carries as u32);

        r as u64
    }

    #[inline(always)]
    pub fn sub16(&mut self, value1: u64, value2: u64) -> u64 {
        let r = (value1 as u16).wrapping_sub(value2 as u16);

        let value1 = value1 as u16;
        let value2 = value2 as u16;
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, r as u64);
        self.set_lazy_oszapc16(r, carries as u32);

        r as u64
    }

    #[inline(always)]
    pub fn sub8(&mut self, value1: u64, value2: u64) -> u64 {
        let r = (value1 as u8).wrapping_sub(value2 as u8);

        let value1 = value1 as u8;
        let value2 = value2 as u8;
        let carries = Self::sub_cout_vec(value1 as u64, value2 as u64, r as u64);
        self.set_lazy_oszapc8(r, carries as u32);

        r as u64
    }

    pub fn inc64(&mut self, value: u64) -> u64 {
        let result = value.wrapping_add(1);
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0xf;
        self.f_of = value == 0x7fff_ffff_ffff_ffff;
        self.f_zf = result == 0;
        self.f_sf = (result as i64) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn inc32(&mut self, value: u64) -> u64 {
        let value = value & 0xffff_ffff;
        let result = value.wrapping_add(1) & 0xffff_ffff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0xf;
        self.f_of = value == 0x7fff_ffff;
        self.f_zf = result == 0;
        self.f_sf = (result as u32 as i32) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn inc16(&mut self, value: u64) -> u64 {
        let value = value & 0xffff;
        let result = value.wrapping_add(1) & 0xffff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0xf;
        self.f_of = value == 0x7fff;
        self.f_zf = result == 0;
        self.f_sf = (result as u16 as i16) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn inc8(&mut self, value: u64) -> u64 {
        let value = value & 0xff;
        let result = value.wrapping_add(1) & 0xff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0xf;
        self.f_of = value == 0x7f;
        self.f_zf = result == 0;
        self.f_sf = (result as u8 as i8) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn dec64(&mut self, value: u64) -> u64 {
        let result = value.wrapping_sub(1);
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0;
        self.f_of = value == 0x8000_0000_0000_0000;
        self.f_zf = result == 0;
        self.f_sf = (result as i64) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn dec32(&mut self, value: u64) -> u64 {
        let value = value & 0xffff_ffff;
        let result = value.wrapping_sub(1) & 0xffff_ffff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0;
        self.f_of = value == 0x8000_0000;
        self.f_zf = result == 0;
        self.f_sf = (result as u32 as i32) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn dec16(&mut self, value: u64) -> u64 {
        let value = value & 0xffff;
        let result = value.wrapping_sub(1) & 0xffff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0;
        self.f_of = value == 0x8000;
        self.f_zf = result == 0;
        self.f_sf = (result as u16 as i16) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn dec8(&mut self, value: u64) -> u64 {
        let value = value & 0xff;
        let result = value.wrapping_sub(1) & 0xff;
        let cf = self.cf();
        self.lazy.kind = LazyFlagsKind::None;
        self.f_cf = cf;
        self.f_af = (value & 0xf) == 0;
        self.f_of = value == 0x80;
        self.f_zf = result == 0;
        self.f_sf = (result as u8 as i8) < 0;
        self.f_pf = Self::calc_pf_value(result as u8);
        result
    }

    pub fn neg64(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x8000000000000000;
        self.f_cf = true;

        let mut ival = value as i64;
        if ival != i64::MIN {
            ival = -ival;
        }

        let res = ival as u64;

        self.calc_flags(res, 64);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg32(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x80000000;
        self.f_cf = true;

        let mut ival = value as i32;
        if ival != i32::MIN {
            ival = -ival;
        }

        let res = ival as u32 as u64;

        self.calc_flags(res, 32);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg16(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x8000;
        self.f_cf = true;

        let mut ival = value as i16;
        if ival != i16::MIN {
            ival = -ival;
        }

        let res = ival as u16 as u64;

        self.calc_flags(res, 16);
        self.calc_pf(res as u8);
        res
    }

    pub fn neg8(&mut self, value: u64) -> u64 {
        self.f_of = value == 0x80;
        self.f_cf = true;

        let mut ival = value as i8;
        if ival != i8::MIN {
            ival = -ival;
        }

        let res = ival as u8 as u64;

        self.calc_flags(res, 8);
        self.calc_pf(res as u8);
        res
    }

    //// sal sar signed ////

    pub fn sal2p64(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p64(value0, value1)
    }

    pub fn sal2p32(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p32(value0, value1)
    }

    pub fn sal2p16(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p16(value0, value1)
    }

    pub fn sal2p8(&mut self, value0: u64, value1: u64) -> u64 {
        self.shl2p8(value0, value1)
    }

    pub fn sal1p64(&mut self, value: u64) -> u64 {
        self.shl1p64(value)
    }

    pub fn sal1p32(&mut self, value: u64) -> u64 {
        self.shl1p32(value)
    }

    pub fn sal1p16(&mut self, value: u64) -> u64 {
        self.shl1p16(value)
    }

    pub fn sal1p8(&mut self, value: u64) -> u64 {
        self.shl1p8(value)
    }

    pub fn sar2p64(&mut self, value0: u64, value1: u64) -> u64 {
        let s64 = value0 as i64;
        let count = (value1 & 0x3f) as u32;

        if count == 0 {
            return value0;
        }

        let s_result = if count < 64 {
            s64 >> count
        } else {
            if s64 < 0 { -1 } else { 0 }
        };

        let result = s_result as u64;

        self.f_cf = if count <= 64 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = false;
        self.calc_flags(result, 64);
        result
    }

    pub fn sar2p64_bug(&mut self, value0: u64, value1: u64) -> u64 {
        let s64: i64 = value0 as i64;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let sResult = s64 >> count;
        let result = sResult as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 64);
        result
    }

    pub fn sar2p32(&mut self, value0: u64, value1: u64) -> u64 {
        let s32 = value0 as u32 as i32;
        let count = (value1 & 0x1f) as u32;

        if count == 0 {
            return value0;
        }

        let s_result = if count < 32 {
            s32 >> count
        } else {
            if s32 < 0 { -1 } else { 0 }
        };

        let result = s_result as u32 as u64;

        self.f_cf = if count <= 32 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = false;
        self.calc_flags(result, 32);
        result
    }

    pub fn sar2p32_bug(&mut self, value0: u64, value1: u64) -> u64 {
        let s32: i32 = value0 as u32 as i32;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s32 >> count;
        let result = sResult as u32 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 32);
        result
    }

    pub fn sar2p16(&mut self, value0: u64, value1: u64) -> u64 {
        let s16 = value0 as u16 as i16;
        let count = (value1 & 0x1f) as u32;

        if count == 0 {
            return value0;
        }

        let s_result = if count < 16 {
            s16 >> count
        } else {
            if s16 < 0 { -1 } else { 0 }
        };

        let result = s_result as u16 as u64;

        self.f_cf = if count <= 16 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = false;
        self.calc_flags(result, 16);
        result
    }

    pub fn sar2p16_bug(&mut self, value0: u64, value1: u64) -> u64 {
        let s16 = value0 as u16 as i16;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s16 >> count;
        let result = sResult as u16 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 16);
        result
    }

    pub fn sar2p8(&mut self, value0: u64, value1: u64) -> u64 {
        let s8: i8 = value0 as u8 as i8;
        let count = (value1 & 0x1f) as u32;

        if count == 0 {
            return value0;
        }

        // avoids UB on shift >= 8
        let s_result = if count < 8 {
            s8 >> count
        } else {
            // - 0x00 if number was positive
            // - 0xFF if it was negative (sign extend)
            if s8 < 0 { -1 } else { 0 }
        };

        let result = s_result as u8 as u64;

        self.f_cf = if count <= 8 {
            ((value0 >> (count - 1)) & 0x1) == 0x1
        } else {
            false
        };

        self.f_of = false;
        self.calc_flags(result, 8);
        result
    }

    pub fn sar2p8_bug(&mut self, value0: u64, value1: u64) -> u64 {
        let s8: i8 = value0 as u8 as i8;
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let sResult = s8 >> count; // attempt to shift right with overflow
        let result = sResult as u8 as u64;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 8);
        result
    }

    pub fn sar1p64(&mut self, value: u64) -> u64 {
        let s64 = value as i64;

        let sResult = s64 >> 1;
        let result = sResult as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 64);
        result
    }

    pub fn sar1p32(&mut self, value: u64) -> u64 {
        let s32 = value as u32 as i32;
        let sResult = s32 >> 1;
        let result = sResult as u32 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 32);
        result
    }

    pub fn sar1p16(&mut self, value: u64) -> u64 {
        let s16 = value as u16 as i16;
        let sResult = s16 >> 1;
        let result = sResult as u16 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 16);
        result
    }

    pub fn sar1p8(&mut self, value: u64) -> u64 {
        let s16 = value as u8 as i16;
        let sResult = s16 >> 1;
        let result = sResult as u8 as u64;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = false;
        self.calc_flags(result, 8);
        result
    }

    //// shr shl unsigned ////

    pub fn shl2p8(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xff;
        }

        let result = (value0 << count) & 0xff ;

        self.f_cf = if count <= 8 {
            ((value0 >> (8 - count)) & 0x1) == 0x1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((result >> 7) & 0x1) != ((value0 >> 7) & 0x1)
        } else {
            false
        };

        self.calc_flags(result, 8);
        result
    }

    pub fn shl2p16(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xffff;
        }

        let result = (value0 << count) & 0xffff ;

        self.f_cf = if count <= 16 {
            ((value0 >> (16 - count)) & 0x1) == 0x1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((result >> 15) & 0x1) != ((value0 >> 15) & 0x1)
        } else {
            false
        };

        self.calc_flags(result, 16);
        result
    }

    pub fn shl2p32(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xffff_ffff;
        }

        let result = (value0 << count) & 0xffff_ffff ;

        self.f_cf = if count <= 32 {
            ((value0 >> (32 - count)) & 0x1) == 0x1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((result >> 31) & 0x1) != ((value0 >> 31) & 0x1)
        } else {
            false
        };

        self.calc_flags(result, 32);
        result
    }

    pub fn shl2p64(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x3f) as u32;
        if count == 0 {
            return value0;
        }

        let result = value0 << count;

        self.f_cf = if count <= 64 {
            ((value0 >> (64 - count)) & 0x1) == 0x1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((result >> 63) & 0x1) != ((value0 >> 63) & 0x1)
        } else {
            false
        };

        self.calc_flags(result, 64);
        result
    }

    pub fn shl2p64_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let result = value0 << count ;
        self.f_cf = ((value0 >> (64 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 63)) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shl2p32_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xffffffff;
        self.f_cf = ((value0 >> (32 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 31)) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shl2p16_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xffff;
        self.f_cf = ((value0 >> (16 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 15)) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shl2p8_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 << count) & 0xff;
        self.f_cf = ((value0 >> (8 - count)) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 7)) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    // TODO: update shl1 the same as shl2
    pub fn shl1p64(&mut self, value: u64) -> u64 {
        let result = value << 1 ;
        self.f_cf = ((value >> 63) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 63)) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shl1p32(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xffffffff;
        self.f_cf = ((value >> 32) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 32)) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shl1p16(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xffff;
        self.f_cf = ((value >> 16) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 16)) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shl1p8(&mut self, value: u64) -> u64 {
        let result = (value << 1) & 0xff;
        self.f_cf = ((value >> 8) & 0x1) == 0x1;
        self.f_of = (self.f_cf as u64 ^ (result >> 8)) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn shr2p8(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xff;
        }

        let result = (value0 >> count) & 0xff ;

        self.f_cf = if count <= 8 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((value0 >> 7) & 1) == 1
        } else {
            false
        };

        self.calc_flags(result, 8);
        result
    }

    pub fn shr2p16(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xffff;
        }

        let result = (value0 >> count) & 0xffff ;

        self.f_cf = if count <= 16 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((value0 >> 15) & 1) == 1
        } else {
            false
        };

        self.calc_flags(result, 16);
        result
    }

    pub fn shr2p32(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x1f) as u32;
        if count == 0 {
            return value0 & 0xffff_ffff;
        }

        let result = (value0 >> count) & 0xffff_ffff ;

        self.f_cf = if count <= 32 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((value0 >> 31) & 1) == 1
        } else {
            false
        };

        self.calc_flags(result, 32);
        result
    }

    pub fn shr2p64(&mut self, value0: u64, value1: u64) -> u64 {
        let count = (value1 & 0x3f) as u32;
        if count == 0 {
            return value0;
        }

        let result = value0 >> count;

        self.f_cf = if count <= 64 {
            ((value0 >> (count - 1)) & 0x1) == 1
        } else {
            false
        };

        self.f_of = if count == 1 {
            ((value0 >> 63) & 1) == 1
        } else {
            false
        };

        self.calc_flags(result, 64);
        result
    }

    pub fn shr2p64_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x3f;
        let result = value0 >> count ;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 63 & 0x1) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shr2p32_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xffffffff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 31 & 0x1) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shr2p16_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xffff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 15 & 0x1) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shr2p8_overflow(&mut self, value0: u64, value1: u64) -> u64 {
        if value1 == 0 {
            return value0;
        }

        let count = value1 & 0x1f;
        let result = (value0 >> count) & 0xff;
        self.f_cf = ((value0 >> (count - 1)) & 0x1) == 0x1;
        self.f_of = ((((result << 1) ^ result) >> 7) & 0x1) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn shr1p64(&mut self, value: u64) -> u64 {
        let result = value >> 1 ;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 63) == 0x1;
        self.calc_flags(result, 64);
        result
    }

    pub fn shr1p32(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xffffffff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 31) == 0x1;
        self.calc_flags(result, 32);
        result
    }

    pub fn shr1p16(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xffff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 15) == 0x1;
        self.calc_flags(result, 16);
        result
    }

    pub fn shr1p8(&mut self, value: u64) -> u64 {
        let result = (value >> 1) & 0xff;
        self.f_cf = (value & 0x1) == 0x1;
        self.f_of = (((result << 1) ^ result) >> 7) == 0x1;
        self.calc_flags(result, 8);
        result
    }

    pub fn test(&mut self, value0: u64, value1: u64, sz: u32) {
        let result: u64 = value0 & value1;

        self.calc_logic_flags_lazy(result, sz);
        //undefined behavior: self.calc_af(value0, value1, result as u64, sz as u64);
    }

    //// imul ////
    pub fn imul64p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i128 = value0 as i64 as i128 * value1 as i64 as i128;
        let uresult: u128 = result as u128;

        if uresult > 0xffffffffffffffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        let res: u64 = (uresult & 0xffffffffffffffff) as u64;
        res
    }

    pub fn imul32p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i64 = value0 as i32 as i64 * value1 as i32 as i64;
        let uresult: u64 = result as u64;

        if uresult > 0xffffffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        let res: u64 = uresult & 0xffffffff;
        res
    }

    pub fn imul16p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i32 = value0 as i16 as i32 * value1 as i16 as i32;
        let uresult: u32 = result as u32;

        if uresult > 0xffff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        
        (uresult & 0xffff) as u64
    }

    pub fn imul8p2(&mut self, value0: u64, value1: u64) -> u64 {
        let result: i16 = value0 as i8 as i16 * value1 as i8 as i16;
        let uresult: u16 = result as u16;

        if uresult > 0xff {
            self.f_cf = true;
            self.f_of = true;
        } else {
            self.f_cf = false;
            self.f_of = false;
        }

        
        (uresult & 0xff) as u64
    }

    pub fn rcr_of_and_cf(&mut self, value0: u64, value1: u64, sz: u32) {
        let count = value1 & 0x3f;
        let res = if count == 1 {
            (value0 >> count) | ((self.f_cf as u64) << (sz - 1))
        } else {
            (value0 >> count)
                | ((self.f_cf as u64) << ((sz as u64) - count))
                | (value0 << ((sz + 1) as u64 - count))
        };

        self.f_cf = ((value0 >> (count - 1)) & 1) == 1;
        self.f_of = ((res ^ (res << 1)) >> 63) == 1;
    }

    pub fn rcr(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = value1 & mask;
        let pow = if sz == 64 { u64::MAX } else { (1u64 << sz) - 1 };
        let count = count % (sz as u64 + 1);
        let res = match count {
            0 => value0 & pow,
            1 => ((value0 >> 1) | ((self.f_cf as u64) << (sz - 1))) & pow,
            _ => {
                ((value0 >> count)
                    | ((self.f_cf as u64) << (sz as u64 - count))
                    | (value0 << (sz as u64 + 1 - count)))
                    & pow
            }
        };

        if count != 0 {
            self.f_cf = ((value0 >> (count - 1)) & 1) != 0;
        }

        if count == 1 {
            self.f_of = (((res ^ (res << 1)) >> (sz - 1)) & 1) != 0;
        }

        res
    }

    pub fn rcr_prev(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = value1 & mask;
        let res = if count == 1 {
            ((value0 >> count) | ((self.f_cf as u64) << (sz - 1))) & (u64::pow(2, sz) - 1)
        } else {
            ((value0 >> count)
                | ((self.f_cf as u64) << ((sz as u64) - count))
                | (value0 << ((sz + 1) as u64 - count)))
                & (u64::pow(2, sz) - 1)
        };

        self.f_cf = ((value0 >> (count - 1)) & 1) == 1;
        self.f_of = ((res ^ (res << 1)) >> (sz - 1)) == 1;
        // don't calculate the flag zf, sf doesn't got effect
        res
    }

    pub fn rcl(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = (value1 & mask) % (sz as u64 + 1);

        if count == 0 {
            let pow = if sz == 64 { u64::MAX } else { (1u64 << sz) - 1 };
            return value0 & pow;
        }

        if sz == 64 {
            let pow128 = u64::MAX as u128;
            let extended = ((value0 as u128 & pow128) << 1) | (self.f_cf as u128);
            let rotated = ((extended << count) | (extended >> (65 - count))) & ((1u128 << 65) - 1);
            let res = (rotated >> 1) & pow128;
            self.f_cf = (rotated & 1) != 0;
            if count == 1 {
                let msb = (res >> 63) & 1;
                self.f_of = self.f_cf ^ (msb != 0);
            }
            res as u64
        } else {
            let pow = (1u64 << sz) - 1;
            let extended = ((value0 & pow) << 1) | (self.f_cf as u64);
            let rotated = ((extended << count) | (extended >> ((sz + 1) as u64 - count)))
                & ((1u64 << (sz + 1)) - 1);
            let res = (rotated >> 1) & pow;
            self.f_cf = (rotated & 1) != 0;
            if count == 1 {
                let msb = (res >> (sz - 1)) & 1;
                self.f_of = self.f_cf ^ (msb != 0);
            }
            res
        }
    }

    pub fn ror(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = value1 & mask;

        let res_mask = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };

        if count == 0 {
            return value0 & res_mask;
        }

        let shift = count % sz as u64;
        let res = ((value0 >> shift) | (value0 << (sz as u64 - shift))) & res_mask;

        // CF is the last bit shifted out (i.e., bit at position count - 1)
        self.f_cf = ((res >> (sz - 1)) & 1) == 1;

        // OF is only defined for 1-bit rotates
        self.f_of = if shift == 1 {
            let msb = (res >> (sz - 1)) & 1;
            let sec_msb = (res >> (sz - 2)) & 1;
            msb ^ sec_msb == 1
        } else {
            false
        };

        res
    }

    pub fn ror_overflow(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };

        // input size can be only 64 32 16 and 8
        let res_mask = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };
        let count = value1 & mask;
        let res = (value0 >> count) | (value0 << (sz as u64 - count)) & res_mask;
        let bit63 = (res >> (sz - 1)) & 1;
        let bit62 = (res >> (sz - 2)) & 1;

        self.f_cf = bit63 == 1;
        self.f_of = bit63 != bit62; // take this for grant
        // don't calculate the flag zf, sf doesn't got effect
        res
    }

    pub fn rol(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = match sz {
            64 => 0x3f,
            _ => 0x1f,
        };

        let res_mask = match sz {
            64 => 0xffff_ffff_ffff_ffff,
            32 => 0xffff_ffff,
            16 => 0xffff,
            8 => 0xff,
            _ => panic!("Unsupported size for ROL: {}", sz),
        };

        let count = (value1 & mask) as u32;
        let width = sz;

        let value0 = value0 & res_mask;

        let count = (value1 & mask) % width as u64;
        let res = if count == 0 {
            value0
        } else {
            ((value0 << count) | (value0 >> (width as u64 - count))) & res_mask
        };

        // CF = least significant bit of the result after the rotate
        self.f_cf = if count != 0 {
            (res & 0x1) == 1
        } else {
            self.f_cf // unchanged
        };

        // OF is defined only when count == 1 for ROL
        self.f_of = if count == 1 {
            let msb = (res >> (width - 1)) & 0x1;
            let lsb = res & 0x1;
            (msb ^ lsb) == 1
        } else {
            self.f_of // unchanged
        };

        res
    }

    pub fn rol_overflow(&mut self, value0: u64, value1: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let res_mask = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };
        let count = value1 & mask;
        let res = ((value0 << count) | (value0 >> (sz as u64 - count))) & res_mask; // panic_const_shr_overflow
        self.f_cf = (res & 0x1) == 1;
        self.f_of = (self.f_cf as u64 ^ (res >> (sz - 1))) == 1;
        // don't calculate the flag zf, sf doesn't got effect
        res
    }

    pub fn shrd(&mut self, value0: u64, value1: u64, count: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = count & mask;
        let res_mask: u64 = match sz {
            64 => 0xffff_ffff_ffff_ffff,
            32 => 0xffff_ffff,
            16 => 0xffff,
            _ => 0xff,
        };
        if count == 0 {
            return value0 & res_mask;
        }

        let res = ((value0 >> count) | (value1 << (sz as u64 - count))) & res_mask;

        self.f_cf = ((value0 >> (count - 1)) & 1) == 1;
        self.f_of = if count == 1 {
            ((value0 >> (sz - 1)) & 1) != ((res >> (sz - 1)) & 1)
        } else {
            false
        };
        self.calc_flags(res, sz);
        res
    }

    pub fn shld(&mut self, value0: u64, value1: u64, count: u64, sz: u32) -> u64 {
        let mask = if sz == 64 { 0x3f } else { 0x1f };
        let count = count & mask;
        let res_mask: u64 = match sz {
            64 => 0xffffffffffffffff,
            32 => 0xffffffff,
            16 => 0xffff,
            _ => 0xff,
        };

        if count == 0 {
            return value0 & res_mask;
        }

        let res = ((value1 >> (sz as u64 - count)) | (value0 << count)) & res_mask;
        self.f_cf = ((value0 >> (sz as u64 - count)) & 1) == 1;
        self.f_of = if count == 1 {
            ((res >> (sz - 1)) & 1) != (self.f_cf as u64)
        } else {
            false
        };
        self.calc_flags(res, sz);
        res
    }
}
