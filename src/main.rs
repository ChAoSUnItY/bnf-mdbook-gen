mod analyzer;

use analyzer::{entry::Entry, scanner};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Expect 1 argument for source .bnf file");

    let src = fs::read_to_string(file_name).expect(&format!("Unable to open file {}", file_name));

    let result = scanner::scan(&src);

    for r in result {
        print!("{:?}", Entry::new(&r));
    }
}
