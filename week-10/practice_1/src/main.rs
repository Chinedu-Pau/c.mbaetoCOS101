fn main() {
    println!("Hello, world!");
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in the heap.

    //only a single variable owns the heap memory at any given time
    let v2 = v; // correction: let v2= v.clone()

//    consider cloning the value if the performance cost is acceptable
//  |
//  | let v2 = v.clone();
//  |           ++++++++

// warning: unused variable: `v2`
    // here two variables own the heap value,
    // two pointers to the same content is not allowed in rust

    // Rust is very smart in terms of memory access ,so it detects a race condition

    println!("{:?}", v );
}
