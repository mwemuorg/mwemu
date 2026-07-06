use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub(crate) enum ScalarKind {
    Byte,
    Word,
    Dword,
    Qword,
    Oword,
}

impl ScalarKind {
    #[inline]
    pub(crate) const fn label(self) -> &'static str {
        match self {
            ScalarKind::Byte => "byte",
            ScalarKind::Word => "word",
            ScalarKind::Dword => "dword",
            ScalarKind::Qword => "qword",
            ScalarKind::Oword => "oword",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ScalarTrace {
    ReadWord,
    ReadDword,
    ReadQword,
    WriteWord,
    WriteDword,
    WriteQword,
    WriteOword,
    ForceWriteWord,
    ForceWriteDword,
    ForceWriteQword,
    ForceWriteOword,
}

impl ScalarTrace {
    #[inline]
    pub(crate) const fn label(self) -> &'static str {
        match self {
            ScalarTrace::ReadWord => "read_word",
            ScalarTrace::ReadDword => "read_dword",
            ScalarTrace::ReadQword => "read_qword",
            ScalarTrace::WriteWord => "write_word",
            ScalarTrace::WriteDword => "write_dword",
            ScalarTrace::WriteQword => "write_qword",
            ScalarTrace::WriteOword => "write_oword",
            ScalarTrace::ForceWriteWord => "force_write_word",
            ScalarTrace::ForceWriteDword => "force_write_dword",
            ScalarTrace::ForceWriteQword => "force_write_qword",
            ScalarTrace::ForceWriteOword => "force_write_oword",
        }
    }
}

pub(crate) trait LittleEndianScalar: Copy {
    const SIZE: usize;

    fn from_le_slice(bytes: &[u8]) -> Self;

    fn to_le_vec(self) -> Vec<u8>;

    /// Write the little-endian bytes into `out[..SIZE]` without allocating.
    /// This is the hot write path (every `write_dword`/`write_qword`/… goes
    /// through it), so it avoids the heap allocation `to_le_vec` does per call.
    fn write_le(self, out: &mut [u8]);
}

macro_rules! impl_little_endian_scalar {
    ($ty:ty, $size:expr) => {
        impl LittleEndianScalar for $ty {
            const SIZE: usize = $size;

            fn from_le_slice(bytes: &[u8]) -> Self {
                let arr: [u8; $size] = bytes.try_into().expect("incorrect length");
                <$ty>::from_le_bytes(arr)
            }

            fn to_le_vec(self) -> Vec<u8> {
                <$ty>::to_le_bytes(self).to_vec()
            }

            #[inline(always)]
            fn write_le(self, out: &mut [u8]) {
                out[..$size].copy_from_slice(&<$ty>::to_le_bytes(self));
            }
        }
    };
}

impl_little_endian_scalar!(u8, 1);
impl_little_endian_scalar!(u16, 2);
impl_little_endian_scalar!(u32, 4);
impl_little_endian_scalar!(u64, 8);
impl_little_endian_scalar!(u128, 16);

pub(crate) fn read_le<T: LittleEndianScalar>(bytes: &[u8]) -> Option<T> {
    if bytes.len() < T::SIZE {
        return None;
    }

    Some(T::from_le_slice(&bytes[..T::SIZE]))
}
