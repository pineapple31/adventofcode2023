use std::fs::read_to_string;
use std::char;

fn main() {
    let file_path = "./input.txt";
    let binding = read_to_string(file_path)
        .unwrap();
    let content : Vec<&str>= binding
        .as_str()
        .lines()
        .collect();
    let mut sum: u32=0;
    for line in content {
        let index1 = line.find(char::is_numeric).unwrap();
        let index2 = line.rfind(char::is_numeric).unwrap();
        let first = line.chars().nth(index1).unwrap();
        let last = line.chars().nth(index2).unwrap();
        let mut number: String = String::from("");
        number.push(first);
        number.push(last);
        let num = number.parse::<u32>().unwrap();
        sum += num;
    }

    println!("{:?}", sum);

    println!("File {file_path} :");
}
