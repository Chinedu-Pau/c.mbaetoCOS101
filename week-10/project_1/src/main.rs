#[derive(Debug)]
struct Laptop {
    brand: String,
    quantity:u32,
    desired_quantity:u32,
    individual_price:u32,
}

//logic to calculate the area of a rectangle
impl Laptop {
    fn cp(&self)->u32 {
        //using the . operator to fetch the value of a field via the self keyword.
        self.desired_quantity * self.individual_price
    }
}

fn main() {
    //initialize a structure
    let laptop_1 = Laptop {
        brand:String::from("HP"),
        quantity:10,
        desired_quantity:3,
        individual_price: 650_000,
    };
    let laptop_2 = Laptop{
        brand:String::from("IBM"),
        quantity:6,
        desired_quantity:3,
        individual_price:755_000,
    };
    let laptop_3 = Laptop {
        brand: String::from("Toshiba"),
        quantity:10,
        desired_quantity:3,
        individual_price:550_000
    };
    let laptop_4 = Laptop {
        brand: String::from("Dell"),
        quantity:4,
        desired_quantity:3,
        individual_price:850_000
    };
    println!("***Laptop_1***");
    println!("Cost price is: N{}", laptop_1.cp());

    println!("***Laptop_2***");
    println!("Cost price is: N{}", laptop_2.cp());

    println!("***Laptop_3***");
    println!("Cost price is: N{}",laptop_3.cp());

    println!("***Laptop_4***");
    println!("Cost price is: N{}",laptop_4.cp());

    let a = &laptop_1.cp();
    let b = &laptop_2.cp();
    let c = &laptop_3.cp();
    let d = &laptop_4.cp();

    let total_cp = a + b + c + d;
    println!("The total cost price for the laptops is: N{}",total_cp);  
}