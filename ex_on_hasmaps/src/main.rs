// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;
fn main() {

    let mut dep_people= HashMap::new(); // we will fill this with Deparments and Employees

    let mut organization = 'register_new_people: loop {
        println!("Write the department in your company that is having a new employee:");
        
        let mut d1=String::new();
        io::stdin()
            .read_line(&mut d1)
            .expect("Failed to readline");
        let d1 = d1.trim().to_string(); // Convert &str to String
        
        println!("Write the name of the employee:");

        let mut p1=String::new();
        io::stdin()
            .read_line(&mut p1)
            .expect("Failed to readline");
        let p1 = p1.trim().to_string(); // Convert &str to String;

        dep_people.entry(d1.clone()).or_insert(Vec::new()).push(p1);

        
        loop {
            println!("Do you want to add more employes? Write either 'Yes' or 'No'.");
            let mut ans=String::new();
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to readline");
            let ans = ans.trim().to_lowercase(); // Convert to lowercase for case-insensitive comparison
            if  ans == "yes" {
                continue 'register_new_people;
            } else if ans == "no" {
                break 'register_new_people dep_people;
            } else {
                println!("Invalid input. Please enter 'yes' or 'no'");
                continue;
            }
        }
    };
    println!("{:?}",organization);//the employees are not necessarily sorted in alphabetical order

    //we sort the employees alphabetically
    for (_,employees) in organization.iter_mut() {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        *employees=sorted_employees;
    }

    println!("{:?}",organization);

}

