# complewise_iter
The complewise_iter crate provides a custom iterator designed to facilitate mutable access to the current element in a collection while simultaneously providing an immutable view of the remaining elements (the complement set). This iterator is particularly useful in scenarios where an element needs to be modified in the context of the rest of the collection.

### Example
```rust
use complewise_iter::IntoComplewiseIterator;

/// Print each element and the complement set of that element.
fn main() {
    let mut items = vec![1, 2, 3, 4, 5];
    items.complewise().for_each(|item, complement| {
        println!("Item: {}, Complement: {:?}", item, complement.collect::<Vec<_>>());
    });

}
// Output:
// 1 [2, 3, 4, 5]
// 2 [1, 3, 4, 5]
// 3 [1, 2, 4, 5]
// 4 [1, 2, 3, 5]
// 5 [1, 2, 3, 4]
```

```rust
use complewise_iter::IntoComplewiseIterator;

/// Calculate the new value of each element in the list by adding the sum of the other elements to it,
/// updating the list in-place with each step so that each calculation uses the most recently updated values.
fn main() {
    let mut items = vec![1, 2, 3, 4, 5];
    items.complewise().for_each(|item, complement| {
        *item += complement.into_iter().sum::<i32>();
    });
}
// Result:
// [15, 29, 56, 109, 214] (1+2+3+4+5, 2+15+3+4+5, 3+15+29+4+5, ...)
```

### Use Cases
The ComplewiseIterator is versatile and can be applied in various scenarios, including:

 - Simulations and Games: Update the state of an entity based on the states of others in the simulation.
 - AI Behavior Calculations: Modify an AI agent's behavior by considering the states of other entities.
 - Environmental Effects: Apply changes to elements based on the overall state of the environment.
 - Algorithms: Implement complex algorithms that require context-aware operations on elements.
 - Evaluate every element in the set against every other element (or a filtered sub-set via filtering of the complement iterator) and update values accordingly.

### How It Works
The iterator yields a mutable reference to the current element and an iterator over immutable references of the complement set. The complement consists of all other elements in the original set, excluding the current one. This allows algorithms to modify the current element while having read-only access to the rest of the collection.

### Lending Iterator
The ComplewiseIter implements the LendingIterator trait from the [gat-lending-iterator](https://github.com/Crazytieguy/gat-lending-iterator/) crate. It cannot implement the standard Iterator trait because it would require the lifetime of the mutable reference to be tied to the lifetime of the iterator, which is not possible without GATs, which were not implemented when the standard Iterator trait was designed.

The LendingIterator trait does not allow the syntax sugar of for loops, so it must be used with for_each or the manual `next` method.

Thanks to adapters provided by the gat-lending-iterator crate, most Iterator methods should work on a LendingIterator. However, due to issues, I have not been able to create an example of this yet.

### Examples
Try out the examples. e.g., 

```
cargo run --example print
```

### Benchmarks

A simple test compared the performance of this iterator to a manual c-style nested for loop using pointers.

```
test_complewise_iter    time:   [125.52 ps 127.14 ps 129.00 ps]
test_c_style            time:   [122.56 ps 123.17 ps 123.81 ps]
```

### Install

Add the following to your cargo.toml
```
[dependencies]
complewise_iter = { git = "https://github.com/CraigBuilds/complewise_iter" }
```

### Comparison to windows_mut

https://docs.rs/lending-iterator/0.1.7/lending_iterator/index.html#examples

window_mut yields sub-lists that move step-by-step over the parent list. complewise_iter can be seen as a variant where the window size is the entire list but the focus shifts.
