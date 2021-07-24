use algorithm::Elements;
use std::io;

fn main() {
    println!("Press q to quit from the program any time.");

    let steps: u32 = loop {
        let mut steps: String = String::new();

        println!("Number of turns:");

        match io::stdin().read_line(&mut steps) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        if steps.trim() == "q" {
            return;
        }

        let steps: u32 = match steps.trim().parse() {
            Ok(num) => num,

            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        break steps;
    };

    for _ in 1..steps + 1 {
        let mut list: Vec<i32> = Vec::new();

        let mut len = String::new();

        let mut desired_sum: String = String::new();

        println!("Desired sum value:");

        match io::stdin().read_line(&mut desired_sum) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        if desired_sum.trim() == "q" {
            return;
        }

        let desired_sum: i32 = match desired_sum.trim().parse() {
            Ok(num) => num,

            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("Size of list: ");

        match io::stdin().read_line(&mut len) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        if len.trim() == "q" {
            return;
        }

        let len: usize = match len.trim().parse() {
            Ok(num) => num,

            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("Enter the numbers in list:");

        for _ in 0..len {
            let mut input_elem: String = String::new();

            let input_elem: String = match io::stdin().read_line(&mut input_elem) {
                Ok(_) => input_elem,

                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let input_elem: i32 = match input_elem.trim().parse() {
                Ok(num) => num,

                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            list.push(input_elem);
        }

        list.sort();

        let mut elements = Elements::new(&list, len, desired_sum);

        elements.find_init_distance().find_pair().result();

        //println!("{:?}", list);
    }
}
