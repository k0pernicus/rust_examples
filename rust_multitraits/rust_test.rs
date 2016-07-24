use std::fmt::Debug;

trait Foo {
    fn foo(&self);
}

trait Foobar : Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("FOO"); }
}

impl Foobar for Baz {
    fn foobar(&self) { println!("FOOBAR"); }
}

fn foo<T,K>(x:T, y:K)
    where T: Clone,
          K: Clone + Debug {

    x.clone();
    y.clone();
    println!("{:?}", y);

}

fn main() {
    foo("Hello", "World");
    let b = Baz {};
    b.foobar();
}
