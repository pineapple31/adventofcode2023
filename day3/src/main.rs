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

fn _find_symbols_coord(matrice: Vec<Vec<char>>) -> Vec<(usize,usize)> {
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
        let line_before;
        let line_after;
        let start_char_before;
        let end_char_after;

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
        let mut line_num: usize = 0;
        let mut char_num: usize = 0;
        for line_index in matrice.clone() {
            if line_num >= line_before && line_num <= line_after {
                for ch_index in line_index {
                    if char_num >= start_char_before && char_num <= end_char_after {
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
        if next_to_symbols {
            
            sum+=num;
        }
    }
    sum
}

fn find_all_stars(matrice: Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut line_star: usize = 0;
    let mut char_star: usize = 0;
    let mut stars_coord: Vec<(usize,usize)> = Vec::new();
    for line in matrice {
        for ch in line {
            if ch == '*' {
                stars_coord.push((line_star,char_star));
            }
            char_star+=1;
        }
        line_star+=1;
        char_star=0;
    }
    stars_coord
}

fn sum_gear_ratios(stars_coord: Vec<(usize,usize)>, matrice: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for star in stars_coord {
        let line = star.0;
        let char = star.1;
        let line_before ;
        let line_after;
        let char_before;
        let char_after;
        let mut numbers_around: Vec<u32> = Vec::new();
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
        if char == 0 {
            char_before = char;
            char_after = char+1;
        } else if char+1 == matrice.len() {
            char_before = char-1;
            char_after = char;
        } else {
            char_before = char-1;
            char_after = char+1;
        }
        let mut line_num: usize = 0;
        let mut char_num: usize = 0;
        for line_index in matrice.clone() {
            if line_num >= line_before && line_num <= line_after {
                for ch_index in line_index {
                    if char_num >= char_before && char_num <= char_after {
                        if ch_index.is_numeric() {
                            let chiffre = find_this_num(line_num,char_num, matrice.clone());
                            if !numbers_around.contains(&chiffre) {
                                numbers_around.push(chiffre);
                            }
                        }
                    }
                    char_num+=1;
                }
            }
            line_num+=1;
            char_num=0;
        }
        if numbers_around.len() ==2 {
            sum += numbers_around[0]*numbers_around[1];
        }
    }
    sum
}

fn find_this_num(line_num: usize ,char_num: usize ,matrice: Vec<Vec<char>>) -> u32 {
    let mut string_num = String::new();
    let mut char=char_num;
    let mut again = true;
    while matrice[line_num][char].is_numeric()&&again {
        if char!=0 {
            char-=1;
        } else {
            again = false;
        }
    }
    if again {
        char+=1;
    }
    while matrice[line_num][char].is_numeric() {
        string_num.push(matrice[line_num][char]);
        if char<matrice.len()-1 {
            char+=1;
        } else {
            break;
        }
    }
    let num =  string_num.parse::<u32>().unwrap();
    num
}

fn part1() {
    println!("---- Part 1 ----");
    let file = BufReader::new(File::open("./input.txt").expect("Unable to open file"));
    let matrice = create_matrice(file);
    // let symbols_coord = find_symbols_coord(matrice.clone());
    let numbers_range = find_all_numbers(matrice.clone());
    let sum = sum_numbers_symbols_in_touch(numbers_range.clone(), matrice.clone());
    println!("sum = {}",sum);
    println!("----------------");
}

fn part2() {
    println!("---- Part 2 ----");
    let file = BufReader::new(File::open("./input.txt").expect("Unable to open file"));
    let matrice = create_matrice(file);
    let stars_coord = find_all_stars(matrice.clone());
    let sum = sum_gear_ratios(stars_coord, matrice.clone());
    println!("sum = {}",sum);
    println!("----------------");
}

fn main() {
    part1();
    part2();
}
