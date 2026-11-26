use std::io::Read;
use std::io;

fn main() {
    println!("Hello user, kindly input your position for globacom: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

        if input == "Administrator"{
            admin();
        } else if input == "Project manager" {
            project_manager();
        } else if input == "Employee" {
            employee();
        } else if input == "Customer" {
            customer();
        } else if input == "Vendor" {
            vendor();
        }

}
fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents2 = String::new();
    file.read_to_string(&mut contents2).unwrap();
    print!("{}", contents2);
}
fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents3 = String::new();
    file.read_to_string(&mut contents3).unwrap();
    print!("{}", contents3);
}

fn customer() {
    let mut file = std::fs::File::open("Customer_tb.sql").unwrap();
    let mut contents4 = String::new();
    file.read_to_string(&mut contents4).unwrap();
    print!("{}", contents4);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents5 = String::new();
    file.read_to_string(&mut contents5).unwrap();
    print!("{}", contents5);
}