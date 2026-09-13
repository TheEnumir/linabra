use linabra::*;

fn main() {
    let mut v1 = vectorND![1.2, 3.0, 4.9];
    println!("{:?}", -&mut v1);
}