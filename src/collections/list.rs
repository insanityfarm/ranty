use crate::gc::{self, Cc, Finalize, Trace};
use crate::{value_eq, RantyTuple, RantyTupleHandle, RantyValue};
use std::{
    cell::RefCell,
    iter::FromIterator,
    ops::{Add, Deref, DerefMut},
};

/// Reference handle for a Ranty list
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyListHandle(Cc<RefCell<RantyList>>);

impl RantyListHandle {
    /// Makes a copy of the underlying list and returns a handle containing it.
    pub fn cloned(&self) -> Self {
        Self(gc::alloc(RefCell::new((*self.0.borrow()).clone())))
    }

    #[inline]
    pub(crate) fn ptr_id(&self) -> usize {
        (&*self.0 as *const RefCell<RantyList>) as usize
    }

    #[inline]
    pub(crate) fn downgrade(&self) -> crate::gc::Weak<RefCell<RantyList>> {
        self.0.downgrade()
    }
}

impl From<RantyList> for RantyListHandle {
    #[inline]
    fn from(list: RantyList) -> Self {
        Self(gc::alloc(RefCell::new(list)))
    }
}

impl Deref for RantyListHandle {
    type Target = RefCell<RantyList>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl PartialEq for RantyListHandle {
    fn eq(&self, other: &Self) -> bool {
        let mut state = value_eq::ValueEqState::default();
        state.eq_list_handles(self, other)
    }
}

/// Represents Ranty's `list` type, which stores an ordered, mutable collection of values.
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyList(Vec<RantyValue>);

impl RantyList {
    /// Creates an empty RantyList.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Creates an empty RantyList with the specified initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn into_handle(self) -> RantyListHandle {
        RantyListHandle::from(self)
    }

    #[inline]
    pub fn into_ranty_tuple(self) -> RantyTuple {
        RantyTuple::from(self.0)
    }

    #[inline]
    pub fn to_ranty_tuple(&self) -> RantyTuple {
        RantyTuple::from(self.0.clone())
    }
}

impl From<Vec<RantyValue>> for RantyList {
    fn from(list: Vec<RantyValue>) -> Self {
        Self(list)
    }
}

impl Default for RantyList {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for RantyList {
    fn eq(&self, other: &Self) -> bool {
        value_eq::list_values_equal(self, other)
    }
}

impl Deref for RantyList {
    type Target = Vec<RantyValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RantyList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<RantyValue> for RantyList {
    fn from_iter<T: IntoIterator<Item = RantyValue>>(iter: T) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl IntoIterator for RantyList {
    type Item = RantyValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Add for RantyList {
    type Output = RantyList;

    fn add(self, rhs: RantyList) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyList> for RantyList {
    type Output = RantyList;

    fn add(self, rhs: &RantyList) -> Self::Output {
        self.into_iter()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add<RantyTuple> for RantyList {
    type Output = RantyList;

    fn add(self, rhs: RantyTuple) -> Self::Output {
        self.into_iter()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyTuple> for RantyList {
    type Output = RantyList;

    fn add(self, rhs: &RantyTuple) -> Self::Output {
        self.into_iter()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add<RantyList> for &RantyList {
    type Output = RantyList;

    fn add(self, rhs: RantyList) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyList> for &RantyList {
    type Output = RantyList;

    fn add(self, rhs: &RantyList) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add<RantyTuple> for &RantyList {
    type Output = RantyList;

    fn add(self, rhs: RantyTuple) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.into_iter())
            .collect::<RantyList>()
    }
}

impl Add<&RantyTuple> for &RantyList {
    type Output = RantyList;

    fn add(self, rhs: &RantyTuple) -> Self::Output {
        self.iter()
            .cloned()
            .chain(rhs.iter().cloned())
            .collect::<RantyList>()
    }
}

impl Add for RantyListHandle {
    type Output = RantyListHandle;

    fn add(self, rhs: Self) -> Self::Output {
        (&*self.borrow() + &*rhs.borrow()).into_handle()
    }
}

impl Add<RantyTupleHandle> for RantyListHandle {
    type Output = RantyListHandle;

    fn add(self, rhs: RantyTupleHandle) -> Self::Output {
        (&*self.borrow() + &*rhs).into_handle()
    }
}
