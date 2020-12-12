use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::fs::File;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut result = 0;
    for line in reader.lines() {
        let line = line?;
        let mut s = line.split(" ");
        let mut range = s.next().unwrap().split("-");
        let pos_one= usize::from_str(range.next().unwrap()).unwrap() - 1;
        let pos_two = usize::from_str(range.next().unwrap()).unwrap() - 1;
        let key = s.next().unwrap().chars().nth(0).unwrap();
        let pw = s.next().unwrap();
        let mut chars = pw.chars();
        println!("{} {}", pos_one, pos_two);
        let char_one = chars.nth(pos_one).unwrap();
        let char_two = chars.nth(pos_two-pos_one-1).unwrap();
        if (key == char_one || key == char_two) &&
           (char_one != char_two) {
               result += 1;
        }
    }
    println!("{}", result);
    Ok(())
}
