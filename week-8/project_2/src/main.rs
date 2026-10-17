use std::io;

fn main() {
    println!("EY Nigeria Interview Tracker");

        let mut input = String::new();
        println!("Please input your name interviewer: ");
        std::io::stdin().read_line(&mut input).expect("That's not a String.");
        let interviewer = input.trim(); 

    loop {
        println!("\nShould we begin interviewing interviewer {}? (Y/N)\nType N to terminate the code and Y to continue it." ,interviewer);
        let mut input_1 = String::new();
        std::io::stdin().read_line(&mut input_1).expect("That's not a String.");
        let response = input_1.trim();

        if response == "Y" {
            println!("Alright let's begin.");
            interview();
        }
        else if response == "N" {
            println!("Thank you for using this user.");
            println!("Terminating the program");
            break;
        } 
        else {
            println!("Invalid response");
        }
    }
}

fn interview() {
    let mut names: Vec<String> = Vec::new();
    let mut experiences: Vec<i32> = Vec::new();

    // 1. Get the number of candidates
    let mut input = String::new();
    println!("Enter the number of candidates: ");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let num_candidates: usize = input.trim().parse().unwrap_or(0);

    // 2. Collect names and experiences together in one loop
    let mut i = 0;
    while i < num_candidates {
        let mut name_input = String::new();
        println!("\nEnter name for candidate {}: ", i + 1);
        io::stdin().read_line(&mut name_input).expect("Failed to read input.");
        let name_str = name_input.trim().to_string(); // Fixed: used to_string() here
        names.push(name_str);

        let mut exp_input = String::new();
        println!("Enter years of experience for {}: ", names[i]);
        io::stdin().read_line(&mut exp_input).expect("Failed to read input.");
        let exp: i32 = exp_input.trim().parse().unwrap_or(0);
        experiences.push(exp);

        i += 1;
    }

    // 3. Print out all candidates and their experience
    println!("\nInterviewee's Information:");
    
    let mut j = 0;
    while j < names.len() {
        println!("{}. The interviewee {} has {} years experience", j + 1, names[j], experiences[j]);
        j += 1;
    }

    // 4. Find the candidate with the maximum experience
    if !experiences.is_empty() { // Fixed: checks if it is NOT empty
        let mut max_exp = experiences[0];
        let mut max_index = 0;

        let mut k = 1;
        while k < experiences.len() {
            if experiences[k] > max_exp {
                max_exp = experiences[k];
                max_index = k;
            }
            k += 1;
        }

        let best_candidate = &names[max_index]; // Fixed: reference names vector instead of c

        println!("\nTop Candidate is: ");
        println!("{} with {} years of experience!", best_candidate, max_exp);
    }
}