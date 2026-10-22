use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Define the CSV header as a single string line
    let heading = "Commissioner, Ministry, Zone\n";

    let commisioner: Vec<String> = vec!["Algbogun Alamba Daudu".to_string(), "Murtala Afeez Bendu".to_string(), "Okorocha Calistus Ogbona".to_string(),"Adewale Jimoh Akanbi".to_string() ,"Osazuwa Faith Etieye".to_string()];

    let ministry: Vec<String> = vec!["Internal Affairs".to_string(), "Justice".to_string(), "Defense".to_string(),"Power & Steel".to_string(),"Petroleum".to_string()];
    
    let zone: Vec<String> = vec!["South West".to_string(),"North East".to_string(),"South South".to_string(),"South West".to_string(),"South East".to_string()];
    

    // Open the CSV file with create, write, and append options
    let mut file = OpenOptions::new().create(true).write(true).append(true).open("EFCC.csv").expect("Cannot open file");

    // 1. Write the header row first
    file.write_all(heading.as_bytes()).expect("Write failed");

    // 2. Loop through the student vectors to write each student's data row
    for i in 0..commisioner.len() {
        let row = format!("{},{},{}\n", commisioner[i], ministry[i], zone[i]);
        file.write_all(row.as_bytes()).expect("Write failed");
    }

    println!("Data successfully written to EFCC.csv!");
}
