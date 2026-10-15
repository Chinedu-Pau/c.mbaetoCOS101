use std::io;

fn main() {
    println!("Good day user. What is your name please?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a Valid String!");

    println!("Alright {}.Welcome to PAU's restaurant!", name.trim());
    println!("Here is a variety of our dishes with their prices. Feel free to try out any dish.");
    println!(" p, Poundo Yam/Edinkaiko Soup: N3,200\n f, Fried Rice & Chicken: N3,000\n a, Amala & Ewedu Soup: N2,500\n e, Eba & Egusi Soup: N2,000\n w White Rice and Stew: 2_500");

    let mut total_price = 0;

    println!("Enter dish codes one by one (e.g.,'p','f', 'a', 'e', 'w'). Type 'done' to finish:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let code = input.trim();

        // This line of code exits the loop when the user is done ordering
        if code == "done" {
            break;
        }

        // This lines of code maps the typed string code to its corresponding price using if / else if
        let price = if code == "p" {
            3_200
        } else if code == "f" {
            3_000
        } else if code == "a" {
            2_500
        } else if code == "e" {
            2_000
        } else if code == "w" {
            2_500
        } else {
            println!("Unknown code '{}', please try again.", code);
            0 // Add 0 for invalid inputs
        };

        total_price += price;
        println!("Current total: N{}", total_price);
    }

    println!("Final total amount: N{}", total_price);

    let new_total = total_price - (total_price * 5) / 100;
    
    if total_price > 10_000
    {
        println!("We will give you a 5% discount our dear customer. So your total is N{}", new_total);
    }
    else 
    {
        println!("Your total amount is: N{}", total_price);
    }
}