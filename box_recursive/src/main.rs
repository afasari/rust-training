#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
// semacam linked list

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}