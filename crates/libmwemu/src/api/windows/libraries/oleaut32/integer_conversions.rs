use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::automation::{self, Number};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{DISP_E_OVERFLOW, S_OK};

enum Output {
    Number(Number),
    Decimal,
}

fn convert_integer(
    emu: &mut Emu,
    out: u64,
    value: i128,
    from_bool: bool,
    kind: Output,
) -> Result<(), u64> {
    let Output::Number(kind) = kind else {
        let value = if from_bool {
            -i128::from(value != 0)
        } else {
            value
        };
        return automation::Decimal {
            magnitude: value.unsigned_abs(),
            scale: 0,
            negative: value < 0,
        }
        .write(emu, out);
    };
    let bytes = match kind {
        Number::Bool => Ok((if value == 0 {
            automation::VARIANT_FALSE
        } else {
            automation::VARIANT_TRUE
        })
        .to_le_bytes()
        .to_vec()),
        Number::Signed(bits) | Number::Unsigned(bits) => {
            let signed = matches!(kind, Number::Signed(_));
            let upper = 1_i128 << (bits - u32::from(signed));
            let lower = if signed { -upper } else { 0 };
            if !from_bool && !(lower..upper).contains(&value) {
                return Err(DISP_E_OVERFLOW);
            }
            Ok(value.to_le_bytes()[..(bits / u8::BITS) as usize].to_vec())
        }
        Number::Single => Ok((value as f32).to_le_bytes().to_vec()),
        Number::Double => Ok((value as f64).to_le_bytes().to_vec()),
        Number::Date => automation::numeric_bytes(value as f64, Number::Date),
        Number::Currency => i64::try_from(value * i128::from(automation::CURRENCY_UNITS))
            .map(|scaled| scaled.to_le_bytes().to_vec())
            .map_err(|_| DISP_E_OVERFLOW),
    }?;
    automation::output(emu, out, &bytes)
}

macro_rules! integer_conversions {
    ($read:ident, $words:expr, $from_bool:expr; $($name:ident => $target:expr),+ $(,)?) => {
        $(pub fn $name(emu: &mut Emu) {
            let call = WinApi(Stdcall);
            let input = api_try!(emu, call.$read(emu, 0));
            let out = api_try!(emu, automation::argument_at(emu, 1, $words));
            let result = convert_integer(emu,out,i128::from(input),$from_bool,$target);
            log_red!(emu, "oleaut32!{} input={input} output={out:#x} result={result:?}", stringify!($name));
            call.set_return(emu, result.err().unwrap_or(S_OK));
            call.cleanup_args(emu, $words + 1);
        })+
    };
}

integer_conversions!(arg_i16, 1, true;
    VarCyFromBool => Output::Number(Number::Currency),
    VarDateFromBool => Output::Number(Number::Date),
    VarDecFromBool => Output::Decimal,
    VarI1FromBool => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromBool => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromBool => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromBool => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromBool => Output::Number(Number::Single),
    VarR8FromBool => Output::Number(Number::Double),
    VarUI1FromBool => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromBool => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromBool => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromBool => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_i8, 1, false;
    VarBoolFromI1 => Output::Number(Number::Bool),
    VarCyFromI1 => Output::Number(Number::Currency),
    VarDateFromI1 => Output::Number(Number::Date),
    VarDecFromI1 => Output::Decimal,
    VarI2FromI1 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromI1 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromI1 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromI1 => Output::Number(Number::Single),
    VarR8FromI1 => Output::Number(Number::Double),
    VarUI1FromI1 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromI1 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromI1 => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromI1 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_i16, 1, false;
    VarBoolFromI2 => Output::Number(Number::Bool),
    VarCyFromI2 => Output::Number(Number::Currency),
    VarDateFromI2 => Output::Number(Number::Date),
    VarDecFromI2 => Output::Decimal,
    VarI1FromI2 => Output::Number(Number::Signed(i8::BITS)),
    VarI4FromI2 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromI2 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromI2 => Output::Number(Number::Single),
    VarR8FromI2 => Output::Number(Number::Double),
    VarUI1FromI2 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromI2 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromI2 => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromI2 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_i32, 1, false;
    VarBoolFromI4 => Output::Number(Number::Bool),
    VarCyFromI4 => Output::Number(Number::Currency),
    VarDateFromI4 => Output::Number(Number::Date),
    VarDecFromI4 => Output::Decimal,
    VarI1FromI4 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromI4 => Output::Number(Number::Signed(i16::BITS)),
    VarR4FromI4 => Output::Number(Number::Single),
    VarR8FromI4 => Output::Number(Number::Double),
    VarUI1FromI4 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromI4 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromI4 => Output::Number(Number::Unsigned(u32::BITS)),
);

integer_conversions!(arg_i64, 2, false;
    VarBoolFromI8 => Output::Number(Number::Bool),
    VarCyFromI8 => Output::Number(Number::Currency),
    VarDateFromI8 => Output::Number(Number::Date),
    VarDecFromI8 => Output::Decimal,
    VarI1FromI8 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromI8 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromI8 => Output::Number(Number::Signed(i32::BITS)),
    VarR4FromI8 => Output::Number(Number::Single),
    VarR8FromI8 => Output::Number(Number::Double),
    VarUI1FromI8 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromI8 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromI8 => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromI8 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_u8, 1, false;
    VarBoolFromUI1 => Output::Number(Number::Bool),
    VarCyFromUI1 => Output::Number(Number::Currency),
    VarDateFromUI1 => Output::Number(Number::Date),
    VarDecFromUI1 => Output::Decimal,
    VarI1FromUI1 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromUI1 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromUI1 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromUI1 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromUI1 => Output::Number(Number::Single),
    VarR8FromUI1 => Output::Number(Number::Double),
    VarUI2FromUI1 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromUI1 => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromUI1 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_u16, 1, false;
    VarBoolFromUI2 => Output::Number(Number::Bool),
    VarCyFromUI2 => Output::Number(Number::Currency),
    VarDateFromUI2 => Output::Number(Number::Date),
    VarDecFromUI2 => Output::Decimal,
    VarI1FromUI2 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromUI2 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromUI2 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromUI2 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromUI2 => Output::Number(Number::Single),
    VarR8FromUI2 => Output::Number(Number::Double),
    VarUI1FromUI2 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI4FromUI2 => Output::Number(Number::Unsigned(u32::BITS)),
    VarUI8FromUI2 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_u32, 1, false;
    VarBoolFromUI4 => Output::Number(Number::Bool),
    VarCyFromUI4 => Output::Number(Number::Currency),
    VarDateFromUI4 => Output::Number(Number::Date),
    VarDecFromUI4 => Output::Decimal,
    VarI1FromUI4 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromUI4 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromUI4 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromUI4 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromUI4 => Output::Number(Number::Single),
    VarR8FromUI4 => Output::Number(Number::Double),
    VarUI1FromUI4 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromUI4 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI8FromUI4 => Output::Number(Number::Unsigned(u64::BITS)),
);

integer_conversions!(arg_u64, 2, false;
    VarBoolFromUI8 => Output::Number(Number::Bool),
    VarCyFromUI8 => Output::Number(Number::Currency),
    VarDateFromUI8 => Output::Number(Number::Date),
    VarDecFromUI8 => Output::Decimal,
    VarI1FromUI8 => Output::Number(Number::Signed(i8::BITS)),
    VarI2FromUI8 => Output::Number(Number::Signed(i16::BITS)),
    VarI4FromUI8 => Output::Number(Number::Signed(i32::BITS)),
    VarI8FromUI8 => Output::Number(Number::Signed(i64::BITS)),
    VarR4FromUI8 => Output::Number(Number::Single),
    VarR8FromUI8 => Output::Number(Number::Double),
    VarUI1FromUI8 => Output::Number(Number::Unsigned(u8::BITS)),
    VarUI2FromUI8 => Output::Number(Number::Unsigned(u16::BITS)),
    VarUI4FromUI8 => Output::Number(Number::Unsigned(u32::BITS)),
);
