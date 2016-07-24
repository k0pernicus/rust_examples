use std::fmt::Debug;

trait MyDebug : Debug {
    fn print_debug(&self) {
        println!("{:?}", self);
    }
}

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

#[derive(Debug)]
struct Rectangle<T> {
    height: T,
    width: T,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> MyDebug for T {
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn print_area<T : HasArea>(shape: T) {
    println!("You area is {0}", shape.area());
}

fn main() {
    let c = Circle{x: 1.5f64, y: 2f64, radius: 8f64};
    let s = Square{side: 4f64};
    let r = Rectangle{height: 5, width: 5};
    let mut p = Point{x: 1, y: 2};

    c.print_debug();
    s.print_debug();
    p.print_debug();

    print_area(c);
    print_area(s);

    println!("Rectange is square? {0}", r.is_square());

    p.swap();

    p.print_debug();
}
