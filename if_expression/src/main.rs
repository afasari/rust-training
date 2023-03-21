fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
    println!("{}",x);

    let mut y = 10;
    y = if y % 2 == 0 {
        y / 2
    } else {
        3 * y + 1
    };
    println!("{}",y);
}