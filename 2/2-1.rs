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
        let mut range = s.next().ok_or()?.split("-");
        let range_low = i32::from_str(range.next().unwrap()).unwrap();
        let range_high = i32::from_str(range.next().unwrap()).unwrap();
        let key = s.next().unwrap().chars().nth(0).unwrap();
        let pw = s.next().unwrap();
        let mut count = 0;
        for c in pw.chars() {
            if c == key {
                count += 1;
            }
        }
        if range_low <= count && count <= range_high {
            result += 1;
        }
    }
    println!("{}", result);
    Ok(())
}
