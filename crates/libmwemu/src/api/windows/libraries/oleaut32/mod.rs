mod dispatch;
pub(crate) use dispatch::dispatch;
pub mod exports;

pub use crate::winapi::common::bstr::{
    SysAllocString, SysAllocStringByteLen, SysAllocStringLen, SysFreeString, SysReAllocString,
    SysReAllocStringLen, SysStringByteLen, SysStringLen,
};
pub use crate::winapi::libraries::unsupported::oleaut32::{
    VarBstrFromDate, VarBstrFromR4, VarBstrFromR8, VarDecFromDate, VarDecFromR4, VarDecFromR8,
};

pub(crate) mod error_info;
pub use error_info::{GetErrorInfo, SetErrorInfo};

mod var_bool_from_date;
pub use var_bool_from_date::VarBoolFromDate;

mod var_bool_from_r4;
pub use var_bool_from_r4::VarBoolFromR4;

mod var_bool_from_r8;
pub use var_bool_from_r8::VarBoolFromR8;

mod var_cy_cmp_r8;
pub use var_cy_cmp_r8::VarCyCmpR8;

mod var_cy_from_date;
pub use var_cy_from_date::VarCyFromDate;

mod var_cy_from_r4;
pub use var_cy_from_r4::VarCyFromR4;

mod var_cy_from_r8;
pub use var_cy_from_r8::VarCyFromR8;

mod var_date_from_r4;
pub use var_date_from_r4::VarDateFromR4;

mod var_date_from_r8;
pub use var_date_from_r8::VarDateFromR8;

mod var_dec_cmp_r8;
pub use var_dec_cmp_r8::VarDecCmpR8;

mod var_i1_from_date;
pub use var_i1_from_date::VarI1FromDate;

mod var_i1_from_r4;
pub use var_i1_from_r4::VarI1FromR4;

mod var_i1_from_r8;
pub use var_i1_from_r8::VarI1FromR8;

mod var_i2_from_date;
pub use var_i2_from_date::VarI2FromDate;

mod var_i2_from_r4;
pub use var_i2_from_r4::VarI2FromR4;

mod var_i2_from_r8;
pub use var_i2_from_r8::VarI2FromR8;

mod var_i4_from_date;
pub use var_i4_from_date::VarI4FromDate;

mod var_i4_from_r4;
pub use var_i4_from_r4::VarI4FromR4;

mod var_i4_from_r8;
pub use var_i4_from_r8::VarI4FromR8;

mod var_i8_from_date;
pub use var_i8_from_date::VarI8FromDate;

mod var_i8_from_r4;
pub use var_i8_from_r4::VarI8FromR4;

mod var_i8_from_r8;
pub use var_i8_from_r8::VarI8FromR8;

mod var_r4_cmp_r8;
pub use var_r4_cmp_r8::VarR4CmpR8;

mod var_r4_from_date;
pub use var_r4_from_date::VarR4FromDate;

mod var_r4_from_r8;
pub use var_r4_from_r8::VarR4FromR8;

mod var_r8_from_date;
pub use var_r8_from_date::VarR8FromDate;

mod var_r8_from_r4;
pub use var_r8_from_r4::VarR8FromR4;

mod var_r8_pow;
pub use var_r8_pow::VarR8Pow;

mod var_r8_round;
pub use var_r8_round::VarR8Round;

mod var_udate_from_date;
pub use var_udate_from_date::VarUdateFromDate;

mod var_ui1_from_date;
pub use var_ui1_from_date::VarUI1FromDate;

mod var_ui1_from_r4;
pub use var_ui1_from_r4::VarUI1FromR4;

mod var_ui1_from_r8;
pub use var_ui1_from_r8::VarUI1FromR8;

mod var_ui2_from_date;
pub use var_ui2_from_date::VarUI2FromDate;

mod var_ui2_from_r4;
pub use var_ui2_from_r4::VarUI2FromR4;

mod var_ui2_from_r8;
pub use var_ui2_from_r8::VarUI2FromR8;

mod var_ui4_from_date;
pub use var_ui4_from_date::VarUI4FromDate;

mod var_ui4_from_r4;
pub use var_ui4_from_r4::VarUI4FromR4;

mod var_ui4_from_r8;
pub use var_ui4_from_r8::VarUI4FromR8;

mod var_ui8_from_date;
pub use var_ui8_from_date::VarUI8FromDate;

mod var_ui8_from_r4;
pub use var_ui8_from_r4::VarUI8FromR4;

mod var_ui8_from_r8;
pub use var_ui8_from_r8::VarUI8FromR8;

mod variant_time_to_dos_date_time;
pub use variant_time_to_dos_date_time::VariantTimeToDosDateTime;

mod variant_time_to_system_time;
pub use variant_time_to_system_time::VariantTimeToSystemTime;

mod integer_conversions;
pub use integer_conversions::{
    VarBoolFromI1, VarBoolFromI2, VarBoolFromI4, VarBoolFromI8, VarBoolFromUI1, VarBoolFromUI2,
    VarBoolFromUI4, VarBoolFromUI8, VarCyFromBool, VarCyFromI1, VarCyFromI2, VarCyFromI4,
    VarCyFromI8, VarCyFromUI1, VarCyFromUI2, VarCyFromUI4, VarCyFromUI8, VarDateFromBool,
    VarDateFromI1, VarDateFromI2, VarDateFromI4, VarDateFromI8, VarDateFromUI1, VarDateFromUI2,
    VarDateFromUI4, VarDateFromUI8, VarDecFromBool, VarDecFromI1, VarDecFromI2, VarDecFromI4,
    VarDecFromI8, VarDecFromUI1, VarDecFromUI2, VarDecFromUI4, VarDecFromUI8, VarI1FromBool,
    VarI1FromI2, VarI1FromI4, VarI1FromI8, VarI1FromUI1, VarI1FromUI2, VarI1FromUI4, VarI1FromUI8,
    VarI2FromBool, VarI2FromI1, VarI2FromI4, VarI2FromI8, VarI2FromUI1, VarI2FromUI2, VarI2FromUI4,
    VarI2FromUI8, VarI4FromBool, VarI4FromI1, VarI4FromI2, VarI4FromI8, VarI4FromUI1, VarI4FromUI2,
    VarI4FromUI4, VarI4FromUI8, VarI8FromBool, VarI8FromI1, VarI8FromI2, VarI8FromUI1,
    VarI8FromUI2, VarI8FromUI4, VarI8FromUI8, VarR4FromBool, VarR4FromI1, VarR4FromI2, VarR4FromI4,
    VarR4FromI8, VarR4FromUI1, VarR4FromUI2, VarR4FromUI4, VarR4FromUI8, VarR8FromBool,
    VarR8FromI1, VarR8FromI2, VarR8FromI4, VarR8FromI8, VarR8FromUI1, VarR8FromUI2, VarR8FromUI4,
    VarR8FromUI8, VarUI1FromBool, VarUI1FromI1, VarUI1FromI2, VarUI1FromI4, VarUI1FromI8,
    VarUI1FromUI2, VarUI1FromUI4, VarUI1FromUI8, VarUI2FromBool, VarUI2FromI1, VarUI2FromI2,
    VarUI2FromI4, VarUI2FromI8, VarUI2FromUI1, VarUI2FromUI4, VarUI2FromUI8, VarUI4FromBool,
    VarUI4FromI1, VarUI4FromI2, VarUI4FromI4, VarUI4FromI8, VarUI4FromUI1, VarUI4FromUI2,
    VarUI4FromUI8, VarUI8FromBool, VarUI8FromI1, VarUI8FromI2, VarUI8FromI8, VarUI8FromUI1,
    VarUI8FromUI2, VarUI8FromUI4,
};

mod fixed_point;
pub use fixed_point::{
    VarBoolFromCy, VarBoolFromDec, VarCyAbs, VarCyAdd, VarCyFix, VarCyInt, VarCyMul, VarCyMulI4,
    VarCyMulI8, VarCyNeg, VarCyRound, VarCySub, VarDateFromCy, VarDecAbs, VarDecFix, VarDecFromCy,
    VarDecInt, VarDecNeg, VarDecRound, VarI1FromCy, VarI1FromDec, VarI2FromCy, VarI2FromDec,
    VarI4FromCy, VarI4FromDec, VarI8FromCy, VarI8FromDec, VarR4FromCy, VarR4FromDec, VarR8FromCy,
    VarR8FromDec, VarUI1FromCy, VarUI1FromDec, VarUI2FromCy, VarUI2FromDec, VarUI4FromCy,
    VarUI4FromDec, VarUI8FromCy, VarUI8FromDec,
};

mod safe_array;
pub use safe_array::{
    SafeArrayAccessData, SafeArrayCreate, SafeArrayCreateEx, SafeArrayCreateVector,
    SafeArrayCreateVectorEx, SafeArrayDestroy, SafeArrayGetDim, SafeArrayGetElement,
    SafeArrayGetElemsize, SafeArrayGetLBound, SafeArrayGetUBound, SafeArrayGetVartype,
    SafeArrayLock, SafeArrayPtrOfIndex, SafeArrayPutElement, SafeArrayUnaccessData,
    SafeArrayUnlock,
};

mod text_conversions;
pub use text_conversions::{
    VarBoolFromStr, VarBstrFromBool, VarBstrFromCy, VarBstrFromDec, VarBstrFromI1, VarBstrFromI2,
    VarBstrFromI4, VarBstrFromI8, VarBstrFromUI1, VarBstrFromUI2, VarBstrFromUI4, VarBstrFromUI8,
    VarDecFromStr, VarI1FromStr, VarI2FromStr, VarI4FromStr, VarI8FromStr, VarR4FromStr,
    VarR8FromStr, VarUI1FromStr, VarUI2FromStr, VarUI4FromStr, VarUI8FromStr,
};

mod variant;
pub use variant::{
    PropVariantClear, PropVariantCopy, VariantChangeType, VariantChangeTypeEx, VariantClear,
    VariantCopy, VariantCopyInd, VariantInit,
};
