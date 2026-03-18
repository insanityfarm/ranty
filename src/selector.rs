use std::{cell::RefCell, error::Error, fmt::Display, ops::Deref};

use crate::{
    gc::{self, Cc, Context, Finalize, Trace},
    rng::RantyRng,
    runtime::{IntoRuntimeResult, RuntimeError},
    RantyValue, TryFromRanty, ValueError,
};

/// Reference handle for a Ranty selector.
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantySelectorHandle(Cc<RefCell<RantySelector>>);

impl RantySelectorHandle {
    pub fn cloned(&self) -> Self {
        Self(gc::alloc(RefCell::new((*self.0.borrow()).clone())))
    }

    #[inline]
    pub(crate) fn ptr_id(&self) -> usize {
        (&*self.0 as *const RefCell<RantySelector>) as usize
    }
}

impl Deref for RantySelectorHandle {
    type Target = RefCell<RantySelector>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for RantySelectorHandle {
    fn eq(&self, other: &Self) -> bool {
        Cc::ptr_eq(&self.0, &other.0)
    }
}

impl From<RantySelector> for RantySelectorHandle {
    fn from(sel: RantySelector) -> Self {
        Self(gc::alloc(RefCell::new(sel)))
    }
}

/// Represents a Ranty selector instance used by the resolver to select block branches.
#[derive(Debug, Clone, Trace, Finalize)]
#[rust_cc(unsafe_no_drop)]
pub struct RantySelector {
    /// Mode of the selector
    mode: SelectorMode,
    /// Match value used by value-driven selectors.
    match_value: Option<RantyValue>,
    /// Current iteration of the selector
    index: usize,
    /// Element count of the selector
    count: usize,
    /// True if the pass is odd (used by modes with alternating pass behaviors, such as mirror modes)
    parity: bool,
    /// If set to true, keeps selecting the last selected index.
    frozen: bool,
    /// Jump table used by some selector modes (won't allocate if unused)
    jump_table: Vec<usize>,
}

impl RantySelector {
    /// Creates a new selector.
    #[inline]
    pub fn new(mode: SelectorMode) -> Self {
        Self {
            mode,
            match_value: None,
            index: 0,
            count: 0,
            frozen: false,
            parity: false,
            jump_table: Default::default(),
        }
    }

    /// Converts the instance into a handle.
    #[inline]
    pub fn into_handle(self) -> RantySelectorHandle {
        self.into()
    }

    #[inline]
    pub fn with_match_value(mut self, value: RantyValue) -> Self {
        self.match_value = Some(value);
        self
    }

    /// The mode assigned to the selector.
    #[inline]
    pub fn mode(&self) -> SelectorMode {
        self.mode
    }

    #[inline]
    pub fn match_value(&self) -> Option<&RantyValue> {
        self.match_value.as_ref()
    }

    /// The next index to be selected.
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// The number of block elements that this selector is initialized for.
    ///
    /// A value of 0 indicates that the selector is uninitialized (as selecting over 0 branches is impossible).
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }

    /// Indicates the parity state of the selector. Some selectors use two alternating passes (such as mirror modes).
    /// The parity indicates which of these passes is currently active.
    #[inline]
    pub fn parity(&self) -> bool {
        self.parity
    }

    /// Indicates whether the selector is frozen.
    #[inline]
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Sets the frozen state of the selector.
    #[inline]
    pub fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
    }

    /// Indicates whether the selector has been initialized with [`Selector::init`].
    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.count > 0
    }

    /// Initializes the selector state using the specified element count.
    #[inline]
    pub fn init(&mut self, rng: &RantyRng, elem_count: usize) -> Result<(), SelectorError> {
        if elem_count == 0 {
            return Err(SelectorError::InvalidElementCount(0));
        }

        self.count = elem_count;

        match self.mode {
            SelectorMode::Match => {
                return Err(SelectorError::UnsupportedOperation(
                    "match selectors are value-driven",
                ))
            }
            SelectorMode::Random | SelectorMode::One | SelectorMode::NoDouble => {
                self.index = rng.next_usize(elem_count);
            }
            SelectorMode::Forward
            | SelectorMode::ForwardClamp
            | SelectorMode::ForwardMirror
            | SelectorMode::Ping
            | SelectorMode::Reverse
            | SelectorMode::ReverseClamp
            | SelectorMode::ReverseMirror
            | SelectorMode::Pong => {
                self.index = 0;
            }
            SelectorMode::Deck
            | SelectorMode::DeckLoop
            | SelectorMode::DeckClamp
            | SelectorMode::DeckMirror => {
                self.shuffle(rng);
            }
        }

        Ok(())
    }

    /// SHuffles the branch indices in the selector's jump table.
    #[inline]
    fn shuffle(&mut self, rng: &RantyRng) {
        let jump_table = &mut self.jump_table;
        let n = self.count;

        // Populate the jump table if it isn't already
        if jump_table.is_empty() {
            jump_table.reserve(n);
            jump_table.extend(0..n);
        }

        // Perform a Fisher-Yates shuffle
        for i in 0..n {
            jump_table.swap(i, rng.next_usize(n));
        }
    }

    /// Returns the next branch index and advances the selector state.
    pub fn select(&mut self, elem_count: usize, rng: &RantyRng) -> Result<usize, SelectorError> {
        // Initialize and sanity check
        if self.mode == SelectorMode::Match {
            return Err(SelectorError::UnsupportedOperation(
                "match selectors do not support cursor selection",
            ));
        }
        if !self.is_initialized() {
            self.init(rng, elem_count)?;
        } else if elem_count != self.count {
            return Err(SelectorError::ElementCountMismatch {
                expected: self.count,
                found: elem_count,
            });
        }

        let cur_index = self.index;
        let cur_index_inv = self.count.saturating_sub(cur_index).saturating_sub(1);

        macro_rules! next_index {
            ($v:expr) => {
                if !self.frozen {
                    self.index = $v;
                }
            };
        }

        macro_rules! next_parity {
            ($v:expr) => {
                if !self.frozen {
                    self.parity = $v;
                }
            };
        }

        // Iterate the selector
        match self.mode {
            SelectorMode::Random => {
                next_index!(rng.next_usize(elem_count));
            }
            SelectorMode::One => {}
            mode @ (SelectorMode::Forward | SelectorMode::Reverse) => {
                next_index!((cur_index + 1) % elem_count);
                if mode == SelectorMode::Reverse {
                    return Ok(cur_index_inv);
                }
            }
            mode @ (SelectorMode::ForwardClamp | SelectorMode::ReverseClamp) => {
                next_index!((cur_index + 1).min(elem_count - 1));
                if mode == SelectorMode::ReverseClamp {
                    return Ok(cur_index_inv);
                }
            }
            mode @ (SelectorMode::ForwardMirror | SelectorMode::ReverseMirror) => {
                let prev_parity = self.parity;
                if (prev_parity && cur_index == 0) || (!prev_parity && cur_index == elem_count - 1)
                {
                    next_parity!(!prev_parity);
                } else if self.parity {
                    next_index!(cur_index.saturating_sub(1));
                } else {
                    next_index!((cur_index + 1) % elem_count);
                }
                if mode == SelectorMode::ReverseMirror {
                    return Ok(cur_index_inv);
                }
            }
            SelectorMode::Deck => {
                // Store the return value before reshuffling to avoid accidental early duplicates
                let jump_index = self.jump_table[cur_index];

                if !self.frozen {
                    if cur_index >= elem_count - 1 {
                        self.shuffle(rng);
                        next_index!(0);
                    } else {
                        next_index!(cur_index + 1);
                    }
                }

                return Ok(jump_index);
            }
            SelectorMode::DeckMirror => {
                // Store the return value before reshuffling to avoid accidental early duplicates
                let jump_index = self.jump_table[cur_index];

                if !self.frozen {
                    let cur_parity = self.parity;

                    // Flip parity after boundary elements
                    if (cur_parity && cur_index == 0)
                        || (!cur_parity && cur_index >= elem_count - 1)
                    {
                        // If previous parity was odd, reshuffle
                        if cur_parity {
                            self.shuffle(rng);
                        }

                        next_parity!(!cur_parity);
                    } else if self.parity {
                        next_index!(cur_index.saturating_sub(1));
                    } else {
                        next_index!((cur_index + 1).min(elem_count - 1))
                    }
                }

                return Ok(jump_index);
            }
            SelectorMode::DeckLoop => {
                next_index!((cur_index + 1) % elem_count);
                return Ok(self.jump_table[cur_index]);
            }
            SelectorMode::DeckClamp => {
                next_index!((cur_index + 1).min(elem_count - 1));
                return Ok(self.jump_table[cur_index]);
            }
            mode @ (SelectorMode::Ping | SelectorMode::Pong) => {
                if !self.frozen {
                    let prev_parity = self.parity;
                    if (prev_parity && cur_index == 0)
                        || (!prev_parity && cur_index >= elem_count - 1)
                    {
                        next_parity!(!prev_parity);
                    }

                    if self.parity {
                        next_index!(cur_index.saturating_sub(1));
                    } else {
                        next_index!((cur_index + 1) % elem_count);
                    }
                }

                if mode == SelectorMode::Pong {
                    return Ok(cur_index_inv);
                }
            }
            SelectorMode::NoDouble => {
                next_index!(if elem_count > 1 {
                    (cur_index + 1 + rng.next_usize(elem_count - 1)) % elem_count
                } else {
                    0
                });
            }
            SelectorMode::Match => {
                return Err(SelectorError::UnsupportedOperation(
                    "match selectors do not support cursor selection",
                ));
            }
        }

        Ok(cur_index)
    }
}

/// Represents error states of a selector.
#[derive(Debug)]
pub enum SelectorError {
    /// The requested element count was different than what the selector was initialized for.
    ElementCountMismatch { expected: usize, found: usize },
    /// The specified element count is not supported.
    InvalidElementCount(usize),
    /// Selector operation is not supported.
    UnsupportedOperation(&'static str),
    /// Match selector could not find a selectable branch.
    NoMatchCandidates,
}

impl Error for SelectorError {}

impl Display for SelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectorError::ElementCountMismatch { expected, found } => write!(
                f,
                "selector expected {} elements, but found {}",
                expected, found
            ),
            SelectorError::InvalidElementCount(n) => {
                write!(f, "selector does not support blocks of size {}", n)
            }
            SelectorError::UnsupportedOperation(message) => write!(f, "{message}"),
            SelectorError::NoMatchCandidates => {
                write!(f, "match selector could not find a selectable branch")
            }
        }
    }
}

impl<T> IntoRuntimeResult<T> for Result<T, SelectorError> {
    fn into_runtime_result(self) -> super::RuntimeResult<T> {
        self.map_err(|err| RuntimeError {
            error_type: super::RuntimeErrorType::SelectorError(err),
            description: None,
            stack_trace: None,
        })
    }
}

/// Defines available branch selection modes for selectors.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum SelectorMode {
    /// Selects a random element each time.
    Random,
    /// Selects the same, random element each time.
    One,
    /// Selects each element in a wrapping sequence from left to right.
    Forward,
    /// Selects each element from left to right, then repeats the right-most element.
    ForwardClamp,
    /// Selects each element from left to right, then right to left, and repeats.
    /// Boundary elements are repeated.
    ForwardMirror,
    /// Selects each element in a wrapping reverse sequence from right to left.
    Reverse,
    /// Selects each element from right to left, then repeats the left-most element.
    ReverseClamp,
    /// Selects each element from right to left, then left to right, and repeats.
    /// Boundary elements are repeated.
    ReverseMirror,
    /// Selects each element once in a random sequence, then reshuffles.
    Deck,
    /// Selects each element once in a wrapping random sequence, without reshuffling.
    DeckLoop,
    /// Selects each element once in a random sequence, repeating the final element.
    DeckClamp,
    /// Selects each element once in a random sequence, then selects the same sequence backwards, then reshuffles and repeats.
    /// Mirror boundary elements are repeated.
    DeckMirror,
    /// Selects each element from left to right, switching directions each time a boundary element is reached.
    /// Boundary elements are not repeated.
    Ping,
    /// Selects each element from right to left, switching directions each time a boundary element is reached.
    /// Boundary elements are not repeated.
    Pong,
    /// Ensures that no one element index is selected twice in a row.
    NoDouble,
    /// Selects among block elements tagged with matching `@on` values.
    Match,
}

unsafe impl Trace for SelectorMode {
    #[inline(always)]
    fn trace(&self, _: &mut Context<'_>) {}
}

impl Finalize for SelectorMode {}

impl TryFromRanty for SelectorMode {
    fn try_from_ranty(val: RantyValue) -> Result<Self, ValueError> {
        match &val {
            RantyValue::String(s) => Ok(match s.as_str() {
                "random" => SelectorMode::Random,
                "one" => SelectorMode::One,
                "forward" => SelectorMode::Forward,
                "forward-clamp" => SelectorMode::ForwardClamp,
                "forward-mirror" => SelectorMode::ForwardMirror,
                "reverse" => SelectorMode::Reverse,
                "reverse-clamp" => SelectorMode::ReverseClamp,
                "reverse-mirror" => SelectorMode::ReverseMirror,
                "deck" => SelectorMode::Deck,
                "deck-loop" => SelectorMode::DeckLoop,
                "deck-clamp" => SelectorMode::DeckClamp,
                "deck-mirror" => SelectorMode::DeckMirror,
                "ping" => SelectorMode::Ping,
                "pong" => SelectorMode::Pong,
                "no-double" => SelectorMode::NoDouble,
                "match" => SelectorMode::Match,
                invalid => {
                    return Err(ValueError::InvalidConversion {
                        from: val.type_name(),
                        to: "selector mode",
                        message: Some(format!("invalid selector mode: '{invalid}'")),
                    })
                }
            }),
            _ => Err(ValueError::InvalidConversion {
                from: val.type_name(),
                to: "selector mode",
                message: None,
            }),
        }
    }
}
