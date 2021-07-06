use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if &input[..] == "q\n" {
            break;
        }

        let mut word = String::from(input.trim());

        let mut chars = word.chars();

        match chars.next() {
            Some(c) => {
                let is_vowel = check_vowel(c);

                if is_vowel {
                    word.push_str("-fay")
                } else {
                    let first_char = word.remove(0);

                    word.push('-');

                    word.push(first_char);

                    word.push_str("ay");
                }
            }
            None => {
                println!("Empty String!");
                continue;
            }
        }

        println!("{}", word);
    }
}

fn check_vowel(c: char) -> bool {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for v in vowels.iter() {
        if &c == v {
            return true;
        }
    }

    return false;
}
