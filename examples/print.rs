use complewise_iter::{IntoComplewiseIterator, LendingIterator};

/// Print each element and the complement set of that element.
fn main() {
    let mut items = vec![1, 2, 3, 4, 5];

    let mut iter = items.complewise();
    while let Some((item, others)) = iter.next() {
        println!("{} {:?}", item, others.collect::<Vec<_>>());
    }
}