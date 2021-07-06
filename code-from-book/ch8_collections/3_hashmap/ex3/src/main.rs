use std::collections::HashMap;

mod input;
mod output;

#[allow(unused_variables)]
fn main() {
    let mut some_company: HashMap<String, Vec<String>> = HashMap::new();

    input::initialize(&mut some_company);

    output::show(&some_company);
}
