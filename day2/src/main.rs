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
    let mut good_games_id: Vec<u32> = Vec::new();
    for line in content {
        let game: Vec<&str> = line.split(": ").collect();
        let game_id= game[0].split(" ").nth(1).unwrap().to_string().parse::<u32>().unwrap();
        let rounds: Vec<&str> = game[1].split("; ").collect();
        let mut good: bool=true;
        for round in rounds {
            round
            .split(", ")
            .for_each(|color| {
                let a: Vec<&str> =color.split(" ").collect();
                let number = a[0].parse::<u32>().unwrap();
                if !((a[1]=="red" && number <=12)||(a[1]=="green" && number <=13)||(a[1]=="blue" && number <=14)) {
                    good=false;
                }
                // match a[1] {
                //     "red" if number <= 12 => println!("{} {} good",a[1],a[0]),
                //     "green" if number <= 13 => println!("{} {} good",a[1],a[0]),
                //     "blue" if number <= 14 => println!("{} {} good",a[1],a[0],),
                //     _ => { println!("false"); good=false}
                // }
            })
        }
        if good {
            good_games_id.push(game_id);
        }
    }
    let sum: u32 = good_games_id.iter().sum();
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
    let mut total_power: u32 = 0;
    for line in content {
        let game: Vec<&str> = line.split(": ").collect();
        let rounds: Vec<&str> = game[1].split("; ").collect();
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for round in rounds {
            round.split(", ").for_each(|color| {
                let a: Vec<&str> =color.split(" ").collect();
                let number = a[0].parse::<u32>().unwrap();
                match a[1] {
                    "red" if number > min_red => min_red = number,
                    "green" if number > min_green => min_green = number,
                    "blue" if number > min_blue => min_blue = number,
                    _ => ()
                }
            });
        }
        total_power += min_red * min_green * min_blue;
    }
    println!("total power: {:?}",total_power);
    println!("----------------");
}


fn main() {
    part1();
    part2();
}
