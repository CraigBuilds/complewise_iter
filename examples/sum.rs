use complewise_iter::{IntoComplewiseIterator, LendingIterator};

/// Add the sum of the other elements to each element in the list, one at a time.
/// The effect of the previous sum will effect the next sum.
/// Use the complewise_iter crate, and a c-style method and ensure the results are the same.
fn main() {

    //using complewise_iter
    let mut items = vec![1,2,3,4,5];
    let mut iter = items.complewise();
    while let Some((item, others)) = iter.next() {
        *item += others.into_iter().sum::<i32>();
    }
    println!("{:?}", items); // [15, 29, 56, 109, 214]

    //using c-style
    let mut items = vec![1,2,3,4,5];
    let ptr = items.as_mut_ptr();
    for i in 0..items.len() {
        unsafe {
            let item: &mut i32 = ptr.add(i).as_mut().unwrap();
            for j in 0..items.len() {
                if i == j {
                    continue;
                }
                let other: &i32 = ptr.add(j).as_ref().unwrap();
                *item += *other;
            }
        }
    }
    println!("{:?}", items); // [15, 29, 56, 109, 214]
}