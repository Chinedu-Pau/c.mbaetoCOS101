use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() ->io::Result<()> {
  let mut efcc = OpenOptions::new().create(true).append(true).open("EFCC.csv")?;  
    let ministry = read("Ministry.csv").unwrap();
    let comissioner = read("Commissioner.csv").unwrap();
    let zone = read("Zone.csv").unwrap();

    for i in 0..comissioner.len(){
        let row = format!("{},{},{}", comissioner[i], ministry[i], zone[i]);
        writeln!(efcc, "{}" ,row);

        // println!("{}, {}, {}",ministry[i], comissioner[i], zone[i] );
    }
    Ok(())

}

fn read (filepath: &str) -> io::Result<Vec::<String>> {
let f = File::open(filepath)?;
let buf = BufReader::new(f);

let mut content:Vec::<String>  = vec![];
for line in buf.lines() {
    content.push(line?);
}
    return Ok(content);
}
