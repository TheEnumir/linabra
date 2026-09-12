use linabra::*;

fn main() {
    let v1 = vectorND![1.2, 3.0, 4.9];
    let v2 = VectorND::new([10.0, 30.0, 20.0]);
    println!("{:?}", &v1 - v2);
}