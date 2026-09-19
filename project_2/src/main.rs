use std::io;

fn main() {
   let mut name = String::new();
   let mut years_of_experience = String::new();

   println!("Enter your name: ");
   io::stdin().read_line(&mut name).expect("Not a valid string");

   println!("Enter your years of experience: ");
   io::stdin().read_line(&mut years_of_experience).expect("Not a valid string");
   let years_of_experience:u8 = years_of_experience.trim().parse().expect("Not a valid number");

   if years_of_experience >= 40
   {
    println!("{name} will earn N1_560_000.");
   }
   else if years_of_experience >= 30 && years_of_experience <= 39
   {
    println!("{name} will earn N1_480_000.");
   }
   else if years_of_experience >= 28 && years_of_experience <= 29 
   {
    println!("{name} will earn N1_300_000.");
   }
   else if years_of_experience == 0
   {
    println!("{name} will earn N100_000.");
   }
   else {
    println!("We will think of a suitable salary for you {name}.");
   }
}
