fn part1() {
    println!("---- Part 1 ----");
    let races: Vec<(u32, u32)> = vec![(38,241),(94,1549),(79,1074),(70,1091)];
    let mut multiplication: u32 = 1;
    for race in races.clone() {
        let time = race.0;
        let distance_to_beat = race.1;
        let mut times_beaten: u32 = 0;
        for i in 0..=time {
            let time_pushing = i;
            let time_racing = time - i;
            let distance = time_racing * time_pushing;
            if distance > distance_to_beat {
                times_beaten+=1;
            }
        }
        multiplication*=times_beaten;
    }
    println!("multiplication = {}",multiplication);
    println!("----------------");
}

fn part2() {
    println!("---- Part 1 ----");
    let races: Vec<(u64, u64)> = vec![(38947970,241154910741091)];
    let mut multiplication: u64 = 1;
    for race in races.clone() {
        let time = race.0;
        let distance_to_beat = race.1;
        let mut times_beaten: u64 = 0;
        for i in 0..=time {
            let time_pushing = i;
            let time_racing = time - i;
            let distance = time_racing * time_pushing;
            if distance > distance_to_beat {
                times_beaten+=1;
            }
        }
        multiplication*=times_beaten;
    }
    println!("multiplication = {}",multiplication);
    println!("----------------");
}

fn main() {
    part1();
    part2();
}
