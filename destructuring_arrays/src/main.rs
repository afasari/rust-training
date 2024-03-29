// #[rustfmt::skip]
// fn main() {
//     let triple = [0, -2, 3];
//     println!("Tell me about {triple:?}");
//     match triple {
//         [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
//         [1, ..]   => println!("First is 1 and the rest were ignored"),
//         _         => println!("All elements were ignored"),
//     }
// }

// Destructuring of slices of unknown length also works with patterns of fixed length.

fn main() {
    inspect(&[0, -2, 3]);
    inspect(&[0, -2, 3, 4]);
}

#[rustfmt::skip]
fn inspect(slice: &[i32]) {
    println!("Tell me about {slice:?}");
    match slice {
        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        &[1, ..]   => println!("First is 1 and the rest were ignored"),
        _          => println!("All elements were ignored"),
    }
}