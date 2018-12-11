use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[allow(dead_code)]
mod day2;

fn main() {
    let f = File::open("day2.txt").unwrap();
    let reader = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    for r in day2::strings_with_one_char_diff(&input).into_iter() {
        println!("{}", r);
    }
}
