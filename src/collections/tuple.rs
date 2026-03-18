use crate::gc::{self, Cc, Finalize, Trace};
use crate::{value_eq, RantyList, RantyListHandle, RantyValue};
use std::{
    iter::FromIterator,
    ops::{Add, Deref},
};

/// Reference handle for a Ranty tuple
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyTupleHandle(Cc<RantyTuple>);

impl RantyTupleHandle {
    /// Makes a copy of the underlying tuple and returns a handle containing it.
    pub fn cloned(&self) -> Self {
        Self(gc::alloc((*self.0).clone()))
    }

    #[inline]
    pub(crate) fn ptr_id(&self) -> usize {
        (&*self.0 as *const RantyTuple) as usize
    }

    #[inline]
    pub(crate) fn downgrade(&self) -> crate::gc::Weak<RantyTuple> {
        self.0.downgrade()
    }
}

impl From<RantyTuple> for RantyTupleHandle {
    #[inline]
    fn from(tuple: RantyTuple) -> Self {
        Self(gc::alloc(tuple))
    }
}

impl Deref for RantyTupleHandle {
    type Target = RantyTuple;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl PartialEq for RantyTupleHandle {
    fn eq(&self, other: &Self) -> bool {
        let mut state = value_eq::ValueEqState::default();
        state.eq_tuple_handles(self, other)
    }
}

/// Represents Ranty's `tuple` type, which stores an ordered, immutable collection of values.
#[derive(Debug, Clone, Trace, Finalize, Default)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyTuple(Vec<RantyValue>);

impl RantyTuple {
    #[inline]
    pub fn new() -> Self {
        Self(vec![])
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn into_handle(self) -> RantyTupleHandle {
        RantyTupleHandle::from(self)
    }

    #[inline]
    pub fn to_ranty_list(&self) -> RantyList {
        RantyList::from(self.0.clone())
    }

    #[inline]
    pub fn into_ranty_list(self) -> RantyList {
        RantyList::from(self.0)
    }
}

impl From<Vec<RantyValue>> for RantyTuple {
    fn from(values: Vec<RantyValue>) -> Self {
        Self(values)
    }
}

impl PartialEq for RantyTuple {
    fn eq(&self, other: &Self) -> bool {
        value_eq::tuple_values_equal(self, other)
    }
}

impl Deref for RantyTuple {
    type Target = Vec<RantyValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromIterator<&'a RantyValue> for RantyTuple {
    fn from_iter<T: IntoIterator<Item = &'a RantyValue>>(iter: T) -> Self {
        let vec: Vec<RantyValue> = iter.into_iter().cloned().collect();
        Self(vec)
    }
}

impl FromIterator<RantyValue> for RantyTuple {
    fn from_iter<T: IntoIterator<Item = RantyValue>>(iter: T) -> Self {
        let vec: Vec<RantyValue> = iter.into_iter().collect();
        Self(vec)
    }
}

impl IntoIterator for RantyTuple {
    type Item = RantyValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Add for RantyTuple {
    type Output = RantyTuple;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter())
            .collect::<RantyTuple>()
    }
}

impl Add<&RantyTuple> for RantyTuple {
    type Output = RantyTuple;

    fn add(self, rhs: &RantyTuple) -> Self::Output {
        self.into_iter()
            .chain(rhs.iter().cloned())
            .collect::<RantyTuple>()
    }
}

impl Add<RantyList> for RantyTuple {
    type Output = RantyList;

    fn add(self, rhs: RantyList) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyList> for RantyTuple {
    type Output = RantyList;

    fn add(self, rhs: &RantyList) -> Self::Output {
        self.into_iter()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add<RantyTuple> for &RantyTuple {
    type Output = RantyTuple;

    fn add(self, rhs: RantyTuple) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.into_iter())
            .collect::<RantyTuple>()
    }
}

impl Add<&RantyTuple> for &RantyTuple {
    type Output = RantyTuple;

    fn add(self, rhs: &RantyTuple) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.iter().cloned())
            .collect::<RantyTuple>()
    }
}

impl Add<RantyList> for &RantyTuple {
    type Output = RantyList;

    fn add(self, rhs: RantyList) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyList> for &RantyTuple {
    type Output = RantyList;

    fn add(self, rhs: &RantyList) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add for RantyTupleHandle {
    type Output = RantyTupleHandle;

    fn add(self, rhs: Self) -> Self::Output {
        (&*self + &*rhs).into_handle()
    }
}

impl Add<RantyListHandle> for RantyTupleHandle {
    type Output = RantyListHandle;

    fn add(self, rhs: RantyListHandle) -> Self::Output {
        (&*self + &*rhs.borrow()).into_handle()
    }
}
