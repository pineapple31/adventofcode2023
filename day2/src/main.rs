use std::fs::read_to_string;

fn main() {
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
                match a[1] {
                    "red" if number <= 12 => println!("{} {} good",a[1],a[0]),
                    "green" if number <= 13 => println!("{} {} good",a[1],a[0]),
                    "blue" if number <= 14 => println!("{} {} good",a[1],a[0],),
                    _ => { println!("false"); good=false}
                }
            })
        }
        if good {
            good_games_id.push(game_id);
        }
        println!("{game:?}");
    }
    let sum: u32 = good_games_id.iter().sum();
    println!("{:?}",sum);
}
