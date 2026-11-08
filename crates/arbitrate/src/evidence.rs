use std::collections::HashMap;
use std::fmt::Display;
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
/// Each must impl [`fmt::Display`].
///
/// For simple usage, [`String`]s can be used (alias provided via [`StringEvidence`]).
/// This comes at the cost of allocation overhead.
///
/// Alternatively, you can substitute dedicated `T` and `I` with zero-sized types which impl [`fmt::Display`].
/// Doing so will prevent storage (and defer rendering) of text until needed (e.g. by a [`Report`]).
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
    T: Display + Hash,
    I: Display,
{
    /// Borrows the inner [`HashMap`]
    #[must_use]
    pub fn inner(&self) -> &HashMap<T, Vec<I>> {
        &self.0
    }
}
impl<T: Eq + Hash, I> Evidence<T, I>
{
    /// Adds a new piece of evidence under the given topic
    pub fn add_piece(&mut self, topic: T, evidence: I) {
        self.0.entry(topic).or_default().push(evidence);
    }
    /// Adds multiple pieces of evidence under the given topic, provided using any standard collection or iter for String
    pub fn add_bag(&mut self, topic: T, bag: impl IntoIterator<Item = I>) {
        self.0.entry(topic).or_default().extend(bag);
    }
    /// Merges in pieces of evidence spanning multiple topics
    pub fn add_locker(&mut self, locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>) {
        for (topic, bag) in locker {
            self.add_bag(topic, bag);
        }
    }

    /// Builder-style alternative of [`add_piece`]
    #[must_use]
    pub fn with_piece(mut self, topic: T, evidence: I) -> Self {
        self.add_piece(topic, evidence);
        self
    }
    /// Builder-style alternative of [`add_bag`]
    #[must_use]
    pub fn with_bag(mut self, topic: T, bag: impl IntoIterator<Item = I>) -> Self {
        self.add_bag(topic, bag);
        self
    }
    /// Builder-style alternative of [`add_locker`]
    #[must_use]
    pub fn with_locker(mut self, locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>) -> Self {
        self.add_locker(locker);
        self
    }
}

// !- Type aliases

/// Evidence stored entirely with strings
///
/// This is less efficient than creating custom structs/enums with [`Display`] impls.
/// It is, however, much simpler to use.
pub type StringEvidence = Evidence<String, String>;
