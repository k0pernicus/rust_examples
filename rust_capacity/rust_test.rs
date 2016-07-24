trait Everyone {
        fn capacity(&self);
    }
    impl <T> Everyone for T {
        fn capacity(&self) {
            println!("we are all one");
        }
    }

fn main() {
    let vec = vec![1,2,3];
    println!("{}", vec.capacity());
    Everyone::capacity(&vec);
}
