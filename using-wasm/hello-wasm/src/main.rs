// import a fxn from another file
use any_name::add_numbers;

fn main() {
    let result = add_numbers(25, 10);
    let result2: i32 = run_add(20, 15);
    println!("25 + 10 = {}", result);
    println!("20 + 15 = {}", result2);
}

// when returning a value, rust will take the last statement in the function and return that
fn run_add(n1: i32, n2:i32) -> i32{
    n1 + n2
}