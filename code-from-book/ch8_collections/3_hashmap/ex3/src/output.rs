use std::io;
use std::collections::HashMap;


pub fn show(some_company: &HashMap<String, Vec<String>>) {
    println!(
        "Do you want to retrieve list of team members from any department?
Enter the name of the department for this purpose. 
If you want to see team members from all department, enter 'all'. 
If you want to quit press 'q'"
    );

    let mut input_show = String::new();

    io::stdin()
        .read_line(&mut input_show)
        .expect("Failed to read line");

    match &input_show[..] {
        "q\n" => println!("System is closing."),
        "all\n" => println!("{:?}", some_company),
        _ => {
            let department_name = String::from(input_show.trim());

            let result = some_company.get(&department_name);

            match result {
                Some(vec) => println!("{:?}", vec),
                None => println!("This department doesn't exist!"),
            }
        }
    }
}
