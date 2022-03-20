extern crate ebnf;

mod analyzer;

use analyzer::{entry::Entry, scanner};
use std::{env, fs::{self, File}, io::Write, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Expect 1 argument for source .bnf file");
    let local_path = String::from(".");
    let output_dir = Path::new(args.get(2).unwrap_or(&local_path));

    let src = fs::read_to_string(file_name).expect(&format!("Unable to open file {}", file_name));

    let result = scanner::scan(&src);

    for r in result {
        let mut entry = Entry::new(&r);

        println!("{:#?}", entry.grammar);

        let mut output_file = File::create(Path::join(output_dir, format!("{}.md", entry.grammar.expressions[0].lhs))).expect("Unable to create output file");
        output_file.write(entry.get_content().join("\n").as_bytes()).expect("Unable to write content to ouput file");
    }
}
