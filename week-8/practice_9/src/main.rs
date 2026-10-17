fn main() {

    let b:(i32,bool,f64) = (110, true, 10.9);
    println!("{:?}", b);
}

//pass the tuple as a parameter

fn output(x:(i32,bool,f64)) {
    println!("Inside print method");
    println!("{:?}",x);
}
/*
 It has issues
fn output(x:(i32,bool,f64)) {
  |    ^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `practice_9` (bin "practice_9") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target\debug\practice_9.exe`
_b
 */
