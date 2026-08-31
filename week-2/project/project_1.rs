/*fn main() {
	let p:f64 = 520,000,000.00;
	let r:f64 = 10%;
	let n:f64 = 5;

    //To find compound interest
    let a = p * (1 + (r/100)) ^ n
    println!("The amount is {}", a);
    let ci = a -p;
    println!("The compound interest is N{}", ci);
}*/
fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;
	let t:f64 = 1.00;

    //To find compound interest
    let a = p * (1.0 + (r/100.00 * t)).powf(n * t);
    println!("The amount is N{}", a);
    let ci = a - p;
    println!("The compound interest is N{}", ci);
}