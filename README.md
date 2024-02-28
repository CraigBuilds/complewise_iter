# ComplewiseIter
The ComplewiseIter crate provides a custom iterator designed to facilitate mutable access to the current element in a collection while simultaneously providing an immutable view of the remaining elements (the complement set). This iterator is particularly useful in scenarios where an element needs to be modified in the context of the rest of the collection.

# Use Cases
The ComplewiseIterator is versatile and can be applied in various scenarios, including:

 - Simulations and Games: Update the state of an entity based on the states of others in the simulation.
 - AI Behavior Calculations: Modify an AI agent's behavior by considering the states of other entities.
 - Environmental Effects: Apply changes to elements based on the overall state of the environment.
 - Algorithms: Implement complex algorithms that require context-aware operations on elements.

# How It Works
The iterator yields a mutable reference to the current element and an immutable reference to the complement of the set. The complement consists of all other elements in the original set, excluding the current one. This allows algorithms to modify the current element while having read-only access to the rest of the collection.

# Lending Iterator
The ComplewiseIter implements the LendingIterator trait from the [gat-lending-iterator](https://github.com/Crazytieguy/gat-lending-iterator/) crate. It cannot implement the standard Iterator trait because it would require the lifetime of the mutable reference to be tied to the lifetime of the iterator, which is not possible without GATs, which were not implemented when then standard Iterator trait was designed.

The LendingIterator trait does not allow the syntax sugar of for loops, so must be used with the `next` method.

Thanks to adapters provided by the gat-lending-iterator crate, most Iterator methods should work on a LendingIterator. However, due to issues I have not been able to create an example of this yet.

## Examples
Try out the examples. e.g., 

```
cargo run --example print
```

### Benchmarks

A simple test compared the performance of this iterator to a manual c style nested for loop using pointers.

```
test_complewise_iter    time:   [125.52 ps 127.14 ps 129.00 ps]
test_c_style            time:   [122.56 ps 123.17 ps 123.81 ps]
```