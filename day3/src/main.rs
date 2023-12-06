use std::{fs::File, io::{BufReader, BufRead}};

fn part1() {
    println!("---- Part 1 ----");
    let file = BufReader::new(File::open("./input.txt").expect("Unable to open file"));
    let mut matrice: Vec<Vec<char>> = Vec::new();
    let mut line_matrice: Vec<char> = Vec::new();
    for line in file.lines() {
        for ch in line.expect("Unable to read line").chars() {
            line_matrice.push(ch);
            print!("{ch}");
        }
        print!("\n");
        matrice.push(line_matrice.clone());
        line_matrice.clear();
    }
    println!("{:?}",matrice[14][2]);
    println!("----------------");
}

fn main() {
    part1();
}
