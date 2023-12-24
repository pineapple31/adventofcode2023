use std::{fs::File, io::{BufReader, BufRead}};

fn create_matrice(file: BufReader<File>) -> Vec<Vec<char>>{
    let mut matrice: Vec<Vec<char>> = Vec::new();
    let mut line_matrice: Vec<char> = Vec::new();
    for line in file.lines() {
        for ch in line.expect("Unable to read line").chars() {
            line_matrice.push(ch);
        }
        matrice.push(line_matrice.clone());
        line_matrice.clear();
    }
    matrice
}

fn find_galaxies (matrice: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize,usize)> = Vec::new();
    for (y, row) in matrice.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == '#' {
                galaxies.push((y,x));
            }
        }
    }
    galaxies
}

fn sum_lenghts (galaxies: Vec<(usize,usize)>) -> usize {
    let mut sum = 0;
    for galaxy in galaxies.clone() {
        for galaxy2 in galaxies.clone() {
            let mut diff_0 = 0;
            let mut diff_1 = 0;
            if !(galaxy.0 == galaxy2.0 && galaxy.1 == galaxy2.1) {
                if galaxy.0 > galaxy2.0 {
                    diff_0 = galaxy.0 - galaxy2.0;
                } else {
                    diff_0 = galaxy2.0 - galaxy.0;
                }
                if galaxy.1 > galaxy2.1 {
                    diff_1 = galaxy.1 - galaxy2.1;
                } else {
                    diff_1 = galaxy2.1 - galaxy.1;
                }
                sum += diff_0 + diff_1;
            }
        }
    }
    sum
}

fn part1() {
    println!("---- Part 1 ----");
    let file = BufReader::new(File::open("./input2.txt").expect("Unable to open file"));
    let matrice = create_matrice(file);
    let galaxies = find_galaxies(matrice.clone());
    let sum = sum_lenghts(galaxies.clone());
    println!("sum= {}",sum/2);
    println!("----------------");
}

fn main() {
    part1();
}