pub use gat_lending_iterator::LendingIterator;
use std::iter::Chain;
use core::slice::Iter;

/// `ComplewiseIterator` is a custom iterator that allows mutable access to the current element
/// while providing an immutable view of the rest of the elements in a slice.
/// 
/// This iterator is particularly useful for algorithms that require modifying the current
/// element based on the context provided by the other elements in the collection.
///
/// For example, In simulations or games, this iterator could be used to update the state of an entity
/// based on the states of other entities in the simulation, useful in AI behavior calculations
/// or environmental effects.
///
/// In algorithms, a sliding window is a sub-list that moves step-by-step over the parent list.
/// While typically used for calculations over sub-lists, this concept can be seen as a variant where
/// the window size is the entire list but the focus shifts.
///
/// This iterator yields a mutable reference to the current element of a set and an immutable reference to the
/// complement of the set. The complement is the set of all elements in the original set except the current element.
/// This is comparable to a pairwise iterator, but instead of yielding a reference to the next element, it yields
/// a reference to the complement set of the element. i.e complement-wise (complewise).
pub struct ComplewiseIterator<'slice, T> {
    /// A mutable reference to the slice being iterated over.
    slice: &'slice mut [T],
    /// The current position within the slice.
    index: usize,
}

/// Implementation of the `LendingIterator` trait from the `gat_lending_iterator` crate.
/// This LendingIterator has adapters so it is interoperable with other iterators.
impl<'slice, T> LendingIterator for ComplewiseIterator<'slice, T> {
    /// Specifies the output of the iterator as a tuple containing a mutable reference to
    /// the current element and a chain iterator over the remaining elements.
    type Item<'a> = (&'a mut T, Chain<Iter<'a, T>, Iter<'a, T>>) where Self: 'a;

    /// Advances the iterator and returns a tuple of a mutable reference to the current element
    /// and an iterator over the rest of the elements. This allows the caller to modify the current
    /// element while reading the rest of the collection.
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.slice.len() {
            let (preceding, subsequent) = self.slice.split_at_mut(self.index);
            let (current_value, subsequent) = subsequent.split_first_mut().unwrap();
            let chain_of_rest = preceding.iter().chain(subsequent.iter());
            self.index += 1;
            Some((current_value, chain_of_rest))
        } else {
            None
        }
    }
}

/// A trait that provides a method to convert a mutable slice into a `ComplewiseIterator`.
pub trait IntoComplewiseIterator<'a, T> {
    fn complewise(self) -> ComplewiseIterator<'a, T>;
}

/// This is so any mutable slice can be converted into a `ComplewiseIterator`.
/// e.g
/// let mut items = vec![1, 2, 3, 4, 5];
/// let mut iter = items.complewise();
impl<'a, T> IntoComplewiseIterator<'a, T> for &'a mut [T] {
    fn complewise(self) -> ComplewiseIterator<'a, T> {
        ComplewiseIterator {
            slice: self,
            index: 0,
        }
    }
}