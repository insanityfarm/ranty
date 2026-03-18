use crate::gc::{self, Cc, Context, Finalize, Trace};
use crate::{InternalString, RantyList, RantyValue};
use fnv::FnvHashMap;
use std::{borrow::Cow, cell::RefCell, ops::Deref};

/// Reference handle for a Ranty map
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantyMapHandle(Cc<RefCell<RantyMap>>);

impl RantyMapHandle {
    pub fn cloned(&self) -> Self {
        Self(gc::alloc(RefCell::new((*self.0.borrow()).clone())))
    }

    #[inline]
    pub(crate) fn ptr_id(&self) -> usize {
        (&*self.0 as *const RefCell<RantyMap>) as usize
    }

    #[inline]
    pub(crate) fn downgrade(&self) -> crate::gc::Weak<RefCell<RantyMap>> {
        self.0.downgrade()
    }

    pub fn would_create_proto_cycle(&self, proto: &RantyMapHandle) -> bool {
        let mut next_proto = Some(RantyMapHandle::clone(proto));
        while let Some(cur_proto) = next_proto {
            if &cur_proto == self {
                return true;
            }
            next_proto = cur_proto.borrow().proto();
        }
        false
    }
}

impl PartialEq for RantyMapHandle {
    fn eq(&self, other: &Self) -> bool {
        Cc::ptr_eq(&self.0, &other.0)
    }
}

impl From<RantyMap> for RantyMapHandle {
    #[inline]
    fn from(map: RantyMap) -> Self {
        Self(gc::alloc(RefCell::new(map)))
    }
}

impl Deref for RantyMapHandle {
    type Target = RefCell<RantyMap>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

/// Represents Ranty's `map` type, which stores a mutable collection of key-value pairs.
/// Map keys are always strings.
#[derive(Debug, Clone)]
pub struct RantyMap {
    /// The physical contents of the map
    map: FnvHashMap<InternalString, RantyValue>,
    /// The prototype of the map
    proto: Option<RantyMapHandle>,
}

impl RantyMap {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
            proto: None,
        }
    }

    #[inline]
    pub fn into_handle(self) -> RantyMapHandle {
        RantyMapHandle::from(self)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[inline]
    pub fn raw_len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn proto(&self) -> Option<RantyMapHandle> {
        self.proto.clone()
    }

    #[inline]
    pub fn extend<M: Deref<Target = RantyMap>>(&mut self, other: M) {
        for (k, v) in other.map.iter() {
            self.map.insert(k.clone(), v.clone());
        }
    }

    #[inline]
    pub fn set_proto(&mut self, proto: Option<RantyMapHandle>) {
        self.proto = proto;
    }

    #[inline]
    pub fn raw_set(&mut self, key: &str, val: RantyValue) {
        self.map.insert(InternalString::from(key), val);
    }

    #[inline]
    pub fn raw_remove(&mut self, key: &str) {
        self.map.remove(key);
    }

    #[inline]
    pub fn raw_take(&mut self, key: &str) -> Option<RantyValue> {
        self.map.remove(key)
    }

    #[inline]
    pub fn raw_get(&self, key: &str) -> Option<&RantyValue> {
        self.map.get(key)
    }

    #[inline]
    pub fn get(&self, key: &str) -> Option<Cow<'_, RantyValue>> {
        // Check if the member is in the map itself
        if let Some(member) = self.raw_get(key) {
            return Some(Cow::Borrowed(member));
        }

        // Climb the prototype chain to see if the member is in one of them
        let mut next_proto = self.proto.as_ref().map(RantyMapHandle::clone);
        while let Some(cur_proto) = next_proto {
            let cur_proto_ref = cur_proto.borrow();
            if let Some(proto_member) = cur_proto_ref.raw_get(key) {
                return Some(Cow::Owned(proto_member.clone()));
            }
            next_proto = cur_proto_ref.proto.as_ref().map(RantyMapHandle::clone);
        }
        None
    }

    #[inline]
    pub fn raw_has_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    #[inline]
    pub fn raw_keys(&self) -> RantyList {
        self.map
            .keys()
            .map(|k| RantyValue::String(k.as_str().into()))
            .collect()
    }

    #[inline]
    pub fn raw_values(&self) -> RantyList {
        self.map.values().cloned().collect()
    }

    #[inline]
    pub(crate) fn raw_pairs_internal(&self) -> impl Iterator<Item = (&'_ str, &'_ RantyValue)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v))
    }
}

impl Default for RantyMap {
    fn default() -> Self {
        RantyMap::new()
    }
}

unsafe impl Trace for RantyMap {
    fn trace(&self, ctx: &mut Context<'_>) {
        self.proto.trace(ctx);
        for value in self.map.values() {
            value.trace(ctx);
        }
    }
}

impl Finalize for RantyMap {}
