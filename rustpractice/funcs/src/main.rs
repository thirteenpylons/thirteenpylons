use std::io

fn main() {
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}