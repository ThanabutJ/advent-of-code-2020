use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut v = vec![0; 12];
        let mut count = 0;

        for line in lines {
            count += 1;
            if let Ok(binary) = line {
                for (i, b) in binary.chars().enumerate() {
                    if b == '1' {
                        v[i] += 1;
                    }
                }
            }
        }

        
        let mut gamma_rate = String::from("");
        let mut epsilon_rate = String::from("");
        for i in v {
            if i > count / 2 {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            } else {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            }
        }

        let i_gamma_rate = i32::from_str_radix(gamma_rate.trim(), 2).expect("Not a binary number!");
        let i_epsilon_rate = i32::from_str_radix(epsilon_rate.trim(), 2).expect("Not a binary number!");
        println!("gamma {} {}", gamma_rate, i_gamma_rate);
        println!("epsilon {} {}", epsilon_rate, i_epsilon_rate);
        println!("answer {}", i_gamma_rate * i_epsilon_rate);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
