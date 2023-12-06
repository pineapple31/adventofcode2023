
fn part1() {
    println!("---- Part 1 ----");
    let file_path = "./input.txt";
    let binding = read_to_string(file_path).unwrap();
    let content : Vec<&str>= binding
        .as_str()
        .lines()
        .collect();
}

fn main() {
    part1();
}
