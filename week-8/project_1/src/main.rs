use std::io;

fn main() {
    println!("Hello user to Federal Government's Nigerian APS level checker.");
    println!("Here is a list of our staff.");

    let mut name = String::new();
    println!("Enter your name user: ");
    io::stdin().read_line(&mut name).expect("Not a valid name."); 
    println!("Ok {}.\nHere is a list of our staff.", name.trim());

    println!("Office Admin,\nAcademic,\nLawyer,\nTeacher");

    loop { 
        let mut role = String::new();
        println!("\nInput your office role (or type 'done' to exit): ");
        io::stdin().read_line(&mut role).expect("Failed to read line");
        let role = role.trim().to_uppercase();
     
        if role == "DONE" {
             break;
        }

        if role == "OFFICE ADMIN" {
             office_admin();
        } else if role == "ACADEMIC" {
             academic();
        } else if role == "LAWYER" {
             lawyer();
        } else if role == "TEACHER" {
             teacher();
        } else {
             println!("Unknown role entered. Please choose from the list.");
        }
    } 
    choice(); 
}

fn office_admin() {
    let position: Vec<&str> = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let experience_levels: [&str; 6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let mut input1 = String::new();
    println!("Enter your years of experience: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let experience = input1.trim().to_lowercase();

    let mut index: usize = 16;

    if experience == "1-2" || experience == "1-2 yrs" || experience == "1-2 years" {
        index = 0;
    } else if experience == "3-5 years" || experience == "3-5" || experience == "3-5 yrs" {
        index = 1; 
    } else if experience == "5-8 years" || experience == "5-8" || experience == "5-8 yrs" {
        index = 2;
    } else if experience == "8-10" || experience == "8-10 yrs" || experience == "8-10 years" {
        index = 3;
    } else if experience == "10-13" || experience == "10-13 yrs" || experience == "10-13 years" {
        index = 4;
    } else if experience == "ses" {
        index = 5;
    }

    if index < 16 {
        println!("Public servant level is: {}", experience_levels[index]);
        println!("Office Administration Position is: {}", position[index]);
    } else {
        println!("Invalid experience input typed.");
    }
}

fn academic() {
    let position: Vec<&str> = vec!["N/A", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let experience_levels: [&str; 6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    
    let mut input1 = String::new();
    println!("Enter your years of experience: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let experience = input1.trim().to_lowercase();

    let mut index: usize = 16;

    if experience == "1-2" || experience == "1-2 yrs" || experience == "1-2 years" {
        index = 0;
    } else if experience == "3-5 years" || experience == "3-5" || experience == "3-5 yrs" {
        index = 1; 
    } else if experience == "5-8 years" || experience == "5-8" || experience == "5-8 yrs" {
        index = 2;
    } else if experience == "8-10" || experience == "8-10 yrs" || experience == "8-10 years" {
        index = 3;
    } else if experience == "10-13" || experience == "10-13 yrs" || experience == "10-13 years" {
        index = 4;
    } else if experience == "ses" {
        index = 5;
    }

    if index < 16 {
        println!("Academic Lecturer level is: {}", experience_levels[index]);
        println!("Academic Position is: {}", position[index]); // Fixed label typo
    } else {
        println!("Invalid experience input typed.");
    }
}

fn lawyer() {
    let position: Vec<&str> = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let experience_levels: [&str; 6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let mut input1 = String::new();
    println!("Enter your years of experience: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let experience = input1.trim().to_lowercase();

    let mut index: usize = 16;

    if experience == "1-2" || experience == "1-2 yrs" || experience == "1-2 years" {
        index = 0;
    } else if experience == "3-5 years" || experience == "3-5" || experience == "3-5 yrs" {
        index = 1; 
    } else if experience == "5-8 years" || experience == "5-8" || experience == "5-8 yrs" {
        index = 2;
    } else if experience == "8-10" || experience == "8-10 yrs" || experience == "8-10 years" {
        index = 3;
    } else if experience == "10-13" || experience == "10-13 yrs" || experience == "10-13 years" {
        index = 4;
    } else if experience == "ses" {
        index = 5;
    }

    if index < 16 {
        println!("Public servant level is: {}", experience_levels[index]);
        println!("Lawyer Position is: {}", position[index]);
    } else {
        println!("Invalid experience input typed.");
    }
}

fn teacher() {
    let position: Vec<&str> = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let experience_levels: [&str; 6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    
    let mut input1 = String::new();
    println!("Enter your years of experience: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let experience = input1.trim().to_lowercase();

    let mut index: usize = 16;

    if experience == "1-2" || experience == "1-2 yrs" || experience == "1-2 years" {
        index = 0;
    } else if experience == "3-5 years" || experience == "3-5" || experience == "3-5 yrs" {
        index = 1; 
    } else if experience == "5-8 years" || experience == "5-8" || experience == "5-8 yrs" {
        index = 2;
    } else if experience == "8-10" || experience == "8-10 yrs" || experience == "8-10 years" {
        index = 3;
    } else if experience == "10-13" || experience == "10-13 yrs" || experience == "10-13 years" {
        index = 4;
    } else if experience == "ses" {
        index = 5;
    }

    if index < 16 {
        println!("Public servant level is: {}", experience_levels[index]);
        println!("Teacher Position is: {}", position[index]); // Fixed label typo
    } else {
        println!("Invalid experience input typed.");
    }
}

fn choice() {
    loop {
        println!("Are we done with the interview? (Y/N)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");
        let response = response.trim().to_uppercase();
     
        if response == "Y" {
            println!("Thank you for using the APS level checker. Goodbye!");
            break;
        }
    }
}