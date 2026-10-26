fn main() {

    let v = vec![20, 40, 60, 80];
    // vector v owns the object in the heap

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}", v); //Solution is to println!("In main {:?}", v2_return);
}

fn display(v:Vec<i32>)->Vec<i32> {
    // returning the same vector
    println!("inside display {:?}", v);
    return v;
}

// 8 |     println!("In main {:?}", v);
//   |                              ^ value borrowed here after move
//   |
// help: consider cloning the value if the performance cost is acceptable
//   |
// 6 |     let v2 = v.clone();
//   |               ++++++++

// warning: unused variable: `v2_return`
//  --> src\main.rs:7:9
//   |
// 7 |     let v2_return = display(v2);
//   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_v2_return`
