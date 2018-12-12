use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[allow(dead_code)]
mod day3;

fn main() {
    let f = File::open("day3.txt").unwrap();
    let reader = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    println!("Day 3: # of square inches = {}", day3::square_inches(&input));
    println!("Day 3: ID# of solo claim = {}", day3::solo_claim(&input));
}
