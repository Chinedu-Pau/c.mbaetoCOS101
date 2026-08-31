fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;
	let t:f64 = 1.00;

	//To find the amount to which the TV has depreciated to
	let a = p * (1.0 - (r/100.00 * t)). powf(n * t);
	println!("The amount to which the TV has depreciated to is N{}", a);
}