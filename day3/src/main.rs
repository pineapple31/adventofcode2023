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

fn find_symbols_coord(matrice: Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut symbols_coord: Vec<(usize,usize)> = Vec::new();
    let mut line_num: usize = 0;
    let mut char_num: usize = 0;
    for line in matrice {
        for ch in line {
            if !(ch.is_numeric()||ch=='.') {
                symbols_coord.push((line_num,char_num));
            }
            char_num+=1;
        }
        line_num+=1;
        char_num=0;
    }
    symbols_coord
}

fn find_all_numbers(matrice: Vec<Vec<char>>) -> Vec<(u32,usize,usize,usize)> {
    let mut line_num: usize = 0;
    let mut char_num: usize = 0;
    let mut first_num = true;
    let mut range: (usize, usize, usize) = (0,0,0);
    let mut numero_string = String::new();
    let mut numbers_range: Vec<(u32,usize,usize,usize)> = Vec::new();
    for line in matrice {
        for ch in line {
            if ch.is_numeric() {
                if first_num {
                    first_num = false;
                    range.0=line_num;
                    range.1=char_num;
                }
                numero_string.push(ch);
                range.2=char_num;
            } else if first_num == false {
                first_num = true;
                numbers_range.push((numero_string.as_str().parse::<u32>().unwrap(),range.0,range.1,range.2));
                numero_string = String::new();
            }
            char_num+=1;
        }
        line_num+=1;
        char_num=0;
    }
    numbers_range
}

fn sum_numbers_symbols_in_touch(numbers_range: Vec<(u32,usize,usize,usize)>, matrice: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for number in numbers_range {
        let num = number.0;
        let line = number.1;
        let start_char = number.2;
        let end_char = number.3;
        let mut next_to_symbols = false;
        let mut line_before = 0;
        let mut line_after = 0;
        let mut start_char_before = 0;
        let mut end_char_after = 0;

        if line == 0 {
            line_before = line;
            line_after = line+1;
        } else if line+1 == matrice.len() {
            line_before = line-1;
            line_after = line;
        } else {
            line_before = line-1;
            line_after = line+1;
        }
        if start_char == 0 {
            start_char_before = start_char;
        } else {
            start_char_before = start_char-1;
        }
        if end_char+1 == matrice[line].len() {
            end_char_after= end_char;
        } else {
            end_char_after = end_char+1;
        }

        // if num == 996 {
        //     println!("--------------------------");
        //     println!("num : {}", num);
        //     println!("line : {}", line);
        //     println!("line_before : {}", line_before);
        //     println!("line_after : {}", line_after);
        //     println!("start_char : {}", start_char);
        //     println!("start_char_before : {}", start_char_before);
        //     println!("end_char : {}", end_char);
        //     println!("end_char_after : {}", end_char_after);
        //     println!("matrice len : {} {} ",matrice[line].len(),matrice.len());
        //     println!("--------------------------");
        // }
        
        let mut line_num: usize = 0;
        let mut char_num: usize = 0;
        for line_index in matrice.clone() {
            if line_num >= line_before && line_num <= line_after {
                for ch_index in line_index {
                    if char_num >= start_char_before && line_num <= end_char_after {
                        if !(ch_index.is_numeric())&&!(ch_index=='.') {
                            next_to_symbols=true;
                        }
                    }
                    char_num+=1;
                }
            }
            line_num+=1;
            char_num=0;
        }
        // for row in [line_before..line_after] {
        //     for cha in [start_char_before..end_char_after] {
        //         let ch = matrice[row][cha].chars();
        //         if !(ch.is_numeric()||ch=='.') {
        //             next_to_symbols = true;
        //         }
        //     }
        // }
        if next_to_symbols {
            print!("{}\t", num);
            sum+=num;
        }
    }
    sum
}

fn part1() {
    println!("---- Part 1 ----");
    let file = BufReader::new(File::open("./input.txt").expect("Unable to open file"));
    let matrice = create_matrice(file);
    let symbols_coord = find_symbols_coord(matrice.clone());
    let numbers_range = find_all_numbers(matrice.clone());
    let sum = sum_numbers_symbols_in_touch(numbers_range.clone(), matrice.clone());
    println!("sum = {}",sum);
    println!("----------------");
}

fn main() {
    part1();
}
