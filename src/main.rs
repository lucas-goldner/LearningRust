use crate::function_pointers::plus_one;

mod function_pointers;
mod hello;

fn main() {
    const X: usize = hello::add(2, 4);
    println!("{}", X);
    let p = plus_one;
    let number_with_one_more = p(5);
    println!("{}", number_with_one_more);
}
