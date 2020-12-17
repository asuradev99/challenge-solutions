use std::io::{stdin};
fn main() {
    let mut input =  String::new();
    let mut line: i8 = 0; 
    while input != "END" {
        line += 1;
        stdin()
            .read_line(&mut input)
            .expect("error reading input");
        let pattern = input
            .split("*")
            .filter(|&i| i.contains("."))
            .collect::<Vec<&str>>();
        let c = pattern
            .iter()
            .all(|j| j.len() == pattern[0].len());
        if c || pattern.len() == 0 {
            println!("{} EVEN", line);
        } else {
            println!("{} NOT EVEN", line);
        }
    }
}
