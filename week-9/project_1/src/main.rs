use std::io::Write;

fn main() { 
    
    let title  = "\nLarger,Stout,Non-Alcoholic";
    let data_1 = "\n33 Export,Legend,Maltina";
    let data_2 = "\nDesperados,Turbo King,Amstel Malta";
    let data_3 = "\nGoldberg, Williams, Malta Gold";
    let data_4 ="\nGulder,-- , Fayrouz";
    let data_5 = "\nHeineken, --, --";
    let data_6 = "\nStar, -- ,--";
    
    /* let title: [&str;3] = ["Larger","Stout","Non-Alcoholic"];
    let data_1: [&str;3] = ["33 Export","Legend","Maltina"];
    let data_2: [&str;3] = ["Desperados","Turbo King","Amstel Malta"];
    let data_3: [&str;3] = ["Goldberg","Williams","Malta Gold"];
    let data_4: [&str;3] = ["Gulder"," ", "Fayrouz"];
    let data_5: [&str;3] = ["Heineken", " ", " "];
    let data_6: [&str;3] = ["Star", " ", " "];*/

    /*let t = title;
    let d1 = data_1;
    let d2 = data_2;
    let d3 = data_3;
    let d4 = data_4;
    let d5 = data_5;
    let d6 = data_6;*/

    let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("Create failed.");
    file.write(title.as_bytes()).expect("Write failed");
    file.write(data_1.as_bytes()).expect("Write failed");
    file.write(data_2.as_bytes()).expect("Write failed");
    file.write(data_3.as_bytes()).expect("Write failed");
    file.write(data_4.as_bytes()).expect("Write failed");
    file.write(data_5.as_bytes()).expect("Write failed");
    file.write(data_6.as_bytes()).expect("Write failed");
    println!("\nData has been written to the file.")
}
