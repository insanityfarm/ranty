use std::{fmt::Debug, fmt::Display, ops::Add};

use once_cell::sync::OnceCell;
use smallvec::SmallVec;
use unicode_segmentation::UnicodeSegmentation;

use crate::{util, InternalString, RantyList, RantyTuple, RantyValue};

type Graphemes = SmallVec<[(usize, usize); 1]>;

/// Represents Ranty's `string` type.
#[derive(Debug, Default)]
pub struct RantyString {
    raw: InternalString,
    graphemes: OnceCell<Option<Graphemes>>,
}

impl RantyString {
    /// Creates a new, empty string.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn from_str(s: &str) -> Self {
        Self {
            raw: InternalString::from(s),
            ..Default::default()
        }
    }

    #[inline]
    fn from_char(c: char) -> Self {
        let mut s = InternalString::new();
        s.push(c);
        Self {
            raw: s,
            ..Default::default()
        }
    }
}

impl RantyString {
    #[inline]
    pub(crate) fn graphemes(&self) -> &Graphemes {
        self.graphemes
            .get_or_init(|| {
                Some(
                    UnicodeSegmentation::grapheme_indices(self.raw.as_str(), true)
                        .map(|(i, slice)| (i, i + slice.len()))
                        .collect::<Graphemes>(),
                )
            })
            .as_ref()
            .unwrap()
    }

    /// Creates a copy of the string with the graphemes in reverse order.
    #[inline]
    pub fn reversed(&self) -> Self {
        let mut buf = InternalString::new();
        for i in (0..self.len()).rev() {
            if let Some(g) = self.grapheme_at(i) {
                buf.push_str(g.as_str());
            }
        }

        Self {
            raw: buf,
            ..Default::default()
        }
    }

    /// Gets a reference to the string as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.raw.as_str()
    }

    /// Gets the grapheme string at the specified index.
    #[inline]
    pub fn grapheme_at(&self, index: usize) -> Option<RantyString> {
        if index >= self.len() {
            return None;
        }

        let (start, end) = self.graphemes()[index];
        Some(RantyString::from(&self.raw[start..end]))
    }

    /// Splits the string into individual graphemes and returns them as a Ranty list.
    #[inline]
    pub fn to_ranty_list(&self) -> RantyList {
        let n = self.len();
        let mut list = RantyList::with_capacity(n);
        for i in 0..n {
            let c = self.grapheme_at(i).unwrap();
            list.push(RantyValue::String(c));
        }
        list
    }

    /// Splits the string into individual graphemes and returns them as a Ranty tuple.
    #[inline]
    pub fn to_ranty_tuple(&self) -> RantyTuple {
        let n = self.len();
        let mut items = Vec::with_capacity(n);
        for i in 0..n {
            let c = self.grapheme_at(i).unwrap();
            items.push(RantyValue::String(c));
        }
        RantyTuple::from(items)
    }

    /// Gets the string at the specified slice.
    pub fn to_slice(&self, start: Option<usize>, end: Option<usize>) -> Option<RantyString> {
        let graphemes = self.graphemes();
        let len = graphemes.len();

        // Bounds checks
        if let Some(start) = start {
            if start > len {
                return None;
            }
        }

        if let Some(end) = end {
            if end > len {
                return None;
            }
        }

        Some(match (start, end) {
            (None, None) => self.clone(),
            (None, Some(end)) => {
                let raw_end = if end < len {
                    graphemes[end].0
                } else {
                    self.raw.len()
                };
                Self::from(&self.raw[..raw_end])
            }
            (Some(start), None) => {
                let raw_start = graphemes[start].0;
                Self::from(&self.raw[raw_start..])
            }
            (Some(start), Some(end)) => {
                let (start, end) = util::minmax(start, end);
                if start == end {
                    return Some(Self::default());
                }
                let raw_start = graphemes[start].0;
                let raw_end = if end < len {
                    graphemes[end].0
                } else {
                    self.raw.len()
                };
                Self::from(&self.raw[raw_start..raw_end])
            }
        })
    }
}

impl Clone for RantyString {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            ..Default::default()
        }
    }
}

impl RantyString {
    #[inline]
    pub fn len(&self) -> usize {
        self.graphemes().len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.graphemes().is_empty()
    }
}

impl From<InternalString> for RantyString {
    fn from(s: InternalString) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<&str> for RantyString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for RantyString {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl From<&String> for RantyString {
    fn from(s: &String) -> Self {
        Self::from_str(s)
    }
}

impl From<char> for RantyString {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

impl Add for RantyString {
    type Output = RantyString;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            raw: self.raw + rhs.raw,
            ..Default::default()
        }
    }
}

impl Display for RantyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.raw)
    }
}

impl PartialEq for RantyString {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl PartialOrd for RantyString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}
