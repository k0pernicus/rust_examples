use std::env;

fn main() {

    let mut args = env::args();

    let mut vec = vec![0,1,2,3,4,5];

    while let Some(x) = args.next() {
        println!("{}", x);
    }

    while let Some(x) = vec.pop() {
        println!("{}", x);
    }

}
