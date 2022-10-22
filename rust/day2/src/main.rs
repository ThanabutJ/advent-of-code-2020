use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part2()
}

fn part2() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut h_pos = 0;
        let mut d_pos = 0;
        let mut aim = 0;
        for line in lines {
            if let Ok(command) = line {
                let command: Vec<&str> = command.split(" ").collect();
                match command[1].parse::<u32>() {
                    Ok(units) => {
                        match command[0] {
                            "forward" => {
                                h_pos += units;
                                d_pos += units * aim;
                            },
                            "up" => {
                                aim -= units;
                            },
                            "down" => {
                                aim += units;
                            },
                            _ => {}
                        };
                    }
                    Err(_) => {}
                }
            }
        }

        println!("horizontal position = {}", h_pos);
        println!("Depth position = {}", d_pos);
        println!("result= {}", d_pos * h_pos);
    }
}

fn part1() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut h_pos = 0;
        let mut d_pos = 0;
        for line in lines {
            if let Ok(command) = line {
                let command: Vec<&str> = command.split(" ").collect();
                match command[1].parse::<u32>() {
                    Ok(units) => {
                        match command[0] {
                            "forward" => h_pos += units,
                            "up" => d_pos -= units,
                            "down" => d_pos += units,
                            _ => {}
                        };
                    }
                    Err(_) => {}
                }
            }
        }

        println!("horizontal position = {}", h_pos);
        println!("Depth position = {}", d_pos);
        println!("result= {}", d_pos * h_pos);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
