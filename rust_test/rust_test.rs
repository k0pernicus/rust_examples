#![feature(specialization)]

use std::fmt::Debug;

trait PrintDebug {
    fn print_debug(&self);
}

#[derive(Debug)]
struct Test;

#[derive(Debug)]
struct Test2;

struct Test3;

impl<T: Debug> PrintDebug for T {
    default fn print_debug(&self) {
        println!("DEBUG")
    }
}

impl PrintDebug for usize {
    fn print_debug(&self) { 
        println!("WRITE")
    }
}

impl PrintDebug for Test3 {
    fn print_debug(&self) {
        println!("TEST 3")
    }
}

fn print_something<T: PrintDebug>(value: &T) {
    value.print_debug()
}

fn main() {
    let test = Test;
    let test2 = Test2;
    let test3 = Test3;
    let test4 : usize = 12;
    print_something(&test);
    print_something(&test2);
    print_something(&test3);
    print_something(&test4);
}
