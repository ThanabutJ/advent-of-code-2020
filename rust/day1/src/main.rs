use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part2()
}

fn part2() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut increases: i32 = 0;
        let mut r1;
        let mut r2 = -1;
        let mut r3 = -1;
        let mut r4 = -1;

        for line in lines {
            if let Ok(reading) = line {
                match reading.parse::<i32>() {
                    Ok(n) => {
                        r1 = r2;
                        r2 = r3;
                        r3 = r4;
                        r4 = n;
                        if r1 != -1 && r4 != -1 && r1 < r4{
                            increases += 1
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        println!("increases {}", increases);
    }
}

fn part1() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut increases: i32 = 0;
        let mut last_reading: i32 = -1;
        for line in lines {
            if let Ok(reading) = line {
                match reading.parse::<i32>() {
                    Ok(n) => {
                        if last_reading != -1 && n > last_reading {
                            increases += 1
                        }
                        last_reading = n
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        println!("increases {}", increases);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
