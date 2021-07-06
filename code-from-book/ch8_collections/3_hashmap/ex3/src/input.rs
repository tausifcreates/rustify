use std::io;
use std::collections::HashMap;

pub fn initialize(some_company: &mut HashMap<String, Vec<String>>) {
    println!(
        "Enter a depratment name and then also
    enter team members for the department.\nPress 'q' to quit."
    );

    loop {
        let mut input_department = String::new();

        let mut members: Vec<String> = Vec::new();

        println!("Department name:");

        io::stdin()
            .read_line(&mut input_department)
            .expect("Failed to read line");

        // break from the io system if "q" is pressed
        if &input_department[..] == "q\n" {
            break;
        }

        let department_name = String::from(input_department.trim());

        println!("Team members:");

        loop {
            let mut input_person: String = String::new();

            io::stdin()
                .read_line(&mut input_person)
                .expect("Failed to read line");

            // break from entering members to the team
            // if "q" is pressed
            if &input_person[..] == "q\n" {
                break;
            }

            let person = String::from(input_person.trim());

            members.push(person);
        }

        some_company.entry(department_name).or_insert(members);
    }
}
