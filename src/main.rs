mod peg;
mod utils;
use core::panic;
use std::env;

fn main() {
    use crate::peg::parse_peg;
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("invalid args")
    }
    let peg_file_path = &args[1];
    let output_file_path = &args[2];
    do_compile_rs(parse_peg, &peg_file_path, &output_file_path);
}

fn do_compile_rs(compile_method: fn(&str) -> String, input_path: &str, output_path: &str) {
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    let input = fs::read_to_string(input_path).unwrap();
    let content = "
    use crate::utils::*;
"
    .to_string()
        + &compile_method(&input)
        + "
 pub fn parse_peg(input: &str) -> String {
    let mut memo = Memo::new();
    let ans = parse_start(&mut memo, \"\".to_string(), input, 0);
    if ans.is_none() {
        return \"\".to_string();
    }
    memo.print_operations(input);
    ans.unwrap().value
}
";
    let mut file = File::create(output_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
