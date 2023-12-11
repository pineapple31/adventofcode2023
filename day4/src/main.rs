use std::fs::read_to_string;

fn part1() {
    println!("---- Part 1 ----");
    let file_path = "./input.txt";
    let binding = read_to_string(file_path)
        .unwrap();
    let content : Vec<&str>= binding
        .as_str()
        .lines()
        .collect();
    let mut sum : u32 = 0;
    for line in content {
        let card: Vec<&str> = line.split(": ").collect();
        let _card_id= card[0].split_whitespace().nth(1).unwrap().to_string().parse::<u32>().unwrap();
        let numbers: Vec<&str> = card[1].split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0].split_whitespace().collect();
        let my_numbers: Vec<&str> = numbers[1].split_whitespace().collect();
        let mut gain: u32 = 0;
        for num in winning_numbers {
            if my_numbers.iter().any(|x| x == &num) {
                if gain == 0 {
                    gain = 1;
                } else {
                    gain = gain*2;
                }
            }
        }
        sum += gain;
    }
    println!("sum: {:?}",sum);
    println!("----------------");
}

fn part2() {
    println!("---- Part 2 ----");
    let file_path = "./input.txt";
    let binding = read_to_string(file_path)
        .unwrap();
    let content : Vec<&str>= binding
        .as_str()
        .lines()
        .collect();
    let nb_lines = content.len();
    let mut cards_coef: Vec<u32> = vec![1;nb_lines];
    for line in content {
        let card: Vec<&str> = line.split(": ").collect();
        let card_id= card[0].split_whitespace().nth(1).unwrap().to_string().parse::<usize>().unwrap();
        let numbers: Vec<&str> = card[1].split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0].split_whitespace().collect();
        let my_numbers: Vec<&str> = numbers[1].split_whitespace().collect();
        let mut gain: usize = 0;
        for num in winning_numbers {
            if my_numbers.iter().any(|x| x == &num) {
                gain+=1;
            }
        }
        if gain!=0 {
            for index in card_id..card_id+gain {
                cards_coef[index] += 1*cards_coef[card_id-1];
            }
        }
    }
    let nb_card: u32 = cards_coef.iter().sum();
    println!("nb_cards: {:?}",nb_card);
    println!("----------------");
}


fn main() {
    part1();
    part2();
}
