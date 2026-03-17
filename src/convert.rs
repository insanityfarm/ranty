//! # ranty::convert
//! Provides conversions between `RantyValue`s and native types.

#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use crate::runtime::*;
use crate::*;
use crate::{
    lang::{Identifier, Parameter, Varity},
    stdlib::RantyStdResult,
};
use cast::Error as CastError;
use cast::*;
use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// Enables infallible conversion into a `RantyValue`.
pub trait IntoRanty: Sized {
    /// Converts to a `RantyValue`.
    fn into_ranty(self) -> RantyValue;
}

/// Enables fallible conversion into a `RantyValue`.
pub trait TryIntoRanty: Sized {
    /// Attempts to convert to a `RantyValue`.
    fn try_into_ranty(self) -> Result<RantyValue, ValueError>;
}

pub trait FromRanty: Sized {
    /// Converts from a `RantyValue`.
    fn from_ranty(val: RantyValue) -> Self;

    /// Returns `true` if the type can be used to represent an optional Ranty parameter in native functions; otherwise, `false`.
    fn is_optional_param_type() -> bool {
        false
    }
}

/// Enables fallible conversion from a `RantyValue`.
pub trait TryFromRanty: Sized {
    /// Convert from a `RantyValue`.
    fn try_from_ranty(val: RantyValue) -> Result<Self, ValueError>;

    /// Returns `true` if the type can be used to represent an optional Ranty parameter in native functions; otherwise, `false`.
    fn is_optional_param_type() -> bool {
        false
    }
}

trait IntoCastResult<T> {
    fn into_cast_result(self) -> Result<T, CastError>;
}

impl<T> IntoCastResult<T> for Result<T, CastError> {
    fn into_cast_result(self) -> Result<T, CastError> {
        self
    }
}

impl IntoCastResult<i64> for i64 {
    fn into_cast_result(self) -> Result<i64, CastError> {
        Ok(self)
    }
}

fn ranty_cast_error(from: &'static str, to: &'static str, err: CastError) -> ValueError {
    ValueError::InvalidConversion {
        from,
        to,
        message: Some(
            match err {
                CastError::Overflow => "integer overflow",
                CastError::Underflow => "integer underflow",
                CastError::Infinite => "infinity",
                CastError::NaN => "NaN",
            }
            .to_owned(),
        ),
    }
}

macro_rules! ranty_fallible_int_conversions {
  ($int_type: ident) => {
    impl TryIntoRanty for $int_type {
      fn try_into_ranty(self) -> ValueResult<RantyValue> {
        match i64(self).into_cast_result() {
          Ok(i) => Ok(RantyValue::Int(i)),
          Err(err) => Err(ranty_cast_error(
            stringify!($int_type),
            stringify!(RantyValue::Int),
            err
          ))
        }
      }
    }

    impl TryFromRanty for $int_type {
      fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        macro_rules! cast_int {
          (i64, $val:expr) => {
            Ok($val)
          };
          (isize, $val:expr) => {
            {
              let val = $val;
              val.try_into().map_err(|_| if val < 0 {
                CastError::Underflow
              } else {
                CastError::Overflow
              })
            }
          };
          ($to:ident, $val:expr) => {
            $to($val)
          };
        }
        macro_rules! cast_float_to_int {
          (i64, $val:expr) => {
            Ok($val as i64)
          };
          ($to:ident, $val:expr) => {
            $to($val)
          };
        }
        match val {
          RantyValue::Int(i) => {
            let result: Result<$int_type, CastError> = cast_int!($int_type, i);
            match result {
              Ok(i) => Ok(i),
              Err(err) => Err(ranty_cast_error(
                val.type_name(),
                stringify!($int_type),
                err
              ))
            }
          },
          RantyValue::Float(f) => {
            let result: Result<$int_type, CastError> = cast_float_to_int!($int_type, f);
            match result {
              Ok(i) => Ok(i),
              Err(err) => Err(ranty_cast_error(
                val.type_name(),
                stringify!($int_type),
                err
              ))
            }
          },
          _ => {
            // Other conversion failure
            let src_type = val.type_name();
            let dest_type = stringify!{$int_type};

            Err(ValueError::InvalidConversion {
              from: src_type,
              to: dest_type,
              message: None
            })
          }
        }
      }
    }
  };
  ($int_type: ident, $($int_type2: ident), *) => {
    ranty_fallible_int_conversions! { $int_type }
    ranty_fallible_int_conversions! { $($int_type2), + }
  };
}

/// Implements `FromRanty` and `TryFromRanty` for a type.
macro_rules! converts_from_ranty {
    ($param:ident -> $t:ty $b:block) => {
        impl FromRanty for $t {
            fn from_ranty($param: RantyValue) -> $t {
                $b
            }
        }

        impl TryFromRanty for $t {
            fn try_from_ranty(val: RantyValue) -> Result<$t, ValueError> {
                Ok(<$t as FromRanty>::from_ranty(val))
            }
        }
    };
}

/// Implements `IntoRanty` and `TryIntoRanty` for a type.
macro_rules! converts_into_ranty {
    ($param:ident: $t:ty $b:block) => {
        impl IntoRanty for $t {
            fn into_ranty(self) -> RantyValue {
                let $param = self;
                $b
            }
        }

        impl TryIntoRanty for $t {
            fn try_into_ranty(self) -> Result<RantyValue, ValueError> {
                Ok(IntoRanty::into_ranty(self))
            }
        }
    };
}

ranty_fallible_int_conversions! { u8, i8, u16, i16, u32, i32, u64, i64, isize, usize }

converts_from_ranty!(v -> RantyNothing { Self });
converts_from_ranty!(v -> RantyValue { v });
converts_from_ranty!(v -> bool { v.to_bool() });
converts_from_ranty!(v -> InternalString { v.to_string().into() });
converts_from_ranty!(v -> RantyString { v.to_string().into() });
converts_from_ranty!(v -> String { v.to_string() });

converts_into_ranty!(v: RantyValue { v });
converts_into_ranty!(v: RantyNothing { RantyValue::Nothing });
converts_into_ranty!(v: bool { RantyValue::Boolean(v) });
converts_into_ranty!(v: char { RantyValue::String(RantyString::from(v)) });
converts_into_ranty!(v: f32 { RantyValue::Float(v as f64) });
converts_into_ranty!(v: f64 { RantyValue::Float(v) });
converts_into_ranty!(v: String { RantyValue::String(v.into()) });
converts_into_ranty!(v: RantyString { RantyValue::String(v) });
converts_into_ranty!(v: InternalString { RantyString::from(v.as_str()).into_ranty() });
converts_into_ranty!(v: RantyMap { RantyValue::Map(v.into_handle()) });
converts_into_ranty!(v: RantyMapHandle { RantyValue::Map(v) });
converts_into_ranty!(v: RantyList { RantyValue::List(v.into_handle()) });
converts_into_ranty!(v: RantyListHandle { RantyValue::List(v) });
converts_into_ranty!(v: RantyTuple { RantyValue::Tuple(v.into_handle()) });
converts_into_ranty!(v: RantyTupleHandle { RantyValue::Tuple(v) });
converts_into_ranty!(v: RantySelector { RantyValue::Selector(v.into_handle()) });
converts_into_ranty!(v: RantySelectorHandle { RantyValue::Selector(v) });
converts_into_ranty!(v: RantyRange { RantyValue::Range(v) });

impl<'a> IntoRanty for &'a str {
    fn into_ranty(self) -> RantyValue {
        RantyValue::String(self.into())
    }
}

impl<'a> TryIntoRanty for &'a str {
    fn try_into_ranty(self) -> Result<RantyValue, ValueError> {
        Ok(self.into_ranty())
    }
}

impl IntoRanty for isize {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for i64 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self)
    }
}

impl IntoRanty for i32 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for u32 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for i16 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for u16 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for i8 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl IntoRanty for u8 {
    fn into_ranty(self) -> RantyValue {
        RantyValue::Int(self as i64)
    }
}

impl TryFromRanty for f32 {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        match val {
            RantyValue::Int(i) => Ok(f32(i)),
            RantyValue::Float(f) => match f32(f) {
                Ok(f) => Ok(f),
                Err(err) => Err(ranty_cast_error(val.type_name(), "f32", err)),
            },
            _ => Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: "f32",
                message: Some(format!(
                    "Ranty value type '{}' cannot be converted to f32",
                    val.type_name()
                )),
            }),
        }
    }
}

impl TryFromRanty for f64 {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        match val {
            RantyValue::Int(i) => Ok(f64(i)),
            RantyValue::Float(f) => Ok(f),
            _ => Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: "f64",
                message: Some(format!(
                    "Ranty value type '{}' cannot be converted to f64",
                    val.type_name()
                )),
            }),
        }
    }
}

impl<T: IntoRanty> IntoRanty for Vec<T> {
    fn into_ranty(mut self) -> RantyValue {
        let list = self.drain(..).map(|v| v.into_ranty()).collect::<RantyList>();
        RantyValue::List(list.into_handle())
    }
}

impl<T: TryIntoRanty> TryIntoRanty for Vec<T> {
    fn try_into_ranty(mut self) -> Result<RantyValue, ValueError> {
        let list = self
            .drain(..)
            .map(|v| v.try_into_ranty())
            .collect::<Result<RantyList, ValueError>>()?;
        Ok(list.into_ranty())
    }
}

impl TryFromRanty for RantyTupleHandle {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        if let RantyValue::Tuple(tuple_ref) = val {
            Ok(tuple_ref)
        } else {
            Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: RantyValueType::Tuple.name(),
                message: None,
            })
        }
    }
}

impl TryFromRanty for RantyListHandle {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        if let RantyValue::List(list_ref) = val {
            Ok(list_ref)
        } else {
            Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: RantyValueType::List.name(),
                message: None,
            })
        }
    }
}

impl TryFromRanty for RantyMapHandle {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        if let RantyValue::Map(map_ref) = val {
            Ok(map_ref)
        } else {
            Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: RantyValueType::Map.name(),
                message: None,
            })
        }
    }
}

impl TryFromRanty for RantyFunctionHandle {
    fn try_from_ranty(val: RantyValue) -> Result<Self, ValueError> {
        if let RantyValue::Function(func_ref) = val {
            Ok(func_ref)
        } else {
            Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: RantyValueType::Function.name(),
                message: None,
            })
        }
    }
}

impl TryFromRanty for RantySelectorHandle {
    fn try_from_ranty(val: RantyValue) -> Result<Self, ValueError> {
        if let RantyValue::Selector(sel_ref) = val {
            Ok(sel_ref)
        } else {
            Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: RantyValueType::Selector.name(),
                message: None,
            })
        }
    }
}

impl<T: TryFromRanty> TryFromRanty for Option<T> {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        match val {
            RantyValue::Nothing => Ok(None),
            other => Ok(Some(T::try_from_ranty(other)?)),
        }
    }
    fn is_optional_param_type() -> bool {
        true
    }
}

impl<T: TryIntoRanty> TryIntoRanty for Option<T> {
    fn try_into_ranty(self) -> ValueResult<RantyValue> {
        match self {
            Some(val) => Ok(val.try_into_ranty()?),
            None => Ok(RantyValue::Nothing),
        }
    }
}

impl<T: IntoRanty> IntoRanty for Option<T> {
    fn into_ranty(self) -> RantyValue {
        match self {
            Some(val) => val.into_ranty(),
            None => RantyValue::Nothing,
        }
    }
}

impl<T: TryFromRanty> TryFromRanty for Vec<T> {
    fn try_from_ranty(val: RantyValue) -> ValueResult<Self> {
        match val {
            RantyValue::List(vec) => Ok(vec
                .borrow()
                .iter()
                .cloned()
                .map(T::try_from_ranty)
                .collect::<ValueResult<Vec<T>>>()?),
            other => Err(ValueError::InvalidConversion {
                from: other.type_name(),
                to: stringify!(Vec<T>),
                message: Some("only lists can be turned into vectors".to_owned()),
            }),
        }
    }
}

#[inline(always)]
fn as_varity<T: TryFromRanty>() -> Varity {
    if T::is_optional_param_type() {
        Varity::Optional
    } else {
        Varity::Required
    }
}

#[inline(always)]
fn inc(counter: &mut usize) -> usize {
    let prev = *counter;
    *counter += 1;
    prev
}

/// Converts from argument list to tuple of `impl TryFromRanty` values
pub trait FromRantyArgs: Sized {
    fn from_ranty_args(args: Vec<RantyValue>) -> ValueResult<Self>;
    fn as_ranty_params() -> Vec<Parameter>;
}

impl<T: TryFromRanty> FromRantyArgs for T {
    fn from_ranty_args(args: Vec<RantyValue>) -> ValueResult<Self> {
        let mut args = args.into_iter();
        T::try_from_ranty(args.next().unwrap_or(RantyValue::Nothing))
    }

    fn as_ranty_params() -> Vec<Parameter> {
        let varity = if T::is_optional_param_type() {
            Varity::Optional
        } else {
            Varity::Required
        };

        let param = Parameter {
            name: Identifier::new(InternalString::from("arg0")),
            varity,
            default_value_expr: None,
        };

        vec![param]
    }
}

/// Semantic wrapper around a `Vec<T>`.
///
/// Use this type to add an optional variadic (`*`) parameter to native functions.
pub struct VarArgs<T: TryFromRanty>(Vec<T>);

impl<T: TryFromRanty> VarArgs<T> {
    pub fn new(args: Vec<T>) -> Self {
        Self(args)
    }
}

impl<T: TryFromRanty> Deref for VarArgs<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: TryFromRanty> DerefMut for VarArgs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: TryFromRanty> VarArgs<T> {
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

/// Semantic wrapper around a `Vec<T>`.
///
/// Use this type to add a required variadic (`+`) parameter to native functions.
pub struct RequiredVarArgs<T: TryFromRanty>(Vec<T>);

impl<T: TryFromRanty> RequiredVarArgs<T> {
    pub fn new(args: Vec<T>) -> Self {
        Self(args)
    }
}

impl<T: TryFromRanty> Deref for RequiredVarArgs<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: TryFromRanty> DerefMut for RequiredVarArgs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_from_ranty_args {
  ($($generic_types:ident),*) => {
    // Non-variadic implementation
    impl<$($generic_types: TryFromRanty,)*> FromRantyArgs for ($($generic_types,)*) {
      fn from_ranty_args(args: Vec<RantyValue>) -> ValueResult<Self> {
        let mut args = args.into_iter();
        Ok(($($generic_types::try_from_ranty(args.next().unwrap_or(RantyValue::Nothing))?,)*))
      }

      fn as_ranty_params() -> Vec<Parameter> {
        let mut i: usize = 0;
        vec![$(Parameter {
          name: Identifier::new(InternalString::from(format!("arg{}", inc(&mut i)))),
          varity: as_varity::<$generic_types>(),
          default_value_expr: None,
        },)*]
      }
    }

    // Variadic* implementation
    impl<$($generic_types: TryFromRanty,)* VarArgItem: TryFromRanty> FromRantyArgs for ($($generic_types,)* VarArgs<VarArgItem>) {
      fn from_ranty_args(mut args: Vec<RantyValue>) -> ValueResult<Self> {
        let mut args = args.drain(..);
        Ok(
          ($($generic_types::try_from_ranty(args.next().unwrap_or(RantyValue::Nothing))?,)*
          VarArgs::new(args
            .map(VarArgItem::try_from_ranty)
            .collect::<ValueResult<Vec<VarArgItem>>>()?
          )
        ))
      }

      fn as_ranty_params() -> Vec<Parameter> {
        let mut i: usize = 0;
        vec![$(Parameter {
          name: Identifier::new(InternalString::from(format!("arg{}", inc(&mut i)))),
          varity: as_varity::<$generic_types>(),
          default_value_expr: None,
        },)*
        Parameter {
          name: Identifier::new(InternalString::from(format!("arg{}", inc(&mut i)))),
          varity: Varity::VariadicStar,
          default_value_expr: None,
        }]
      }
    }

    // Variadic+ implementation
    impl<$($generic_types: TryFromRanty,)* VarArgItem: TryFromRanty> FromRantyArgs for ($($generic_types,)* RequiredVarArgs<VarArgItem>) {
      fn from_ranty_args(mut args: Vec<RantyValue>) -> ValueResult<Self> {
        let mut args = args.drain(..);
        Ok(
          ($($generic_types::try_from_ranty(args.next().unwrap_or(RantyValue::Nothing))?,)*
          RequiredVarArgs::new(args
            .map(VarArgItem::try_from_ranty)
            .collect::<ValueResult<Vec<VarArgItem>>>()?
          )
        ))
      }

      fn as_ranty_params() -> Vec<Parameter> {
        let mut i: usize = 0;
        vec![$(Parameter {
          name: Identifier::new(InternalString::from(format!("arg{}", inc(&mut i)))),
          varity: as_varity::<$generic_types>(),
          default_value_expr: None,
        },)*
        Parameter {
          name: Identifier::new(InternalString::from(format!("arg{}", inc(&mut i)))),
          varity: Varity::VariadicPlus,
          default_value_expr: None,
        }]
      }
    }
  }
}

impl_from_ranty_args!();
impl_from_ranty_args!(A);
impl_from_ranty_args!(A, B);
impl_from_ranty_args!(A, B, C);
impl_from_ranty_args!(A, B, C, D);
impl_from_ranty_args!(A, B, C, D, E);
impl_from_ranty_args!(A, B, C, D, E, F);
impl_from_ranty_args!(A, B, C, D, E, F, G);
impl_from_ranty_args!(A, B, C, D, E, F, G, H);
impl_from_ranty_args!(A, B, C, D, E, F, G, H, I);
impl_from_ranty_args!(A, B, C, D, E, F, G, H, I, J);
impl_from_ranty_args!(A, B, C, D, E, F, G, H, I, J, K);
//impl_from_ranty_args!(A, B, C, D, E, F, G, H, I, J, K, L);

/// Trait for converting something to a Ranty function.
pub trait IntoRantyFunction<Params: FromRantyArgs> {
    /// Performs the conversion.
    fn into_ranty_func(self) -> RantyFunction;
}

impl<Params: FromRantyArgs, Function: 'static + Fn(&mut VM, Params) -> RantyStdResult>
    IntoRantyFunction<Params> for Function
{
    fn into_ranty_func(self) -> RantyFunction {
        let body = RantyFunctionInterface::Foreign(Rc::new(move |vm, args| {
            self(vm, Params::from_ranty_args(args).into_runtime_result()?)
        }));

        let params = Rc::new(Params::as_ranty_params());

        RantyFunction {
            body,
            captured_vars: vec![],
            min_arg_count: params.iter().take_while(|p| p.is_required()).count(),
            vararg_start_index: params
                .iter()
                .enumerate()
                .find_map(|(i, p)| {
                    if p.varity.is_variadic() {
                        Some(i)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| params.len()),
            params,
            flavor: None,
        }
    }
}
