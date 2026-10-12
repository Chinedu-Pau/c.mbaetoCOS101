use std::io;

fn main() 
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f32 = c.trim().parse().expect("Not a valid number");

    let mut d:f32 = b*b - 4.0*a*c;
    d = d.sqrt();
    println!("this is the condition of the values of d:");

    
    if d > 0.0
    {
        println!("d has two distinct roots");
    }
    else if d == 0.0 
    {
        println!("d has exactly one real root");
    }
    else{
        println!("d has no real roots");
    }


}
