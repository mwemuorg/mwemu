use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::automation::{self, CURRENCY_UNITS, Decimal, Number};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{DISP_E_OVERFLOW, HRESULT_E_INVALID_ARG, S_OK};

fn rounded_quotient(value: i128, divisor: i128) -> i128 {
    let quotient = value / divisor;
    let twice = (value % divisor).abs() * 2;
    if twice > divisor || twice == divisor && quotient % 2 != 0 {
        quotient + value.signum()
    } else {
        quotient
    }
}

#[derive(Clone, Copy)]
enum CurrencyOp {
    Abs,
    Neg,
    Fix,
    Int,
    Add,
    Sub,
    Mul,
    MulI4,
    MulI8,
    Round,
    Date,
}

impl CurrencyOp {
    fn apply(self, a: i64, b: i64) -> Result<i64, u64> {
        let a = i128::from(a);
        let b = i128::from(b);
        let units = i128::from(CURRENCY_UNITS);
        let value = match self {
            Self::Abs => a.abs(),
            Self::Neg => -a,
            Self::Fix => a / units * units,
            Self::Int => a.div_euclid(units) * units,
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => rounded_quotient(a * b, units),
            Self::MulI4 | Self::MulI8 => a * b,
            Self::Round if b < 0 => return Err(HRESULT_E_INVALID_ARG),
            Self::Round if b < 4 => {
                let step = 10i128.pow(4 - b as u32);
                rounded_quotient(a, step) * step
            }
            Self::Round | Self::Date => a,
        };
        i64::try_from(value).map_err(|_| DISP_E_OVERFLOW)
    }
}

fn currency(emu: &mut Emu, name: &str, op: CurrencyOp) {
    let call = WinApi(Stdcall);
    let a = api_try!(emu, call.arg_i64(emu, 0));
    let binary = matches!(
        op,
        CurrencyOp::Add
            | CurrencyOp::Sub
            | CurrencyOp::Mul
            | CurrencyOp::MulI4
            | CurrencyOp::MulI8
            | CurrencyOp::Round
    );
    let small = matches!(op, CurrencyOp::MulI4 | CurrencyOp::Round);
    let mut words = 2;
    let b = if binary {
        let idx = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
        if small {
            words += 1;
            i64::from(api_try!(emu, call.arg_i32(emu, idx)))
        } else {
            words += 2;
            api_try!(emu, call.arg_i64(emu, idx))
        }
    } else {
        0
    };
    let out = api_try!(
        emu,
        automation::argument_at(emu, 1 + usize::from(binary), words)
    );
    let result = match op {
        CurrencyOp::Date => {
            automation::numeric_bytes(a as f64 / f64::from(CURRENCY_UNITS), Number::Date)
                .and_then(|bytes| automation::output(emu, out, &bytes))
        }
        CurrencyOp::MulI8 => {
            let (value, overflow) = a.overflowing_mul(b);
            automation::output(emu, out, &value.to_le_bytes()).and(if overflow {
                Err(DISP_E_OVERFLOW)
            } else {
                Ok(())
            })
        }
        _ => op
            .apply(a, b)
            .and_then(|value| automation::output(emu, out, &value.to_le_bytes())),
    };
    log_red!(emu, "oleaut32!{name} output={out:#x} result={result:?}");
    call.set_return(emu, result.err().unwrap_or(S_OK));
    call.cleanup_args(emu, words + 1);
}

#[derive(Clone, Copy)]
enum DecimalOp {
    Abs,
    Neg,
    Fix,
    Int,
    Round,
}

fn decimal(emu: &mut Emu, name: &str, op: DecimalOp) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let rounding = matches!(op, DecimalOp::Round);
    let digits = if rounding {
        api_try!(emu, call.arg_i32(emu, 1))
    } else {
        0
    };
    let last = 1 + usize::from(rounding);
    let out = api_try!(emu, call.arg(emu, last));
    let result = Decimal::read(emu, source).and_then(|mut value| {
        match op {
            DecimalOp::Abs => value.negative = false,
            DecimalOp::Neg => value.negative = !value.negative,
            _ if digits < 0 => return Err(HRESULT_E_INVALID_ARG),
            _ if digits < i32::from(value.scale) => {
                let power = 10i128.pow(u32::from(value.scale) - digits as u32);
                let absolute = value.magnitude as i128;
                value.magnitude = match op {
                    DecimalOp::Round => rounded_quotient(absolute, power),
                    DecimalOp::Int if value.negative => (absolute + power - 1) / power,
                    _ => absolute / power,
                } as u128;
                value.scale = digits as u8;
            }
            _ => {}
        }
        value.write(emu, out)
    });
    log_red!(
        emu,
        "oleaut32!{name} decimal={source:#x} output={out:#x} result={result:?}"
    );
    call.set_return(emu, result.err().unwrap_or(S_OK));
    call.cleanup_args(emu, last + 1);
}

macro_rules! operations {
    ($handler:ident,$op:ident; $($name:ident => $variant:ident),+ $(,)?) => {
        $(pub fn $name(emu:&mut Emu){$handler(emu,stringify!($name),$op::$variant);})+
    }
}
operations!(currency,CurrencyOp;
    VarCyAbs=>Abs, VarCyNeg=>Neg, VarCyFix=>Fix, VarCyInt=>Int, VarCyAdd=>Add,
    VarCySub=>Sub, VarCyMul=>Mul, VarCyMulI4=>MulI4, VarCyMulI8=>MulI8, VarCyRound=>Round, VarDateFromCy=>Date,
);
operations!(decimal,DecimalOp;
    VarDecAbs=>Abs, VarDecNeg=>Neg,VarDecFix=>Fix,VarDecInt=>Int,VarDecRound=>Round,
);

pub(super) fn write_fixed(
    emu: &mut Emu,
    destination: u64,
    source: Decimal,
    kind: Option<Number>,
) -> Result<(), u64> {
    let Some(number) = kind else {
        return source.write(emu, destination);
    };
    let signed = if source.negative {
        -(source.magnitude as i128)
    } else {
        source.magnitude as i128
    };
    let divisor = 10i128.pow(source.scale.into());
    match number {
        Number::Signed(bits) | Number::Unsigned(bits) => {
            let integer = rounded_quotient(signed, divisor);
            let signed_output = matches!(number, Number::Signed(_));
            let upper = 1i128 << (bits - u32::from(signed_output));
            let lower = if signed_output { -upper } else { 0 };
            if !(lower..upper).contains(&integer) {
                return Err(DISP_E_OVERFLOW);
            }
            automation::output(
                emu,
                destination,
                &integer.to_le_bytes()[..(bits / u8::BITS) as usize],
            )
        }
        Number::Currency => {
            let value = rounded_quotient(signed * i128::from(CURRENCY_UNITS), divisor);
            let currency = i64::try_from(value).map_err(|_| DISP_E_OVERFLOW)?;
            automation::output(emu, destination, &currency.to_le_bytes())
        }
        _ => {
            let bytes = automation::numeric_bytes(signed as f64 / divisor as f64, number)?;
            automation::output(emu, destination, &bytes)
        }
    }
}

fn convert_fixed(emu: &mut Emu, api: &str, currency: bool, kind: Option<Number>) {
    let call = WinApi(Stdcall);
    let (source, destination, words) = if currency {
        let raw = api_try!(emu, call.arg_i64(emu, 0));
        let destination = api_try!(emu, automation::argument_at(emu, 1, 2));
        log_red!(
            emu,
            "oleaut32!{api} currency={raw} destination={destination:#x}"
        );
        (
            Ok(Decimal {
                magnitude: u128::from(raw.unsigned_abs()),
                scale: 4,
                negative: raw < 0,
            }),
            destination,
            3,
        )
    } else {
        let [decimal, destination] = api_try!(emu, call.args(emu));
        log_red!(
            emu,
            "oleaut32!{api} decimal={decimal:#x} destination={destination:#x}"
        );
        (Decimal::read(emu, decimal), destination, 2)
    };
    let result = source.and_then(|source| write_fixed(emu, destination, source, kind));
    call.set_return(emu, result.err().unwrap_or(S_OK));
    call.cleanup_args(emu, words);
}

macro_rules! fixed_conversions {
    ($is_currency:literal; $($name:ident => $output:expr),+ $(,)?) => {
        $(
            pub fn $name(emu: &mut Emu) {
                convert_fixed(emu, stringify!($name), $is_currency, $output);
            }
        )+
    };
}

fixed_conversions!(true;
    VarBoolFromCy => Some(Number::Bool),
    VarR4FromCy => Some(Number::Single),
    VarR8FromCy => Some(Number::Double),
    VarI1FromCy => Some(Number::Signed(i8::BITS)),
    VarUI1FromCy => Some(Number::Unsigned(u8::BITS)),
    VarI2FromCy => Some(Number::Signed(i16::BITS)),
    VarUI2FromCy => Some(Number::Unsigned(u16::BITS)),
    VarI4FromCy => Some(Number::Signed(i32::BITS)),
    VarUI4FromCy => Some(Number::Unsigned(u32::BITS)),
    VarI8FromCy => Some(Number::Signed(i64::BITS)),
    VarUI8FromCy => Some(Number::Unsigned(u64::BITS)),
    VarDecFromCy => None,
);

fixed_conversions!(false;
    VarBoolFromDec => Some(Number::Bool),
    VarR4FromDec => Some(Number::Single),
    VarR8FromDec => Some(Number::Double),
    VarI1FromDec => Some(Number::Signed(i8::BITS)),
    VarUI1FromDec => Some(Number::Unsigned(u8::BITS)),
    VarI2FromDec => Some(Number::Signed(i16::BITS)),
    VarUI2FromDec => Some(Number::Unsigned(u16::BITS)),
    VarI4FromDec => Some(Number::Signed(i32::BITS)),
    VarUI4FromDec => Some(Number::Unsigned(u32::BITS)),
    VarI8FromDec => Some(Number::Signed(i64::BITS)),
    VarUI8FromDec => Some(Number::Unsigned(u64::BITS)),
);
