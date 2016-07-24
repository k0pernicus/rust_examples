use std::ops::Fn;

fn call_with_one<F, T>(x : T, y: T, some_closure: F) -> T
    where F: Fn(T, T) -> T {
        some_closure(x,y)
    }

fn main() {
    println!("{}", call_with_one(5.6, 6.2, |x, y| x + y));
    println!("{}", call_with_one(5, 6, |x, y| x + y));
}
