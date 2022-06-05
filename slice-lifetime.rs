// Initial Q: Don't understand the // cannot borrow `*slice` as mutable more than once at a time
#[derive(Debug)]
struct X {}

fn test(slice: &mut [X]) -> Option<&X> {
    for _i in 0..10 {
        for x in slice.into_iter() { // cannot borrow `*slice` as mutable more than once at a time
            return Some(x);
        }
        // break; // works without outer loop or with break
    }
    None
}

fn main() {
    let mut s = vec![X{}, X{}, X{}];
    let r = test(&mut s);
    println!("{:?}", r);
}

////// "Either it consumes it, and then it's invalid for next iteration, or it reborrows and the lifetime is shorter"
////// An answer where only first fn fails to compile
#[derive(Debug)]
struct X {}

pub fn same_lifetime<'a, F: Fn(&'a X)>(slice: &'a mut [X], f: F) {
    for _i in 0..10 {
        for x in slice.into_iter() { // cannot borrow `*slice` as mutable more than once at a time
            f(x);
        }
    }
}

pub fn shorter_lifetime<'a, F: for <'b> Fn(&'b X)>(slice: &'a mut [X], f: F) {
    for _i in 0..10 {
        for x in slice.into_iter() { // cannot borrow `*slice` as mutable more than once at a time
            f(x);
        }
    }
}
