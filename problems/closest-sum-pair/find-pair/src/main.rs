use engine::Elements;
use std::io;

fn main() {
    println!("Press q to quit from the program any time.");

    let steps: u32 = turns();

    for _ in 1..steps + 1 {
        let desired_sum: i32 = desired_sum();

        let len: usize = init_size();

        let mut list: Vec<i32> = listing(len);

        use_engine(&mut list, len, desired_sum);
    }
}

fn turns() -> u32 {
    return loop {
        let mut steps: String = String::new();

        println!("Number of turns:");

        match io::stdin().read_line(&mut steps) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
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
}

fn desired_sum() -> i32 {
    return loop {
        let mut desired_sum: String = String::new();

        println!("Desired sum value:");

        match io::stdin().read_line(&mut desired_sum) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        let desired_sum: i32 = match desired_sum.trim().parse() {
            Ok(num) => num,

            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        break desired_sum;
    };
}

fn init_size() -> usize {
    return loop {
        let mut len: String = String::new();

        println!("Size of list: ");

        match io::stdin().read_line(&mut len) {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        let len: usize = match len.trim().parse() {
            Ok(num) => num,

            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        break len;
    };
}

fn listing(len: usize) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();

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

    list
}

fn use_engine(list: &mut Vec<i32>, len: usize, desired_sum: i32) {
    let mut elements: Elements = Elements::new(list, len, desired_sum);

    elements
        .sort_list()
        .find_init_distance()
        .find_pair()
        .result();
}
