use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// !- Concrete types

/// Supporting evidence for the decision behind a vote
///
/// # Generics
///
/// - `T`: Topic
/// - `I` Item
///
/// Topics and evidence items are generic for storage flexibility
///
/// - Topics must impl [`Display`], [`Debug`], [`Eq`], and [`Hash`].
/// - Items must impl [`Display`]
///
/// For simple usage, [`String`]s can be used (alias provided via [`StringEvidence`]).
/// This comes at the cost of allocation overhead.
///
/// Alternatively, you can substitute dedicated `T` and `I` with zero-sized types which
/// have a [`Display`] impl.
/// Doing so will prevent storage (and defer rendering) of text until needed (e.g. during reporting)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "
    T: Deserialize<'de> + Hash + Eq,
    I: Deserialize<'de>
")))]
pub struct Evidence<T, I>(HashMap<T, Vec<I>>);

impl<T, I> Default for Evidence<T, I> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<T, I> Evidence<T, I>
where
    T: EvidenceTopic,
    I: EvidenceItem,
{
    /// Borrows the inner [`HashMap`]
    #[must_use]
    pub fn inner(&self) -> &HashMap<T, Vec<I>> {
        &self.0
    }

    /// Adds a new piece of evidence under the given topic
    pub fn add_piece(&mut self, topic: T, evidence: I) {
        self.0.entry(topic).or_default().push(evidence);
    }
    /// Adds multiple pieces of evidence under the given topic, provided using any standard collection or iter for the underlying item type
    pub fn add_bag(&mut self, topic: T, bag: impl IntoIterator<Item = I>) {
        self.0.entry(topic).or_default().extend(bag);
    }
    /// Merges in pieces of evidence spanning multiple topics
    pub fn add_locker(
        &mut self,
        locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>,
    ) {
        for (topic, bag) in locker {
            self.add_bag(topic, bag);
        }
    }

    /// Builder-style alternative to [`add_piece()`](Self::add_piece)
    #[must_use]
    pub fn with_piece(mut self, topic: T, evidence: I) -> Self {
        self.add_piece(topic, evidence);
        self
    }
    /// Builder-style alternative to [`add_bag()`](Self::add_bag)
    #[must_use]
    pub fn with_bag(mut self, topic: T, bag: impl IntoIterator<Item = I>) -> Self {
        self.add_bag(topic, bag);
        self
    }
    /// Builder-style alternative to [`add_locker()`](Self::add_locker)
    #[must_use]
    pub fn with_locker(
        mut self,
        locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>,
    ) -> Self {
        self.add_locker(locker);
        self
    }
}
// !- TODO: add FromIterator, IntoIterator impls

// !- Marker traits

// ! Topic

/// Marker trait - used to localize trait bounds
pub trait EvidenceTopic
where
    Self: Debug
        + Display
        + Hash
        + Eq,
{}

impl<T> EvidenceTopic for T
where
    T: Debug + Display
     // + Clone
     + Hash
     + Eq,
{}

// ! Item

/// Marker trait - used to localize trait bounds
pub trait EvidenceItem
where
    Self: Display
        // + Clone,
{}

impl<I> EvidenceItem for I
where
    I: Display
     // + Clone,
{}

// !- Type aliases

/// Evidence stored
///
/// This is less efficient than creating custom structs/enums with [`Display`] impls.
/// It is, however, simpler
pub type BoxDynEvidence<T> = Evidence<T, Box<dyn EvidenceItem>>;

/// Evidence stored entirely with strings
///
/// This is less efficient than creating custom structs/enums with [`Display`] impls.
/// It is, however, simpler
pub type StringEvidence = Evidence<String, String>;

