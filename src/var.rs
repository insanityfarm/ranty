use std::{cell::RefCell, mem, ops::Deref};

use crate::gc::{self, Finalize, Trace};
use crate::RantyValue;

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
        }
    }
}

impl RantyVar {
    /// Returns `true` if the variable is a constant.
    #[inline]
    pub fn is_const(&self) -> bool {
        matches!(self, Self::ByValConst(_) | Self::ByRefConst(_))
    }

    /// Returns `true` if the variable is by-value.
    #[inline]
    pub fn is_by_val(&self) -> bool {
        matches!(self, Self::ByVal(_) | Self::ByValConst(_))
    }

    /// Returns `true` if the variable is by-reference.
    #[inline]
    pub fn is_by_ref(&self) -> bool {
        matches!(self, Self::ByRef(_) | Self::ByRefConst(_))
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
            Self::ByRefConst(_) | Self::ByValConst(_) => return false,
        }
        true
    }

    /// Returns an immutable reference to the contained value.
    #[inline]
    pub fn value_ref(&self) -> impl Deref<Target = RantyValue> + '_ {
        match self {
            Self::ByVal(val) | Self::ByValConst(val) => cervine::Cow::Borrowed(val),
            Self::ByRef(val_ref) => cervine::Cow::Owned(val_ref.borrow()),
            Self::ByRefConst(val_ref) => cervine::Cow::Borrowed(val_ref.as_ref()),
        }
    }

    /// Returns a copy of the variable value.
    #[inline]
    pub fn value_cloned(&self) -> RantyValue {
        match self {
            RantyVar::ByVal(val) | RantyVar::ByValConst(val) => val.clone(),
            RantyVar::ByRef(val_ref) => val_ref.borrow().clone(),
            RantyVar::ByRefConst(val_ref) => val_ref.as_ref().clone(),
        }
    }
}
