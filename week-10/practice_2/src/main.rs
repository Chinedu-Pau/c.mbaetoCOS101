fn main() {

    let v = vec![10, 20, 30];
    // vector v owns the object in the heap.

    let v2 = v;    // moves ownership to v2

    display(v2); // Correction use display(v2.clone());
    // v2 is moved to display and v2 is invalidated

    println!("In main {:?}", v2);
    //v2 is No longer usable here 
}

fn display(v:Vec<i32>) {
    println!("Inside display {:?}", v);
}

// Correction:
// note: consider changing this parameter type in function `display` to borrow instead if owning the value isn't necessary
//   --> src\main.rs:15:14
//    |
// 15 | fn display(v:Vec<i32>) {
//    |    -------   ^^^^^^^^ this parameter takes ownership of the value
//    |    |
//    |    in this function
// help: consider cloning the value if the performance cost is acceptable
//    |
//  8 |     display(v2.clone());
//    |               ++++++++