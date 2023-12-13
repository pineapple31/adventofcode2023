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

fn find_start_coord (matrice: Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in matrice.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == 'S' {
                return (y,x);
            }
        }
    }
    (0,0)
}

fn follow_path(matrice: Vec<Vec<char>>,start_pos: (usize,usize)) -> u32 {
    let mut line = start_pos.0+1;
    let mut char = start_pos.1;
    let mut last_position = 'N';
    let mut nb_steps = 1;
    while matrice[line][char] != 'S' {
        // print!("{},{} : {} _{}\t",line,char,matrice[line][char], last_position);
        match matrice[line][char] {
            '|' if last_position == 'S' => line = line - 1,
            '|' if last_position == 'N' => line = line + 1,
            '-' if last_position == 'E' => char = char - 1,
            '-' if last_position == 'W' => char = char + 1,
            'L' if last_position == 'N' => { char = char + 1; last_position = 'W'},
            'L' if last_position == 'E' => { line = line - 1; last_position = 'S'},
            'J' if last_position == 'N' => { char = char - 1; last_position = 'E'},
            'J' if last_position == 'W' => { line = line - 1; last_position = 'S'},
            '7' if last_position == 'S' => { char = char - 1; last_position = 'E'},
            '7' if last_position == 'W' => { line = line + 1; last_position = 'N'},
            '7' if last_position == 'S' => { char = char - 1; last_position = 'E'},
            '7' if last_position == 'W' => { line = line + 1; last_position = 'N'},
            'F' if last_position == 'S' => { char = char + 1; last_position = 'W'},
            'F' if last_position == 'E' => { line = line + 1; last_position = 'N'},
            _ => {println!("char {} :({},{}) not recognized",matrice[line][char],line,char);break;},
        }
        nb_steps+=1;
    }
    nb_steps
}

fn part1() {
    println!("---- Part 1 ----");
    let file = BufReader::new(File::open("./input.txt").expect("Unable to open file"));
    let matrice = create_matrice(file);
    let start_pos = find_start_coord(matrice.clone());
    let distance_max = follow_path(matrice, start_pos);
    println!("distance max = {}",(distance_max)/2);
    println!("----------------");
}

fn main() {
    part1();
}
