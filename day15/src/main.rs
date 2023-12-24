use std::fs::read_to_string;

fn file_to_string(path: &str) -> String {
    let binding = read_to_string(path)
        .unwrap();
    binding
}

fn content_to_vec(content: &String) -> Vec<& str> {
    let list = content.split(',').collect();
    list
}

fn get_hash_algo(word: &str) -> u32 {
    let mut output = 0;
    for char in word.chars() {
        output += char as u32;
        output *= 17;
        output = output % 256;
    }
    output
}

fn part1() {
    let content = file_to_string("./input.txt");
    let list = content_to_vec(&content);
    let mut sum = 0;
    for word in &list {
        sum += get_hash_algo(word);
    }
    println!("sum hash : {sum}");

}


fn main() {
    part1();
}
