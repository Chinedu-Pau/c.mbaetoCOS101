use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Define the CSV header as a single string line
    let heading = "Student Name, Matric.Number, Department, Level\n";

    let name: Vec<String> = vec!["Oluchi Mordi".to_string(), "Adams Aliyu".to_string(), "Shania Bolade".to_string(), "Adekunle Gold".to_string(), "Blanca Edemeh".to_string()];

    let matric_number: Vec<String> = vec!["ACC10211111".to_string(),"ECO10110101".to_string(), "CSC103228828".to_string(),"EEE11020202".to_string(),"MEE10202001".to_string()];
    
    let department: Vec<String> = vec!["Accounting".to_string(),"Economics".to_string(),"Computer".to_string(),"Electrical".to_string(),"Mechanical".to_string()];
    
    let level: Vec<u16> = vec![300, 100, 200, 200, 100];

    // Open the CSV file with create, write, and append options
    let mut file = OpenOptions::new().create(true).write(true).append(true).open("Attendance.csv").expect("Cannot open file");

    // 1. Write the header row first
    file.write_all(heading.as_bytes()).expect("Write failed");

    // 2. Loop through the student vectors to write each student's data row
    for i in 0..name.len() {
        let row = format!("{},{},{},{}\n", name[i], matric_number[i], department[i], level[i]);
        file.write_all(row.as_bytes()).expect("Write failed");
    }

    println!("Data successfully written to Attendance.csv!");
}

