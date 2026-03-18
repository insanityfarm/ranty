use std::{cell::RefCell, mem, rc::Rc};

use crate::gc::{self, Finalize, Trace};
use crate::lang::{Identifier, Sequence};
use crate::{CapturedVar, RantyValue};

#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct LazyThunk {
    #[rust_cc(ignore)]
    pub(crate) name: Option<Identifier>,
    #[rust_cc(ignore)]
    pub(crate) expr: Rc<Sequence>,
    pub(crate) captured_vars: Vec<CapturedVar>,
    pub(crate) captured_pipeval: Option<RantyValue>,
}

#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub enum LazyState {
    Pending(LazyThunk),
    Evaluating(LazyThunk),
    Ready(RantyValue),
}

impl LazyState {
    #[inline]
    pub fn display_name(&self) -> Option<&Identifier> {
        match self {
            Self::Pending(thunk) => thunk.name.as_ref(),
            Self::Evaluating(thunk) => thunk.name.as_ref(),
            Self::Ready(_) => None,
        }
    }
}

/// Represents a Ranty variable of one of two flavors: **by-value** or **by-reference**.
///
/// ## Cloning
/// The `Clone` implementation for this type does not copy any data in `ByRef*` variants; it only copies the reference.
#[derive(Debug, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub enum RantyVar {
    /// By-value variable
    ByVal(RantyValue),
    /// By-value constant
    ByValConst(RantyValue),
    /// By-ref variable
    ByRef(crate::gc::Cc<RefCell<RantyValue>>),
    /// By-ref constant
    ByRefConst(crate::gc::Cc<RantyValue>),
    /// Lazily-evaluated variable
    Lazy(crate::gc::Cc<RefCell<LazyState>>),
    /// Lazily-evaluated constant
    LazyConst(crate::gc::Cc<RefCell<LazyState>>),
}

impl Default for RantyVar {
    fn default() -> Self {
        Self::ByVal(RantyValue::Nothing)
    }
}

impl Clone for RantyVar {
    /// Creates a copy of the variable, preserving references.
    fn clone(&self) -> Self {
        match self {
            Self::ByVal(val) => Self::ByVal(val.clone()),
            Self::ByRef(val_ref) => Self::ByRef(val_ref.clone()),
            Self::ByValConst(val) => Self::ByValConst(val.clone()),
            Self::ByRefConst(val_ref) => Self::ByRefConst(val_ref.clone()),
            Self::Lazy(state) => Self::Lazy(state.clone()),
            Self::LazyConst(state) => Self::LazyConst(state.clone()),
        }
    }
}

impl RantyVar {
    #[inline]
    pub fn new_lazy(thunk: LazyThunk, is_const: bool) -> Self {
        let state = gc::alloc(RefCell::new(LazyState::Pending(thunk)));
        if is_const {
            Self::LazyConst(state)
        } else {
            Self::Lazy(state)
        }
    }

    /// Returns `true` if the variable is a constant.
    #[inline]
    pub fn is_const(&self) -> bool {
        matches!(
            self,
            Self::ByValConst(_) | Self::ByRefConst(_) | Self::LazyConst(_)
        )
    }

    /// Returns `true` if the variable is by-value.
    #[inline]
    pub fn is_by_val(&self) -> bool {
        matches!(self, Self::ByVal(_) | Self::ByValConst(_))
    }

    /// Returns `true` if the variable is by-reference.
    #[inline]
    pub fn is_by_ref(&self) -> bool {
        matches!(
            self,
            Self::ByRef(_) | Self::ByRefConst(_) | Self::Lazy(_) | Self::LazyConst(_)
        )
    }

    /// Converts the variable to its by-reference counterpart.
    #[inline]
    pub fn make_by_ref(&mut self) {
        if self.is_by_ref() {
            return;
        }
        match mem::take(self) {
            Self::ByVal(val) => *self = Self::ByRef(gc::alloc(RefCell::new(val))),
            Self::ByValConst(val) => *self = Self::ByRefConst(gc::alloc(val)),
            Self::Lazy(state) => *self = Self::Lazy(state),
            Self::LazyConst(state) => *self = Self::LazyConst(state),
            _ => unreachable!(),
        }
    }

    /// Attempts to write the specified value to the variable.
    /// If the variable is a constant, returns `false`; otherwise, returns `true`.
    #[inline]
    pub fn write(&mut self, value: RantyValue) -> bool {
        match self {
            Self::ByVal(val) => *val = value,
            Self::ByRef(val_ref) => {
                val_ref.replace(value);
            }
            Self::Lazy(state) => {
                state.replace(LazyState::Ready(value));
            }
            Self::ByRefConst(_) | Self::ByValConst(_) | Self::LazyConst(_) => return false,
        }
        true
    }

    #[inline]
    pub fn as_lazy_state(&self) -> Option<crate::gc::Cc<RefCell<LazyState>>> {
        match self {
            Self::Lazy(state) | Self::LazyConst(state) => Some(state.clone()),
            _ => None,
        }
    }

    /// Returns a cloned copy of the contained value.
    #[inline]
    pub fn value_ref(&self) -> RantyValue {
        match self {
            Self::ByVal(val) | Self::ByValConst(val) => val.clone(),
            Self::ByRef(val_ref) => val_ref.borrow().clone(),
            Self::ByRefConst(val_ref) => val_ref.as_ref().clone(),
            Self::Lazy(state) | Self::LazyConst(state) => match &*state.borrow() {
                LazyState::Ready(val) => val.clone(),
                _ => panic!("attempted to read an unresolved lazy variable directly"),
            },
        }
    }

    /// Returns a copy of the variable value.
    #[inline]
    pub fn value_cloned(&self) -> RantyValue {
        match self {
            RantyVar::ByVal(val) | RantyVar::ByValConst(val) => val.clone(),
            RantyVar::ByRef(val_ref) => val_ref.borrow().clone(),
            RantyVar::ByRefConst(val_ref) => val_ref.as_ref().clone(),
            RantyVar::Lazy(state) | RantyVar::LazyConst(state) => match &*state.borrow() {
                LazyState::Ready(val) => val.clone(),
                _ => panic!("attempted to clone an unresolved lazy variable directly"),
            },
        }
    }
}
