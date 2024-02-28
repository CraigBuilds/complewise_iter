use complewise_iter::IntoComplewiseIterator;

fn main() {
    let mut items = vec![1, 2, 3, 4, 5];

    items.complewise().for_each(|item, complement| {
        println!("Item: {}, Complement: {:?}", item, complement.collect::<Vec<_>>());
    });

}


