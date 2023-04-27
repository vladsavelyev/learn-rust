// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    departments.insert(
        String::from("Engineering"),
        vec![String::from("Sally"), String::from("John")],
    );
    departments.insert(
        String::from("Sales"),
        vec![
            String::from("Amir"),
            String::from("John"),
            String::from("Ann"),
        ],
    );
    departments.insert(String::from("Bosses"), vec![String::from("Vlad")]);
    for (_, ppl) in &mut departments {
        ppl.sort();
    }

    loop {
        let mut cmd = String::new();
        println!("Please enter command:");
        io::stdin().read_line(&mut cmd).unwrap();
        let words = cmd.split_whitespace().collect::<Vec<&str>>();

        match words[0].to_lowercase().as_str() {
            "add" => {
                if words.len() < 4 {
                    println!("Add commands must be in format: Add <person> to <department>");
                }
                let person = String::from(words[1]);
                let dep = String::from(words[3]);
                let ppl = departments.entry(dep).or_insert(Vec::new());
                ppl.push(person);
                ppl.sort();
            }
            "list" => {
                if words.len() > 1 {
                    let dep = words[1];
                    if let Some(ppl) = &departments.get(dep) {
                        println!("People in {dep}: {:#?}", *ppl)
                    } else {
                        println!("No department named {dep} found")
                    }
                } else {
                    println!("All people in company by department:");
                    for (dep, ppl) in &departments {
                        println!("{dep}: {:#?}", ppl);
                    }
                }
            }
            "q" => {
                println!("Exiting");
                break;
            }
            _ => println!("Unknown command"),
        }
    }
}
