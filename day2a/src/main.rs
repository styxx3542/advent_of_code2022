
enum Move {
    Rock,
    Paper,
    Scissors
}

fn determine_outcome(player1: Move,player2: Move) -> u32 {
    match player1 {
        Rock => {
            match player2{
                Rock,
                
            }
        },
        Paper => ,
        _ => 
    }
}

fn main() {
    let mut arr = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    arr.sort_unstable();

    println!("{}", arr.into_iter().rev().take(3).sum::<u32>())
}
