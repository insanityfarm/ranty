use crate::lang::*;
use crate::runtime::*;
use crate::stdlib::*;
use crate::*;
use std::{
    fmt::Debug,
    mem::{size_of, transmute},
    rc::Rc,
};

/// A function callable from Ranty.
#[derive(Debug)]
pub struct RantyFunction {
    /// Parameter information for the function.
    pub(crate) params: Rc<Vec<Parameter>>,
    /// The number of required parameters.
    pub(crate) min_arg_count: usize,
    /// The parameter index at which variadic parameters start.
    /// If this is greater than or equal to the number of params, there are no variadic parameters.
    pub(crate) vararg_start_index: usize,
    /// The external variables captured by the function when it was defined.
    pub(crate) captured_vars: Vec<(Identifier, RantyVar)>,
    /// The body of the function.
    pub(crate) body: RantyFunctionInterface,
    /// Assigns a custom flavor to the stack frame created by the function call.
    /// If not set, the default function call flavor will be used.
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
}

/// Defines endpoint variants for Ranty functions.
#[derive(Clone)]
pub enum RantyFunctionInterface {
    /// Represents a foreign function as a wrapper function accepting a variable number of arguments.
    Foreign(Rc<dyn 'static + Fn(&mut VM, Vec<RantyValue>) -> RantyStdResult>),
    /// Represents a user function as an RST.
    User(Rc<Sequence>),
}

impl Debug for RantyFunctionInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RantyFunctionInterface::Foreign(func) => unsafe {
                let (a, b) = transmute::<_, (usize, usize)>(Rc::as_ptr(func));
                write!(f, "{:#02$x}{:02$x}", a, b, &(size_of::<usize>() * 2))
            },
            RantyFunctionInterface::User(func) => write!(f, "{:#p}", Rc::as_ptr(func)),
        }
    }
}
