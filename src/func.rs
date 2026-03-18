use crate::gc::{Finalize, Trace};
use crate::lang::*;
use crate::runtime::*;
use crate::stdlib::*;
use crate::*;
use std::{fmt::Debug, mem::transmute, rc::Rc};

#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub(crate) struct CapturedVar {
    #[rust_cc(ignore)]
    pub(crate) name: Identifier,
    pub(crate) var: RantyVar,
}

#[derive(Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub(crate) struct RantyNativeFunction {
    #[rust_cc(ignore)]
    pub(crate) callback:
        unsafe fn(&mut VM, Vec<RantyValue>, &[RantyValue], *const ()) -> RantyStdResult,
    #[rust_cc(ignore)]
    pub(crate) callback_data: *const (),
    pub(crate) captures: Vec<RantyValue>,
}

impl Debug for RantyNativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.callback_data)
    }
}

/// A function callable from Ranty.
#[derive(Debug, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyFunction {
    /// Parameter information for the function.
    #[rust_cc(ignore)]
    pub(crate) params: Rc<Vec<Parameter>>,
    /// The number of required parameters.
    #[rust_cc(ignore)]
    pub(crate) min_arg_count: usize,
    /// The parameter index at which variadic parameters start.
    /// If this is greater than or equal to the number of params, there are no variadic parameters.
    #[rust_cc(ignore)]
    pub(crate) vararg_start_index: usize,
    /// The external variables captured by the function when it was defined.
    pub(crate) captured_vars: Vec<CapturedVar>,
    /// The body of the function.
    pub(crate) body: RantyFunctionInterface,
    /// Assigns a custom flavor to the stack frame created by the function call.
    /// If not set, the default function call flavor will be used.
    #[rust_cc(ignore)]
    pub(crate) flavor: Option<StackFrameFlavor>,
}

impl RantyFunction {
    /// Returns true if the function should be treated as variadic.
    #[inline]
    pub fn is_variadic(&self) -> bool {
        self.vararg_start_index < self.params.len() || self.vararg_start_index < self.min_arg_count
    }

    /// Returns true if the function is native.
    #[inline]
    pub fn is_native(&self) -> bool {
        matches!(self.body, RantyFunctionInterface::Foreign(_))
    }

    #[inline]
    pub fn has_lazy_params(&self) -> bool {
        self.params.iter().any(|param| param.is_lazy)
    }

    pub fn from_native<P: FromRantyArgs>(func: fn(&mut VM, P) -> RantyStdResult) -> Self {
        fn erased<P: FromRantyArgs>(
            vm: &mut VM,
            args: Vec<RantyValue>,
            _captures: &[RantyValue],
            callback_data: *const (),
        ) -> RantyStdResult {
            let callback: fn(&mut VM, P) -> RantyStdResult = unsafe { transmute(callback_data) };
            callback(vm, P::from_ranty_args(args).into_runtime_result()?)
        }

        Self::build_foreign(
            RantyNativeFunction {
                callback: erased::<P>,
                callback_data: func as *const (),
                captures: vec![],
            },
            P::as_ranty_params(),
        )
    }

    pub fn from_captured_native<P: FromRantyArgs>(
        captures: Vec<RantyValue>,
        func: fn(&mut VM, P, &[RantyValue]) -> RantyStdResult,
    ) -> Self {
        fn erased<P: FromRantyArgs>(
            vm: &mut VM,
            args: Vec<RantyValue>,
            captures: &[RantyValue],
            callback_data: *const (),
        ) -> RantyStdResult {
            let callback: fn(&mut VM, P, &[RantyValue]) -> RantyStdResult =
                unsafe { transmute(callback_data) };
            callback(
                vm,
                P::from_ranty_args(args).into_runtime_result()?,
                captures,
            )
        }

        Self::build_foreign(
            RantyNativeFunction {
                callback: erased::<P>,
                callback_data: func as *const (),
                captures,
            },
            P::as_ranty_params(),
        )
    }

    fn build_foreign(native: RantyNativeFunction, params: Vec<Parameter>) -> Self {
        let params = Rc::new(params);

        Self {
            body: RantyFunctionInterface::Foreign(native),
            captured_vars: vec![],
            min_arg_count: params.iter().take_while(|p| p.is_required()).count(),
            vararg_start_index: params
                .iter()
                .enumerate()
                .find_map(|(i, p)| p.varity.is_variadic().then_some(i))
                .unwrap_or_else(|| params.len()),
            params,
            flavor: None,
        }
    }
}

/// Defines endpoint variants for Ranty functions.
#[derive(Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub(crate) enum RantyFunctionInterface {
    /// Represents a foreign function as a function pointer plus an explicit capture list.
    Foreign(RantyNativeFunction),
    /// Represents a user function as an RST.
    #[rust_cc(ignore)]
    User(Rc<Sequence>),
}

impl Debug for RantyFunctionInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RantyFunctionInterface::Foreign(func) => write!(f, "{:p}", func.callback_data),
            RantyFunctionInterface::User(func) => write!(f, "{:#p}", Rc::as_ptr(func)),
        }
    }
}
