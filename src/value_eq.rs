use std::collections::HashSet;

use crate::{RantyList, RantyListHandle, RantyTuple, RantyTupleHandle, RantyValue};

#[derive(Default)]
pub(crate) struct ValueEqState {
    seen_lists: HashSet<(usize, usize)>,
    seen_tuples: HashSet<(usize, usize)>,
}

impl ValueEqState {
    #[inline]
    pub(crate) fn eq_values(&mut self, lhs: &RantyValue, rhs: &RantyValue) -> bool {
        match (lhs, rhs) {
            (RantyValue::Nothing, RantyValue::Nothing) => true,
            (RantyValue::String(a), RantyValue::String(b)) => a == b,
            (RantyValue::Int(a), RantyValue::Int(b)) => a == b,
            (RantyValue::Int(a), RantyValue::Float(b)) => *a as f64 == *b,
            (RantyValue::Float(a), RantyValue::Float(b)) => a == b,
            (RantyValue::Float(a), RantyValue::Int(b)) => *a == *b as f64,
            (RantyValue::Boolean(a), RantyValue::Boolean(b)) => a == b,
            (RantyValue::Range(ra), RantyValue::Range(rb)) => ra == rb,
            (RantyValue::List(a), RantyValue::List(b)) => self.eq_list_handles(a, b),
            (RantyValue::Tuple(a), RantyValue::Tuple(b)) => self.eq_tuple_handles(a, b),
            (RantyValue::Map(a), RantyValue::Map(b)) => a == b,
            (RantyValue::Selector(a), RantyValue::Selector(b)) => a == b,
            _ => false,
        }
    }

    pub(crate) fn eq_list_handles(&mut self, lhs: &RantyListHandle, rhs: &RantyListHandle) -> bool {
        if lhs.ptr_id() == rhs.ptr_id() {
            return true;
        }

        if !self
            .seen_lists
            .insert(normalize_pair(lhs.ptr_id(), rhs.ptr_id()))
        {
            return true;
        }

        let lhs = lhs.borrow();
        let rhs = rhs.borrow();
        self.eq_iter(lhs.iter(), rhs.iter())
    }

    pub(crate) fn eq_tuple_handles(
        &mut self,
        lhs: &RantyTupleHandle,
        rhs: &RantyTupleHandle,
    ) -> bool {
        if lhs.ptr_id() == rhs.ptr_id() {
            return true;
        }

        if !self
            .seen_tuples
            .insert(normalize_pair(lhs.ptr_id(), rhs.ptr_id()))
        {
            return true;
        }

        self.eq_iter(lhs.iter(), rhs.iter())
    }

    #[inline]
    pub(crate) fn eq_list_values(&mut self, lhs: &RantyList, rhs: &RantyList) -> bool {
        self.eq_iter(lhs.iter(), rhs.iter())
    }

    #[inline]
    pub(crate) fn eq_tuple_values(&mut self, lhs: &RantyTuple, rhs: &RantyTuple) -> bool {
        self.eq_iter(lhs.iter(), rhs.iter())
    }

    fn eq_iter<'a, I, J>(&mut self, lhs: I, rhs: J) -> bool
    where
        I: ExactSizeIterator<Item = &'a RantyValue>,
        J: ExactSizeIterator<Item = &'a RantyValue>,
    {
        if lhs.len() != rhs.len() {
            return false;
        }

        lhs.zip(rhs).all(|(lhs, rhs)| self.eq_values(lhs, rhs))
    }
}

#[inline]
pub(crate) fn values_equal(lhs: &RantyValue, rhs: &RantyValue) -> bool {
    let mut state = ValueEqState::default();
    state.eq_values(lhs, rhs)
}

#[inline]
pub(crate) fn list_values_equal(lhs: &RantyList, rhs: &RantyList) -> bool {
    let mut state = ValueEqState::default();
    state.eq_list_values(lhs, rhs)
}

#[inline]
pub(crate) fn tuple_values_equal(lhs: &RantyTuple, rhs: &RantyTuple) -> bool {
    let mut state = ValueEqState::default();
    state.eq_tuple_values(lhs, rhs)
}

#[inline]
fn normalize_pair(lhs: usize, rhs: usize) -> (usize, usize) {
    if lhs <= rhs {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    }
}
