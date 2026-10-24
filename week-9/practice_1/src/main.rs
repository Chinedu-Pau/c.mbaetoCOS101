use  std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    //This File is a Struct data type and is used under fs(file system).
    //In the file the create meethod creates any type of file desired .csv
    //Edge case handling in case it case it caould not ceate the desired file.
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("Write failed");
    file.write_all(dept.as_bytes()).expect("Write failed");
    println!("\nData written to file.");

}
